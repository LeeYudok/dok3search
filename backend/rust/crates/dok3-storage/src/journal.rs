//! Append-only JSONL change journal implementing `ChangeJournalPort`.

use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use dok3_core::Dok3Error;
use dok3_domain::{ChangeEvent, ChangeJournalPort, Sequence};

use crate::fsutil::{acquire_exclusive_lock, storage_err, sync_dir};

#[derive(Debug)]
struct JournalState {
    last: u64,
    poisoned: bool,
    dir_synced: bool,
}

/// Durable single-writer change journal. Assigns sequences strictly monotonically (`last + 1`);
/// any caller-supplied `sequence` is overwritten.
#[derive(Debug)]
pub struct FileChangeJournal {
    _lock: File,
    path: PathBuf,
    state: Mutex<JournalState>,
}

impl FileChangeJournal {
    /// Opens (or creates) `journal.jsonl` under `root` with an exclusive `.journal.lock`,
    /// recovering from any partial or corrupt trailing line at EOF.
    pub fn open(root: &Path) -> Result<Self, Dok3Error> {
        fs::create_dir_all(root).map_err(|e| storage_err("create journal dir", e))?;
        let lock = acquire_exclusive_lock(&root.join(".journal.lock"))?;

        let path = root.join("journal.jsonl");
        let (events, valid_len) = read_all(&path)?;
        let mut dir_synced = false;
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
            sync_dir(root)?;
            dir_synced = true;
        }
        let last = events.last().map_or(0, |e| e.sequence.get());
        Ok(Self {
            _lock: lock,
            path,
            state: Mutex::new(JournalState {
                last,
                poisoned: false,
                dir_synced,
            }),
        })
    }

    fn lock_state(&self) -> Result<std::sync::MutexGuard<'_, JournalState>, Dok3Error> {
        self.state
            .lock()
            .map_err(|_| Dok3Error::Internal("journal mutex poisoned".to_string()))
    }

    /// Returns the latest monotonically assigned sequence number (`0` when empty).
    pub fn last_sequence(&self) -> Result<Sequence, Dok3Error> {
        let state = self.lock_state()?;
        if state.poisoned {
            return Err(Dok3Error::Storage(
                "change journal is poisoned after failed rollback; reopen required".to_string(),
            ));
        }
        Ok(Sequence::new(state.last))
    }

    fn append_with_hooks<S, W, R>(
        &self,
        mut change: ChangeEvent,
        sync_dir_fn: S,
        write_fn: W,
        rollback_fn: R,
    ) -> Result<Sequence, Dok3Error>
    where
        S: FnOnce(&Path) -> Result<(), Dok3Error>,
        W: FnOnce(&mut File, &[u8]) -> io::Result<()>,
        R: FnOnce(&mut File, u64) -> io::Result<()>,
    {
        let mut state = self.lock_state()?;
        if state.poisoned {
            return Err(Dok3Error::Storage(
                "change journal is poisoned after failed rollback; reopen required".to_string(),
            ));
        }

        let next = state.last + 1;
        change.sequence = Sequence::new(next);
        let mut line = serde_json::to_vec(&change).map_err(|e| storage_err("encode event", e))?;
        line.push(b'\n');

        let mut f = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(&self.path)
            .map_err(|e| storage_err("open journal", e))?;
        // Track directory durability explicitly so a retry continues syncing parent dir
        // even if a prior attempt already created an empty journal.jsonl on disk.
        if !state.dir_synced
            && let Some(parent) = self.path.parent()
        {
            sync_dir_fn(parent)?;
            state.dir_synced = true;
        }
        let prev_len = f
            .seek(SeekFrom::End(0))
            .map_err(|e| storage_err("seek journal end", e))?;

        if let Err(err) = write_fn(&mut f, &line) {
            if rollback_fn(&mut f, prev_len).is_err() {
                state.poisoned = true;
            }
            return Err(storage_err("append journal", err));
        }

        state.last = next;
        Ok(Sequence::new(next))
    }
}

fn write_and_sync(f: &mut File, line: &[u8]) -> io::Result<()> {
    f.write_all(line)?;
    f.sync_all()
}

fn rollback_truncate(f: &mut File, prev_len: u64) -> io::Result<()> {
    f.set_len(prev_len)?;
    f.sync_all()
}

impl ChangeJournalPort for FileChangeJournal {
    fn append(&self, change: ChangeEvent) -> Result<Sequence, Dok3Error> {
        self.append_with_hooks(change, sync_dir, write_and_sync, rollback_truncate)
    }

    fn read_after(&self, seq: Sequence, limit: usize) -> Result<Vec<ChangeEvent>, Dok3Error> {
        let state = self.lock_state()?;
        if state.poisoned {
            return Err(Dok3Error::Storage(
                "change journal is poisoned after failed rollback; reopen required".to_string(),
            ));
        }
        let (events, _) = read_all(&self.path)?;
        Ok(events
            .into_iter()
            .filter(|e| e.sequence > seq)
            .take(limit)
            .collect())
    }
}

/// Returns complete events and the byte length of the valid prefix.
/// Torn lines (no `\n`) and corrupt trailing lines right before EOF stop cleanly for truncation;
/// corrupt lines followed by more bytes return `Dok3Error::Storage`.
fn read_all(path: &Path) -> Result<(Vec<ChangeEvent>, u64), Dok3Error> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok((Vec::new(), 0)),
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
            break;
        }
        match serde_json::from_slice::<ChangeEvent>(&buf[..n - 1]) {
            Ok(event) => {
                events.push(event);
                valid_len += n as u64;
            }
            Err(err) => {
                let mut probe = [0u8; 1];
                let more = reader
                    .read(&mut probe)
                    .map_err(|io_e| storage_err("probe journal EOF", io_e))?;
                if more == 0 {
                    break;
                }
                return Err(storage_err(
                    &format!("corrupt journal line at byte {valid_len}"),
                    err,
                ));
            }
        }
    }
    Ok((events, valid_len))
}

#[cfg(test)]
mod tests {
    use std::io::{self, Write};
    use std::path::PathBuf;

    use dok3_core::Dok3Error;
    use dok3_domain::{
        ChangeEvent, ChangeJournalPort, ChangeKind, DocumentId, RevisionId, Sequence,
    };

    use super::{FileChangeJournal, rollback_truncate, write_and_sync};
    use crate::fsutil::sync_dir;

    struct TempDir(PathBuf);

    impl TempDir {
        fn new(name: &str) -> Self {
            let nanos = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let dir = std::env::temp_dir().join(format!(
                "dok3-jrn-unit-{name}-{}-{nanos}",
                std::process::id()
            ));
            std::fs::create_dir_all(&dir).unwrap();
            Self(dir)
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn sample_event(id: &str) -> ChangeEvent {
        ChangeEvent {
            sequence: Sequence::new(0),
            document_id: DocumentId::new(id),
            revision_id: RevisionId::new("sha256:test"),
            kind: ChangeKind::Upsert,
        }
    }

    #[test]
    fn failed_append_rolls_back_partial_write_and_preserves_sequence() {
        let dir = TempDir::new("rollback");
        let journal = FileChangeJournal::open(&dir.0).unwrap();
        assert_eq!(journal.append(sample_event("a")).unwrap(), Sequence::new(1));

        let err = journal
            .append_with_hooks(
                sample_event("partial"),
                sync_dir,
                |f, line| {
                    f.write_all(&line[..line.len() / 2])?;
                    Err(io::Error::other("simulated disk full"))
                },
                rollback_truncate,
            )
            .unwrap_err();
        assert!(matches!(err, Dok3Error::Storage(_)));

        // Next append must reclaim seq 2 cleanly without line corruption.
        assert_eq!(journal.append(sample_event("b")).unwrap(), Sequence::new(2));
        let all = journal.read_after(Sequence::new(0), 10).unwrap();
        assert_eq!(all.len(), 2);
        assert_eq!(all[1].document_id, DocumentId::new("b"));
        drop(journal);

        let reopened = FileChangeJournal::open(&dir.0).unwrap();
        assert_eq!(reopened.read_after(Sequence::new(0), 10).unwrap().len(), 2);
    }

    #[test]
    fn failed_initial_dir_sync_retries_sync_dir_on_next_append() {
        let dir = TempDir::new("dir-sync-retry");
        let journal = FileChangeJournal::open(&dir.0).unwrap();

        let err = journal
            .append_with_hooks(
                sample_event("first-fail"),
                |_| {
                    Err(Dok3Error::Storage(
                        "simulated dir fsync failure".to_string(),
                    ))
                },
                write_and_sync,
                rollback_truncate,
            )
            .unwrap_err();
        assert!(matches!(err, Dok3Error::Storage(msg) if msg.contains("dir fsync")));
        // journal.jsonl now exists on disk (empty) from the failed first attempt.
        assert!(dir.0.join("journal.jsonl").exists());

        // Retry even with existing empty file must STILL invoke sync_dir_fn before writing seq 1.
        let mut sync_calls = 0usize;
        let seq = journal
            .append_with_hooks(
                sample_event("recovered"),
                |parent| {
                    sync_calls += 1;
                    sync_dir(parent)
                },
                write_and_sync,
                rollback_truncate,
            )
            .unwrap();
        assert_eq!(sync_calls, 1);
        assert_eq!(seq, Sequence::new(1));

        // Subsequent append does not re-sync the directory unnecessarily.
        let mut subsequent_sync_calls = 0usize;
        let seq2 = journal
            .append_with_hooks(
                sample_event("second"),
                |_| {
                    subsequent_sync_calls += 1;
                    Ok(())
                },
                write_and_sync,
                rollback_truncate,
            )
            .unwrap();
        assert_eq!(subsequent_sync_calls, 0);
        assert_eq!(seq2, Sequence::new(2));
    }

    #[test]
    fn failed_rollback_poisons_journal_until_reopen() {
        let dir = TempDir::new("poison");
        let journal = FileChangeJournal::open(&dir.0).unwrap();

        let err = journal
            .append_with_hooks(
                sample_event("a"),
                sync_dir,
                |f, line| {
                    f.write_all(&line[..4])?;
                    Err(io::Error::other("write failed"))
                },
                |_f, _len| Err(io::Error::other("truncate failed")),
            )
            .unwrap_err();
        assert!(matches!(err, Dok3Error::Storage(_)));

        let poisoned_append = journal.append(sample_event("b")).unwrap_err();
        assert!(matches!(poisoned_append, Dok3Error::Storage(msg) if msg.contains("poisoned")));
        drop(journal);

        // Reopen truncates the torn fragment and restores usability.
        let reopened = FileChangeJournal::open(&dir.0).unwrap();
        assert_eq!(reopened.last_sequence().unwrap(), Sequence::new(0));
        assert_eq!(
            reopened.append(sample_event("c")).unwrap(),
            Sequence::new(1)
        );
        assert_eq!(reopened.last_sequence().unwrap(), Sequence::new(1));
    }
}
