# Module dok3-domain::admin

## Overview

Administration DTOs for background indexing jobs, classification, and source management.

## Items

- `CreateSourceRequest` — Request payload for POST /api/v1/sources.
- `DeleteSourceResponse` — Response returned by DELETE /api/v1/sources/{id}.
- `DocumentClassification` — Stored classification metadata associated with a document without altering original bytes.
- `IndexJobListResponse` — Response returned by GET /api/v1/index/jobs.
- `IndexJobRecord` — Persistent audit record for a startup, periodic, or manual indexing run.
- `RegisteredSource` — Durable registered local filesystem source definition and sync status.
- `SourceListResponse` — Response returned by GET /api/v1/sources.
- `TriggerIndexJobRequest` — Request payload for triggering a manual indexing run (POST /api/v1/index/jobs).
- `UpdateClassificationRequest` — Request payload for PATCH /api/v1/documents/{id}/classification.
- `ClassificationSource` — Origin of the active document classification metadata.
- `IndexJobMode` — Indexing run mode for manual or scheduled jobs.
- `IndexJobState` — Execution lifecycle state of an indexing job.
- `ALLOWED_CLASSIFICATION_CATEGORIES` — Top-level document classification categories enforced by REST and search taxonomy.
- `MAX_CLASSIFICATION_SUMMARY_CHARS` — Maximum character length of a classification summary.
- `MAX_CLASSIFICATION_TERMS` — Maximum number of tags or keywords per document classification.
- `MAX_SOURCE_ID_LEN` — Maximum byte length of a source identifier slug.
- `normalize_classification_terms` — Trims, removes empty entries, deduplicates case-insensitively, and enforces MAX_CLASSIFICATION_TERMS.
- `validate_classification_category` — Validates and normalizes a classification category against ALLOWED_CLASSIFICATION_CATEGORIES.
- `validate_classification_summary` — Validates and trims a classification summary (<= MAX_CLASSIFICATION_SUMMARY_CHARS).
- `validate_source_id_slug` — Validates and normalizes a source identifier slug against ^[a-z0-9][a-z0-9_-]{0,63}$.

### constant dok3-domain::admin::ALLOWED_CLASSIFICATION_CATEGORIES

```rust
pub const ALLOWED_CLASSIFICATION_CATEGORIES: [&str; 5];
```

Top-level document classification categories enforced by REST and search taxonomy.

### constant dok3-domain::admin::MAX_CLASSIFICATION_SUMMARY_CHARS

```rust
pub const MAX_CLASSIFICATION_SUMMARY_CHARS: usize = 500;
```

Maximum character length of a classification summary.

### constant dok3-domain::admin::MAX_CLASSIFICATION_TERMS

```rust
pub const MAX_CLASSIFICATION_TERMS: usize = 12;
```

Maximum number of tags or keywords per document classification.

### constant dok3-domain::admin::MAX_SOURCE_ID_LEN

```rust
pub const MAX_SOURCE_ID_LEN: usize = 64;
```

Maximum byte length of a source identifier slug.

### enum dok3-domain::admin::ClassificationSource

```rust
pub enum ClassificationSource {
    Rule,
    Manual,
}
```

Origin of the active document classification metadata.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### enum dok3-domain::admin::IndexJobMode

```rust
pub enum IndexJobMode {
    Incremental,
    Full,
}
```

Indexing run mode for manual or scheduled jobs.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### enum dok3-domain::admin::IndexJobState

```rust
pub enum IndexJobState {
    Pending,
    Running,
    Succeeded,
    Failed,
}
```

Execution lifecycle state of an indexing job.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### fn dok3-domain::admin::normalize_classification_terms

```rust
pub fn normalize_classification_terms(
    field_name: &str,
    items: &[String],
) -> Result<Vec<String>, Dok3Error>
```

Trims, removes empty entries, deduplicates case-insensitively, and enforces `MAX_CLASSIFICATION_TERMS`.

### fn dok3-domain::admin::validate_classification_category

```rust
pub fn validate_classification_category(raw: &str) -> Result<String, Dok3Error>
```

Validates and normalizes a classification category against `ALLOWED_CLASSIFICATION_CATEGORIES`.

### fn dok3-domain::admin::validate_classification_summary

```rust
pub fn validate_classification_summary(raw: &str) -> Result<String, Dok3Error>
```

Validates and trims a classification summary (`<= MAX_CLASSIFICATION_SUMMARY_CHARS`).

### fn dok3-domain::admin::validate_source_id_slug

```rust
pub fn validate_source_id_slug(raw: &str) -> Result<String, Dok3Error>
```

Validates and normalizes a source identifier slug against `^[a-z0-9][a-z0-9_-]{0,63}$`.

### struct dok3-domain::admin::CreateSourceRequest

```rust
pub struct CreateSourceRequest {
    pub source_id: String,
    pub label: String,
    pub path: String,
    pub default_category: Option<String>,
    pub default_tag: Option<String>,
}
```

Request payload for `POST /api/v1/sources`.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-domain::admin::DeleteSourceResponse

```rust
pub struct DeleteSourceResponse {
    pub source_id: String,
    pub deleted_documents: usize,
    pub active_generation: Option<String>,
    pub active_epoch: u64,
}
```

Response returned by `DELETE /api/v1/sources/{id}`.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-domain::admin::DocumentClassification

```rust
pub struct DocumentClassification {
    pub document_id: DocumentId,
    pub summary: String,
    pub category: String,
    pub tags: Vec<String>,
    pub keywords: Vec<String>,
    pub source: ClassificationSource,
    pub manual_locked: bool,
    pub updated_at_ms: u64,
    pub content_hash: String,
}
```

Stored classification metadata associated with a document without altering original bytes.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-domain::admin::IndexJobListResponse

```rust
pub struct IndexJobListResponse {
    pub jobs: Vec<IndexJobRecord>,
    pub active_generation: Option<String>,
    pub active_epoch: u64,
    pub refresh_interval_secs: Option<u64>,
}
```

Response returned by `GET /api/v1/index/jobs`.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-domain::admin::IndexJobRecord

```rust
pub struct IndexJobRecord {
    pub job_id: String,
    pub source_id: Option<String>,
    pub trigger: String,
    pub mode: IndexJobMode,
    pub state: IndexJobState,
    pub started_at_ms: u64,
    pub finished_at_ms: Option<u64>,
    pub indexed_documents: usize,
    pub rebuilt: bool,
    pub active_generation: Option<String>,
    pub active_epoch: u64,
    pub skipped_files: Vec<String>,
    pub error: Option<String>,
}
```

Persistent audit record for a startup, periodic, or manual indexing run.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-domain::admin::RegisteredSource

```rust
pub struct RegisteredSource {
    pub source_id: String,
    pub label: String,
    pub path: String,
    pub default_category: String,
    pub default_tag: String,
    pub managed_by_env: bool,
    pub created_at_ms: u64,
    pub last_synced_at_ms: Option<u64>,
    pub document_count: usize,
    pub last_error: Option<String>,
}
```

Durable registered local filesystem source definition and sync status.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-domain::admin::SourceListResponse

```rust
pub struct SourceListResponse {
    pub sources: Vec<RegisteredSource>,
}
```

Response returned by `GET /api/v1/sources`.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-domain::admin::TriggerIndexJobRequest

```rust
pub struct TriggerIndexJobRequest {
    pub source_id: Option<String>,
    pub mode: IndexJobMode,
}
```

Request payload for triggering a manual indexing run (`POST /api/v1/index/jobs`).

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-domain::admin::UpdateClassificationRequest

```rust
pub struct UpdateClassificationRequest {
    pub summary: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub keywords: Option<Vec<String>>,
    pub reset_to_rule: bool,
}
```

Request payload for `PATCH /api/v1/documents/{id}/classification`.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`
