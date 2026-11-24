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
    ALLOWED_CLASSIFICATION_CATEGORIES, ClassificationSource, CreateSourceRequest,
    DeleteSourceResponse, DocumentClassification, IndexJobListResponse, IndexJobMode,
    IndexJobRecord, IndexJobState, MAX_CLASSIFICATION_SUMMARY_CHARS, MAX_CLASSIFICATION_TERMS,
    MAX_SOURCE_ID_LEN, RegisteredSource, SourceKind, SourceListResponse, SqliteCursorKind,
    SqliteSourceOptions, TriggerIndexJobRequest, UpdateClassificationRequest,
    normalize_classification_terms, validate_classification_category,
    validate_classification_summary, validate_source_id_slug,
};
pub use change::{ChangeEvent, ChangeKind};
pub use document::{
    DEFAULT_SECTION_MAX_CHARS, Document, DocumentDetail, DocumentFormatLinks, DocumentSection,
    MAX_SECTION_MAX_CHARS, ParsedDocument, SectionCitation, SectionReadRequest,
    SectionReadResponse, SectionSummary,
};
pub use ids::{DocumentId, GenerationId, RevisionId, SectionId, Sequence, SourceId};
pub use ports::{
    ChangeJournalPort, ClassificationPort, DocumentParserPort, DocumentStorePort, IndexJobPort,
    OriginalDocumentPayload, SearchPort, SourceRegistryPort,
};
pub use search::{
    CategoryCount, CorpusCatalogEntry, CorpusCatalogResponse, CorpusPackageVersion,
    HealthReadyResponse, ParsedQueryOperators, ReaderState, RecommendedTerm, SearchFilters,
    SearchHit, SearchRecommendationsResponse, SearchRequest, SearchResponse, SearchSnapshotMeta,
    build_corpus_catalog, build_recommended_terms, parse_query_operators,
};
