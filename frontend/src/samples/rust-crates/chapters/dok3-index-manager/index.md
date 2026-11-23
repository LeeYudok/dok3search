# Crate dok3-index-manager

## Overview

Blue-green index generation control. P1 persists the pointer and manifests
in a single file transaction. Backends supply the journal, index and reader.

## Items

- `Manifest` — The backend publishes this manifest with each durable index commit.
- `ReaderAccess` — Cloneable request entry point. An outstanding lease retains the writer lock even when the manager closes, so reopening cannot bypass it.
- `ReaderLease` — Owns the reader and protects the generation files for the request lifetime.
- `GenerationBackend` — Index and journal adapter. All snapshot consumers must hold a ReaderLease for the duration of their request to participate in retired-file protection.

### constant dok3-index-manager::REQUIRED_CHECKS

```rust
pub const REQUIRED_CHECKS: [ValidationCheck; 8];
```

### enum dok3-index-manager::Error

```rust
pub enum Error {
    Io(Error),
    Json(Error),
    WriterLocked,
    Invalid(String),
    Backend(String),
    InsufficientDisk {
        available: u64,
        required: u64,
    },
    JournalCutChanged {
        expected: u64,
        actual: u64,
    },
    ReconcileRequired,
}
```

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(error: Error) -> Self`
- `fn from(error: Error) -> Self`

### enum dok3-index-manager::GenerationFiles

```rust
pub enum GenerationFiles {
    Present,
    Deleting,
    Deleted,
}
```

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### enum dok3-index-manager::GenerationState

```rust
pub enum GenerationState {
    Empty,
    Building,
    CatchingUp,
    Validating,
    Ready,
    Activating,
    Active,
    Retired,
    Failed,
    Recovering,
}
```

**Methods**

- `pub fn transition(self, next: Self) -> Result<Self>`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### enum dok3-index-manager::IntentStage

```rust
pub enum IntentStage {
    Prepared,
    RuntimeSwapped,
}
```

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### enum dok3-index-manager::ValidationCheck

```rust
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
```

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-index-manager::ActivationIntent

```rust
pub struct ActivationIntent {
    pub from_generation: Option<String>,
    pub to_generation: String,
    pub from_epoch: u64,
    pub target_epoch: u64,
    pub target_sequence: u64,
    pub intent_stage: IntentStage,
}
```

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-index-manager::DurableState

```rust
pub struct DurableState {
    pub active_generation: Option<String>,
    pub active_epoch: u64,
    pub activating_transition: Option<ActivationIntent>,
    pub generations: BTreeMap<String, GenerationRecord>,
}
```

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-index-manager::GenerationRecord

```rust
pub struct GenerationRecord {
    pub state: GenerationState,
    pub created_at_unix_secs: u64,
    pub retired_at_unix_secs: Option<u64>,
    pub manifest_checksum: Option<String>,
    pub manifest: Option<Manifest>,
    pub target_sequence: u64,
    pub validated_sequence: Option<u64>,
    pub files: GenerationFiles,
}
```

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-index-manager::IndexManager

```rust
pub struct IndexManager<B: GenerationBackend> { /* private fields */ }
```

**Methods**

- `pub fn open( root: impl AsRef<Path>, backend: B, min_free_disk_bytes: u64, ) -> Result<Self>`
- `pub fn state(&self) -> &DurableState`
- `pub fn reader_access(&self) -> ReaderAccess<B::Reader>`
- `pub fn build_generation(&mut self, generation: &str, s0: u64) -> Result<()>`
- `pub fn prepare_generation( &mut self, generation: &str, target_sequence: u64, ) -> Result<()>`
- `pub fn activate(&mut self, generation: &str) -> Result<()>`
- `pub fn rollback( &mut self, generation: &str, current_journal_seq: u64, ) -> Result<()>`
- `pub fn reconcile_on_startup(&mut self) -> Result<()>`
- `pub fn gc_retired(&mut self) -> Result<Vec<String>>`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`

### struct dok3-index-manager::Manifest

```rust
pub struct Manifest {
    pub generation_id: String,
    pub schema_version: u64,
    pub analyzer_version: u64,
    pub acl_policy_version: u64,
    pub applied_change_sequence: u64,
    pub doc_count: u64,
}
```

The backend publishes this manifest with each durable index commit.

**Methods**

- `pub fn checksum(&self) -> Result<String>`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-index-manager::ReaderAccess

```rust
pub struct ReaderAccess<R> { /* private fields */ }
```

Cloneable request entry point. An outstanding lease retains the writer
lock even when the manager closes, so reopening cannot bypass it.

**Methods**

- `pub fn acquire(&self) -> Option<ReaderLease<R>>`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`

### struct dok3-index-manager::ReaderLease

```rust
pub struct ReaderLease<R> { /* private fields */ }
```

Owns the reader and protects the generation files for the request lifetime.

**Methods**

- `fn deref(&self) -> &Self::Target`

### struct dok3-index-manager::SearchSnapshot

```rust
pub struct SearchSnapshot<R> {
    pub manifest: Manifest,
    pub epoch: u64,
    pub created_at_unix_secs: u64,
    pub reader: R,
}
```

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### trait dok3-index-manager::GenerationBackend

```rust
pub trait GenerationBackend {
    type Reader: Send + Sync + 'static;

    // Required methods
    fn build_generation(&mut self, generation: &str, s0: u64) -> Result<()>;
    fn replay_changes(
        &mut self,
        generation: &str,
        after_seq: u64,
        target_seq: u64,
    ) -> Result<()>;
    fn read_manifest(&mut self, generation: &str) -> Result<Manifest>;
    fn validate_generation(
        &mut self,
        manifest: &Manifest,
        checks: &[ValidationCheck],
    ) -> Result<()>;
    fn open_reader(&mut self, manifest: &Manifest) -> Result<Self::Reader>;
    fn swap_active_snapshot(
        &mut self,
        snapshot: Option<Arc<SearchSnapshot<Self::Reader>>>,
    ) -> Result<()>;
    fn journal_sequence(&mut self) -> Result<u64>;
    fn free_disk_bytes(&mut self) -> Result<u64>;
    fn delete_generation_files(&mut self, generation: &str) -> Result<()>;
}
```

Index and journal adapter. All snapshot consumers must hold a ReaderLease
for the duration of their request to participate in retired-file protection.

**Methods**

- `type Reader: Send + Sync + 'static`
- `fn build_generation(&mut self, generation: &str, s0: u64) -> Result<()>`
- `fn replay_changes( &mut self, generation: &str, after_seq: u64, target_seq: u64, ) -> Result<()>`
- `fn read_manifest(&mut self, generation: &str) -> Result<Manifest>`
- `fn validate_generation( &mut self, manifest: &Manifest, checks: &[ValidationCheck], ) -> Result<()>`
- `fn open_reader(&mut self, manifest: &Manifest) -> Result<Self::Reader>`
- `fn swap_active_snapshot( &mut self, snapshot: Option<Arc<SearchSnapshot<Self::Reader>>>, ) -> Result<()>`
- `fn journal_sequence(&mut self) -> Result<u64>`
- `fn free_disk_bytes(&mut self) -> Result<u64>`
- `fn delete_generation_files(&mut self, generation: &str) -> Result<()>`

### type dok3-index-manager::Result

```rust
pub type Result<T> = Result<T, Error>;
```
