//! Blue-green index generation control. P1 persists the pointer and manifests
//! in a single file transaction. Backends supply the journal, index and reader.

use std::collections::BTreeMap;
use std::fmt;
use std::ops::Deref;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, Weak};

mod persistence;
mod state;

use persistence::FileStateStore;
pub use state::{
    ActivationIntent, DurableState, GenerationFiles, GenerationRecord, GenerationState,
    IntentStage, Manifest,
};
use state::{now, valid_id};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Json(serde_json::Error),
    WriterLocked,
    Invalid(String),
    Backend(String),
    InsufficientDisk { available: u64, required: u64 },
    JournalCutChanged { expected: u64, actual: u64 },
    ReconcileRequired,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => error.fmt(f),
            Self::Json(error) => error.fmt(f),
            Self::WriterLocked => f.write_str("another index manager holds the writer lock"),
            Self::Invalid(message) | Self::Backend(message) => f.write_str(message),
            Self::InsufficientDisk {
                available,
                required,
            } => {
                write!(
                    f,
                    "available disk {available} bytes, required {required} bytes"
                )
            }
            Self::JournalCutChanged { expected, actual } => {
                write!(f, "journal cut changed from {expected} to {actual}")
            }
            Self::ReconcileRequired => f.write_str("index manager requires startup reconciliation"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Json(error) => Some(error),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationCheck {
    ManifestChecksum,
    ReaderReopen,
    SchemaAnalyzerCompatibility,
    AppliedSequenceEqual,
    DocumentAndTombstoneReconcile,
    AclNegativeTests,
    SearchLatencyUnderLoad,
    FreeDiskSpace,
}

pub const REQUIRED_CHECKS: [ValidationCheck; 8] = [
    ValidationCheck::ManifestChecksum,
    ValidationCheck::ReaderReopen,
    ValidationCheck::SchemaAnalyzerCompatibility,
    ValidationCheck::AppliedSequenceEqual,
    ValidationCheck::DocumentAndTombstoneReconcile,
    ValidationCheck::AclNegativeTests,
    ValidationCheck::SearchLatencyUnderLoad,
    ValidationCheck::FreeDiskSpace,
];

/// Index and journal adapter. All snapshot consumers must hold a ReaderLease
/// for the duration of their request to participate in retired-file protection.
pub trait GenerationBackend {
    type Reader: Send + Sync + 'static;

    /// Build from an original snapshot consistent with s0, or reconcile
    /// the source against that cut before publishing the manifest.
    fn build_generation(&mut self, generation: &str, s0: u64) -> Result<()>;

    /// Replay (after_seq, target_seq] in order. Replaying a committed range
    /// again must be idempotent, including versioned upserts and tombstones.
    fn replay_changes(&mut self, generation: &str, after_seq: u64, target_seq: u64) -> Result<()>;

    /// Read the independently persisted manifest, including any backend
    /// integrity checks. Commits publish the exact requested cut.
    fn read_manifest(&mut self, generation: &str) -> Result<Manifest>;

    /// Return success only if every supplied check passes for this manifest.
    fn validate_generation(
        &mut self,
        manifest: &Manifest,
        checks: &[ValidationCheck],
    ) -> Result<()>;

    /// Open a reader pinned to this commit. A reader already in use must
    /// remain valid if its retired generation is replayed for rollback.
    fn open_reader(&mut self, manifest: &Manifest) -> Result<Self::Reader>;

    /// Atomically publish the whole snapshot, or clear it when there is no active.
    /// A failed swap requires startup
    /// reconciliation. Re-publishing a snapshot during recovery is safe.
    fn swap_active_snapshot(
        &mut self,
        snapshot: Option<Arc<SearchSnapshot<Self::Reader>>>,
    ) -> Result<()>;

    /// The last call before activation selects the cut. Changes appended
    /// after that observation belong to post-cut processing on the new active.
    fn journal_sequence(&mut self) -> Result<u64>;

    fn free_disk_bytes(&mut self) -> Result<u64>;

    /// Retry-safe, including when the files have already been removed.
    fn delete_generation_files(&mut self, generation: &str) -> Result<()>;
}

#[derive(Debug)]
pub struct SearchSnapshot<R> {
    pub manifest: Manifest,
    pub epoch: u64,
    pub created_at_unix_secs: u64,
    pub reader: R,
}

struct Published<R> {
    snapshot: Arc<SearchSnapshot<R>>,
    leases: Arc<AtomicUsize>,
}

/// Owns the reader and protects the generation files for the request lifetime.
pub struct ReaderLease<R> {
    snapshot: Arc<SearchSnapshot<R>>,
    _guard: LeaseGuard,
    _store: Arc<FileStateStore>,
}

impl<R> Deref for ReaderLease<R> {
    type Target = SearchSnapshot<R>;

    fn deref(&self) -> &Self::Target {
        &self.snapshot
    }
}

// Field drop order releases the snapshot before this guard announces it to GC.
struct LeaseGuard(Arc<AtomicUsize>);

impl Drop for LeaseGuard {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::Release);
    }
}

type Runtime<R> = Arc<Mutex<Option<Published<R>>>>;

/// Cloneable request entry point. An outstanding lease retains the writer
/// lock even when the manager closes, so reopening cannot bypass it.
pub struct ReaderAccess<R> {
    runtime: Runtime<R>,
    store: Weak<FileStateStore>,
}

impl<R> Clone for ReaderAccess<R> {
    fn clone(&self) -> Self {
        Self {
            runtime: Arc::clone(&self.runtime),
            store: self.store.clone(),
        }
    }
}

impl<R> ReaderAccess<R> {
    pub fn acquire(&self) -> Option<ReaderLease<R>> {
        let current = self
            .runtime
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        let published = current.as_ref()?;
        let store = self.store.upgrade()?;
        published.leases.fetch_add(1, Ordering::Relaxed);
        Some(ReaderLease {
            snapshot: Arc::clone(&published.snapshot),
            _guard: LeaseGuard(Arc::clone(&published.leases)),
            _store: store,
        })
    }
}

pub struct IndexManager<B: GenerationBackend> {
    backend: B,
    store: Arc<FileStateStore>,
    state: DurableState,
    runtime: Runtime<B::Reader>,
    leases: BTreeMap<String, Arc<AtomicUsize>>,
    min_free_disk_bytes: u64,
    needs_reconcile: bool,
    state_reload_required: bool,
}

impl<B: GenerationBackend> Drop for IndexManager<B> {
    fn drop(&mut self) {
        *self
            .runtime
            .lock()
            .unwrap_or_else(|error| error.into_inner()) = None;
    }
}

impl<B: GenerationBackend> IndexManager<B> {
    /// Open durable state. Call reconcile_on_startup before issuing writes or
    /// accepting searches.
    pub fn open(root: impl AsRef<Path>, backend: B, min_free_disk_bytes: u64) -> Result<Self> {
        let (store, state) = FileStateStore::open(root.as_ref())?;
        Ok(Self {
            backend,
            store: Arc::new(store),
            state,
            runtime: Arc::default(),
            leases: BTreeMap::new(),
            min_free_disk_bytes,
            needs_reconcile: true,
            state_reload_required: false,
        })
    }

    pub fn state(&self) -> &DurableState {
        &self.state
    }

    pub fn reader_access(&self) -> ReaderAccess<B::Reader> {
        ReaderAccess {
            runtime: Arc::clone(&self.runtime),
            store: Arc::downgrade(&self.store),
        }
    }

    pub fn build_generation(&mut self, generation: &str, s0: u64) -> Result<()> {
        self.ensure_operational()?;
        if !valid_id(generation) || self.state.generations.contains_key(generation) {
            return Err(Error::Invalid(format!(
                "invalid or existing generation ID: {generation}"
            )));
        }
        self.check_disk()?;
        let mut next = self.state.clone();
        next.generations
            .insert(generation.to_owned(), GenerationRecord::new(s0)?);
        self.save(next)?;

        let result = self
            .backend
            .build_generation(generation, s0)
            .and_then(|()| self.read_cut(generation, s0));
        let manifest = self.fail_on_error(generation, result)?;
        let mut next = self.state.clone();
        let record = next.generations.get_mut(generation).unwrap();
        record.state = record.state.transition(GenerationState::CatchingUp)?;
        record.manifest_checksum = Some(manifest.checksum()?);
        record.manifest = Some(manifest);
        self.save(next)
    }

    /// Replay and validate through target_sequence. Advancing a Ready
    /// candidate first durably clears its previous validation.
    pub fn prepare_generation(&mut self, generation: &str, target_sequence: u64) -> Result<()> {
        self.ensure_operational()?;
        let record = self.record(generation)?;
        if !matches!(
            record.state,
            GenerationState::CatchingUp | GenerationState::Ready
        ) || target_sequence < record.target_sequence
        {
            return Err(Error::Invalid(
                "candidate state or target sequence is invalid".into(),
            ));
        }
        if record.verified_at(target_sequence) && record.state == GenerationState::Ready {
            return Ok(());
        }
        self.begin_replay(generation, target_sequence)?;
        self.resume_candidate(generation)
    }

    pub fn activate(&mut self, generation: &str) -> Result<()> {
        self.ensure_operational()?;
        let record = self.record(generation)?;
        let target_sequence = record.target_sequence;
        if record.state != GenerationState::Ready || !record.verified_at(target_sequence) {
            return Err(Error::Invalid(
                "activation requires a validated candidate".into(),
            ));
        }
        if let Some(active) = &self.state.active_generation
            && self.record(active)?.target_sequence > target_sequence
        {
            return Err(Error::Invalid(
                "activation would regress the active cut".into(),
            ));
        }
        self.check_disk()?;
        let epoch = self
            .state
            .active_epoch
            .checked_add(1)
            .ok_or_else(|| Error::Invalid("active epoch exhausted".into()))?;
        let snapshot = self.open_snapshot(generation, epoch)?;
        self.check_journal_cut(target_sequence)?;

        let mut next = self.state.clone();
        next.activating_transition = Some(ActivationIntent {
            from_generation: next.active_generation.clone(),
            to_generation: generation.to_owned(),
            from_epoch: next.active_epoch,
            target_epoch: epoch,
            target_sequence,
            intent_stage: IntentStage::Prepared,
        });
        let record = next.generations.get_mut(generation).unwrap();
        record.state = record.state.transition(GenerationState::Activating)?;
        self.save(next)?;
        self.needs_reconcile = true;
        self.commit_activation(snapshot)?;
        self.needs_reconcile = false;
        Ok(())
    }

    pub fn rollback(&mut self, generation: &str, current_journal_seq: u64) -> Result<()> {
        self.ensure_operational()?;
        let record = self.record(generation)?;
        if !matches!(
            record.state,
            GenerationState::Retired | GenerationState::CatchingUp | GenerationState::Ready
        ) || record.retired_at_unix_secs.is_none()
            || record.files != GenerationFiles::Present
            || current_journal_seq < record.target_sequence
        {
            return Err(Error::Invalid(
                "rollback requires a retained generation and a forward cut".into(),
            ));
        }
        let retired = record.state == GenerationState::Retired;
        self.check_journal_cut(current_journal_seq)?;
        self.check_disk()?;
        if retired {
            self.begin_replay(generation, current_journal_seq)?;
            self.resume_candidate(generation)?;
        } else {
            self.prepare_generation(generation, current_journal_seq)?;
        }
        self.activate(generation)
    }

    /// Recover using the durable intent and independently verified manifest.
    /// Either intent stage may be replayed because a process-local swap does
    /// not survive a restart. A valid intent commits its fixed journal cut.
    pub fn reconcile_on_startup(&mut self) -> Result<()> {
        if !self.needs_reconcile {
            return Ok(());
        }
        if self.state_reload_required {
            self.state = self.store.load()?;
            self.state_reload_required = false;
        }
        if let Some(intent) = self.state.activating_transition.clone() {
            let id = &intent.to_generation;
            if self.record(id)?.state == GenerationState::Activating {
                match self.recover_intent(&intent) {
                    Ok(snapshot) => self.commit_activation(snapshot)?,
                    Err(_) => {
                        self.restore_active()?;
                        self.mark(id, GenerationState::Recovering)?;
                        self.abort_intent(id)?;
                    }
                }
            } else {
                self.restore_active()?;
                self.abort_intent(id)?;
            }
        } else {
            self.restore_active()?;
        }

        for (id, record) in self.state.generations.clone() {
            match record.state {
                GenerationState::Building => {
                    self.mark(&id, GenerationState::Recovering)?;
                    self.mark(&id, GenerationState::Failed)?;
                }
                GenerationState::Recovering => self.mark(&id, GenerationState::Failed)?,
                GenerationState::CatchingUp if record.target_sequence > self.checkpoint(&id)? => {
                    self.resume_after_restart(&id)?;
                }
                GenerationState::Validating => self.resume_after_restart(&id)?,
                _ => {}
            }
        }
        self.needs_reconcile = false;
        Ok(())
    }

    pub fn gc_retired(&mut self) -> Result<Vec<String>> {
        self.ensure_operational()?;
        let mut deleted = Vec::new();
        for (id, record) in self.state.generations.clone() {
            if record.state != GenerationState::Retired
                || record.files == GenerationFiles::Deleted
                || self
                    .leases
                    .get(&id)
                    .is_some_and(|count| count.load(Ordering::Acquire) != 0)
            {
                continue;
            }
            self.set_files(&id, GenerationFiles::Deleting)?;
            self.backend.delete_generation_files(&id)?;
            self.set_files(&id, GenerationFiles::Deleted)?;
            self.leases.remove(&id);
            deleted.push(id);
        }
        Ok(deleted)
    }

    fn ensure_operational(&self) -> Result<()> {
        if self.needs_reconcile {
            return Err(Error::ReconcileRequired);
        }
        Ok(())
    }

    fn save(&mut self, next: DurableState) -> Result<()> {
        if let Err(error) = self.store.save(&next) {
            // A rename may already have happened before a directory sync failed.
            self.needs_reconcile = true;
            self.state_reload_required = true;
            return Err(error);
        }
        self.state = next;
        Ok(())
    }

    fn record(&self, generation: &str) -> Result<&GenerationRecord> {
        self.state
            .generations
            .get(generation)
            .ok_or_else(|| Error::Invalid(format!("unknown generation: {generation}")))
    }

    fn mark(&mut self, generation: &str, state: GenerationState) -> Result<()> {
        let mut next = self.state.clone();
        let record = next.generations.get_mut(generation).unwrap();
        record.state = record.state.transition(state)?;
        self.save(next)
    }

    fn set_files(&mut self, generation: &str, files: GenerationFiles) -> Result<()> {
        let mut next = self.state.clone();
        next.generations.get_mut(generation).unwrap().files = files;
        self.save(next)
    }

    fn check_disk(&mut self) -> Result<()> {
        let available = self.backend.free_disk_bytes()?;
        if available < self.min_free_disk_bytes {
            return Err(Error::InsufficientDisk {
                available,
                required: self.min_free_disk_bytes,
            });
        }
        Ok(())
    }

    fn check_journal_cut(&mut self, expected: u64) -> Result<()> {
        let actual = self.backend.journal_sequence()?;
        if actual != expected {
            return Err(Error::JournalCutChanged { expected, actual });
        }
        Ok(())
    }

    fn checkpoint(&self, generation: &str) -> Result<u64> {
        self.record(generation)?
            .manifest
            .as_ref()
            .map(|manifest| manifest.applied_change_sequence)
            .ok_or_else(|| Error::Invalid("generation has no checkpoint".into()))
    }

    fn begin_replay(&mut self, generation: &str, target_sequence: u64) -> Result<()> {
        let mut next = self.state.clone();
        let record = next.generations.get_mut(generation).unwrap();
        if record.state != GenerationState::CatchingUp {
            record.state = record.state.transition(GenerationState::CatchingUp)?;
        }
        record.validated_sequence = None;
        record.target_sequence = target_sequence;
        self.save(next)
    }

    fn resume_candidate(&mut self, generation: &str) -> Result<()> {
        let target = self.record(generation)?.target_sequence;
        if self.record(generation)?.state == GenerationState::CatchingUp {
            let after = self.checkpoint(generation)?;
            self.check_disk()?;
            if after < target {
                // Keep CatchingUp and its old checkpoint if replay fails, so a
                // backend commit without a checkpoint is retried on restart.
                self.backend.replay_changes(generation, after, target)?;
            }
            let manifest = self.read_cut(generation, target)?;
            if after == target {
                self.verify_manifest(generation, &manifest)?;
            }
            let mut next = self.state.clone();
            let record = next.generations.get_mut(generation).unwrap();
            record.state = record.state.transition(GenerationState::Validating)?;
            record.manifest_checksum = Some(manifest.checksum()?);
            record.manifest = Some(manifest);
            self.save(next)?;
        }

        let result = self.validate(generation, target);
        self.fail_on_error(generation, result)?;
        let mut next = self.state.clone();
        let record = next.generations.get_mut(generation).unwrap();
        record.state = record.state.transition(GenerationState::Ready)?;
        record.validated_sequence = Some(target);
        self.save(next)
    }

    fn resume_after_restart(&mut self, generation: &str) -> Result<()> {
        if let Err(error) = self.resume_candidate(generation) {
            if self.state_reload_required {
                return Err(error);
            }
            if self.record(generation)?.state != GenerationState::Failed {
                self.mark(generation, GenerationState::Recovering)?;
                self.mark(generation, GenerationState::Failed)?;
            }
        }
        Ok(())
    }

    fn fail_on_error<T>(&mut self, generation: &str, result: Result<T>) -> Result<T> {
        if result.is_err() {
            self.mark(generation, GenerationState::Failed)?;
        }
        result
    }

    fn read_cut(&mut self, generation: &str, sequence: u64) -> Result<Manifest> {
        let manifest = self.backend.read_manifest(generation)?;
        if manifest.generation_id != generation || manifest.applied_change_sequence != sequence {
            return Err(Error::Invalid(format!(
                "manifest generation or sequence mismatch for {generation}"
            )));
        }
        Ok(manifest)
    }

    fn verify_manifest(&self, generation: &str, manifest: &Manifest) -> Result<()> {
        if self.record(generation)?.manifest_checksum.as_ref() != Some(&manifest.checksum()?) {
            return Err(Error::Invalid(format!(
                "manifest checksum mismatch for {generation}"
            )));
        }
        Ok(())
    }

    fn validate(&mut self, generation: &str, sequence: u64) -> Result<()> {
        self.check_disk()?;
        let manifest = self.read_cut(generation, sequence)?;
        self.verify_manifest(generation, &manifest)?;
        self.backend.open_reader(&manifest)?;
        self.backend
            .validate_generation(&manifest, &REQUIRED_CHECKS)
    }

    fn open_snapshot(
        &mut self,
        generation: &str,
        epoch: u64,
    ) -> Result<Arc<SearchSnapshot<B::Reader>>> {
        let record = self.record(generation)?;
        let sequence = record.target_sequence;
        let created_at_unix_secs = record.created_at_unix_secs;
        let manifest = self.read_cut(generation, sequence)?;
        self.verify_manifest(generation, &manifest)?;
        let reader = self.backend.open_reader(&manifest)?;
        Ok(Arc::new(SearchSnapshot {
            manifest,
            epoch,
            created_at_unix_secs,
            reader,
        }))
    }

    fn publish(&mut self, snapshot: Arc<SearchSnapshot<B::Reader>>) -> Result<()> {
        let leases = self
            .leases
            .entry(snapshot.manifest.generation_id.clone())
            .or_default();
        let mut runtime = self
            .runtime
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        self.backend
            .swap_active_snapshot(Some(Arc::clone(&snapshot)))?;
        *runtime = Some(Published {
            snapshot,
            leases: Arc::clone(leases),
        });
        Ok(())
    }

    fn restore_active(&mut self) -> Result<()> {
        if let Some(active) = self.state.active_generation.clone() {
            let snapshot = self.open_snapshot(&active, self.state.active_epoch)?;
            self.publish(snapshot)?;
        } else {
            let mut runtime = self
                .runtime
                .lock()
                .unwrap_or_else(|error| error.into_inner());
            self.backend.swap_active_snapshot(None)?;
            *runtime = None;
        }
        Ok(())
    }

    fn recover_intent(
        &mut self,
        intent: &ActivationIntent,
    ) -> Result<Arc<SearchSnapshot<B::Reader>>> {
        self.validate(&intent.to_generation, intent.target_sequence)?;
        self.open_snapshot(&intent.to_generation, intent.target_epoch)
    }

    fn abort_intent(&mut self, generation: &str) -> Result<()> {
        let mut next = self.state.clone();
        let record = next.generations.get_mut(generation).unwrap();
        record.state = record.state.transition(GenerationState::Failed)?;
        next.activating_transition = None;
        self.save(next)
    }

    fn commit_activation(&mut self, snapshot: Arc<SearchSnapshot<B::Reader>>) -> Result<()> {
        self.publish(snapshot)?;
        let mut next = self.state.clone();
        let intent = next.activating_transition.as_mut().unwrap();
        intent.intent_stage = IntentStage::RuntimeSwapped;
        self.save(next)?;

        let mut next = self.state.clone();
        let intent = next.activating_transition.take().unwrap();
        if let Some(previous) = &intent.from_generation {
            let record = next.generations.get_mut(previous).unwrap();
            record.state = record.state.transition(GenerationState::Retired)?;
            record.retired_at_unix_secs = Some(now());
        }
        let record = next.generations.get_mut(&intent.to_generation).unwrap();
        record.state = record.state.transition(GenerationState::Active)?;
        record.retired_at_unix_secs = None;
        next.active_generation = Some(intent.to_generation);
        next.active_epoch = intent.target_epoch;
        self.save(next)
    }
}
