# Module dok3-domain::search

## Overview

Search query, response, snapshot, and health readiness contracts.

## Items

- `CategoryCount` — Aggregated hit count for a single category computed under the caller’s effective ACL.
- `HealthReadyResponse` — Service readiness payload returned by /health/ready.
- `SearchFilters` — Structured query filters applied alongside ACL scope enforcement.
- `SearchHit` — Individual search hit returned for a query.
- `SearchRequest` — Search request contract. requested_scope is advisory only and cannot escalate ACLs.
- `SearchResponse` — Search response payload containing paginated hits, category facets, and snapshot metadata.
- `SearchSnapshotMeta` — Metadata identifying the generation and consistency boundary of a search response.
- `ReaderState` — Lifecycle state of the search reader associated with a snapshot.

### enum dok3-domain::search::ReaderState

```rust
pub enum ReaderState {
    Ready,
    Warming,
    Degraded,
}
```

Lifecycle state of the search reader associated with a snapshot.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-domain::search::CategoryCount

```rust
pub struct CategoryCount {
    pub id: String,
    pub label: String,
    pub count: usize,
}
```

Aggregated hit count for a single category computed under the caller’s effective ACL.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-domain::search::HealthReadyResponse

```rust
pub struct HealthReadyResponse {
    pub status: String,
    pub mode: String,
    pub document_count: usize,
    pub snapshot: SearchSnapshotMeta,
}
```

Service readiness payload returned by `/health/ready`.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-domain::search::SearchFilters

```rust
pub struct SearchFilters {
    pub category: Option<String>,
    pub language: Option<String>,
    pub source_id: Option<SourceId>,
    pub tags: Vec<String>,
}
```

Structured query filters applied alongside ACL scope enforcement.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-domain::search::SearchHit

```rust
pub struct SearchHit {
    pub document_id: DocumentId,
    pub revision_id: RevisionId,
    pub title: String,
    pub summary: String,
    pub snippet: String,
    pub category: String,
    pub topic: Option<String>,
    pub source: String,
    pub tag: String,
    pub tags: Vec<String>,
    pub date: Option<String>,
    pub sample_path: Option<String>,
    pub acl_scope: AclScope,
    pub has_paired_formats: bool,
    pub matched_section_id: Option<SectionId>,
    pub score: f32,
}
```

Individual search hit returned for a query.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-domain::search::SearchRequest

```rust
pub struct SearchRequest {
    pub query: String,
    pub filters: SearchFilters,
    pub page: usize,
    pub limit: usize,
    pub include_snippets: bool,
    pub requested_scope: Option<AclScope>,
}
```

Search request contract. `requested_scope` is advisory only and cannot escalate ACLs.

**Methods**

- `pub fn normalized(self) -> Self`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-domain::search::SearchResponse

```rust
pub struct SearchResponse {
    pub query: String,
    pub page: usize,
    pub limit: usize,
    pub total_hits: usize,
    pub total_pages: usize,
    pub total_unfiltered_hits: usize,
    pub category_counts: Vec<CategoryCount>,
    pub related_searches: Vec<String>,
    pub hits: Vec<SearchHit>,
    pub snapshot: SearchSnapshotMeta,
}
```

Search response payload containing paginated hits, category facets, and snapshot metadata.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-domain::search::SearchSnapshotMeta

```rust
pub struct SearchSnapshotMeta {
    pub generation_id: GenerationId,
    pub epoch: u64,
    pub schema_version: u32,
    pub acl_policy_version: u32,
    pub applied_change_sequence: Sequence,
    pub reader_state: ReaderState,
}
```

Metadata identifying the generation and consistency boundary of a search response.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`
