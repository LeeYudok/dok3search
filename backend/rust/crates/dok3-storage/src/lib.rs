//! File-backed original store, document metadata store, and durable change journal (P1-2).
//!
//! Layout under a storage root:
//! - `objects/<sha256 hex>`: immutable original bytes (content-addressed).
//! - `documents.json`: document metadata keyed by id (atomic replace on every write).
//! - `journal.jsonl`: append-only change events, one JSON object per line, fsync on append.

mod fsutil;
mod journal;
mod store;

pub use journal::FileChangeJournal;
pub use store::{FileDocumentStore, PutOutcome, content_revision};
