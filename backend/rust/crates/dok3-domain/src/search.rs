//! Search query, response, snapshot, and health readiness contracts.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::acl::{AclScope, Principal};
use crate::document::Document;
use crate::ids::{DocumentId, GenerationId, RevisionId, SectionId, Sequence, SourceId};

const fn default_page() -> usize {
    1
}

const fn default_limit() -> usize {
    15
}

const fn default_include_snippets() -> bool {
    true
}

/// Lifecycle state of the search reader associated with a snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReaderState {
    Ready,
    Warming,
    Degraded,
}

/// Metadata identifying the generation and consistency boundary of a search response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchSnapshotMeta {
    pub generation_id: GenerationId,
    pub epoch: u64,
    pub schema_version: u32,
    pub acl_policy_version: u32,
    pub applied_change_sequence: Sequence,
    pub reader_state: ReaderState,
}

/// Structured query filters applied alongside ACL scope enforcement.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchFilters {
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub source_id: Option<SourceId>,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Search request contract. `requested_scope` is advisory only and cannot escalate ACLs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchRequest {
    #[serde(default)]
    pub query: String,
    #[serde(default)]
    pub filters: SearchFilters,
    #[serde(default = "default_page")]
    pub page: usize,
    #[serde(default = "default_limit")]
    pub limit: usize,
    #[serde(default = "default_include_snippets")]
    pub include_snippets: bool,
    #[serde(default)]
    pub requested_scope: Option<AclScope>,
}

impl Default for SearchRequest {
    fn default() -> Self {
        Self {
            query: String::new(),
            filters: SearchFilters::default(),
            page: default_page(),
            limit: default_limit(),
            include_snippets: default_include_snippets(),
            requested_scope: None,
        }
    }
}

impl SearchRequest {
    /// Normalizes pagination bounds (`page >= 1`, `limit` in `1..=100` with `0 -> 15`).
    #[must_use]
    pub fn normalized(mut self) -> Self {
        self.page = self.page.max(1);
        self.limit = if self.limit == 0 {
            default_limit()
        } else {
            self.limit.min(100)
        };
        self
    }
}

/// Individual search hit returned for a query.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchHit {
    pub document_id: DocumentId,
    pub revision_id: RevisionId,
    pub title: String,
    pub summary: String,
    pub snippet: String,
    pub category: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    pub source: String,
    pub tag: String,
    pub tags: Vec<String>,
    pub date: Option<String>,
    pub sample_path: Option<String>,
    pub acl_scope: AclScope,
    pub has_paired_formats: bool,
    pub matched_section_id: Option<SectionId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matched_section_heading: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matched_line_start: Option<usize>,
    pub score: f32,
}

/// Inline search query operators (`path:`, `ext:`, `sym:` / `symbol:`) extracted from raw queries.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ParsedQueryOperators {
    pub free_query: String,
    pub path_terms: Vec<String>,
    pub ext_terms: Vec<String>,
    pub symbol_terms: Vec<String>,
}

/// Extracts `path:<term>`, `ext:<ext>`, and `sym:<symbol>` operators from `raw`,
/// leaving all remaining tokens in `free_query`.
#[must_use]
pub fn parse_query_operators(raw: &str) -> ParsedQueryOperators {
    let mut out = ParsedQueryOperators::default();
    let mut free = Vec::new();
    for token in raw.split_whitespace() {
        let lower = token.to_ascii_lowercase();
        if let Some(val) = lower.strip_prefix("path:") {
            let clean = val.trim_matches(|c: char| c == '/' || c == '\\');
            if !clean.is_empty() {
                out.path_terms.push(clean.to_string());
                continue;
            }
        } else if let Some(val) = lower.strip_prefix("ext:") {
            let clean = val.trim_start_matches('.');
            if !clean.is_empty() {
                out.ext_terms.push(clean.to_string());
                continue;
            }
        } else if let Some(val) = lower
            .strip_prefix("sym:")
            .or_else(|| lower.strip_prefix("symbol:"))
            && !val.is_empty()
        {
            out.symbol_terms.push(val.to_string());
            continue;
        }
        free.push(token);
    }
    out.free_query = free.join(" ");
    out
}

/// Aggregated hit count for a single category computed under the caller's effective ACL.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryCount {
    pub id: String,
    pub label: String,
    pub count: usize,
}

/// Search response payload containing paginated hits, category facets, and snapshot metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResponse {
    pub query: String,
    pub page: usize,
    pub limit: usize,
    pub total_hits: usize,
    pub total_pages: usize,
    pub total_unfiltered_hits: usize,
    pub category_counts: Vec<CategoryCount>,
    pub related_searches: Vec<String>,
    pub hits: Vec<SearchHit>,
    pub snapshot: SearchSnapshotMeta,
}

/// Service readiness payload returned by `/health/ready`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthReadyResponse {
    pub status: String,
    pub mode: String,
    pub document_count: usize,
    pub snapshot: SearchSnapshotMeta,
}

/// Package/crate version summary inside a corpus catalog entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorpusPackageVersion {
    pub name: String,
    pub version: String,
    pub document_count: usize,
}

/// Public summary of an indexed corpus and its version/provenance metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorpusCatalogEntry {
    pub corpus_id: String,
    pub label: String,
    pub category: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    pub language: String,
    pub version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub toolchain: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    pub document_count: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub packages: Vec<CorpusPackageVersion>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

/// Public corpus catalog payload returned by `GET /api/v1/corpora`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorpusCatalogResponse {
    pub mode: String,
    pub total_documents: usize,
    pub corpora: Vec<CorpusCatalogEntry>,
    pub snapshot: SearchSnapshotMeta,
}

/// Single recommended search term derived from authorized indexed documents.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendedTerm {
    pub term: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    pub document_count: usize,
}

/// Public ACL-scoped recommended search terms response (`GET /api/v1/search/recommendations`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchRecommendationsResponse {
    pub query: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    pub recommendations: Vec<RecommendedTerm>,
    pub snapshot: SearchSnapshotMeta,
}

const CURATED_RECOMMENDATION_SEEDS: &[&str] = &[
    "kordoc",
    "hwpx",
    "HWP",
    "Tantivy",
    "폐쇄망",
    "세대 전환",
    "소유권",
    "goroutine",
    "MCP",
    "SQLite",
    "형태소",
    "백업",
    "E0382",
    "Cargo",
    "아키텍처",
    "보안",
];

const SYMBOL_HEADING_PREFIXES: &[&str] = &[
    "function ",
    "class ",
    "interface ",
    "type ",
    "enum ",
    "const ",
    "let ",
    "fn ",
    "struct ",
    "trait ",
    "mod ",
    "impl ",
    "func ",
    "def ",
    "method ",
    "record ",
];

fn is_noise_recommendation(term: &str) -> bool {
    let char_len = term.chars().count();
    if !(2..=32).contains(&char_len) {
        return true;
    }
    if term.chars().all(|c| c.is_ascii_digit()) {
        return true;
    }
    matches!(
        term.to_ascii_lowercase().as_str(),
        "ko" | "en"
            | "docs"
            | "guide"
            | "code"
            | "manual"
            | "policy"
            | "design"
            | "v1"
            | "markdown"
            | "yaml"
            | "section"
            | "preamble"
            | "body"
    )
}

#[derive(Default)]
struct RecommendationAcc {
    display: String,
    curated: bool,
    document_count: usize,
    category_counts: BTreeMap<String, usize>,
}

/// Computes ACL-scoped recommended search terms from visible documents.
#[must_use]
pub fn build_recommended_terms<'a, I>(
    docs: I,
    principal: &Principal,
    query: &str,
    category: Option<&str>,
    limit: usize,
) -> Vec<RecommendedTerm>
where
    I: IntoIterator<Item = &'a Document>,
{
    let limit = if limit == 0 { 8 } else { limit.min(20) };
    let q_trimmed = query.trim();
    let q_lower = q_trimmed.to_lowercase();
    let cat_filter = category.map(str::trim).filter(|c| !c.is_empty());

    let mut acc: BTreeMap<String, RecommendationAcc> = BTreeMap::new();

    for doc in docs {
        if !principal.allows(&doc.acl_scope) {
            continue;
        }
        if cat_filter.is_some_and(|c| !doc.category.eq_ignore_ascii_case(c)) {
            continue;
        }

        let mut per_doc: BTreeMap<String, (String, bool)> = BTreeMap::new();
        let mut add_candidate = |raw: &str, curated: bool| {
            let trimmed = raw.trim();
            if is_noise_recommendation(trimmed) {
                return;
            }
            let key = trimmed.to_lowercase();
            per_doc
                .entry(key)
                .and_modify(|existing| {
                    if curated {
                        existing.1 = true;
                        existing.0 = trimmed.to_string();
                    }
                })
                .or_insert_with(|| (trimmed.to_string(), curated));
        };

        if !doc.tag.trim().is_empty() {
            add_candidate(&doc.tag, false);
        }
        for tag in &doc.tags {
            add_candidate(tag, false);
        }
        for kw in doc.keywords.iter().take(6) {
            add_candidate(kw, false);
        }

        let title_lower = doc.title.to_lowercase();
        let summary_lower = doc.summary.to_lowercase();
        for &seed in CURATED_RECOMMENDATION_SEEDS {
            let seed_lower = seed.to_lowercase();
            if title_lower.contains(&seed_lower)
                || summary_lower.contains(&seed_lower)
                || doc
                    .tags
                    .iter()
                    .any(|t| t.to_lowercase().contains(&seed_lower))
            {
                add_candidate(seed, true);
            }
        }

        if !q_lower.is_empty() {
            for sec in &doc.sections {
                let mut heading = sec.heading.trim();
                for prefix in SYMBOL_HEADING_PREFIXES {
                    if let Some(rest) = heading.strip_prefix(prefix) {
                        heading = rest.trim();
                        break;
                    }
                }
                if heading.to_lowercase().contains(&q_lower) {
                    add_candidate(heading, false);
                }
            }
        }

        for (key, (display, curated)) in per_doc {
            let entry = acc.entry(key).or_insert_with(|| RecommendationAcc {
                display,
                curated,
                document_count: 0,
                category_counts: BTreeMap::new(),
            });
            if curated {
                entry.curated = true;
            }
            entry.document_count += 1;
            *entry
                .category_counts
                .entry(doc.category.clone())
                .or_insert(0) += 1;
        }
    }

    let dominant_category = |entry: &RecommendationAcc| -> Option<String> {
        entry
            .category_counts
            .iter()
            .max_by(|(ca, na), (cb, nb)| na.cmp(nb).then_with(|| cb.cmp(ca)))
            .map(|(c, _)| c.clone())
    };

    if !q_lower.is_empty() {
        let mut matches: Vec<(String, RecommendationAcc)> = acc
            .into_iter()
            .filter(|(key, _)| {
                key != &q_lower && (key.starts_with(&q_lower) || key.contains(&q_lower))
            })
            .collect();
        matches.sort_by(|(ka, a), (kb, b)| {
            kb.starts_with(&q_lower)
                .cmp(&ka.starts_with(&q_lower))
                .then_with(|| b.curated.cmp(&a.curated))
                .then_with(|| b.document_count.cmp(&a.document_count))
                .then_with(|| a.display.chars().count().cmp(&b.display.chars().count()))
                .then_with(|| ka.cmp(kb))
        });
        return matches
            .into_iter()
            .take(limit)
            .map(|(_, entry)| RecommendedTerm {
                category: dominant_category(&entry),
                term: entry.display,
                document_count: entry.document_count,
            })
            .collect();
    }

    let mut ranked: Vec<(String, RecommendationAcc)> = acc.into_iter().collect();
    ranked.sort_by(|(ka, a), (kb, b)| {
        b.curated
            .cmp(&a.curated)
            .then_with(|| b.document_count.cmp(&a.document_count))
            .then_with(|| ka.cmp(kb))
    });

    if cat_filter.is_none() {
        let mut selected: Vec<RecommendedTerm> = Vec::with_capacity(limit);
        let mut used_keys: BTreeSet<String> = BTreeSet::new();
        let mut per_cat: BTreeMap<String, usize> = BTreeMap::new();

        for (key, entry) in &ranked {
            if selected.len() >= limit {
                break;
            }
            let cat = dominant_category(entry);
            let cat_key = cat.clone().unwrap_or_default();
            if *per_cat.get(&cat_key).unwrap_or(&0) >= 2 {
                continue;
            }
            *per_cat.entry(cat_key).or_insert(0) += 1;
            used_keys.insert(key.clone());
            selected.push(RecommendedTerm {
                term: entry.display.clone(),
                category: cat,
                document_count: entry.document_count,
            });
        }

        for (key, entry) in ranked {
            if selected.len() >= limit {
                break;
            }
            if used_keys.contains(&key) {
                continue;
            }
            selected.push(RecommendedTerm {
                category: dominant_category(&entry),
                term: entry.display,
                document_count: entry.document_count,
            });
        }
        return selected;
    }

    ranked
        .into_iter()
        .take(limit)
        .map(|(_, entry)| RecommendedTerm {
            category: dominant_category(&entry),
            term: entry.display,
            document_count: entry.document_count,
        })
        .collect()
}

#[derive(Default)]
struct CorpusAcc {
    label: String,
    categories: BTreeMap<String, usize>,
    topics: BTreeMap<String, usize>,
    languages: BTreeSet<String>,
    versions: BTreeMap<String, usize>,
    toolchain: Option<String>,
    source_url: Option<String>,
    license: Option<String>,
    document_count: usize,
    packages: BTreeMap<(String, String), usize>,
    tags: BTreeSet<String>,
}

/// Aggregates visible documents into a deterministic corpus/version catalog under `principal`'s ACL.
#[must_use]
pub fn build_corpus_catalog<'a, I>(
    docs: I,
    principal: &Principal,
) -> (usize, Vec<CorpusCatalogEntry>)
where
    I: IntoIterator<Item = &'a Document>,
{
    let mut total_documents = 0usize;
    let mut groups: BTreeMap<String, CorpusAcc> = BTreeMap::new();

    for doc in docs {
        if !principal.allows(&doc.acl_scope) {
            continue;
        }
        total_documents += 1;

        let spec = classify_corpus(doc);
        let entry = groups
            .entry(spec.corpus_id.clone())
            .or_insert_with(|| CorpusAcc {
                label: spec.label.clone(),
                toolchain: spec.toolchain.clone(),
                source_url: spec.source_url.clone(),
                license: spec.license.clone(),
                ..CorpusAcc::default()
            });

        entry.document_count += 1;
        *entry.categories.entry(doc.category.clone()).or_insert(0) += 1;
        if let Some(topic) = doc.topic.as_deref().filter(|t| !t.trim().is_empty()) {
            *entry.topics.entry(topic.to_string()).or_insert(0) += 1;
        }
        if !doc.language.trim().is_empty() {
            entry.languages.insert(doc.language.clone());
        }
        if !doc.version.trim().is_empty() {
            *entry.versions.entry(doc.version.clone()).or_insert(0) += 1;
        }
        if entry.toolchain.is_none() {
            entry.toolchain = spec.toolchain.clone();
        }
        if entry.source_url.is_none() {
            entry.source_url = spec.source_url.clone();
        }
        if entry.license.is_none() {
            entry.license = spec.license.clone();
        }
        if let Some((pkg_name, pkg_ver)) = extract_package_version(doc, &spec.corpus_id) {
            *entry.packages.entry((pkg_name, pkg_ver)).or_insert(0) += 1;
        }
        for tag in &doc.tags {
            let trimmed = tag.trim();
            if !trimmed.is_empty() && entry.tags.len() < 12 {
                entry.tags.insert(trimmed.to_string());
            }
        }
    }

    let corpora = groups
        .into_iter()
        .map(|(corpus_id, acc)| {
            let category = acc
                .categories
                .iter()
                .max_by_key(|(_, count)| *count)
                .map_or_else(|| "guide".to_string(), |(cat, _)| cat.clone());
            let topic = acc
                .topics
                .iter()
                .max_by_key(|(_, count)| *count)
                .map(|(t, _)| t.clone());
            let language = if acc.languages.len() == 1 {
                acc.languages.iter().next().cloned().unwrap_or_default()
            } else if acc.languages.is_empty() {
                "en".to_string()
            } else {
                acc.languages.iter().cloned().collect::<Vec<_>>().join(",")
            };
            let mut version = acc
                .versions
                .iter()
                .max_by_key(|(_, count)| *count)
                .map_or_else(|| "v1".to_string(), |(v, _)| v.clone());
            let packages: Vec<CorpusPackageVersion> = acc
                .packages
                .into_iter()
                .map(|((name, ver), document_count)| CorpusPackageVersion {
                    name,
                    version: ver,
                    document_count,
                })
                .collect();
            if corpus_id == "rust-crates" && packages.len() > 1 {
                version = format!("{} crates", packages.len());
            }

            CorpusCatalogEntry {
                corpus_id,
                label: acc.label,
                category,
                topic,
                language,
                version,
                toolchain: acc.toolchain,
                source_url: acc.source_url,
                license: acc.license,
                document_count: acc.document_count,
                packages,
                tags: acc.tags.into_iter().collect(),
            }
        })
        .collect();

    (total_documents, corpora)
}

struct CorpusSpec {
    corpus_id: String,
    label: String,
    toolchain: Option<String>,
    source_url: Option<String>,
    license: Option<String>,
}

fn corpus_slug_from_path(raw: &str) -> Option<&str> {
    let trimmed = raw
        .trim_start_matches("file:///")
        .trim_start_matches("design://")
        .trim_start_matches('/');
    let rest = trimmed.strip_prefix("samples/").unwrap_or(trimmed);
    rest.split('/').next().filter(|seg| !seg.is_empty())
}

fn classify_corpus(doc: &Document) -> CorpusSpec {
    let path_candidate = doc
        .sample_path
        .as_deref()
        .unwrap_or(doc.source_uri.as_str());
    let slug = corpus_slug_from_path(path_candidate).unwrap_or_default();

    if doc.source_id.as_str() == "dok3search-manual"
        || path_candidate.ends_with("dok3search-manual.md")
    {
        return CorpusSpec {
            corpus_id: "dok3search-manual".to_string(),
            label: "dok3search Product & Operations Manual".to_string(),
            toolchain: None,
            source_url: Some("https://github.com/LeeYudok/dok3search".to_string()),
            license: Some("MIT".to_string()),
        };
    }

    if doc.source_id.as_str() == "dok3search-design" || slug == "design" {
        return CorpusSpec {
            corpus_id: "dok3search-design".to_string(),
            label: "dok3search Architecture & Contracts (dok3search-design)".to_string(),
            toolchain: None,
            source_url: Some("https://github.com/LeeYudok/dok3search-design".to_string()),
            license: Some("MIT".to_string()),
        };
    }

    match slug {
        "rust-std" => CorpusSpec {
            corpus_id: "rust-std".to_string(),
            label: "Rust Standard Library (std/core/alloc)".to_string(),
            toolchain: (!doc.version.trim().is_empty()).then(|| doc.version.clone()),
            source_url: Some("https://doc.rust-lang.org/std/".to_string()),
            license: Some("MIT OR Apache-2.0".to_string()),
        },
        "rust-crates" => CorpusSpec {
            corpus_id: "rust-crates".to_string(),
            label: "Rust Workspace & Dependency Crates (cargo doc)".to_string(),
            toolchain: None,
            source_url: None,
            license: Some("MIT OR Apache-2.0".to_string()),
        },
        "rustc-error-codes" => CorpusSpec {
            corpus_id: "rustc-error-codes".to_string(),
            label: "Rust Compiler Error Index (rustc --explain)".to_string(),
            toolchain: (!doc.version.trim().is_empty()).then(|| doc.version.clone()),
            source_url: Some("https://doc.rust-lang.org/error_codes/".to_string()),
            license: Some("MIT OR Apache-2.0".to_string()),
        },
        "rust-reference" => CorpusSpec {
            corpus_id: "rust-reference".to_string(),
            label: "The Rust Reference".to_string(),
            toolchain: (!doc.version.trim().is_empty()).then(|| doc.version.clone()),
            source_url: Some("https://doc.rust-lang.org/reference/".to_string()),
            license: Some("MIT OR Apache-2.0".to_string()),
        },
        "cargo-book" => CorpusSpec {
            corpus_id: "cargo-book".to_string(),
            label: "The Cargo Book".to_string(),
            toolchain: (!doc.version.trim().is_empty()).then(|| doc.version.clone()),
            source_url: Some("https://doc.rust-lang.org/cargo/".to_string()),
            license: Some("MIT OR Apache-2.0".to_string()),
        },
        "rust-book" => CorpusSpec {
            corpus_id: "rust-book".to_string(),
            label: "The Rust Programming Language".to_string(),
            toolchain: (!doc.version.trim().is_empty()).then(|| doc.version.clone()),
            source_url: Some("https://doc.rust-lang.org/book/".to_string()),
            license: Some("MIT OR Apache-2.0".to_string()),
        },
        "comprehensive-rust" => CorpusSpec {
            corpus_id: "comprehensive-rust".to_string(),
            label: "Comprehensive Rust (한국어)".to_string(),
            toolchain: None,
            source_url: Some("https://google.github.io/comprehensive-rust/ko/".to_string()),
            license: Some("Apache-2.0 AND CC-BY-4.0".to_string()),
        },
        "go-reference" => CorpusSpec {
            corpus_id: "go-reference".to_string(),
            label: "Go Official Reference & Practice Guides".to_string(),
            toolchain: (!doc.version.trim().is_empty()).then(|| doc.version.clone()),
            source_url: Some("https://go.dev/doc/".to_string()),
            license: Some("CC-BY-4.0 AND BSD-3-Clause".to_string()),
        },
        "kordoc" => CorpusSpec {
            corpus_id: "kordoc".to_string(),
            label: "kordoc (HWP·HWPX·PDF→Markdown Parser Source)".to_string(),
            toolchain: None,
            source_url: Some("https://github.com/LeeYudok/kordoc".to_string()),
            license: Some("MIT".to_string()),
        },
        "polyglot-samples" => CorpusSpec {
            corpus_id: "polyglot-samples".to_string(),
            label: "Polyglot Enterprise & Web Code Samples (Java·JSP·JS·TS·React·Next.js)"
                .to_string(),
            toolchain: None,
            source_url: Some("https://github.com/LeeYudok/dok3search".to_string()),
            license: Some("MIT".to_string()),
        },
        _ => CorpusSpec {
            corpus_id: doc.source_id.as_str().to_string(),
            label: doc.source.clone(),
            toolchain: None,
            source_url: None,
            license: None,
        },
    }
}

fn extract_package_version(doc: &Document, corpus_id: &str) -> Option<(String, String)> {
    if !matches!(corpus_id, "rust-std" | "rust-crates") {
        return None;
    }
    let path = doc.sample_path.as_deref().unwrap_or("");
    let idx = path.find("chapters/")?;
    let after = &path[idx + "chapters/".len()..];
    let pkg = after.split('/').next()?.trim();
    if pkg.is_empty() {
        return None;
    }
    let version = if doc.version.trim().is_empty() {
        "unknown".to_string()
    } else {
        doc.version.clone()
    };
    Some((pkg.to_string(), version))
}
