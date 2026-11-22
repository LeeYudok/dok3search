//! Deterministic local filesystem Markdown connector.

use std::fs::{self, File};
use std::io::Read;
use std::path::{Component, Path, PathBuf};

use dok3_core::Dok3Error;
use dok3_domain::{AclScope, Document, DocumentId, ParsedDocument, RevisionId, SourceId};

/// Default per-file upper bound (4 MiB) before a file is skipped during scanning.
pub const DEFAULT_MAX_FILE_BYTES: u64 = 4 * 1024 * 1024;

/// Configuration describing a local filesystem knowledge source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceConfig {
    pub source_id: SourceId,
    pub source_label: String,
    pub root_dir: PathBuf,
    pub category: String,
    pub language: String,
    pub default_tag: String,
    pub acl_scope: AclScope,
    pub allowed_extensions: Vec<String>,
    pub max_file_bytes: u64,
}

impl SourceConfig {
    /// Creates a `SourceConfig` preconfigured for local Markdown documentation.
    #[must_use]
    pub fn markdown_defaults(
        source_id: SourceId,
        source_label: impl Into<String>,
        root_dir: impl Into<PathBuf>,
    ) -> Self {
        Self {
            source_id,
            source_label: source_label.into(),
            root_dir: root_dir.into(),
            category: "guide".to_string(),
            language: "ko".to_string(),
            default_tag: "docs".to_string(),
            acl_scope: AclScope::Public,
            allowed_extensions: vec![
                "md".to_string(),
                "markdown".to_string(),
                "mdown".to_string(),
                "mkd".to_string(),
            ],
            max_file_bytes: DEFAULT_MAX_FILE_BYTES,
        }
    }
}

/// Raw file payload collected by [`LocalFileConnector::scan`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScannedFile {
    pub document_id: DocumentId,
    pub relative_path: String,
    pub absolute_path: PathBuf,
    pub filename: String,
    pub media_type: String,
    pub bytes: Vec<u8>,
    pub size_bytes: u64,
}

/// Metadata for a matching file skipped during scanning (for example exceeding size bounds).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkippedFile {
    pub relative_path: String,
    pub size_bytes: u64,
    pub reason: String,
}

/// Aggregated result returned by [`LocalFileConnector::scan`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ScanReport {
    pub files: Vec<ScannedFile>,
    pub skipped: Vec<SkippedFile>,
}

/// Scans a local directory tree deterministically for Markdown sources.
#[derive(Debug, Clone)]
pub struct LocalFileConnector {
    config: SourceConfig,
}

impl LocalFileConnector {
    /// Validates `config` and constructs a connector.
    pub fn new(config: SourceConfig) -> Result<Self, Dok3Error> {
        if config.source_id.as_str().trim().is_empty() {
            return Err(Dok3Error::InvalidInput(
                "source_id must not be empty".to_string(),
            ));
        }
        if config.allowed_extensions.is_empty() {
            return Err(Dok3Error::InvalidInput(
                "allowed_extensions must contain at least one extension".to_string(),
            ));
        }
        Ok(Self { config })
    }

    /// Returns a reference to the connector configuration.
    #[must_use]
    pub fn config(&self) -> &SourceConfig {
        &self.config
    }

    /// Walks `root_dir` in sorted order, collecting eligible files and recording size skips.
    /// Hidden entries (`.`-prefixed) and symbolic links (including `root_dir` itself) are rejected or excluded.
    pub fn scan(&self) -> Result<ScanReport, Dok3Error> {
        let root = &self.config.root_dir;
        let root_meta = match fs::symlink_metadata(root) {
            Ok(m) => m,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Err(Dok3Error::NotFound(format!(
                    "source root directory '{}' does not exist",
                    root.display()
                )));
            }
            Err(e) => {
                return Err(Dok3Error::Storage(format!(
                    "failed to stat source root '{}': {e}",
                    root.display()
                )));
            }
        };
        if root_meta.file_type().is_symlink() {
            return Err(Dok3Error::InvalidInput(format!(
                "source root '{}' must not be a symbolic link",
                root.display()
            )));
        }
        if !root_meta.is_dir() {
            return Err(Dok3Error::InvalidInput(format!(
                "source root '{}' is not a directory",
                root.display()
            )));
        }

        let mut report = ScanReport::default();
        self.walk_dir(root, &mut report)?;
        Ok(report)
    }

    fn walk_dir(&self, dir: &Path, report: &mut ScanReport) -> Result<(), Dok3Error> {
        let mut entries = fs::read_dir(dir)
            .map_err(|e| {
                Dok3Error::Storage(format!("failed to read dir '{}': {e}", dir.display()))
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                Dok3Error::Storage(format!("failed to iterate dir '{}': {e}", dir.display()))
            })?;
        entries.sort_by_key(|entry| entry.file_name());

        for entry in entries {
            let name = entry.file_name();
            if name.to_string_lossy().starts_with('.') {
                continue;
            }
            let file_type = entry.file_type().map_err(|e| {
                Dok3Error::Storage(format!(
                    "failed to stat entry '{}': {e}",
                    entry.path().display()
                ))
            })?;
            if file_type.is_symlink() {
                continue;
            }
            let path = entry.path();
            if file_type.is_dir() {
                self.walk_dir(&path, report)?;
            } else if file_type.is_file() {
                self.collect_file(&path, report)?;
            }
        }
        Ok(())
    }

    fn collect_file(&self, path: &Path, report: &mut ScanReport) -> Result<(), Dok3Error> {
        let Some(ext) = path
            .extension()
            .and_then(|s| s.to_str())
            .map(str::to_ascii_lowercase)
        else {
            return Ok(());
        };
        if !self
            .config
            .allowed_extensions
            .iter()
            .any(|allowed| allowed.eq_ignore_ascii_case(&ext))
        {
            return Ok(());
        }

        let lmeta = fs::symlink_metadata(path).map_err(|e| {
            Dok3Error::Storage(format!("failed to stat file '{}': {e}", path.display()))
        })?;
        if lmeta.file_type().is_symlink() || !lmeta.is_file() {
            return Ok(());
        }

        let relative_path = normalize_relative(&self.config.root_dir, path)?;
        let file = File::open(path).map_err(|e| {
            Dok3Error::Storage(format!("failed to open file '{}': {e}", path.display()))
        })?;
        let cap = self.config.max_file_bytes.saturating_add(1);
        let mut bytes = Vec::new();
        file.take(cap).read_to_end(&mut bytes).map_err(|e| {
            Dok3Error::Storage(format!("failed to read file '{}': {e}", path.display()))
        })?;

        let actual_len = bytes.len() as u64;
        if actual_len > self.config.max_file_bytes {
            report.skipped.push(SkippedFile {
                relative_path,
                size_bytes: lmeta.len().max(actual_len),
                reason: format!("exceeds max_file_bytes ({})", self.config.max_file_bytes),
            });
            return Ok(());
        }

        let filename = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(relative_path.as_str())
            .to_string();
        let document_id = DocumentId::new(format!("{}:{}", self.config.source_id, relative_path));

        report.files.push(ScannedFile {
            document_id,
            relative_path,
            absolute_path: path.to_path_buf(),
            filename,
            media_type: media_type_for_ext(&ext).to_string(),
            bytes,
            size_bytes: actual_len,
        });
        Ok(())
    }
}

/// Converts a scanned file and its parsed Markdown sections into a canonical domain [`Document`].
#[must_use]
pub fn build_document(
    config: &SourceConfig,
    scanned: &ScannedFile,
    parsed: &ParsedDocument,
) -> Document {
    let summary = extract_summary(parsed);
    let mut tags = Vec::new();
    if !config.default_tag.trim().is_empty() {
        tags.push(config.default_tag.trim().to_string());
    }
    let lang = config.language.trim();
    if !lang.is_empty() && !tags.iter().any(|t| t.eq_ignore_ascii_case(lang)) {
        tags.push(lang.to_string());
    }

    Document {
        id: scanned.document_id.clone(),
        revision_id: RevisionId::new("pending"),
        source_id: config.source_id.clone(),
        source: config.source_label.clone(),
        title: parsed.title.clone(),
        summary,
        category: config.category.clone(),
        topic: None,
        tag: config.default_tag.clone(),
        tags,
        keywords: Vec::new(),
        language: config.language.clone(),
        version: "v1".to_string(),
        date: None,
        source_uri: format!("file:///{}", scanned.relative_path),
        sample_path: Some(scanned.relative_path.clone()),
        acl_scope: config.acl_scope.clone(),
        content_hash: String::new(),
        body: parsed.normalized_text.clone(),
        markdown: Some(parsed.normalized_text.clone()),
        sections: parsed.sections.clone(),
    }
}

fn normalize_relative(root: &Path, path: &Path) -> Result<String, Dok3Error> {
    let rel = path.strip_prefix(root).map_err(|e| {
        Dok3Error::Internal(format!(
            "scanned path '{}' is outside root '{}': {e}",
            path.display(),
            root.display()
        ))
    })?;
    let mut parts = Vec::new();
    for component in rel.components() {
        match component {
            Component::Normal(segment) => {
                let s = segment.to_str().ok_or_else(|| {
                    Dok3Error::InvalidInput(format!(
                        "non-UTF-8 path component in '{}'",
                        rel.display()
                    ))
                })?;
                if s.contains('/') || s.contains('\\') {
                    return Err(Dok3Error::InvalidInput(format!(
                        "path component '{s}' contains a path separator"
                    )));
                }
                parts.push(s);
            }
            other => {
                return Err(Dok3Error::InvalidInput(format!(
                    "unexpected relative path component {:?} in '{}'",
                    other,
                    rel.display()
                )));
            }
        }
    }
    Ok(parts.join("/"))
}

fn media_type_for_ext(ext: &str) -> &'static str {
    match ext {
        "md" | "markdown" | "mdown" | "mkd" => "text/markdown",
        _ => "text/plain",
    }
}

fn extract_summary(parsed: &ParsedDocument) -> String {
    for section in &parsed.sections {
        for line in section.body.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("```") || trimmed.starts_with("~~~") {
                continue;
            }
            return trimmed.chars().take(200).collect();
        }
    }
    parsed.title.chars().take(200).collect()
}
