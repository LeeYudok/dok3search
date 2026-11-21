use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use dok3_index_manager::{
    Error, GenerationBackend, GenerationFiles, GenerationState, IndexManager, IntentStage,
    Manifest, REQUIRED_CHECKS, Result, SearchSnapshot, ValidationCheck,
};

const MIN_DISK: u64 = 100;

#[derive(Default)]
enum Fault {
    #[default]
    None,
    BuildCrash,
    CheckpointWrite,
    DiskDropsDuringReplay,
    BeforeSwap,
    AfterSwap,
    Validation(ValidationCheck),
    AppendDuringValidation,
}

#[derive(Default)]
struct MockState {
    fault: Fault,
    manifests: BTreeMap<String, Manifest>,
    journal: u64,
    disk: u64,
    runtime: Option<(String, u64)>,
    replays: Vec<(String, u64, u64)>,
    applied: BTreeSet<(String, u64)>,
    root: PathBuf,
}

#[derive(Clone)]
struct FaultInjectingBackend(Arc<Mutex<MockState>>);

impl FaultInjectingBackend {
    fn state(&self) -> std::sync::MutexGuard<'_, MockState> {
        self.0.lock().unwrap_or_else(|error| error.into_inner())
    }
}

impl GenerationBackend for FaultInjectingBackend {
    type Reader = String;

    fn build_generation(&mut self, generation: &str, s0: u64) -> Result<()> {
        let mut state = self.state();
        if matches!(state.fault, Fault::BuildCrash) {
            panic!("process stopped during build");
        }
        state.manifests.insert(
            generation.into(),
            Manifest {
                generation_id: generation.into(),
                schema_version: 3,
                analyzer_version: 1,
                acl_policy_version: 2,
                applied_change_sequence: s0,
                doc_count: 1,
            },
        );
        Ok(())
    }

    fn replay_changes(&mut self, generation: &str, after: u64, target: u64) -> Result<()> {
        let mut state = self.state();
        state.replays.push((generation.into(), after, target));
        let mut inserted = 0;
        for sequence in after + 1..=target {
            inserted += u64::from(state.applied.insert((generation.into(), sequence)));
        }
        let manifest = state.manifests.get_mut(generation).unwrap();
        manifest.doc_count += inserted;
        manifest.applied_change_sequence = target;
        if matches!(state.fault, Fault::CheckpointWrite) {
            // The durable index commit succeeded. Make the actual state
            // checkpoint write fail before it can replace the old pointer.
            fs::create_dir(state.root.join("generation_state.json.tmp"))?;
        }
        if matches!(state.fault, Fault::DiskDropsDuringReplay) {
            state.disk = MIN_DISK - 1;
        }
        Ok(())
    }

    fn read_manifest(&mut self, generation: &str) -> Result<Manifest> {
        self.state()
            .manifests
            .get(generation)
            .cloned()
            .ok_or_else(|| Error::Backend("manifest missing".into()))
    }

    fn validate_generation(&mut self, _: &Manifest, checks: &[ValidationCheck]) -> Result<()> {
        assert_eq!(checks, REQUIRED_CHECKS);
        let mut state = self.state();
        match state.fault {
            Fault::Validation(check) => {
                assert!(checks.contains(&check));
                Err(Error::Backend(format!("{check:?} failed")))
            }
            Fault::AppendDuringValidation => {
                state.journal += 1;
                state.fault = Fault::None;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn open_reader(&mut self, manifest: &Manifest) -> Result<Self::Reader> {
        Ok(manifest.generation_id.clone())
    }

    fn swap_active_snapshot(
        &mut self,
        snapshot: Option<Arc<SearchSnapshot<Self::Reader>>>,
    ) -> Result<()> {
        let mut state = self.state();
        if matches!(state.fault, Fault::BeforeSwap) {
            panic!("process stopped before runtime swap");
        }
        state.runtime =
            snapshot.map(|snapshot| (snapshot.manifest.generation_id.clone(), snapshot.epoch));
        if matches!(state.fault, Fault::AfterSwap) {
            panic!("process stopped after runtime swap");
        }
        Ok(())
    }

    fn journal_sequence(&mut self) -> Result<u64> {
        Ok(self.state().journal)
    }

    fn free_disk_bytes(&mut self) -> Result<u64> {
        Ok(self.state().disk)
    }

    fn delete_generation_files(&mut self, generation: &str) -> Result<()> {
        self.state().manifests.remove(generation);
        Ok(())
    }
}

struct Fixture(FaultInjectingBackend);

impl Fixture {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let root = std::env::temp_dir().join(format!(
            "dok3-index-manager-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        Self(FaultInjectingBackend(Arc::new(Mutex::new(MockState {
            disk: 1000,
            journal: 5,
            root,
            ..MockState::default()
        }))))
    }

    fn open(&self) -> Result<IndexManager<FaultInjectingBackend>> {
        IndexManager::open(&self.0.state().root, self.0.clone(), MIN_DISK)
    }

    fn start(&self) -> IndexManager<FaultInjectingBackend> {
        let mut manager = self.open().unwrap();
        manager.reconcile_on_startup().unwrap();
        manager
    }

    fn with_blue(&self) -> IndexManager<FaultInjectingBackend> {
        let mut manager = self.start();
        prepare(&mut manager, "blue", 5);
        manager.activate("blue").unwrap();
        manager
    }

    fn restart(
        &self,
        manager: IndexManager<FaultInjectingBackend>,
    ) -> IndexManager<FaultInjectingBackend> {
        drop(manager);
        let mut state = self.0.state();
        state.fault = Fault::None;
        state.runtime = None;
        drop(state);
        self.start()
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0.state().root).unwrap();
    }
}

fn prepare(manager: &mut IndexManager<FaultInjectingBackend>, id: &str, sequence: u64) {
    manager.build_generation(id, 5).unwrap();
    manager.prepare_generation(id, sequence).unwrap();
}

fn assert_active(manager: &IndexManager<FaultInjectingBackend>, id: &str, epoch: u64) {
    assert_eq!(manager.state().active_generation.as_deref(), Some(id));
    assert_eq!(manager.state().active_epoch, epoch);
    let reader = manager.reader_access().acquire().unwrap();
    assert_eq!(reader.reader, id);
    assert_eq!(reader.epoch, epoch);
}

#[test]
fn crash_during_build_preserves_active() {
    let fixture = Fixture::new();
    let mut manager = fixture.with_blue();
    fixture.0.state().fault = Fault::BuildCrash;
    assert!(catch_unwind(AssertUnwindSafe(|| manager.build_generation("green", 5))).is_err());
    let manager = fixture.restart(manager);
    assert_active(&manager, "blue", 1);
    assert_eq!(
        manager.state().generations["green"].state,
        GenerationState::Failed
    );
}

#[test]
fn committed_replay_without_checkpoint_is_retried_idempotently() {
    let fixture = Fixture::new();
    let mut manager = fixture.with_blue();
    manager.build_generation("green", 5).unwrap();
    fixture.0.state().journal = 8;
    fixture.0.state().fault = Fault::CheckpointWrite;
    assert!(matches!(
        manager.prepare_generation("green", 8),
        Err(Error::Io(_))
    ));
    assert_eq!(
        manager.state().generations["green"]
            .manifest
            .as_ref()
            .unwrap()
            .applied_change_sequence,
        5
    );
    assert_eq!(
        fixture.0.state().manifests["green"].applied_change_sequence,
        8
    );
    fs::remove_dir(fixture.0.state().root.join("generation_state.json.tmp")).unwrap();
    let mut manager = fixture.restart(manager);
    assert_eq!(fixture.0.state().replays, vec![("green".into(), 5, 8); 2]);
    assert_eq!(fixture.0.state().manifests["green"].doc_count, 4);
    assert_eq!(
        manager.state().generations["green"].state,
        GenerationState::Ready
    );
    manager.activate("green").unwrap();
    assert_active(&manager, "green", 2);
}

fn crash_during_activation(
    fixture: &Fixture,
    fault: Fault,
    swapped: bool,
) -> IndexManager<FaultInjectingBackend> {
    let mut manager = fixture.with_blue();
    prepare(&mut manager, "green", 5);
    fixture.0.state().fault = fault;
    assert!(catch_unwind(AssertUnwindSafe(|| manager.activate("green"))).is_err());
    assert_eq!(
        fixture.0.state().runtime,
        if swapped {
            Some(("green".into(), 2))
        } else {
            Some(("blue".into(), 1))
        }
    );
    assert_eq!(
        manager
            .state()
            .activating_transition
            .as_ref()
            .unwrap()
            .intent_stage,
        IntentStage::Prepared
    );
    manager
}

#[test]
fn restart_after_intent_before_swap_completes_transition() {
    let fixture = Fixture::new();
    let manager = crash_during_activation(&fixture, Fault::BeforeSwap, false);
    let manager = fixture.restart(manager);
    assert_active(&manager, "green", 2);
    assert!(manager.state().activating_transition.is_none());
}

#[test]
fn restart_after_swap_before_active_completes_transition_once() {
    let fixture = Fixture::new();
    let manager = crash_during_activation(&fixture, Fault::AfterSwap, true);
    let manager = fixture.restart(manager);
    assert_active(&manager, "green", 2);
    let manager = fixture.restart(manager);
    assert_active(&manager, "green", 2);
}

#[test]
fn invalid_manifest_during_recovery_preserves_active() {
    let fixture = Fixture::new();
    let manager = crash_during_activation(&fixture, Fault::AfterSwap, true);
    fixture
        .0
        .state()
        .manifests
        .get_mut("green")
        .unwrap()
        .doc_count += 1;
    let manager = fixture.restart(manager);
    assert_active(&manager, "blue", 1);
    assert!(manager.state().activating_transition.is_none());
    assert_eq!(
        manager.state().generations["green"].state,
        GenerationState::Failed
    );
}

#[test]
fn every_validation_gate_is_required_including_acl_and_missing_documents() {
    for check in REQUIRED_CHECKS {
        let fixture = Fixture::new();
        let mut manager = fixture.with_blue();
        manager.build_generation("green", 5).unwrap();
        fixture.0.state().fault = Fault::Validation(check);
        assert!(manager.prepare_generation("green", 5).is_err());
        assert!(manager.activate("green").is_err());
        assert_active(&manager, "blue", 1);
    }
}

#[test]
fn rollback_rejects_an_outdated_cut_and_revalidates_the_latest() {
    let fixture = Fixture::new();
    let mut manager = fixture.with_blue();
    prepare(&mut manager, "green", 5);
    manager.activate("green").unwrap();
    fixture.0.state().journal = 7;
    fixture.0.state().fault = Fault::AppendDuringValidation;
    assert!(matches!(
        manager.rollback("blue", 7),
        Err(Error::JournalCutChanged {
            expected: 7,
            actual: 8
        })
    ));
    assert_active(&manager, "green", 2);
    manager.rollback("blue", 8).unwrap();
    assert_active(&manager, "blue", 3);
    assert_eq!(
        manager.state().generations["blue"].validated_sequence,
        Some(8)
    );
    assert_eq!(
        fixture.0.state().replays,
        vec![("blue".into(), 5, 7), ("blue".into(), 7, 8)]
    );
}

#[test]
fn ready_candidate_cannot_activate_after_journal_advances() {
    let fixture = Fixture::new();
    let mut manager = fixture.with_blue();
    prepare(&mut manager, "green", 5);
    fixture.0.state().journal = 6;
    assert!(matches!(
        manager.activate("green"),
        Err(Error::JournalCutChanged { .. })
    ));
    manager.prepare_generation("green", 6).unwrap();
    manager.activate("green").unwrap();
    assert_active(&manager, "green", 2);
}

#[test]
fn gc_waits_for_old_reader_and_deletes_after_drop() {
    let fixture = Fixture::new();
    let mut manager = fixture.with_blue();
    let reader_access = manager.reader_access();
    let old_reader = reader_access.acquire().unwrap();
    prepare(&mut manager, "green", 5);
    manager.activate("green").unwrap();
    assert_eq!(old_reader.reader, "blue");
    assert_eq!(reader_access.acquire().unwrap().reader, "green");
    assert!(manager.gc_retired().unwrap().is_empty());
    assert!(fixture.0.state().manifests.contains_key("blue"));
    drop(old_reader);
    assert_eq!(manager.gc_retired().unwrap(), ["blue"]);
    assert_eq!(
        manager.state().generations["blue"].files,
        GenerationFiles::Deleted
    );
    assert!(!fixture.0.state().manifests.contains_key("blue"));
    assert!(manager.rollback("blue", 5).is_err());
}

#[test]
fn low_disk_stops_build_and_validation_while_search_stays_active() {
    let fixture = Fixture::new();
    let mut manager = fixture.with_blue();
    manager.build_generation("green", 5).unwrap();
    fixture.0.state().disk = MIN_DISK - 1;
    assert!(matches!(
        manager.build_generation("red", 5),
        Err(Error::InsufficientDisk { .. })
    ));
    assert!(!manager.state().generations.contains_key("red"));
    assert!(matches!(
        manager.prepare_generation("green", 5),
        Err(Error::InsufficientDisk { .. })
    ));
    assert_active(&manager, "blue", 1);

    {
        let mut state = fixture.0.state();
        state.disk = MIN_DISK;
        state.journal = 6;
        state.fault = Fault::DiskDropsDuringReplay;
    }
    assert!(matches!(
        manager.prepare_generation("green", 6),
        Err(Error::InsufficientDisk { .. })
    ));
    assert_eq!(
        manager.state().generations["green"].state,
        GenerationState::Failed
    );
    assert_active(&manager, "blue", 1);
}

#[test]
fn lock_and_file_checksum_reject_concurrent_writers_and_tampering() {
    let fixture = Fixture::new();
    let manager = fixture.with_blue();
    assert!(matches!(fixture.open(), Err(Error::WriterLocked)));
    let lease = manager.reader_access().acquire().unwrap();
    drop(manager);
    assert!(matches!(fixture.open(), Err(Error::WriterLocked)));
    drop(lease);
    let file = fixture.0.state().root.join("generation_state.json");
    let mut state: serde_json::Value = serde_json::from_slice(&fs::read(&file).unwrap()).unwrap();
    state["active_epoch"] = 42.into();
    fs::write(file, serde_json::to_vec(&state).unwrap()).unwrap();
    assert!(matches!(fixture.open(), Err(Error::Invalid(_))));
}
