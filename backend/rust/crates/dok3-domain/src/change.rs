//! Durable change journal contracts for incremental indexing and generation catch-up.

use serde::{Deserialize, Serialize};

use crate::ids::{DocumentId, RevisionId, Sequence};

/// Mutation type recorded in the durable change journal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeKind {
    Upsert,
    Delete,
}

/// Single ordered change event emitted by connectors or ingestion jobs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeEvent {
    pub sequence: Sequence,
    pub document_id: DocumentId,
    pub revision_id: RevisionId,
    pub kind: ChangeKind,
}
