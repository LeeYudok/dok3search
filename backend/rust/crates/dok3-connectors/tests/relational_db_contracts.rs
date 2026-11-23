use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use dok3_connectors::{
    DbDialect, DbParam, DbRow, DbRowReader, RelationalSourceAdapter, RelationalSourceConfig,
    SqliteColumnMapping, SqliteCursor, SqliteCursorMode, TombstoneStrategy,
    validate_readonly_select,
};
use dok3_domain::{AclScope, SourceId};

type RecordedQueries = Arc<Mutex<Vec<(String, Vec<DbParam>)>>>;

#[derive(Clone, Default)]
struct FakeDbReader {
    rows: Vec<DbRow>,
    recorded: RecordedQueries,
}

impl DbRowReader for FakeDbReader {
    fn query_rows(
        &self,
        sql: &str,
        params: &[DbParam],
    ) -> Result<Vec<DbRow>, dok3_core::Dok3Error> {
        self.recorded
            .lock()
            .unwrap()
            .push((sql.to_string(), params.to_vec()));
        Ok(self.rows.clone())
    }
}

fn make_row(id: &str, title: &str, body: &str, deleted_at: Option<&str>) -> DbRow {
    let mut columns = BTreeMap::new();
    columns.insert("id".to_string(), Some(id.to_string()));
    columns.insert("title".to_string(), Some(title.to_string()));
    columns.insert("body".to_string(), Some(body.to_string()));
    columns.insert("deleted_at".to_string(), deleted_at.map(str::to_string));
    DbRow { columns }
}

#[test]
fn sql_guard_allows_cte_and_semicolons_inside_literals_or_comments() {
    let cte = "WITH recent AS (SELECT id, title, body FROM docs WHERE note = 'a;b' /* ; DROP TABLE docs */) SELECT id, title, body FROM recent -- trailing ; comment\n;";
    let normalized = validate_readonly_select(cte).expect("valid CTE SELECT");
    assert!(normalized.starts_with("WITH recent"));
    assert!(!normalized.ends_with(';'));

    assert!(validate_readonly_select("SELECT 1; DELETE FROM docs").is_err());
    assert!(validate_readonly_select("WITH x AS (SELECT 1) DELETE FROM docs").is_err());
    assert!(validate_readonly_select("SELECT * INTO OUTFILE '/tmp/x' FROM docs").is_err());
}

#[test]
fn generates_dialect_specific_parameterized_queries_for_mysql_pg_oracle() {
    for (dialect, expected_ph, expected_limit) in [
        (DbDialect::MySql, "WHERE `id` > ?", "LIMIT ?"),
        (DbDialect::MariaDb, "WHERE `id` > ?", "LIMIT ?"),
        (DbDialect::PostgreSql, "WHERE \"id\" > $1", "LIMIT $2"),
        (
            DbDialect::Oracle,
            "WHERE \"id\" > :cursor",
            "FETCH FIRST :limit ROWS ONLY",
        ),
    ] {
        let cfg = RelationalSourceConfig {
            dialect,
            source_id: SourceId::new("db-test"),
            source_label: "DB".to_string(),
            connection_label: "host/db".to_string(),
            select_query: "SELECT id, title, body FROM docs".to_string(),
            columns: SqliteColumnMapping {
                document_id: "id".to_string(),
                title: "title".to_string(),
                body: "body".to_string(),
                summary: None,
            },
            cursor: SqliteCursorMode::IntegerPk {
                column: "id".to_string(),
            },
            tombstone: None,
            category: "database".to_string(),
            language: "ko".to_string(),
            default_tag: "database".to_string(),
            acl_scope: AclScope::Public,
            batch_limit: 100,
        };
        let adapter = RelationalSourceAdapter::new(cfg, FakeDbReader::default()).unwrap();
        let (sql, params) = adapter
            .build_query(Some(&SqliteCursor::Integer(42)))
            .unwrap();
        assert!(sql.contains(expected_ph), "dialect {dialect:?}: {sql}");
        assert!(sql.contains(expected_limit), "dialect {dialect:?}: {sql}");
        assert_eq!(params, vec![DbParam::Integer(42), DbParam::Integer(101)]);
    }
}

#[test]
fn collects_batch_with_has_more_and_tombstone_deletes() {
    let reader = FakeDbReader {
        rows: vec![
            make_row("1", "Active", "Body 1", None),
            make_row("2", "Deleted", "Body 2", Some("2026-11-24T00:00:00Z")),
            make_row("3", "NextPage", "Body 3", None),
        ],
        recorded: Arc::default(),
    };
    let cfg = RelationalSourceConfig {
        dialect: DbDialect::PostgreSql,
        source_id: SourceId::new("pg-docs"),
        source_label: "PG".to_string(),
        connection_label: "pg-host/kb".to_string(),
        select_query:
            "WITH src AS (SELECT id, title, body, deleted_at FROM docs) SELECT * FROM src"
                .to_string(),
        columns: SqliteColumnMapping {
            document_id: "id".to_string(),
            title: "title".to_string(),
            body: "body".to_string(),
            summary: None,
        },
        cursor: SqliteCursorMode::IntegerPk {
            column: "id".to_string(),
        },
        tombstone: Some(TombstoneStrategy::DeletedAtColumn("deleted_at".to_string())),
        category: "database".to_string(),
        language: "ko".to_string(),
        default_tag: "pg".to_string(),
        acl_scope: AclScope::Public,
        batch_limit: 2,
    };

    let adapter = RelationalSourceAdapter::new(cfg, reader).unwrap();
    let batch = adapter.collect(None).unwrap();
    assert_eq!(batch.documents.len(), 1);
    assert_eq!(batch.documents[0].id.as_str(), "pg-docs:1");
    assert_eq!(batch.deleted_ids.len(), 1);
    assert_eq!(batch.deleted_ids[0].as_str(), "pg-docs:2");
    assert_eq!(batch.next_cursor, Some(SqliteCursor::Integer(2)));
    assert!(batch.has_more);
}
