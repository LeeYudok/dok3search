//! Administration DTOs for background indexing jobs, classification, and source management.

use dok3_core::Dok3Error;
use serde::{Deserialize, Serialize};

use crate::ids::DocumentId;

/// Top-level document classification categories enforced by REST and search taxonomy.
pub const ALLOWED_CLASSIFICATION_CATEGORIES: [&str; 5] =
    ["guide", "manual", "code", "policy", "design"];
/// Maximum number of tags or keywords per document classification.
pub const MAX_CLASSIFICATION_TERMS: usize = 12;
/// Maximum character length of a classification summary.
pub const MAX_CLASSIFICATION_SUMMARY_CHARS: usize = 500;
/// Maximum byte length of a source identifier slug.
pub const MAX_SOURCE_ID_LEN: usize = 64;

/// Indexing run mode for manual or scheduled jobs.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IndexJobMode {
    #[default]
    Incremental,
    Full,
}

/// Execution lifecycle state of an indexing job.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IndexJobState {
    Pending,
    Running,
    Succeeded,
    Failed,
}

/// Persistent audit record for a startup, periodic, or manual indexing run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexJobRecord {
    pub job_id: String,
    pub source_id: Option<String>,
    pub trigger: String,
    pub mode: IndexJobMode,
    pub state: IndexJobState,
    pub started_at_ms: u64,
    pub finished_at_ms: Option<u64>,
    pub indexed_documents: usize,
    pub rebuilt: bool,
    pub active_generation: Option<String>,
    pub active_epoch: u64,
    pub skipped_files: Vec<String>,
    pub error: Option<String>,
}

/// Request payload for triggering a manual indexing run (`POST /api/v1/index/jobs`).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriggerIndexJobRequest {
    #[serde(default)]
    pub source_id: Option<String>,
    #[serde(default)]
    pub mode: IndexJobMode,
}

/// Response returned by `GET /api/v1/index/jobs`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexJobListResponse {
    pub jobs: Vec<IndexJobRecord>,
    pub active_generation: Option<String>,
    pub active_epoch: u64,
    pub refresh_interval_secs: Option<u64>,
}

/// Origin of the active document classification metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClassificationSource {
    Rule,
    Manual,
}

/// Stored classification metadata associated with a document without altering original bytes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentClassification {
    pub document_id: DocumentId,
    pub summary: String,
    pub category: String,
    pub tags: Vec<String>,
    pub keywords: Vec<String>,
    pub source: ClassificationSource,
    pub manual_locked: bool,
    pub updated_at_ms: u64,
    pub content_hash: String,
}

/// Request payload for `PATCH /api/v1/documents/{id}/classification`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateClassificationRequest {
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub keywords: Option<Vec<String>>,
    #[serde(default)]
    pub reset_to_rule: bool,
}

/// Validates and normalizes a classification category against [`ALLOWED_CLASSIFICATION_CATEGORIES`].
pub fn validate_classification_category(raw: &str) -> Result<String, Dok3Error> {
    let normalized = raw.trim().to_ascii_lowercase();
    if ALLOWED_CLASSIFICATION_CATEGORIES.contains(&normalized.as_str()) {
        Ok(normalized)
    } else {
        Err(Dok3Error::InvalidInput(format!(
            "unsupported classification category '{}' (allowed: {})",
            raw.trim(),
            ALLOWED_CLASSIFICATION_CATEGORIES.join(", ")
        )))
    }
}

/// Validates and trims a classification summary (`<= MAX_CLASSIFICATION_SUMMARY_CHARS`).
pub fn validate_classification_summary(raw: &str) -> Result<String, Dok3Error> {
    let trimmed = raw.trim();
    if trimmed.chars().count() > MAX_CLASSIFICATION_SUMMARY_CHARS {
        return Err(Dok3Error::InvalidInput(format!(
            "classification summary exceeds {MAX_CLASSIFICATION_SUMMARY_CHARS} characters"
        )));
    }
    Ok(trimmed.to_string())
}

/// Trims, removes empty entries, deduplicates case-insensitively, and enforces [`MAX_CLASSIFICATION_TERMS`].
pub fn normalize_classification_terms(
    field_name: &str,
    items: &[String],
) -> Result<Vec<String>, Dok3Error> {
    let mut normalized: Vec<String> = Vec::new();
    for item in items {
        let trimmed = item.trim();
        if trimmed.is_empty() {
            continue;
        }
        if !normalized
            .iter()
            .any(|existing| existing.eq_ignore_ascii_case(trimmed))
        {
            normalized.push(trimmed.to_string());
        }
    }
    if normalized.len() > MAX_CLASSIFICATION_TERMS {
        return Err(Dok3Error::InvalidInput(format!(
            "classification {field_name} exceeds maximum of {MAX_CLASSIFICATION_TERMS} items"
        )));
    }
    Ok(normalized)
}

/// Durable registered local filesystem source definition and sync status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegisteredSource {
    pub source_id: String,
    pub label: String,
    pub path: String,
    pub default_category: String,
    pub default_tag: String,
    pub managed_by_env: bool,
    pub created_at_ms: u64,
    pub last_synced_at_ms: Option<u64>,
    pub document_count: usize,
    pub last_error: Option<String>,
}

/// Request payload for `POST /api/v1/sources`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateSourceRequest {
    pub source_id: String,
    pub label: String,
    pub path: String,
    #[serde(default)]
    pub default_category: Option<String>,
    #[serde(default)]
    pub default_tag: Option<String>,
}

/// Response returned by `GET /api/v1/sources`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceListResponse {
    pub sources: Vec<RegisteredSource>,
}

/// Response returned by `DELETE /api/v1/sources/{id}`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteSourceResponse {
    pub source_id: String,
    pub deleted_documents: usize,
    pub active_generation: Option<String>,
    pub active_epoch: u64,
}

/// Validates and normalizes a source identifier slug against `^[a-z0-9][a-z0-9_-]{0,63}$`.
pub fn validate_source_id_slug(raw: &str) -> Result<String, Dok3Error> {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed.len() > MAX_SOURCE_ID_LEN {
        return Err(Dok3Error::InvalidInput(format!(
            "source_id must be 1..={MAX_SOURCE_ID_LEN} characters matching ^[a-z0-9][a-z0-9_-]{{0,63}}$"
        )));
    }
    let bytes = trimmed.as_bytes();
    let valid_first = bytes[0].is_ascii_lowercase() || bytes[0].is_ascii_digit();
    let valid_rest = bytes[1..]
        .iter()
        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || *b == b'-' || *b == b'_');
    if !valid_first || !valid_rest {
        return Err(Dok3Error::InvalidInput(format!(
            "invalid source_id '{trimmed}' (must match ^[a-z0-9][a-z0-9_-]{{0,63}}$)"
        )));
    }
    Ok(trimmed.to_string())
}
