use std::fs;
use std::path::{Path, PathBuf};

use dok3_connectors::{
    SqliteColumnMapping, SqliteCursor, SqliteCursorMode, SqliteSourceAdapter, SqliteSourceConfig,
};
use dok3_core::Dok3Error;
use dok3_domain::SourceId;
use rusqlite::Connection;

struct TempDir(PathBuf);

impl TempDir {
    fn new(name: &str) -> Self {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "dok3-sqlite-source-{name}-{}-{nanos}",
            std::process::id()
        ));
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }

    fn db_path(&self) -> PathBuf {
        self.0.join("source.sqlite")
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn column_mapping() -> SqliteColumnMapping {
    SqliteColumnMapping {
        document_id: "id".to_string(),
        title: "title".to_string(),
        body: "body".to_string(),
        summary: None,
    }
}

fn adapter(
    db_path: &Path,
    select_query: &str,
    cursor: SqliteCursorMode,
) -> Result<SqliteSourceAdapter, Dok3Error> {
    SqliteSourceAdapter::new(SqliteSourceConfig::new(
        SourceId::new("sqlite-source"),
        "SQLite fixture",
        db_path,
        select_query,
        column_mapping(),
        cursor,
    ))
}

fn create_integer_fixture(db_path: &Path) {
    let connection = Connection::open(db_path).unwrap();
    connection
        .execute_batch(
            "
            CREATE TABLE docs (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                body TEXT NOT NULL
            );
            INSERT INTO docs (id, title, body) VALUES
                (1, 'First', 'First body'),
                (2, 'Second', 'Second body');
            ",
        )
        .unwrap();
}

#[test]
fn rejects_non_select_and_multiple_statements() {
    let dir = TempDir::new("reject-write");
    let db_path = dir.db_path();
    create_integer_fixture(&db_path);

    for query in [
        "UPDATE docs SET title = 'x'",
        "DELETE FROM docs",
        "CREATE TABLE another (id INTEGER)",
        "SELECT * FROM docs; DELETE FROM docs",
        "PRAGMA writable_schema = ON",
    ] {
        let cursor = SqliteCursorMode::IntegerPk {
            column: "id".to_string(),
        };
        let error = adapter(&db_path, query, cursor).unwrap_err();
        assert!(
            matches!(error, Dok3Error::InvalidInput(_)),
            "query should be rejected as non-SELECT: {query}"
        );
    }
}

#[test]
fn missing_database_is_not_created_by_read_only_collection() {
    let dir = TempDir::new("missing-db");
    let db_path = dir.0.join("missing.sqlite");
    let cursor = SqliteCursorMode::IntegerPk {
        column: "id".to_string(),
    };
    let adapter = adapter(&db_path, "SELECT id, title, body FROM docs", cursor).unwrap();
    let error = adapter.collect(None).unwrap_err();
    assert!(matches!(error, Dok3Error::NotFound(_)));
    assert!(!db_path.exists());
}

#[test]
fn integer_pk_cursor_collects_only_rows_after_the_last_key() {
    let dir = TempDir::new("integer-cursor");
    let db_path = dir.db_path();
    create_integer_fixture(&db_path);

    let adapter = adapter(
        &db_path,
        "SELECT id, title, body FROM docs",
        SqliteCursorMode::IntegerPk {
            column: "id".to_string(),
        },
    )
    .unwrap();
    let first = adapter.collect(None).unwrap();
    assert_eq!(first.documents.len(), 2);
    assert_eq!(first.documents[0].id.as_str(), "sqlite-source:1");
    assert_eq!(first.documents[1].id.as_str(), "sqlite-source:2");
    assert_eq!(first.next_cursor, Some(SqliteCursor::Integer(2)));
    assert!(!first.has_more);

    let connection = Connection::open(&db_path).unwrap();
    connection
        .execute(
            "INSERT INTO docs (id, title, body) VALUES (3, 'Third', 'Third body')",
            [],
        )
        .unwrap();

    let second = adapter.collect(Some(&SqliteCursor::Integer(2))).unwrap();
    assert_eq!(second.documents.len(), 1);
    assert_eq!(second.documents[0].id.as_str(), "sqlite-source:3");
    assert_eq!(second.documents[0].title, "Third");
    assert_eq!(second.next_cursor, Some(SqliteCursor::Integer(3)));
}

#[test]
fn updated_at_cursor_recollects_added_and_changed_rows_from_ddl_fixture() {
    let dir = TempDir::new("updated-at-cursor");
    let db_path = dir.db_path();
    let connection = Connection::open(&db_path).unwrap();
    connection
        .execute_batch(
            "
            CREATE TABLE docs (
                id INTEGER PRIMARY KEY,
                updated_at TEXT NOT NULL,
                title TEXT NOT NULL,
                body TEXT NOT NULL
            );
            INSERT INTO docs (id, updated_at, title, body) VALUES
                (1, '2026-11-22T00:00:01Z', 'First', 'First body'),
                (2, '2026-11-22T00:00:02Z', 'Second', 'Second body');
            ",
        )
        .unwrap();

    let adapter = adapter(
        &db_path,
        "SELECT id, updated_at, title, body FROM docs",
        SqliteCursorMode::UpdatedAt {
            column: "updated_at".to_string(),
        },
    )
    .unwrap();
    let first = adapter.collect(None).unwrap();
    assert_eq!(first.documents.len(), 2);
    assert_eq!(
        first.next_cursor,
        Some(SqliteCursor::UpdatedAt("2026-11-22T00:00:02Z".to_string()))
    );

    connection
        .execute_batch(
            "
            UPDATE docs
               SET title = 'First changed',
                   body = 'Changed body',
                   updated_at = '2026-11-22T00:00:05Z'
             WHERE id = 1;
            INSERT INTO docs (id, updated_at, title, body)
            VALUES (3, '2026-11-22T00:00:04Z', 'Third', 'Third body');
            ",
        )
        .unwrap();

    let second = adapter
        .collect(Some(&SqliteCursor::UpdatedAt(
            "2026-11-22T00:00:02Z".to_string(),
        )))
        .unwrap();
    let ids: Vec<&str> = second
        .documents
        .iter()
        .map(|document| document.id.as_str())
        .collect();
    assert_eq!(ids, vec!["sqlite-source:3", "sqlite-source:1"]);
    let changed = second
        .documents
        .iter()
        .find(|document| document.id.as_str() == "sqlite-source:1")
        .unwrap();
    assert_eq!(changed.title, "First changed");
    assert_eq!(changed.body, "Changed body");
    assert_eq!(
        second.next_cursor,
        Some(SqliteCursor::UpdatedAt("2026-11-22T00:00:05Z".to_string()))
    );
}
