# Module dok3-search::in_memory

## Overview

In-memory SearchPort and DocumentStorePort implementation for P0-3 vertical slice.

## Items

- `InMemorySearchRepository` — Thread-safe in-memory repository implementing search and document store ports.

### struct dok3-search::in_memory::InMemorySearchRepository

```rust
pub struct InMemorySearchRepository { /* private fields */ }
```

Thread-safe in-memory repository implementing search and document store ports.

**Methods**

- `pub fn new(documents: Vec<Document>, pairs: Vec<DocumentFormatLinks>) -> Self`
- `pub fn from_seed(design_dir: Option<&Path>) -> Self`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn get_document( &self, principal: &Principal, id: &DocumentId, ) -> Result<DocumentDetail, Dok3Error>`
- `fn get_section( &self, principal: &Principal, req: &SectionReadRequest, ) -> Result<SectionReadResponse, Dok3Error>`
- `fn document_count(&self) -> usize`
- `fn search( &self, principal: &Principal, req: &SearchRequest, ) -> Result<SearchResponse, Dok3Error>`
- `fn snapshot(&self) -> SearchSnapshotMeta`
