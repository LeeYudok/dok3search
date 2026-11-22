//! Administration DTOs for background indexing jobs, classification, and source management.

use serde::{Deserialize, Serialize};

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
