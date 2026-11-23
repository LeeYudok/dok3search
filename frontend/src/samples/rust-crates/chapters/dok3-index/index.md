# Crate dok3-index

## Overview

Tantivy schema, custom tokenizers, and reader/writer index adapter (P1-3).

## Items

- `IndexFields` — Tantivy field handles bound to a schema.
- `TantivyIndex` — Tantivy-backed search index with a single serialized writer and manual reader reloads.
- `SCHEMA_VERSION` — Increment when index field definitions or tokenizers change incompatibly.
- `TOKENIZER_NGRAM` — Tokenizer name for 2-3 character n-gram partial matching.
- `TOKENIZER_TEXT` — Tokenizer name for general words and code identifier segments.

### constant dok3-index::SCHEMA_VERSION

```rust
pub const SCHEMA_VERSION: u32 = 2;
```

Increment when index field definitions or tokenizers change incompatibly.

### constant dok3-index::TOKENIZER_NGRAM

```rust
pub const TOKENIZER_NGRAM: &str = "dok3_ngram_v2";
```

Tokenizer name for 2-3 character n-gram partial matching.

### constant dok3-index::TOKENIZER_TEXT

```rust
pub const TOKENIZER_TEXT: &str = "dok3_text_v2";
```

Tokenizer name for general words and code identifier segments.

### struct dok3-index::IndexFields

```rust
pub struct IndexFields {
    pub document_id: Field,
    pub revision_id: Field,
    pub source_id: Field,
    pub acl_scope: Field,
    pub category: Field,
    pub language: Field,
    pub tag_exact: Field,
    pub title: Field,
    pub summary: Field,
    pub tags_text: Field,
    pub body: Field,
    pub body_ngram: Field,
    pub hit_payload: Field,
}
```

Tantivy field handles bound to a schema.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct dok3-index::TantivyIndex

```rust
pub struct TantivyIndex { /* private fields */ }
```

Tantivy-backed search index with a single serialized writer and manual reader reloads.

**Methods**

- `pub fn create_in_ram(generation_id: GenerationId) -> Result<Self, Dok3Error>`
- `pub fn open_or_create( dir: &Path, generation_id: GenerationId, ) -> Result<Self, Dok3Error>`
- `pub fn upsert_document(&self, doc: &Document) -> Result<(), Dok3Error>`
- `pub fn delete_document(&self, document_id: &DocumentId) -> Result<(), Dok3Error>`
- `pub fn commit_and_reload( &self, applied_sequence: Option<Sequence>, ) -> Result<SearchSnapshotMeta, Dok3Error>`
- `pub fn reload(&self) -> Result<SearchSnapshotMeta, Dok3Error>`
- `pub fn doc_count(&self) -> u64`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn search( &self, principal: &Principal, req: &SearchRequest, ) -> Result<SearchResponse, Dok3Error>`
- `fn snapshot(&self) -> SearchSnapshotMeta`
