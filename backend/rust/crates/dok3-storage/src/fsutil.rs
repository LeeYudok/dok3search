//! Small filesystem helpers shared by the store and journal.

use std::fs::{self, File, OpenOptions, TryLockError};
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::Duration;

use dok3_core::Dok3Error;

static TMP_SEQ: AtomicU64 = AtomicU64::new(0);
const RENAME_RETRY_DELAYS_MS: [u64; 4] = [10, 20, 40, 80];

pub(crate) fn storage_err(context: &str, err: impl std::fmt::Display) -> Dok3Error {
    Dok3Error::Storage(format!("{context}: {err}"))
}

/// Acquires a non-blocking exclusive advisory file lock to enforce single-writer access.
pub(crate) fn acquire_exclusive_lock(path: &Path) -> Result<File, Dok3Error> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| storage_err("create lock dir", e))?;
    }
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)
        .map_err(|e| storage_err("open writer lock", e))?;
    match file.try_lock() {
        Ok(()) => Ok(file),
        Err(TryLockError::WouldBlock) => Err(Dok3Error::Storage(format!(
            "writer lock '{}' is already held by another process or instance",
            path.display()
        ))),
        Err(TryLockError::Error(e)) => Err(storage_err("acquire writer lock", e)),
    }
}

/// Fsyncs a directory on Unix so newly created or renamed entries are durable.
pub(crate) fn sync_dir(dir: &Path) -> Result<(), Dok3Error> {
    #[cfg(unix)]
    {
        let handle = File::open(dir).map_err(|e| storage_err("open dir for fsync", e))?;
        handle.sync_all().map_err(|e| storage_err("fsync dir", e))?;
    }
    #[cfg(not(unix))]
    {
        let _ = dir;
    }
    Ok(())
}

/// Removes leftover `.tmp.` staging files left behind by interrupted atomic writes.
pub(crate) fn cleanup_stale_temps(dir: &Path) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let Some(name) = entry.file_name().to_str().map(str::to_owned) else {
            continue;
        };
        if !name.starts_with('.') || !name.contains(".tmp.") {
            continue;
        }
        if entry.file_type().is_ok_and(|ft| ft.is_file()) {
            let _ = fs::remove_file(entry.path());
        }
    }
}

/// Renames `from` over `to`, backing off briefly on `PermissionDenied` (Windows AV/EDR scans).
pub(crate) fn rename_with_retry(from: &Path, to: &Path) -> Result<(), Dok3Error> {
    let mut attempt = 0usize;
    loop {
        match fs::rename(from, to) {
            Ok(()) => return Ok(()),
            Err(err)
                if err.kind() == std::io::ErrorKind::PermissionDenied
                    && attempt < RENAME_RETRY_DELAYS_MS.len() =>
            {
                thread::sleep(Duration::from_millis(RENAME_RETRY_DELAYS_MS[attempt]));
                attempt += 1;
            }
            Err(err) => return Err(storage_err("rename temp", err)),
        }
    }
}

/// Writes `bytes` to a sibling temp file, fsyncs it, renames over `path`, and fsyncs `dir`.
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
        rename_with_retry(&tmp, path)?;
        sync_dir(dir)
    })();
    if result.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    result
}

/// Atomically creates `path` with `bytes` only if `path` does not already exist.
/// Returns `Ok(true)` if newly created, `Ok(false)` if `path` already existed untouched.
pub(crate) fn atomic_write_if_absent(path: &Path, bytes: &[u8]) -> Result<bool, Dok3Error> {
    let dir = path
        .parent()
        .ok_or_else(|| storage_err("atomic create-if-absent", "path has no parent"))?;
    fs::create_dir_all(dir).map_err(|e| storage_err("create dir", e))?;
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| storage_err("atomic create-if-absent", "invalid file name"))?;
    let seq = TMP_SEQ.fetch_add(1, Ordering::Relaxed);
    let tmp = dir.join(format!(".{file_name}.tmp.{}.{seq}", std::process::id()));
    let write_res = (|| {
        let mut f = File::create(&tmp).map_err(|e| storage_err("create temp", e))?;
        f.write_all(bytes)
            .map_err(|e| storage_err("write temp", e))?;
        f.sync_all().map_err(|e| storage_err("fsync temp", e))
    })();
    if let Err(err) = write_res {
        let _ = fs::remove_file(&tmp);
        return Err(err);
    }
    match fs::hard_link(&tmp, path) {
        Ok(()) => {
            let _ = fs::remove_file(&tmp);
            sync_dir(dir)?;
            Ok(true)
        }
        Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {
            let _ = fs::remove_file(&tmp);
            Ok(false)
        }
        Err(err) => {
            let _ = fs::remove_file(&tmp);
            Err(storage_err("link immutable object", err))
        }
    }
}
