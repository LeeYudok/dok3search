use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

struct TempDir(PathBuf);
impl TempDir {
    fn new(name: &str) -> Self {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("dok3-vcs-{name}-{}-{nanos}", std::process::id()));
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }
    fn path(&self) -> &Path {
        &self.0
    }
}
impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

use dok3_connectors::{
    GitRepoConnector, SourceConfig, SvnConnector, VcsCommandOutput, VcsCommandRunner,
};
use dok3_core::Dok3Error;
use dok3_domain::SourceId;

fn git(dir: &Path, args: &[&str]) {
    let status = Command::new("git")
        .args(args)
        .current_dir(dir)
        .status()
        .unwrap();
    assert!(status.success(), "git {args:?} failed");
}

#[test]
fn git_connector_collects_initial_and_incremental_rename_delete() {
    let dir = TempDir::new("git");
    git(dir.path(), &["init", "-q"]);
    git(dir.path(), &["config", "user.name", "test"]);
    git(dir.path(), &["config", "user.email", "test@example.com"]);

    fs::write(dir.path().join("a.md"), "# A\nfirst").unwrap();
    fs::write(dir.path().join("b.md"), "# B\nsecond").unwrap();
    git(dir.path(), &["add", "."]);
    git(dir.path(), &["commit", "-q", "-m", "init"]);

    let cfg =
        SourceConfig::supported_document_defaults(SourceId::new("git-src"), "Git Repo", dir.path());
    let conn = GitRepoConnector::new(cfg);
    let initial = conn.collect_since(None).unwrap();
    assert_eq!(initial.files.len(), 2);

    git(dir.path(), &["mv", "a.md", "a_renamed.md"]);
    git(dir.path(), &["rm", "-q", "b.md"]);
    fs::write(dir.path().join("c.rs"), "pub fn added() {}\n").unwrap();
    git(dir.path(), &["add", "."]);
    git(dir.path(), &["commit", "-q", "-m", "second"]);

    let inc = conn.collect_since(Some(&initial.revision)).unwrap();
    let paths: Vec<&str> = inc.files.iter().map(|f| f.relative_path.as_str()).collect();
    assert!(paths.contains(&"a_renamed.md"), "{paths:?}");
    assert!(paths.contains(&"c.rs"), "{paths:?}");
    assert!(
        inc.deleted_ids
            .iter()
            .any(|id| id.as_str() == "git-src:a.md")
    );
    assert!(
        inc.deleted_ids
            .iter()
            .any(|id| id.as_str() == "git-src:b.md")
    );
}

struct MockSvnRunner;
impl VcsCommandRunner for MockSvnRunner {
    fn run(
        &self,
        _program: &str,
        args: &[&str],
        _cwd: &Path,
    ) -> Result<VcsCommandOutput, Dok3Error> {
        let stdout = if args.starts_with(&["info"]) {
            b"42\n".to_vec()
        } else if args.starts_with(&["diff"]) {
            b"M       docs/guide.md\nD       docs/old.md\n".to_vec()
        } else {
            b"# Updated Guide\n".to_vec()
        };
        Ok(VcsCommandOutput {
            stdout,
            stderr: String::new(),
            status_code: 0,
        })
    }
}

#[test]
fn svn_connector_parses_diff_summarize_and_deletes() {
    let dir = TempDir::new("git");
    let cfg =
        SourceConfig::supported_document_defaults(SourceId::new("svn-src"), "SVN Repo", dir.path());
    let conn = SvnConnector::with_runner(cfg, MockSvnRunner);
    let batch = conn.collect_since(Some("40")).unwrap();
    assert_eq!(batch.revision, "42");
    assert_eq!(batch.files.len(), 1);
    assert_eq!(batch.files[0].relative_path, "docs/guide.md");
    assert_eq!(batch.deleted_ids[0].as_str(), "svn-src:docs/old.md");
}
