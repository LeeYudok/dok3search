# ADR-0001: P1 File-Backed Storage Layout and Planned SQLite Migration

- **Status**: Accepted (P1 temporary implementation)
- **Date**: '2026-11-22'
- **Context**: `dok3search#19` (PR `#22`), `dok3search#23`

## Decision

For P1 (`FileDocumentStore` + `FileChangeJournal` in `dok3-storage`), metadata (`documents.json`) and the change journal (`journal.jsonl`) are persisted as plain UTF-8 files under the storage root alongside content-addressed original bytes (`objects/<sha256 hex>`), protected by exclusive advisory file locks (`.store.lock`, `.journal.lock`), `fsync` + Unix-only directory `fsync` (`#[cfg(unix)]`), and partial-write rollback.

## Rationale

1. **Zero external native C/SQLite build dependencies** during P1 offline bring-up in air-gapped Windows/macOS environments.
2. **Simple crash inspection and deterministic recovery** while `dok3-parser`, `dok3-index`, and `dok3-index-manager` converge on P1-4 end-to-end indexing.

## Consequences & Migration Boundary

- Every `FileDocumentStore::put` / `delete` rewrites the full `documents.json` map (O(N²) total bytes across N puts), and `FileChangeJournal::read_after` scans `journal.jsonl` linearly.
- This layout is bounded to P1/P2 development and early corpora (up to ~10,000 documents).
- **Planned follow-up (before P3 / 500k-document release gate)**: Migrate `documents.json` metadata and `journal.jsonl` checkpoints to SQLite (keeping `objects/<sha256>` immutable content-addressed blobs or chunked store) behind the same `DocumentStorePort` and `ChangeJournalPort` traits.
