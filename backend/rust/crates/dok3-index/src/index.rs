//! Single-writer Tantivy index, manual reader reload, and `SearchPort` adapter.

use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::sync::{Mutex, RwLock};

use dok3_core::Dok3Error;
use dok3_domain::{
    AclScope, CategoryCount, Document, DocumentId, GenerationId, Principal, ReaderState, SearchHit,
    SearchPort, SearchRequest, SearchResponse, SearchSnapshotMeta, SectionId, Sequence,
};
use tantivy::collector::{Count, TopDocs};
use tantivy::directory::MmapDirectory;
use tantivy::query::{AllQuery, BooleanQuery, BoostQuery, Occur, Query, QueryParser, TermQuery};
use tantivy::schema::{IndexRecordOption, Schema};
use tantivy::snippet::SnippetGenerator;
use tantivy::tokenizer::TokenStream;
use tantivy::{Index, IndexReader, IndexWriter, ReloadPolicy, Searcher, TantivyDocument, Term};

use crate::schema::{
    IndexFields, SCHEMA_VERSION, StoredHitMeta, TOKENIZER_NGRAM, build_schema, read_stored_str,
    register_tokenizers, to_tantivy_document,
};

const WRITER_HEAP_BYTES: usize = 15_000_000;

type QueryClause = (Occur, Box<dyn Query>);

const DEFAULT_CATEGORIES: &[(&str, &str)] = &[
    ("guide", "개발 가이드"),
    ("manual", "운영 매뉴얼"),
    ("code", "코드 레퍼런스"),
    ("policy", "내부 규정"),
    ("design", "dok3search-design"),
];

/// Tantivy-backed search index with a single serialized writer and manual reader reloads.
pub struct TantivyIndex {
    index: Index,
    fields: IndexFields,
    writer: Mutex<IndexWriter<TantivyDocument>>,
    reader: IndexReader,
    snapshot: RwLock<SearchSnapshotMeta>,
}

impl std::fmt::Debug for TantivyIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TantivyIndex")
            .field("snapshot", &self.snapshot())
            .field("doc_count", &self.doc_count())
            .finish()
    }
}

impl TantivyIndex {
    /// Creates an in-memory RAM index for fast unit/contract tests.
    pub fn create_in_ram(generation_id: GenerationId) -> Result<Self, Dok3Error> {
        let (schema, fields) = build_schema();
        let index = Index::create_in_ram(schema);
        Self::finish_init(index, fields, generation_id)
    }

    /// Opens an existing disk index at `dir` or initializes a new one with the schema.
    pub fn open_or_create(dir: &Path, generation_id: GenerationId) -> Result<Self, Dok3Error> {
        fs::create_dir_all(dir)
            .map_err(|e| Dok3Error::Index(format!("failed to create index dir: {e}")))?;
        let (schema, fields) = build_schema();
        let directory = MmapDirectory::open(dir)
            .map_err(|e| Dok3Error::Index(format!("failed to open index dir: {e}")))?;
        let index = Index::open_or_create(directory, schema.clone())
            .map_err(|e| Dok3Error::Index(format!("failed to open/create tantivy index: {e}")))?;
        let reconciled = reconcile_fields(&index.schema(), fields)?;
        Self::finish_init(index, reconciled, generation_id)
    }

    fn finish_init(
        index: Index,
        fields: IndexFields,
        generation_id: GenerationId,
    ) -> Result<Self, Dok3Error> {
        register_tokenizers(&index)?;
        let writer = index
            .writer_with_num_threads(1, WRITER_HEAP_BYTES)
            .map_err(|e| Dok3Error::Index(format!("failed to open index writer: {e}")))?;
        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::Manual)
            .try_into()
            .map_err(|e| Dok3Error::Index(format!("failed to open index reader: {e}")))?;

        let snapshot = SearchSnapshotMeta {
            generation_id,
            epoch: 1,
            schema_version: SCHEMA_VERSION,
            acl_policy_version: 1,
            applied_change_sequence: Sequence::new(0),
            reader_state: ReaderState::Ready,
        };

        Ok(Self {
            index,
            fields,
            writer: Mutex::new(writer),
            reader,
            snapshot: RwLock::new(snapshot),
        })
    }

    /// Idempotently upserts `doc` by deleting any prior document with the same `document_id`.
    /// Changes become searchable after [`Self::commit_and_reload`].
    pub fn upsert_document(&self, doc: &Document) -> Result<(), Dok3Error> {
        if doc.id.as_str().trim().is_empty() {
            return Err(Dok3Error::InvalidInput(
                "document_id must not be empty".to_string(),
            ));
        }
        let tv_doc = to_tantivy_document(&self.fields, doc)?;
        let writer = self
            .writer
            .lock()
            .map_err(|_| Dok3Error::Internal("index writer mutex poisoned".to_string()))?;
        writer.delete_term(Term::from_field_text(
            self.fields.document_id,
            doc.id.as_str(),
        ));
        writer
            .add_document(tv_doc)
            .map_err(|e| Dok3Error::Index(format!("failed to add document '{}': {e}", doc.id)))?;
        Ok(())
    }

    /// Queues deletion of `document_id`. Takes effect after [`Self::commit_and_reload`].
    pub fn delete_document(&self, document_id: &DocumentId) -> Result<(), Dok3Error> {
        if document_id.as_str().trim().is_empty() {
            return Err(Dok3Error::InvalidInput(
                "document_id must not be empty".to_string(),
            ));
        }
        let writer = self
            .writer
            .lock()
            .map_err(|_| Dok3Error::Internal("index writer mutex poisoned".to_string()))?;
        writer.delete_term(Term::from_field_text(
            self.fields.document_id,
            document_id.as_str(),
        ));
        Ok(())
    }

    /// Commits pending writer changes, reloads the reader, and advances snapshot epoch atomically.
    pub fn commit_and_reload(
        &self,
        applied_sequence: Option<Sequence>,
    ) -> Result<SearchSnapshotMeta, Dok3Error> {
        let mut writer = self
            .writer
            .lock()
            .map_err(|_| Dok3Error::Internal("index writer mutex poisoned".to_string()))?;
        writer
            .commit()
            .map_err(|e| Dok3Error::Index(format!("failed to commit index: {e}")))?;
        self.publish_reload_locked(&writer, applied_sequence)
    }

    /// Reloads the reader under the writer/snapshot barrier so reader and snapshot epoch always match.
    pub fn reload(&self) -> Result<SearchSnapshotMeta, Dok3Error> {
        let writer = self
            .writer
            .lock()
            .map_err(|_| Dok3Error::Internal("index writer mutex poisoned".to_string()))?;
        self.publish_reload_locked(&writer, None)
    }

    fn publish_reload_locked(
        &self,
        _writer: &std::sync::MutexGuard<'_, IndexWriter<TantivyDocument>>,
        applied_sequence: Option<Sequence>,
    ) -> Result<SearchSnapshotMeta, Dok3Error> {
        let mut snap = self
            .snapshot
            .write()
            .map_err(|_| Dok3Error::Internal("snapshot lock poisoned".to_string()))?;
        self.reader
            .reload()
            .map_err(|e| Dok3Error::Index(format!("failed to reload index reader: {e}")))?;

        snap.epoch += 1;
        if let Some(seq) = applied_sequence
            && seq > snap.applied_change_sequence
        {
            snap.applied_change_sequence = seq;
        }
        snap.reader_state = ReaderState::Ready;
        Ok(snap.clone())
    }

    /// Returns the number of committed live documents visible to the current reader.
    #[must_use]
    pub fn doc_count(&self) -> u64 {
        self.reader.searcher().num_docs()
    }

    fn build_base_clauses(
        &self,
        effective_scopes: &[AclScope],
        req: &SearchRequest,
    ) -> (Vec<QueryClause>, Option<Box<dyn Query>>) {
        let mut clauses: Vec<QueryClause> = Vec::new();

        let scope_queries: Vec<Box<dyn Query>> = effective_scopes
            .iter()
            .map(|scope| {
                Box::new(TermQuery::new(
                    Term::from_field_text(self.fields.acl_scope, &scope.to_string()),
                    IndexRecordOption::Basic,
                )) as Box<dyn Query>
            })
            .collect();
        clauses.push((Occur::Must, Box::new(BooleanQuery::union(scope_queries))));

        if let Some(lang) = req
            .filters
            .language
            .as_deref()
            .map(str::trim)
            .filter(|v| !v.is_empty())
        {
            clauses.push((
                Occur::Must,
                Box::new(TermQuery::new(
                    Term::from_field_text(self.fields.language, &lang.to_lowercase()),
                    IndexRecordOption::Basic,
                )),
            ));
        }

        if let Some(source_id) = req
            .filters
            .source_id
            .as_ref()
            .filter(|s| !s.as_str().trim().is_empty())
        {
            clauses.push((
                Occur::Must,
                Box::new(TermQuery::new(
                    Term::from_field_text(self.fields.source_id, source_id.as_str()),
                    IndexRecordOption::Basic,
                )),
            ));
        }

        for tag in &req.filters.tags {
            let clean = tag.trim().to_lowercase();
            if !clean.is_empty() {
                clauses.push((
                    Occur::Must,
                    Box::new(TermQuery::new(
                        Term::from_field_text(self.fields.tag_exact, &clean),
                        IndexRecordOption::Basic,
                    )),
                ));
            }
        }

        let trimmed_query = req.query.trim();
        if trimmed_query.is_empty() {
            clauses.push((Occur::Must, Box::new(AllQuery)));
            return (clauses, None);
        }

        let (combined_q, snippet_q) = self.build_text_query(trimmed_query);
        clauses.push((Occur::Must, combined_q));
        (clauses, Some(snippet_q))
    }

    fn build_text_query(&self, raw_query: &str) -> (Box<dyn Query>, Box<dyn Query>) {
        let mut text_parser = QueryParser::for_index(
            &self.index,
            vec![
                self.fields.title,
                self.fields.summary,
                self.fields.tags_text,
                self.fields.body,
            ],
        );
        text_parser.set_field_boost(self.fields.title, 3.0);
        text_parser.set_field_boost(self.fields.tags_text, 2.0);
        text_parser.set_field_boost(self.fields.summary, 1.5);
        text_parser.set_field_boost(self.fields.body, 1.0);
        text_parser.set_conjunction_by_default();

        let normalized = normalize_query_text(raw_query);
        let parse_input = if normalized.is_empty() {
            raw_query
        } else {
            normalized.as_str()
        };

        let (text_q, _) = text_parser.parse_query_lenient(parse_input);
        let snippet_q = text_q.box_clone();
        match self.build_ngram_query(parse_input) {
            Some(ngram_q) => {
                let combined =
                    BooleanQuery::union(vec![text_q, Box::new(BoostQuery::new(ngram_q, 0.35))]);
                (Box::new(combined), snippet_q)
            }
            None => (text_q, snippet_q),
        }
    }

    fn build_ngram_query(&self, parse_input: &str) -> Option<Box<dyn Query>> {
        let mut analyzer = self.index.tokenizers().get(TOKENIZER_NGRAM)?;
        let words: Vec<&str> = parse_input.split_whitespace().collect();
        if words.is_empty() {
            return None;
        }

        let mut word_queries: Vec<Box<dyn Query>> = Vec::with_capacity(words.len());
        for word in words {
            let mut seen = HashSet::new();
            let mut term_queries: Vec<Box<dyn Query>> = Vec::new();
            let mut stream = analyzer.token_stream(word);
            stream.process(&mut |token| {
                if seen.insert(token.text.clone()) {
                    term_queries.push(Box::new(TermQuery::new(
                        Term::from_field_text(self.fields.body_ngram, &token.text),
                        IndexRecordOption::Basic,
                    )));
                }
            });
            if term_queries.is_empty() {
                return None;
            }
            let word_query: Box<dyn Query> = if term_queries.len() == 1 {
                term_queries.into_iter().next()?
            } else {
                Box::new(BooleanQuery::intersection(term_queries))
            };
            word_queries.push(word_query);
        }

        if word_queries.len() == 1 {
            word_queries.into_iter().next()
        } else {
            Some(Box::new(BooleanQuery::intersection(word_queries)))
        }
    }

    fn compute_category_counts(
        &self,
        searcher: &Searcher,
        base_clauses: &[(Occur, Box<dyn Query>)],
    ) -> Result<Vec<CategoryCount>, Dok3Error> {
        let mut counts = Vec::with_capacity(DEFAULT_CATEGORIES.len());
        for &(id, label) in DEFAULT_CATEGORIES {
            let mut cat_clauses = clone_clauses(base_clauses);
            cat_clauses.push((
                Occur::Must,
                Box::new(TermQuery::new(
                    Term::from_field_text(self.fields.category, id),
                    IndexRecordOption::Basic,
                )),
            ));
            let cat_query = BooleanQuery::new(cat_clauses);
            let count = searcher
                .search(&cat_query, &Count)
                .map_err(|e| Dok3Error::Index(format!("category count search failed: {e}")))?;
            counts.push(CategoryCount {
                id: id.to_string(),
                label: label.to_string(),
                count,
            });
        }
        Ok(counts)
    }

    fn load_hits(
        &self,
        searcher: &Searcher,
        top_docs: Vec<(f32, tantivy::DocAddress)>,
        include_snippets: bool,
        snippet_query: Option<&dyn Query>,
    ) -> Result<Vec<SearchHit>, Dok3Error> {
        let snippet_gen = match (include_snippets, snippet_query) {
            (true, Some(q)) => SnippetGenerator::create(searcher, q, self.fields.body).ok(),
            _ => None,
        };

        let mut hits = Vec::with_capacity(top_docs.len());
        for (score, addr) in top_docs {
            let doc: TantivyDocument = searcher
                .doc(addr)
                .map_err(|e| Dok3Error::Index(format!("failed to load stored doc: {e}")))?;

            let payload_raw = read_stored_str(&doc, self.fields.hit_payload);
            let meta: StoredHitMeta = serde_json::from_str(payload_raw)
                .map_err(|e| Dok3Error::Index(format!("invalid stored hit payload: {e}")))?;
            let acl_scope = AclScope::from_str(read_stored_str(&doc, self.fields.acl_scope))?;

            let snippet = if include_snippets {
                let fragment = snippet_gen
                    .as_ref()
                    .map(|g| g.snippet_from_doc(&doc).fragment().trim().to_string())
                    .filter(|s| !s.is_empty());
                match fragment {
                    Some(text) => text.chars().take(280).collect(),
                    None => meta.snippet_source.chars().take(280).collect(),
                }
            } else {
                String::new()
            };

            hits.push(SearchHit {
                document_id: read_stored_str(&doc, self.fields.document_id).into(),
                revision_id: read_stored_str(&doc, self.fields.revision_id).into(),
                title: read_stored_str(&doc, self.fields.title).to_string(),
                summary: read_stored_str(&doc, self.fields.summary).to_string(),
                snippet,
                category: read_stored_str(&doc, self.fields.category).to_string(),
                topic: None,
                source: meta.source,
                tag: meta.tag,
                tags: meta.tags,
                date: meta.date,
                sample_path: meta.sample_path,
                acl_scope,
                has_paired_formats: false,
                matched_section_id: meta.matched_section_id.map(SectionId::new),
                score,
            });
        }
        Ok(hits)
    }
}

impl SearchPort for TantivyIndex {
    fn search(
        &self,
        principal: &Principal,
        req: &SearchRequest,
    ) -> Result<SearchResponse, Dok3Error> {
        let req = req.clone().normalized();
        let query_trimmed = req.query.trim().to_string();
        let effective_scopes = principal.effective_scopes(req.requested_scope.as_ref());

        if effective_scopes.is_empty() {
            return Ok(empty_response(query_trimmed, &req, self.snapshot()));
        }

        let (searcher, snapshot) = {
            let snap_guard = self
                .snapshot
                .read()
                .map_err(|_| Dok3Error::Internal("snapshot lock poisoned".to_string()))?;
            (self.reader.searcher(), snap_guard.clone())
        };
        let (base_clauses, snippet_q) = self.build_base_clauses(&effective_scopes, &req);
        let base_query = BooleanQuery::new(clone_clauses(&base_clauses));

        let total_unfiltered_hits = searcher
            .search(&base_query, &Count)
            .map_err(|e| Dok3Error::Index(format!("count search failed: {e}")))?;
        let category_counts = self.compute_category_counts(&searcher, &base_clauses)?;

        let active_category = req
            .filters
            .category
            .as_deref()
            .map(str::trim)
            .filter(|c| !c.is_empty());

        let mut visible_clauses = base_clauses;
        if let Some(cat) = active_category {
            visible_clauses.push((
                Occur::Must,
                Box::new(TermQuery::new(
                    Term::from_field_text(self.fields.category, cat),
                    IndexRecordOption::Basic,
                )),
            ));
        }
        let visible_query = BooleanQuery::new(visible_clauses);

        let total_hits = searcher
            .search(&visible_query, &Count)
            .map_err(|e| Dok3Error::Index(format!("visible count search failed: {e}")))?;

        let limit = req.limit;
        let total_pages = if total_hits == 0 {
            0
        } else {
            total_hits.div_ceil(limit)
        };
        let page = if total_pages == 0 {
            1
        } else {
            req.page.clamp(1, total_pages)
        };
        let offset = (page - 1) * limit;

        let top_docs = if total_hits == 0 {
            Vec::new()
        } else {
            searcher
                .search(
                    &visible_query,
                    &TopDocs::with_limit(limit)
                        .and_offset(offset)
                        .order_by_score(),
                )
                .map_err(|e| Dok3Error::Index(format!("top-docs search failed: {e}")))?
        };

        let hits = self.load_hits(
            &searcher,
            top_docs,
            req.include_snippets,
            snippet_q.as_deref(),
        )?;

        let related_searches = collect_related_tags(&query_trimmed, &hits);

        Ok(SearchResponse {
            query: query_trimmed,
            page,
            limit,
            total_hits,
            total_pages,
            total_unfiltered_hits,
            category_counts,
            related_searches,
            hits,
            snapshot,
        })
    }

    fn snapshot(&self) -> SearchSnapshotMeta {
        self.snapshot
            .read()
            .map(|s| s.clone())
            .unwrap_or_else(|_| SearchSnapshotMeta {
                generation_id: GenerationId::new("unknown"),
                epoch: 0,
                schema_version: SCHEMA_VERSION,
                acl_policy_version: 1,
                applied_change_sequence: Sequence::new(0),
                reader_state: ReaderState::Degraded,
            })
    }
}

fn reconcile_fields(schema: &Schema, fallback: IndexFields) -> Result<IndexFields, Dok3Error> {
    let field = |name: &str| {
        schema
            .get_field(name)
            .map_err(|e| Dok3Error::Index(format!("schema missing field '{name}': {e}")))
    };
    let _ = fallback;
    Ok(IndexFields {
        document_id: field("document_id")?,
        revision_id: field("revision_id")?,
        source_id: field("source_id")?,
        acl_scope: field("acl_scope")?,
        category: field("category")?,
        language: field("language")?,
        tag_exact: field("tag_exact")?,
        title: field("title")?,
        summary: field("summary")?,
        tags_text: field("tags_text")?,
        body: field("body")?,
        body_ngram: field("body_ngram")?,
        hit_payload: field("hit_payload")?,
    })
}

fn normalize_query_text(raw: &str) -> String {
    raw.chars()
        .map(|ch| {
            if matches!(
                ch,
                ':' | '('
                    | ')'
                    | '['
                    | ']'
                    | '{'
                    | '}'
                    | '^'
                    | '~'
                    | '*'
                    | '?'
                    | '\\'
                    | '"'
                    | '+'
                    | '/'
                    | '-'
            ) {
                ' '
            } else {
                ch
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn clone_clauses(clauses: &[(Occur, Box<dyn Query>)]) -> Vec<(Occur, Box<dyn Query>)> {
    clauses
        .iter()
        .map(|(occur, q)| (*occur, q.box_clone()))
        .collect()
}

fn collect_related_tags(query_trimmed: &str, hits: &[SearchHit]) -> Vec<String> {
    if query_trimmed.is_empty() || hits.is_empty() {
        return Vec::new();
    }
    let query_lower = query_trimmed.to_lowercase();
    let mut seen = HashSet::new();
    let mut related = Vec::new();
    for hit in hits {
        let tag = hit.tag.trim();
        if tag.is_empty() || query_lower.contains(&tag.to_lowercase()) {
            continue;
        }
        if seen.insert(tag.to_string()) {
            related.push(tag.to_string());
            if related.len() == 6 {
                break;
            }
        }
    }
    related
}

fn empty_response(
    query: String,
    req: &SearchRequest,
    snapshot: SearchSnapshotMeta,
) -> SearchResponse {
    let category_counts = DEFAULT_CATEGORIES
        .iter()
        .map(|&(id, label)| CategoryCount {
            id: id.to_string(),
            label: label.to_string(),
            count: 0,
        })
        .collect();
    SearchResponse {
        query,
        page: 1,
        limit: req.limit,
        total_hits: 0,
        total_pages: 0,
        total_unfiltered_hits: 0,
        category_counts,
        related_searches: Vec::new(),
        hits: Vec::new(),
        snapshot,
    }
}
