//! Source connectors for local files, Git, SVN, and relational databases.

mod local_fs;

pub use local_fs::{
    DEFAULT_MAX_FILE_BYTES, LocalFileConnector, ScanReport, ScannedFile, SkippedFile, SourceConfig,
    build_document,
};
