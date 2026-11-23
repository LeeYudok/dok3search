//! Agent Communication Protocol (ACP) adapter delegating to [`SearchService`].

use std::sync::Arc;

use dok3_core::Dok3Error;
use dok3_domain::{
    DocumentDetail, DocumentId, Principal, SearchRequest, SearchResponse, SectionId,
    SectionReadRequest, SectionReadResponse,
};
use dok3_search::SearchService;
use serde::{Deserialize, Serialize};

/// ACP operation envelope routed through [`AcpAdapter`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum AcpRequest {
    Search {
        query: String,
        #[serde(default = "default_limit")]
        limit: usize,
    },
    GetDocument {
        document_id: String,
    },
    GetSection {
        document_id: String,
        section_id: String,
    },
}

const fn default_limit() -> usize {
    10
}

/// ACP operation response payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AcpResponse {
    Search(SearchResponse),
    Document(Box<DocumentDetail>),
    Section(SectionReadResponse),
}

/// Stateless ACP adapter enforcing per-call [`Principal`] ACL through [`SearchService`].
#[derive(Clone)]
pub struct AcpAdapter {
    search: Arc<SearchService>,
}

impl AcpAdapter {
    #[must_use]
    pub fn new(search: Arc<SearchService>) -> Self {
        Self { search }
    }

    pub fn handle(&self, principal: &Principal, req: AcpRequest) -> Result<AcpResponse, Dok3Error> {
        match req {
            AcpRequest::Search { query, limit } => {
                let resp = self.search.search(
                    principal,
                    &SearchRequest {
                        query,
                        limit,
                        ..SearchRequest::default()
                    },
                )?;
                Ok(AcpResponse::Search(resp))
            }
            AcpRequest::GetDocument { document_id } => {
                let doc = self
                    .search
                    .get_document(principal, &DocumentId::new(document_id))?;
                Ok(AcpResponse::Document(Box::new(doc)))
            }
            AcpRequest::GetSection {
                document_id,
                section_id,
            } => {
                let section = self.search.get_section(
                    principal,
                    &SectionReadRequest {
                        document_id: DocumentId::new(document_id),
                        section_id: SectionId::new(section_id),
                        cursor: None,
                        max_chars: 4000,
                    },
                )?;
                Ok(AcpResponse::Section(section))
            }
        }
    }
}
