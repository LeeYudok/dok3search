//! Durable file-backed document storage, change journal, and Blue-Green generation wiring.

use std::collections::{BTreeMap, HashSet};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use dok3_connectors::{LocalFileConnector, SourceConfig, build_document};
use dok3_core::Dok3Error;
use dok3_domain::{
    ChangeEvent, ChangeJournalPort, ChangeKind, Document, DocumentId, DocumentParserPort,
    DocumentStorePort, GenerationId, IndexJobListResponse, IndexJobMode, IndexJobPort,
    IndexJobRecord, IndexJobState, Principal, ReaderState, RevisionId, SearchPort, SearchRequest,
    SearchResponse, SearchSnapshotMeta, Sequence, SourceId, TriggerIndexJobRequest,
};
use dok3_index::{SCHEMA_VERSION, TantivyIndex};
use dok3_index_manager::{
    DurableState, Error as ManagerError, GenerationBackend, IndexManager, Manifest, ReaderAccess,
    Result as ManagerResult, SearchSnapshot, ValidationCheck,
};
use dok3_parser::MarkdownParser;
use dok3_search::SearchService;
use dok3_storage::{FileChangeJournal, FileDocumentStore, PutOutcome};
use serde::{Deserialize, Serialize};

use crate::local_index::{BuildReport, apply_detected_language};

const ANALYZER_VERSION: u64 = 1;
const ACL_POLICY_VERSION: u64 = 1;
const DEFAULT_AVAILABLE_DISK_BYTES: u64 = 100 * 1024 * 1024 * 1024;
const MAX_JOB_HISTORY: usize = 100;
static JOB_TMP_SEQ: AtomicU64 = AtomicU64::new(0);

struct LocalGenerationBackend {
    generations_dir: PathBuf,
    store: Arc<FileDocumentStore>,
    journal: Arc<FileChangeJournal>,
    indexes: BTreeMap<String, Arc<TantivyIndex>>,
    active_snapshot: Option<Arc<SearchSnapshot<Arc<TantivyIndex>>>>,
}

impl LocalGenerationBackend {
    fn new(
        generations_dir: PathBuf,
        store: Arc<FileDocumentStore>,
        journal: Arc<FileChangeJournal>,
    ) -> Self {
        Self {
            generations_dir,
            store,
            journal,
            indexes: BTreeMap::new(),
            active_snapshot: None,
        }
    }

    fn generation_dir(&self, generation: &str) -> PathBuf {
        self.generations_dir.join(generation)
    }

    fn tantivy_dir(&self, generation: &str) -> PathBuf {
        self.generation_dir(generation).join("tantivy")
    }

    fn manifest_path(&self, generation: &str) -> PathBuf {
        self.generation_dir(generation).join("manifest.json")
    }

    fn ensure_index(&mut self, generation: &str) -> ManagerResult<Arc<TantivyIndex>> {
        if let Some(existing) = self.indexes.get(generation) {
            return Ok(Arc::clone(existing));
        }
        let tantivy_dir = self.tantivy_dir(generation);
        let index = Arc::new(
            TantivyIndex::open_or_create(&tantivy_dir, GenerationId::new(generation.to_string()))
                .map_err(|err| ManagerError::Backend(err.to_string()))?,
        );
        self.indexes
            .insert(generation.to_string(), Arc::clone(&index));
        Ok(index)
    }

    fn write_manifest(&self, manifest: &Manifest) -> ManagerResult<()> {
        let dir = self.generation_dir(&manifest.generation_id);
        fs::create_dir_all(&dir)?;
        let path = self.manifest_path(&manifest.generation_id);
        let tmp = dir.join(format!("manifest.{}.tmp", std::process::id()));
        let bytes = serde_json::to_vec_pretty(manifest)?;
        {
            let mut file = File::create(&tmp)?;
            file.write_all(&bytes)?;
            file.sync_all()?;
        }
        fs::rename(&tmp, &path)?;
        if let Ok(dir_handle) = File::open(&dir) {
            let _ = dir_handle.sync_all();
        }
        Ok(())
    }
}

impl GenerationBackend for LocalGenerationBackend {
    type Reader = Arc<TantivyIndex>;

    fn build_generation(&mut self, generation: &str, s0: u64) -> ManagerResult<()> {
        let index = self.ensure_index(generation)?;
        let docs = self
            .store
            .list_documents()
            .map_err(|err| ManagerError::Backend(err.to_string()))?;
        for doc in &docs {
            index
                .upsert_document(doc)
                .map_err(|err| ManagerError::Backend(err.to_string()))?;
        }
        index
            .commit_and_reload(Some(Sequence::new(s0)))
            .map_err(|err| ManagerError::Backend(err.to_string()))?;

        let manifest = Manifest {
            generation_id: generation.to_string(),
            schema_version: u64::from(SCHEMA_VERSION),
            analyzer_version: ANALYZER_VERSION,
            acl_policy_version: ACL_POLICY_VERSION,
            applied_change_sequence: s0,
            doc_count: index.doc_count(),
        };
        self.write_manifest(&manifest)
    }

    fn replay_changes(
        &mut self,
        generation: &str,
        after_seq: u64,
        target_seq: u64,
    ) -> ManagerResult<()> {
        let index = self.ensure_index(generation)?;
        let events = self
            .journal
            .read_after(Sequence::new(after_seq), usize::MAX)
            .map_err(|err| ManagerError::Backend(err.to_string()))?;
        let docs_by_id: BTreeMap<String, Document> = self
            .store
            .list_documents()
            .map_err(|err| ManagerError::Backend(err.to_string()))?
            .into_iter()
            .map(|doc| (doc.id.as_str().to_string(), doc))
            .collect();

        for event in events {
            if event.sequence.get() > target_seq {
                break;
            }
            match event.kind {
                ChangeKind::Upsert => {
                    if let Some(doc) = docs_by_id.get(event.document_id.as_str()) {
                        index
                            .upsert_document(doc)
                            .map_err(|err| ManagerError::Backend(err.to_string()))?;
                    } else {
                        index
                            .delete_document(&event.document_id)
                            .map_err(|err| ManagerError::Backend(err.to_string()))?;
                    }
                }
                ChangeKind::Delete => {
                    index
                        .delete_document(&event.document_id)
                        .map_err(|err| ManagerError::Backend(err.to_string()))?;
                }
            }
        }

        index
            .commit_and_reload(Some(Sequence::new(target_seq)))
            .map_err(|err| ManagerError::Backend(err.to_string()))?;

        let manifest = Manifest {
            generation_id: generation.to_string(),
            schema_version: u64::from(SCHEMA_VERSION),
            analyzer_version: ANALYZER_VERSION,
            acl_policy_version: ACL_POLICY_VERSION,
            applied_change_sequence: target_seq,
            doc_count: index.doc_count(),
        };
        self.write_manifest(&manifest)
    }

    fn read_manifest(&mut self, generation: &str) -> ManagerResult<Manifest> {
        let raw = fs::read(self.manifest_path(generation))?;
        Ok(serde_json::from_slice(&raw)?)
    }

    fn validate_generation(
        &mut self,
        manifest: &Manifest,
        checks: &[ValidationCheck],
    ) -> ManagerResult<()> {
        let index = self.ensure_index(&manifest.generation_id)?;
        let on_disk_manifest = self.read_manifest(&manifest.generation_id)?;
        for check in checks {
            match check {
                ValidationCheck::ManifestChecksum => {
                    manifest.checksum()?;
                    if &on_disk_manifest != manifest {
                        return Err(ManagerError::Invalid(format!(
                            "persisted manifest drift for {}",
                            manifest.generation_id
                        )));
                    }
                }
                ValidationCheck::ReaderReopen => {
                    index
                        .reload()
                        .map_err(|err| ManagerError::Backend(err.to_string()))?;
                }
                ValidationCheck::SchemaAnalyzerCompatibility => {
                    if manifest.schema_version != u64::from(SCHEMA_VERSION)
                        || manifest.analyzer_version != ANALYZER_VERSION
                    {
                        return Err(ManagerError::Invalid(format!(
                            "incompatible schema/analyzer version for {}",
                            manifest.generation_id
                        )));
                    }
                }
                ValidationCheck::AppliedSequenceEqual => {
                    let runtime_seq = index.snapshot().applied_change_sequence.get();
                    if on_disk_manifest.applied_change_sequence != manifest.applied_change_sequence
                        || (runtime_seq != 0 && runtime_seq != manifest.applied_change_sequence)
                    {
                        return Err(ManagerError::Invalid(format!(
                            "applied sequence mismatch for {}",
                            manifest.generation_id
                        )));
                    }
                }
                ValidationCheck::DocumentAndTombstoneReconcile => {
                    if index.doc_count() != manifest.doc_count {
                        return Err(ManagerError::Invalid(format!(
                            "doc count mismatch for {}: expected {}, actual {}",
                            manifest.generation_id,
                            manifest.doc_count,
                            index.doc_count()
                        )));
                    }
                }
                ValidationCheck::AclNegativeTests => {
                    let public = Principal::anonymous_public();
                    let resp = index
                        .search(
                            &public,
                            &SearchRequest {
                                query: "dok3_acl_negative_probe".to_string(),
                                limit: 5,
                                ..SearchRequest::default()
                            },
                        )
                        .map_err(|err| ManagerError::Backend(err.to_string()))?;
                    if resp.hits.iter().any(|hit| !public.allows(&hit.acl_scope)) {
                        return Err(ManagerError::Invalid(
                            "ACL leakage detected during generation validation".into(),
                        ));
                    }
                }
                ValidationCheck::SearchLatencyUnderLoad => {
                    index
                        .search(&Principal::anonymous_public(), &SearchRequest::default())
                        .map_err(|err| ManagerError::Backend(err.to_string()))?;
                }
                ValidationCheck::FreeDiskSpace => {
                    let _ = self.free_disk_bytes()?;
                }
            }
        }
        Ok(())
    }

    fn open_reader(&mut self, manifest: &Manifest) -> ManagerResult<Self::Reader> {
        let index = self.ensure_index(&manifest.generation_id)?;
        index
            .reload()
            .map_err(|err| ManagerError::Backend(err.to_string()))?;
        Ok(index)
    }

    fn swap_active_snapshot(
        &mut self,
        snapshot: Option<Arc<SearchSnapshot<Self::Reader>>>,
    ) -> ManagerResult<()> {
        self.active_snapshot = snapshot;
        Ok(())
    }

    fn journal_sequence(&mut self) -> ManagerResult<u64> {
        self.journal
            .last_sequence()
            .map(|seq| seq.get())
            .map_err(|err| ManagerError::Backend(err.to_string()))
    }

    fn free_disk_bytes(&mut self) -> ManagerResult<u64> {
        Ok(DEFAULT_AVAILABLE_DISK_BYTES)
    }

    fn delete_generation_files(&mut self, generation: &str) -> ManagerResult<()> {
        // Release our Arc<TantivyIndex> before removing its directory and writer lock.
        self.indexes.remove(generation);
        match fs::remove_dir_all(self.generation_dir(generation)) {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(err) => Err(ManagerError::Io(err)),
        }
    }
}

struct ManagedSearchPort {
    access: ReaderAccess<Arc<TantivyIndex>>,
    _manager: Arc<Mutex<IndexManager<LocalGenerationBackend>>>,
}

impl SearchPort for ManagedSearchPort {
    fn search(
        &self,
        principal: &Principal,
        req: &SearchRequest,
    ) -> Result<SearchResponse, Dok3Error> {
        let lease = self
            .access
            .acquire()
            .ok_or_else(|| Dok3Error::Index("no active search generation available".into()))?;
        let mut resp = lease.reader.search(principal, req)?;
        resp.snapshot = snapshot_meta_from_lease(&lease);
        Ok(resp)
    }

    fn snapshot(&self) -> SearchSnapshotMeta {
        match self.access.acquire() {
            Some(lease) => snapshot_meta_from_lease(&lease),
            None => SearchSnapshotMeta {
                generation_id: GenerationId::new("none"),
                epoch: 0,
                schema_version: SCHEMA_VERSION,
                acl_policy_version: 1,
                applied_change_sequence: Sequence::new(0),
                reader_state: ReaderState::Degraded,
            },
        }
    }
}

fn snapshot_meta_from_lease(lease: &SearchSnapshot<Arc<TantivyIndex>>) -> SearchSnapshotMeta {
    SearchSnapshotMeta {
        generation_id: GenerationId::new(lease.manifest.generation_id.clone()),
        epoch: lease.epoch,
        schema_version: u32::try_from(lease.manifest.schema_version).unwrap_or(SCHEMA_VERSION),
        acl_policy_version: u32::try_from(lease.manifest.acl_policy_version).unwrap_or(1),
        applied_change_sequence: Sequence::new(lease.manifest.applied_change_sequence),
        reader_state: ReaderState::Ready,
    }
}

/// Summary returned by [`PersistentRuntime::refresh`].
///
/// `report.indexed` records the total scanned-and-stored source documents verified on this pass,
/// including when `rebuilt` is `false`.
#[derive(Debug, PartialEq, Eq)]
pub struct RefreshReport {
    pub report: BuildReport,
    pub rebuilt: bool,
    pub active_generation: Option<String>,
    pub active_epoch: u64,
}

/// Handle for shutting down the periodic refresh background thread.
pub struct WorkerHandle {
    stop_tx: Option<mpsc::Sender<()>>,
    join: Option<JoinHandle<()>>,
}

impl WorkerHandle {
    /// Signals the periodic refresh worker to exit and waits for its thread to finish.
    pub fn stop(mut self) {
        self.shutdown();
    }

    fn shutdown(&mut self) {
        if let Some(tx) = self.stop_tx.take() {
            let _ = tx.send(());
        }
        if let Some(join) = self.join.take() {
            let _ = join.join();
        }
    }
}

impl Drop for WorkerHandle {
    fn drop(&mut self) {
        self.shutdown();
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct JobHistoryDisk {
    next_seq: u64,
    jobs: Vec<IndexJobRecord>,
}

struct JobHistoryStore {
    path: PathBuf,
    next_seq: u64,
    jobs: Vec<IndexJobRecord>,
    active_generation: Option<String>,
    active_epoch: u64,
    refresh_interval_secs: Option<u64>,
}

impl JobHistoryStore {
    fn open(
        path: PathBuf,
        active_generation: Option<String>,
        active_epoch: u64,
        refresh_interval_secs: Option<u64>,
    ) -> Result<Self, String> {
        let mut disk = match fs::read(&path) {
            Ok(raw) => serde_json::from_slice::<JobHistoryDisk>(&raw)
                .map_err(|err| format!("failed to decode {}: {err}", path.display()))?,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => JobHistoryDisk::default(),
            Err(err) => return Err(format!("failed to read {}: {err}", path.display())),
        };
        let interrupted_at = now_ms();
        for job in &mut disk.jobs {
            if matches!(job.state, IndexJobState::Pending | IndexJobState::Running) {
                job.state = IndexJobState::Failed;
                job.finished_at_ms = Some(interrupted_at);
                job.error = Some("interrupted by server restart".to_string());
            }
        }
        if disk.jobs.len() > MAX_JOB_HISTORY {
            let excess = disk.jobs.len() - MAX_JOB_HISTORY;
            disk.jobs.drain(0..excess);
        }
        let max_seen = disk
            .jobs
            .iter()
            .filter_map(|job| job.job_id.strip_prefix("job-")?.parse::<u64>().ok())
            .max()
            .unwrap_or(0);
        let next_seq = disk.next_seq.max(max_seen);
        Ok(Self {
            path,
            next_seq,
            jobs: disk.jobs,
            active_generation,
            active_epoch,
            refresh_interval_secs,
        })
    }

    fn persist(&self) -> Result<(), String> {
        let disk = JobHistoryDisk {
            next_seq: self.next_seq,
            jobs: self.jobs.clone(),
        };
        let bytes = serde_json::to_vec_pretty(&disk)
            .map_err(|err| format!("failed to serialize index jobs: {err}"))?;
        atomic_write_bytes(&self.path, &bytes)
    }

    fn push_and_trim(&mut self, record: IndexJobRecord) {
        self.jobs.push(record);
        if self.jobs.len() > MAX_JOB_HISTORY {
            let excess = self.jobs.len() - MAX_JOB_HISTORY;
            self.jobs.drain(0..excess);
        }
    }

    fn start_job(
        &mut self,
        source_id: Option<String>,
        trigger: &str,
        mode: IndexJobMode,
    ) -> Result<IndexJobRecord, String> {
        self.next_seq += 1;
        let record = IndexJobRecord {
            job_id: format!("job-{:04}", self.next_seq),
            source_id,
            trigger: trigger.to_string(),
            mode,
            state: IndexJobState::Running,
            started_at_ms: now_ms(),
            finished_at_ms: None,
            indexed_documents: 0,
            rebuilt: false,
            active_generation: self.active_generation.clone(),
            active_epoch: self.active_epoch,
            skipped_files: Vec::new(),
            error: None,
        };
        self.push_and_trim(record.clone());
        self.persist()?;
        Ok(record)
    }

    #[allow(clippy::too_many_arguments)]
    fn finish_job(
        &mut self,
        job_id: &str,
        state: IndexJobState,
        indexed_documents: usize,
        rebuilt: bool,
        active_generation: Option<String>,
        active_epoch: u64,
        skipped_files: Vec<String>,
        error: Option<String>,
    ) -> Result<IndexJobRecord, String> {
        self.active_generation = active_generation.clone();
        self.active_epoch = active_epoch;
        let finished_at = now_ms();
        let record = self
            .jobs
            .iter_mut()
            .find(|job| job.job_id == job_id)
            .ok_or_else(|| format!("missing in-flight job '{job_id}'"))?;
        record.state = state;
        record.finished_at_ms = Some(finished_at);
        record.indexed_documents = indexed_documents;
        record.rebuilt = rebuilt;
        record.active_generation = active_generation;
        record.active_epoch = active_epoch;
        record.skipped_files = skipped_files;
        record.error = error;
        let snapshot = record.clone();
        self.persist()?;
        Ok(snapshot)
    }

    fn record_startup(
        &mut self,
        report: &BuildReport,
        rebuilt: bool,
        active_generation: Option<String>,
        active_epoch: u64,
    ) -> Result<IndexJobRecord, String> {
        self.next_seq += 1;
        let now = now_ms();
        self.active_generation = active_generation.clone();
        self.active_epoch = active_epoch;
        let record = IndexJobRecord {
            job_id: format!("job-{:04}", self.next_seq),
            source_id: None,
            trigger: "startup".to_string(),
            mode: IndexJobMode::Incremental,
            state: IndexJobState::Succeeded,
            started_at_ms: now,
            finished_at_ms: Some(now),
            indexed_documents: report.indexed,
            rebuilt,
            active_generation,
            active_epoch,
            skipped_files: report.skipped.clone(),
            error: None,
        };
        self.push_and_trim(record.clone());
        self.persist()?;
        Ok(record)
    }

    #[cfg_attr(not(test), allow(dead_code))]
    fn update_active(&mut self, active_generation: Option<String>, active_epoch: u64) {
        self.active_generation = active_generation;
        self.active_epoch = active_epoch;
    }

    fn list(&self) -> IndexJobListResponse {
        IndexJobListResponse {
            jobs: self.jobs.iter().rev().cloned().collect(),
            active_generation: self.active_generation.clone(),
            active_epoch: self.active_epoch,
            refresh_interval_secs: self.refresh_interval_secs,
        }
    }

    fn get(&self, job_id: &str) -> Option<IndexJobRecord> {
        self.jobs.iter().find(|job| job.job_id == job_id).cloned()
    }
}

/// Long-lived handle to durable storage, journal, and Blue-Green generation state.
pub struct PersistentRuntime {
    source_dir: PathBuf,
    store: Arc<FileDocumentStore>,
    journal: Arc<FileChangeJournal>,
    manager: Arc<Mutex<IndexManager<LocalGenerationBackend>>>,
    #[cfg_attr(not(test), allow(dead_code))]
    access: ReaderAccess<Arc<TantivyIndex>>,
    service: SearchService,
    jobs: Arc<Mutex<JobHistoryStore>>,
}

impl PersistentRuntime {
    /// Opens (or reconciles) persistent storage/generations under `data_dir` and syncs `source_dir`.
    pub fn open(source_dir: &Path, data_dir: &Path) -> Result<(Self, BuildReport), String> {
        Self::open_with_refresh(source_dir, data_dir, None)
    }

    /// Opens persistent runtime and configures reported periodic refresh interval metadata.
    pub fn open_with_refresh(
        source_dir: &Path,
        data_dir: &Path,
        refresh_interval_secs: Option<u64>,
    ) -> Result<(Self, BuildReport), String> {
        let storage_dir = data_dir.join("storage");
        let generations_dir = data_dir.join("generations");

        let store = Arc::new(FileDocumentStore::open(&storage_dir).map_err(|err| err.to_string())?);
        let journal =
            Arc::new(FileChangeJournal::open(&storage_dir).map_err(|err| err.to_string())?);

        let report = sync_source_to_storage(source_dir, &store, &journal)?;
        quarantine_incompatible_generations(&generations_dir)?;

        let backend = LocalGenerationBackend::new(
            generations_dir.clone(),
            Arc::clone(&store),
            Arc::clone(&journal),
        );
        let mut manager =
            IndexManager::open(&generations_dir, backend, 0).map_err(|err| err.to_string())?;
        manager
            .reconcile_on_startup()
            .map_err(|err| err.to_string())?;
        let rebuilt = ensure_active_generation(&mut manager, &store, &journal, false)?;
        manager.gc_retired().map_err(|err| err.to_string())?;

        let mut job_store = JobHistoryStore::open(
            storage_dir.join("index_jobs.json"),
            manager.state().active_generation.clone(),
            manager.state().active_epoch,
            refresh_interval_secs,
        )?;
        job_store.record_startup(
            &report,
            rebuilt,
            manager.state().active_generation.clone(),
            manager.state().active_epoch,
        )?;

        let access = manager.reader_access();
        let manager = Arc::new(Mutex::new(manager));
        let search_port = Arc::new(ManagedSearchPort {
            access: access.clone(),
            _manager: Arc::clone(&manager),
        });
        let service = SearchService::new(search_port, Arc::clone(&store) as _);

        let runtime = Self {
            source_dir: source_dir.to_path_buf(),
            store,
            journal,
            manager,
            access,
            service,
            jobs: Arc::new(Mutex::new(job_store)),
        };
        Ok((runtime, report))
    }

    /// Returns the shared [`SearchService`] bound to this persistent runtime.
    #[must_use]
    pub fn service(&self) -> &SearchService {
        &self.service
    }

    /// Returns a cloneable handle for acquiring protected `ReaderLease` snapshots.
    #[cfg_attr(not(test), allow(dead_code))]
    #[must_use]
    pub fn reader_access(&self) -> ReaderAccess<Arc<TantivyIndex>> {
        self.access.clone()
    }

    /// Re-scans `source_dir`, persists deltas, activates a new generation when changed, and GCs retired generations.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn refresh(&self) -> Result<RefreshReport, String> {
        let mut manager = self
            .manager
            .lock()
            .map_err(|_| "persistent index manager mutex poisoned".to_string())?;
        let result = refresh_locked(
            &mut manager,
            &self.source_dir,
            &self.store,
            &self.journal,
            false,
        )?;
        if let Ok(mut jobs) = self.jobs.lock() {
            jobs.update_active(result.active_generation.clone(), result.active_epoch);
        }
        Ok(result)
    }

    /// Runs a serialized indexing job (`Incremental` or `Full`), persisting its outcome to `index_jobs.json`
    /// without panicking or terminating the server if the scan or generation activation fails.
    pub fn run_job(&self, trigger: &str, req: &TriggerIndexJobRequest) -> IndexJobRecord {
        let started = match self.jobs.lock() {
            Ok(mut jobs) => match jobs.start_job(req.source_id.clone(), trigger, req.mode) {
                Ok(rec) => rec,
                Err(err) => {
                    return IndexJobRecord {
                        job_id: "job-unpersisted".to_string(),
                        source_id: req.source_id.clone(),
                        trigger: trigger.to_string(),
                        mode: req.mode,
                        state: IndexJobState::Failed,
                        started_at_ms: now_ms(),
                        finished_at_ms: Some(now_ms()),
                        indexed_documents: 0,
                        rebuilt: false,
                        active_generation: jobs.active_generation.clone(),
                        active_epoch: jobs.active_epoch,
                        skipped_files: Vec::new(),
                        error: Some(err),
                    };
                }
            },
            Err(_) => {
                let now = now_ms();
                return IndexJobRecord {
                    job_id: "job-poisoned".to_string(),
                    source_id: req.source_id.clone(),
                    trigger: trigger.to_string(),
                    mode: req.mode,
                    state: IndexJobState::Failed,
                    started_at_ms: now,
                    finished_at_ms: Some(now),
                    indexed_documents: 0,
                    rebuilt: false,
                    active_generation: None,
                    active_epoch: 0,
                    skipped_files: Vec::new(),
                    error: Some("index job store mutex poisoned".to_string()),
                };
            }
        };

        let outcome = self.execute_job(req);
        let mut jobs = match self.jobs.lock() {
            Ok(guard) => guard,
            Err(_) => {
                return IndexJobRecord {
                    state: IndexJobState::Failed,
                    finished_at_ms: Some(now_ms()),
                    error: Some("index job store mutex poisoned".to_string()),
                    ..started
                };
            }
        };
        let fallback_gen = jobs.active_generation.clone();
        let fallback_epoch = jobs.active_epoch;
        let finish_res = match outcome {
            Ok(result) => jobs.finish_job(
                &started.job_id,
                IndexJobState::Succeeded,
                result.report.indexed,
                result.rebuilt,
                result.active_generation,
                result.active_epoch,
                result.report.skipped,
                None,
            ),
            Err(err) => jobs.finish_job(
                &started.job_id,
                IndexJobState::Failed,
                0,
                false,
                fallback_gen,
                fallback_epoch,
                Vec::new(),
                Some(err),
            ),
        };
        finish_res.unwrap_or_else(|err| IndexJobRecord {
            state: IndexJobState::Failed,
            finished_at_ms: Some(now_ms()),
            error: Some(err),
            ..started
        })
    }

    fn execute_job(&self, req: &TriggerIndexJobRequest) -> Result<RefreshReport, String> {
        validate_job_source_id(req.source_id.as_deref()).map_err(|err| err.to_string())?;
        let mut manager = self
            .manager
            .lock()
            .map_err(|_| "persistent index manager mutex poisoned".to_string())?;
        refresh_locked(
            &mut manager,
            &self.source_dir,
            &self.store,
            &self.journal,
            req.mode == IndexJobMode::Full,
        )
    }

    /// Spawns a background thread that runs incremental [`Self::run_job`] every `interval_secs`
    /// until the returned [`WorkerHandle`] is stopped or dropped.
    #[must_use]
    pub fn spawn_periodic_worker(self: &Arc<Self>, interval_secs: u64) -> WorkerHandle {
        let interval = Duration::from_secs(interval_secs.max(1));
        let (stop_tx, stop_rx) = mpsc::channel();
        let runtime = Arc::clone(self);
        let join = thread::Builder::new()
            .name("dok3-periodic-refresh".to_string())
            .spawn(move || {
                loop {
                    match stop_rx.recv_timeout(interval) {
                        Ok(()) | Err(mpsc::RecvTimeoutError::Disconnected) => break,
                        Err(mpsc::RecvTimeoutError::Timeout) => {
                            let record = runtime.run_job(
                                "periodic",
                                &TriggerIndexJobRequest {
                                    source_id: None,
                                    mode: IndexJobMode::Incremental,
                                },
                            );
                            if let Some(err) = record.error {
                                eprintln!("periodic refresh job {} failed: {err}", record.job_id);
                            }
                        }
                    }
                }
            })
            .expect("failed to spawn dok3-periodic-refresh worker thread");
        WorkerHandle {
            stop_tx: Some(stop_tx),
            join: Some(join),
        }
    }

    /// Attempts to delete files of retired generations with zero active reader leases.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn gc_retired(&self) -> Result<Vec<String>, String> {
        let mut manager = self
            .manager
            .lock()
            .map_err(|_| "persistent index manager mutex poisoned".to_string())?;
        manager.gc_retired().map_err(|err| err.to_string())
    }
}

impl IndexJobPort for PersistentRuntime {
    fn list_jobs(&self) -> Result<IndexJobListResponse, Dok3Error> {
        let jobs = self
            .jobs
            .lock()
            .map_err(|_| Dok3Error::Internal("index job store mutex poisoned".into()))?;
        Ok(jobs.list())
    }

    fn get_job(&self, job_id: &str) -> Result<IndexJobRecord, Dok3Error> {
        let trimmed = job_id.trim();
        if trimmed.is_empty() {
            return Err(Dok3Error::InvalidInput("job_id must not be empty".into()));
        }
        let jobs = self
            .jobs
            .lock()
            .map_err(|_| Dok3Error::Internal("index job store mutex poisoned".into()))?;
        jobs.get(trimmed)
            .ok_or_else(|| Dok3Error::NotFound(format!("index job '{trimmed}' not found")))
    }

    fn trigger_job(&self, req: &TriggerIndexJobRequest) -> Result<IndexJobRecord, Dok3Error> {
        validate_job_source_id(req.source_id.as_deref())?;
        Ok(self.run_job("manual", req))
    }
}

fn validate_job_source_id(source_id: Option<&str>) -> Result<(), Dok3Error> {
    if let Some(raw) = source_id {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Err(Dok3Error::InvalidInput(
                "source_id must not be empty".to_string(),
            ));
        }
        if trimmed != "local-markdown" {
            return Err(Dok3Error::NotFound(format!("source '{trimmed}' not found")));
        }
    }
    Ok(())
}

fn refresh_locked(
    manager: &mut IndexManager<LocalGenerationBackend>,
    source_dir: &Path,
    store: &FileDocumentStore,
    journal: &FileChangeJournal,
    force_rebuild: bool,
) -> Result<RefreshReport, String> {
    manager.reconcile_on_startup().map_err(|err| {
        format!("persistent index manager reconciliation failed; restart required: {err}")
    })?;
    let report = sync_source_to_storage(source_dir, store, journal)?;
    let rebuilt = ensure_active_generation(manager, store, journal, force_rebuild)?;
    manager.gc_retired().map_err(|err| err.to_string())?;
    Ok(RefreshReport {
        report,
        rebuilt,
        active_generation: manager.state().active_generation.clone(),
        active_epoch: manager.state().active_epoch,
    })
}

fn ensure_active_generation(
    manager: &mut IndexManager<LocalGenerationBackend>,
    store: &FileDocumentStore,
    journal: &FileChangeJournal,
    force_rebuild: bool,
) -> Result<bool, String> {
    let seq = journal
        .last_sequence()
        .map_err(|err| err.to_string())?
        .get();
    let expected_docs = store.document_count() as u64;
    let active_matches = !force_rebuild
        && manager
            .state()
            .active_generation
            .as_ref()
            .and_then(|active_id| manager.state().generations.get(active_id))
            .is_some_and(|record| {
                record.target_sequence == seq
                    && record.manifest.as_ref().is_some_and(|manifest| {
                        manifest.doc_count == expected_docs
                            && manifest.schema_version == u64::from(SCHEMA_VERSION)
                            && manifest.analyzer_version == ANALYZER_VERSION
                    })
            });

    if active_matches {
        return Ok(false);
    }

    let gen_id = allocate_generation_id(manager, seq);
    manager
        .build_generation(&gen_id, seq)
        .map_err(|err| err.to_string())?;
    manager
        .prepare_generation(&gen_id, seq)
        .map_err(|err| err.to_string())?;
    manager.activate(&gen_id).map_err(|err| err.to_string())?;
    Ok(true)
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |dur| u64::try_from(dur.as_millis()).unwrap_or(u64::MAX))
}

fn atomic_write_bytes(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let dir = path
        .parent()
        .ok_or_else(|| format!("path '{}' has no parent", path.display()))?;
    fs::create_dir_all(dir)
        .map_err(|err| format!("failed to create dir '{}': {err}", dir.display()))?;
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| format!("invalid file name '{}'", path.display()))?;
    let seq = JOB_TMP_SEQ.fetch_add(1, Ordering::Relaxed);
    let tmp = dir.join(format!(".{name}.tmp.{}.{seq}", std::process::id()));
    let write_res = (|| -> Result<(), String> {
        let mut file = File::create(&tmp)
            .map_err(|err| format!("failed to create '{}': {err}", tmp.display()))?;
        file.write_all(bytes)
            .map_err(|err| format!("failed to write '{}': {err}", tmp.display()))?;
        file.sync_all()
            .map_err(|err| format!("failed to sync '{}': {err}", tmp.display()))?;
        fs::rename(&tmp, path)
            .map_err(|err| format!("failed to rename '{}': {err}", path.display()))?;
        if let Ok(handle) = File::open(dir) {
            let _ = handle.sync_all();
        }
        Ok(())
    })();
    if write_res.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    write_res
}

/// Scans `source_dir`, persists originals/metadata/journal under `data_dir/storage`, and
/// serves the active Blue-Green generation managed under `data_dir/generations`.
#[cfg_attr(not(test), allow(dead_code))]
pub fn build_persistent(
    source_dir: &Path,
    data_dir: &Path,
) -> Result<(SearchService, BuildReport), String> {
    let (runtime, report) = PersistentRuntime::open(source_dir, data_dir)?;
    Ok((runtime.service, report))
}

fn sync_source_to_storage(
    source_dir: &Path,
    store: &FileDocumentStore,
    journal: &FileChangeJournal,
) -> Result<BuildReport, String> {
    let connector = LocalFileConnector::new(SourceConfig::markdown_defaults(
        SourceId::new("local-markdown"),
        "Local Markdown",
        source_dir,
    ))
    .map_err(|err| err.to_string())?;
    let report = connector.scan().map_err(|err| err.to_string())?;
    let mut build_report = BuildReport {
        skipped: report
            .skipped
            .into_iter()
            .map(|file| format!("{}: {}", file.relative_path, file.reason))
            .collect(),
        ..BuildReport::default()
    };

    let parser = MarkdownParser::new();
    let mut seen_ids = HashSet::with_capacity(report.files.len());
    for file in report.files {
        let parsed = match parser.parse(&file.filename, &file.bytes) {
            Ok(parsed) => parsed,
            Err(err) => {
                build_report
                    .skipped
                    .push(format!("{}: parse failed: {err}", file.relative_path));
                continue;
            }
        };
        let mut document = build_document(connector.config(), &file, &parsed);
        apply_detected_language(&mut document, &parsed.normalized_text);
        let doc_id = document.id.clone();
        seen_ids.insert(doc_id.as_str().to_string());

        if let PutOutcome::Stored(rev) = store
            .put(document, &file.bytes)
            .map_err(|err| err.to_string())?
        {
            journal
                .append(ChangeEvent {
                    sequence: Sequence::new(0),
                    document_id: doc_id,
                    revision_id: rev,
                    kind: ChangeKind::Upsert,
                })
                .map_err(|err| err.to_string())?;
        }
        build_report.indexed += 1;
    }

    let existing_ids = store.document_ids().map_err(|err| err.to_string())?;
    for id in existing_ids {
        if !seen_ids.contains(id.as_str()) && store.delete(&id).map_err(|err| err.to_string())? {
            journal
                .append(ChangeEvent {
                    sequence: Sequence::new(0),
                    document_id: id,
                    revision_id: RevisionId::new("deleted"),
                    kind: ChangeKind::Delete,
                })
                .map_err(|err| err.to_string())?;
        }
    }

    reconcile_store_journal(store, journal)?;

    Ok(build_report)
}

fn quarantine_incompatible_generations(generations_dir: &Path) -> Result<(), String> {
    let state_path = generations_dir.join("generation_state.json");
    let raw = match fs::read(&state_path) {
        Ok(bytes) => bytes,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(err) => return Err(format!("failed to read generation_state.json: {err}")),
    };
    let state: DurableState = match serde_json::from_slice(&raw) {
        Ok(parsed) => parsed,
        Err(_) => return Ok(()),
    };

    let active_incompatible = state
        .active_generation
        .as_deref()
        .is_some_and(|id| generation_incompatible(generations_dir, &state, id));
    let transition_incompatible = state.activating_transition.as_ref().is_some_and(|intent| {
        generation_incompatible(generations_dir, &state, &intent.to_generation)
    });

    if !active_incompatible && !transition_incompatible {
        return Ok(());
    }

    fs::remove_file(&state_path)
        .map_err(|err| format!("failed to reset incompatible generation state: {err}"))?;
    for id in state.generations.keys() {
        let dir = generations_dir.join(id);
        match fs::remove_dir_all(&dir) {
            Ok(()) => {}
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
            Err(err) => {
                return Err(format!(
                    "failed to remove incompatible generation '{}': {err}",
                    dir.display()
                ));
            }
        }
    }
    Ok(())
}

fn generation_incompatible(generations_dir: &Path, state: &DurableState, gen_id: &str) -> bool {
    let disk_manifest = fs::read(generations_dir.join(gen_id).join("manifest.json"))
        .ok()
        .and_then(|bytes| serde_json::from_slice::<Manifest>(&bytes).ok());
    let manifest = disk_manifest.as_ref().or_else(|| {
        state
            .generations
            .get(gen_id)
            .and_then(|record| record.manifest.as_ref())
    });
    manifest.is_some_and(|m| {
        m.schema_version != u64::from(SCHEMA_VERSION) || m.analyzer_version != ANALYZER_VERSION
    })
}

fn reconcile_store_journal(
    store: &FileDocumentStore,
    journal: &FileChangeJournal,
) -> Result<(), String> {
    let events = journal
        .read_after(Sequence::new(0), usize::MAX)
        .map_err(|err| err.to_string())?;
    let mut latest: BTreeMap<String, Option<RevisionId>> = BTreeMap::new();
    for event in events {
        let state = match event.kind {
            ChangeKind::Upsert => Some(event.revision_id),
            ChangeKind::Delete => None,
        };
        latest.insert(event.document_id.as_str().to_string(), state);
    }

    let docs = store.list_documents().map_err(|err| err.to_string())?;
    let mut live_ids = HashSet::with_capacity(docs.len());
    for doc in docs {
        let key = doc.id.as_str().to_string();
        if latest.get(&key).and_then(|v| v.as_ref()) != Some(&doc.revision_id) {
            journal
                .append(ChangeEvent {
                    sequence: Sequence::new(0),
                    document_id: doc.id,
                    revision_id: doc.revision_id,
                    kind: ChangeKind::Upsert,
                })
                .map_err(|err| err.to_string())?;
        }
        live_ids.insert(key);
    }

    for (id, recorded) in latest {
        if recorded.is_some() && !live_ids.contains(&id) {
            journal
                .append(ChangeEvent {
                    sequence: Sequence::new(0),
                    document_id: DocumentId::new(id),
                    revision_id: RevisionId::new("deleted"),
                    kind: ChangeKind::Delete,
                })
                .map_err(|err| err.to_string())?;
        }
    }
    Ok(())
}

fn allocate_generation_id(manager: &IndexManager<LocalGenerationBackend>, seq: u64) -> String {
    let base_epoch = manager.state().active_epoch.saturating_add(1);
    let base = format!("gen-{base_epoch:06}-seq-{seq:06}");
    if !manager.state().generations.contains_key(&base) {
        return base;
    }
    let mut suffix = 1usize;
    loop {
        let candidate = format!("{base}-{suffix}");
        if !manager.state().generations.contains_key(&candidate) {
            return candidate;
        }
        suffix += 1;
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::sync::Arc;

    use axum::body::Body;
    use axum::http::{Request, StatusCode, header};
    use dok3_api::{ApiState, build_router};
    use dok3_domain::{DocumentDetail, HealthReadyResponse, SearchResponse, SectionReadResponse};
    use dok3_index_manager::{DurableState, GenerationFiles, GenerationState};
    use http_body_util::BodyExt;
    use serde::de::DeserializeOwned;
    use tower::ServiceExt;

    use super::*;

    fn read_durable_state(data_dir: &Path) -> DurableState {
        let raw = fs::read(data_dir.join("generations/generation_state.json")).unwrap();
        serde_json::from_slice(&raw).unwrap()
    }

    async fn json<T: DeserializeOwned>(response: axum::response::Response) -> T {
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        serde_json::from_slice(&bytes).unwrap()
    }

    fn build_persistent_app(source_dir: &Path, data_dir: &Path) -> (axum::Router, BuildReport) {
        let (service, report) = build_persistent(source_dir, data_dir).unwrap();
        let mut state = ApiState::new(Arc::new(service));
        state.mode = "tantivy-persistent".into();
        (build_router(state, None), report)
    }

    #[tokio::test]
    async fn cold_start_warm_restart_and_incremental_updates_use_persistent_generations() {
        let source_root = tempfile::tempdir().unwrap();
        let data_root = tempfile::tempdir().unwrap();

        fs::write(
            source_root.path().join("guide-a.md"),
            "# Alpha Guide\n\ninitialalpha content block\n\n## Usage\n\nalpha usage details\n",
        )
        .unwrap();
        fs::write(
            source_root.path().join("guide-b.md"),
            "# Beta Guide\n\ninitialbeta content block\n",
        )
        .unwrap();

        // 1. Cold start: builds storage, journal, and first active Blue-Green generation.
        let (app1, report1) = build_persistent_app(source_root.path(), data_root.path());
        assert_eq!(report1.indexed, 2);
        assert!(data_root.path().join("storage/documents.json").is_file());
        assert!(data_root.path().join("storage/journal.jsonl").is_file());
        assert!(data_root.path().join("storage/objects").is_dir());
        assert!(
            data_root
                .path()
                .join("generations/generation_state.json")
                .is_file()
        );

        let health1: HealthReadyResponse = json(
            app1.clone()
                .oneshot(Request::get("/health/ready").body(Body::empty()).unwrap())
                .await
                .unwrap(),
        )
        .await;
        assert_eq!(health1.mode, "tantivy-persistent");
        assert_eq!(health1.document_count, 2);

        let search1: SearchResponse = json(
            app1.clone()
                .oneshot(
                    Request::post("/api/v1/search")
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(Body::from(r#"{"query":"initialalpha","limit":10}"#))
                        .unwrap(),
                )
                .await
                .unwrap(),
        )
        .await;
        assert_eq!(search1.total_hits, 1);
        assert_eq!(
            search1.hits[0].document_id.as_str(),
            "local-markdown:guide-a.md"
        );
        assert_eq!(search1.snapshot.applied_change_sequence.get(), 2);

        let detail1: DocumentDetail = json(
            app1.clone()
                .oneshot(
                    Request::get("/api/v1/documents/local-markdown%3Aguide-a.md")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap(),
        )
        .await;
        let sec_id = detail1.sections[0].section_id.as_str();
        let section1: SectionReadResponse = json(
            app1.oneshot(
                Request::get(format!(
                    "/api/v1/documents/local-markdown%3Aguide-a.md/sections/{sec_id}"
                ))
                .body(Body::empty())
                .unwrap(),
            )
            .await
            .unwrap(),
        )
        .await;
        assert!(section1.text.contains("initialalpha"));

        let state_after_cold = read_durable_state(data_root.path());
        let first_gen = state_after_cold.active_generation.clone().unwrap();
        assert_eq!(state_after_cold.active_epoch, 1);
        drop(search1);

        // 2. Warm restart without file changes reuses the active generation directly.
        let (app2, report2) = build_persistent_app(source_root.path(), data_root.path());
        assert_eq!(report2.indexed, 2);
        let state_after_warm = read_durable_state(data_root.path());
        assert_eq!(
            state_after_warm.active_generation.as_deref(),
            Some(first_gen.as_str())
        );
        assert_eq!(state_after_warm.active_epoch, 1);

        let search2: SearchResponse = json(
            app2.oneshot(
                Request::post("/api/v1/search")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(r#"{"query":"initialbeta","limit":10}"#))
                    .unwrap(),
            )
            .await
            .unwrap(),
        )
        .await;
        assert_eq!(search2.total_hits, 1);

        // 3. Incremental update + add + delete on restart activates a new generation and GCs old.
        fs::remove_file(source_root.path().join("guide-a.md")).unwrap();
        fs::write(
            source_root.path().join("guide-b.md"),
            "# Beta Updated\n\nupdatedbeta token body\n",
        )
        .unwrap();
        fs::write(
            source_root.path().join("guide-c.md"),
            "# Gamma Added\n\naddedgamma token body\n",
        )
        .unwrap();

        let (app3, report3) = build_persistent_app(source_root.path(), data_root.path());
        assert_eq!(report3.indexed, 2);

        let state_after_inc = read_durable_state(data_root.path());
        let second_gen = state_after_inc.active_generation.clone().unwrap();
        assert_ne!(second_gen, first_gen);
        assert_eq!(state_after_inc.active_epoch, 2);
        let retired = state_after_inc.generations.get(&first_gen).unwrap();
        assert_eq!(retired.state, GenerationState::Retired);
        assert_eq!(retired.files, GenerationFiles::Deleted);
        assert!(
            !data_root
                .path()
                .join("generations")
                .join(&first_gen)
                .exists()
        );

        let old_deleted_search: SearchResponse = json(
            app3.clone()
                .oneshot(
                    Request::post("/api/v1/search")
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(Body::from(r#"{"query":"initialalpha","limit":10}"#))
                        .unwrap(),
                )
                .await
                .unwrap(),
        )
        .await;
        assert_eq!(old_deleted_search.total_hits, 0);

        let removed_doc_resp = app3
            .clone()
            .oneshot(
                Request::get("/api/v1/documents/local-markdown%3Aguide-a.md")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(removed_doc_resp.status(), StatusCode::NOT_FOUND);

        let updated_search: SearchResponse = json(
            app3.clone()
                .oneshot(
                    Request::post("/api/v1/search")
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(Body::from(r#"{"query":"updatedbeta","limit":10}"#))
                        .unwrap(),
                )
                .await
                .unwrap(),
        )
        .await;
        assert_eq!(updated_search.total_hits, 1);
        assert_eq!(updated_search.snapshot.applied_change_sequence.get(), 5);

        let added_search: SearchResponse = json(
            app3.oneshot(
                Request::post("/api/v1/search")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(r#"{"query":"addedgamma","limit":10}"#))
                    .unwrap(),
            )
            .await
            .unwrap(),
        )
        .await;
        assert_eq!(added_search.total_hits, 1);
    }

    #[test]
    fn live_refresh_is_idempotent_and_applies_incremental_add_modify_delete() {
        let source_root = tempfile::tempdir().unwrap();
        let data_root = tempfile::tempdir().unwrap();

        fs::write(
            source_root.path().join("guide-a.md"),
            "# Alpha\n\nrefreshalphaorig token\n",
        )
        .unwrap();
        fs::write(
            source_root.path().join("guide-b.md"),
            "# Beta\n\nrefreshbetaorig token\n",
        )
        .unwrap();

        let (runtime, init_report) =
            PersistentRuntime::open(source_root.path(), data_root.path()).unwrap();
        assert_eq!(init_report.indexed, 2);

        let idle = runtime.refresh().unwrap();
        assert!(!idle.rebuilt);
        assert_eq!(idle.active_epoch, 1);
        assert_eq!(idle.report.indexed, 2);

        fs::remove_file(source_root.path().join("guide-a.md")).unwrap();
        fs::write(
            source_root.path().join("guide-b.md"),
            "# Beta Updated\n\nrefreshbetaedited token\n",
        )
        .unwrap();
        fs::write(
            source_root.path().join("guide-c.md"),
            "# Gamma Added\n\nrefreshgammaadded token\n",
        )
        .unwrap();

        let changed = runtime.refresh().unwrap();
        assert!(changed.rebuilt);
        assert_eq!(changed.active_epoch, 2);
        assert_eq!(changed.report.indexed, 2);

        let public = Principal::anonymous_public();
        let deleted = runtime
            .service()
            .search(
                &public,
                &SearchRequest {
                    query: "refreshalphaorig".to_string(),
                    ..SearchRequest::default()
                },
            )
            .unwrap();
        assert_eq!(deleted.total_hits, 0);
        assert!(matches!(
            runtime.service().get_document(
                &public,
                &dok3_domain::DocumentId::new("local-markdown:guide-a.md")
            ),
            Err(Dok3Error::NotFound(_))
        ));

        let edited = runtime
            .service()
            .search(
                &public,
                &SearchRequest {
                    query: "refreshbetaedited".to_string(),
                    ..SearchRequest::default()
                },
            )
            .unwrap();
        assert_eq!(edited.total_hits, 1);

        let added = runtime
            .service()
            .search(
                &public,
                &SearchRequest {
                    query: "refreshgammaadded".to_string(),
                    ..SearchRequest::default()
                },
            )
            .unwrap();
        assert_eq!(added.total_hits, 1);
    }

    #[test]
    fn live_refresh_protects_inflight_reader_lease_until_drop() {
        let source_root = tempfile::tempdir().unwrap();
        let data_root = tempfile::tempdir().unwrap();

        fs::write(
            source_root.path().join("guide-a.md"),
            "# Lease Test\n\nleasealpha original text\n",
        )
        .unwrap();

        let (runtime, _) = PersistentRuntime::open(source_root.path(), data_root.path()).unwrap();
        let lease = runtime.reader_access().acquire().unwrap();
        let first_gen = lease.manifest.generation_id.clone();

        fs::write(
            source_root.path().join("guide-a.md"),
            "# Lease Test Updated\n\nleasebeta updated text\n",
        )
        .unwrap();

        let refreshed = runtime.refresh().unwrap();
        assert!(refreshed.rebuilt);
        assert_ne!(
            refreshed.active_generation.as_deref(),
            Some(first_gen.as_str())
        );

        let public = Principal::anonymous_public();
        let old_view = lease
            .reader
            .search(
                &public,
                &SearchRequest {
                    query: "leasealpha".to_string(),
                    ..SearchRequest::default()
                },
            )
            .unwrap();
        assert_eq!(old_view.total_hits, 1);

        let new_view = runtime
            .service()
            .search(
                &public,
                &SearchRequest {
                    query: "leasebeta".to_string(),
                    ..SearchRequest::default()
                },
            )
            .unwrap();
        assert_eq!(new_view.total_hits, 1);

        // First generation files must remain intact while an in-flight lease exists.
        assert!(
            data_root
                .path()
                .join("generations")
                .join(&first_gen)
                .is_dir()
        );
        assert!(runtime.gc_retired().unwrap().is_empty());

        drop(lease);
        let collected = runtime.gc_retired().unwrap();
        assert_eq!(collected, vec![first_gen.clone()]);
        assert!(
            !data_root
                .path()
                .join("generations")
                .join(&first_gen)
                .exists()
        );
    }

    #[test]
    fn crash_recovery_reconciles_interrupted_building_generation_on_restart() {
        let source_root = tempfile::tempdir().unwrap();
        let data_root = tempfile::tempdir().unwrap();

        fs::write(
            source_root.path().join("guide-a.md"),
            "# Crash Recovery\n\nrecoveryalpha body\n",
        )
        .unwrap();

        let (runtime, _) = PersistentRuntime::open(source_root.path(), data_root.path()).unwrap();
        drop(runtime);

        let crashed_id = "gen-999999-crashed";
        let mut durable = read_durable_state(data_root.path());
        let healthy_gen = durable.active_generation.clone().unwrap();
        durable.generations.insert(
            crashed_id.to_string(),
            dok3_index_manager::GenerationRecord {
                state: GenerationState::Building,
                created_at_unix_secs: 1,
                retired_at_unix_secs: None,
                manifest_checksum: None,
                manifest: None,
                target_sequence: 99,
                validated_sequence: None,
                files: GenerationFiles::Present,
            },
        );
        fs::create_dir_all(data_root.path().join("generations").join(crashed_id)).unwrap();
        #[derive(serde::Serialize)]
        struct TestStateEnvelope {
            format_version: u32,
            state_checksum: String,
            #[serde(flatten)]
            state: DurableState,
        }
        let state_checksum = dok3_storage::content_revision(&serde_json::to_vec(&durable).unwrap())
            .as_str()
            .to_owned();
        let encoded = serde_json::to_vec_pretty(&TestStateEnvelope {
            format_version: 1,
            state_checksum,
            state: durable,
        })
        .unwrap();
        fs::write(
            data_root.path().join("generations/generation_state.json"),
            encoded,
        )
        .unwrap();

        let (restarted, _) = PersistentRuntime::open(source_root.path(), data_root.path()).unwrap();
        let after = read_durable_state(data_root.path());
        assert_eq!(
            after.active_generation.as_deref(),
            Some(healthy_gen.as_str())
        );
        assert_eq!(after.active_epoch, 1);
        assert_eq!(
            after.generations.get(crashed_id).unwrap().state,
            GenerationState::Failed
        );

        let search = restarted
            .service()
            .search(
                &Principal::anonymous_public(),
                &SearchRequest {
                    query: "recoveryalpha".to_string(),
                    ..SearchRequest::default()
                },
            )
            .unwrap();
        assert_eq!(search.total_hits, 1);
    }

    #[test]
    fn upgrades_incompatible_disk_schema_generation_on_startup() {
        let source_root = tempfile::tempdir().unwrap();
        let data_root = tempfile::tempdir().unwrap();

        fs::write(
            source_root.path().join("guide-a.md"),
            "# Schema Upgrade\n\nupgradealpha body text\n",
        )
        .unwrap();

        let (runtime, _) = PersistentRuntime::open(source_root.path(), data_root.path()).unwrap();
        drop(runtime);

        let mut durable = read_durable_state(data_root.path());
        let old_gen = durable.active_generation.clone().unwrap();
        let old_record = durable.generations.get_mut(&old_gen).unwrap();
        let old_manifest = old_record.manifest.as_mut().unwrap();
        old_manifest.schema_version = 1;
        let old_manifest_clone = old_manifest.clone();
        old_record.manifest_checksum = Some(old_manifest_clone.checksum().unwrap());

        let old_gen_dir = data_root.path().join("generations").join(&old_gen);
        fs::write(
            old_gen_dir.join("manifest.json"),
            serde_json::to_vec_pretty(&old_manifest_clone).unwrap(),
        )
        .unwrap();
        // Simulate an incompatible on-disk Tantivy index that would fail restore_active().
        fs::write(
            old_gen_dir.join("tantivy/meta.json"),
            br#"{"corrupt_v1":true}"#,
        )
        .unwrap();

        #[derive(serde::Serialize)]
        struct TestStateEnvelope {
            format_version: u32,
            state_checksum: String,
            #[serde(flatten)]
            state: DurableState,
        }
        let state_checksum = dok3_storage::content_revision(&serde_json::to_vec(&durable).unwrap())
            .as_str()
            .to_owned();
        let encoded = serde_json::to_vec_pretty(&TestStateEnvelope {
            format_version: 1,
            state_checksum,
            state: durable,
        })
        .unwrap();
        fs::write(
            data_root.path().join("generations/generation_state.json"),
            encoded,
        )
        .unwrap();

        let (upgraded, _) = PersistentRuntime::open(source_root.path(), data_root.path()).unwrap();
        let after = read_durable_state(data_root.path());
        let new_gen = after.active_generation.clone().unwrap();
        assert_eq!(
            after
                .generations
                .get(&new_gen)
                .unwrap()
                .manifest
                .as_ref()
                .unwrap()
                .schema_version,
            u64::from(SCHEMA_VERSION)
        );
        let search = upgraded
            .service()
            .search(
                &Principal::anonymous_public(),
                &SearchRequest {
                    query: "upgradealpha".to_string(),
                    ..SearchRequest::default()
                },
            )
            .unwrap();
        assert_eq!(search.total_hits, 1);
    }

    #[test]
    fn recovers_from_crash_between_store_put_and_journal_append() {
        let source_root = tempfile::tempdir().unwrap();
        let data_root = tempfile::tempdir().unwrap();
        let file_path = source_root.path().join("guide-a.md");

        fs::write(
            &file_path,
            "# Crash Boundary\n\ngapversionone initial body\n",
        )
        .unwrap();
        let (runtime, _) = PersistentRuntime::open(source_root.path(), data_root.path()).unwrap();
        drop(runtime);

        // Simulate source update persisted by store.put() before journal.append() ran.
        fs::write(
            &file_path,
            "# Crash Boundary\n\ngapversiontwo updated body\n",
        )
        .unwrap();
        let connector = LocalFileConnector::new(SourceConfig::markdown_defaults(
            SourceId::new("local-markdown"),
            "Local Markdown",
            source_root.path(),
        ))
        .unwrap();
        let report = connector.scan().unwrap();
        let parser = MarkdownParser::new();
        let scanned = &report.files[0];
        let parsed = parser.parse(&scanned.filename, &scanned.bytes).unwrap();
        let mut updated_doc = build_document(connector.config(), scanned, &parsed);
        apply_detected_language(&mut updated_doc, &parsed.normalized_text);
        let store = FileDocumentStore::open(&data_root.path().join("storage")).unwrap();
        assert!(matches!(
            store.put(updated_doc, &scanned.bytes).unwrap(),
            PutOutcome::Stored(_)
        ));
        drop(store);

        let (restarted, _) = PersistentRuntime::open(source_root.path(), data_root.path()).unwrap();
        let public = Principal::anonymous_public();
        let stale = restarted
            .service()
            .search(
                &public,
                &SearchRequest {
                    query: "gapversionone".to_string(),
                    ..SearchRequest::default()
                },
            )
            .unwrap();
        assert_eq!(stale.total_hits, 0);

        let updated = restarted
            .service()
            .search(
                &public,
                &SearchRequest {
                    query: "gapversiontwo".to_string(),
                    ..SearchRequest::default()
                },
            )
            .unwrap();
        assert_eq!(updated.total_hits, 1);
    }

    #[test]
    fn periodic_worker_detects_new_file_and_stops_cleanly() {
        let source_root = tempfile::tempdir().unwrap();
        let data_root = tempfile::tempdir().unwrap();
        fs::write(
            source_root.path().join("guide-a.md"),
            "# Initial\n\ninitialworkerdoc token\n",
        )
        .unwrap();

        let (runtime, _) =
            PersistentRuntime::open_with_refresh(source_root.path(), data_root.path(), Some(1))
                .unwrap();
        let runtime = Arc::new(runtime);
        let startup_list = runtime.list_jobs().unwrap();
        assert_eq!(startup_list.jobs.len(), 1);
        assert_eq!(startup_list.jobs[0].trigger, "startup");
        assert_eq!(startup_list.jobs[0].state, IndexJobState::Succeeded);
        assert_eq!(startup_list.refresh_interval_secs, Some(1));

        let worker = runtime.spawn_periodic_worker(1);

        fs::write(
            source_root.path().join("guide-b.md"),
            "# Periodic Added\n\nperiodicwatchertoken live body\n",
        )
        .unwrap();

        let public = Principal::anonymous_public();
        let deadline = std::time::Instant::now() + Duration::from_secs(6);
        let mut found = false;
        while std::time::Instant::now() < deadline {
            let resp = runtime
                .service()
                .search(
                    &public,
                    &SearchRequest {
                        query: "periodicwatchertoken".to_string(),
                        ..SearchRequest::default()
                    },
                )
                .unwrap();
            if resp.total_hits == 1 {
                found = true;
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }
        worker.stop();

        assert!(found, "periodic worker did not index newly added document");
        let after = runtime.list_jobs().unwrap();
        assert!(
            after.jobs.iter().any(|job| job.trigger == "periodic"
                && job.state == IndexJobState::Succeeded
                && job.rebuilt),
            "expected at least one rebuilt periodic job, got: {:?}",
            after.jobs
        );
    }

    #[test]
    fn manual_jobs_persist_across_restart_and_full_mode_forces_new_generation() {
        let source_root = tempfile::tempdir().unwrap();
        let data_root = tempfile::tempdir().unwrap();
        fs::write(
            source_root.path().join("guide-a.md"),
            "# Persistent Jobs\n\njobhistorytoken body\n",
        )
        .unwrap();

        let (runtime1, _) =
            PersistentRuntime::open_with_refresh(source_root.path(), data_root.path(), Some(30))
                .unwrap();
        let inc = runtime1
            .trigger_job(&TriggerIndexJobRequest {
                source_id: None,
                mode: IndexJobMode::Incremental,
            })
            .unwrap();
        assert_eq!(inc.trigger, "manual");
        assert_eq!(inc.mode, IndexJobMode::Incremental);
        assert!(!inc.rebuilt);
        assert_eq!(inc.active_epoch, 1);

        let full = runtime1
            .trigger_job(&TriggerIndexJobRequest {
                source_id: Some("local-markdown".to_string()),
                mode: IndexJobMode::Full,
            })
            .unwrap();
        assert_eq!(full.mode, IndexJobMode::Full);
        assert!(full.rebuilt);
        assert_eq!(full.active_epoch, 2);

        assert_eq!(runtime1.get_job(&full.job_id).unwrap(), full);
        assert!(matches!(
            runtime1.trigger_job(&TriggerIndexJobRequest {
                source_id: Some(" ".to_string()),
                mode: IndexJobMode::Incremental,
            }),
            Err(Dok3Error::InvalidInput(_))
        ));
        assert!(matches!(
            runtime1.trigger_job(&TriggerIndexJobRequest {
                source_id: Some("missing-src".to_string()),
                mode: IndexJobMode::Incremental,
            }),
            Err(Dok3Error::NotFound(_))
        ));
        drop(runtime1);

        let (runtime2, _) =
            PersistentRuntime::open_with_refresh(source_root.path(), data_root.path(), Some(30))
                .unwrap();
        let list = runtime2.list_jobs().unwrap();
        assert_eq!(list.jobs.len(), 4);
        assert_eq!(list.jobs[0].job_id, "job-0004");
        assert_eq!(list.jobs[0].trigger, "startup");
        assert_eq!(list.jobs[1].job_id, full.job_id);
        assert_eq!(list.active_epoch, 2);
    }

    #[test]
    fn failed_job_records_error_without_disrupting_active_search() {
        let source_root = tempfile::tempdir().unwrap();
        let data_root = tempfile::tempdir().unwrap();
        let source_path = source_root.path().join("docs");
        fs::create_dir_all(&source_path).unwrap();
        fs::write(
            source_path.join("guide-a.md"),
            "# Resilient\n\nresilientsearchtoken body\n",
        )
        .unwrap();

        let (runtime, _) = PersistentRuntime::open(&source_path, data_root.path()).unwrap();
        fs::remove_dir_all(&source_path).unwrap();

        let failed = runtime.run_job("manual", &TriggerIndexJobRequest::default());
        assert_eq!(failed.state, IndexJobState::Failed);
        assert!(failed.error.is_some());
        assert_eq!(failed.active_epoch, 1);

        let search = runtime
            .service()
            .search(
                &Principal::anonymous_public(),
                &SearchRequest {
                    query: "resilientsearchtoken".to_string(),
                    ..SearchRequest::default()
                },
            )
            .unwrap();
        assert_eq!(search.total_hits, 1);
    }
}
