//! Durable file-backed document storage, change journal, and Blue-Green generation wiring.

use std::collections::{BTreeMap, HashSet};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use dok3_connectors::{LocalFileConnector, SourceConfig, build_document};
use dok3_core::Dok3Error;
use dok3_domain::{
    ChangeEvent, ChangeJournalPort, ChangeKind, Document, DocumentParserPort, DocumentStorePort,
    GenerationId, Principal, ReaderState, RevisionId, SearchPort, SearchRequest, SearchResponse,
    SearchSnapshotMeta, Sequence, SourceId,
};
use dok3_index::{SCHEMA_VERSION, TantivyIndex};
use dok3_index_manager::{
    Error as ManagerError, GenerationBackend, IndexManager, Manifest, ReaderAccess,
    Result as ManagerResult, SearchSnapshot, ValidationCheck,
};
use dok3_parser::MarkdownParser;
use dok3_search::SearchService;
use dok3_storage::{FileChangeJournal, FileDocumentStore, PutOutcome};

use crate::local_index::{BuildReport, apply_detected_language};

const ANALYZER_VERSION: u64 = 1;
const ACL_POLICY_VERSION: u64 = 1;
const DEFAULT_AVAILABLE_DISK_BYTES: u64 = 100 * 1024 * 1024 * 1024;

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
    _manager: Mutex<IndexManager<LocalGenerationBackend>>,
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

/// Scans `source_dir`, persists originals/metadata/journal under `data_dir/storage`, and
/// serves the active Blue-Green generation managed under `data_dir/generations`.
pub fn build_persistent(
    source_dir: &Path,
    data_dir: &Path,
) -> Result<(SearchService, BuildReport), String> {
    let storage_dir = data_dir.join("storage");
    let generations_dir = data_dir.join("generations");

    let store = Arc::new(FileDocumentStore::open(&storage_dir).map_err(|err| err.to_string())?);
    let journal = Arc::new(FileChangeJournal::open(&storage_dir).map_err(|err| err.to_string())?);

    let report = sync_source_to_storage(source_dir, &store, &journal)?;

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

    let seq = journal
        .last_sequence()
        .map_err(|err| err.to_string())?
        .get();
    let expected_docs = store.document_count() as u64;
    let active_matches = manager
        .state()
        .active_generation
        .as_ref()
        .and_then(|active_id| manager.state().generations.get(active_id))
        .is_some_and(|record| {
            record.target_sequence == seq
                && record
                    .manifest
                    .as_ref()
                    .is_some_and(|manifest| manifest.doc_count == expected_docs)
        });

    if !active_matches {
        let gen_id = allocate_generation_id(&manager, seq);
        manager
            .build_generation(&gen_id, seq)
            .map_err(|err| err.to_string())?;
        manager
            .prepare_generation(&gen_id, seq)
            .map_err(|err| err.to_string())?;
        manager.activate(&gen_id).map_err(|err| err.to_string())?;
    }
    manager.gc_retired().map_err(|err| err.to_string())?;

    let access = manager.reader_access();
    let search_port = Arc::new(ManagedSearchPort {
        access,
        _manager: Mutex::new(manager),
    });
    Ok((SearchService::new(search_port, store), report))
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

    Ok(build_report)
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
}
