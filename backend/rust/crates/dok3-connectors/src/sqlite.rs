//! Read-only SQLite source adapter with cursor-based incremental collection.

use std::path::{Path, PathBuf};
use std::time::Duration;

use dok3_core::Dok3Error;
use dok3_domain::{
    AclScope, Document, DocumentId, DocumentSection, RevisionId, SectionId, SourceId,
};
use rusqlite::types::{Value, ValueRef};
use rusqlite::{Connection, OpenFlags, Row, params};
use sha2::{Digest, Sha256};

/// Column mapping from a configured SELECT result to a domain document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SqliteColumnMapping {
    pub document_id: String,
    pub title: String,
    pub body: String,
    pub summary: Option<String>,
}

/// Supported incremental cursor columns for the first SQLite adapter slice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SqliteCursorMode {
    /// Monotonic integer primary key. Updates to existing keys are not detected.
    IntegerPk { column: String },
    /// Sortable timestamp watermark, normally an ISO-8601 TEXT column.
    UpdatedAt { column: String },
}

impl SqliteCursorMode {
    fn cursor_column(&self) -> &str {
        match self {
            Self::IntegerPk { column } | Self::UpdatedAt { column } => column,
        }
    }
}

/// Last successfully observed source position.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SqliteCursor {
    Integer(i64),
    UpdatedAt(String),
}

/// Configuration for [`SqliteSourceAdapter`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SqliteSourceConfig {
    pub source_id: SourceId,
    pub source_label: String,
    pub database_path: PathBuf,
    pub select_query: String,
    pub columns: SqliteColumnMapping,
    pub cursor: SqliteCursorMode,
    pub category: String,
    pub language: String,
    pub default_tag: String,
    pub acl_scope: AclScope,
}

impl SqliteSourceConfig {
    /// Creates a configuration with database-oriented default metadata.
    #[must_use]
    pub fn new(
        source_id: SourceId,
        source_label: impl Into<String>,
        database_path: impl Into<PathBuf>,
        select_query: impl Into<String>,
        columns: SqliteColumnMapping,
        cursor: SqliteCursorMode,
    ) -> Self {
        Self {
            source_id,
            source_label: source_label.into(),
            database_path: database_path.into(),
            select_query: select_query.into(),
            columns,
            cursor,
            category: "database".to_string(),
            language: "ko".to_string(),
            default_tag: "database".to_string(),
            acl_scope: AclScope::Public,
        }
    }
}

/// Documents and next cursor produced by one incremental collection pass.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SqliteSourceBatch {
    pub documents: Vec<Document>,
    pub next_cursor: Option<SqliteCursor>,
    pub has_more: bool,
}

/// Reads configured SELECT rows from a SQLite file without opening it for writes.
#[derive(Debug, Clone)]
pub struct SqliteSourceAdapter {
    config: SqliteSourceConfig,
    normalized_query: String,
}

impl SqliteSourceAdapter {
    /// Validates column names and the SELECT-only query before constructing the adapter.
    pub fn new(config: SqliteSourceConfig) -> Result<Self, Dok3Error> {
        validate_config(&config)?;
        let normalized_query = normalize_select_query(&config.select_query)?;
        Ok(Self {
            config,
            normalized_query,
        })
    }

    /// Returns the immutable adapter configuration.
    #[must_use]
    pub fn config(&self) -> &SqliteSourceConfig {
        &self.config
    }

    /// Collects rows after `cursor`, ordered by the configured cursor column.
    ///
    /// A fresh read-only connection is opened for each call so committed source
    /// changes are visible without keeping a long-lived transaction. Physical
    /// source deletes are reconciled by later P4 work and are not emitted here.
    pub fn collect(&self, cursor: Option<&SqliteCursor>) -> Result<SqliteSourceBatch, Dok3Error> {
        let connection = open_read_only(&self.config.database_path)?;
        let cursor_value = self.cursor_value(cursor)?;
        let sql = self.collection_sql(cursor_value.is_some());
        let mut statement = connection
            .prepare(&sql)
            .map_err(|error| sqlite_error("prepare SELECT", &error))?;
        let mut rows = match cursor_value.as_ref() {
            Some(value) => statement
                .query(params![value])
                .map_err(|error| sqlite_error("execute SELECT", &error))?,
            None => statement
                .query([])
                .map_err(|error| sqlite_error("execute SELECT", &error))?,
        };

        let mut documents = Vec::new();
        let mut next_cursor = cursor.cloned();
        while let Some(row) = rows
            .next()
            .map_err(|error| sqlite_error("read SELECT row", &error))?
        {
            let row_cursor = self.read_row_cursor(row)?;
            let document = self.convert_row(row)?;
            next_cursor = Some(row_cursor);
            documents.push(document);
        }

        Ok(SqliteSourceBatch {
            documents,
            next_cursor,
            has_more: false,
        })
    }

    fn collection_sql(&self, has_cursor: bool) -> String {
        let column = quote_identifier(self.config.cursor.cursor_column());
        if has_cursor {
            format!(
                "SELECT * FROM ({}) AS dok3_source WHERE {column} > ?1 ORDER BY {column} ASC",
                self.normalized_query
            )
        } else {
            format!(
                "SELECT * FROM ({}) AS dok3_source ORDER BY {column} ASC",
                self.normalized_query
            )
        }
    }

    fn cursor_value(&self, cursor: Option<&SqliteCursor>) -> Result<Option<Value>, Dok3Error> {
        match (cursor, &self.config.cursor) {
            (None, _) => Ok(None),
            (Some(SqliteCursor::Integer(value)), SqliteCursorMode::IntegerPk { .. }) => {
                Ok(Some(Value::Integer(*value)))
            }
            (Some(SqliteCursor::UpdatedAt(value)), SqliteCursorMode::UpdatedAt { .. }) => {
                Ok(Some(Value::Text(value.clone())))
            }
            (Some(_), _) => Err(Dok3Error::InvalidInput(
                "cursor variant does not match configured SQLite cursor mode".to_string(),
            )),
        }
    }

    fn read_row_cursor(&self, row: &Row<'_>) -> Result<SqliteCursor, Dok3Error> {
        match &self.config.cursor {
            SqliteCursorMode::IntegerPk { column } => {
                let value = match row
                    .get_ref(column.as_str())
                    .map_err(|error| sqlite_error("read integer cursor column", &error))?
                {
                    ValueRef::Integer(value) => value,
                    ValueRef::Text(bytes) => std::str::from_utf8(bytes)
                        .map_err(|error| {
                            Dok3Error::Storage(format!(
                                "integer cursor column '{column}' is not UTF-8: {error}"
                            ))
                        })?
                        .parse::<i64>()
                        .map_err(|error| {
                            Dok3Error::Storage(format!(
                                "integer cursor column '{column}' is not an i64: {error}"
                            ))
                        })?,
                    other => {
                        return Err(Dok3Error::Storage(format!(
                            "integer cursor column '{column}' has incompatible SQLite type {other:?}"
                        )));
                    }
                };
                Ok(SqliteCursor::Integer(value))
            }
            SqliteCursorMode::UpdatedAt { column } => {
                let value = read_required_text(row, column, "updated_at watermark")?;
                if value.trim().is_empty() {
                    return Err(Dok3Error::Storage(format!(
                        "updated_at watermark column '{column}' must not be empty"
                    )));
                }
                Ok(SqliteCursor::UpdatedAt(value))
            }
        }
    }

    fn convert_row(&self, row: &Row<'_>) -> Result<Document, Dok3Error> {
        let source_row_id =
            read_required_text(row, &self.config.columns.document_id, "document ID column")?;
        if source_row_id.trim().is_empty() {
            return Err(Dok3Error::Storage(
                "document ID column must not be empty".to_string(),
            ));
        }
        let body = read_required_text(row, &self.config.columns.body, "body column")?;
        let title = read_optional_text(row, &self.config.columns.title, "title column")?
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| source_row_id.clone());
        let summary = match self.config.columns.summary.as_deref() {
            Some(column) => read_optional_text(row, column, "summary column")?,
            None => None,
        }
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| first_summary_line(&body, &title));

        let document_id = DocumentId::new(format!("{}:{source_row_id}", self.config.source_id));
        let content_hash = document_hash(&title, &body);
        let section_id = SectionId::new(format!("{document_id}:body"));
        let line_end = body.lines().count().max(1);
        let section = DocumentSection {
            section_id,
            heading: title.clone(),
            anchor: "body".to_string(),
            order: 0,
            line_start: 1,
            line_end,
            body: body.clone(),
        };
        let mut tags = Vec::new();
        if !self.config.default_tag.trim().is_empty() {
            tags.push(self.config.default_tag.trim().to_string());
        }
        let language = self.config.language.trim();
        if !language.is_empty() && !tags.iter().any(|tag| tag.eq_ignore_ascii_case(language)) {
            tags.push(language.to_string());
        }

        Ok(Document {
            id: document_id,
            revision_id: RevisionId::new(content_hash.clone()),
            source_id: self.config.source_id.clone(),
            source: self.config.source_label.clone(),
            title,
            summary,
            category: self.config.category.clone(),
            topic: None,
            tag: self.config.default_tag.clone(),
            tags,
            keywords: Vec::new(),
            language: self.config.language.clone(),
            version: "v1".to_string(),
            date: None,
            source_uri: format!(
                "sqlite://{}#row={source_row_id}",
                self.config.database_path.display()
            ),
            sample_path: None,
            acl_scope: self.config.acl_scope.clone(),
            content_hash,
            body,
            markdown: None,
            sections: vec![section],
        })
    }
}

fn validate_config(config: &SqliteSourceConfig) -> Result<(), Dok3Error> {
    if config.source_id.as_str().trim().is_empty() {
        return Err(Dok3Error::InvalidInput(
            "source_id must not be empty".to_string(),
        ));
    }
    if config.source_label.trim().is_empty() {
        return Err(Dok3Error::InvalidInput(
            "source_label must not be empty".to_string(),
        ));
    }
    if config.database_path.as_os_str().is_empty() {
        return Err(Dok3Error::InvalidInput(
            "database_path must not be empty".to_string(),
        ));
    }
    for (name, column) in [
        ("document_id", config.columns.document_id.as_str()),
        ("title", config.columns.title.as_str()),
        ("body", config.columns.body.as_str()),
    ] {
        if column.trim().is_empty() {
            return Err(Dok3Error::InvalidInput(format!(
                "{name} column must not be empty"
            )));
        }
    }
    if let Some(column) = config.columns.summary.as_deref()
        && column.trim().is_empty()
    {
        return Err(Dok3Error::InvalidInput(
            "summary column must not be empty when configured".to_string(),
        ));
    }
    if config.cursor.cursor_column().trim().is_empty() {
        return Err(Dok3Error::InvalidInput(
            "cursor column must not be empty".to_string(),
        ));
    }
    Ok(())
}

fn normalize_select_query(query: &str) -> Result<String, Dok3Error> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return Err(Dok3Error::InvalidInput(
            "select_query must not be empty".to_string(),
        ));
    }
    let without_trailing_semicolon = trimmed
        .strip_suffix(';')
        .map(str::trim_end)
        .unwrap_or(trimmed);
    if without_trailing_semicolon.contains(';') {
        return Err(Dok3Error::InvalidInput(
            "select_query must contain exactly one SELECT statement".to_string(),
        ));
    }
    let first_keyword = without_trailing_semicolon
        .split_whitespace()
        .next()
        .unwrap_or_default();
    if !first_keyword.eq_ignore_ascii_case("SELECT") {
        return Err(Dok3Error::InvalidInput(
            "select_query is SELECT-only; write and DDL statements are rejected".to_string(),
        ));
    }
    Ok(without_trailing_semicolon.to_string())
}

fn open_read_only(path: &Path) -> Result<Connection, Dok3Error> {
    if !path.is_file() {
        return Err(Dok3Error::NotFound(format!(
            "SQLite database '{}' does not exist",
            path.display()
        )));
    }
    let connection = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .map_err(|error| sqlite_error("open SQLite database read-only", &error))?;
    connection
        .busy_timeout(Duration::from_secs(1))
        .map_err(|error| sqlite_error("set SQLite busy_timeout", &error))?;
    connection
        .execute_batch("PRAGMA query_only = ON;")
        .map_err(|error| sqlite_error("enable SQLite query_only", &error))?;
    Ok(connection)
}

fn read_required_text(row: &Row<'_>, column: &str, label: &str) -> Result<String, Dok3Error> {
    read_optional_text(row, column, label)?
        .ok_or_else(|| Dok3Error::Storage(format!("{label} '{column}' must not be NULL")))
}

fn read_optional_text(
    row: &Row<'_>,
    column: &str,
    label: &str,
) -> Result<Option<String>, Dok3Error> {
    match row
        .get_ref(column)
        .map_err(|error| sqlite_error(&format!("read {label} '{column}'"), &error))?
    {
        ValueRef::Null => Ok(None),
        ValueRef::Integer(value) => Ok(Some(value.to_string())),
        ValueRef::Real(value) => Ok(Some(value.to_string())),
        ValueRef::Text(bytes) => std::str::from_utf8(bytes)
            .map(str::to_string)
            .map(Some)
            .map_err(|error| {
                Dok3Error::Storage(format!("{label} '{column}' is not UTF-8: {error}"))
            }),
        ValueRef::Blob(_) => Err(Dok3Error::Storage(format!(
            "{label} '{column}' must be text, not BLOB"
        ))),
    }
}

fn first_summary_line(body: &str, title: &str) -> String {
    for line in body.lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            return trimmed.chars().take(200).collect();
        }
    }
    title.chars().take(200).collect()
}

fn document_hash(title: &str, body: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(title.as_bytes());
    hasher.update(b"\n\n");
    hasher.update(body.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn quote_identifier(identifier: &str) -> String {
    format!("\"{}\"", identifier.replace('"', "\"\""))
}

fn sqlite_error(context: &str, error: &rusqlite::Error) -> Dok3Error {
    Dok3Error::Storage(format!("{context}: {error}"))
}
