use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use dok3_connectors::{LocalFileConnector, SourceConfig, build_document};
use dok3_core::Dok3Error;
use dok3_domain::{AclScope, DocumentParserPort, SourceId};
use dok3_parser::{MarkdownParser, ParserRegistry};

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

    // Symlink root_dir is rejected.
    #[cfg(unix)]
    {
        let link_root = dir.0.join("root-link");
        std::os::unix::fs::symlink(&sub_a, &link_root).unwrap();
        let sym_cfg = SourceConfig::markdown_defaults(SourceId::new("sym"), "Sym", &link_root);
        let sym_err = LocalFileConnector::new(sym_cfg)
            .unwrap()
            .scan()
            .unwrap_err();
        assert!(matches!(sym_err, Dok3Error::InvalidInput(_)));

        // Unix filename containing a backslash is rejected to avoid path collisions.
        let slash_dir = TempDir::new("backslash");
        fs::write(slash_dir.0.join("bad\\name.md"), "# Bad").unwrap();
        let bs_cfg = SourceConfig::markdown_defaults(SourceId::new("bs"), "BS", &slash_dir.0);
        let bs_err = LocalFileConnector::new(bs_cfg).unwrap().scan().unwrap_err();
        assert!(matches!(bs_err, Dok3Error::InvalidInput(_)));
    }
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
    assert_eq!(report.files.len(), 337);
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
        assert!(doc.source_uri.starts_with("file:///"));
    }
}

#[test]
fn supported_document_defaults_scan_and_render_multi_format_sources() {
    let dir = TempDir::new("multi-format");
    fs::write(dir.0.join("guide.md"), "# Guide\nbody").unwrap();
    fs::write(
        dir.0.join("page.html"),
        "<html><body><h1>Page</h1><p>hello</p></body></html>",
    )
    .unwrap();
    fs::write(
        dir.0.join("lib.rs"),
        "/// ```\n/// fenced\n/// ```\npub fn alpha() {}\n",
    )
    .unwrap();
    fs::write(
        dir.0.join("Account.java"),
        "public class AccountService {}\n",
    )
    .unwrap();
    fs::write(dir.0.join("main.go"), "func Run() {}\n").unwrap();
    fs::write(
        dir.0.join("schema.sql"),
        "CREATE TABLE accounts (id INT);\n",
    )
    .unwrap();
    fs::write(
        dir.0.join("view.jsp"),
        "<%@ page language=\"java\" %>\n<p>x</p>\n",
    )
    .unwrap();
    fs::write(dir.0.join("note.txt"), "plain note\n").unwrap();
    fs::write(dir.0.join("app.log"), "log line\n").unwrap();
    fs::write(dir.0.join("provenance.json"), "{}").unwrap();
    fs::write(dir.0.join("index.yaml"), "a: 1\n").unwrap();

    let cfg = SourceConfig::supported_document_defaults(SourceId::new("multi"), "Multi", &dir.0);
    let connector = LocalFileConnector::new(cfg).unwrap();
    let report = connector.scan().unwrap();
    let names: Vec<&str> = report
        .files
        .iter()
        .map(|f| f.relative_path.as_str())
        .collect();
    assert_eq!(
        names,
        vec![
            "Account.java",
            "app.log",
            "guide.md",
            "lib.rs",
            "main.go",
            "note.txt",
            "page.html",
            "schema.sql",
            "view.jsp"
        ]
    );
    let media = |name: &str| {
        report
            .files
            .iter()
            .find(|f| f.relative_path == name)
            .unwrap()
            .media_type
            .clone()
    };
    assert_eq!(media("page.html"), "text/html");
    assert_eq!(media("lib.rs"), "text/x-rust");
    assert_eq!(media("schema.sql"), "application/sql");
    assert_eq!(media("view.jsp"), "application/jsp");
    assert_eq!(media("note.txt"), "text/plain");

    let registry = ParserRegistry::with_defaults();
    for file in &report.files {
        let parsed = registry
            .parse_auto(&file.media_type, &file.filename, &file.bytes)
            .unwrap_or_else(|err| panic!("{}: {err}", file.relative_path));
        let doc = build_document(connector.config(), file, &parsed);
        let markdown = doc.markdown.clone().unwrap();
        match file.relative_path.as_str() {
            "lib.rs" => {
                assert!(markdown.contains("````rust"), "{markdown}");
                assert!(doc.body.contains("pub fn alpha"));
            }
            "view.jsp" => {
                assert!(markdown.contains("```jsp"), "{markdown}");
                assert!(markdown.contains("<%@ page"));
            }
            "note.txt" => assert!(markdown.starts_with("```text\nplain note\n```")),
            "guide.md" => assert_eq!(markdown, doc.body),
            _ => {}
        }
    }
}

#[test]
fn disambiguates_nextjs_app_router_and_barrel_filenames_by_relative_path() {
    let dir = TempDir::new("nextjs-routes");
    fs::create_dir_all(dir.0.join("app/orders")).unwrap();
    fs::create_dir_all(dir.0.join("app/users")).unwrap();
    fs::create_dir_all(dir.0.join("app/api/orders")).unwrap();
    fs::create_dir_all(dir.0.join("java")).unwrap();

    fs::write(
        dir.0.join("app/orders/page.tsx"),
        "export default function OrdersPage() { return <main>Orders</main>; }\n",
    )
    .unwrap();
    fs::write(
        dir.0.join("app/users/page.tsx"),
        "export default function UsersPage() { return <main>Users</main>; }\n",
    )
    .unwrap();
    fs::write(
        dir.0.join("app/api/orders/route.ts"),
        "export async function GET() { return Response.json([]); }\n",
    )
    .unwrap();
    fs::write(
        dir.0.join("java/OrderService.java"),
        "public class OrderService {}\n",
    )
    .unwrap();

    let cfg = SourceConfig::supported_document_defaults(SourceId::new("repo"), "Repo", &dir.0);
    let connector = LocalFileConnector::new(cfg.clone()).unwrap();
    let report = connector.scan().unwrap();
    let registry = ParserRegistry::with_defaults();
    let titles: Vec<(String, String)> = report
        .files
        .iter()
        .map(|file| {
            let parsed = registry
                .parse_auto(&file.media_type, &file.filename, &file.bytes)
                .unwrap();
            let doc = build_document(&cfg, file, &parsed);
            (file.relative_path.clone(), doc.title)
        })
        .collect();

    assert_eq!(
        titles,
        vec![
            (
                "app/api/orders/route.ts".to_string(),
                "app/api/orders/route.ts".to_string()
            ),
            (
                "app/orders/page.tsx".to_string(),
                "app/orders/page.tsx".to_string()
            ),
            (
                "app/users/page.tsx".to_string(),
                "app/users/page.tsx".to_string()
            ),
            (
                "java/OrderService.java".to_string(),
                "OrderService.java".to_string()
            ),
        ]
    );
}
