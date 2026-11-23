# Crate dok3-connectors

## Overview

Source connectors for local files, Git, SVN, and relational databases.

## Items

- `LocalFileConnector` — Scans a local directory tree deterministically for Markdown sources.
- `ScanReport` — Aggregated result returned by LocalFileConnector::scan.
- `ScannedFile` — Raw file payload collected by LocalFileConnector::scan.
- `SkippedFile` — Metadata for a matching file skipped during scanning (for example exceeding size bounds).
- `SourceConfig` — Configuration describing a local filesystem knowledge source.
- `SqliteColumnMapping` — Column mapping from a configured SELECT result to a domain document.
- `SqliteSourceAdapter` — Reads configured SELECT rows from a SQLite file without opening it for writes.
- `SqliteSourceBatch` — Documents and next cursor produced by one incremental collection pass.
- `SqliteSourceConfig` — Configuration for SqliteSourceAdapter.
- `SqliteCursor` — Last successfully observed source position.
- `SqliteCursorMode` — Supported incremental cursor columns for the first SQLite adapter slice.
- `DEFAULT_MAX_FILE_BYTES` — Default per-file upper bound (4 MiB) before a file is skipped during scanning.
- `build_document` — Converts a scanned file and its parsed Markdown sections into a canonical domain Document.

### constant dok3-connectors::DEFAULT_MAX_FILE_BYTES

```rust
pub const DEFAULT_MAX_FILE_BYTES: u64 = _; // 4_194_304u64
```

Default per-file upper bound (4 MiB) before a file is skipped during scanning.

### enum dok3-connectors::SqliteCursor

```rust
pub enum SqliteCursor {
    Integer(i64),
    UpdatedAt {
        updated_at: String,
    },
}
```

Last successfully observed source position.

The `updated_at` variant is an inclusive watermark. Its last timestamp
group may be emitted more than once; consumers should upsert by document ID.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### enum dok3-connectors::SqliteCursorMode

```rust
pub enum SqliteCursorMode {
    IntegerPk {
        column: String,
    },
    UpdatedAt {
        column: String,
    },
}
```

Supported incremental cursor columns for the first SQLite adapter slice.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### fn dok3-connectors::build_document

```rust
pub fn build_document(
    config: &SourceConfig,
    scanned: &ScannedFile,
    parsed: &ParsedDocument,
) -> Document
```

Converts a scanned file and its parsed Markdown sections into a canonical domain `Document`.

### struct dok3-connectors::LocalFileConnector

```rust
pub struct LocalFileConnector { /* private fields */ }
```

Scans a local directory tree deterministically for Markdown sources.

**Methods**

- `pub fn new(config: SourceConfig) -> Result<Self, Dok3Error>`
- `pub fn config(&self) -> &SourceConfig`
- `pub fn scan(&self) -> Result<ScanReport, Dok3Error>`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct dok3-connectors::ScanReport

```rust
pub struct ScanReport {
    pub files: Vec<ScannedFile>,
    pub skipped: Vec<SkippedFile>,
}
```

Aggregated result returned by `LocalFileConnector::scan`.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### struct dok3-connectors::ScannedFile

```rust
pub struct ScannedFile {
    pub document_id: DocumentId,
    pub relative_path: String,
    pub absolute_path: PathBuf,
    pub filename: String,
    pub media_type: String,
    pub bytes: Vec<u8>,
    pub size_bytes: u64,
}
```

Raw file payload collected by `LocalFileConnector::scan`.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### struct dok3-connectors::SkippedFile

```rust
pub struct SkippedFile {
    pub relative_path: String,
    pub size_bytes: u64,
    pub reason: String,
}
```

Metadata for a matching file skipped during scanning (for example exceeding size bounds).

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### struct dok3-connectors::SourceConfig

```rust
pub struct SourceConfig {
    pub source_id: SourceId,
    pub source_label: String,
    pub root_dir: PathBuf,
    pub category: String,
    pub language: String,
    pub default_tag: String,
    pub acl_scope: AclScope,
    pub allowed_extensions: Vec<String>,
    pub max_file_bytes: u64,
}
```

Configuration describing a local filesystem knowledge source.

**Methods**

- `pub fn markdown_defaults( source_id: SourceId, source_label: impl Into<String>, root_dir: impl Into<PathBuf>, ) -> Self`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### struct dok3-connectors::SqliteColumnMapping

```rust
pub struct SqliteColumnMapping {
    pub document_id: String,
    pub title: String,
    pub body: String,
    pub summary: Option<String>,
}
```

Column mapping from a configured SELECT result to a domain document.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### struct dok3-connectors::SqliteSourceAdapter

```rust
pub struct SqliteSourceAdapter { /* private fields */ }
```

Reads configured SELECT rows from a SQLite file without opening it for writes.

**Methods**

- `pub fn new(config: SqliteSourceConfig) -> Result<Self, Dok3Error>`
- `pub fn config(&self) -> &SqliteSourceConfig`
- `pub fn collect( &self, cursor: Option<&SqliteCursor>, ) -> Result<SqliteSourceBatch, Dok3Error>`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct dok3-connectors::SqliteSourceBatch

```rust
pub struct SqliteSourceBatch {
    pub documents: Vec<Document>,
    pub next_cursor: Option<SqliteCursor>,
    pub has_more: bool,
}
```

Documents and next cursor produced by one incremental collection pass.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### struct dok3-connectors::SqliteSourceConfig

```rust
pub struct SqliteSourceConfig {
    pub source_id: SourceId,
    pub source_label: String,
    pub database_path: PathBuf,
    pub select_query: String,
    pub columns: SqliteColumnMapping,
    pub cursor: SqliteCursorMode,
    pub category: String,
    pub language: String,
    pub default_tag: String,
    pub acl_scope: AclScope,
}
```

Configuration for `SqliteSourceAdapter`.

**Methods**

- `pub fn new( source_id: SourceId, source_label: impl Into<String>, database_path: impl Into<PathBuf>, select_query: impl Into<String>, columns: SqliteColumnMapping, cursor: SqliteCursorMode, ) -> Self`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
