# Module dok3-domain::ports

## Overview

Hexagonal port traits decoupled from concrete engines and web frameworks.

## Items

- `ChangeJournalPort` — Port for durable change-journal persistence and catch-up replay.
- `ClassificationPort` — Port for reading and updating document classification without altering original bytes.
- `DocumentParserPort` — Port for format-specific document parsing without touching Tantivy or storage directly.
- `DocumentStorePort` — Port for retrieving authorized document details and bounded sections.
- `IndexJobPort` — Port for inspecting and triggering durable indexing jobs.
- `SearchPort` — Port for executing ACL-scoped document search queries.
- `SourceRegistryPort` — Port for listing, registering, and deleting local document sources without altering disk files.

### trait dok3-domain::ports::ChangeJournalPort

```rust
pub trait ChangeJournalPort: Send + Sync {
    // Required methods
    fn append(&self, change: ChangeEvent) -> Result<Sequence, Dok3Error>;
    fn read_after(
        &self,
        seq: Sequence,
        limit: usize,
    ) -> Result<Vec<ChangeEvent>, Dok3Error>;
}
```

Port for durable change-journal persistence and catch-up replay.

**Methods**

- `fn append(&self, change: ChangeEvent) -> Result<Sequence, Dok3Error>`
- `fn read_after( &self, seq: Sequence, limit: usize, ) -> Result<Vec<ChangeEvent>, Dok3Error>`

### trait dok3-domain::ports::ClassificationPort

```rust
pub trait ClassificationPort: Send + Sync {
    // Required methods
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
```

Port for reading and updating document classification without altering original bytes.

**Methods**

- `fn get_classification( &self, principal: &Principal, id: &DocumentId, ) -> Result<DocumentClassification, Dok3Error>`
- `fn update_classification( &self, principal: &Principal, id: &DocumentId, req: &UpdateClassificationRequest, ) -> Result<DocumentClassification, Dok3Error>`

### trait dok3-domain::ports::DocumentParserPort

```rust
pub trait DocumentParserPort: Send + Sync {
    // Required methods
    fn supports(&self, media_type: &str, filename: &str) -> bool;
    fn parse(
        &self,
        filename: &str,
        content: &[u8],
    ) -> Result<ParsedDocument, Dok3Error>;
}
```

Port for format-specific document parsing without touching Tantivy or storage directly.

**Methods**

- `fn supports(&self, media_type: &str, filename: &str) -> bool`
- `fn parse( &self, filename: &str, content: &[u8], ) -> Result<ParsedDocument, Dok3Error>`

### trait dok3-domain::ports::DocumentStorePort

```rust
pub trait DocumentStorePort: Send + Sync {
    // Required methods
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
```

Port for retrieving authorized document details and bounded sections.

**Methods**

- `fn get_document( &self, principal: &Principal, id: &DocumentId, ) -> Result<DocumentDetail, Dok3Error>`
- `fn get_section( &self, principal: &Principal, req: &SectionReadRequest, ) -> Result<SectionReadResponse, Dok3Error>`
- `fn document_count(&self) -> usize`

### trait dok3-domain::ports::IndexJobPort

```rust
pub trait IndexJobPort: Send + Sync {
    // Required methods
    fn list_jobs(&self) -> Result<IndexJobListResponse, Dok3Error>;
    fn get_job(&self, job_id: &str) -> Result<IndexJobRecord, Dok3Error>;
    fn trigger_job(
        &self,
        req: &TriggerIndexJobRequest,
    ) -> Result<IndexJobRecord, Dok3Error>;
}
```

Port for inspecting and triggering durable indexing jobs.

**Methods**

- `fn list_jobs(&self) -> Result<IndexJobListResponse, Dok3Error>`
- `fn get_job(&self, job_id: &str) -> Result<IndexJobRecord, Dok3Error>`
- `fn trigger_job( &self, req: &TriggerIndexJobRequest, ) -> Result<IndexJobRecord, Dok3Error>`

### trait dok3-domain::ports::SearchPort

```rust
pub trait SearchPort: Send + Sync {
    // Required methods
    fn search(
        &self,
        principal: &Principal,
        req: &SearchRequest,
    ) -> Result<SearchResponse, Dok3Error>;
    fn snapshot(&self) -> SearchSnapshotMeta;
}
```

Port for executing ACL-scoped document search queries.

**Methods**

- `fn search( &self, principal: &Principal, req: &SearchRequest, ) -> Result<SearchResponse, Dok3Error>`
- `fn snapshot(&self) -> SearchSnapshotMeta`

### trait dok3-domain::ports::SourceRegistryPort

```rust
pub trait SourceRegistryPort: Send + Sync {
    // Required methods
    fn list_sources(&self) -> Result<SourceListResponse, Dok3Error>;
    fn register_source(
        &self,
        req: &CreateSourceRequest,
    ) -> Result<RegisteredSource, Dok3Error>;
    fn delete_source(
        &self,
        source_id: &str,
    ) -> Result<DeleteSourceResponse, Dok3Error>;
}
```

Port for listing, registering, and deleting local document sources without altering disk files.

**Methods**

- `fn list_sources(&self) -> Result<SourceListResponse, Dok3Error>`
- `fn register_source( &self, req: &CreateSourceRequest, ) -> Result<RegisteredSource, Dok3Error>`
- `fn delete_source( &self, source_id: &str, ) -> Result<DeleteSourceResponse, Dok3Error>`
