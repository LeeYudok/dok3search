//! Application service layer for search, document reading, and readiness checks.

use std::sync::Arc;

use dok3_core::Dok3Error;
use dok3_domain::{
    CorpusCatalogResponse, DocumentDetail, DocumentId, DocumentStorePort, HealthReadyResponse,
    Principal, SearchPort, SearchRequest, SearchResponse, SectionReadRequest, SectionReadResponse,
};

const MAX_QUERY_CHARS: usize = 512;

/// Coordinates search and document-retrieval use cases over domain ports.
#[derive(Clone)]
pub struct SearchService {
    search_port: Arc<dyn SearchPort>,
    doc_store: Arc<dyn DocumentStorePort>,
}

impl SearchService {
    /// Creates a `SearchService` backed by the provided ports.
    #[must_use]
    pub fn new(search_port: Arc<dyn SearchPort>, doc_store: Arc<dyn DocumentStorePort>) -> Self {
        Self {
            search_port,
            doc_store,
        }
    }

    /// Executes an ACL-scoped search request after input validation and pagination normalization.
    pub fn search(
        &self,
        principal: &Principal,
        req: &SearchRequest,
    ) -> Result<SearchResponse, Dok3Error> {
        if req.query.chars().count() > MAX_QUERY_CHARS {
            return Err(Dok3Error::InvalidInput(format!(
                "query exceeds maximum length of {MAX_QUERY_CHARS} characters"
            )));
        }
        let normalized = req.clone().normalized();
        self.search_port.search(principal, &normalized)
    }

    /// Retrieves document metadata, body, section TOC, and paired format links if authorized.
    pub fn get_document(
        &self,
        principal: &Principal,
        id: &DocumentId,
    ) -> Result<DocumentDetail, Dok3Error> {
        if id.as_str().trim().is_empty() {
            return Err(Dok3Error::InvalidInput(
                "document_id must not be empty".to_string(),
            ));
        }
        self.doc_store.get_document(principal, id)
    }

    /// Retrieves a bounded, cursor-paginated slice of a single document section if authorized.
    pub fn get_section(
        &self,
        principal: &Principal,
        req: &SectionReadRequest,
    ) -> Result<SectionReadResponse, Dok3Error> {
        if req.document_id.as_str().trim().is_empty() || req.section_id.as_str().trim().is_empty() {
            return Err(Dok3Error::InvalidInput(
                "document_id and section_id must not be empty".to_string(),
            ));
        }
        let normalized = req.clone().normalized();
        self.doc_store.get_section(principal, &normalized)
    }

    /// Reports service readiness, active document count, and current search snapshot metadata.
    #[must_use]
    pub fn health_ready(&self, mode: impl Into<String>) -> HealthReadyResponse {
        HealthReadyResponse {
            status: "ready".to_string(),
            mode: mode.into(),
            document_count: self.doc_store.document_count(),
            snapshot: self.search_port.snapshot(),
        }
    }

    /// Returns the public/authorized corpus and version catalog for the active snapshot.
    pub fn list_corpora(
        &self,
        principal: &Principal,
        mode: impl Into<String>,
    ) -> Result<CorpusCatalogResponse, Dok3Error> {
        let (total_documents, corpora) = self.doc_store.list_corpora(principal)?;
        Ok(CorpusCatalogResponse {
            mode: mode.into(),
            total_documents,
            corpora,
            snapshot: self.search_port.snapshot(),
        })
    }
}
