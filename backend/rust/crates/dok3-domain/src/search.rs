//! Search query, response, snapshot, and health readiness contracts.

use serde::{Deserialize, Serialize};

use crate::acl::AclScope;
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
    pub source: String,
    pub tag: String,
    pub tags: Vec<String>,
    pub date: Option<String>,
    pub sample_path: Option<String>,
    pub acl_scope: AclScope,
    pub has_paired_formats: bool,
    pub matched_section_id: Option<SectionId>,
    pub score: f32,
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
