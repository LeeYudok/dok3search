//! Hexagonal port traits decoupled from concrete engines and web frameworks.

use dok3_core::Dok3Error;

use crate::acl::Principal;
use crate::admin::{
    CreateSourceRequest, DeleteSourceResponse, DocumentClassification, IndexJobListResponse,
    IndexJobRecord, RegisteredSource, SourceListResponse, TriggerIndexJobRequest,
    UpdateClassificationRequest,
};
use crate::change::ChangeEvent;
use crate::document::{DocumentDetail, ParsedDocument, SectionReadRequest, SectionReadResponse};
use crate::ids::{DocumentId, Sequence};
use crate::search::{CorpusCatalogEntry, SearchRequest, SearchResponse, SearchSnapshotMeta};

/// Port for executing ACL-scoped document search queries.
pub trait SearchPort: Send + Sync {
    fn search(
        &self,
        principal: &Principal,
        req: &SearchRequest,
    ) -> Result<SearchResponse, Dok3Error>;
    fn snapshot(&self) -> SearchSnapshotMeta;
}

/// Original document bytes and safe download filename returned by [`DocumentStorePort::get_original`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OriginalDocumentPayload {
    pub filename: String,
    pub bytes: Vec<u8>,
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
    fn get_original(
        &self,
        principal: &Principal,
        id: &DocumentId,
    ) -> Result<OriginalDocumentPayload, Dok3Error> {
        let detail = self.get_document(principal, id)?;
        let filename = detail
            .sample_path
            .as_deref()
            .and_then(|p| p.rsplit('/').next())
            .filter(|s| !s.trim().is_empty())
            .unwrap_or("document.txt")
            .to_string();
        Ok(OriginalDocumentPayload {
            filename,
            bytes: detail.body.into_bytes(),
        })
    }
    fn document_count(&self) -> usize;
    fn list_corpora(
        &self,
        principal: &Principal,
    ) -> Result<(usize, Vec<CorpusCatalogEntry>), Dok3Error> {
        let _ = principal;
        Ok((0, Vec::new()))
    }
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

/// Port for listing, registering, and deleting local document sources without altering disk files.
pub trait SourceRegistryPort: Send + Sync {
    fn list_sources(&self) -> Result<SourceListResponse, Dok3Error>;
    fn register_source(&self, req: &CreateSourceRequest) -> Result<RegisteredSource, Dok3Error>;
    fn delete_source(&self, source_id: &str) -> Result<DeleteSourceResponse, Dok3Error>;
}
