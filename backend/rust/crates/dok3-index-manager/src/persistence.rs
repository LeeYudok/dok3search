use std::fs::{self, File, OpenOptions, TryLockError};
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::state::{DurableState, checksum};
use crate::{Error, Result};

/// P1 file implementation. Each save replaces the entire control transaction.
pub(crate) struct FileStateStore {
    root: PathBuf,
    _writer_lock: File,
}

#[derive(Serialize, Deserialize)]
struct StateFile {
    format_version: u32,
    state_checksum: String,
    #[serde(flatten)]
    state: DurableState,
}

impl FileStateStore {
    pub(crate) fn open(root: &Path) -> Result<(Self, DurableState)> {
        fs::create_dir_all(root)?;
        let lock = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(root.join(".index-manager.lock"))?;
        lock.try_lock().map_err(|error| match error {
            TryLockError::WouldBlock => Error::WriterLocked,
            TryLockError::Error(error) => Error::Io(error),
        })?;

        let store = Self {
            root: root.to_owned(),
            _writer_lock: lock,
        };
        let state = store.load()?;
        Ok((store, state))
    }

    pub(crate) fn load(&self) -> Result<DurableState> {
        let state = match File::open(self.root.join("generation_state.json")) {
            Ok(file) => {
                let envelope: StateFile = serde_json::from_reader(file)?;
                if envelope.format_version != 1
                    || envelope.state_checksum != checksum(&envelope.state)?
                {
                    return Err(Error::Invalid(
                        "generation state checksum or format mismatch".into(),
                    ));
                }
                envelope.state.verify()?;
                envelope.state
            }
            Err(error) if error.kind() == ErrorKind::NotFound => DurableState::default(),
            Err(error) => return Err(error.into()),
        };
        Ok(state)
    }

    pub(crate) fn save(&self, state: &DurableState) -> Result<()> {
        state.verify()?;
        let contents = serde_json::to_vec_pretty(&StateFile {
            format_version: 1,
            state_checksum: checksum(state)?,
            state: state.clone(),
        })?;
        let temp = self.root.join("generation_state.json.tmp");
        match fs::remove_file(&temp) {
            Ok(()) => {}
            Err(error) if error.kind() == ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&temp)?;
        file.write_all(&contents)?;
        file.sync_all()?;
        drop(file);
        replace(&temp, &self.root.join("generation_state.json"))?;
        #[cfg(unix)]
        File::open(&self.root)?.sync_all()?;
        Ok(())
    }
}

fn replace(from: &Path, to: &Path) -> std::io::Result<()> {
    #[cfg(windows)]
    for delay_ms in [5, 10, 20] {
        match fs::rename(from, to) {
            Err(error) if error.kind() == ErrorKind::PermissionDenied => {
                std::thread::sleep(std::time::Duration::from_millis(delay_ms));
            }
            result => return result,
        }
    }
    fs::rename(from, to)
}
