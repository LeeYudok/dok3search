//! Source connectors for local files, Git, SVN, and relational databases.

mod local_fs;
mod sqlite;

pub use local_fs::{
    DEFAULT_MAX_FILE_BYTES, LocalFileConnector, ScanReport, ScannedFile, SkippedFile, SourceConfig,
    build_document,
};
pub use sqlite::{
    SqliteColumnMapping, SqliteCursor, SqliteCursorMode, SqliteSourceAdapter, SqliteSourceBatch,
    SqliteSourceConfig,
};
