use std::path::PathBuf;
use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use dok3_api::{ApiState, build_router};
use dok3_domain::{
    AclScope, Document, DocumentDetail, DocumentId, DocumentSection, HealthReadyResponse,
    RevisionId, SearchResponse, SectionId, SectionReadResponse, SourceId,
};
use dok3_search::{InMemorySearchRepository, SearchService};
use http_body_util::BodyExt;
use tower::ServiceExt;

fn sample_design_dir() -> PathBuf {
    let dir =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../frontend/src/samples/design");
    assert!(
        dir.is_dir(),
        "missing design samples dir: {}",
        dir.display()
    );
    dir
}

async fn response_json<T: serde::de::DeserializeOwned>(
    response: axum::response::Response,
) -> (StatusCode, axum::http::HeaderMap, T) {
    let status = response.status();
    let headers = response.headers().clone();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let parsed: T = serde_json::from_slice(&bytes).unwrap();
    (status, headers, parsed)
}

#[tokio::test]
async fn health_search_document_section_and_404_routes_work() {
    let design_dir = sample_design_dir();
    let repo = Arc::new(InMemorySearchRepository::from_seed(Some(&design_dir)));
    let service = Arc::new(SearchService::new(repo.clone(), repo));
    let app = build_router(ApiState::new(service), None);

    // 1. /health/ready + security headers
    let (status, headers, health): (_, _, HealthReadyResponse) = response_json(
        app.clone()
            .oneshot(Request::get("/health/ready").body(Body::empty()).unwrap())
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(health.status, "ready");
    assert!(health.document_count > 20);
    assert_eq!(
        headers.get(header::X_CONTENT_TYPE_OPTIONS).unwrap(),
        "nosniff"
    );
    assert_eq!(headers.get(header::X_FRAME_OPTIONS).unwrap(), "DENY");
    assert!(headers.contains_key(header::CONTENT_SECURITY_POLICY));

    // 2. POST /api/v1/search
    let (status, _, search): (_, _, SearchResponse) = response_json(
        app.clone()
            .oneshot(
                Request::post("/api/v1/search")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(r#"{"query":"Tantivy","limit":5}"#))
                    .unwrap(),
            )
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(!search.hits.is_empty());
    assert_eq!(
        search.snapshot.generation_id.as_str(),
        "gen-inmemory-000001"
    );

    // 3. GET /api/v1/documents/{id}
    let (status, _, doc): (_, _, DocumentDetail) = response_json(
        app.clone()
            .oneshot(
                Request::get("/api/v1/documents/design-index-md")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(doc.id.as_str(), "design-index-md");
    assert!(doc.formats.is_some());
    assert!(!doc.sections.is_empty());

    // 4. GET /api/v1/documents/{id}/sections/{sid}
    let first_sec = doc.sections[0].section_id.as_str();
    let (status, _, sec): (_, _, SectionReadResponse) = response_json(
        app.clone()
            .oneshot(
                Request::get(format!(
                    "/api/v1/documents/design-index-md/sections/{first_sec}?max_chars=40"
                ))
                .body(Body::empty())
                .unwrap(),
            )
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(sec.text.chars().count() <= 40);

    // 5. Unknown /api route returns JSON 404 (not HTML)
    let unknown_api = app
        .clone()
        .oneshot(
            Request::get("/api/v1/does-not-exist")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(unknown_api.status(), StatusCode::NOT_FOUND);
    let body_json: serde_json::Value =
        serde_json::from_slice(&unknown_api.into_body().collect().await.unwrap().to_bytes())
            .unwrap();
    assert_eq!(body_json["error"]["code"], "NOT_FOUND");

    // 6. Missing document returns 404
    let missing_doc = app
        .oneshot(
            Request::get("/api/v1/documents/no-such-doc")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(missing_doc.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn internal_documents_are_blocked_over_http_for_public_principal() {
    let docs = vec![Document {
        id: DocumentId::new("secret-doc"),
        revision_id: RevisionId::new("rev-1"),
        source_id: SourceId::new("vault"),
        source: "보안 문서".to_string(),
        title: "내부 보안 지침".to_string(),
        summary: "대외비".to_string(),
        category: "policy".to_string(),
        tag: "보안".to_string(),
        tags: vec!["보안".to_string()],
        keywords: vec![],
        language: "ko".to_string(),
        version: "1.0".to_string(),
        date: None,
        source_uri: "internal://secret-doc".to_string(),
        sample_path: None,
        acl_scope: AclScope::internal("security"),
        content_hash: "h".to_string(),
        body: "기밀 내용".to_string(),
        markdown: None,
        sections: vec![DocumentSection {
            section_id: SectionId::new("body"),
            heading: "본문".to_string(),
            anchor: "body".to_string(),
            order: 1,
            line_start: 1,
            line_end: 1,
            body: "기밀 내용".to_string(),
        }],
    }];
    let repo = Arc::new(InMemorySearchRepository::new(docs, vec![]));
    let service = Arc::new(SearchService::new(repo.clone(), repo));
    let app = build_router(ApiState::new(service), None);

    // Search even with requested_scope: "internal:security" returns 0 hits
    let (_, _, search): (_, _, SearchResponse) = response_json(
        app.clone()
            .oneshot(
                Request::post("/api/v1/search")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        r#"{"query":"기밀","requested_scope":"internal:security"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(search.total_hits, 0);

    // Direct document and section endpoints return 403 Forbidden
    let doc_resp = app
        .clone()
        .oneshot(
            Request::get("/api/v1/documents/secret-doc")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(doc_resp.status(), StatusCode::FORBIDDEN);

    let sec_resp = app
        .oneshot(
            Request::get("/api/v1/documents/secret-doc/sections/body")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(sec_resp.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn spa_fallback_serves_200_for_routes_and_404_for_missing_assets() {
    let static_dir = std::env::temp_dir().join(format!("dok3-spa-test-{}", std::process::id()));
    std::fs::create_dir_all(&static_dir).unwrap();
    std::fs::write(
        static_dir.join("index.html"),
        "<html><div id=\"root\"></div></html>",
    )
    .unwrap();

    let repo = Arc::new(InMemorySearchRepository::from_seed(None));
    let service = Arc::new(SearchService::new(repo.clone(), repo));
    let app = build_router(ApiState::new(service), Some(static_dir.clone()));

    let root_resp = app
        .clone()
        .oneshot(Request::get("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(root_resp.status(), StatusCode::OK);

    let search_route = app
        .clone()
        .oneshot(
            Request::get("/search?q=Tantivy")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(search_route.status(), StatusCode::OK);

    let missing_asset = app
        .oneshot(
            Request::get("/assets/missing.js")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(missing_asset.status(), StatusCode::NOT_FOUND);

    let _ = std::fs::remove_dir_all(&static_dir);
}
