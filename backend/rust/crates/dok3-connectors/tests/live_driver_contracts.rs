use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use dok3_connectors::{
    DbDialect, DbParam, DbRowReader, MysqlRowReader, PostgresRowReader, RelationalSourceAdapter,
    RelationalSourceConfig, SqliteColumnMapping, SqliteCursor, SqliteCursorMode, TombstoneStrategy,
    mysql_value_to_string,
};
use dok3_core::Dok3Error;
use dok3_domain::{AclScope, SourceId};
use mysql::Value;

fn config(dialect: DbDialect) -> RelationalSourceConfig {
    RelationalSourceConfig {
        dialect,
        source_id: SourceId::new("live"),
        source_label: "Live".to_string(),
        connection_label: "127.0.0.1/db".to_string(),
        select_query: "SELECT id, title, body, deleted_at FROM docs".to_string(),
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
        default_tag: "db".to_string(),
        acl_scope: AclScope::Public,
        batch_limit: 5,
    }
}

// ---------------------------------------------------------------------------
// Minimal PostgreSQL v3 mock server (extended query protocol, binary results).
// ---------------------------------------------------------------------------

fn msg(tag: u8, body: &[u8]) -> Vec<u8> {
    let mut out = vec![tag];
    out.extend_from_slice(&(u32::try_from(body.len()).unwrap() + 4).to_be_bytes());
    out.extend_from_slice(body);
    out
}

fn cstr(s: &str) -> Vec<u8> {
    let mut v = s.as_bytes().to_vec();
    v.push(0);
    v
}

fn read_full(stream: &mut TcpStream, buf: &mut [u8]) -> bool {
    stream.read_exact(buf).is_ok()
}

fn row_description() -> Vec<u8> {
    let cols: [(&str, u32, i16); 4] = [
        ("id", 23, 4),
        ("title", 25, -1),
        ("body", 25, -1),
        ("deleted_at", 25, -1),
    ];
    let mut body = Vec::new();
    body.extend_from_slice(&4i16.to_be_bytes());
    for (name, oid, len) in cols {
        body.extend(cstr(name));
        body.extend_from_slice(&0u32.to_be_bytes());
        body.extend_from_slice(&0i16.to_be_bytes());
        body.extend_from_slice(&oid.to_be_bytes());
        body.extend_from_slice(&len.to_be_bytes());
        body.extend_from_slice(&(-1i32).to_be_bytes());
        body.extend_from_slice(&1i16.to_be_bytes()); // binary
    }
    msg(b'T', &body)
}

fn data_row(id: i32, title: &str, body: &str, deleted: Option<&str>) -> Vec<u8> {
    let mut b = Vec::new();
    b.extend_from_slice(&4i16.to_be_bytes());
    b.extend_from_slice(&4i32.to_be_bytes());
    b.extend_from_slice(&id.to_be_bytes());
    for text in [Some(title), Some(body), deleted] {
        match text {
            Some(t) => {
                b.extend_from_slice(&i32::try_from(t.len()).unwrap().to_be_bytes());
                b.extend_from_slice(t.as_bytes());
            }
            None => b.extend_from_slice(&(-1i32).to_be_bytes()),
        }
    }
    msg(b'D', &b)
}

fn serve_postgres(mut stream: TcpStream, log: &Mutex<Vec<String>>) {
    // StartupMessage (untagged).
    let mut len = [0u8; 4];
    if !read_full(&mut stream, &mut len) {
        return;
    }
    let mut startup = vec![0u8; u32::from_be_bytes(len) as usize - 4];
    if !read_full(&mut stream, &mut startup) {
        return;
    }
    let mut hello = msg(b'R', &0u32.to_be_bytes());
    hello.extend(msg(b'S', &[cstr("client_encoding"), cstr("UTF8")].concat()));
    hello.extend(msg(b'Z', b"I"));
    stream.write_all(&hello).unwrap();

    let mut param_count = 0usize;
    loop {
        let mut head = [0u8; 5];
        if !read_full(&mut stream, &mut head) {
            return;
        }
        let mut body = vec![0u8; u32::from_be_bytes(head[1..5].try_into().unwrap()) as usize - 4];
        if !read_full(&mut stream, &mut body) {
            return;
        }
        let reply = match head[0] {
            b'Q' => {
                let sql = String::from_utf8_lossy(&body[..body.len() - 1]).into_owned();
                log.lock().unwrap().push(sql);
                let mut r = msg(b'C', &cstr("OK"));
                r.extend(msg(b'Z', b"I"));
                r
            }
            b'P' => {
                let name_end = body.iter().position(|b| *b == 0).unwrap();
                let rest = &body[name_end + 1..];
                let sql_end = rest.iter().position(|b| *b == 0).unwrap();
                let sql = String::from_utf8_lossy(&rest[..sql_end]).into_owned();
                param_count = (1..=9).filter(|n| sql.contains(&format!("${n}"))).count();
                log.lock().unwrap().push(sql);
                msg(b'1', &[])
            }
            b'D' => {
                let mut types = Vec::new();
                types.extend_from_slice(&i16::try_from(param_count).unwrap().to_be_bytes());
                for _ in 0..param_count {
                    types.extend_from_slice(&20u32.to_be_bytes()); // int8
                }
                let mut r = msg(b't', &types);
                r.extend(row_description());
                r
            }
            b'B' => msg(b'2', &[]),
            b'E' => {
                let mut r = data_row(1, "첫번째", "본문 1", None);
                r.extend(data_row(
                    2,
                    "삭제됨",
                    "본문 2",
                    Some("2026-11-24T00:00:00Z"),
                ));
                r.extend(msg(b'C', &cstr("SELECT 2")));
                r
            }
            b'C' => msg(b'3', &[]),
            b'S' => msg(b'Z', b"I"),
            b'X' => return,
            _ => Vec::new(),
        };
        if !reply.is_empty() {
            stream.write_all(&reply).unwrap();
        }
    }
}

#[test]
fn postgres_adapter_collects_over_real_tcp_with_mock_server() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let log = Arc::new(Mutex::new(Vec::new()));
    let server_log = Arc::clone(&log);
    let server = thread::spawn(move || {
        for _ in 0..2 {
            let (stream, _) = listener.accept().unwrap();
            serve_postgres(stream, &server_log);
        }
    });

    let dsn = format!("postgresql://user:supersecret@127.0.0.1:{port}/db");
    let adapter = RelationalSourceAdapter::new_postgres(config(DbDialect::PostgreSql), &dsn)
        .expect("adapter");
    for cursor in [None, Some(SqliteCursor::Integer(0))] {
        let batch = adapter.collect(cursor.as_ref()).expect("collect");
        assert_eq!(batch.documents.len(), 1);
        assert_eq!(batch.documents[0].id.as_str(), "live:1");
        assert_eq!(batch.deleted_ids.len(), 1);
        assert_eq!(batch.deleted_ids[0].as_str(), "live:2");
        assert_eq!(batch.next_cursor, Some(SqliteCursor::Integer(2)));
        assert!(!batch.has_more);
    }
    drop(adapter);
    server.join().unwrap();

    let log = log.lock().unwrap();
    assert!(
        log.iter()
            .any(|q| q.to_ascii_uppercase().contains("TRANSACTION READ ONLY")),
        "expected a read-only transaction, saw {log:?}"
    );
    assert!(log.iter().any(|q| q.contains("$1") && q.contains("$2")));
}

#[test]
fn postgres_rejects_bad_dsn_and_redacts_connect_errors() {
    assert!(PostgresRowReader::new("").is_err());
    assert!(PostgresRowReader::new("   ").is_err());
    assert!(PostgresRowReader::new("mysql://u:p@h/db").is_err());

    let port = closed_port();
    let reader =
        PostgresRowReader::new(&format!("postgres://user:supersecret@127.0.0.1:{port}/db"))
            .unwrap();
    let err = reader.query_rows("SELECT 1", &[]).unwrap_err();
    assert!(matches!(err, Dok3Error::Storage(_)));
    assert!(!format!("{err} {err:?}").contains("supersecret"));
}

// ---------------------------------------------------------------------------
// MySQL
// ---------------------------------------------------------------------------

fn closed_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.local_addr().unwrap().port()
}

#[test]
fn mysql_value_conversion_covers_all_variants() {
    assert_eq!(mysql_value_to_string(&Value::NULL), None);
    assert_eq!(
        mysql_value_to_string(&Value::Bytes("안녕".as_bytes().to_vec())).as_deref(),
        Some("안녕")
    );
    assert_eq!(
        mysql_value_to_string(&Value::Int(-7)).as_deref(),
        Some("-7")
    );
    assert_eq!(
        mysql_value_to_string(&Value::UInt(u64::MAX)).as_deref(),
        Some("18446744073709551615")
    );
    assert_eq!(
        mysql_value_to_string(&Value::Float(1.5)).as_deref(),
        Some("1.5")
    );
    assert_eq!(
        mysql_value_to_string(&Value::Double(2.25)).as_deref(),
        Some("2.25")
    );
    assert_eq!(
        mysql_value_to_string(&Value::Date(2026, 11, 24, 1, 2, 3, 0)).as_deref(),
        Some("2026-11-24 01:02:03")
    );
    assert_eq!(
        mysql_value_to_string(&Value::Date(2026, 11, 24, 1, 2, 3, 45)).as_deref(),
        Some("2026-11-24 01:02:03.000045")
    );
    assert_eq!(
        mysql_value_to_string(&Value::Time(true, 1, 2, 3, 4, 0)).as_deref(),
        Some("-26:03:04")
    );
    assert_eq!(
        mysql_value_to_string(&Value::Time(false, 0, 2, 3, 4, 500)).as_deref(),
        Some("02:03:04.000500")
    );
}

#[test]
fn dialect_guards_reject_mismatched_constructors() {
    let err = RelationalSourceAdapter::new_mysql(config(DbDialect::Oracle), "mysql://u:p@h/db")
        .err()
        .expect("oracle rejected");
    assert!(matches!(err, Dok3Error::InvalidInput(_)));
    assert!(
        RelationalSourceAdapter::new_postgres(config(DbDialect::MySql), "postgres://u@h/db")
            .is_err()
    );
    assert!(
        RelationalSourceAdapter::new_mysql(config(DbDialect::MariaDb), "mysql://u:p@h/db").is_ok()
    );
}

#[test]
fn mysql_rejects_bad_url_and_redacts_connect_errors() {
    assert!(MysqlRowReader::new("").is_err());
    assert!(MysqlRowReader::new("not a url").is_err());

    let port = closed_port();
    let url = format!("mysql://user:supersecret@127.0.0.1:{port}/db");
    let text = match MysqlRowReader::new(&url) {
        Err(err) => format!("{err} {err:?}"),
        Ok(reader) => {
            let err = reader
                .query_rows("SELECT 1", &[DbParam::Integer(1)])
                .unwrap_err();
            assert!(matches!(err, Dok3Error::Storage(_)));
            format!("{err} {err:?}")
        }
    };
    assert!(!text.contains("supersecret"), "{text}");
}
