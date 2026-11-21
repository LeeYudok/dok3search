//! File-backed original store, document metadata store, and durable change journal (P1-2).
//!
//! Layout under a storage root:
//! - `.store.lock` / `.journal.lock`: exclusive advisory locks preventing concurrent writers.
//! - `objects/<sha256 hex>`: immutable original bytes (content-addressed, atomic write + dir fsync).
//! - `documents.json`: document metadata keyed by id (atomic replace on write; P1 temporary layout).
//! - `journal.jsonl`: append-only change events with partial-write rollback and EOF crash recovery.
//!
//! See `docs/adr/0001-p1-file-storage-and-sqlite-migration.md` for the planned SQLite migration.

mod fsutil;
mod journal;
mod store;

pub use journal::FileChangeJournal;
pub use store::{FileDocumentStore, PutOutcome, content_revision};
