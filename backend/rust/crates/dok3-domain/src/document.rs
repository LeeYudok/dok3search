//! Document, section, and section-reading contracts.

use serde::{Deserialize, Serialize};

use crate::acl::AclScope;
use crate::ids::{DocumentId, RevisionId, SectionId, SourceId};

/// Default character limit when reading a single document section.
pub const DEFAULT_SECTION_MAX_CHARS: usize = 12_000;
/// Maximum allowed character limit for a single section-read response.
pub const MAX_SECTION_MAX_CHARS: usize = 100_000;

const fn default_section_max_chars() -> usize {
    DEFAULT_SECTION_MAX_CHARS
}

/// Full section record owned by an indexed or stored document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentSection {
    pub section_id: SectionId,
    pub heading: String,
    pub anchor: String,
    pub order: usize,
    pub line_start: usize,
    pub line_end: usize,
    pub body: String,
}

/// Lightweight section metadata exposed in document detail views and TOCs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SectionSummary {
    pub section_id: SectionId,
    pub heading: String,
    pub anchor: String,
    pub order: usize,
    pub line_start: usize,
    pub line_end: usize,
}

impl From<&DocumentSection> for SectionSummary {
    fn from(section: &DocumentSection) -> Self {
        Self {
            section_id: section.section_id.clone(),
            heading: section.heading.clone(),
            anchor: section.anchor.clone(),
            order: section.order,
            line_start: section.line_start,
            line_end: section.line_end,
        }
    }
}

/// Paired Markdown and YAML document IDs for design entrypoint documents.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentFormatLinks {
    pub markdown_id: DocumentId,
    pub yaml_id: DocumentId,
}

/// Canonical domain representation of an indexed knowledge document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Document {
    pub id: DocumentId,
    pub revision_id: RevisionId,
    pub source_id: SourceId,
    pub source: String,
    pub title: String,
    pub summary: String,
    pub category: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    pub tag: String,
    pub tags: Vec<String>,
    pub keywords: Vec<String>,
    pub language: String,
    pub version: String,
    pub date: Option<String>,
    pub source_uri: String,
    pub sample_path: Option<String>,
    pub acl_scope: AclScope,
    pub content_hash: String,
    pub body: String,
    pub markdown: Option<String>,
    pub sections: Vec<DocumentSection>,
}

/// Detailed document payload returned by document retrieval endpoints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentDetail {
    pub id: DocumentId,
    pub revision_id: RevisionId,
    pub source_id: SourceId,
    pub source: String,
    pub title: String,
    pub summary: String,
    pub category: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    pub tag: String,
    pub tags: Vec<String>,
    pub keywords: Vec<String>,
    pub language: String,
    pub version: String,
    pub date: Option<String>,
    pub source_uri: String,
    pub sample_path: Option<String>,
    pub acl_scope: AclScope,
    pub body: String,
    pub markdown: Option<String>,
    pub sections: Vec<SectionSummary>,
    pub formats: Option<DocumentFormatLinks>,
}

/// Output produced by format-specific document parsers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParsedDocument {
    pub title: String,
    pub sections: Vec<DocumentSection>,
    pub normalized_text: String,
    pub parse_warnings: Vec<String>,
}

/// Request to fetch a bounded slice of an individual section.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SectionReadRequest {
    pub document_id: DocumentId,
    pub section_id: SectionId,
    #[serde(default)]
    pub cursor: Option<usize>,
    #[serde(default = "default_section_max_chars")]
    pub max_chars: usize,
}

impl SectionReadRequest {
    /// Clamps `max_chars` into `[1, MAX_SECTION_MAX_CHARS]`.
    #[must_use]
    pub fn normalized(mut self) -> Self {
        self.max_chars = self.max_chars.clamp(1, MAX_SECTION_MAX_CHARS);
        self
    }
}

/// Citation metadata returned alongside section text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SectionCitation {
    pub source_uri: String,
    pub section: SectionId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_start: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_end: Option<usize>,
}

/// Paginated, source-preserving section response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SectionReadResponse {
    pub source_revision: RevisionId,
    pub heading_path: Vec<String>,
    pub citation: SectionCitation,
    pub text: String,
    pub next_cursor: Option<usize>,
    pub truncated: bool,
}
