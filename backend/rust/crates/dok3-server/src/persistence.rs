//! Durable file-backed document storage, change journal, and Blue-Green generation wiring.

use std::collections::{BTreeMap, HashSet};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use dok3_connectors::{LocalFileConnector, RelationalSourceAdapter, build_document};
use dok3_core::Dok3Error;
use dok3_domain::{
    ChangeEvent, ChangeJournalPort, ChangeKind, ClassificationPort, CreateSourceRequest,
    DeleteSourceResponse, Document, DocumentClassification, DocumentId, DocumentStorePort,
    GenerationId, IndexJobListResponse, IndexJobMode, IndexJobPort, IndexJobRecord, IndexJobState,
    Principal, ReaderState, RegisteredSource, RevisionId, SearchPort, SearchRequest,
    SearchResponse, SearchSnapshotMeta, Sequence, SourceKind, SourceListResponse,
    SourceRegistryPort, TriggerIndexJobRequest, UpdateClassificationRequest,
};
use dok3_index::{SCHEMA_VERSION, TantivyIndex};
use dok3_index_manager::{
    DurableState, Error as ManagerError, GenerationBackend, IndexManager, Manifest, ReaderAccess,
    Result as ManagerResult, SearchSnapshot, ValidationCheck,
};
use dok3_parser::ParserRegistry;
use dok3_search::SearchService;
use dok3_storage::{FileChangeJournal, FileDocumentStore, PutOutcome, content_revision};
use serde::{Deserialize, Serialize};

use crate::classification::{ClassificationStore, classification_from_doc};
use crate::corpus_meta::CorpusMetadataIndex;
use crate::local_index::{BuildReport, apply_detected_language};
use crate::sources::{
    SourceCursorStore, SourceRegistry, relational_source_config_for, source_config_for,
    sqlite_adapter_for, validate_real_directory, validate_real_sqlite_file,
};

const ACL_POLICY_VERSION: u64 = 1;

fn current_analyzer_version() -> u64 {
    dok3_analyzer::active_analyzer_version()
}
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
            analyzer_version: current_analyzer_version(),
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
            analyzer_version: current_analyzer_version(),
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
                        || manifest.analyzer_version != current_analyzer_version()
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
    dirty: bool,
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
            dirty: false,
        })
    }

    fn persist_snapshot(path: &Path, next_seq: u64, jobs: &[IndexJobRecord]) -> Result<(), String> {
        let disk = JobHistoryDisk {
            next_seq,
            jobs: jobs.to_vec(),
        };
        let bytes = serde_json::to_vec_pretty(&disk)
            .map_err(|err| format!("failed to serialize index jobs: {err}"))?;
        atomic_write_bytes(path, &bytes)
    }

    fn trim_completed_history(jobs: &mut Vec<IndexJobRecord>) {
        while jobs.len() > MAX_JOB_HISTORY {
            if let Some(idx) = jobs.iter().position(|job| {
                !matches!(job.state, IndexJobState::Pending | IndexJobState::Running)
            }) {
                jobs.remove(idx);
            } else {
                break;
            }
        }
    }

    fn start_job(
        &mut self,
        source_id: Option<String>,
        trigger: &str,
        mode: IndexJobMode,
    ) -> Result<IndexJobRecord, Dok3Error> {
        let next_seq = self.next_seq + 1;
        let record = IndexJobRecord {
            job_id: format!("job-{next_seq:04}"),
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
        let mut next_jobs = self.jobs.clone();
        next_jobs.push(record.clone());
        Self::trim_completed_history(&mut next_jobs);
        Self::persist_snapshot(&self.path, next_seq, &next_jobs).map_err(Dok3Error::Storage)?;
        self.next_seq = next_seq;
        self.jobs = next_jobs;
        self.dirty = false;
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
    ) -> Result<IndexJobRecord, Dok3Error> {
        let finished_at = now_ms();
        let snapshot = {
            let record = self
                .jobs
                .iter_mut()
                .find(|job| job.job_id == job_id)
                .ok_or_else(|| Dok3Error::Internal(format!("missing in-flight job '{job_id}'")))?;
            record.state = state;
            record.finished_at_ms = Some(finished_at);
            record.indexed_documents = indexed_documents;
            record.rebuilt = rebuilt;
            record.active_generation = active_generation.clone();
            record.active_epoch = active_epoch;
            record.skipped_files = skipped_files;
            record.error = error;
            record.clone()
        };
        Self::trim_completed_history(&mut self.jobs);
        self.active_generation = active_generation;
        self.active_epoch = active_epoch;
        self.dirty = true;
        Self::persist_snapshot(&self.path, self.next_seq, &self.jobs)
            .map_err(Dok3Error::Storage)?;
        self.dirty = false;
        Ok(snapshot)
    }

    fn flush_if_dirty(&mut self) {
        if self.dirty && Self::persist_snapshot(&self.path, self.next_seq, &self.jobs).is_ok() {
            self.dirty = false;
        }
    }

    fn record_startup(
        &mut self,
        report: &BuildReport,
        rebuilt: bool,
        active_generation: Option<String>,
        active_epoch: u64,
    ) -> Result<IndexJobRecord, String> {
        let next_seq = self.next_seq + 1;
        let now = now_ms();
        let record = IndexJobRecord {
            job_id: format!("job-{next_seq:04}"),
            source_id: None,
            trigger: "startup".to_string(),
            mode: IndexJobMode::Incremental,
            state: IndexJobState::Succeeded,
            started_at_ms: now,
            finished_at_ms: Some(now),
            indexed_documents: report.indexed,
            rebuilt,
            active_generation: active_generation.clone(),
            active_epoch,
            skipped_files: report.skipped.clone(),
            error: None,
        };
        let mut next_jobs = self.jobs.clone();
        next_jobs.push(record.clone());
        Self::trim_completed_history(&mut next_jobs);
        Self::persist_snapshot(&self.path, next_seq, &next_jobs)?;
        self.next_seq = next_seq;
        self.jobs = next_jobs;
        self.active_generation = active_generation;
        self.active_epoch = active_epoch;
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
    sources: Arc<Mutex<SourceRegistry>>,
    store: Arc<FileDocumentStore>,
    journal: Arc<FileChangeJournal>,
    manager: Arc<Mutex<IndexManager<LocalGenerationBackend>>>,
    access: ReaderAccess<Arc<TantivyIndex>>,
    service: SearchService,
    job_exec_lock: Mutex<()>,
    jobs: Arc<Mutex<JobHistoryStore>>,
    classifications: Arc<Mutex<ClassificationStore>>,
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
        let mut source_registry = SourceRegistry::open(&storage_dir, source_dir)?;
        let mut classification_store = ClassificationStore::open(&storage_dir)?;

        let report = sync_sources_to_storage(
            &mut source_registry,
            None,
            &store,
            &journal,
            &mut classification_store,
            false,
        )?;
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
        classification_store.clear_all_pending_index_sync()?;
        source_registry.finalize_all_pending_deletes()?;
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
            sources: Arc::new(Mutex::new(source_registry)),
            store,
            journal,
            manager,
            access,
            service,
            job_exec_lock: Mutex::new(()),
            jobs: Arc::new(Mutex::new(job_store)),
            classifications: Arc::new(Mutex::new(classification_store)),
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

    /// Re-scans all registered sources, persists deltas, activates a new generation when changed, and GCs retired generations.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn refresh(&self) -> Result<RefreshReport, String> {
        let _exec_guard = self
            .job_exec_lock
            .lock()
            .map_err(|_| "index job execution lock poisoned".to_string())?;
        let mut sources = self
            .sources
            .lock()
            .map_err(|_| "source registry mutex poisoned".to_string())?;
        let mut manager = self
            .manager
            .lock()
            .map_err(|_| "persistent index manager mutex poisoned".to_string())?;
        let mut classifications = self
            .classifications
            .lock()
            .map_err(|_| "classification store mutex poisoned".to_string())?;
        let result = refresh_locked(
            &mut manager,
            &mut sources,
            None,
            &self.store,
            &self.journal,
            &mut classifications,
            false,
        )?;
        drop(classifications);
        drop(manager);
        drop(sources);
        if let Ok(mut jobs) = self.jobs.lock() {
            jobs.update_active(result.active_generation.clone(), result.active_epoch);
        }
        Ok(result)
    }

    /// Runs a serialized indexing job (`Incremental` or `Full`), persisting its outcome to `index_jobs.json`.
    /// Scan/indexing errors are recorded as `IndexJobState::Failed`; persistence failures return `Dok3Error::Storage`.
    pub fn run_job(
        &self,
        trigger: &str,
        req: &TriggerIndexJobRequest,
    ) -> Result<IndexJobRecord, Dok3Error> {
        self.run_job_with_hook(trigger, req, || {})
    }

    fn run_job_with_hook(
        &self,
        trigger: &str,
        req: &TriggerIndexJobRequest,
        after_execute: impl FnOnce(),
    ) -> Result<IndexJobRecord, Dok3Error> {
        let _exec_guard = self
            .job_exec_lock
            .lock()
            .map_err(|_| Dok3Error::Internal("index job execution lock poisoned".to_string()))?;

        let started = {
            let mut jobs = self
                .jobs
                .lock()
                .map_err(|_| Dok3Error::Internal("index job store mutex poisoned".to_string()))?;
            jobs.start_job(req.source_id.clone(), trigger, req.mode)?
        };

        let outcome = self.execute_job(req);
        after_execute();

        let (live_gen, live_epoch) = self.live_active_generation();
        let mut jobs = self
            .jobs
            .lock()
            .map_err(|_| Dok3Error::Internal("index job store mutex poisoned".to_string()))?;
        let fallback_gen = live_gen.or_else(|| jobs.active_generation.clone());
        let fallback_epoch = live_epoch.unwrap_or(jobs.active_epoch);
        match outcome {
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
        }
    }

    fn live_active_generation(&self) -> (Option<String>, Option<u64>) {
        match self.access.acquire() {
            Some(lease) => (
                Some(lease.manifest.generation_id.clone()),
                Some(lease.epoch),
            ),
            None => (None, None),
        }
    }

    fn execute_job(&self, req: &TriggerIndexJobRequest) -> Result<RefreshReport, String> {
        let mut sources = self
            .sources
            .lock()
            .map_err(|_| "source registry mutex poisoned".to_string())?;
        let target = validate_job_source_id(&sources, req.source_id.as_deref())
            .map_err(|err| err.to_string())?;
        let mut manager = self
            .manager
            .lock()
            .map_err(|_| "persistent index manager mutex poisoned".to_string())?;
        let mut classifications = self
            .classifications
            .lock()
            .map_err(|_| "classification store mutex poisoned".to_string())?;
        refresh_locked(
            &mut manager,
            &mut sources,
            target.as_deref(),
            &self.store,
            &self.journal,
            &mut classifications,
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
                            match runtime.run_job(
                                "periodic",
                                &TriggerIndexJobRequest {
                                    source_id: None,
                                    mode: IndexJobMode::Incremental,
                                },
                            ) {
                                Ok(record) => {
                                    if let Some(err) = record.error {
                                        eprintln!(
                                            "periodic refresh job {} failed: {err}",
                                            record.job_id
                                        );
                                    }
                                }
                                Err(err) => {
                                    eprintln!("periodic refresh job persistence failed: {err}");
                                }
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
        let mut resp = {
            let mut jobs = self
                .jobs
                .lock()
                .map_err(|_| Dok3Error::Internal("index job store mutex poisoned".into()))?;
            jobs.flush_if_dirty();
            jobs.list()
        };
        if let Some(lease) = self.access.acquire() {
            resp.active_generation = Some(lease.manifest.generation_id.clone());
            resp.active_epoch = lease.epoch;
        }
        Ok(resp)
    }

    fn get_job(&self, job_id: &str) -> Result<IndexJobRecord, Dok3Error> {
        let trimmed = job_id.trim();
        if trimmed.is_empty() {
            return Err(Dok3Error::InvalidInput("job_id must not be empty".into()));
        }
        let mut jobs = self
            .jobs
            .lock()
            .map_err(|_| Dok3Error::Internal("index job store mutex poisoned".into()))?;
        jobs.flush_if_dirty();
        jobs.get(trimmed)
            .ok_or_else(|| Dok3Error::NotFound(format!("index job '{trimmed}' not found")))
    }

    fn trigger_job(&self, req: &TriggerIndexJobRequest) -> Result<IndexJobRecord, Dok3Error> {
        {
            let sources = self
                .sources
                .lock()
                .map_err(|_| Dok3Error::Internal("source registry mutex poisoned".into()))?;
            let _ = validate_job_source_id(&sources, req.source_id.as_deref())?;
        }
        self.run_job("manual", req)
    }
}

impl ClassificationPort for PersistentRuntime {
    fn get_classification(
        &self,
        principal: &Principal,
        id: &DocumentId,
    ) -> Result<DocumentClassification, Dok3Error> {
        if id.as_str().trim().is_empty() {
            return Err(Dok3Error::InvalidInput(
                "document_id must not be empty".to_string(),
            ));
        }
        let doc = self.store.get_stored_document(principal, id)?;
        let classifications = self
            .classifications
            .lock()
            .map_err(|_| Dok3Error::Internal("classification store mutex poisoned".into()))?;
        Ok(classifications
            .get(id)
            .unwrap_or_else(|| classification_from_doc(&doc)))
    }

    fn update_classification(
        &self,
        principal: &Principal,
        id: &DocumentId,
        req: &UpdateClassificationRequest,
    ) -> Result<DocumentClassification, Dok3Error> {
        if !principal.is_admin {
            return Err(Dok3Error::Forbidden(format!(
                "principal '{}' does not have admin privileges",
                principal.subject
            )));
        }
        if id.as_str().trim().is_empty() {
            return Err(Dok3Error::InvalidInput(
                "document_id must not be empty".to_string(),
            ));
        }

        let _exec_guard = self
            .job_exec_lock
            .lock()
            .map_err(|_| Dok3Error::Internal("index job execution lock poisoned".into()))?;
        let _orig_bytes = self.store.read_original(principal, id)?;
        let mut doc = self.store.get_stored_document(principal, id)?;

        let orig_body = doc.body.clone();
        let orig_markdown = doc.markdown.clone();
        let orig_uri = doc.source_uri.clone();
        let orig_hash = doc.content_hash.clone();
        let orig_rev = doc.revision_id.clone();

        let (default_category, default_tag) = {
            let sources = self
                .sources
                .lock()
                .map_err(|_| Dok3Error::Internal("source registry mutex poisoned".into()))?;
            sources.defaults_for(doc.source_id.as_str())
        };
        let (updated_classification, pending_sync) = {
            let mut classifications = self
                .classifications
                .lock()
                .map_err(|_| Dok3Error::Internal("classification store mutex poisoned".into()))?;
            let updated = classifications.update_manual_with_defaults(
                &mut doc,
                &default_category,
                &default_tag,
                principal,
                req,
            )?;
            let pending = classifications.is_pending_index_sync(id);
            (updated, pending)
        };

        // Enforce original-content immutability invariant.
        doc.body = orig_body;
        doc.markdown = orig_markdown;
        doc.source_uri = orig_uri;
        doc.content_hash = orig_hash;
        doc.revision_id = orig_rev;

        let outcome = self.store.update_metadata(doc)?;
        if matches!(outcome, PutOutcome::Stored(_)) || pending_sync {
            let rev = outcome.revision().clone();
            self.journal.append(ChangeEvent {
                sequence: Sequence::new(0),
                document_id: id.clone(),
                revision_id: rev,
                kind: ChangeKind::Upsert,
            })?;
            let mut manager = self.manager.lock().map_err(|_| {
                Dok3Error::Internal("persistent index manager mutex poisoned".into())
            })?;
            ensure_active_generation(&mut manager, &self.store, &self.journal, false)
                .map_err(Dok3Error::Index)?;
            manager
                .gc_retired()
                .map_err(|err| Dok3Error::Index(err.to_string()))?;
            if let Ok(mut jobs) = self.jobs.lock() {
                jobs.update_active(
                    manager.state().active_generation.clone(),
                    manager.state().active_epoch,
                );
            }
            drop(manager);
            let mut classifications = self
                .classifications
                .lock()
                .map_err(|_| Dok3Error::Internal("classification store mutex poisoned".into()))?;
            classifications
                .clear_pending_index_sync(id)
                .map_err(Dok3Error::Storage)?;
        }
        Ok(updated_classification)
    }
}

impl SourceRegistryPort for PersistentRuntime {
    fn list_sources(&self) -> Result<SourceListResponse, Dok3Error> {
        let sources = self
            .sources
            .lock()
            .map_err(|_| Dok3Error::Internal("source registry mutex poisoned".into()))?;
        Ok(SourceListResponse {
            sources: sources.list(),
        })
    }

    fn register_source(&self, req: &CreateSourceRequest) -> Result<RegisteredSource, Dok3Error> {
        self.register_source_with_hook(req, || Ok(()))
    }

    fn delete_source(&self, source_id: &str) -> Result<DeleteSourceResponse, Dok3Error> {
        let _exec_guard = self
            .job_exec_lock
            .lock()
            .map_err(|_| Dok3Error::Internal("index job execution lock poisoned".into()))?;

        let (entry, deleted_documents, active_generation, active_epoch) = {
            let mut sources = self
                .sources
                .lock()
                .map_err(|_| Dok3Error::Internal("source registry mutex poisoned".into()))?;
            let entry = sources.mark_pending_delete(source_id)?;

            let mut manager = self.manager.lock().map_err(|_| {
                Dok3Error::Internal("persistent index manager mutex poisoned".into())
            })?;
            let mut classifications = self
                .classifications
                .lock()
                .map_err(|_| Dok3Error::Internal("classification store mutex poisoned".into()))?;
            let (deleted_documents, active_gen, active_ep) = finalize_pending_source_delete_locked(
                &mut manager,
                &mut sources,
                &mut classifications,
                &self.store,
                &self.journal,
                &entry.source_id,
            )?;
            (entry, deleted_documents, active_gen, active_ep)
        };

        if let Ok(mut jobs) = self.jobs.lock() {
            jobs.update_active(active_generation.clone(), active_epoch);
        }

        Ok(DeleteSourceResponse {
            source_id: entry.source_id,
            deleted_documents,
            active_generation,
            active_epoch,
        })
    }
}

impl PersistentRuntime {
    fn register_source_with_hook(
        &self,
        req: &CreateSourceRequest,
        after_refresh: impl FnOnce() -> Result<(), String>,
    ) -> Result<RegisteredSource, Dok3Error> {
        let _exec_guard = self
            .job_exec_lock
            .lock()
            .map_err(|_| Dok3Error::Internal("index job execution lock poisoned".into()))?;

        let registered = {
            let mut sources = self
                .sources
                .lock()
                .map_err(|_| Dok3Error::Internal("source registry mutex poisoned".into()))?;
            let candidate = sources.validate_new_source(req)?;
            let source_id = candidate.source_id.clone();
            sources
                .insert_source(candidate)
                .map_err(Dok3Error::Storage)?;

            let sync_outcome = (|| -> Result<RefreshReport, String> {
                let mut manager = self
                    .manager
                    .lock()
                    .map_err(|_| "persistent index manager mutex poisoned".to_string())?;
                let mut classifications = self
                    .classifications
                    .lock()
                    .map_err(|_| "classification store mutex poisoned".to_string())?;
                let report = refresh_locked(
                    &mut manager,
                    &mut sources,
                    Some(&source_id),
                    &self.store,
                    &self.journal,
                    &mut classifications,
                    false,
                )?;
                after_refresh()?;
                Ok(report)
            })();

            match sync_outcome {
                Ok(refresh) => {
                    let result = sources.get_active(&source_id).ok_or_else(|| {
                        Dok3Error::Internal(format!(
                            "registered source '{source_id}' missing after sync"
                        ))
                    })?;
                    drop(sources);
                    if let Ok(mut jobs) = self.jobs.lock() {
                        jobs.update_active(refresh.active_generation, refresh.active_epoch);
                    }
                    result
                }
                Err(err) => {
                    if let Err(mark_err) = sources.mark_pending_delete(&source_id) {
                        return Err(Dok3Error::Storage(format!(
                            "source registration failed ({err}); failed to record rollback pending delete: {mark_err}"
                        )));
                    }
                    let rollback_res = (|| -> Result<(), Dok3Error> {
                        let mut manager = self.manager.lock().map_err(|_| {
                            Dok3Error::Internal("persistent index manager mutex poisoned".into())
                        })?;
                        let mut classifications = self.classifications.lock().map_err(|_| {
                            Dok3Error::Internal("classification store mutex poisoned".into())
                        })?;
                        let (_, active_gen, active_ep) = finalize_pending_source_delete_locked(
                            &mut manager,
                            &mut sources,
                            &mut classifications,
                            &self.store,
                            &self.journal,
                            &source_id,
                        )?;
                        drop(classifications);
                        drop(manager);
                        if let Ok(mut jobs) = self.jobs.lock() {
                            jobs.update_active(active_gen, active_ep);
                        }
                        Ok(())
                    })();
                    if let Err(rollback_err) = rollback_res {
                        return Err(Dok3Error::Storage(format!(
                            "source registration failed ({err}); rollback failed (kept pending_delete): {rollback_err}"
                        )));
                    }
                    return Err(Dok3Error::Storage(err));
                }
            }
        };
        Ok(registered)
    }
}

fn finalize_pending_source_delete_locked(
    manager: &mut IndexManager<LocalGenerationBackend>,
    sources: &mut SourceRegistry,
    classifications: &mut ClassificationStore,
    store: &FileDocumentStore,
    journal: &FileChangeJournal,
    source_id: &str,
) -> Result<(usize, Option<String>, u64), Dok3Error> {
    manager.reconcile_on_startup().map_err(|err| {
        Dok3Error::Index(format!(
            "persistent index manager reconciliation failed; restart required: {err}"
        ))
    })?;
    let deleted_documents = cleanup_source_docs(source_id, store, journal, classifications)?;
    ensure_active_generation(manager, store, journal, false).map_err(Dok3Error::Index)?;
    classifications
        .clear_all_pending_index_sync()
        .map_err(Dok3Error::Storage)?;
    manager
        .gc_retired()
        .map_err(|err| Dok3Error::Index(err.to_string()))?;
    sources
        .finalize_delete(source_id)
        .map_err(Dok3Error::Storage)?;
    Ok((
        deleted_documents,
        manager.state().active_generation.clone(),
        manager.state().active_epoch,
    ))
}

fn cleanup_source_docs(
    source_id: &str,
    store: &FileDocumentStore,
    journal: &FileChangeJournal,
    classifications: &mut ClassificationStore,
) -> Result<usize, Dok3Error> {
    let docs = store.list_documents()?;
    let mut deleted_documents = 0usize;
    for doc in docs {
        if doc.source_id.as_str() == source_id && store.delete(&doc.id)? {
            journal.append(ChangeEvent {
                sequence: Sequence::new(0),
                document_id: doc.id,
                revision_id: RevisionId::new("deleted"),
                kind: ChangeKind::Delete,
            })?;
            deleted_documents += 1;
        }
    }
    let surviving_ids: HashSet<String> = store
        .document_ids()?
        .into_iter()
        .map(|id| id.as_str().to_string())
        .collect();
    classifications
        .prune_deleted(&surviving_ids)
        .map_err(Dok3Error::Storage)?;
    reconcile_store_journal(store, journal, &classifications.pending_index_ids())
        .map_err(Dok3Error::Storage)?;
    Ok(deleted_documents)
}

fn validate_job_source_id(
    sources: &SourceRegistry,
    source_id: Option<&str>,
) -> Result<Option<String>, Dok3Error> {
    if let Some(raw) = source_id {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Err(Dok3Error::InvalidInput(
                "source_id must not be empty".to_string(),
            ));
        }
        if !sources.contains_active(trimmed) {
            return Err(Dok3Error::NotFound(format!("source '{trimmed}' not found")));
        }
        return Ok(Some(trimmed.to_string()));
    }
    Ok(None)
}

fn refresh_locked(
    manager: &mut IndexManager<LocalGenerationBackend>,
    sources: &mut SourceRegistry,
    target_source_id: Option<&str>,
    store: &FileDocumentStore,
    journal: &FileChangeJournal,
    classifications: &mut ClassificationStore,
    force_rebuild: bool,
) -> Result<RefreshReport, String> {
    manager.reconcile_on_startup().map_err(|err| {
        format!("persistent index manager reconciliation failed; restart required: {err}")
    })?;
    let report = sync_sources_to_storage(
        sources,
        target_source_id,
        store,
        journal,
        classifications,
        force_rebuild,
    )?;
    let rebuilt = ensure_active_generation(manager, store, journal, force_rebuild)?;
    classifications.clear_all_pending_index_sync()?;
    sources.finalize_all_pending_deletes()?;
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
                            && manifest.analyzer_version == current_analyzer_version()
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

pub(crate) fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |dur| u64::try_from(dur.as_millis()).unwrap_or(u64::MAX))
}

pub(crate) fn atomic_write_bytes(path: &Path, bytes: &[u8]) -> Result<(), String> {
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

enum SourceScanOutcome {
    Scanned {
        indexed: usize,
        skipped: Vec<String>,
        seen_ids: HashSet<String>,
    },
    ScanFailed(String),
}

fn scan_and_store_source(
    entry: &RegisteredSource,
    store: &FileDocumentStore,
    journal: &FileChangeJournal,
    classifications: &mut ClassificationStore,
    synced_this_pass: &mut HashSet<String>,
    cursors: &mut SourceCursorStore,
    full: bool,
) -> Result<SourceScanOutcome, String> {
    if matches!(entry.kind, SourceKind::Postgres | SourceKind::Mysql) {
        return scan_and_store_relational_source(
            entry,
            store,
            journal,
            classifications,
            cursors,
            full,
            synced_this_pass,
        );
    }
    if entry.kind == SourceKind::Sqlite {
        return scan_and_store_sqlite_source(
            entry,
            store,
            journal,
            classifications,
            synced_this_pass,
            cursors,
            full,
        );
    }
    if let Err(err) = validate_real_directory(Path::new(&entry.path)) {
        return Ok(SourceScanOutcome::ScanFailed(err.to_string()));
    }
    let connector = match LocalFileConnector::new(source_config_for(entry)) {
        Ok(connector) => connector,
        Err(err) => return Ok(SourceScanOutcome::ScanFailed(err.to_string())),
    };
    let report = match connector.scan() {
        Ok(report) => report,
        Err(err) => return Ok(SourceScanOutcome::ScanFailed(err.to_string())),
    };

    let mut skipped: Vec<String> = report
        .skipped
        .into_iter()
        .map(|file| format!("{}: {}", file.relative_path, file.reason))
        .collect();
    let parser = ParserRegistry::with_defaults();
    let corpus_meta = CorpusMetadataIndex::load(Path::new(&entry.path));
    let mut seen_ids = HashSet::with_capacity(report.files.len());
    let mut indexed = 0usize;
    let mut batch_items: Vec<(Document, Option<Vec<u8>>)> = Vec::with_capacity(report.files.len());

    for file in report.files {
        let parsed = match parser.parse_auto(&file.media_type, &file.filename, &file.bytes) {
            Ok(parsed) => parsed,
            Err(err) => {
                skipped.push(format!("{}: parse failed: {err}", file.relative_path));
                continue;
            }
        };
        let mut document = build_document(connector.config(), &file, &parsed);
        apply_detected_language(&mut document, &parsed.normalized_text);
        let corpus_override = corpus_meta.apply(&mut document, &file.relative_path);
        let rev = content_revision(&file.bytes);
        let content_hash = rev
            .as_str()
            .strip_prefix("sha256:")
            .unwrap_or(rev.as_str())
            .to_string();
        let default_cat = corpus_override
            .as_ref()
            .map_or(connector.config().category.as_str(), |m| {
                m.category.as_str()
            });
        let default_tag = corpus_override
            .as_ref()
            .and_then(|m| m.tags.first().map(String::as_str))
            .unwrap_or(connector.config().default_tag.as_str());
        let manual_locked = classifications.is_manual_locked(&document.id);
        classifications.apply_during_sync_with_defaults(
            &mut document,
            default_cat,
            default_tag,
            &content_hash,
        )?;
        if let Some(meta) = corpus_override
            && !manual_locked
        {
            document.category = meta.category;
            document.topic = meta.topic;
            document.source = meta.source;
            document.version = meta.version;
            if let Some(summary) = meta.summary.as_deref().filter(|s| !s.trim().is_empty()) {
                document.summary = summary.to_string();
            }
            for tag in &meta.tags {
                let trimmed = tag.trim();
                if !trimmed.is_empty()
                    && !document
                        .tags
                        .iter()
                        .any(|t| t.eq_ignore_ascii_case(trimmed))
                {
                    document.tags.push(trimmed.to_string());
                }
            }
        }
        let doc_id = document.id.clone();
        seen_ids.insert(doc_id.as_str().to_string());

        let unchanged_content = store
            .existing_content_hash(&doc_id)
            .map_err(|err| err.to_string())?
            .as_deref()
            == Some(content_hash.as_str());
        let bytes_opt = (!unchanged_content).then_some(file.bytes);
        batch_items.push((document, bytes_opt));
        indexed += 1;
    }

    let outcomes = store
        .sync_scanned_batch(batch_items)
        .map_err(|err| err.to_string())?;
    for (doc_id, put_outcome) in outcomes {
        if let PutOutcome::Stored(rev) = put_outcome {
            journal
                .append(ChangeEvent {
                    sequence: Sequence::new(0),
                    document_id: doc_id.clone(),
                    revision_id: rev,
                    kind: ChangeKind::Upsert,
                })
                .map_err(|err| err.to_string())?;
            synced_this_pass.insert(doc_id.as_str().to_string());
        }
    }

    Ok(SourceScanOutcome::Scanned {
        indexed,
        skipped,
        seen_ids,
    })
}

/// Upper bound on cursor pages per pass to guard against a non-advancing source.
const MAX_SQLITE_PAGES_PER_PASS: usize = 100_000;

fn scan_and_store_sqlite_source(
    entry: &RegisteredSource,
    store: &FileDocumentStore,
    journal: &FileChangeJournal,
    classifications: &mut ClassificationStore,
    synced_this_pass: &mut HashSet<String>,
    cursors: &mut SourceCursorStore,
    full: bool,
) -> Result<SourceScanOutcome, String> {
    if let Err(err) = validate_real_sqlite_file(Path::new(&entry.path)) {
        return Ok(SourceScanOutcome::ScanFailed(err.to_string()));
    }
    let adapter = match sqlite_adapter_for(entry, None) {
        Ok(adapter) => adapter,
        Err(err) => return Ok(SourceScanOutcome::ScanFailed(err.to_string())),
    };
    let mut cursor = if full {
        None
    } else {
        cursors.get(&entry.source_id)
    };
    let mut seen_ids = HashSet::new();
    let mut indexed = 0usize;
    let mut completed = false;

    for _ in 0..MAX_SQLITE_PAGES_PER_PASS {
        let batch = match adapter.collect(cursor.as_ref()) {
            Ok(batch) => batch,
            Err(err) => return Ok(SourceScanOutcome::ScanFailed(err.to_string())),
        };
        let mut batch_items: Vec<(Document, Option<Vec<u8>>)> =
            Vec::with_capacity(batch.documents.len());
        for mut document in batch.documents {
            let bytes = document.body.clone().into_bytes();
            let rev = content_revision(&bytes);
            let content_hash = rev
                .as_str()
                .strip_prefix("sha256:")
                .unwrap_or(rev.as_str())
                .to_string();
            document.category = entry.default_category.clone();
            document.markdown = Some(document.body.clone());
            let text = document.body.clone();
            apply_detected_language(&mut document, &text);
            classifications.apply_during_sync_with_defaults(
                &mut document,
                &entry.default_category,
                &entry.default_tag,
                &content_hash,
            )?;
            let doc_id = document.id.clone();
            seen_ids.insert(doc_id.as_str().to_string());
            let unchanged_content = store
                .existing_content_hash(&doc_id)
                .map_err(|err| err.to_string())?
                .as_deref()
                == Some(content_hash.as_str());
            let bytes_opt = (!unchanged_content).then_some(bytes);
            batch_items.push((document, bytes_opt));
            indexed += 1;
        }
        let outcomes = store
            .sync_scanned_batch(batch_items)
            .map_err(|err| err.to_string())?;
        for (doc_id, put_outcome) in outcomes {
            if let PutOutcome::Stored(rev) = put_outcome {
                journal
                    .append(ChangeEvent {
                        sequence: Sequence::new(0),
                        document_id: doc_id.clone(),
                        revision_id: rev,
                        kind: ChangeKind::Upsert,
                    })
                    .map_err(|err| err.to_string())?;
                synced_this_pass.insert(doc_id.as_str().to_string());
            }
        }

        let next = batch.next_cursor;
        if !batch.has_more {
            cursor = next;
            completed = true;
            break;
        }
        if next == cursor {
            return Ok(SourceScanOutcome::ScanFailed(
                "sqlite cursor did not advance while more rows remain (increase batch_limit)"
                    .to_string(),
            ));
        }
        cursor = next;
    }

    // Persist the cursor only after documents and journal entries are durable.
    if let Some(cursor) = cursor.as_ref() {
        cursors.set(&entry.source_id, cursor)?;
    }
    if !completed {
        return Ok(SourceScanOutcome::ScanFailed(format!(
            "sqlite scan exceeded page limit ({MAX_SQLITE_PAGES_PER_PASS} pages) with more rows remaining"
        )));
    }
    Ok(SourceScanOutcome::Scanned {
        indexed,
        skipped: Vec::new(),
        seen_ids,
    })
}

fn scan_and_store_relational_source(
    entry: &RegisteredSource,
    store: &FileDocumentStore,
    journal: &FileChangeJournal,
    classifications: &mut ClassificationStore,
    cursors: &mut SourceCursorStore,
    full: bool,
    synced_this_pass: &mut HashSet<String>,
) -> Result<SourceScanOutcome, String> {
    let (cfg, dsn) = match relational_source_config_for(entry) {
        Ok(pair) => pair,
        Err(err) => return Ok(SourceScanOutcome::ScanFailed(err.to_string())),
    };
    let mut cursor = if full {
        None
    } else {
        cursors.get(&entry.source_id)
    };
    let mut seen_ids = HashSet::new();
    let mut indexed = 0usize;
    let mut completed = false;

    for _ in 0..MAX_SQLITE_PAGES_PER_PASS {
        let batch = match entry.kind {
            SourceKind::Postgres => {
                let adapter = match RelationalSourceAdapter::new_postgres(cfg.clone(), &dsn) {
                    Ok(a) => a,
                    Err(err) => return Ok(SourceScanOutcome::ScanFailed(err.to_string())),
                };
                match adapter.collect(cursor.as_ref()) {
                    Ok(b) => b,
                    Err(err) => return Ok(SourceScanOutcome::ScanFailed(err.to_string())),
                }
            }
            SourceKind::Mysql => {
                let adapter = match RelationalSourceAdapter::new_mysql(cfg.clone(), &dsn) {
                    Ok(a) => a,
                    Err(err) => return Ok(SourceScanOutcome::ScanFailed(err.to_string())),
                };
                match adapter.collect(cursor.as_ref()) {
                    Ok(b) => b,
                    Err(err) => return Ok(SourceScanOutcome::ScanFailed(err.to_string())),
                }
            }
            _ => unreachable!(),
        };

        let mut batch_items: Vec<(Document, Option<Vec<u8>>)> =
            Vec::with_capacity(batch.documents.len());
        for mut document in batch.documents {
            let bytes = document.body.clone().into_bytes();
            let rev = content_revision(&bytes);
            let content_hash = rev
                .as_str()
                .strip_prefix("sha256:")
                .unwrap_or(rev.as_str())
                .to_string();
            document.category = entry.default_category.clone();
            document.markdown = Some(document.body.clone());
            let text = document.body.clone();
            apply_detected_language(&mut document, &text);
            classifications.apply_during_sync_with_defaults(
                &mut document,
                &entry.default_category,
                &entry.default_tag,
                &content_hash,
            )?;
            let doc_id = document.id.clone();
            seen_ids.insert(doc_id.as_str().to_string());
            let unchanged_content = store
                .existing_content_hash(&doc_id)
                .map_err(|err| err.to_string())?
                .as_deref()
                == Some(content_hash.as_str());
            let bytes_opt = (!unchanged_content).then_some(bytes);
            batch_items.push((document, bytes_opt));
            indexed += 1;
        }
        let outcomes = store
            .sync_scanned_batch(batch_items)
            .map_err(|err| err.to_string())?;
        for (doc_id, put_outcome) in outcomes {
            if let PutOutcome::Stored(rev) = put_outcome {
                journal
                    .append(ChangeEvent {
                        sequence: Sequence::new(0),
                        document_id: doc_id.clone(),
                        revision_id: rev,
                        kind: ChangeKind::Upsert,
                    })
                    .map_err(|err| err.to_string())?;
                synced_this_pass.insert(doc_id.as_str().to_string());
            }
        }
        for del_id in batch.deleted_ids {
            seen_ids.remove(del_id.as_str());
            if store.delete(&del_id).map_err(|err| err.to_string())? {
                journal
                    .append(ChangeEvent {
                        sequence: Sequence::new(0),
                        document_id: del_id,
                        revision_id: RevisionId::new("deleted"),
                        kind: ChangeKind::Delete,
                    })
                    .map_err(|err| err.to_string())?;
            }
        }

        let next = batch.next_cursor;
        if !batch.has_more {
            cursor = next;
            completed = true;
            break;
        }
        if next == cursor {
            return Ok(SourceScanOutcome::ScanFailed(
                "relational cursor did not advance while more rows remain (increase batch_limit)"
                    .to_string(),
            ));
        }
        cursor = next;
    }

    if let Some(cursor) = cursor.as_ref() {
        cursors.set(&entry.source_id, cursor)?;
    }
    if !completed {
        return Ok(SourceScanOutcome::ScanFailed(format!(
            "relational scan exceeded page limit ({MAX_SQLITE_PAGES_PER_PASS} pages) with more rows remaining"
        )));
    }
    Ok(SourceScanOutcome::Scanned {
        indexed,
        skipped: Vec::new(),
        seen_ids,
    })
}

fn doc_counts_by_source(docs: &[Document]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for doc in docs {
        *counts
            .entry(doc.source_id.as_str().to_string())
            .or_insert(0) += 1;
    }
    counts
}

fn sync_sources_to_storage(
    sources: &mut SourceRegistry,
    target_source_id: Option<&str>,
    store: &FileDocumentStore,
    journal: &FileChangeJournal,
    classifications: &mut ClassificationStore,
    full: bool,
) -> Result<BuildReport, String> {
    let targets: Vec<RegisteredSource> = match target_source_id {
        Some(id) => {
            let entry = sources
                .get_active(id)
                .ok_or_else(|| format!("source '{id}' not found"))?;
            vec![entry]
        }
        None => sources.active_sources(),
    };

    let mut build_report = BuildReport::default();
    let mut synced_this_pass = HashSet::new();
    let mut healthy_scanned_sources = HashSet::new();
    let mut seen_ids_by_source: BTreeMap<String, HashSet<String>> = BTreeMap::new();
    let mut scan_outcomes: BTreeMap<String, Result<usize, String>> = BTreeMap::new();
    let mut failed_errors = Vec::new();

    let mut incremental_sqlite_sources = HashSet::new();
    for entry in &targets {
        if matches!(
            entry.kind,
            SourceKind::Sqlite | SourceKind::Postgres | SourceKind::Mysql
        ) && !full
        {
            incremental_sqlite_sources.insert(entry.source_id.clone());
        }
        match scan_and_store_source(
            entry,
            store,
            journal,
            classifications,
            &mut synced_this_pass,
            sources.cursors_mut(),
            full,
        )? {
            SourceScanOutcome::Scanned {
                indexed,
                skipped,
                seen_ids,
            } => {
                build_report.indexed += indexed;
                build_report.skipped.extend(skipped);
                healthy_scanned_sources.insert(entry.source_id.clone());
                seen_ids_by_source.insert(entry.source_id.clone(), seen_ids);
                scan_outcomes.insert(entry.source_id.clone(), Ok(indexed));
            }
            SourceScanOutcome::ScanFailed(err) => {
                let message = format!("source:{}: {err}", entry.source_id);
                build_report.skipped.push(message.clone());
                failed_errors.push(message);
                scan_outcomes.insert(entry.source_id.clone(), Err(err));
            }
        }
    }

    if healthy_scanned_sources.is_empty() && !failed_errors.is_empty() {
        let existing_docs = store.list_documents().map_err(|err| err.to_string())?;
        let counts = doc_counts_by_source(&existing_docs);
        let _ = sources.update_sync_results(&scan_outcomes, &counts, now_ms());
        return Err(failed_errors.join("; "));
    }

    let live_source_ids: HashSet<String> = sources
        .active_sources()
        .into_iter()
        .map(|src| src.source_id)
        .collect();
    let existing_docs = store.list_documents().map_err(|err| err.to_string())?;
    for doc in existing_docs {
        let src_id = doc.source_id.as_str();
        let should_delete = if !live_source_ids.contains(src_id) {
            true
        } else if incremental_sqlite_sources.contains(src_id) {
            // Cursor collection only emits new/updated rows; absence is not deletion.
            false
        } else if healthy_scanned_sources.contains(src_id) {
            !seen_ids_by_source
                .get(src_id)
                .is_some_and(|seen| seen.contains(doc.id.as_str()))
        } else {
            false
        };
        if should_delete && store.delete(&doc.id).map_err(|err| err.to_string())? {
            journal
                .append(ChangeEvent {
                    sequence: Sequence::new(0),
                    document_id: doc.id,
                    revision_id: RevisionId::new("deleted"),
                    kind: ChangeKind::Delete,
                })
                .map_err(|err| err.to_string())?;
        }
    }

    let surviving_docs = store.list_documents().map_err(|err| err.to_string())?;
    let surviving_ids: HashSet<String> = surviving_docs
        .iter()
        .map(|doc| doc.id.as_str().to_string())
        .collect();
    let counts = doc_counts_by_source(&surviving_docs);
    classifications.prune_deleted(&surviving_ids)?;
    sources.update_sync_results(&scan_outcomes, &counts, now_ms())?;

    let pending_for_reconcile: HashSet<String> = classifications
        .pending_index_ids()
        .into_iter()
        .filter(|id| !synced_this_pass.contains(id))
        .collect();
    reconcile_store_journal(store, journal, &pending_for_reconcile)?;

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
        m.schema_version != u64::from(SCHEMA_VERSION)
            || m.analyzer_version != current_analyzer_version()
    })
}

fn reconcile_store_journal(
    store: &FileDocumentStore,
    journal: &FileChangeJournal,
    pending_index_ids: &HashSet<String>,
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
        if latest.get(&key).and_then(|v| v.as_ref()) != Some(&doc.revision_id)
            || pending_index_ids.contains(&key)
        {
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
    use dok3_connectors::SourceConfig;
    use dok3_domain::{
        ClassificationSource, DocumentDetail, DocumentParserPort, HealthReadyResponse,
        SearchResponse, SectionReadResponse, SourceId,
    };
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
    fn rebuilds_generation_with_older_analyzer_version_on_startup() {
        let source_root = tempfile::tempdir().unwrap();
        let data_root = tempfile::tempdir().unwrap();

        fs::write(
            source_root.path().join("guide-a.md"),
            "# Analyzer Upgrade\n\nupgradealpha body text\n",
        )
        .unwrap();

        let (runtime, _) = PersistentRuntime::open(source_root.path(), data_root.path()).unwrap();
        drop(runtime);

        let mut durable = read_durable_state(data_root.path());
        let old_gen = durable.active_generation.clone().unwrap();
        let old_record = durable.generations.get_mut(&old_gen).unwrap();
        let old_manifest = old_record.manifest.as_mut().unwrap();
        old_manifest.analyzer_version = current_analyzer_version() - 1;
        let old_manifest_clone = old_manifest.clone();
        old_record.manifest_checksum = Some(old_manifest_clone.checksum().unwrap());

        let old_gen_dir = data_root.path().join("generations").join(&old_gen);
        fs::write(
            old_gen_dir.join("manifest.json"),
            serde_json::to_vec_pretty(&old_manifest_clone).unwrap(),
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
                .analyzer_version,
            current_analyzer_version()
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
        let parser = dok3_parser::MarkdownParser::new();
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

        let failed = runtime
            .run_job("manual", &TriggerIndexJobRequest::default())
            .unwrap();
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

    #[test]
    fn concurrent_jobs_serialize_epochs_and_preserve_disk_consistency_on_write_failure() {
        let source_root = tempfile::tempdir().unwrap();
        let data_root = tempfile::tempdir().unwrap();
        fs::write(
            source_root.path().join("guide-a.md"),
            "# Concurrency\n\nconcurrencytoken body\n",
        )
        .unwrap();

        let (runtime, _) = PersistentRuntime::open(source_root.path(), data_root.path()).unwrap();
        let runtime = Arc::new(runtime);

        let mut handles = Vec::new();
        for idx in 0..4 {
            let rt = Arc::clone(&runtime);
            handles.push(thread::spawn(move || {
                let trigger = if idx % 2 == 0 { "manual" } else { "periodic" };
                rt.run_job(
                    trigger,
                    &TriggerIndexJobRequest {
                        source_id: None,
                        mode: IndexJobMode::Full,
                    },
                )
            }));
        }
        let mut results = Vec::new();
        for h in handles {
            results.push(h.join().unwrap().unwrap());
        }
        let list = runtime.list_jobs().unwrap();
        assert_eq!(list.jobs.len(), 5);
        assert_eq!(list.active_epoch, 5);
        assert_eq!(list.jobs[0].active_epoch, 5);
        let epochs: Vec<u64> = list.jobs.iter().rev().map(|j| j.active_epoch).collect();
        assert_eq!(epochs, vec![1, 2, 3, 4, 5]);

        // Force persistence failure on index_jobs.json by replacing it with a directory.
        let jobs_path = data_root.path().join("storage/index_jobs.json");
        fs::remove_file(&jobs_path).unwrap();
        fs::create_dir(&jobs_path).unwrap();

        let err = runtime
            .trigger_job(&TriggerIndexJobRequest::default())
            .unwrap_err();
        assert!(matches!(err, Dok3Error::Storage(_)));
        let list_after_err = runtime.list_jobs().unwrap();
        assert_eq!(list_after_err.jobs.len(), 5);
        assert_eq!(list_after_err.active_epoch, 5);

        // Restore a valid JSON file containing a simulated interrupted Running job, then reopen.
        fs::remove_dir(&jobs_path).unwrap();
        let interrupted_disk = JobHistoryDisk {
            next_seq: 6,
            jobs: vec![IndexJobRecord {
                job_id: "job-0006".to_string(),
                source_id: None,
                trigger: "manual".to_string(),
                mode: IndexJobMode::Incremental,
                state: IndexJobState::Running,
                started_at_ms: 1,
                finished_at_ms: None,
                indexed_documents: 0,
                rebuilt: false,
                active_generation: list_after_err.active_generation.clone(),
                active_epoch: 5,
                skipped_files: Vec::new(),
                error: None,
            }],
        };
        fs::write(&jobs_path, serde_json::to_vec(&interrupted_disk).unwrap()).unwrap();
        drop(runtime);

        let (restarted, _) = PersistentRuntime::open(source_root.path(), data_root.path()).unwrap();
        let recovered = restarted.get_job("job-0006").unwrap();
        assert_eq!(recovered.state, IndexJobState::Failed);
        assert_eq!(
            recovered.error.as_deref(),
            Some("interrupted by server restart")
        );
    }

    #[test]
    fn finish_job_write_failure_reports_live_epoch_and_recovers_history() {
        let source_root = tempfile::tempdir().unwrap();
        let data_root = tempfile::tempdir().unwrap();
        fs::write(
            source_root.path().join("guide-a.md"),
            "# Finish Failure\n\nfinishfailureinit token\n",
        )
        .unwrap();

        let (runtime, _) = PersistentRuntime::open(source_root.path(), data_root.path()).unwrap();
        let jobs_path = data_root.path().join("storage/index_jobs.json");

        // Start job succeeds, generation epoch 2 activates, then finish_job persistence fails.
        let err = runtime
            .run_job_with_hook(
                "manual",
                &TriggerIndexJobRequest {
                    source_id: None,
                    mode: IndexJobMode::Full,
                },
                || {
                    fs::remove_file(&jobs_path).unwrap();
                    fs::create_dir(&jobs_path).unwrap();
                },
            )
            .unwrap_err();
        assert!(matches!(err, Dok3Error::Storage(_)));

        // Even while persist failed, list_jobs reports live active epoch 2 and non-zombie Succeeded state.
        let live_list = runtime.list_jobs().unwrap();
        assert_eq!(live_list.active_epoch, 2);
        assert_eq!(live_list.jobs[0].job_id, "job-0002");
        assert_eq!(live_list.jobs[0].state, IndexJobState::Succeeded);
        assert_eq!(live_list.jobs[0].active_epoch, 2);

        // Once disk obstacle is cleared, reading jobs flushes dirty state and restart preserves Succeeded.
        fs::remove_dir(&jobs_path).unwrap();
        let flushed = runtime.get_job("job-0002").unwrap();
        assert_eq!(flushed.state, IndexJobState::Succeeded);
        drop(runtime);

        let (restarted, _) = PersistentRuntime::open(source_root.path(), data_root.path()).unwrap();
        let persisted = restarted.get_job("job-0002").unwrap();
        assert_eq!(persisted.state, IndexJobState::Succeeded);
        assert_eq!(persisted.active_epoch, 2);
    }

    #[test]
    fn manual_classification_keeps_original_bytes_survives_refresh_and_updates_search_and_reset() {
        let source_root = tempfile::tempdir().unwrap();
        let data_root = tempfile::tempdir().unwrap();
        let doc_path = source_root.path().join("guide-a.md");
        fs::write(
            &doc_path,
            "# Retry 운영 매뉴얼\n\nretryoriginaltoken first body\n",
        )
        .unwrap();

        let admin = Principal::local_admin();
        let public = Principal::anonymous_public();
        let doc_id = DocumentId::new("local-markdown:guide-a.md");

        let (runtime, _) = PersistentRuntime::open(source_root.path(), data_root.path()).unwrap();
        assert!(
            data_root
                .path()
                .join("storage/classifications.json")
                .is_file()
        );

        let orig_bytes = runtime.store.read_original(&admin, &doc_id).unwrap();
        let orig_doc = runtime.store.get_stored_document(&admin, &doc_id).unwrap();
        let initial = runtime.get_classification(&public, &doc_id).unwrap();
        assert_eq!(initial.source, ClassificationSource::Rule);
        assert!(!initial.manual_locked);
        assert_eq!(initial.category, "manual");

        let forbidden = runtime.update_classification(
            &public,
            &doc_id,
            &UpdateClassificationRequest {
                category: Some("policy".to_string()),
                ..UpdateClassificationRequest::default()
            },
        );
        assert!(matches!(forbidden, Err(Dok3Error::Forbidden(_))));

        let updated = runtime
            .update_classification(
                &admin,
                &doc_id,
                &UpdateClassificationRequest {
                    category: Some("policy".to_string()),
                    summary: Some("manual classification summary custommanualtoken".to_string()),
                    tags: Some(vec!["manualtagalpha".to_string()]),
                    keywords: Some(vec!["kwbeta".to_string()]),
                    reset_to_rule: false,
                },
            )
            .unwrap();
        assert_eq!(updated.source, ClassificationSource::Manual);
        assert!(updated.manual_locked);
        assert_eq!(updated.category, "policy");
        assert_eq!(updated.content_hash, orig_doc.content_hash);

        // Original bytes, body, markdown, URI, and hashes stay strictly immutable.
        assert_eq!(
            runtime.store.read_original(&admin, &doc_id).unwrap(),
            orig_bytes
        );
        let stored_after = runtime.store.get_stored_document(&admin, &doc_id).unwrap();
        assert_eq!(stored_after.body, orig_doc.body);
        assert_eq!(stored_after.markdown, orig_doc.markdown);
        assert_eq!(stored_after.source_uri, orig_doc.source_uri);
        assert_eq!(stored_after.content_hash, orig_doc.content_hash);
        assert_eq!(stored_after.revision_id, orig_doc.revision_id);

        // Tantivy reflects updated summary, category filter, tags, and keywords immediately.
        for query in ["custommanualtoken", "manualtagalpha", "kwbeta"] {
            let res = runtime
                .service()
                .search(
                    &public,
                    &SearchRequest {
                        query: query.to_string(),
                        filters: dok3_domain::SearchFilters {
                            category: Some("policy".to_string()),
                            ..Default::default()
                        },
                        ..SearchRequest::default()
                    },
                )
                .unwrap();
            assert_eq!(res.total_hits, 1, "missing search hit for {query}");
        }

        // Source body changes + incremental refresh must preserve manual override.
        fs::write(
            &doc_path,
            "# Retry 운영 매뉴얼\n\nrefreshedbodytoken updated body content\n",
        )
        .unwrap();
        runtime
            .run_job(
                "manual",
                &TriggerIndexJobRequest {
                    source_id: None,
                    mode: IndexJobMode::Incremental,
                },
            )
            .unwrap();

        let after_refresh = runtime
            .service()
            .search(
                &public,
                &SearchRequest {
                    query: "refreshedbodytoken".to_string(),
                    filters: dok3_domain::SearchFilters {
                        category: Some("policy".to_string()),
                        ..Default::default()
                    },
                    ..SearchRequest::default()
                },
            )
            .unwrap();
        assert_eq!(after_refresh.total_hits, 1);
        let preserved = runtime.get_classification(&public, &doc_id).unwrap();
        assert_eq!(preserved.source, ClassificationSource::Manual);
        assert!(preserved.manual_locked);
        assert_eq!(preserved.category, "policy");
        assert_eq!(
            preserved.summary,
            "manual classification summary custommanualtoken"
        );
        drop(runtime);

        // Restart persistence & audit log verification.
        let (restarted, _) = PersistentRuntime::open(source_root.path(), data_root.path()).unwrap();
        assert!(
            data_root
                .path()
                .join("storage/classification_audit.jsonl")
                .is_file()
        );
        let persisted = restarted.get_classification(&public, &doc_id).unwrap();
        assert_eq!(persisted.source, ClassificationSource::Manual);
        assert!(persisted.manual_locked);
        assert_eq!(persisted.category, "policy");

        let reset = restarted
            .update_classification(
                &admin,
                &doc_id,
                &UpdateClassificationRequest {
                    reset_to_rule: true,
                    ..UpdateClassificationRequest::default()
                },
            )
            .unwrap();
        assert_eq!(reset.source, ClassificationSource::Rule);
        assert!(!reset.manual_locked);
        assert_eq!(reset.category, "manual");

        let old_policy_hits = restarted
            .service()
            .search(
                &public,
                &SearchRequest {
                    query: "refreshedbodytoken".to_string(),
                    filters: dok3_domain::SearchFilters {
                        category: Some("policy".to_string()),
                        ..Default::default()
                    },
                    ..SearchRequest::default()
                },
            )
            .unwrap();
        assert_eq!(old_policy_hits.total_hits, 0);

        let reverted_hits = restarted
            .service()
            .search(
                &public,
                &SearchRequest {
                    query: "refreshedbodytoken".to_string(),
                    filters: dok3_domain::SearchFilters {
                        category: Some("manual".to_string()),
                        ..Default::default()
                    },
                    ..SearchRequest::default()
                },
            )
            .unwrap();
        assert_eq!(reverted_hits.total_hits, 1);
    }

    #[test]
    fn classification_pending_sync_recovers_after_crash_between_metadata_and_journal() {
        let source_root = tempfile::tempdir().unwrap();
        let data_root = tempfile::tempdir().unwrap();
        fs::write(
            source_root.path().join("guide-a.md"),
            "# Crash Recovery Doc\n\ncrashrecoverytoken body\n",
        )
        .unwrap();

        let admin = Principal::local_admin();
        let public = Principal::anonymous_public();
        let doc_id = DocumentId::new("local-markdown:guide-a.md");

        let (runtime, _) = PersistentRuntime::open(source_root.path(), data_root.path()).unwrap();
        let orig_bytes = runtime.store.read_original(&admin, &doc_id).unwrap();
        let orig_doc = runtime.store.get_stored_document(&admin, &doc_id).unwrap();

        // Simulate crash after saving classifications.json + documents.json before journal/index.
        let mut crashed_doc = runtime.store.get_stored_document(&admin, &doc_id).unwrap();
        let req_a = UpdateClassificationRequest {
            category: Some("policy".to_string()),
            summary: Some("crashrecoverysummaryalpha".to_string()),
            tags: Some(vec!["recoverytag".to_string()]),
            keywords: Some(vec!["pendingidx".to_string()]),
            reset_to_rule: false,
        };
        {
            let mut cls = runtime.classifications.lock().unwrap();
            cls.update_manual(&mut crashed_doc, "docs", &admin, &req_a)
                .unwrap();
            assert!(cls.is_pending_index_sync(&doc_id));
        }
        assert!(matches!(
            runtime.store.update_metadata(crashed_doc).unwrap(),
            PutOutcome::Stored(_)
        ));
        assert_eq!(
            runtime
                .service()
                .search(
                    &public,
                    &SearchRequest {
                        query: "crashrecoverysummaryalpha".to_string(),
                        filters: dok3_domain::SearchFilters {
                            category: Some("policy".to_string()),
                            ..Default::default()
                        },
                        ..SearchRequest::default()
                    },
                )
                .unwrap()
                .total_hits,
            0
        );

        // Part (a): Retrying the identical PATCH recovers journal + Tantivy index immediately.
        runtime
            .update_classification(&admin, &doc_id, &req_a)
            .unwrap();
        assert!(
            !runtime
                .classifications
                .lock()
                .unwrap()
                .is_pending_index_sync(&doc_id)
        );
        assert_eq!(
            runtime
                .service()
                .search(
                    &public,
                    &SearchRequest {
                        query: "crashrecoverysummaryalpha".to_string(),
                        filters: dok3_domain::SearchFilters {
                            category: Some("policy".to_string()),
                            ..Default::default()
                        },
                        ..SearchRequest::default()
                    },
                )
                .unwrap()
                .total_hits,
            1
        );

        // Part (b): Another crash between metadata write and journal is recovered on restart.
        let mut crashed_doc_b = runtime.store.get_stored_document(&admin, &doc_id).unwrap();
        let req_b = UpdateClassificationRequest {
            category: Some("manual".to_string()),
            summary: Some("restartrecoveredbeta".to_string()),
            tags: Some(vec!["restarttag".to_string()]),
            keywords: Some(vec!["restartkw".to_string()]),
            reset_to_rule: false,
        };
        {
            let mut cls = runtime.classifications.lock().unwrap();
            cls.update_manual(&mut crashed_doc_b, "docs", &admin, &req_b)
                .unwrap();
            assert!(cls.is_pending_index_sync(&doc_id));
        }
        assert!(matches!(
            runtime.store.update_metadata(crashed_doc_b).unwrap(),
            PutOutcome::Stored(_)
        ));
        drop(runtime);

        let (restarted, _) = PersistentRuntime::open(source_root.path(), data_root.path()).unwrap();
        assert!(
            !restarted
                .classifications
                .lock()
                .unwrap()
                .is_pending_index_sync(&doc_id)
        );
        assert_eq!(
            restarted
                .service()
                .search(
                    &public,
                    &SearchRequest {
                        query: "restartrecoveredbeta".to_string(),
                        filters: dok3_domain::SearchFilters {
                            category: Some("manual".to_string()),
                            ..Default::default()
                        },
                        ..SearchRequest::default()
                    },
                )
                .unwrap()
                .total_hits,
            1
        );
        let stored_after = restarted
            .store
            .get_stored_document(&admin, &doc_id)
            .unwrap();
        assert_eq!(stored_after.revision_id, orig_doc.revision_id);
        assert_eq!(stored_after.content_hash, orig_doc.content_hash);
        assert_eq!(
            restarted.store.read_original(&admin, &doc_id).unwrap(),
            orig_bytes
        );
    }

    #[test]
    fn refresh_rejects_corrupted_existing_object_without_overwriting() {
        let source_root = tempfile::tempdir().unwrap();
        let data_root = tempfile::tempdir().unwrap();
        fs::write(
            source_root.path().join("guide-a.md"),
            "# Object Immutability\n\nimmutableobjectcheck body\n",
        )
        .unwrap();

        let admin = Principal::local_admin();
        let doc_id = DocumentId::new("local-markdown:guide-a.md");
        let (runtime, _) = PersistentRuntime::open(source_root.path(), data_root.path()).unwrap();
        let hash = runtime
            .store
            .get_stored_document(&admin, &doc_id)
            .unwrap()
            .content_hash;
        let obj_path = data_root.path().join("storage/objects").join(&hash);

        fs::write(&obj_path, b"corrupted-by-fault").unwrap();
        assert!(runtime.refresh().is_err());
        assert_eq!(fs::read(&obj_path).unwrap(), b"corrupted-by-fault");
    }

    #[test]
    fn multi_source_registration_rescan_restart_error_isolation_and_delete_preserves_disk() {
        let env_dir = tempfile::tempdir().unwrap();
        let team_dir = tempfile::tempdir().unwrap();
        let broken_dir = tempfile::tempdir().unwrap();
        let data_root = tempfile::tempdir().unwrap();

        fs::write(
            env_dir.path().join("env-a.md"),
            "# Env Guide\n\nenvsourcetoken body\n",
        )
        .unwrap();
        fs::write(
            team_dir.path().join("note-b.md"),
            "# Team Notes\n\nteamsourcetoken body\n",
        )
        .unwrap();
        fs::write(
            broken_dir.path().join("temp-c.md"),
            "# Broken Source Doc\n\nbrokensourcetoken body\n",
        )
        .unwrap();

        let public = Principal::anonymous_public();
        let (runtime, _) = PersistentRuntime::open(env_dir.path(), data_root.path()).unwrap();
        assert!(data_root.path().join("storage/sources.json").is_file());

        let initial_sources = runtime.list_sources().unwrap();
        assert_eq!(initial_sources.sources.len(), 1);
        assert_eq!(initial_sources.sources[0].source_id, "local-markdown");
        assert!(initial_sources.sources[0].managed_by_env);
        assert_eq!(initial_sources.sources[0].document_count, 1);

        // Environment-managed source cannot be deleted.
        assert!(matches!(
            runtime.delete_source("local-markdown"),
            Err(Dok3Error::InvalidInput(_))
        ));

        // Register a second local source with custom default category and tag.
        let team_src = runtime
            .register_source(&CreateSourceRequest {
                source_id: "team-notes".to_string(),
                label: "Team Notes".to_string(),
                path: team_dir.path().to_string_lossy().into_owned(),
                default_category: Some("manual".to_string()),
                default_tag: Some("ops".to_string()),
                kind: dok3_domain::SourceKind::LocalDir,
                sqlite: None,
                relational: None,
            })
            .unwrap();
        assert_eq!(team_src.source_id, "team-notes");
        assert_eq!(team_src.document_count, 1);
        assert!(!team_src.managed_by_env);
        assert!(team_src.last_synced_at_ms.is_some());

        // Reject duplicate slug or canonical path.
        assert!(matches!(
            runtime.register_source(&CreateSourceRequest {
                source_id: "team-notes".to_string(),
                label: "Dup Slug".to_string(),
                path: broken_dir.path().to_string_lossy().into_owned(),
                default_category: None,
                default_tag: None,
                kind: dok3_domain::SourceKind::LocalDir,
                sqlite: None,
                relational: None,
            }),
            Err(Dok3Error::InvalidInput(_))
        ));
        assert!(matches!(
            runtime.register_source(&CreateSourceRequest {
                source_id: "dup-path".to_string(),
                label: "Dup Path".to_string(),
                path: team_dir.path().to_string_lossy().into_owned(),
                default_category: None,
                default_tag: None,
                kind: dok3_domain::SourceKind::LocalDir,
                sqlite: None,
                relational: None,
            }),
            Err(Dok3Error::InvalidInput(_))
        ));

        let team_hits = runtime
            .service()
            .search(
                &public,
                &SearchRequest {
                    query: "teamsourcetoken".to_string(),
                    filters: dok3_domain::SearchFilters {
                        category: Some("manual".to_string()),
                        ..Default::default()
                    },
                    ..SearchRequest::default()
                },
            )
            .unwrap();
        assert_eq!(team_hits.total_hits, 1);
        assert_eq!(team_hits.hits[0].category, "manual");
        assert!(team_hits.hits[0].tags.contains(&"ops".to_string()));

        // Targeted single-source rescan indexes new team file without disturbing unscanned env docs.
        fs::write(
            team_dir.path().join("added-d.md"),
            "# Team Added\n\nteamsinglescantoken\n",
        )
        .unwrap();
        fs::write(
            env_dir.path().join("unscanned-e.md"),
            "# Env Added\n\nenvunscannedtoken\n",
        )
        .unwrap();
        let partial_job = runtime
            .trigger_job(&TriggerIndexJobRequest {
                source_id: Some("team-notes".to_string()),
                mode: IndexJobMode::Incremental,
            })
            .unwrap();
        assert_eq!(partial_job.state, IndexJobState::Succeeded);
        assert_eq!(
            runtime
                .service()
                .search(
                    &public,
                    &SearchRequest {
                        query: "teamsinglescantoken".to_string(),
                        ..SearchRequest::default()
                    },
                )
                .unwrap()
                .total_hits,
            1
        );
        assert_eq!(
            runtime
                .service()
                .search(
                    &public,
                    &SearchRequest {
                        query: "envunscannedtoken".to_string(),
                        ..SearchRequest::default()
                    },
                )
                .unwrap()
                .total_hits,
            0
        );
        assert_eq!(
            runtime
                .service()
                .search(
                    &public,
                    &SearchRequest {
                        query: "envsourcetoken".to_string(),
                        ..SearchRequest::default()
                    },
                )
                .unwrap()
                .total_hits,
            1
        );

        // Broken source isolation: deleting broken_dir records last_error while keeping healthy sync & existing docs.
        runtime
            .register_source(&CreateSourceRequest {
                source_id: "broken-src".to_string(),
                label: "Broken Source".to_string(),
                path: broken_dir.path().to_string_lossy().into_owned(),
                default_category: None,
                default_tag: None,
                kind: dok3_domain::SourceKind::LocalDir,
                sqlite: None,
                relational: None,
            })
            .unwrap();
        fs::remove_dir_all(broken_dir.path()).unwrap();

        let all_job = runtime
            .trigger_job(&TriggerIndexJobRequest::default())
            .unwrap();
        assert_eq!(all_job.state, IndexJobState::Succeeded);
        let sources_after_err = runtime.list_sources().unwrap();
        let broken_state = sources_after_err
            .sources
            .iter()
            .find(|s| s.source_id == "broken-src")
            .unwrap();
        assert!(broken_state.last_error.is_some());
        assert_eq!(broken_state.document_count, 1);
        assert_eq!(
            runtime
                .service()
                .search(
                    &public,
                    &SearchRequest {
                        query: "brokensourcetoken".to_string(),
                        ..SearchRequest::default()
                    },
                )
                .unwrap()
                .total_hits,
            1
        );
        assert_eq!(
            runtime
                .service()
                .search(
                    &public,
                    &SearchRequest {
                        query: "envunscannedtoken".to_string(),
                        ..SearchRequest::default()
                    },
                )
                .unwrap()
                .total_hits,
            1
        );
        drop(runtime);

        // Restart restores all sources from sources.json.
        let (restarted, _) = PersistentRuntime::open(env_dir.path(), data_root.path()).unwrap();
        assert_eq!(restarted.list_sources().unwrap().sources.len(), 3);

        // Deleting team-notes removes its indexed documents & classifications but leaves user disk files untouched.
        let deleted = restarted.delete_source("team-notes").unwrap();
        assert_eq!(deleted.source_id, "team-notes");
        assert_eq!(deleted.deleted_documents, 2);
        assert_eq!(
            restarted
                .service()
                .search(
                    &public,
                    &SearchRequest {
                        query: "teamsourcetoken".to_string(),
                        ..SearchRequest::default()
                    },
                )
                .unwrap()
                .total_hits,
            0
        );
        assert!(matches!(
            restarted.get_classification(&public, &DocumentId::new("team-notes:note-b.md")),
            Err(Dok3Error::NotFound(_))
        ));
        assert!(team_dir.path().join("note-b.md").is_file());
        assert!(team_dir.path().join("added-d.md").is_file());
    }

    #[test]
    fn interrupted_source_delete_retries_without_404_and_recovers_on_restart() {
        let env_dir = tempfile::tempdir().unwrap();
        let src_a_dir = tempfile::tempdir().unwrap();
        let src_b_dir = tempfile::tempdir().unwrap();
        let data_root = tempfile::tempdir().unwrap();

        fs::write(env_dir.path().join("env.md"), "# Env\n\nenvkeep\n").unwrap();
        fs::write(src_a_dir.path().join("a.md"), "# Source A\n\npendingdela\n").unwrap();
        fs::write(src_b_dir.path().join("b.md"), "# Source B\n\npendingdelb\n").unwrap();

        let public = Principal::anonymous_public();
        let (runtime, _) = PersistentRuntime::open(env_dir.path(), data_root.path()).unwrap();
        runtime
            .register_source(&CreateSourceRequest {
                source_id: "src-a".to_string(),
                label: "Source A".to_string(),
                path: src_a_dir.path().to_string_lossy().into_owned(),
                default_category: None,
                default_tag: None,
                kind: dok3_domain::SourceKind::LocalDir,
                sqlite: None,
                relational: None,
            })
            .unwrap();
        runtime
            .register_source(&CreateSourceRequest {
                source_id: "src-b".to_string(),
                label: "Source B".to_string(),
                path: src_b_dir.path().to_string_lossy().into_owned(),
                default_category: None,
                default_tag: None,
                kind: dok3_domain::SourceKind::LocalDir,
                sqlite: None,
                relational: None,
            })
            .unwrap();

        // Simulate interrupted generation build AND failed index transition during DELETE,
        // then verify retrying DELETE reconciles IndexManager and finishes cleanly without 404.
        runtime
            .manager
            .lock()
            .unwrap()
            .build_generation("gen-interrupted-del", 99)
            .unwrap();
        let blocking_del_gen = data_root.path().join("generations/gen-000004-seq-000004");
        fs::write(&blocking_del_gen, b"block-delete-gen").unwrap();
        assert!(runtime.delete_source("src-a").is_err());
        assert!(
            runtime
                .list_sources()
                .unwrap()
                .sources
                .iter()
                .any(|s| s.source_id == "src-a")
        );

        fs::remove_file(&blocking_del_gen).unwrap();
        runtime
            .manager
            .lock()
            .unwrap()
            .build_generation("gen-interrupted-retry", 99)
            .unwrap();
        let retried = runtime.delete_source("src-a").unwrap();
        assert_eq!(retried.source_id, "src-a");
        assert_eq!(
            runtime
                .service()
                .search(
                    &public,
                    &SearchRequest {
                        query: "pendingdela".to_string(),
                        ..SearchRequest::default()
                    },
                )
                .unwrap()
                .total_hits,
            0
        );

        // Now mark pending_delete for src-b and drop runtime (simulate restart recovery).
        runtime
            .sources
            .lock()
            .unwrap()
            .mark_pending_delete("src-b")
            .unwrap();
        drop(runtime);

        let (restarted, _) = PersistentRuntime::open(env_dir.path(), data_root.path()).unwrap();
        assert!(
            !restarted
                .list_sources()
                .unwrap()
                .sources
                .iter()
                .any(|s| s.source_id == "src-b")
        );
        assert_eq!(
            restarted
                .service()
                .search(
                    &public,
                    &SearchRequest {
                        query: "pendingdelb".to_string(),
                        ..SearchRequest::default()
                    },
                )
                .unwrap()
                .total_hits,
            0
        );
        assert!(src_b_dir.path().join("b.md").is_file());
    }

    #[test]
    fn failed_source_registration_cleans_up_orphaned_docs_and_allows_retry() {
        let env_dir = tempfile::tempdir().unwrap();
        let team_dir = tempfile::tempdir().unwrap();
        let post_dir = tempfile::tempdir().unwrap();
        let data_root = tempfile::tempdir().unwrap();

        fs::write(env_dir.path().join("env.md"), "# Env\n\nenvstable\n").unwrap();
        fs::write(
            team_dir.path().join("note.md"),
            "# Team Doc\n\nteamrollbacktoken\n",
        )
        .unwrap();
        fs::write(
            post_dir.path().join("post.md"),
            "# Post Activate Fail\n\npostfailtoken\n",
        )
        .unwrap();

        let public = Principal::anonymous_public();
        let (runtime, _) = PersistentRuntime::open(env_dir.path(), data_root.path()).unwrap();
        // Part A: Force generation build failure BEFORE activation (after store/journal writes).
        let blocking_gen = data_root.path().join("generations/gen-000002-seq-000002");
        fs::write(&blocking_gen, b"block-build-generation").unwrap();

        let err = runtime
            .register_source(&CreateSourceRequest {
                source_id: "team-notes".to_string(),
                label: "Team Notes".to_string(),
                path: team_dir.path().to_string_lossy().into_owned(),
                default_category: None,
                default_tag: None,
                kind: dok3_domain::SourceKind::LocalDir,
                sqlite: None,
                relational: None,
            })
            .unwrap_err();
        assert!(matches!(err, Dok3Error::Storage(_)));

        // Zero orphaned docs remain in store and source is removed cleanly from registry.
        assert!(
            !runtime
                .list_sources()
                .unwrap()
                .sources
                .iter()
                .any(|s| s.source_id == "team-notes")
        );
        assert!(
            !runtime
                .store
                .document_ids()
                .unwrap()
                .iter()
                .any(|id| id.as_str().starts_with("team-notes:"))
        );

        // Clearing fault allows clean retry of the exact same registration.
        fs::remove_file(&blocking_gen).unwrap();
        let registered = runtime
            .register_source(&CreateSourceRequest {
                source_id: "team-notes".to_string(),
                label: "Team Notes".to_string(),
                path: team_dir.path().to_string_lossy().into_owned(),
                default_category: None,
                default_tag: None,
                kind: dok3_domain::SourceKind::LocalDir,
                sqlite: None,
                relational: None,
            })
            .unwrap();
        assert_eq!(registered.source_id, "team-notes");
        assert_eq!(registered.document_count, 1);

        // Part B: Simulate failure AFTER generation activation: rollback MUST republish active index
        // without the rolled-back source's docs AND remove it from registry.
        let post_err = runtime
            .register_source_with_hook(
                &CreateSourceRequest {
                    source_id: "team-postfail".to_string(),
                    label: "Team Post Fail".to_string(),
                    path: post_dir.path().to_string_lossy().into_owned(),
                    default_category: None,
                    default_tag: None,
                    kind: dok3_domain::SourceKind::LocalDir,
                    sqlite: None,
                    relational: None,
                },
                || Err("simulated post-activation failure".to_string()),
            )
            .unwrap_err();
        assert!(matches!(post_err, Dok3Error::Storage(_)));
        assert!(
            !runtime
                .list_sources()
                .unwrap()
                .sources
                .iter()
                .any(|s| s.source_id == "team-postfail")
        );
        assert_eq!(
            runtime
                .service()
                .search(
                    &public,
                    &SearchRequest {
                        query: "postfailtoken".to_string(),
                        ..SearchRequest::default()
                    },
                )
                .unwrap()
                .total_hits,
            0
        );

        // Clean retry after post-activation rollback succeeds.
        let retried = runtime
            .register_source(&CreateSourceRequest {
                source_id: "team-postfail".to_string(),
                label: "Team Post Fail".to_string(),
                path: post_dir.path().to_string_lossy().into_owned(),
                default_category: None,
                default_tag: None,
                kind: dok3_domain::SourceKind::LocalDir,
                sqlite: None,
                relational: None,
            })
            .unwrap();
        assert_eq!(retried.document_count, 1);
        assert_eq!(
            runtime
                .service()
                .search(
                    &public,
                    &SearchRequest {
                        query: "postfailtoken".to_string(),
                        ..SearchRequest::default()
                    },
                )
                .unwrap()
                .total_hits,
            1
        );
    }

    #[test]
    fn persistent_runtime_indexes_code_and_text_and_refreshes_incrementally() {
        let source_root = tempfile::tempdir().unwrap();
        let data_root = tempfile::tempdir().unwrap();
        fs::write(source_root.path().join("guide.md"), "# Guide\n\nmdtoken\n").unwrap();
        fs::write(
            source_root.path().join("lib.rs"),
            "// firstrusttoken\npub fn alpha() {}\n",
        )
        .unwrap();
        fs::write(source_root.path().join("note.txt"), "plaintexttoken\n").unwrap();
        fs::write(
            source_root.path().join("schema.sql"),
            "CREATE TABLE accounts (id INT);\n",
        )
        .unwrap();

        let (runtime, report) =
            PersistentRuntime::open(source_root.path(), data_root.path()).unwrap();
        assert_eq!(report.indexed, 4, "{report:?}");
        let anon = Principal::anonymous_public();
        let hits = |runtime: &PersistentRuntime, query: &str| {
            runtime
                .service()
                .search(
                    &anon,
                    &SearchRequest {
                        query: query.to_string(),
                        ..SearchRequest::default()
                    },
                )
                .unwrap()
                .total_hits
        };
        assert_eq!(hits(&runtime, "firstrusttoken"), 1);
        assert_eq!(hits(&runtime, "plaintexttoken"), 1);
        for name in ["lib.rs", "schema.sql"] {
            let doc = runtime
                .service()
                .get_document(&anon, &DocumentId::new(format!("local-markdown:{name}")))
                .unwrap();
            assert_eq!(doc.category, "code", "{name}");
        }

        fs::write(
            source_root.path().join("lib.rs"),
            "// secondrusttoken\npub fn alpha() {}\n",
        )
        .unwrap();
        fs::remove_file(source_root.path().join("note.txt")).unwrap();
        let refreshed = runtime.refresh().unwrap();
        assert!(refreshed.rebuilt);
        assert_eq!(hits(&runtime, "firstrusttoken"), 0);
        assert_eq!(hits(&runtime, "secondrusttoken"), 1);
        assert_eq!(hits(&runtime, "plaintexttoken"), 0);
    }

    fn run_sqlite_script(db: &Path, script: &str) {
        let status = std::process::Command::new("python3")
            .arg("-c")
            .arg(
                "import sqlite3,sys\nc=sqlite3.connect(sys.argv[1])\nc.executescript(sys.argv[2])\nc.commit()\nc.close()",
            )
            .arg(db)
            .arg(script)
            .status()
            .unwrap();
        assert!(status.success());
    }

    fn sqlite_options() -> dok3_domain::SqliteSourceOptions {
        dok3_domain::SqliteSourceOptions {
            select_query: "SELECT id, title, body FROM notes".to_string(),
            id_column: "id".to_string(),
            title_column: "title".to_string(),
            body_column: "body".to_string(),
            summary_column: None,
            cursor_mode: dok3_domain::SqliteCursorKind::IntegerPk,
            cursor_column: "id".to_string(),
            batch_limit: Some(2),
        }
    }

    fn sqlite_request(db: &Path) -> CreateSourceRequest {
        CreateSourceRequest {
            source_id: "notes-db".to_string(),
            label: "Notes DB".to_string(),
            path: db.to_string_lossy().into_owned(),
            kind: SourceKind::Sqlite,
            sqlite: Some(sqlite_options()),
            relational: None,
            default_category: None,
            default_tag: None,
        }
    }

    #[test]
    fn sqlite_source_indexes_incrementally_reconciles_on_full_and_keeps_db_on_delete() {
        let env_dir = tempfile::tempdir().unwrap();
        let data_root = tempfile::tempdir().unwrap();
        let db_dir = tempfile::tempdir().unwrap();
        let db = db_dir.path().join("notes.db");
        run_sqlite_script(
            &db,
            "CREATE TABLE notes(id INTEGER PRIMARY KEY, title TEXT, body TEXT);\
             INSERT INTO notes VALUES (1,'One','sqliteaaatoken'),(2,'Two','sqlitebbbtoken'),(3,'Three','sqliteccctoken');",
        );
        fs::write(env_dir.path().join("env.md"), "# Env\n\nenvdoc\n").unwrap();

        let (runtime, _) = PersistentRuntime::open(env_dir.path(), data_root.path()).unwrap();
        let anon = Principal::anonymous_public();
        let hits = |runtime: &PersistentRuntime, query: &str| {
            runtime
                .service()
                .search(
                    &anon,
                    &SearchRequest {
                        query: query.to_string(),
                        ..SearchRequest::default()
                    },
                )
                .unwrap()
                .total_hits
        };

        let registered = runtime.register_source(&sqlite_request(&db)).unwrap();
        assert_eq!(registered.kind, SourceKind::Sqlite);
        assert_eq!(registered.default_category, "guide");
        assert_eq!(registered.default_tag, "sqlite");
        assert_eq!(registered.document_count, 3);
        assert_eq!(hits(&runtime, "sqliteaaatoken"), 1);
        assert_eq!(hits(&runtime, "sqliteccctoken"), 1);
        let cursor_file = data_root.path().join("storage/source_cursors.json");
        assert!(
            fs::read_to_string(&cursor_file)
                .unwrap()
                .contains("notes-db")
        );

        // Duplicate (path, query) is rejected; bad SQL is rejected at registration.
        let mut dup = sqlite_request(&db);
        dup.source_id = "notes-db-2".to_string();
        assert!(matches!(
            runtime.register_source(&dup),
            Err(Dok3Error::InvalidInput(_))
        ));
        let mut bad = sqlite_request(&db);
        bad.source_id = "bad-sql".to_string();
        bad.sqlite.as_mut().unwrap().select_query = "SELECT id, title, nope FROM notes".to_string();
        assert!(runtime.register_source(&bad).is_err());
        assert!(
            runtime
                .list_sources()
                .unwrap()
                .sources
                .iter()
                .all(|src| src.source_id != "bad-sql")
        );

        // Incremental: new rows indexed, existing rows kept, cursor advanced.
        run_sqlite_script(
            &db,
            "INSERT INTO notes VALUES (4,'Four','sqliteddtoken'),(5,'Five','sqliteeetoken');",
        );
        let job = runtime
            .run_job(
                "test",
                &TriggerIndexJobRequest {
                    source_id: Some("notes-db".to_string()),
                    mode: IndexJobMode::Incremental,
                },
            )
            .unwrap();
        assert_eq!(job.state, IndexJobState::Succeeded, "{job:?}");
        assert_eq!(hits(&runtime, "sqliteddtoken"), 1);
        assert_eq!(hits(&runtime, "sqliteeetoken"), 1);
        assert_eq!(hits(&runtime, "sqliteaaatoken"), 1);
        let cursor_json = fs::read_to_string(&cursor_file).unwrap();
        assert!(cursor_json.contains("\"value\": 5"), "{cursor_json}");
        let count = |runtime: &PersistentRuntime| {
            runtime
                .list_sources()
                .unwrap()
                .sources
                .iter()
                .find(|src| src.source_id == "notes-db")
                .unwrap()
                .document_count
        };
        assert_eq!(count(&runtime), 5);

        // Source row deletion is ignored by incremental passes but reconciled by Full.
        run_sqlite_script(&db, "DELETE FROM notes WHERE id = 2;");
        let job = runtime
            .run_job(
                "test",
                &TriggerIndexJobRequest {
                    source_id: Some("notes-db".to_string()),
                    mode: IndexJobMode::Incremental,
                },
            )
            .unwrap();
        assert_eq!(job.state, IndexJobState::Succeeded);
        assert_eq!(hits(&runtime, "sqlitebbbtoken"), 1);
        let job = runtime
            .run_job(
                "test",
                &TriggerIndexJobRequest {
                    source_id: Some("notes-db".to_string()),
                    mode: IndexJobMode::Full,
                },
            )
            .unwrap();
        assert_eq!(job.state, IndexJobState::Succeeded, "{job:?}");
        assert_eq!(hits(&runtime, "sqlitebbbtoken"), 0);
        assert_eq!(hits(&runtime, "sqliteaaatoken"), 1);
        assert_eq!(count(&runtime), 4);
        assert_eq!(hits(&runtime, "envdoc"), 1);

        // Deleting the source removes its documents and cursor but never the .db file.
        let deleted = runtime.delete_source("notes-db").unwrap();
        assert_eq!(deleted.deleted_documents, 4);
        assert_eq!(hits(&runtime, "sqliteaaatoken"), 0);
        assert!(db.is_file());
        assert!(
            !fs::read_to_string(&cursor_file)
                .unwrap()
                .contains("notes-db")
        );
        assert_eq!(hits(&runtime, "envdoc"), 1);
    }

    #[tokio::test]
    async fn sources_http_accepts_kindless_local_dir_and_sqlite_requests() {
        let env_dir = tempfile::tempdir().unwrap();
        let data_root = tempfile::tempdir().unwrap();
        let team_dir = tempfile::tempdir().unwrap();
        let db_dir = tempfile::tempdir().unwrap();
        let db = db_dir.path().join("notes.db");
        run_sqlite_script(
            &db,
            "CREATE TABLE notes(id INTEGER PRIMARY KEY, title TEXT, body TEXT);\
             INSERT INTO notes VALUES (1,'One','httpsqlitetoken');",
        );
        fs::write(env_dir.path().join("env.md"), "# Env\n\nenvdoc\n").unwrap();
        fs::write(team_dir.path().join("team.md"), "# Team\n\nhttpteamtoken\n").unwrap();

        let (runtime, _) = PersistentRuntime::open(env_dir.path(), data_root.path()).unwrap();
        let runtime = Arc::new(runtime);
        let state = ApiState::new(Arc::new(runtime.service().clone()))
            .with_principal(Principal::local_admin())
            .with_sources(Arc::clone(&runtime) as _);
        let app = build_router(state, None);

        let post = |body: serde_json::Value| {
            Request::builder()
                .method("POST")
                .uri("/api/v1/sources")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(body.to_string()))
                .unwrap()
        };
        let response = app
            .clone()
            .oneshot(post(serde_json::json!({
                "source_id": "team-http",
                "label": "Team",
                "path": team_dir.path().to_string_lossy(),
            })))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
        let created: RegisteredSource = json(response).await;
        assert_eq!(created.kind, SourceKind::LocalDir);
        assert_eq!(created.document_count, 1);

        let response = app
            .clone()
            .oneshot(post(serde_json::json!({
                "source_id": "sqlite-http",
                "label": "SQLite",
                "path": db.to_string_lossy(),
                "kind": "sqlite",
                "sqlite": {
                    "select_query": "SELECT id, title, body FROM notes",
                    "id_column": "id",
                    "title_column": "title",
                    "body_column": "body",
                    "cursor_mode": "integer_pk",
                    "cursor_column": "id"
                }
            })))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
        let created: RegisteredSource = json(response).await;
        assert_eq!(created.kind, SourceKind::Sqlite);
        assert_eq!(created.default_tag, "sqlite");
        assert_eq!(created.document_count, 1);

        // sqlite options on a local_dir request are rejected.
        let response = app
            .clone()
            .oneshot(post(serde_json::json!({
                "source_id": "mixed",
                "label": "Mixed",
                "path": team_dir.path().to_string_lossy(),
                "sqlite": {
                    "select_query": "SELECT 1",
                    "id_column": "id",
                    "title_column": "t",
                    "body_column": "b",
                    "cursor_mode": "integer_pk",
                    "cursor_column": "id"
                }
            })))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
