//! Small filesystem helpers shared by the store and journal.

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use dok3_core::Dok3Error;

static TMP_SEQ: AtomicU64 = AtomicU64::new(0);

pub(crate) fn storage_err(context: &str, err: impl std::fmt::Display) -> Dok3Error {
    Dok3Error::Storage(format!("{context}: {err}"))
}

/// Writes `bytes` to a sibling temp file, fsyncs it, then renames over `path`.
pub(crate) fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), Dok3Error> {
    let dir = path
        .parent()
        .ok_or_else(|| storage_err("atomic write", "path has no parent"))?;
    fs::create_dir_all(dir).map_err(|e| storage_err("create dir", e))?;
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| storage_err("atomic write", "invalid file name"))?;
    let seq = TMP_SEQ.fetch_add(1, Ordering::Relaxed);
    let tmp = dir.join(format!(".{file_name}.tmp.{}.{seq}", std::process::id()));
    let result = (|| {
        let mut f = File::create(&tmp).map_err(|e| storage_err("create temp", e))?;
        f.write_all(bytes)
            .map_err(|e| storage_err("write temp", e))?;
        f.sync_all().map_err(|e| storage_err("fsync temp", e))?;
        fs::rename(&tmp, path).map_err(|e| storage_err("rename temp", e))
    })();
    if result.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    result
}
