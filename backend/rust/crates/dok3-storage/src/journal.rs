//! Append-only JSONL change journal implementing `ChangeJournalPort`.

use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use dok3_core::Dok3Error;
use dok3_domain::{ChangeEvent, ChangeJournalPort, Sequence};

use crate::fsutil::storage_err;

/// Durable change journal. The journal assigns sequences itself (last + 1);
/// the `sequence` field of an appended event is ignored and overwritten.
#[derive(Debug)]
pub struct FileChangeJournal {
    path: PathBuf,
    last: Mutex<u64>,
}

impl FileChangeJournal {
    /// Opens (or creates) `journal.jsonl` under `root` and recovers the last sequence.
    ///
    /// A trailing partial line left by a crash mid-append is ignored on read and
    /// truncated away here so later appends stay line-aligned.
    pub fn open(root: &Path) -> Result<Self, Dok3Error> {
        fs::create_dir_all(root).map_err(|e| storage_err("create journal dir", e))?;
        let path = root.join("journal.jsonl");
        let (events, valid_len) = read_all(&path)?;
        if path.exists() {
            let len = fs::metadata(&path)
                .map_err(|e| storage_err("stat journal", e))?
                .len();
            if len != valid_len {
                let f = OpenOptions::new()
                    .write(true)
                    .open(&path)
                    .map_err(|e| storage_err("open journal", e))?;
                f.set_len(valid_len)
                    .map_err(|e| storage_err("truncate journal", e))?;
                f.sync_all().map_err(|e| storage_err("fsync journal", e))?;
            }
        }
        let last = events.last().map_or(0, |e| e.sequence.get());
        Ok(Self {
            path,
            last: Mutex::new(last),
        })
    }

    fn lock(&self) -> Result<std::sync::MutexGuard<'_, u64>, Dok3Error> {
        self.last
            .lock()
            .map_err(|_| Dok3Error::Internal("journal mutex poisoned".to_string()))
    }
}

impl ChangeJournalPort for FileChangeJournal {
    fn append(&self, mut change: ChangeEvent) -> Result<Sequence, Dok3Error> {
        let mut last = self.lock()?;
        let next = *last + 1;
        change.sequence = Sequence::new(next);
        let mut line = serde_json::to_vec(&change).map_err(|e| storage_err("encode event", e))?;
        line.push(b'\n');
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|e| storage_err("open journal", e))?;
        f.write_all(&line)
            .map_err(|e| storage_err("append journal", e))?;
        f.sync_all().map_err(|e| storage_err("fsync journal", e))?;
        *last = next;
        Ok(Sequence::new(next))
    }

    fn read_after(&self, seq: Sequence, limit: usize) -> Result<Vec<ChangeEvent>, Dok3Error> {
        let _guard = self.lock()?;
        let (events, _) = read_all(&self.path)?;
        Ok(events
            .into_iter()
            .filter(|e| e.sequence > seq)
            .take(limit)
            .collect())
    }
}

/// Returns complete events and the byte length of the valid (newline-terminated) prefix.
fn read_all(path: &Path) -> Result<(Vec<ChangeEvent>, u64), Dok3Error> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok((Vec::new(), 0)),
        Err(e) => return Err(storage_err("open journal", e)),
    };
    let mut reader = BufReader::new(file);
    let mut events = Vec::new();
    let mut valid_len = 0u64;
    let mut buf = Vec::new();
    loop {
        buf.clear();
        let n = reader
            .read_until(b'\n', &mut buf)
            .map_err(|e| storage_err("read journal", e))?;
        if n == 0 || buf.last() != Some(&b'\n') {
            break; // EOF or torn trailing write
        }
        let event: ChangeEvent = serde_json::from_slice(&buf[..n - 1])
            .map_err(|e| storage_err(&format!("corrupt journal line at byte {valid_len}"), e))?;
        events.push(event);
        valid_len += n as u64;
    }
    Ok((events, valid_len))
}
