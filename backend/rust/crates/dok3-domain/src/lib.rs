//! Core domain types and port traits (DocumentId, Revision, SectionId, GenerationId).

pub mod acl;
pub mod admin;
pub mod change;
pub mod document;
pub mod ids;
pub mod ports;
pub mod search;

pub use acl::{AclScope, Principal};
pub use admin::{
    IndexJobListResponse, IndexJobMode, IndexJobRecord, IndexJobState, TriggerIndexJobRequest,
};
pub use change::{ChangeEvent, ChangeKind};
pub use document::{
    DEFAULT_SECTION_MAX_CHARS, Document, DocumentDetail, DocumentFormatLinks, DocumentSection,
    MAX_SECTION_MAX_CHARS, ParsedDocument, SectionCitation, SectionReadRequest,
    SectionReadResponse, SectionSummary,
};
pub use ids::{DocumentId, GenerationId, RevisionId, SectionId, Sequence, SourceId};
pub use ports::{
    ChangeJournalPort, DocumentParserPort, DocumentStorePort, IndexJobPort, SearchPort,
};
pub use search::{
    CategoryCount, HealthReadyResponse, ReaderState, SearchFilters, SearchHit, SearchRequest,
    SearchResponse, SearchSnapshotMeta,
};
