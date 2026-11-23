# Module dok3-domain::document

## Overview

Document, section, and section-reading contracts.

## Items

- `Document` — Canonical domain representation of an indexed knowledge document.
- `DocumentDetail` — Detailed document payload returned by document retrieval endpoints.
- `DocumentFormatLinks` — Paired Markdown and YAML document IDs for design entrypoint documents.
- `DocumentSection` — Full section record owned by an indexed or stored document.
- `ParsedDocument` — Output produced by format-specific document parsers.
- `SectionCitation` — Citation metadata returned alongside section text.
- `SectionReadRequest` — Request to fetch a bounded slice of an individual section.
- `SectionReadResponse` — Paginated, source-preserving section response.
- `SectionSummary` — Lightweight section metadata exposed in document detail views and TOCs.
- `DEFAULT_SECTION_MAX_CHARS` — Default character limit when reading a single document section.
- `MAX_SECTION_MAX_CHARS` — Maximum allowed character limit for a single section-read response.

### constant dok3-domain::document::DEFAULT_SECTION_MAX_CHARS

```rust
pub const DEFAULT_SECTION_MAX_CHARS: usize = 12_000;
```

Default character limit when reading a single document section.

### constant dok3-domain::document::MAX_SECTION_MAX_CHARS

```rust
pub const MAX_SECTION_MAX_CHARS: usize = 100_000;
```

Maximum allowed character limit for a single section-read response.

### struct dok3-domain::document::Document

```rust
pub struct Document {
    pub id: DocumentId,
    pub revision_id: RevisionId,
    pub source_id: SourceId,
    pub source: String,
    pub title: String,
    pub summary: String,
    pub category: String,
    pub topic: Option<String>,
    pub tag: String,
    pub tags: Vec<String>,
    pub keywords: Vec<String>,
    pub language: String,
    pub version: String,
    pub date: Option<String>,
    pub source_uri: String,
    pub sample_path: Option<String>,
    pub acl_scope: AclScope,
    pub content_hash: String,
    pub body: String,
    pub markdown: Option<String>,
    pub sections: Vec<DocumentSection>,
}
```

Canonical domain representation of an indexed knowledge document.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-domain::document::DocumentDetail

```rust
pub struct DocumentDetail {
    pub id: DocumentId,
    pub revision_id: RevisionId,
    pub source_id: SourceId,
    pub source: String,
    pub title: String,
    pub summary: String,
    pub category: String,
    pub topic: Option<String>,
    pub tag: String,
    pub tags: Vec<String>,
    pub keywords: Vec<String>,
    pub language: String,
    pub version: String,
    pub date: Option<String>,
    pub source_uri: String,
    pub sample_path: Option<String>,
    pub acl_scope: AclScope,
    pub body: String,
    pub markdown: Option<String>,
    pub sections: Vec<SectionSummary>,
    pub formats: Option<DocumentFormatLinks>,
}
```

Detailed document payload returned by document retrieval endpoints.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-domain::document::DocumentFormatLinks

```rust
pub struct DocumentFormatLinks {
    pub markdown_id: DocumentId,
    pub yaml_id: DocumentId,
}
```

Paired Markdown and YAML document IDs for design entrypoint documents.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-domain::document::DocumentSection

```rust
pub struct DocumentSection {
    pub section_id: SectionId,
    pub heading: String,
    pub anchor: String,
    pub order: usize,
    pub line_start: usize,
    pub line_end: usize,
    pub body: String,
}
```

Full section record owned by an indexed or stored document.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn from(section: &DocumentSection) -> Self`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-domain::document::ParsedDocument

```rust
pub struct ParsedDocument {
    pub title: String,
    pub sections: Vec<DocumentSection>,
    pub normalized_text: String,
    pub parse_warnings: Vec<String>,
}
```

Output produced by format-specific document parsers.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-domain::document::SectionCitation

```rust
pub struct SectionCitation {
    pub source_uri: String,
    pub section: SectionId,
}
```

Citation metadata returned alongside section text.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-domain::document::SectionReadRequest

```rust
pub struct SectionReadRequest {
    pub document_id: DocumentId,
    pub section_id: SectionId,
    pub cursor: Option<usize>,
    pub max_chars: usize,
}
```

Request to fetch a bounded slice of an individual section.

**Methods**

- `pub fn normalized(self) -> Self`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-domain::document::SectionReadResponse

```rust
pub struct SectionReadResponse {
    pub source_revision: RevisionId,
    pub heading_path: Vec<String>,
    pub citation: SectionCitation,
    pub text: String,
    pub next_cursor: Option<usize>,
    pub truncated: bool,
}
```

Paginated, source-preserving section response.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-domain::document::SectionSummary

```rust
pub struct SectionSummary {
    pub section_id: SectionId,
    pub heading: String,
    pub anchor: String,
    pub order: usize,
    pub line_start: usize,
    pub line_end: usize,
}
```

Lightweight section metadata exposed in document detail views and TOCs.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn from(section: &DocumentSection) -> Self`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`
