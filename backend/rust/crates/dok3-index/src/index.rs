//! Single-writer Tantivy index, manual reader reload, and `SearchPort` adapter.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::sync::{Mutex, RwLock};

use dok3_analyzer::{
    AnalyzeOptions, EMBEDDING_DIM, SectionCandidate, analyze_text, cosine_similarity, embed_text,
    expand_synonyms, is_chosung_query, select_matched_section_id, word_spans,
};
use dok3_core::Dok3Error;
use dok3_domain::{
    AclScope, CategoryCount, Document, DocumentId, FacetCount, GenerationId, Principal,
    ReaderState, SearchHit, SearchMode, SearchPort, SearchRequest, SearchResponse,
    SearchSnapshotMeta, SearchSort, SectionId, Sequence, TopicCount,
};
use tantivy::collector::{Count, TopDocs};
use tantivy::directory::MmapDirectory;
use tantivy::query::{
    AllQuery, BooleanQuery, BoostQuery, EmptyQuery, FuzzyTermQuery, Occur, Query, TermQuery,
};
use tantivy::schema::{IndexRecordOption, Schema};
use tantivy::{Index, IndexReader, IndexWriter, ReloadPolicy, Searcher, TantivyDocument, Term};

use crate::schema::{
    IndexFields, SCHEMA_VERSION, StoredHitMeta, build_schema, read_stored_str, register_tokenizers,
    to_tantivy_document,
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

const DEFAULT_TOPICS: &[(&str, &str)] = &[("rust", "Rust"), ("go", "Go")];

/// Tantivy-backed search index with a single serialized writer and manual reader reloads.
pub struct TantivyIndex {
    fields: IndexFields,
    writer: Mutex<IndexWriter<TantivyDocument>>,
    reader: IndexReader,
    snapshot: RwLock<SearchSnapshotMeta>,
    embeddings: RwLock<HashMap<String, [f32; EMBEDDING_DIM]>>,
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
        let index = Index::open_or_create(directory, schema.clone()).map_err(|e| {
            Dok3Error::Index(format!(
                "failed to open/create Tantivy index (expected schema v{SCHEMA_VERSION}): {e}"
            ))
        })?;
        let reconciled = reconcile_fields(&index.schema(), fields)?;
        Self::finish_init(index, reconciled, generation_id)
    }

    fn finish_init(
        index: Index,
        fields: IndexFields,
        generation_id: GenerationId,
    ) -> Result<Self, Dok3Error> {
        register_tokenizers(&index);
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

        let embeddings = load_embeddings_from_reader(&reader, &fields);

        Ok(Self {
            fields,
            writer: Mutex::new(writer),
            reader,
            snapshot: RwLock::new(snapshot),
            embeddings: RwLock::new(embeddings),
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
        let emb_input = format!(
            "{} {} {} {}",
            doc.title,
            doc.summary,
            doc.tags.join(" "),
            doc.keywords.join(" ")
        );
        let vec = embed_text(&emb_input);
        {
            let writer = self
                .writer
                .lock()
                .map_err(|_| Dok3Error::Internal("index writer mutex poisoned".to_string()))?;
            writer.delete_term(Term::from_field_text(
                self.fields.document_id,
                doc.id.as_str(),
            ));
            writer.add_document(tv_doc).map_err(|e| {
                Dok3Error::Index(format!("failed to add document '{}': {e}", doc.id))
            })?;
        }
        if let Ok(mut guard) = self.embeddings.write() {
            guard.insert(doc.id.as_str().to_string(), vec);
        }
        Ok(())
    }

    /// Queues deletion of `document_id`. Takes effect after [`Self::commit_and_reload`].
    pub fn delete_document(&self, document_id: &DocumentId) -> Result<(), Dok3Error> {
        if document_id.as_str().trim().is_empty() {
            return Err(Dok3Error::InvalidInput(
                "document_id must not be empty".to_string(),
            ));
        }
        {
            let writer = self
                .writer
                .lock()
                .map_err(|_| Dok3Error::Internal("index writer mutex poisoned".to_string()))?;
            writer.delete_term(Term::from_field_text(
                self.fields.document_id,
                document_id.as_str(),
            ));
        }
        if let Ok(mut guard) = self.embeddings.write() {
            guard.remove(document_id.as_str());
        }
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
        include_text_query: bool,
        allow_fuzzy: bool,
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

        let ops = dok3_domain::parse_query_operators(req.query.trim());
        let mut has_operator = false;

        for term_raw in ops.path_terms.iter().chain(ops.ext_terms.iter()) {
            let analyzed = analyze_text(
                term_raw,
                AnalyzeOptions {
                    split_identifiers: true,
                    char_ngrams: false,
                },
            );
            for t in analyzed.terms {
                has_operator = true;
                clauses.push((
                    Occur::Must,
                    Box::new(TermQuery::new(
                        Term::from_field_text(self.fields.path_text, &t),
                        IndexRecordOption::WithFreqs,
                    )),
                ));
            }
        }

        for sym_raw in &ops.symbol_terms {
            let analyzed = analyze_text(
                sym_raw,
                AnalyzeOptions {
                    split_identifiers: true,
                    char_ngrams: false,
                },
            );
            for t in analyzed.terms {
                has_operator = true;
                clauses.push((
                    Occur::Must,
                    Box::new(TermQuery::new(
                        Term::from_field_text(self.fields.symbols, &t),
                        IndexRecordOption::WithFreqs,
                    )),
                ));
            }
        }

        if !include_text_query || ops.free_query.is_empty() {
            if !has_operator {
                clauses.push((Occur::Must, Box::new(AllQuery)));
            }
            return (clauses, None);
        }

        let (combined_q, snippet_q) = self.build_text_query(&ops.free_query, allow_fuzzy);
        clauses.push((Occur::Must, combined_q));
        (clauses, Some(snippet_q))
    }

    fn build_text_query(&self, raw_query: &str, fuzzy: bool) -> (Box<dyn Query>, Box<dyn Query>) {
        if is_chosung_query(raw_query) {
            let chosung_tokens: Vec<Box<dyn Query>> = raw_query
                .split_whitespace()
                .filter(|t| !t.is_empty())
                .map(|t| {
                    Box::new(BoostQuery::new(
                        Box::new(TermQuery::new(
                            Term::from_field_text(self.fields.title_chosung, t),
                            IndexRecordOption::WithFreqs,
                        )),
                        2.0,
                    )) as Box<dyn Query>
                })
                .collect();
            if !chosung_tokens.is_empty() {
                let q = BooleanQuery::intersection(chosung_tokens);
                return (Box::new(q.clone()), Box::new(q));
            }
        }

        let whole = analyze_text(
            raw_query,
            AnalyzeOptions {
                split_identifiers: false,
                char_ngrams: true,
            },
        );
        if whole.terms.is_empty() {
            return (Box::new(EmptyQuery), Box::new(EmptyQuery));
        }

        let mut seen = HashSet::new();
        let words = word_spans(raw_query)
            .filter(|&(_, _, word)| seen.insert(word))
            .map(|(_, _, word)| {
                let analyzed = analyze_text(
                    word,
                    AnalyzeOptions {
                        split_identifiers: true,
                        char_ngrams: false,
                    },
                );
                let mut terms = analyzed.terms.iter();
                let first_term = terms.next().expect("nonempty analyzed word");
                let exact = self.build_term_query(first_term);
                let mut word_alts = vec![exact];

                for syn in expand_synonyms(first_term) {
                    word_alts.push(Box::new(BoostQuery::new(self.build_term_query(&syn), 0.45)));
                }
                if fuzzy && first_term.chars().count() >= 4 {
                    word_alts.push(Box::new(BoostQuery::new(
                        Box::new(FuzzyTermQuery::new_prefix(
                            Term::from_field_text(self.fields.title, first_term),
                            1,
                            true,
                        )),
                        0.35,
                    )));
                    word_alts.push(Box::new(BoostQuery::new(
                        Box::new(FuzzyTermQuery::new_prefix(
                            Term::from_field_text(self.fields.body, first_term),
                            1,
                            true,
                        )),
                        0.25,
                    )));
                }

                // A particle-stripped compound stem (e.g. 설계검토및보완결과 from
                // 설계검토및보완결과를) may never appear as a document token when
                // the text is spaced; its segmented parts are required instead.
                let rest: Vec<&String> = terms.collect();
                let components: Vec<_> = rest
                    .iter()
                    .enumerate()
                    .filter(|(i, term)| {
                        !(*i == 0
                            && rest.len() >= 3
                            && term.chars().count() >= 4
                            && word.to_lowercase().starts_with(term.as_str())
                            && rest.len() > 1)
                    })
                    .map(|(_, term)| self.build_term_query(term))
                    .collect();
                if !components.is_empty() {
                    word_alts.push(Box::new(BoostQuery::new(
                        Box::new(BooleanQuery::intersection(components)),
                        0.6,
                    )));
                }
                if word_alts.len() == 1 {
                    word_alts.pop().expect("single alt")
                } else {
                    Box::new(BooleanQuery::union(word_alts)) as Box<dyn Query>
                }
            })
            .collect();
        let snippet = BooleanQuery::intersection(words);
        let mut alternatives = vec![Box::new(snippet.clone()) as Box<dyn Query>];

        // A word too short to emit a gram must still match as a whole term.
        if !whole.char_ngrams.is_empty() && whole.terms.iter().all(|word| word.chars().count() >= 2)
        {
            let mut seen = HashSet::new();
            let grams = whole
                .char_ngrams
                .iter()
                .filter(|gram| seen.insert(*gram))
                .map(|gram| {
                    Box::new(TermQuery::new(
                        Term::from_field_text(self.fields.body_ngram, gram),
                        IndexRecordOption::Basic,
                    )) as Box<dyn Query>
                })
                .collect();
            alternatives.push(Box::new(BoostQuery::new(
                Box::new(BooleanQuery::intersection(grams)),
                0.35,
            )));
        }
        (
            Box::new(BooleanQuery::union(alternatives)),
            Box::new(snippet),
        )
    }

    fn build_term_query(&self, term: &str) -> Box<dyn Query> {
        let fields = [
            (self.fields.symbols, 3.5),
            (self.fields.title, 3.0),
            (self.fields.path_text, 2.5),
            (self.fields.tags_text, 2.0),
            (self.fields.summary, 1.5),
            (self.fields.body, 1.0),
        ];
        let matches = fields
            .into_iter()
            .map(|(field, boost)| {
                Box::new(BoostQuery::new(
                    Box::new(TermQuery::new(
                        Term::from_field_text(field, term),
                        IndexRecordOption::WithFreqs,
                    )),
                    boost,
                )) as Box<dyn Query>
            })
            .collect();
        Box::new(BooleanQuery::union(matches))
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

    fn compute_topic_counts(
        &self,
        searcher: &Searcher,
        clauses_with_category: &[(Occur, Box<dyn Query>)],
    ) -> Result<Vec<TopicCount>, Dok3Error> {
        let mut counts = Vec::with_capacity(DEFAULT_TOPICS.len());
        for &(id, label) in DEFAULT_TOPICS {
            let mut topic_clauses = clone_clauses(clauses_with_category);
            topic_clauses.push((
                Occur::Must,
                Box::new(TermQuery::new(
                    Term::from_field_text(self.fields.topic, id),
                    IndexRecordOption::Basic,
                )),
            ));
            let topic_query = BooleanQuery::new(topic_clauses);
            let count = searcher
                .search(&topic_query, &Count)
                .map_err(|e| Dok3Error::Index(format!("topic count search failed: {e}")))?;
            counts.push(TopicCount {
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
        query: &str,
        include_snippets: bool,
        snippet_terms: &[String],
    ) -> Result<Vec<SearchHit>, Dok3Error> {
        let mut hits = Vec::with_capacity(top_docs.len());
        for (score, addr) in top_docs {
            let doc: TantivyDocument = searcher
                .doc(addr)
                .map_err(|e| Dok3Error::Index(format!("failed to load stored doc: {e}")))?;

            let payload_raw = read_stored_str(&doc, self.fields.hit_payload);
            let meta: StoredHitMeta = serde_json::from_str(payload_raw)
                .map_err(|e| Dok3Error::Index(format!("invalid stored hit payload: {e}")))?;
            let acl_scope = AclScope::from_str(read_stored_str(&doc, self.fields.acl_scope))?;

            let matched_id = if meta.sections.is_empty() {
                meta.matched_section_id.clone()
            } else {
                let candidates: Vec<SectionCandidate<'_>> = meta
                    .sections
                    .iter()
                    .map(|s| SectionCandidate {
                        section_id: &s.section_id,
                        heading: &s.heading,
                        body: &s.body,
                    })
                    .collect();
                select_matched_section_id(query, &candidates)
                    .map(str::to_string)
                    .or_else(|| meta.matched_section_id.clone())
            };

            let matched_section = matched_id
                .as_deref()
                .and_then(|sid| meta.sections.iter().find(|s| s.section_id == sid));
            let matched_section_heading = matched_section
                .filter(|s| s.section_id != "body" && s.section_id != "preamble")
                .map(|s| s.heading.clone());
            let matched_line_start =
                matched_section.and_then(|s| (s.line_start > 0).then_some(s.line_start));
            let matched_section_body = matched_section
                .map(|s| s.body.trim())
                .filter(|b| !b.is_empty());

            let preserve_lines = meta.sample_path.as_deref().is_some_and(|p| {
                let lower = p.to_ascii_lowercase();
                [
                    ".ts", ".tsx", ".mts", ".cts", ".js", ".jsx", ".mjs", ".cjs", ".rs", ".go",
                    ".py", ".java", ".sql", ".jsp",
                ]
                .iter()
                .any(|ext| lower.ends_with(ext))
            });

            let snippet = if include_snippets {
                let body_str = read_stored_str(&doc, self.fields.body);
                extract_fast_snippet(
                    matched_section_body,
                    body_str,
                    &meta.snippet_source,
                    snippet_terms,
                    280,
                    preserve_lines,
                )
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
                topic: meta.topic,
                source: meta.source,
                tag: meta.tag,
                tags: meta.tags,
                date: meta.date,
                sample_path: meta.sample_path,
                acl_scope,
                has_paired_formats: false,
                matched_section_id: matched_id.map(SectionId::new),
                matched_section_heading,
                matched_line_start,
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
        let (base_clauses, _) = self.build_base_clauses(&effective_scopes, &req, true, false);
        let base_query = BooleanQuery::new(clone_clauses(&base_clauses));

        let mut total_unfiltered_hits = searcher
            .search(&base_query, &Count)
            .map_err(|e| Dok3Error::Index(format!("count search failed: {e}")))?;
        let mut category_counts = self.compute_category_counts(&searcher, &base_clauses)?;

        let active_category = req
            .filters
            .category
            .as_deref()
            .map(str::trim)
            .filter(|c| !c.is_empty());
        let active_topic = req
            .filters
            .topic
            .as_deref()
            .map(str::trim)
            .filter(|t| !t.is_empty())
            .map(str::to_lowercase);

        let mut category_clauses = base_clauses;
        if let Some(cat) = active_category {
            category_clauses.push((
                Occur::Must,
                Box::new(TermQuery::new(
                    Term::from_field_text(self.fields.category, cat),
                    IndexRecordOption::Basic,
                )),
            ));
        }

        let mut topic_counts = if active_category == Some("guide") || active_topic.is_some() {
            self.compute_topic_counts(&searcher, &category_clauses)?
        } else {
            Vec::new()
        };

        let mut visible_clauses = category_clauses;
        if let Some(topic) = active_topic.as_deref() {
            visible_clauses.push((
                Occur::Must,
                Box::new(TermQuery::new(
                    Term::from_field_text(self.fields.topic, topic),
                    IndexRecordOption::Basic,
                )),
            ));
        }
        let mut visible_query = BooleanQuery::new(visible_clauses);

        // Fuzzy typo fallback runs only when exact/synonym/ngram matching produces zero hits.
        if req.fuzzy
            && !query_trimmed.is_empty()
            && searcher
                .search(&visible_query, &Count)
                .map_err(|e| Dok3Error::Index(format!("visible count check failed: {e}")))?
                == 0
        {
            let (fuzzy_base, _) = self.build_base_clauses(&effective_scopes, &req, true, true);
            let fuzzy_base_q = BooleanQuery::new(clone_clauses(&fuzzy_base));
            total_unfiltered_hits = searcher
                .search(&fuzzy_base_q, &Count)
                .map_err(|e| Dok3Error::Index(format!("fuzzy count search failed: {e}")))?;
            category_counts = self.compute_category_counts(&searcher, &fuzzy_base)?;
            let mut fuzzy_cat_clauses = fuzzy_base;
            if let Some(cat) = active_category {
                fuzzy_cat_clauses.push((
                    Occur::Must,
                    Box::new(TermQuery::new(
                        Term::from_field_text(self.fields.category, cat),
                        IndexRecordOption::Basic,
                    )),
                ));
            }
            if active_category == Some("guide") || active_topic.is_some() {
                topic_counts = self.compute_topic_counts(&searcher, &fuzzy_cat_clauses)?;
            }
            let mut fuzzy_visible_clauses = fuzzy_cat_clauses;
            if let Some(topic) = active_topic.as_deref() {
                fuzzy_visible_clauses.push((
                    Occur::Must,
                    Box::new(TermQuery::new(
                        Term::from_field_text(self.fields.topic, topic),
                        IndexRecordOption::Basic,
                    )),
                ));
            }
            visible_query = BooleanQuery::new(fuzzy_visible_clauses);
        }

        let date_from = req
            .filters
            .date_from
            .as_deref()
            .map(str::trim)
            .filter(|d| !d.is_empty());
        let date_to = req
            .filters
            .date_to
            .as_deref()
            .map(str::trim)
            .filter(|d| !d.is_empty());
        let has_date_filter = date_from.is_some() || date_to.is_some();
        let needs_post_process = has_date_filter
            || req.sort != SearchSort::Relevance
            || req.search_mode != SearchMode::Keyword;

        let limit = req.limit;
        let snippet_terms = if req.include_snippets && !query_trimmed.is_empty() {
            precompute_snippet_terms(&query_trimmed)
        } else {
            Vec::new()
        };

        let (total_hits, total_pages, page, hits, tag_facets, source_facets) =
            if !needs_post_process {
                let total_hits = searcher
                    .search(&visible_query, &Count)
                    .map_err(|e| Dok3Error::Index(format!("visible count search failed: {e}")))?;
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
                    &query_trimmed,
                    req.include_snippets,
                    &snippet_terms,
                )?;
                let (tag_facets, source_facets) = compute_facets(&hits);
                (
                    total_hits,
                    total_pages,
                    page,
                    hits,
                    tag_facets,
                    source_facets,
                )
            } else {
                let candidate_docs = if req.search_mode == SearchMode::Keyword
                    || query_trimmed.is_empty()
                {
                    searcher
                        .search(&visible_query, &TopDocs::with_limit(2000).order_by_score())
                        .map_err(|e| Dok3Error::Index(format!("candidate search failed: {e}")))?
                } else {
                    self.collect_hybrid_or_semantic_candidates(
                        &searcher,
                        &effective_scopes,
                        &req,
                        &visible_query,
                        &query_trimmed,
                    )?
                };

                let mut all_hits = self.load_hits(
                    &searcher,
                    candidate_docs,
                    &query_trimmed,
                    req.include_snippets,
                    &snippet_terms,
                )?;

                if has_date_filter {
                    all_hits
                        .retain(|hit| match_date_range(hit.date.as_deref(), date_from, date_to));
                }
                sort_hits(&mut all_hits, req.sort);

                let (tag_facets, source_facets) = compute_facets(&all_hits);
                let total_hits = all_hits.len();
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
                let start = (page - 1) * limit;
                let end = start.saturating_add(limit).min(total_hits);
                let page_hits = all_hits[start..end].to_vec();
                (
                    total_hits,
                    total_pages,
                    page,
                    page_hits,
                    tag_facets,
                    source_facets,
                )
            };

        let related_searches = collect_related_tags(&query_trimmed, &hits);

        Ok(SearchResponse {
            query: query_trimmed,
            page,
            limit,
            total_hits,
            total_pages,
            total_unfiltered_hits,
            category_counts,
            topic_counts,
            tag_facets,
            source_facets,
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
        topic: field("topic")?,
        language: field("language")?,
        tag_exact: field("tag_exact")?,
        title: field("title")?,
        title_chosung: field("title_chosung")?,
        summary: field("summary")?,
        tags_text: field("tags_text")?,
        symbols: field("symbols")?,
        path_text: field("path_text")?,
        body: field("body")?,
        body_ngram: field("body_ngram")?,
        hit_payload: field("hit_payload")?,
    })
}

impl TantivyIndex {
    fn collect_hybrid_or_semantic_candidates(
        &self,
        searcher: &Searcher,
        effective_scopes: &[AclScope],
        req: &SearchRequest,
        visible_query: &BooleanQuery,
        query_trimmed: &str,
    ) -> Result<Vec<(f32, tantivy::DocAddress)>, Dok3Error> {
        let q_vec = embed_text(query_trimmed);
        let (mut filter_only_clauses, _) =
            self.build_base_clauses(effective_scopes, req, false, false);
        if let Some(cat) = req
            .filters
            .category
            .as_deref()
            .map(str::trim)
            .filter(|c| !c.is_empty())
        {
            filter_only_clauses.push((
                Occur::Must,
                Box::new(TermQuery::new(
                    Term::from_field_text(self.fields.category, cat),
                    IndexRecordOption::Basic,
                )),
            ));
        }
        if let Some(topic) = req
            .filters
            .topic
            .as_deref()
            .map(str::trim)
            .filter(|t| !t.is_empty())
        {
            filter_only_clauses.push((
                Occur::Must,
                Box::new(TermQuery::new(
                    Term::from_field_text(self.fields.topic, &topic.to_lowercase()),
                    IndexRecordOption::Basic,
                )),
            ));
        }
        let filter_query = BooleanQuery::new(filter_only_clauses);
        let scope_docs = searcher
            .search(&filter_query, &TopDocs::with_limit(5000).order_by_score())
            .map_err(|e| Dok3Error::Index(format!("semantic scope search failed: {e}")))?;

        let emb_guard = self
            .embeddings
            .read()
            .map_err(|_| Dok3Error::Internal("embeddings lock poisoned".to_string()))?;

        let mut vec_scored: Vec<(f32, tantivy::DocAddress)> = Vec::new();
        for (_, addr) in scope_docs {
            if let Ok(doc) = searcher.doc::<TantivyDocument>(addr) {
                let doc_id = read_stored_str(&doc, self.fields.document_id);
                if let Some(doc_vec) = emb_guard.get(doc_id) {
                    let sim = cosine_similarity(&q_vec, doc_vec);
                    if sim > 0.05 {
                        vec_scored.push((sim, addr));
                    }
                }
            }
        }
        vec_scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        if req.search_mode == SearchMode::Semantic {
            vec_scored.truncate(500);
            return Ok(vec_scored);
        }

        // Hybrid RRF: combine BM25 rank + cosine vector rank
        let bm25_docs = searcher
            .search(visible_query, &TopDocs::with_limit(500).order_by_score())
            .map_err(|e| Dok3Error::Index(format!("hybrid bm25 search failed: {e}")))?;

        let mut rrf_scores: HashMap<tantivy::DocAddress, f32> = HashMap::new();
        for (rank, (_, addr)) in bm25_docs.into_iter().enumerate() {
            *rrf_scores.entry(addr).or_insert(0.0) += 1.0 / (60.0 + rank as f32 + 1.0);
        }
        for (rank, (_, addr)) in vec_scored.into_iter().take(200).enumerate() {
            *rrf_scores.entry(addr).or_insert(0.0) += 0.8 / (60.0 + rank as f32 + 1.0);
        }

        let mut merged: Vec<(f32, tantivy::DocAddress)> =
            rrf_scores.into_iter().map(|(addr, s)| (s, addr)).collect();
        merged.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        Ok(merged)
    }
}

fn load_embeddings_from_reader(
    reader: &IndexReader,
    fields: &IndexFields,
) -> HashMap<String, [f32; EMBEDDING_DIM]> {
    let searcher = reader.searcher();
    let mut map = HashMap::new();
    let Ok(all_docs) = searcher.search(&AllQuery, &TopDocs::with_limit(20_000).order_by_score())
    else {
        return map;
    };
    for (_, addr) in all_docs {
        if let Ok(doc) = searcher.doc::<TantivyDocument>(addr) {
            let doc_id = read_stored_str(&doc, fields.document_id);
            if doc_id.is_empty() {
                continue;
            }
            let title = read_stored_str(&doc, fields.title);
            let summary = read_stored_str(&doc, fields.summary);
            let vec = embed_text(&format!("{title} {summary}"));
            map.insert(doc_id.to_string(), vec);
        }
    }
    map
}

fn match_date_range(date: Option<&str>, from: Option<&str>, to: Option<&str>) -> bool {
    let Some(d) = date.map(str::trim).filter(|s| !s.is_empty()) else {
        return false;
    };
    if let Some(f) = from
        && d < f
    {
        return false;
    }
    if let Some(t) = to
        && d > t
    {
        return false;
    }
    true
}

fn sort_hits(hits: &mut [SearchHit], sort: SearchSort) {
    match sort {
        SearchSort::Relevance => {}
        SearchSort::DateDesc => {
            hits.sort_by(|a, b| b.date.cmp(&a.date).then_with(|| a.title.cmp(&b.title)));
        }
        SearchSort::DateAsc => {
            hits.sort_by(|a, b| {
                let da = a.date.as_deref().unwrap_or("9999-99-99");
                let db = b.date.as_deref().unwrap_or("9999-99-99");
                da.cmp(db).then_with(|| a.title.cmp(&b.title))
            });
        }
        SearchSort::TitleAsc => {
            hits.sort_by_key(|a| a.title.to_lowercase());
        }
    }
}

fn compute_facets(hits: &[SearchHit]) -> (Vec<FacetCount>, Vec<FacetCount>) {
    let mut tag_counts: HashMap<String, usize> = HashMap::new();
    let mut source_counts: HashMap<String, usize> = HashMap::new();
    for hit in hits.iter().take(100) {
        if !hit.source.trim().is_empty() {
            *source_counts.entry(hit.source.clone()).or_insert(0) += 1;
        }
        for tag in &hit.tags {
            let t = tag.trim();
            if !t.is_empty() {
                *tag_counts.entry(t.to_string()).or_insert(0) += 1;
            }
        }
    }
    let to_sorted = |map: HashMap<String, usize>| -> Vec<FacetCount> {
        let mut items: Vec<FacetCount> = map
            .into_iter()
            .map(|(value, count)| FacetCount { value, count })
            .collect();
        items.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.value.cmp(&b.value)));
        items.truncate(10);
        items
    };
    (to_sorted(tag_counts), to_sorted(source_counts))
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

fn precompute_snippet_terms(query: &str) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut terms = Vec::new();
    for (_, _, word) in word_spans(query) {
        let lower = word.to_lowercase();
        if !lower.is_empty() && seen.insert(lower.clone()) {
            terms.push(lower);
        }
    }
    let analyzed = analyze_text(
        query,
        AnalyzeOptions {
            split_identifiers: true,
            char_ngrams: false,
        },
    );
    for term in analyzed.terms {
        if !term.is_empty() && seen.insert(term.clone()) {
            terms.push(term);
        }
    }
    terms
}

fn find_term_byte_offset(text: &str, term: &str) -> Option<usize> {
    if term.is_empty() {
        return None;
    }
    if term.is_ascii() {
        let hay = text.as_bytes();
        let needle = term.as_bytes();
        hay.windows(needle.len())
            .position(|w| w.eq_ignore_ascii_case(needle))
            .filter(|&pos| text.is_char_boundary(pos))
    } else {
        let lower = text.to_lowercase();
        let lower_pos = lower.find(term)?;
        let char_idx = lower[..lower_pos].chars().count();
        text.char_indices().nth(char_idx).map(|(idx, _)| idx)
    }
}

fn extract_fast_snippet(
    matched_section_body: Option<&str>,
    body: &str,
    fallback_source: &str,
    terms: &[String],
    max_chars: usize,
    preserve_lines: bool,
) -> String {
    let format_out = |raw: &str| -> String {
        if preserve_lines {
            raw.lines()
                .map(str::trim_end)
                .filter(|l| !l.trim().is_empty())
                .take(3)
                .collect::<Vec<_>>()
                .join("\n")
                .chars()
                .take(max_chars)
                .collect()
        } else {
            raw.split_whitespace().collect::<Vec<_>>().join(" ")
        }
    };
    let candidates = [matched_section_body, Some(body)];
    for candidate in candidates.into_iter().flatten() {
        let trimmed = candidate.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(byte_pos) = terms.iter().find_map(|t| find_term_byte_offset(trimmed, t)) {
            let prefix_chars = trimmed[..byte_pos].chars().count();
            let start_char = prefix_chars.saturating_sub(64);
            let mut start_byte = trimmed
                .char_indices()
                .nth(start_char)
                .map_or(0, |(idx, _)| idx);
            if start_char > 0
                && let Some(ws) = trimmed[start_byte..byte_pos].find(char::is_whitespace)
            {
                let next = start_byte
                    + ws
                    + trimmed[start_byte + ws..]
                        .chars()
                        .next()
                        .map_or(0, char::len_utf8);
                if next < byte_pos {
                    start_byte = next;
                }
            }
            let snippet: String = trimmed[start_byte..].chars().take(max_chars).collect();
            return format_out(&snippet);
        }
    }
    let fallback: String = matched_section_body
        .unwrap_or(fallback_source)
        .chars()
        .take(max_chars)
        .collect();
    format_out(&fallback)
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
        topic_counts: Vec::new(),
        tag_facets: Vec::new(),
        source_facets: Vec::new(),
        related_searches: Vec::new(),
        hits: Vec::new(),
        snapshot,
    }
}
