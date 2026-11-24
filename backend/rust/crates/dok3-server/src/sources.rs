//! Durable local source registry (`sources.json`) backing `/api/v1/sources*`.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use dok3_connectors::{
    DEFAULT_SQLITE_BATCH_LIMIT, LocalFileConnector, SourceConfig, SqliteColumnMapping,
    SqliteCursor, SqliteCursorMode, SqliteSourceAdapter, SqliteSourceConfig,
};
use dok3_core::Dok3Error;
use dok3_domain::{
    CreateSourceRequest, RegisteredSource, SourceId, SourceKind, SqliteCursorKind,
    validate_classification_category, validate_source_id_slug,
};
use serde::{Deserialize, Serialize};

use crate::persistence::{atomic_write_bytes, now_ms};

pub(crate) const ENV_SOURCE_ID: &str = "local-markdown";
const ENV_SOURCE_LABEL: &str = "Local Markdown";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct SourcesDisk {
    sources: Vec<RegisteredSource>,
    #[serde(default)]
    pending_deletes: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum StoredCursor {
    Integer { value: i64 },
    UpdatedAt { value: String },
}

impl From<&SqliteCursor> for StoredCursor {
    fn from(cursor: &SqliteCursor) -> Self {
        match cursor {
            SqliteCursor::Integer(value) => Self::Integer { value: *value },
            SqliteCursor::UpdatedAt { updated_at } => Self::UpdatedAt {
                value: updated_at.clone(),
            },
        }
    }
}

impl From<&StoredCursor> for SqliteCursor {
    fn from(cursor: &StoredCursor) -> Self {
        match cursor {
            StoredCursor::Integer { value } => Self::Integer(*value),
            StoredCursor::UpdatedAt { value } => Self::UpdatedAt {
                updated_at: value.clone(),
            },
        }
    }
}

/// Per-source SQLite cursors persisted at `storage/source_cursors.json`.
pub(crate) struct SourceCursorStore {
    path: PathBuf,
    cursors: BTreeMap<String, StoredCursor>,
}

impl SourceCursorStore {
    fn open(storage_dir: &Path) -> Result<Self, String> {
        let path = storage_dir.join("source_cursors.json");
        let cursors = match fs::read(&path) {
            Ok(raw) => serde_json::from_slice(&raw)
                .map_err(|err| format!("failed to decode {}: {err}", path.display()))?,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => BTreeMap::new(),
            Err(err) => return Err(format!("failed to read {}: {err}", path.display())),
        };
        Ok(Self { path, cursors })
    }

    pub(crate) fn get(&self, source_id: &str) -> Option<SqliteCursor> {
        self.cursors.get(source_id).map(SqliteCursor::from)
    }

    pub(crate) fn set(&mut self, source_id: &str, cursor: &SqliteCursor) -> Result<(), String> {
        let stored = StoredCursor::from(cursor);
        if self.cursors.get(source_id) == Some(&stored) {
            return Ok(());
        }
        let mut next = self.cursors.clone();
        next.insert(source_id.to_string(), stored);
        self.persist(&next)?;
        self.cursors = next;
        Ok(())
    }

    fn remove(&mut self, source_id: &str) -> Result<(), String> {
        if !self.cursors.contains_key(source_id) {
            return Ok(());
        }
        let mut next = self.cursors.clone();
        next.remove(source_id);
        self.persist(&next)?;
        self.cursors = next;
        Ok(())
    }

    fn persist(&self, cursors: &BTreeMap<String, StoredCursor>) -> Result<(), String> {
        let bytes = serde_json::to_vec_pretty(cursors)
            .map_err(|err| format!("failed to serialize source cursors: {err}"))?;
        atomic_write_bytes(&self.path, &bytes)
    }
}

/// Persistent registry of local Markdown directories and SQLite databases indexed by
/// [`crate::persistence::PersistentRuntime`].
pub(crate) struct SourceRegistry {
    path: PathBuf,
    cursors: SourceCursorStore,
    sources: Vec<RegisteredSource>,
    pending_deletes: BTreeSet<String>,
}

impl SourceRegistry {
    /// Opens `data_dir/storage/sources.json` and ensures the env-configured `local-markdown`
    /// source is tracked as `managed_by_env: true`.
    pub(crate) fn open(storage_dir: &Path, env_source_dir: &Path) -> Result<Self, String> {
        let path = storage_dir.join("sources.json");
        let (mut disk, existed) = match fs::read(&path) {
            Ok(raw) => (
                serde_json::from_slice::<SourcesDisk>(&raw)
                    .map_err(|err| format!("failed to decode {}: {err}", path.display()))?,
                true,
            ),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                (SourcesDisk::default(), false)
            }
            Err(err) => return Err(format!("failed to read {}: {err}", path.display())),
        };

        let env_path_str = canonical_or_raw(env_source_dir);
        let mut dirty = !existed;
        disk.pending_deletes.remove(ENV_SOURCE_ID);
        if let Some(existing) = disk
            .sources
            .iter_mut()
            .find(|src| src.source_id == ENV_SOURCE_ID)
        {
            if existing.path != env_path_str {
                existing.path = env_path_str;
                dirty = true;
            }
            if existing.label != ENV_SOURCE_LABEL {
                existing.label = ENV_SOURCE_LABEL.to_string();
                dirty = true;
            }
            if !existing.managed_by_env {
                existing.managed_by_env = true;
                dirty = true;
            }
            if existing.default_category.trim().is_empty() {
                existing.default_category = "guide".to_string();
                dirty = true;
            }
            if existing.default_tag.trim().is_empty() {
                existing.default_tag = "docs".to_string();
                dirty = true;
            }
        } else {
            disk.sources.insert(
                0,
                RegisteredSource {
                    source_id: ENV_SOURCE_ID.to_string(),
                    label: ENV_SOURCE_LABEL.to_string(),
                    path: env_path_str,
                    default_category: "guide".to_string(),
                    kind: dok3_domain::SourceKind::LocalDir,
                    sqlite: None,
                    default_tag: "docs".to_string(),
                    managed_by_env: true,
                    created_at_ms: now_ms(),
                    last_synced_at_ms: None,
                    document_count: 0,
                    last_error: None,
                },
            );
            dirty = true;
        }

        if dirty {
            persist_state(&path, &disk.sources, &disk.pending_deletes)?;
        }

        let cursors = SourceCursorStore::open(storage_dir)?;
        Ok(Self {
            path,
            cursors,
            sources: disk.sources,
            pending_deletes: disk.pending_deletes,
        })
    }

    pub(crate) fn cursors_mut(&mut self) -> &mut SourceCursorStore {
        &mut self.cursors
    }

    pub(crate) fn list(&self) -> Vec<RegisteredSource> {
        self.sources.clone()
    }

    pub(crate) fn active_sources(&self) -> Vec<RegisteredSource> {
        self.sources
            .iter()
            .filter(|src| !self.pending_deletes.contains(&src.source_id))
            .cloned()
            .collect()
    }

    pub(crate) fn get_active(&self, source_id: &str) -> Option<RegisteredSource> {
        if self.pending_deletes.contains(source_id) {
            return None;
        }
        self.sources
            .iter()
            .find(|src| src.source_id == source_id)
            .cloned()
    }

    pub(crate) fn contains_active(&self, source_id: &str) -> bool {
        !self.pending_deletes.contains(source_id)
            && self.sources.iter().any(|src| src.source_id == source_id)
    }

    pub(crate) fn defaults_for(&self, source_id: &str) -> (String, String) {
        self.sources
            .iter()
            .find(|src| src.source_id == source_id)
            .map(|src| {
                let category = if src.default_category.trim().is_empty() {
                    "guide".to_string()
                } else {
                    src.default_category.clone()
                };
                let tag = if src.default_tag.trim().is_empty() {
                    "docs".to_string()
                } else {
                    src.default_tag.clone()
                };
                (category, tag)
            })
            .unwrap_or_else(|| ("guide".to_string(), "docs".to_string()))
    }

    /// Validates a new source registration request (slug, label, non-symlink existing directory,
    /// deduplicated canonical path, category, and dry scan via `LocalFileConnector`).
    pub(crate) fn validate_new_source(
        &self,
        req: &CreateSourceRequest,
    ) -> Result<RegisteredSource, Dok3Error> {
        let source_id = validate_source_id_slug(&req.source_id)?;
        let label = req.label.trim();
        if label.is_empty() || label.chars().count() > 120 {
            return Err(Dok3Error::InvalidInput(
                "source label must be 1..=120 non-whitespace characters".to_string(),
            ));
        }
        if self.sources.iter().any(|src| src.source_id == source_id) {
            return Err(Dok3Error::InvalidInput(format!(
                "source '{source_id}' already exists"
            )));
        }

        let default_category = match req.default_category.as_deref() {
            Some(raw) if !raw.trim().is_empty() => validate_classification_category(raw)?,
            _ => "guide".to_string(),
        };
        let fallback_tag = match req.kind {
            SourceKind::LocalDir => "docs",
            SourceKind::Sqlite => "sqlite",
        };
        let default_tag = match req.default_tag.as_deref() {
            Some(raw) if !raw.trim().is_empty() => {
                let trimmed = raw.trim();
                if trimmed.chars().count() > 64 {
                    return Err(Dok3Error::InvalidInput(
                        "default_tag exceeds 64 characters".to_string(),
                    ));
                }
                trimmed.to_string()
            }
            _ => fallback_tag.to_string(),
        };

        let raw_path = req.path.trim();
        if raw_path.is_empty() {
            return Err(Dok3Error::InvalidInput(
                "source path must not be empty".to_string(),
            ));
        }
        let candidate = Path::new(raw_path);

        if req.kind == SourceKind::Sqlite {
            return self.validate_new_sqlite_source(
                req,
                source_id,
                label,
                candidate,
                default_category,
                default_tag,
            );
        }
        if req.sqlite.is_some() {
            return Err(Dok3Error::InvalidInput(
                "sqlite options are only allowed for kind 'sqlite'".to_string(),
            ));
        }
        let normalized = validate_real_directory(candidate)?;
        let canonical = fs::canonicalize(&normalized).map_err(|err| {
            Dok3Error::InvalidInput(format!("failed to canonicalize path '{raw_path}': {err}"))
        })?;
        for existing in &self.sources {
            let existing_canon =
                fs::canonicalize(&existing.path).unwrap_or_else(|_| PathBuf::from(&existing.path));
            if existing_canon == canonical {
                return Err(Dok3Error::InvalidInput(format!(
                    "source directory '{}' is already registered as '{}'",
                    canonical.display(),
                    existing.source_id
                )));
            }
        }

        let mut cfg = SourceConfig::supported_document_defaults(
            SourceId::new(source_id.clone()),
            label,
            &canonical,
        );
        cfg.category = default_category.clone();
        cfg.default_tag = default_tag.clone();
        let connector = LocalFileConnector::new(cfg)?;
        let _ = connector.scan()?;

        Ok(RegisteredSource {
            source_id,
            label: label.to_string(),
            path: canonical.to_string_lossy().into_owned(),
            kind: SourceKind::LocalDir,
            sqlite: None,
            default_category,
            default_tag,
            managed_by_env: false,
            created_at_ms: now_ms(),
            last_synced_at_ms: None,
            document_count: 0,
            last_error: None,
        })
    }

    fn validate_new_sqlite_source(
        &self,
        req: &CreateSourceRequest,
        source_id: String,
        label: &str,
        candidate: &Path,
        default_category: String,
        default_tag: String,
    ) -> Result<RegisteredSource, Dok3Error> {
        let options = req.sqlite.clone().ok_or_else(|| {
            Dok3Error::InvalidInput("sqlite options are required for kind 'sqlite'".to_string())
        })?;
        let normalized = validate_real_sqlite_file(candidate)?;
        let canonical = fs::canonicalize(&normalized).map_err(|err| {
            Dok3Error::InvalidInput(format!(
                "failed to canonicalize path '{}': {err}",
                candidate.display()
            ))
        })?;
        let canonical_str = canonical.to_string_lossy().into_owned();
        for existing in &self.sources {
            if existing.kind != SourceKind::Sqlite {
                continue;
            }
            let same_query = existing
                .sqlite
                .as_ref()
                .is_some_and(|opts| opts.select_query.trim() == options.select_query.trim());
            let existing_canon =
                fs::canonicalize(&existing.path).unwrap_or_else(|_| PathBuf::from(&existing.path));
            if same_query && existing_canon == canonical {
                return Err(Dok3Error::InvalidInput(format!(
                    "sqlite source '{}' with the same query is already registered as '{}'",
                    canonical.display(),
                    existing.source_id
                )));
            }
        }

        let entry = RegisteredSource {
            source_id,
            label: label.to_string(),
            path: canonical_str,
            kind: SourceKind::Sqlite,
            sqlite: Some(options),
            default_category,
            default_tag,
            managed_by_env: false,
            created_at_ms: now_ms(),
            last_synced_at_ms: None,
            document_count: 0,
            last_error: None,
        };
        let probe = sqlite_adapter_for(&entry, Some(1))?;
        probe.collect(None)?;
        Ok(entry)
    }

    pub(crate) fn insert_source(&mut self, source: RegisteredSource) -> Result<(), String> {
        let mut next = self.sources.clone();
        next.push(source);
        persist_state(&self.path, &next, &self.pending_deletes)?;
        self.sources = next;
        Ok(())
    }

    pub(crate) fn mark_pending_delete(
        &mut self,
        source_id: &str,
    ) -> Result<RegisteredSource, Dok3Error> {
        let trimmed = source_id.trim();
        if trimmed.is_empty() {
            return Err(Dok3Error::InvalidInput(
                "source_id must not be empty".to_string(),
            ));
        }
        let Some(existing) = self.sources.iter().find(|src| src.source_id == trimmed) else {
            return Err(Dok3Error::NotFound(format!("source '{trimmed}' not found")));
        };
        if existing.managed_by_env {
            return Err(Dok3Error::InvalidInput(format!(
                "cannot delete environment-managed source '{trimmed}'"
            )));
        }
        let entry = existing.clone();
        if !self.pending_deletes.contains(trimmed) {
            let mut next_pending = self.pending_deletes.clone();
            next_pending.insert(trimmed.to_string());
            persist_state(&self.path, &self.sources, &next_pending).map_err(Dok3Error::Storage)?;
            self.pending_deletes = next_pending;
        }
        Ok(entry)
    }

    pub(crate) fn finalize_delete(&mut self, source_id: &str) -> Result<(), String> {
        let mut next_sources = self.sources.clone();
        let before_len = next_sources.len();
        next_sources.retain(|src| src.source_id != source_id);
        let mut next_pending = self.pending_deletes.clone();
        let removed_pending = next_pending.remove(source_id);
        if next_sources.len() == before_len && !removed_pending {
            return Ok(());
        }
        persist_state(&self.path, &next_sources, &next_pending)?;
        self.sources = next_sources;
        self.pending_deletes = next_pending;
        self.cursors.remove(source_id)?;
        Ok(())
    }

    pub(crate) fn finalize_all_pending_deletes(&mut self) -> Result<(), String> {
        if self.pending_deletes.is_empty() {
            return Ok(());
        }
        let mut next_sources = self.sources.clone();
        next_sources.retain(|src| !self.pending_deletes.contains(&src.source_id));
        let next_pending = BTreeSet::new();
        persist_state(&self.path, &next_sources, &next_pending)?;
        let removed: Vec<String> = self.pending_deletes.iter().cloned().collect();
        self.sources = next_sources;
        self.pending_deletes = next_pending;
        for id in removed {
            self.cursors.remove(&id)?;
        }
        Ok(())
    }

    pub(crate) fn update_sync_results(
        &mut self,
        scan_outcomes: &BTreeMap<String, Result<usize, String>>,
        doc_counts_by_source: &BTreeMap<String, usize>,
        synced_at_ms: u64,
    ) -> Result<(), String> {
        let mut next = self.sources.clone();
        for src in &mut next {
            src.document_count = doc_counts_by_source
                .get(&src.source_id)
                .copied()
                .unwrap_or(0);
            if let Some(outcome) = scan_outcomes.get(&src.source_id) {
                match outcome {
                    Ok(_) => {
                        src.last_synced_at_ms = Some(synced_at_ms);
                        src.last_error = None;
                    }
                    Err(err) => {
                        src.last_error = Some(err.clone());
                    }
                }
            }
        }
        if next != self.sources {
            persist_state(&self.path, &next, &self.pending_deletes)?;
            self.sources = next;
        }
        Ok(())
    }
}

pub(crate) fn validate_real_directory(path: &Path) -> Result<PathBuf, Dok3Error> {
    let mut normalized = PathBuf::new();
    for comp in path.components() {
        match comp {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                return Err(Dok3Error::InvalidInput(format!(
                    "source path '{}' must not contain '..' components",
                    path.display()
                )));
            }
            std::path::Component::Normal(part) => {
                normalized.push(part);
                let is_macos_root_link = cfg!(target_os = "macos")
                    && matches!(normalized.to_str(), Some("/tmp" | "/var" | "/etc"));
                if !is_macos_root_link {
                    let meta = fs::symlink_metadata(&normalized).map_err(|err| {
                        Dok3Error::InvalidInput(format!(
                            "source directory '{}' is inaccessible: {err}",
                            path.display()
                        ))
                    })?;
                    if meta.file_type().is_symlink() {
                        return Err(Dok3Error::InvalidInput(format!(
                            "source directory '{}' must not be or traverse a symbolic link",
                            path.display()
                        )));
                    }
                }
            }
            other => normalized.push(other),
        }
    }
    if normalized.as_os_str().is_empty() {
        normalized.push(".");
    }
    let meta = fs::symlink_metadata(&normalized).map_err(|err| {
        Dok3Error::InvalidInput(format!(
            "source directory '{}' is inaccessible: {err}",
            path.display()
        ))
    })?;
    if meta.file_type().is_symlink() {
        return Err(Dok3Error::InvalidInput(format!(
            "source directory '{}' must not be or traverse a symbolic link",
            path.display()
        )));
    }
    if !meta.is_dir() {
        return Err(Dok3Error::InvalidInput(format!(
            "source path '{}' is not a directory",
            path.display()
        )));
    }
    Ok(normalized)
}

pub(crate) fn validate_real_sqlite_file(path: &Path) -> Result<PathBuf, Dok3Error> {
    let mut normalized = PathBuf::new();
    for comp in path.components() {
        match comp {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                return Err(Dok3Error::InvalidInput(format!(
                    "sqlite path '{}' must not contain '..' components",
                    path.display()
                )));
            }
            std::path::Component::Normal(part) => {
                normalized.push(part);
                let is_macos_root_link = cfg!(target_os = "macos")
                    && matches!(normalized.to_str(), Some("/tmp" | "/var" | "/etc"));
                if !is_macos_root_link {
                    let meta = fs::symlink_metadata(&normalized).map_err(|err| {
                        Dok3Error::InvalidInput(format!(
                            "sqlite file '{}' is inaccessible: {err}",
                            path.display()
                        ))
                    })?;
                    if meta.file_type().is_symlink() {
                        return Err(Dok3Error::InvalidInput(format!(
                            "sqlite file '{}' must not be or traverse a symbolic link",
                            path.display()
                        )));
                    }
                }
            }
            other => normalized.push(other),
        }
    }
    let meta = fs::symlink_metadata(&normalized).map_err(|err| {
        Dok3Error::InvalidInput(format!(
            "sqlite file '{}' is inaccessible: {err}",
            path.display()
        ))
    })?;
    if !meta.is_file() {
        return Err(Dok3Error::InvalidInput(format!(
            "sqlite path '{}' is not a regular file",
            path.display()
        )));
    }
    Ok(normalized)
}

/// Builds the SQLite adapter for a registered `Sqlite` source (optionally overriding batch size).
pub(crate) fn sqlite_adapter_for(
    entry: &RegisteredSource,
    batch_limit_override: Option<usize>,
) -> Result<SqliteSourceAdapter, Dok3Error> {
    let options = entry.sqlite.as_ref().ok_or_else(|| {
        Dok3Error::InvalidInput(format!(
            "source '{}' is missing sqlite options",
            entry.source_id
        ))
    })?;
    let columns = SqliteColumnMapping {
        document_id: options.id_column.clone(),
        title: options.title_column.clone(),
        body: options.body_column.clone(),
        summary: options
            .summary_column
            .clone()
            .filter(|col| !col.trim().is_empty()),
    };
    let cursor = match options.cursor_mode {
        SqliteCursorKind::IntegerPk => SqliteCursorMode::IntegerPk {
            column: options.cursor_column.clone(),
        },
        SqliteCursorKind::UpdatedAt => SqliteCursorMode::UpdatedAt {
            column: options.cursor_column.clone(),
        },
    };
    let mut cfg = SqliteSourceConfig::new(
        SourceId::new(entry.source_id.clone()),
        entry.label.clone(),
        PathBuf::from(&entry.path),
        options.select_query.clone(),
        columns,
        cursor,
    );
    cfg.category = entry.default_category.clone();
    cfg.default_tag = entry.default_tag.clone();
    cfg.batch_limit = batch_limit_override
        .unwrap_or_else(|| options.batch_limit.unwrap_or(DEFAULT_SQLITE_BATCH_LIMIT));
    SqliteSourceAdapter::new(cfg)
}

pub(crate) fn source_config_for(entry: &RegisteredSource) -> SourceConfig {
    let mut cfg = SourceConfig::supported_document_defaults(
        SourceId::new(entry.source_id.clone()),
        entry.label.clone(),
        PathBuf::from(&entry.path),
    );
    cfg.category = entry.default_category.clone();
    cfg.default_tag = entry.default_tag.clone();
    cfg
}

fn canonical_or_raw(path: &Path) -> String {
    fs::canonicalize(path)
        .unwrap_or_else(|_| path.to_path_buf())
        .to_string_lossy()
        .into_owned()
}

fn persist_state(
    path: &Path,
    sources: &[RegisteredSource],
    pending_deletes: &BTreeSet<String>,
) -> Result<(), String> {
    let disk = SourcesDisk {
        sources: sources.to_vec(),
        pending_deletes: pending_deletes.clone(),
    };
    let bytes = serde_json::to_vec_pretty(&disk)
        .map_err(|err| format!("failed to serialize sources registry: {err}"))?;
    atomic_write_bytes(path, &bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opens_env_source_validates_new_sources_and_protects_env_from_delete() {
        let env_dir = tempfile::tempdir().unwrap();
        let data_dir = tempfile::tempdir().unwrap();
        let extra_dir = tempfile::tempdir().unwrap();
        fs::write(extra_dir.path().join("note.md"), "# Note\n\nhello\n").unwrap();

        let mut reg = SourceRegistry::open(data_dir.path(), env_dir.path()).unwrap();
        assert_eq!(reg.list().len(), 1);
        assert_eq!(reg.list()[0].source_id, ENV_SOURCE_ID);
        assert!(reg.list()[0].managed_by_env);

        assert!(matches!(
            reg.mark_pending_delete(ENV_SOURCE_ID),
            Err(Dok3Error::InvalidInput(_))
        ));

        // Reject duplicate canonical path matching env_dir.
        assert!(matches!(
            reg.validate_new_source(&CreateSourceRequest {
                source_id: "dup-env".to_string(),
                label: "Dup".to_string(),
                path: env_dir.path().to_string_lossy().into_owned(),
                default_category: None,
                default_tag: None,
                kind: dok3_domain::SourceKind::LocalDir,
                sqlite: None,
            }),
            Err(Dok3Error::InvalidInput(_))
        ));

        // Reject symlink directory (including trailing `/` and trailing `/.`) plus `..` components.
        assert!(matches!(
            reg.validate_new_source(&CreateSourceRequest {
                source_id: "parent-rel".to_string(),
                label: "Parent".to_string(),
                path: format!(
                    "{}/../{}",
                    extra_dir.path().display(),
                    extra_dir.path().file_name().unwrap().to_string_lossy()
                ),
                default_category: None,
                default_tag: None,
                kind: dok3_domain::SourceKind::LocalDir,
                sqlite: None,
            }),
            Err(Dok3Error::InvalidInput(_))
        ));
        #[cfg(unix)]
        {
            fs::create_dir_all(extra_dir.path().join("sub")).unwrap();
            let link_parent = tempfile::tempdir().unwrap();
            let link_path = link_parent.path().join("sym");
            std::os::unix::fs::symlink(extra_dir.path(), &link_path).unwrap();
            for candidate in [
                link_path.to_string_lossy().into_owned(),
                format!("{}/", link_path.display()),
                format!("{}/.", link_path.display()),
                format!("{}/sub", link_path.display()),
            ] {
                assert!(
                    matches!(
                        reg.validate_new_source(&CreateSourceRequest {
                            source_id: "sym-src".to_string(),
                            label: "Sym".to_string(),
                            path: candidate.clone(),
                            default_category: None,
                            default_tag: None,
                            kind: dok3_domain::SourceKind::LocalDir,
                            sqlite: None,
                        }),
                        Err(Dok3Error::InvalidInput(_))
                    ),
                    "expected symlink path '{candidate}' to be rejected"
                );
            }
        }

        let validated = reg
            .validate_new_source(&CreateSourceRequest {
                source_id: "team-notes".to_string(),
                label: "Team Notes".to_string(),
                path: extra_dir.path().to_string_lossy().into_owned(),
                default_category: Some("manual".to_string()),
                default_tag: Some("ops".to_string()),
                kind: dok3_domain::SourceKind::LocalDir,
                sqlite: None,
            })
            .unwrap();
        reg.insert_source(validated).unwrap();
        assert_eq!(
            reg.defaults_for("team-notes"),
            ("manual".to_string(), "ops".to_string())
        );

        drop(reg);
        let mut reopened = SourceRegistry::open(data_dir.path(), env_dir.path()).unwrap();
        assert_eq!(reopened.list().len(), 2);
        let marked = reopened.mark_pending_delete("team-notes").unwrap();
        assert_eq!(marked.source_id, "team-notes");
        assert!(!reopened.contains_active("team-notes"));
        reopened.finalize_delete("team-notes").unwrap();
        assert_eq!(reopened.list().len(), 1);
        assert!(extra_dir.path().join("note.md").is_file());
    }
}
