//! Shared SQLite `meta.db` connection helper for `dok3-storage` (`ADR-0001`).

use std::path::Path;

use dok3_core::Dok3Error;
use rusqlite::Connection;

pub(crate) fn open_meta_db(root: &Path) -> Result<Connection, Dok3Error> {
    let db_path = root.join("meta.db");
    let conn = Connection::open(&db_path)
        .map_err(|err| Dok3Error::Storage(format!("open meta.db: {err}")))?;
    conn.execute_batch(
        "PRAGMA journal_mode = WAL;
         PRAGMA synchronous = FULL;
         PRAGMA busy_timeout = 5000;
         CREATE TABLE IF NOT EXISTS documents (
             id TEXT PRIMARY KEY,
             source_id TEXT NOT NULL,
             content_hash TEXT NOT NULL,
             payload_json TEXT NOT NULL
         );
         CREATE INDEX IF NOT EXISTS idx_documents_source_id ON documents(source_id);
         CREATE TABLE IF NOT EXISTS change_journal (
             sequence INTEGER PRIMARY KEY,
             document_id TEXT NOT NULL,
             revision_id TEXT NOT NULL,
             kind TEXT NOT NULL
         );",
    )
    .map_err(|err| Dok3Error::Storage(format!("init meta.db schema: {err}")))?;
    Ok(conn)
}
