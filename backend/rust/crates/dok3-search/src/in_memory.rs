//! In-memory SearchPort and DocumentStorePort implementation for P0-3 vertical slice.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use dok3_analyzer::{SectionCandidate, select_matched_section_id};
use dok3_core::Dok3Error;
use dok3_domain::{
    AclScope, CategoryCount, CorpusCatalogEntry, Document, DocumentDetail, DocumentFormatLinks,
    DocumentId, DocumentStorePort, GenerationId, Principal, ReaderState, SearchHit, SearchPort,
    SearchRequest, SearchResponse, SearchSnapshotMeta, SectionCitation, SectionReadRequest,
    SectionReadResponse, SectionSummary, Sequence, build_corpus_catalog,
};

use crate::seed::load_seed_dataset;

const DEFAULT_CATEGORIES: &[(&str, &str)] = &[
    ("guide", "개발 가이드"),
    ("manual", "운영 매뉴얼"),
    ("code", "코드 레퍼런스"),
    ("policy", "내부 규정"),
    ("design", "dok3search-design"),
];

/// Thread-safe in-memory repository implementing search and document store ports.
#[derive(Debug, Clone)]
pub struct InMemorySearchRepository {
    documents: Vec<Document>,
    index_by_id: HashMap<String, usize>,
    paired_formats: HashMap<String, DocumentFormatLinks>,
    hidden_yaml_ids: HashSet<String>,
    legacy_aliases: HashMap<String, String>,
    snapshot: SearchSnapshotMeta,
}

impl InMemorySearchRepository {
    /// Builds a repository from explicit documents and optional Markdown/YAML format pairs.
    #[must_use]
    pub fn new(documents: Vec<Document>, pairs: Vec<DocumentFormatLinks>) -> Self {
        let mut index_by_id = HashMap::with_capacity(documents.len());
        for (idx, doc) in documents.iter().enumerate() {
            index_by_id.insert(doc.id.as_str().to_string(), idx);
        }

        let mut paired_formats = HashMap::new();
        let mut hidden_yaml_ids = HashSet::new();
        for pair in pairs {
            if index_by_id.contains_key(pair.markdown_id.as_str())
                && index_by_id.contains_key(pair.yaml_id.as_str())
            {
                hidden_yaml_ids.insert(pair.yaml_id.as_str().to_string());
                paired_formats.insert(pair.markdown_id.as_str().to_string(), pair.clone());
                paired_formats.insert(pair.yaml_id.as_str().to_string(), pair);
            }
        }

        let legacy_aliases = HashMap::from([
            (
                "design-design-md".to_string(),
                "design-index-md".to_string(),
            ),
            (
                "design-design-yaml".to_string(),
                "design-map-yaml".to_string(),
            ),
        ]);

        let snapshot = SearchSnapshotMeta {
            generation_id: GenerationId::new("gen-inmemory-000001"),
            epoch: 1,
            schema_version: 1,
            acl_policy_version: 1,
            applied_change_sequence: Sequence::new(documents.len() as u64),
            reader_state: ReaderState::Ready,
        };

        Self {
            documents,
            index_by_id,
            paired_formats,
            hidden_yaml_ids,
            legacy_aliases,
            snapshot,
        }
    }

    /// Loads built-in Korean sample documents and optional `dok3search-design` files from `design_dir`.
    #[must_use]
    pub fn from_seed(design_dir: Option<&Path>) -> Self {
        let seed = load_seed_dataset(design_dir);
        Self::new(seed.documents, seed.pairs)
    }

    fn resolve_doc(&self, id: &DocumentId) -> Result<&Document, Dok3Error> {
        let key = self
            .legacy_aliases
            .get(id.as_str())
            .map_or_else(|| id.as_str(), String::as_str);
        self.index_by_id
            .get(key)
            .and_then(|&idx| self.documents.get(idx))
            .ok_or_else(|| Dok3Error::NotFound(format!("document '{id}' not found")))
    }

    fn authorized_variants<'a>(
        &'a self,
        doc: &'a Document,
        effective_scopes: &[AclScope],
    ) -> Vec<&'a Document> {
        if let Some(links) = self.paired_formats.get(doc.id.as_str()) {
            let mut variants = Vec::with_capacity(2);
            for variant_id in [&links.markdown_id, &links.yaml_id] {
                if let Some(&idx) = self.index_by_id.get(variant_id.as_str())
                    && let Some(candidate) = self.documents.get(idx)
                    && effective_scopes.contains(&candidate.acl_scope)
                {
                    variants.push(candidate);
                }
            }
            if !variants.is_empty() {
                return variants;
            }
        }
        vec![doc]
    }
}

impl SearchPort for InMemorySearchRepository {
    fn search(
        &self,
        principal: &Principal,
        req: &SearchRequest,
    ) -> Result<SearchResponse, Dok3Error> {
        let req = req.clone().normalized();
        let query_trimmed = req.query.trim().to_string();
        let query_lower = query_trimmed.to_lowercase();
        let terms: Vec<String> = query_lower
            .split_whitespace()
            .filter(|t| !t.is_empty())
            .map(str::to_string)
            .collect();

        let effective_scopes = principal.effective_scopes(req.requested_scope.as_ref());

        struct ScoredDoc<'a> {
            idx: usize,
            doc: &'a Document,
            score: f32,
        }

        let mut matched = Vec::new();

        for (idx, doc) in self.documents.iter().enumerate() {
            if self.hidden_yaml_ids.contains(doc.id.as_str()) {
                continue;
            }
            if !effective_scopes.contains(&doc.acl_scope) {
                continue;
            }
            if let Some(lang) = req
                .filters
                .language
                .as_deref()
                .filter(|v| !v.trim().is_empty())
                && !doc.language.eq_ignore_ascii_case(lang.trim())
            {
                continue;
            }
            if let Some(source_id) = &req.filters.source_id
                && &doc.source_id != source_id
            {
                continue;
            }
            if !req.filters.tags.is_empty()
                && !req
                    .filters
                    .tags
                    .iter()
                    .all(|tag| doc.tags.iter().any(|dt| dt.eq_ignore_ascii_case(tag)))
            {
                continue;
            }

            if terms.is_empty() {
                matched.push(ScoredDoc {
                    idx,
                    doc,
                    score: 1.0,
                });
                continue;
            }

            let variants = self.authorized_variants(doc, &effective_scopes);
            let mut best_score: Option<f32> = None;
            for variant in variants {
                if let Some(score) = score_variant(variant, &query_lower, &terms) {
                    best_score = Some(best_score.map_or(score, |prev| prev.max(score)));
                }
            }
            if let Some(score) = best_score {
                matched.push(ScoredDoc { idx, doc, score });
            }
        }

        if !terms.is_empty() {
            matched.sort_by(|a, b| {
                b.score
                    .partial_cmp(&a.score)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| a.idx.cmp(&b.idx))
            });
        }

        let total_unfiltered_hits = matched.len();
        let mut counts_by_cat: HashMap<&str, usize> = HashMap::new();
        for item in &matched {
            *counts_by_cat.entry(item.doc.category.as_str()).or_insert(0) += 1;
        }
        let category_counts: Vec<CategoryCount> = DEFAULT_CATEGORIES
            .iter()
            .map(|(id, label)| CategoryCount {
                id: (*id).to_string(),
                label: (*label).to_string(),
                count: counts_by_cat.get(id).copied().unwrap_or(0),
            })
            .collect();

        let active_category = req
            .filters
            .category
            .as_deref()
            .map(str::trim)
            .filter(|c| !c.is_empty());

        let visible: Vec<&ScoredDoc<'_>> = matched
            .iter()
            .filter(|item| active_category.is_none_or(|cat| item.doc.category == cat))
            .collect();

        let related_searches = if !terms.is_empty() && !visible.is_empty() {
            let matched_categories: HashSet<&str> = visible
                .iter()
                .map(|item| item.doc.category.as_str())
                .collect();
            let mut seen_tags = HashSet::new();
            let mut related = Vec::new();
            for doc in &self.documents {
                if self.hidden_yaml_ids.contains(doc.id.as_str())
                    || !effective_scopes.contains(&doc.acl_scope)
                    || !matched_categories.contains(doc.category.as_str())
                {
                    continue;
                }
                let tag = doc.tag.trim();
                if tag.is_empty() || query_lower.contains(&tag.to_lowercase()) {
                    continue;
                }
                if seen_tags.insert(tag.to_string()) {
                    related.push(tag.to_string());
                    if related.len() == 6 {
                        break;
                    }
                }
            }
            related
        } else {
            Vec::new()
        };

        let total_hits = visible.len();
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

        let start = (page - 1) * limit;
        let end = start.saturating_add(limit).min(total_hits);
        let hits = visible[start..end]
            .iter()
            .map(|item| {
                let doc = item.doc;
                let candidates: Vec<SectionCandidate<'_>> = doc
                    .sections
                    .iter()
                    .map(|s| SectionCandidate {
                        section_id: s.section_id.as_str(),
                        heading: &s.heading,
                        body: &s.body,
                    })
                    .collect();
                let matched_id =
                    select_matched_section_id(&query_trimmed, &candidates).map(str::to_string);
                let snippet = if req.include_snippets {
                    build_snippet_for_section(doc, &query_trimmed, matched_id.as_deref())
                } else {
                    String::new()
                };
                SearchHit {
                    document_id: doc.id.clone(),
                    revision_id: doc.revision_id.clone(),
                    title: doc.title.clone(),
                    summary: doc.summary.clone(),
                    snippet,
                    category: doc.category.clone(),
                    topic: doc.topic.clone(),
                    source: doc.source.clone(),
                    tag: doc.tag.clone(),
                    tags: doc.tags.clone(),
                    date: doc.date.clone(),
                    sample_path: doc.sample_path.clone(),
                    acl_scope: doc.acl_scope.clone(),
                    has_paired_formats: self.paired_formats.contains_key(doc.id.as_str()),
                    matched_section_id: matched_id.map(dok3_domain::SectionId::new),
                    score: item.score,
                }
            })
            .collect();

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
            snapshot: self.snapshot.clone(),
        })
    }

    fn snapshot(&self) -> SearchSnapshotMeta {
        self.snapshot.clone()
    }
}

impl DocumentStorePort for InMemorySearchRepository {
    fn get_document(
        &self,
        principal: &Principal,
        id: &DocumentId,
    ) -> Result<DocumentDetail, Dok3Error> {
        let doc = self.resolve_doc(id)?;
        if !principal.allows(&doc.acl_scope) {
            return Err(Dok3Error::Forbidden(format!(
                "principal '{}' is not permitted to access scope '{}'",
                principal.subject, doc.acl_scope
            )));
        }

        let formats = self.paired_formats.get(doc.id.as_str()).and_then(|pair| {
            let md_allowed = self
                .resolve_doc(&pair.markdown_id)
                .is_ok_and(|d| principal.allows(&d.acl_scope));
            let yaml_allowed = self
                .resolve_doc(&pair.yaml_id)
                .is_ok_and(|d| principal.allows(&d.acl_scope));
            (md_allowed && yaml_allowed).then(|| pair.clone())
        });

        Ok(DocumentDetail {
            id: doc.id.clone(),
            revision_id: doc.revision_id.clone(),
            source_id: doc.source_id.clone(),
            source: doc.source.clone(),
            title: doc.title.clone(),
            summary: doc.summary.clone(),
            category: doc.category.clone(),
            topic: doc.topic.clone(),
            tag: doc.tag.clone(),
            tags: doc.tags.clone(),
            keywords: doc.keywords.clone(),
            language: doc.language.clone(),
            version: doc.version.clone(),
            date: doc.date.clone(),
            source_uri: doc.source_uri.clone(),
            sample_path: doc.sample_path.clone(),
            acl_scope: doc.acl_scope.clone(),
            body: doc.body.clone(),
            markdown: doc.markdown.clone(),
            sections: doc.sections.iter().map(SectionSummary::from).collect(),
            formats,
        })
    }

    fn get_section(
        &self,
        principal: &Principal,
        req: &SectionReadRequest,
    ) -> Result<SectionReadResponse, Dok3Error> {
        let req = req.clone().normalized();
        let doc = self.resolve_doc(&req.document_id)?;
        if !principal.allows(&doc.acl_scope) {
            return Err(Dok3Error::Forbidden(format!(
                "principal '{}' is not permitted to access scope '{}'",
                principal.subject, doc.acl_scope
            )));
        }

        let section = doc
            .sections
            .iter()
            .find(|s| s.section_id == req.section_id)
            .ok_or_else(|| {
                Dok3Error::NotFound(format!(
                    "section '{}' not found in document '{}'",
                    req.section_id, req.document_id
                ))
            })?;

        let cursor = req.cursor.unwrap_or(0);
        let total_chars = section.body.chars().count();
        if cursor > total_chars {
            return Err(Dok3Error::InvalidInput(format!(
                "section cursor {cursor} exceeds section length {total_chars}"
            )));
        }

        let end = cursor.saturating_add(req.max_chars).min(total_chars);
        let text: String = section
            .body
            .chars()
            .skip(cursor)
            .take(req.max_chars)
            .collect();
        let truncated = end < total_chars;
        let next_cursor = truncated.then_some(end);

        Ok(SectionReadResponse {
            source_revision: doc.revision_id.clone(),
            heading_path: vec![doc.title.clone(), section.heading.clone()],
            citation: SectionCitation {
                source_uri: doc.source_uri.clone(),
                section: section.section_id.clone(),
            },
            text,
            next_cursor,
            truncated,
        })
    }

    fn document_count(&self) -> usize {
        self.documents.len()
    }

    fn list_corpora(
        &self,
        principal: &Principal,
    ) -> Result<(usize, Vec<CorpusCatalogEntry>), Dok3Error> {
        let visible_docs = self
            .documents
            .iter()
            .filter(|doc| !self.hidden_yaml_ids.contains(doc.id.as_str()));
        Ok(build_corpus_catalog(visible_docs, principal))
    }
}

fn score_variant(doc: &Document, query_lower: &str, terms: &[String]) -> Option<f32> {
    let title_lower = doc.title.to_lowercase();
    let summary_lower = doc.summary.to_lowercase();
    let meta_lower = [
        doc.source.as_str(),
        doc.tag.as_str(),
        &doc.tags.join(" "),
        &doc.keywords.join(" "),
        doc.sample_path.as_deref().unwrap_or(""),
    ]
    .join(" ")
    .to_lowercase();
    let body_lower = [doc.body.as_str(), doc.markdown.as_deref().unwrap_or("")]
        .join(" ")
        .to_lowercase();

    let mut component_hits: Vec<(&String, Vec<String>)> = Vec::new();
    for term in terms {
        if !title_lower.contains(term)
            && !summary_lower.contains(term)
            && !meta_lower.contains(term)
            && !body_lower.contains(term)
        {
            let components = analyzer_components(term);
            let all_found = !components.is_empty()
                && components.iter().all(|c| {
                    title_lower.contains(c.as_str())
                        || summary_lower.contains(c.as_str())
                        || meta_lower.contains(c.as_str())
                        || body_lower.contains(c.as_str())
                });
            if !all_found {
                return None;
            }
            component_hits.push((term, components));
        }
    }

    let mut score = 1.0_f32;
    if title_lower == query_lower {
        score += 4.0;
    }
    for term in terms {
        if component_hits.iter().any(|(t, _)| std::ptr::eq(*t, term)) {
            continue;
        }
        if title_lower.contains(term) {
            score += 2.0;
        }
        if summary_lower.contains(term) || meta_lower.contains(term) {
            score += 1.0;
        }
        if body_lower.contains(term) {
            score += 0.5;
        }
    }
    for (_, components) in &component_hits {
        let total = components.len() as f32;
        let in_title = components
            .iter()
            .filter(|c| title_lower.contains(c.as_str()))
            .count() as f32;
        score += 2.0 * in_title / total + 0.5;
    }
    Some(score)
}

/// Analyzer components (after the whole term) of a term with no direct match.
fn analyzer_components(term: &str) -> Vec<String> {
    let options = dok3_analyzer::AnalyzeOptions {
        split_identifiers: true,
        char_ngrams: false,
    };
    dok3_analyzer::analyze_text(term, options)
        .terms
        .into_iter()
        .skip(1)
        .collect()
}

fn build_snippet_for_section(
    doc: &Document,
    query_trimmed: &str,
    matched_section_id: Option<&str>,
) -> String {
    if !query_trimmed.is_empty()
        && let Some(sid) = matched_section_id
        && let Some(sec) = doc.sections.iter().find(|s| s.section_id.as_str() == sid)
        && !sec.body.trim().is_empty()
    {
        return sec.body.trim().chars().take(280).collect();
    }
    build_snippet(doc)
}

fn build_snippet(doc: &Document) -> String {
    let combined = if doc.body.trim().is_empty() {
        doc.summary.trim().to_string()
    } else if doc.summary.trim().is_empty() {
        doc.body.trim().to_string()
    } else {
        format!("{} {}", doc.summary.trim(), doc.body.trim())
    };
    combined.chars().take(280).collect()
}
