//! Append-only JSONL change journal implementing `ChangeJournalPort`.

use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use dok3_core::Dok3Error;
use dok3_domain::{ChangeEvent, ChangeJournalPort, ChangeKind, DocumentId, RevisionId, Sequence};

use crate::fsutil::{acquire_exclusive_lock, storage_err, sync_dir};

#[derive(Debug)]
struct JournalState {
    last: u64,
    dir_synced: bool,
}

/// Durable single-writer change journal backed by SQLite `meta.db`.
#[derive(Debug)]
pub struct FileChangeJournal {
    _lock: File,
    root: PathBuf,
    state: Mutex<JournalState>,
}

impl FileChangeJournal {
    /// Opens (or creates) `meta.db` under `root` with an exclusive `.journal.lock`,
    /// migrating any legacy `journal.jsonl` (including torn-tail recovery).
    pub fn open(root: &Path) -> Result<Self, Dok3Error> {
        fs::create_dir_all(root).map_err(|e| storage_err("create journal dir", e))?;
        let lock = acquire_exclusive_lock(&root.join(".journal.lock"))?;

        let conn = crate::meta_db::open_meta_db(root)?;
        let max_seq: i64 = conn
            .query_row(
                "SELECT COALESCE(MAX(sequence), 0) FROM change_journal",
                [],
                |row| row.get(0),
            )
            .map_err(|e| Dok3Error::Storage(format!("query journal max sequence: {e}")))?;
        let mut last = u64::try_from(max_seq).unwrap_or(0);
        let legacy_path = root.join("journal.jsonl");
        let mut dir_synced = false;
        if legacy_path.is_file() {
            let (events, _) = read_all(&legacy_path)?;
            if !events.is_empty() {
                let tx = conn
                    .unchecked_transaction()
                    .map_err(|e| Dok3Error::Storage(format!("begin journal migration tx: {e}")))?;
                {
                    let mut stmt = tx
                        .prepare(
                            "INSERT OR REPLACE INTO change_journal (sequence, document_id, revision_id, kind) VALUES (?1, ?2, ?3, ?4)",
                        )
                        .map_err(|e| Dok3Error::Storage(format!("prepare journal migration: {e}")))?;
                    for ev in &events {
                        if ev.sequence.get() > last {
                            let kind = match ev.kind {
                                ChangeKind::Upsert => "upsert",
                                ChangeKind::Delete => "delete",
                            };
                            stmt.execute(rusqlite::params![
                                i64::try_from(ev.sequence.get()).unwrap_or(i64::MAX),
                                ev.document_id.as_str(),
                                ev.revision_id.as_str(),
                                kind
                            ])
                            .map_err(|e| {
                                Dok3Error::Storage(format!("migrate journal event: {e}"))
                            })?;
                            last = last.max(ev.sequence.get());
                        }
                    }
                }
                tx.commit()
                    .map_err(|e| Dok3Error::Storage(format!("commit journal migration: {e}")))?;
            }
            let _ = fs::rename(&legacy_path, root.join("journal.jsonl.migrated"));
            sync_dir(root)?;
            dir_synced = true;
        }
        Ok(Self {
            _lock: lock,
            root: root.to_path_buf(),
            state: Mutex::new(JournalState { last, dir_synced }),
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
        Ok(Sequence::new(state.last))
    }

    fn append_with_hooks<S, I>(
        &self,
        mut change: ChangeEvent,
        sync_dir_fn: S,
        insert_fn: I,
    ) -> Result<Sequence, Dok3Error>
    where
        S: FnOnce(&Path) -> Result<(), Dok3Error>,
        I: FnOnce(&rusqlite::Connection, &ChangeEvent) -> Result<(), Dok3Error>,
    {
        let mut state = self.lock_state()?;
        if !state.dir_synced {
            sync_dir_fn(&self.root)?;
            state.dir_synced = true;
        }

        let next = state.last + 1;
        change.sequence = Sequence::new(next);
        let conn = crate::meta_db::open_meta_db(&self.root)?;
        insert_fn(&conn, &change)?;

        state.last = next;
        Ok(Sequence::new(next))
    }
}

fn insert_event_db(conn: &rusqlite::Connection, change: &ChangeEvent) -> Result<(), Dok3Error> {
    let kind = match change.kind {
        ChangeKind::Upsert => "upsert",
        ChangeKind::Delete => "delete",
    };
    conn.execute(
        "INSERT INTO change_journal (sequence, document_id, revision_id, kind) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![
            i64::try_from(change.sequence.get()).unwrap_or(i64::MAX),
            change.document_id.as_str(),
            change.revision_id.as_str(),
            kind
        ],
    )
    .map_err(|e| Dok3Error::Storage(format!("append journal: {e}")))?;
    Ok(())
}

impl ChangeJournalPort for FileChangeJournal {
    fn append(&self, change: ChangeEvent) -> Result<Sequence, Dok3Error> {
        self.append_with_hooks(change, sync_dir, insert_event_db)
    }

    fn read_after(&self, seq: Sequence, limit: usize) -> Result<Vec<ChangeEvent>, Dok3Error> {
        let _state = self.lock_state()?;
        if limit == 0 {
            return Ok(Vec::new());
        }
        let conn = crate::meta_db::open_meta_db(&self.root)?;
        let mut stmt = conn
            .prepare(
                "SELECT sequence, document_id, revision_id, kind FROM change_journal WHERE sequence > ?1 ORDER BY sequence ASC LIMIT ?2",
            )
            .map_err(|e| Dok3Error::Storage(format!("prepare read_after: {e}")))?;
        let rows = stmt
            .query_map(
                rusqlite::params![
                    i64::try_from(seq.get()).unwrap_or(i64::MAX),
                    i64::try_from(limit).unwrap_or(i64::MAX)
                ],
                |row| {
                    let seq_raw: i64 = row.get(0)?;
                    let doc_id: String = row.get(1)?;
                    let rev_id: String = row.get(2)?;
                    let kind_str: String = row.get(3)?;
                    Ok((seq_raw, doc_id, rev_id, kind_str))
                },
            )
            .map_err(|e| Dok3Error::Storage(format!("query read_after: {e}")))?;
        let mut events = Vec::new();
        for row in rows {
            let (seq_raw, doc_id, rev_id, kind_str) =
                row.map_err(|e| Dok3Error::Storage(format!("read journal row: {e}")))?;
            let kind = match kind_str.as_str() {
                "upsert" => ChangeKind::Upsert,
                "delete" => ChangeKind::Delete,
                other => {
                    return Err(Dok3Error::Storage(format!(
                        "unknown journal kind '{other}'"
                    )));
                }
            };
            events.push(ChangeEvent {
                sequence: Sequence::new(u64::try_from(seq_raw).unwrap_or(0)),
                document_id: DocumentId::new(doc_id),
                revision_id: RevisionId::new(rev_id),
                kind,
            });
        }
        Ok(events)
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
    use std::path::PathBuf;

    use dok3_core::Dok3Error;
    use dok3_domain::{
        ChangeEvent, ChangeJournalPort, ChangeKind, DocumentId, RevisionId, Sequence,
    };

    use super::{FileChangeJournal, insert_event_db};
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
    fn failed_append_rolls_back_and_preserves_sequence() {
        let dir = TempDir::new("rollback");
        let journal = FileChangeJournal::open(&dir.0).unwrap();
        assert_eq!(journal.append(sample_event("a")).unwrap(), Sequence::new(1));

        let err = journal
            .append_with_hooks(sample_event("partial"), sync_dir, |_conn, _ev| {
                Err(Dok3Error::Storage("simulated db failure".to_string()))
            })
            .unwrap_err();
        assert!(matches!(err, Dok3Error::Storage(_)));

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
                insert_event_db,
            )
            .unwrap_err();
        assert!(matches!(err, Dok3Error::Storage(msg) if msg.contains("dir fsync")));

        let mut sync_calls = 0usize;
        let seq = journal
            .append_with_hooks(
                sample_event("recovered"),
                |parent| {
                    sync_calls += 1;
                    sync_dir(parent)
                },
                insert_event_db,
            )
            .unwrap();
        assert_eq!(sync_calls, 1);
        assert_eq!(seq, Sequence::new(1));

        let mut subsequent_sync_calls = 0usize;
        let seq2 = journal
            .append_with_hooks(
                sample_event("second"),
                |_| {
                    subsequent_sync_calls += 1;
                    Ok(())
                },
                insert_event_db,
            )
            .unwrap();
        assert_eq!(subsequent_sync_calls, 0);
        assert_eq!(seq2, Sequence::new(2));
    }
}
