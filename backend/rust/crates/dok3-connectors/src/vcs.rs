//! Git and Subversion (SVN) incremental source connectors.

use std::path::PathBuf;
use std::process::Command;

use dok3_core::Dok3Error;
use dok3_domain::{DocumentId, SourceId};

use crate::local_fs::{DEFAULT_MAX_FILE_BYTES, ScannedFile, SourceConfig, media_type_for_ext};

/// Output of a VCS CLI command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VcsCommandOutput {
    pub stdout: Vec<u8>,
    pub stderr: String,
    pub status_code: i32,
}

/// Injectable command runner so VCS connectors can be tested deterministically without `svn`.
pub trait VcsCommandRunner: Send + Sync {
    fn run(
        &self,
        program: &str,
        args: &[&str],
        cwd: &std::path::Path,
    ) -> Result<VcsCommandOutput, Dok3Error>;
}

/// Default process-spawning [`VcsCommandRunner`].
#[derive(Debug, Default, Clone, Copy)]
pub struct StdVcsCommandRunner;

impl VcsCommandRunner for StdVcsCommandRunner {
    fn run(
        &self,
        program: &str,
        args: &[&str],
        cwd: &std::path::Path,
    ) -> Result<VcsCommandOutput, Dok3Error> {
        let out = Command::new(program)
            .args(args)
            .current_dir(cwd)
            .output()
            .map_err(|err| Dok3Error::Storage(format!("failed to execute {program}: {err}")))?;
        Ok(VcsCommandOutput {
            stdout: out.stdout,
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
            status_code: out.status.code().unwrap_or(-1),
        })
    }
}

/// Incremental batch returned by [`GitRepoConnector`] or [`SvnConnector`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VcsBatch {
    pub revision: String,
    pub files: Vec<ScannedFile>,
    pub deleted_ids: Vec<DocumentId>,
    pub skipped: Vec<String>,
}

/// Git repository connector supporting full tree scan and `diff-tree` incremental sync.
pub struct GitRepoConnector<R: VcsCommandRunner = StdVcsCommandRunner> {
    config: SourceConfig,
    runner: R,
}

impl GitRepoConnector<StdVcsCommandRunner> {
    #[must_use]
    pub fn new(config: SourceConfig) -> Self {
        Self {
            config,
            runner: StdVcsCommandRunner,
        }
    }
}

impl<R: VcsCommandRunner> GitRepoConnector<R> {
    #[must_use]
    pub fn with_runner(config: SourceConfig, runner: R) -> Self {
        Self { config, runner }
    }

    pub fn head_revision(&self) -> Result<String, Dok3Error> {
        let out = self
            .runner
            .run("git", &["rev-parse", "HEAD"], &self.config.root_dir)?;
        if out.status_code != 0 {
            return Err(Dok3Error::Storage(format!(
                "git rev-parse HEAD failed: {}",
                out.stderr.trim()
            )));
        }
        Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
    }

    pub fn collect_since(&self, previous_rev: Option<&str>) -> Result<VcsBatch, Dok3Error> {
        let head = self.head_revision()?;
        if previous_rev == Some(head.as_str()) {
            return Ok(VcsBatch {
                revision: head,
                ..VcsBatch::default()
            });
        }

        let mut batch = VcsBatch {
            revision: head.clone(),
            ..VcsBatch::default()
        };

        let paths_to_fetch: Vec<String> = match previous_rev {
            None => {
                let out = self.runner.run(
                    "git",
                    &["ls-tree", "-r", "-z", "--name-only", &head],
                    &self.config.root_dir,
                )?;
                if out.status_code != 0 {
                    return Err(Dok3Error::Storage(format!(
                        "git ls-tree failed: {}",
                        out.stderr.trim()
                    )));
                }
                out.stdout
                    .split(|&b| b == 0)
                    .filter(|s| !s.is_empty())
                    .map(|s| String::from_utf8_lossy(s).into_owned())
                    .collect()
            }
            Some(old_rev) => {
                let out = self.runner.run(
                    "git",
                    &[
                        "diff-tree",
                        "--no-commit-id",
                        "--name-status",
                        "-r",
                        "-M",
                        "-z",
                        old_rev,
                        &head,
                    ],
                    &self.config.root_dir,
                )?;
                if out.status_code != 0 {
                    return Err(Dok3Error::Storage(format!(
                        "git diff-tree failed: {}",
                        out.stderr.trim()
                    )));
                }
                let tokens: Vec<String> = out
                    .stdout
                    .split(|&b| b == 0)
                    .filter(|s| !s.is_empty())
                    .map(|s| String::from_utf8_lossy(s).into_owned())
                    .collect();
                let mut fetch = Vec::new();
                let mut i = 0;
                while i < tokens.len() {
                    let status = &tokens[i];
                    if status.starts_with('R') && i + 2 < tokens.len() {
                        let old_path = &tokens[i + 1];
                        let new_path = &tokens[i + 2];
                        batch.deleted_ids.push(DocumentId::new(format!(
                            "{}:{old_path}",
                            self.config.source_id
                        )));
                        fetch.push(new_path.clone());
                        i += 3;
                    } else if status == "D" && i + 1 < tokens.len() {
                        let del_path = &tokens[i + 1];
                        batch.deleted_ids.push(DocumentId::new(format!(
                            "{}:{del_path}",
                            self.config.source_id
                        )));
                        i += 2;
                    } else if i + 1 < tokens.len() {
                        fetch.push(tokens[i + 1].clone());
                        i += 2;
                    } else {
                        break;
                    }
                }
                fetch
            }
        };

        for rel in paths_to_fetch {
            if !is_allowed_rel_path(&self.config, &rel) {
                continue;
            }
            let spec = format!("{head}:{rel}");
            let blob =
                self.runner
                    .run("git", &["cat-file", "blob", &spec], &self.config.root_dir)?;
            if blob.status_code != 0 {
                batch.skipped.push(format!(
                    "{rel}: git cat-file failed: {}",
                    blob.stderr.trim()
                ));
                continue;
            }
            let size_bytes = blob.stdout.len() as u64;
            let limit = self
                .config
                .max_file_bytes
                .min(DEFAULT_MAX_FILE_BYTES.max(self.config.max_file_bytes));
            if size_bytes > limit {
                batch
                    .skipped
                    .push(format!("{rel}: exceeds max_file_bytes ({limit})"));
                continue;
            }
            batch.files.push(make_scanned_file(
                &self.config.source_id,
                &self.config.root_dir,
                rel,
                blob.stdout,
            ));
        }

        Ok(batch)
    }
}

/// Subversion (SVN) working-copy/repository connector over [`VcsCommandRunner`].
pub struct SvnConnector<R: VcsCommandRunner = StdVcsCommandRunner> {
    config: SourceConfig,
    runner: R,
}

impl SvnConnector<StdVcsCommandRunner> {
    #[must_use]
    pub fn new(config: SourceConfig) -> Self {
        Self {
            config,
            runner: StdVcsCommandRunner,
        }
    }
}

impl<R: VcsCommandRunner> SvnConnector<R> {
    #[must_use]
    pub fn with_runner(config: SourceConfig, runner: R) -> Self {
        Self { config, runner }
    }

    pub fn head_revision(&self) -> Result<String, Dok3Error> {
        let out = self.runner.run(
            "svn",
            &["info", "--show-item", "revision"],
            &self.config.root_dir,
        )?;
        if out.status_code != 0 {
            return Err(Dok3Error::Storage(format!(
                "svn info failed: {}",
                out.stderr.trim()
            )));
        }
        Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
    }

    pub fn collect_since(&self, previous_rev: Option<&str>) -> Result<VcsBatch, Dok3Error> {
        let rev = self.head_revision()?;
        if previous_rev == Some(rev.as_str()) {
            return Ok(VcsBatch {
                revision: rev,
                ..VcsBatch::default()
            });
        }
        let mut batch = VcsBatch {
            revision: rev.clone(),
            ..VcsBatch::default()
        };
        let to_fetch: Vec<String> = match previous_rev {
            None => {
                let out =
                    self.runner
                        .run("svn", &["list", "-R", "-r", &rev], &self.config.root_dir)?;
                if out.status_code != 0 {
                    return Err(Dok3Error::Storage(format!(
                        "svn list failed: {}",
                        out.stderr.trim()
                    )));
                }
                String::from_utf8_lossy(&out.stdout)
                    .lines()
                    .map(str::trim)
                    .filter(|l| !l.is_empty() && !l.ends_with('/'))
                    .map(str::to_string)
                    .collect()
            }
            Some(old_rev) => {
                let range = format!("{old_rev}:{rev}");
                let out = self.runner.run(
                    "svn",
                    &["diff", "--summarize", "-r", &range],
                    &self.config.root_dir,
                )?;
                if out.status_code != 0 {
                    return Err(Dok3Error::Storage(format!(
                        "svn diff failed: {}",
                        out.stderr.trim()
                    )));
                }
                let mut fetch = Vec::new();
                for line in String::from_utf8_lossy(&out.stdout).lines() {
                    let trimmed = line.trim();
                    if trimmed.len() < 2 {
                        continue;
                    }
                    let (flag, rest) = trimmed.split_at(1);
                    let rel = rest.trim().to_string();
                    if flag == "D" {
                        batch
                            .deleted_ids
                            .push(DocumentId::new(format!("{}:{rel}", self.config.source_id)));
                    } else if !rel.is_empty() {
                        fetch.push(rel);
                    }
                }
                fetch
            }
        };

        for rel in to_fetch {
            if !is_allowed_rel_path(&self.config, &rel) {
                continue;
            }
            let out = self
                .runner
                .run("svn", &["cat", "-r", &rev, &rel], &self.config.root_dir)?;
            if out.status_code != 0 {
                batch
                    .skipped
                    .push(format!("{rel}: svn cat failed: {}", out.stderr.trim()));
                continue;
            }
            batch.files.push(make_scanned_file(
                &self.config.source_id,
                &self.config.root_dir,
                rel,
                out.stdout,
            ));
        }

        Ok(batch)
    }
}

fn is_allowed_rel_path(config: &SourceConfig, rel: &str) -> bool {
    if rel.split('/').any(|seg| seg.starts_with('.')) {
        return false;
    }
    let Some(ext) = std::path::Path::new(rel)
        .extension()
        .and_then(|s| s.to_str())
    else {
        return false;
    };
    config
        .allowed_extensions
        .iter()
        .any(|allowed| allowed.eq_ignore_ascii_case(ext))
}

fn make_scanned_file(
    source_id: &SourceId,
    root_dir: &std::path::Path,
    relative_path: String,
    bytes: Vec<u8>,
) -> ScannedFile {
    let ext = std::path::Path::new(&relative_path)
        .extension()
        .and_then(|s| s.to_str())
        .map(str::to_ascii_lowercase)
        .unwrap_or_default();
    let filename = std::path::Path::new(&relative_path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(&relative_path)
        .to_string();
    let size_bytes = bytes.len() as u64;
    ScannedFile {
        document_id: DocumentId::new(format!("{source_id}:{relative_path}")),
        absolute_path: root_dir.join(&relative_path),
        relative_path,
        filename,
        media_type: media_type_for_ext(&ext).to_string(),
        bytes,
        size_bytes,
    }
}
