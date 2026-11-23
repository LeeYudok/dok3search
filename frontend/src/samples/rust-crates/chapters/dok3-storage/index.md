# Crate dok3-storage

## Overview

File-backed original store, document metadata store, and durable change journal (P1-2).

Layout under a storage root:

- `.store.lock` / `.journal.lock`: exclusive advisory locks preventing concurrent writers.

- `objects/<sha256 hex>`: immutable original bytes (content-addressed, atomic write + dir fsync).

- `documents.json`: document metadata keyed by id (atomic replace on write; P1 temporary layout).

- `journal.jsonl`: append-only change events with partial-write rollback and EOF crash recovery.

See `docs/adr/0001-p1-file-storage-and-sqlite-migration.md` for the planned SQLite migration.

## Items

- `FileChangeJournal` — Durable single-writer change journal. Assigns sequences strictly monotonically (last + 1); any caller-supplied sequence is overwritten.
- `FileDocumentStore` — File-backed store: originals in objects/, metadata in documents.json.
- `PutOutcome` — Result of FileDocumentStore::put.
- `content_revision` — Returns the canonical revision id (sha256:<hex>) for original bytes.

### enum dok3-storage::PutOutcome

```rust
pub enum PutOutcome {
    Stored(RevisionId),
    Unchanged(RevisionId),
}
```

Result of `FileDocumentStore::put`.

**Methods**

- `pub fn revision(&self) -> &RevisionId`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### fn dok3-storage::content_revision

```rust
pub fn content_revision(content: &[u8]) -> RevisionId
```

Returns the canonical revision id (`sha256:<hex>`) for original bytes.

### struct dok3-storage::FileChangeJournal

```rust
pub struct FileChangeJournal { /* private fields */ }
```

Durable single-writer change journal. Assigns sequences strictly monotonically (`last + 1`);
any caller-supplied `sequence` is overwritten.

**Methods**

- `pub fn open(root: &Path) -> Result<Self, Dok3Error>`
- `pub fn last_sequence(&self) -> Result<Sequence, Dok3Error>`
- `fn append(&self, change: ChangeEvent) -> Result<Sequence, Dok3Error>`
- `fn read_after( &self, seq: Sequence, limit: usize, ) -> Result<Vec<ChangeEvent>, Dok3Error>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct dok3-storage::FileDocumentStore

```rust
pub struct FileDocumentStore { /* private fields */ }
```

File-backed store: originals in `objects/`, metadata in `documents.json`.

**Methods**

- `pub fn open(root: &Path) -> Result<Self, Dok3Error>`
- `pub fn put( &self, doc: Document, content: &[u8], ) -> Result<PutOutcome, Dok3Error>`
- `pub fn delete(&self, id: &DocumentId) -> Result<bool, Dok3Error>`
- `pub fn read_original( &self, principal: &Principal, id: &DocumentId, ) -> Result<Vec<u8>, Dok3Error>`
- `pub fn get_stored_document( &self, principal: &Principal, id: &DocumentId, ) -> Result<Document, Dok3Error>`
- `pub fn document_ids(&self) -> Result<Vec<DocumentId>, Dok3Error>`
- `pub fn existing_content_hash( &self, id: &DocumentId, ) -> Result<Option<String>, Dok3Error>`
- `pub fn put_immutable( &self, doc: Document, content: &[u8], ) -> Result<PutOutcome, Dok3Error>`
- `pub fn update_metadata(&self, doc: Document) -> Result<PutOutcome, Dok3Error>`
- `pub fn list_documents(&self) -> Result<Vec<Document>, Dok3Error>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn get_document( &self, principal: &Principal, id: &DocumentId, ) -> Result<DocumentDetail, Dok3Error>`
- `fn get_section( &self, principal: &Principal, req: &SectionReadRequest, ) -> Result<SectionReadResponse, Dok3Error>`
- `fn document_count(&self) -> usize`
