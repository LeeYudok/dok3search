//! Dialect-neutral read-only relational database source adapter
//! supporting SQLite, MySQL, MariaDB, PostgreSQL, and Oracle.

use std::collections::BTreeMap;

use dok3_core::Dok3Error;
use dok3_domain::{
    AclScope, Document, DocumentId, DocumentSection, RevisionId, SectionId, SourceId,
};
use sha2::{Digest, Sha256};

use crate::sql_guard::validate_readonly_select;
use crate::{SqliteColumnMapping, SqliteCursor, SqliteCursorMode};

/// Supported SQL database dialects for incremental cursor-based collection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DbDialect {
    Sqlite,
    MySql,
    MariaDb,
    PostgreSql,
    Oracle,
}

impl DbDialect {
    #[must_use]
    pub const fn uri_scheme(self) -> &'static str {
        match self {
            Self::Sqlite => "sqlite",
            Self::MySql => "mysql",
            Self::MariaDb => "mariadb",
            Self::PostgreSql => "postgres",
            Self::Oracle => "oracle",
        }
    }

    fn quote_ident(self, ident: &str) -> String {
        match self {
            Self::MySql | Self::MariaDb => format!("`{}`", ident.replace('`', "``")),
            Self::Sqlite | Self::PostgreSql | Self::Oracle => {
                format!("\"{}\"", ident.replace('"', "\"\""))
            }
        }
    }

    fn placeholder(self, index_1based: usize, name: &str) -> String {
        match self {
            Self::Sqlite | Self::MySql | Self::MariaDb => "?".to_string(),
            Self::PostgreSql => format!("${index_1based}"),
            Self::Oracle => format!(":{name}"),
        }
    }
}

/// Strategy for detecting soft-deleted rows in incremental database collection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TombstoneStrategy {
    /// Non-NULL timestamp column marks deleted rows.
    DeletedAtColumn(String),
    /// Truthy flag column (`1`, `true`, `Y`, `y`) marks deleted rows.
    BooleanFlagColumn(String),
}

impl TombstoneStrategy {
    fn column(&self) -> &str {
        match self {
            Self::DeletedAtColumn(c) | Self::BooleanFlagColumn(c) => c,
        }
    }

    fn is_deleted(&self, row: &DbRow) -> bool {
        match self {
            Self::DeletedAtColumn(col) => row.get_str(col).is_some_and(|v| !v.trim().is_empty()),
            Self::BooleanFlagColumn(col) => row.get_str(col).is_some_and(|v| {
                let norm = v.trim();
                norm == "1" || norm.eq_ignore_ascii_case("true") || norm.eq_ignore_ascii_case("y")
            }),
        }
    }
}

/// Bind parameter value handed to [`DbRowReader`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DbParam {
    Integer(i64),
    Text(String),
}

/// Normalized row returned by a [`DbRowReader`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DbRow {
    pub columns: BTreeMap<String, Option<String>>,
}

impl DbRow {
    #[must_use]
    pub fn get_str(&self, column: &str) -> Option<&str> {
        self.columns.get(column).and_then(|v| v.as_deref())
    }
}

/// Read-only query execution port implemented by dialect drivers or test fakes.
pub trait DbRowReader: Send + Sync {
    fn query_rows(&self, sql: &str, params: &[DbParam]) -> Result<Vec<DbRow>, Dok3Error>;
}

/// Configuration for [`RelationalSourceAdapter`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationalSourceConfig {
    pub dialect: DbDialect,
    pub source_id: SourceId,
    pub source_label: String,
    pub connection_label: String,
    pub select_query: String,
    pub columns: SqliteColumnMapping,
    pub cursor: SqliteCursorMode,
    pub tombstone: Option<TombstoneStrategy>,
    pub category: String,
    pub language: String,
    pub default_tag: String,
    pub acl_scope: AclScope,
    pub batch_limit: usize,
}

/// Incremental batch emitted by [`RelationalSourceAdapter::collect`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationalSourceBatch {
    pub documents: Vec<Document>,
    pub deleted_ids: Vec<DocumentId>,
    pub next_cursor: Option<SqliteCursor>,
    pub has_more: bool,
}

/// Dialect-neutral incremental database source adapter.
pub struct RelationalSourceAdapter<R: DbRowReader> {
    config: RelationalSourceConfig,
    normalized_query: String,
    reader: R,
}

impl<R: DbRowReader> RelationalSourceAdapter<R> {
    /// Validates `select_query` (SELECT/CTE only) and configuration before creating the adapter.
    pub fn new(config: RelationalSourceConfig, reader: R) -> Result<Self, Dok3Error> {
        if config.source_id.as_str().trim().is_empty() {
            return Err(Dok3Error::InvalidInput(
                "source_id must not be empty".to_string(),
            ));
        }
        if config.batch_limit == 0 {
            return Err(Dok3Error::InvalidInput(
                "batch_limit must be at least 1".to_string(),
            ));
        }
        let normalized_query = validate_readonly_select(&config.select_query)?;
        Ok(Self {
            config,
            normalized_query,
            reader,
        })
    }

    /// Builds the dialect-specific parameterized SQL and bind parameters for `cursor`.
    pub fn build_query(
        &self,
        cursor: Option<&SqliteCursor>,
    ) -> Result<(String, Vec<DbParam>), Dok3Error> {
        let mut params = Vec::new();
        let dialect = self.config.dialect;
        let (filter, order_by) = match (&self.config.cursor, cursor) {
            (SqliteCursorMode::IntegerPk { column }, Some(SqliteCursor::Integer(v))) => {
                let qcol = dialect.quote_ident(column);
                params.push(DbParam::Integer(*v));
                let ph = dialect.placeholder(params.len(), "cursor");
                (format!(" WHERE {qcol} > {ph}"), format!("{qcol} ASC"))
            }
            (SqliteCursorMode::IntegerPk { column }, None) => {
                let qcol = dialect.quote_ident(column);
                (String::new(), format!("{qcol} ASC"))
            }
            (
                SqliteCursorMode::UpdatedAt { column },
                Some(SqliteCursor::UpdatedAt { updated_at }),
            ) => {
                let qcol = dialect.quote_ident(column);
                let qdoc = dialect.quote_ident(&self.config.columns.document_id);
                params.push(DbParam::Text(updated_at.clone()));
                let ph = dialect.placeholder(params.len(), "cursor");
                (
                    format!(" WHERE {qcol} >= {ph}"),
                    format!("{qcol} ASC, {qdoc} ASC"),
                )
            }
            (SqliteCursorMode::UpdatedAt { column }, None) => {
                let qcol = dialect.quote_ident(column);
                let qdoc = dialect.quote_ident(&self.config.columns.document_id);
                (String::new(), format!("{qcol} ASC, {qdoc} ASC"))
            }
            _ => {
                return Err(Dok3Error::InvalidInput(
                    "cursor variant does not match configured cursor mode".to_string(),
                ));
            }
        };

        let fetch_limit = self.config.batch_limit.saturating_add(1);
        params.push(DbParam::Integer(
            i64::try_from(fetch_limit).unwrap_or(i64::MAX),
        ));
        let limit_ph = dialect.placeholder(params.len(), "limit");

        let alias = match dialect {
            DbDialect::Oracle => "dok3_source",
            _ => "AS dok3_source",
        };
        let limit_clause = match dialect {
            DbDialect::Oracle => format!("FETCH FIRST {limit_ph} ROWS ONLY"),
            _ => format!("LIMIT {limit_ph}"),
        };

        let sql = format!(
            "SELECT * FROM ({}\n) {alias}{filter} ORDER BY {order_by} {limit_clause}",
            self.normalized_query
        );
        Ok((sql, params))
    }

    /// Collects up to `batch_limit` documents and tombstone deletes after `cursor`.
    pub fn collect(
        &self,
        cursor: Option<&SqliteCursor>,
    ) -> Result<RelationalSourceBatch, Dok3Error> {
        let (sql, params) = self.build_query(cursor)?;
        let rows = self.reader.query_rows(&sql, &params)?;

        let mut documents = Vec::new();
        let mut deleted_ids = Vec::new();
        let mut next_cursor = cursor.cloned();
        let mut has_more = false;

        for (idx, row) in rows.iter().enumerate() {
            if idx >= self.config.batch_limit {
                has_more = true;
                break;
            }
            next_cursor = Some(self.read_cursor(row)?);
            let row_id = row
                .get_str(&self.config.columns.document_id)
                .filter(|s| !s.trim().is_empty())
                .ok_or_else(|| {
                    Dok3Error::Storage("document_id column must not be empty".to_string())
                })?;
            let doc_id = DocumentId::new(format!("{}:{row_id}", self.config.source_id));

            if let Some(tombstone) = &self.config.tombstone
                && tombstone.is_deleted(row)
            {
                let _ = tombstone.column();
                deleted_ids.push(doc_id);
                continue;
            }

            documents.push(self.convert_row(row, doc_id, row_id)?);
        }

        Ok(RelationalSourceBatch {
            documents,
            deleted_ids,
            next_cursor,
            has_more,
        })
    }

    fn read_cursor(&self, row: &DbRow) -> Result<SqliteCursor, Dok3Error> {
        match &self.config.cursor {
            SqliteCursorMode::IntegerPk { column } => {
                let raw = row.get_str(column).ok_or_else(|| {
                    Dok3Error::Storage(format!("integer cursor column '{column}' missing"))
                })?;
                let parsed = raw.trim().parse::<i64>().map_err(|err| {
                    Dok3Error::Storage(format!("integer cursor '{column}' invalid: {err}"))
                })?;
                Ok(SqliteCursor::Integer(parsed))
            }
            SqliteCursorMode::UpdatedAt { column } => {
                let raw = row
                    .get_str(column)
                    .filter(|s| !s.trim().is_empty())
                    .ok_or_else(|| {
                        Dok3Error::Storage(format!("updated_at cursor '{column}' missing"))
                    })?;
                Ok(SqliteCursor::UpdatedAt {
                    updated_at: raw.to_string(),
                })
            }
        }
    }

    fn convert_row(
        &self,
        row: &DbRow,
        doc_id: DocumentId,
        row_id: &str,
    ) -> Result<Document, Dok3Error> {
        let body = row
            .get_str(&self.config.columns.body)
            .ok_or_else(|| Dok3Error::Storage("body column must not be NULL".to_string()))?
            .to_string();
        let title = row
            .get_str(&self.config.columns.title)
            .filter(|s| !s.trim().is_empty())
            .unwrap_or(row_id)
            .to_string();
        let summary = self
            .config
            .columns
            .summary
            .as_deref()
            .and_then(|c| row.get_str(c))
            .filter(|s| !s.trim().is_empty())
            .map(str::to_string)
            .unwrap_or_else(|| {
                body.lines()
                    .find(|l| !l.trim().is_empty())
                    .unwrap_or(&title)
                    .trim()
                    .chars()
                    .take(200)
                    .collect()
            });

        let mut hasher = Sha256::new();
        hasher.update(title.as_bytes());
        hasher.update(b"\n\n");
        hasher.update(body.as_bytes());
        let content_hash = format!("{:x}", hasher.finalize());

        let section = DocumentSection {
            section_id: SectionId::new(format!("{doc_id}:body")),
            heading: title.clone(),
            anchor: "body".to_string(),
            order: 0,
            line_start: 1,
            line_end: body.lines().count().max(1),
            body: body.clone(),
        };

        Ok(Document {
            id: doc_id,
            revision_id: RevisionId::new(content_hash.clone()),
            source_id: self.config.source_id.clone(),
            source: self.config.source_label.clone(),
            title,
            summary,
            category: self.config.category.clone(),
            topic: None,
            tag: self.config.default_tag.clone(),
            tags: vec![
                self.config.default_tag.clone(),
                self.config.language.clone(),
            ],
            keywords: Vec::new(),
            language: self.config.language.clone(),
            version: "v1".to_string(),
            date: None,
            source_uri: format!(
                "{}://{}#row={row_id}",
                self.config.dialect.uri_scheme(),
                self.config.connection_label
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
