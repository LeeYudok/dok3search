//! Pure-Rust PostgreSQL and MySQL/MariaDB [`DbRowReader`] drivers (no OpenSSL / C deps).
//!
//! Both drivers run every query inside a read-only transaction, return all values as
//! strings, and redact the connection password from every error message.

use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::Write as _;
use std::str::FromStr;
use std::time::Duration;

use dok3_core::Dok3Error;
use mysql::prelude::Queryable;
use mysql::{Opts, OptsBuilder, Params, Pool, PoolConstraints, PoolOpts, Value};
use postgres::types::{ToSql, Type};
use postgres::{Config, NoTls};

use crate::relational::{DbDialect, DbParam, DbRow, DbRowReader};
use crate::relational::{RelationalSourceAdapter, RelationalSourceConfig};

/// Default TCP connect timeout for PostgreSQL connections.
const PG_CONNECT_TIMEOUT: Duration = Duration::from_secs(5);

/// Extracts the password from `scheme://user:password@host/...` style URLs.
fn url_password(url: &str) -> Option<String> {
    let rest = url.split_once("://")?.1;
    let authority = rest.split(['/', '?']).next()?;
    let (userinfo, _) = authority.rsplit_once('@')?;
    let (_, password) = userinfo.split_once(':')?;
    (!password.is_empty()).then(|| password.to_string())
}

fn redact(message: &str, secrets: &[String]) -> String {
    let mut out = message.to_string();
    for secret in secrets.iter().filter(|s| !s.is_empty()) {
        out = out.replace(secret.as_str(), "***");
    }
    out
}

fn error_chain(err: &(dyn StdError + 'static)) -> String {
    let mut text = err.to_string();
    let mut source = err.source();
    while let Some(inner) = source {
        let _ = write!(text, ": {inner}");
        source = inner.source();
    }
    text
}

fn storage_err(context: &str, err: &(dyn StdError + 'static), secrets: &[String]) -> Dok3Error {
    Dok3Error::Storage(redact(&format!("{context}: {}", error_chain(err)), secrets))
}

/// PostgreSQL read-only row reader (plain TCP, no TLS).
pub struct PostgresRowReader {
    config: Config,
    secrets: Vec<String>,
}

impl PostgresRowReader {
    /// Validates a `postgres://` / `postgresql://` DSN. Does not connect.
    pub fn new(dsn: &str) -> Result<Self, Dok3Error> {
        let dsn = dsn.trim();
        if dsn.is_empty() {
            return Err(Dok3Error::InvalidInput(
                "postgres dsn must not be empty".to_string(),
            ));
        }
        if !(dsn.starts_with("postgres://") || dsn.starts_with("postgresql://")) {
            return Err(Dok3Error::InvalidInput(
                "postgres dsn must start with postgres:// or postgresql://".to_string(),
            ));
        }
        let mut secrets: Vec<String> = url_password(dsn).into_iter().collect();
        let mut config = Config::from_str(dsn).map_err(|err| {
            Dok3Error::InvalidInput(redact(
                &format!("invalid postgres dsn: {}", error_chain(&err)),
                &secrets,
            ))
        })?;
        if let Some(pw) = config.get_password() {
            let pw = String::from_utf8_lossy(pw).into_owned();
            if !pw.is_empty() && !secrets.contains(&pw) {
                secrets.push(pw);
            }
        }
        if config.get_connect_timeout().is_none() {
            config.connect_timeout(PG_CONNECT_TIMEOUT);
        }
        Ok(Self { config, secrets })
    }

    fn run(&self, sql: &str, params: &[DbParam]) -> Result<Vec<DbRow>, Dok3Error> {
        let fail = |context: &str, err: postgres::Error| storage_err(context, &err, &self.secrets);
        let mut client = self
            .config
            .connect(NoTls)
            .map_err(|e| fail("postgres connect failed", e))?;
        let mut tx = client
            .build_transaction()
            .read_only(true)
            .start()
            .map_err(|e| fail("postgres begin read-only transaction failed", e))?;
        let stmt = tx
            .prepare(sql)
            .map_err(|e| fail("postgres prepare failed", e))?;
        if stmt.params().len() != params.len() {
            return Err(Dok3Error::Storage(format!(
                "postgres statement expects {} parameters, got {}",
                stmt.params().len(),
                params.len()
            )));
        }
        let mut bound: Vec<Box<dyn ToSql + Sync>> = Vec::with_capacity(params.len());
        for (param, ty) in params.iter().zip(stmt.params()) {
            bound.push(bind_pg_param(param, ty)?);
        }
        let refs: Vec<&(dyn ToSql + Sync)> = bound.iter().map(|b| &**b).collect();
        let rows = tx
            .query(&stmt, &refs)
            .map_err(|e| fail("postgres query failed", e))?;
        let mut out = Vec::with_capacity(rows.len());
        for row in &rows {
            out.push(decode_pg_row(row, &self.secrets)?);
        }
        tx.rollback()
            .map_err(|e| fail("postgres rollback failed", e))?;
        Ok(out)
    }
}

fn bind_pg_param(param: &DbParam, ty: &Type) -> Result<Box<dyn ToSql + Sync>, Dok3Error> {
    let range_err =
        |v: i64| Dok3Error::InvalidInput(format!("integer parameter {v} out of range for {ty}"));
    match param {
        DbParam::Integer(v) => {
            if *ty == Type::INT8 {
                Ok(Box::new(*v))
            } else if *ty == Type::INT4 {
                Ok(Box::new(i32::try_from(*v).map_err(|_| range_err(*v))?))
            } else if *ty == Type::INT2 {
                Ok(Box::new(i16::try_from(*v).map_err(|_| range_err(*v))?))
            } else if *ty == Type::TEXT || *ty == Type::VARCHAR {
                Ok(Box::new(v.to_string()))
            } else {
                Err(Dok3Error::InvalidInput(format!(
                    "cannot bind integer parameter to postgres type {ty}"
                )))
            }
        }
        DbParam::Text(s) => {
            if *ty == Type::TEXT || *ty == Type::VARCHAR || *ty == Type::BPCHAR {
                Ok(Box::new(s.clone()))
            } else {
                Err(Dok3Error::InvalidInput(format!(
                    "cannot bind text parameter to postgres type {ty}"
                )))
            }
        }
    }
}

fn decode_pg_row(row: &postgres::Row, secrets: &[String]) -> Result<DbRow, Dok3Error> {
    let mut columns = BTreeMap::new();
    for (idx, col) in row.columns().iter().enumerate() {
        let ty = col.type_();
        let wrap = |err: postgres::Error| storage_err("postgres decode failed", &err, secrets);
        let value: Option<String> = if *ty == Type::BOOL {
            row.try_get::<_, Option<bool>>(idx)
                .map_err(wrap)?
                .map(|v| v.to_string())
        } else if *ty == Type::INT2 {
            row.try_get::<_, Option<i16>>(idx)
                .map_err(wrap)?
                .map(|v| v.to_string())
        } else if *ty == Type::INT4 {
            row.try_get::<_, Option<i32>>(idx)
                .map_err(wrap)?
                .map(|v| v.to_string())
        } else if *ty == Type::INT8 {
            row.try_get::<_, Option<i64>>(idx)
                .map_err(wrap)?
                .map(|v| v.to_string())
        } else if *ty == Type::FLOAT4 {
            row.try_get::<_, Option<f32>>(idx)
                .map_err(wrap)?
                .map(|v| v.to_string())
        } else if *ty == Type::FLOAT8 {
            row.try_get::<_, Option<f64>>(idx)
                .map_err(wrap)?
                .map(|v| v.to_string())
        } else if *ty == Type::TEXT
            || *ty == Type::VARCHAR
            || *ty == Type::BPCHAR
            || *ty == Type::NAME
        {
            row.try_get::<_, Option<String>>(idx).map_err(wrap)?
        } else {
            return Err(Dok3Error::Storage(format!(
                "unsupported postgres column type '{ty}' for column '{}'; cast it to text in the select query",
                col.name()
            )));
        };
        columns.insert(col.name().to_string(), value);
    }
    Ok(DbRow { columns })
}

impl DbRowReader for PostgresRowReader {
    fn query_rows(&self, sql: &str, params: &[DbParam]) -> Result<Vec<DbRow>, Dok3Error> {
        self.run(sql, params)
    }
}

/// MySQL / MariaDB read-only row reader.
pub struct MysqlRowReader {
    pool: Pool,
    secrets: Vec<String>,
}

impl MysqlRowReader {
    /// Validates a `mysql://` URL and creates a lazily connecting pool (0..4 connections).
    pub fn new(url: &str) -> Result<Self, Dok3Error> {
        let url = url.trim();
        if url.is_empty() {
            return Err(Dok3Error::InvalidInput(
                "mysql url must not be empty".to_string(),
            ));
        }
        let mut secrets: Vec<String> = url_password(url).into_iter().collect();
        let opts = Opts::from_url(url).map_err(|err| {
            Dok3Error::InvalidInput(redact(
                &format!("invalid mysql url: {}", error_chain(&err)),
                &secrets,
            ))
        })?;
        if let Some(pw) = opts.get_pass()
            && !pw.is_empty()
            && !secrets.iter().any(|s| s == pw)
        {
            secrets.push(pw.to_string());
        }
        let constraints = PoolConstraints::new(0, 4)
            .ok_or_else(|| Dok3Error::Internal("invalid mysql pool constraints".to_string()))?;
        let opts = OptsBuilder::from_opts(opts)
            .pool_opts(PoolOpts::default().with_constraints(constraints));
        let pool =
            Pool::new(opts).map_err(|err| storage_err("mysql pool failed", &err, &secrets))?;
        Ok(Self { pool, secrets })
    }

    fn run(&self, sql: &str, params: &[DbParam]) -> Result<Vec<DbRow>, Dok3Error> {
        let fail = |context: &str, err: mysql::Error| storage_err(context, &err, &self.secrets);
        let mut conn = self
            .pool
            .get_conn()
            .map_err(|e| fail("mysql connect failed", e))?;
        conn.query_drop("SET SESSION TRANSACTION READ ONLY")
            .map_err(|e| fail("mysql set read-only failed", e))?;
        conn.query_drop("START TRANSACTION READ ONLY")
            .map_err(|e| fail("mysql begin read-only transaction failed", e))?;
        let result = Self::fetch(&mut conn, sql, params);
        let rollback = conn.query_drop("ROLLBACK");
        let rows = result.map_err(|e| fail("mysql query failed", e))?;
        rollback.map_err(|e| fail("mysql rollback failed", e))?;
        Ok(rows)
    }

    fn fetch(
        conn: &mut mysql::PooledConn,
        sql: &str,
        params: &[DbParam],
    ) -> Result<Vec<DbRow>, mysql::Error> {
        let values: Vec<Value> = params
            .iter()
            .map(|p| match p {
                DbParam::Integer(v) => Value::Int(*v),
                DbParam::Text(s) => Value::Bytes(s.clone().into_bytes()),
            })
            .collect();
        let params = if values.is_empty() {
            Params::Empty
        } else {
            Params::Positional(values)
        };
        let result = conn.exec_iter(sql, params)?;
        let mut out = Vec::new();
        for row in result {
            let row = row?;
            let mut columns = BTreeMap::new();
            for (idx, col) in row.columns_ref().iter().enumerate() {
                let value = row.as_ref(idx).and_then(mysql_value_to_string);
                columns.insert(col.name_str().into_owned(), value);
            }
            out.push(DbRow { columns });
        }
        Ok(out)
    }
}

impl DbRowReader for MysqlRowReader {
    fn query_rows(&self, sql: &str, params: &[DbParam]) -> Result<Vec<DbRow>, Dok3Error> {
        self.run(sql, params)
    }
}

/// Converts a MySQL wire [`Value`] into its string form (`NULL` → `None`).
#[must_use]
pub fn mysql_value_to_string(value: &Value) -> Option<String> {
    match value {
        Value::NULL => None,
        Value::Bytes(bytes) => Some(String::from_utf8_lossy(bytes).into_owned()),
        Value::Int(v) => Some(v.to_string()),
        Value::UInt(v) => Some(v.to_string()),
        Value::Float(v) => Some(v.to_string()),
        Value::Double(v) => Some(v.to_string()),
        Value::Date(y, mo, d, h, mi, s, micro) => {
            let mut text = format!("{y:04}-{mo:02}-{d:02} {h:02}:{mi:02}:{s:02}");
            if *micro != 0 {
                let _ = write!(text, ".{micro:06}");
            }
            Some(text)
        }
        Value::Time(neg, days, h, mi, s, micro) => {
            let hours = *days * 24 + u32::from(*h);
            let sign = if *neg { "-" } else { "" };
            let mut text = format!("{sign}{hours:02}:{mi:02}:{s:02}");
            if *micro != 0 {
                let _ = write!(text, ".{micro:06}");
            }
            Some(text)
        }
    }
}

impl RelationalSourceAdapter<PostgresRowReader> {
    /// Creates a PostgreSQL-backed adapter; `config.dialect` must be `PostgreSql`.
    pub fn new_postgres(config: RelationalSourceConfig, dsn: &str) -> Result<Self, Dok3Error> {
        if config.dialect != DbDialect::PostgreSql {
            return Err(Dok3Error::InvalidInput(format!(
                "new_postgres requires the PostgreSql dialect, got {:?}",
                config.dialect
            )));
        }
        Self::new(config, PostgresRowReader::new(dsn)?)
    }
}

impl RelationalSourceAdapter<MysqlRowReader> {
    /// Creates a MySQL/MariaDB-backed adapter; `config.dialect` must be `MySql` or `MariaDb`.
    pub fn new_mysql(config: RelationalSourceConfig, url: &str) -> Result<Self, Dok3Error> {
        if !matches!(config.dialect, DbDialect::MySql | DbDialect::MariaDb) {
            return Err(Dok3Error::InvalidInput(format!(
                "new_mysql requires the MySql or MariaDb dialect, got {:?}",
                config.dialect
            )));
        }
        Self::new(config, MysqlRowReader::new(url)?)
    }
}
