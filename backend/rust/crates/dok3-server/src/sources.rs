//! Durable local source registry (`sources.json`) backing `/api/v1/sources*`.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use dok3_connectors::{LocalFileConnector, SourceConfig};
use dok3_core::Dok3Error;
use dok3_domain::{
    CreateSourceRequest, RegisteredSource, SourceId, validate_classification_category,
    validate_source_id_slug,
};
use serde::{Deserialize, Serialize};

use crate::persistence::{atomic_write_bytes, now_ms};

pub(crate) const ENV_SOURCE_ID: &str = "local-markdown";
const ENV_SOURCE_LABEL: &str = "Local Markdown";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct SourcesDisk {
    sources: Vec<RegisteredSource>,
}

/// Persistent registry of local Markdown directories indexed by [`crate::persistence::PersistentRuntime`].
pub(crate) struct SourceRegistry {
    path: PathBuf,
    sources: Vec<RegisteredSource>,
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
            persist_sources(&path, &disk.sources)?;
        }

        Ok(Self {
            path,
            sources: disk.sources,
        })
    }

    pub(crate) fn list(&self) -> Vec<RegisteredSource> {
        self.sources.clone()
    }

    pub(crate) fn get(&self, source_id: &str) -> Option<RegisteredSource> {
        self.sources
            .iter()
            .find(|src| src.source_id == source_id)
            .cloned()
    }

    pub(crate) fn contains(&self, source_id: &str) -> bool {
        self.sources.iter().any(|src| src.source_id == source_id)
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
        if self.contains(&source_id) {
            return Err(Dok3Error::InvalidInput(format!(
                "source '{source_id}' already exists"
            )));
        }

        let default_category = match req.default_category.as_deref() {
            Some(raw) if !raw.trim().is_empty() => validate_classification_category(raw)?,
            _ => "guide".to_string(),
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
            _ => "docs".to_string(),
        };

        let raw_path = req.path.trim();
        if raw_path.is_empty() {
            return Err(Dok3Error::InvalidInput(
                "source path must not be empty".to_string(),
            ));
        }
        let candidate = Path::new(raw_path);
        validate_real_directory(candidate)?;
        let canonical = fs::canonicalize(candidate).map_err(|err| {
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

        let mut cfg =
            SourceConfig::markdown_defaults(SourceId::new(source_id.clone()), label, &canonical);
        cfg.category = default_category.clone();
        cfg.default_tag = default_tag.clone();
        let connector = LocalFileConnector::new(cfg)?;
        let _ = connector.scan()?;

        Ok(RegisteredSource {
            source_id,
            label: label.to_string(),
            path: canonical.to_string_lossy().into_owned(),
            default_category,
            default_tag,
            managed_by_env: false,
            created_at_ms: now_ms(),
            last_synced_at_ms: None,
            document_count: 0,
            last_error: None,
        })
    }

    pub(crate) fn insert_source(&mut self, source: RegisteredSource) -> Result<(), String> {
        let mut next = self.sources.clone();
        next.push(source);
        persist_sources(&self.path, &next)?;
        self.sources = next;
        Ok(())
    }

    pub(crate) fn remove_source(&mut self, source_id: &str) -> Result<RegisteredSource, Dok3Error> {
        let trimmed = source_id.trim();
        if trimmed.is_empty() {
            return Err(Dok3Error::InvalidInput(
                "source_id must not be empty".to_string(),
            ));
        }
        let Some(pos) = self.sources.iter().position(|src| src.source_id == trimmed) else {
            return Err(Dok3Error::NotFound(format!("source '{trimmed}' not found")));
        };
        if self.sources[pos].managed_by_env {
            return Err(Dok3Error::InvalidInput(format!(
                "cannot delete environment-managed source '{trimmed}'"
            )));
        }
        let mut next = self.sources.clone();
        let removed = next.remove(pos);
        persist_sources(&self.path, &next).map_err(Dok3Error::Storage)?;
        self.sources = next;
        Ok(removed)
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
            persist_sources(&self.path, &next)?;
            self.sources = next;
        }
        Ok(())
    }
}

pub(crate) fn validate_real_directory(path: &Path) -> Result<(), Dok3Error> {
    let meta = fs::symlink_metadata(path).map_err(|err| {
        Dok3Error::InvalidInput(format!(
            "source directory '{}' is inaccessible: {err}",
            path.display()
        ))
    })?;
    if meta.file_type().is_symlink() {
        return Err(Dok3Error::InvalidInput(format!(
            "source directory '{}' must not be a symbolic link",
            path.display()
        )));
    }
    if !meta.is_dir() {
        return Err(Dok3Error::InvalidInput(format!(
            "source path '{}' is not a directory",
            path.display()
        )));
    }
    Ok(())
}

pub(crate) fn source_config_for(entry: &RegisteredSource) -> SourceConfig {
    let mut cfg = SourceConfig::markdown_defaults(
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

fn persist_sources(path: &Path, sources: &[RegisteredSource]) -> Result<(), String> {
    let disk = SourcesDisk {
        sources: sources.to_vec(),
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
            reg.remove_source(ENV_SOURCE_ID),
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
            }),
            Err(Dok3Error::InvalidInput(_))
        ));

        // Reject symlink directory.
        #[cfg(unix)]
        {
            let link_parent = tempfile::tempdir().unwrap();
            let link_path = link_parent.path().join("sym");
            std::os::unix::fs::symlink(extra_dir.path(), &link_path).unwrap();
            assert!(matches!(
                reg.validate_new_source(&CreateSourceRequest {
                    source_id: "sym-src".to_string(),
                    label: "Sym".to_string(),
                    path: link_path.to_string_lossy().into_owned(),
                    default_category: None,
                    default_tag: None,
                }),
                Err(Dok3Error::InvalidInput(_))
            ));
        }

        let validated = reg
            .validate_new_source(&CreateSourceRequest {
                source_id: "team-notes".to_string(),
                label: "Team Notes".to_string(),
                path: extra_dir.path().to_string_lossy().into_owned(),
                default_category: Some("manual".to_string()),
                default_tag: Some("ops".to_string()),
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
        let removed = reopened.remove_source("team-notes").unwrap();
        assert_eq!(removed.source_id, "team-notes");
        assert!(extra_dir.path().join("note.md").is_file());
    }
}
