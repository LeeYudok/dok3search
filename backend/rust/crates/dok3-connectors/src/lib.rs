//! Source connectors for local files, Git, SVN, and relational databases.

pub mod drivers;
mod local_fs;
pub mod relational;
pub mod sql_guard;
mod sqlite;
pub mod vcs;

pub use drivers::{MysqlRowReader, PostgresRowReader, mysql_value_to_string};
pub use local_fs::{
    DEFAULT_DOCUMENT_EXTENSIONS, DEFAULT_MAX_FILE_BYTES, LocalFileConnector, ScanReport,
    ScannedFile, SkippedFile, SourceConfig, build_document,
};
pub use relational::{
    DbDialect, DbParam, DbRow, DbRowReader, RelationalSourceAdapter, RelationalSourceBatch,
    RelationalSourceConfig, TombstoneStrategy,
};
pub use sql_guard::validate_readonly_select;
pub use sqlite::{
    DEFAULT_SQLITE_BATCH_LIMIT, SqliteColumnMapping, SqliteCursor, SqliteCursorMode,
    SqliteSourceAdapter, SqliteSourceBatch, SqliteSourceConfig,
};
pub use vcs::{
    GitRepoConnector, StdVcsCommandRunner, SvnConnector, VcsBatch, VcsCommandOutput,
    VcsCommandRunner,
};
