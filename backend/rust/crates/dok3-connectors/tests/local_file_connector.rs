use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use dok3_connectors::{LocalFileConnector, SourceConfig, build_document};
use dok3_core::Dok3Error;
use dok3_domain::{AclScope, DocumentParserPort, SourceId};
use dok3_parser::MarkdownParser;

struct TempDir(PathBuf);

impl TempDir {
    fn new(name: &str) -> Self {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "dok3-connectors-{name}-{}-{nanos}",
            std::process::id()
        ));
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn scan_walks_deterministically_skips_hidden_unsupported_oversized_and_symlinks() {
    let dir = TempDir::new("scan");
    let sub_b = dir.0.join("b");
    let sub_a = dir.0.join("a");
    let hidden = dir.0.join(".hidden");
    fs::create_dir_all(&sub_b).unwrap();
    fs::create_dir_all(&sub_a).unwrap();
    fs::create_dir_all(&hidden).unwrap();

    fs::write(dir.0.join(".dotfile.md"), "# hidden").unwrap();
    fs::write(hidden.join("secret.md"), "# hidden dir").unwrap();
    fs::write(dir.0.join("notes.txt"), "not markdown").unwrap();
    fs::write(sub_b.join("second.MD"), "# Second\nBody B").unwrap();
    fs::write(sub_a.join("first.md"), "# First\nBody A").unwrap();
    fs::write(sub_a.join("big.md"), "# Too Big\n12345678901234567890").unwrap();

    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(sub_a.join("first.md"), dir.0.join("alias.md")).unwrap();
        std::os::unix::fs::symlink(&sub_a, dir.0.join("alias-dir")).unwrap();
    }

    let mut cfg =
        SourceConfig::markdown_defaults(SourceId::new("local-test"), "로컬 테스트", &dir.0);
    cfg.max_file_bytes = 24;
    cfg.acl_scope = AclScope::internal("dev");

    let connector = LocalFileConnector::new(cfg).unwrap();
    let report = connector.scan().unwrap();

    let paths: Vec<&str> = report
        .files
        .iter()
        .map(|f| f.relative_path.as_str())
        .collect();
    assert_eq!(paths, vec!["a/first.md", "b/second.MD"]);
    assert_eq!(
        report.files[0].document_id.as_str(),
        "local-test:a/first.md"
    );
    assert_eq!(report.files[0].media_type, "text/markdown");

    assert_eq!(report.skipped.len(), 1);
    assert_eq!(report.skipped[0].relative_path, "a/big.md");
    assert!(report.skipped[0].size_bytes > 24);

    // Missing directory returns Dok3Error::NotFound.
    let missing_cfg = SourceConfig::markdown_defaults(
        SourceId::new("missing"),
        "Missing",
        dir.0.join("does-not-exist"),
    );
    let missing_err = LocalFileConnector::new(missing_cfg)
        .unwrap()
        .scan()
        .unwrap_err();
    assert!(matches!(missing_err, Dok3Error::NotFound(_)));
}

#[test]
fn scan_and_parse_over_100_real_offline_markdown_samples() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let samples_root = manifest_dir
        .join("../../../../frontend/src/samples/comprehensive-rust/chapters")
        .canonicalize()
        .expect("comprehensive-rust samples directory should exist");

    let cfg = SourceConfig::markdown_defaults(
        SourceId::new("comprehensive-rust"),
        "Comprehensive Rust",
        &samples_root,
    );
    let connector = LocalFileConnector::new(cfg.clone()).unwrap();
    let report = connector.scan().unwrap();
    assert!(
        report.files.len() >= 100,
        "expected >= 100 markdown files, got {}",
        report.files.len()
    );
    assert!(report.skipped.is_empty());

    // Determinism check across repeat scan.
    let repeat = connector.scan().unwrap();
    assert_eq!(
        report
            .files
            .iter()
            .map(|f| f.document_id.as_str())
            .collect::<Vec<_>>(),
        repeat
            .files
            .iter()
            .map(|f| f.document_id.as_str())
            .collect::<Vec<_>>()
    );

    let parser = MarkdownParser::new();
    let mut seen_ids = HashSet::new();
    for file in &report.files {
        assert!(parser.supports(&file.media_type, &file.filename));
        let parsed = parser.parse(&file.filename, &file.bytes).unwrap();
        let doc = build_document(&cfg, file, &parsed);
        assert!(seen_ids.insert(doc.id.clone()), "duplicate id: {}", doc.id);
        assert!(!doc.title.trim().is_empty());
        assert!(!doc.sections.is_empty());
        assert_eq!(doc.category, "guide");
        assert!(doc.source_uri.starts_with("file://"));
    }
}
