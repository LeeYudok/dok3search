//! Builds an in-memory local Markdown snapshot before the HTTP listener opens.

use std::{path::Path, sync::Arc};

use crate::corpus_meta::CorpusMetadataIndex;
use dok3_connectors::{LocalFileConnector, SourceConfig, build_document};
use dok3_domain::{GenerationId, SourceId};
use dok3_index::TantivyIndex;
use dok3_parser::ParserRegistry;
use dok3_search::{InMemorySearchRepository, SearchService};
use dok3_storage::content_revision;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct BuildReport {
    pub indexed: usize,
    pub skipped: Vec<String>,
}

pub fn build(root: &Path) -> Result<(SearchService, BuildReport), String> {
    let connector = LocalFileConnector::new(SourceConfig::supported_document_defaults(
        SourceId::new("local-markdown"),
        "Local Markdown",
        root,
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
    let index = Arc::new(
        TantivyIndex::create_in_ram(GenerationId::new("local-markdown"))
            .map_err(|err| err.to_string())?,
    );
    let parser = ParserRegistry::with_defaults();
    let corpus_meta = CorpusMetadataIndex::load(root);
    let mut documents = Vec::with_capacity(report.files.len());
    for file in report.files {
        let parsed = match parser.parse_auto(&file.media_type, &file.filename, &file.bytes) {
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
        corpus_meta.apply(&mut document, &file.relative_path);
        let revision = content_revision(&file.bytes);
        document.content_hash = revision
            .as_str()
            .strip_prefix("sha256:")
            .unwrap_or(revision.as_str())
            .to_string();
        document.revision_id = revision;
        index
            .upsert_document(&document)
            .map_err(|err| format!("index {}: {err}", file.relative_path))?;
        documents.push(document);
    }
    index
        .commit_and_reload(None)
        .map_err(|err| err.to_string())?;
    build_report.indexed = documents.len();
    let store = Arc::new(InMemorySearchRepository::new(documents, Vec::new()));
    Ok((SearchService::new(index, store), build_report))
}

pub(crate) fn apply_detected_language(document: &mut dok3_domain::Document, text: &str) {
    let language = detect_language(text);
    document.language = language.to_string();
    document
        .tags
        .retain(|tag| !matches!(tag.as_str(), "ko" | "en"));
    document.tags.push(language.to_string());
}

fn detect_language(text: &str) -> &'static str {
    let mut hangul = 0usize;
    let mut latin = 0usize;
    for ch in text.chars() {
        if is_hangul(ch) {
            hangul += 1;
        } else if ch.is_ascii_alphabetic() {
            latin += 1;
        }
    }
    if hangul >= 10 && hangul.saturating_mul(3) >= latin {
        "ko"
    } else {
        "en"
    }
}

fn is_hangul(ch: char) -> bool {
    matches!(ch as u32, 0xAC00..=0xD7A3 | 0x1100..=0x11FF | 0x3130..=0x318F)
}

#[cfg(test)]
mod tests {
    use std::{fs, sync::Arc};

    use axum::body::Body;
    use axum::http::{Request, StatusCode, header};
    use dok3_api::{ApiState, build_router};
    use dok3_domain::{
        DocumentDetail, DocumentId, HealthReadyResponse, Principal, SearchRequest, SearchResponse,
        SectionReadResponse,
    };
    use http_body_util::BodyExt;
    use serde::de::DeserializeOwned;
    use tower::ServiceExt;

    use super::*;

    fn build_app(root: &Path) -> (axum::Router, BuildReport) {
        let (service, report) = build(root).unwrap();
        let app = build_router(ApiState::new(Arc::new(service)), None);
        (app, report)
    }

    async fn json<T: DeserializeOwned>(response: axum::response::Response) -> T {
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        serde_json::from_slice(&bytes).unwrap()
    }

    #[tokio::test]
    async fn indexes_100_documents_and_serves_health_search_document_and_section() {
        let root = tempfile::tempdir().unwrap();
        for idx in 0..100 {
            let dir = root.path().join(format!("group-{:02}", idx % 10));
            fs::create_dir_all(&dir).unwrap();
            fs::write(
                dir.join(format!("guide-{idx:03}.md")),
                format!(
                    "# Bulk {idx:03}\n\nbulkneedle{idx:03} local Markdown search body.\n\n## Detail\n\ndetail body {idx:03}\n"
                ),
            )
            .unwrap();
        }

        let (app, report) = build_app(root.path());
        assert_eq!(report.indexed, 100);
        assert!(report.skipped.is_empty());

        let health: HealthReadyResponse = json(
            app.clone()
                .oneshot(Request::get("/health/ready").body(Body::empty()).unwrap())
                .await
                .unwrap(),
        )
        .await;
        assert_eq!(health.document_count, 100);
        assert_eq!(health.mode, "in-memory-fake");

        let search: SearchResponse = json(
            app.clone()
                .oneshot(
                    Request::post("/api/v1/search")
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(Body::from(
                            r#"{"query":"bulkneedle042","limit":100,"include_snippets":false}"#,
                        ))
                        .unwrap(),
                )
                .await
                .unwrap(),
        )
        .await;
        assert_eq!(search.total_hits, 1);
        assert_eq!(
            search.hits[0].document_id.as_str(),
            "local-markdown:group-02/guide-042.md"
        );

        let document_response = app
            .clone()
            .oneshot(
                Request::get("/api/v1/documents/local-markdown%3Agroup-02%2Fguide-042.md")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(document_response.status(), StatusCode::OK);
        let document: DocumentDetail = json(document_response).await;
        assert_eq!(document.source_uri, "file:///group-02/guide-042.md");
        assert!(!document.sections.is_empty());

        let section_id = document.sections[0].section_id.as_str();
        let section: SectionReadResponse = json(
            app.oneshot(
                Request::get(format!(
                    "/api/v1/documents/local-markdown%3Agroup-02%2Fguide-042.md/sections/{section_id}"
                ))
                .body(Body::empty())
                .unwrap(),
            )
            .await
            .unwrap(),
        )
        .await;
        assert!(section.text.contains("bulkneedle042"));
    }

    #[tokio::test]
    async fn skips_empty_and_cp949_files_and_detects_korean_and_english() {
        let root = tempfile::tempdir().unwrap();
        fs::write(
            root.path().join("korean.md"),
            "# 한국어 문서\n\n한국어 검색 품질을 확인하는 문장입니다. 충분한 한글 문자가 들어 있습니다. validtoken\n",
        )
        .unwrap();
        fs::write(
            root.path().join("english.md"),
            "# English document\n\nThis document verifies English language detection. validtoken\n",
        )
        .unwrap();
        fs::write(root.path().join("empty.md"), " \n\t\n").unwrap();
        fs::write(
            root.path().join("cp949.md"),
            [0xB0, 0xA1, 0xB3, 0xAA, 0x20, 0xC7, 0xD1],
        )
        .unwrap();

        let (app, report) = build_app(root.path());
        assert_eq!(report.indexed, 2);
        assert_eq!(report.skipped.len(), 2);
        assert!(
            report
                .skipped
                .iter()
                .any(|entry| entry.contains("empty.md"))
        );
        assert!(
            report
                .skipped
                .iter()
                .any(|entry| entry.contains("cp949.md"))
        );

        let korean: DocumentDetail = json(
            app.clone()
                .oneshot(
                    Request::get("/api/v1/documents/local-markdown%3Akorean.md")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap(),
        )
        .await;
        assert_eq!(korean.language, "ko");
        assert!(korean.tags.iter().any(|tag| tag == "ko"));

        let english: DocumentDetail = json(
            app.clone()
                .oneshot(
                    Request::get("/api/v1/documents/local-markdown%3Aenglish.md")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap(),
        )
        .await;
        assert_eq!(english.language, "en");
        assert!(english.tags.iter().any(|tag| tag == "en"));

        let search: SearchResponse = json(
            app.oneshot(
                Request::post("/api/v1/search")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(r#"{"query":"validtoken","limit":10}"#))
                    .unwrap(),
            )
            .await
            .unwrap(),
        )
        .await;
        assert_eq!(search.total_hits, 2);
    }

    #[test]
    fn rebuild_drops_documents_removed_from_the_source_tree() {
        let root = tempfile::tempdir().unwrap();
        let old_path = root.path().join("old.md");
        fs::write(&old_path, "# Old\n\nremovedtoken old body\n").unwrap();

        let (old_service, old_report) = build(root.path()).unwrap();
        assert_eq!(old_report.indexed, 1);
        let old_search = old_service
            .search(
                &Principal::anonymous_public(),
                &SearchRequest {
                    query: "removedtoken".to_string(),
                    ..SearchRequest::default()
                },
            )
            .unwrap();
        assert_eq!(old_search.total_hits, 1);

        fs::remove_file(old_path).unwrap();
        fs::write(
            root.path().join("new.md"),
            "# New\n\nremainingtoken new body\n",
        )
        .unwrap();

        let (new_service, new_report) = build(root.path()).unwrap();
        assert_eq!(new_report.indexed, 1);
        let removed = new_service
            .search(
                &Principal::anonymous_public(),
                &SearchRequest {
                    query: "removedtoken".to_string(),
                    ..SearchRequest::default()
                },
            )
            .unwrap();
        let remaining = new_service
            .search(
                &Principal::anonymous_public(),
                &SearchRequest {
                    query: "remainingtoken".to_string(),
                    ..SearchRequest::default()
                },
            )
            .unwrap();
        assert_eq!(removed.total_hits, 0);
        assert_eq!(remaining.total_hits, 1);
    }

    #[test]
    fn indexes_html_code_and_text_through_the_parser_registry() {
        let root = tempfile::tempdir().unwrap();
        fs::write(root.path().join("guide.md"), "# Guide\n\nmdtoken body\n").unwrap();
        fs::write(
            root.path().join("page.html"),
            "<html><body><h1>Page</h1><p>htmltoken</p><script>scriptpayload()</script></body></html>",
        )
        .unwrap();
        fs::write(
            root.path().join("lib.rs"),
            "// rusttoken\npub fn alpha_fn() {}\n\npub fn beta_fn() {}\n",
        )
        .unwrap();
        fs::write(
            root.path().join("Account.java"),
            "public class AccountService {\n    // javatoken\n}\n",
        )
        .unwrap();
        fs::write(
            root.path().join("schema.sql"),
            "CREATE TABLE accounts (id INT);\n",
        )
        .unwrap();
        fs::write(
            root.path().join("view.jsp"),
            "<%@ page language=\"java\" %>\n",
        )
        .unwrap();
        fs::write(root.path().join("note.txt"), "plaintoken note\n").unwrap();
        fs::write(root.path().join("data.json"), "{\"jsontoken\": 1}").unwrap();
        fs::write(root.path().join("bad.txt"), [0xff, 0xfe, 0xfd]).unwrap();

        let (service, report) = build(root.path()).unwrap();
        assert_eq!(report.indexed, 7, "{report:?}");
        assert!(
            report
                .skipped
                .iter()
                .any(|s| s.starts_with("bad.txt: parse failed")),
            "{report:?}"
        );

        let anon = Principal::anonymous_public();
        let hits = |query: &str| {
            service
                .search(
                    &anon,
                    &SearchRequest {
                        query: query.to_string(),
                        ..SearchRequest::default()
                    },
                )
                .unwrap()
                .total_hits
        };
        assert_eq!(hits("htmltoken"), 1);
        assert_eq!(hits("rusttoken"), 1);
        assert_eq!(hits("javatoken"), 1);
        assert_eq!(hits("plaintoken"), 1);
        assert_eq!(hits("scriptpayload"), 0);
        assert_eq!(hits("jsontoken"), 0);

        let rs = service
            .get_document(&anon, &DocumentId::new("local-markdown:lib.rs"))
            .unwrap();
        assert!(rs.markdown.unwrap().starts_with("## "));
        assert!(rs.sections.len() >= 2, "{:?}", rs.sections);
    }
}
