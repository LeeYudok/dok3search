//! Hexagonal port traits decoupled from concrete engines and web frameworks.

use dok3_core::Dok3Error;

use crate::acl::Principal;
use crate::admin::{
    DocumentClassification, IndexJobListResponse, IndexJobRecord, TriggerIndexJobRequest,
    UpdateClassificationRequest,
};
use crate::change::ChangeEvent;
use crate::document::{DocumentDetail, ParsedDocument, SectionReadRequest, SectionReadResponse};
use crate::ids::{DocumentId, Sequence};
use crate::search::{SearchRequest, SearchResponse, SearchSnapshotMeta};

/// Port for executing ACL-scoped document search queries.
pub trait SearchPort: Send + Sync {
    fn search(
        &self,
        principal: &Principal,
        req: &SearchRequest,
    ) -> Result<SearchResponse, Dok3Error>;
    fn snapshot(&self) -> SearchSnapshotMeta;
}

/// Port for retrieving authorized document details and bounded sections.
pub trait DocumentStorePort: Send + Sync {
    fn get_document(
        &self,
        principal: &Principal,
        id: &DocumentId,
    ) -> Result<DocumentDetail, Dok3Error>;
    fn get_section(
        &self,
        principal: &Principal,
        req: &SectionReadRequest,
    ) -> Result<SectionReadResponse, Dok3Error>;
    fn document_count(&self) -> usize;
}

/// Port for format-specific document parsing without touching Tantivy or storage directly.
pub trait DocumentParserPort: Send + Sync {
    fn supports(&self, media_type: &str, filename: &str) -> bool;
    fn parse(&self, filename: &str, content: &[u8]) -> Result<ParsedDocument, Dok3Error>;
}

/// Port for durable change-journal persistence and catch-up replay.
pub trait ChangeJournalPort: Send + Sync {
    fn append(&self, change: ChangeEvent) -> Result<Sequence, Dok3Error>;
    fn read_after(&self, seq: Sequence, limit: usize) -> Result<Vec<ChangeEvent>, Dok3Error>;
}

/// Port for inspecting and triggering durable indexing jobs.
pub trait IndexJobPort: Send + Sync {
    fn list_jobs(&self) -> Result<IndexJobListResponse, Dok3Error>;
    fn get_job(&self, job_id: &str) -> Result<IndexJobRecord, Dok3Error>;
    fn trigger_job(&self, req: &TriggerIndexJobRequest) -> Result<IndexJobRecord, Dok3Error>;
}

/// Port for reading and updating document classification without altering original bytes.
pub trait ClassificationPort: Send + Sync {
    fn get_classification(
        &self,
        principal: &Principal,
        id: &DocumentId,
    ) -> Result<DocumentClassification, Dok3Error>;
    fn update_classification(
        &self,
        principal: &Principal,
        id: &DocumentId,
        req: &UpdateClassificationRequest,
    ) -> Result<DocumentClassification, Dok3Error>;
}
