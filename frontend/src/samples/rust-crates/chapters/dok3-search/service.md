# Module dok3-search::service

## Overview

Application service layer for search, document reading, and readiness checks.

## Items

- `SearchService` — Coordinates search and document-retrieval use cases over domain ports.

### struct dok3-search::service::SearchService

```rust
pub struct SearchService { /* private fields */ }
```

Coordinates search and document-retrieval use cases over domain ports.

**Methods**

- `pub fn new( search_port: Arc<dyn SearchPort>, doc_store: Arc<dyn DocumentStorePort>, ) -> Self`
- `pub fn search( &self, principal: &Principal, req: &SearchRequest, ) -> Result<SearchResponse, Dok3Error>`
- `pub fn get_document( &self, principal: &Principal, id: &DocumentId, ) -> Result<DocumentDetail, Dok3Error>`
- `pub fn get_section( &self, principal: &Principal, req: &SectionReadRequest, ) -> Result<SectionReadResponse, Dok3Error>`
- `pub fn health_ready(&self, mode: impl Into<String>) -> HealthReadyResponse`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
