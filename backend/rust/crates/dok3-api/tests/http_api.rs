use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use dok3_api::{ApiState, build_router};
use dok3_domain::{
    AclScope, ClassificationPort, ClassificationSource, CreateSourceRequest, DeleteSourceResponse,
    Document, DocumentClassification, DocumentDetail, DocumentId, DocumentSection,
    HealthReadyResponse, IndexJobListResponse, IndexJobMode, IndexJobPort, IndexJobRecord,
    IndexJobState, Principal, RegisteredSource, RevisionId, SearchResponse, SectionId,
    SectionReadResponse, SourceId, SourceListResponse, SourceRegistryPort, TriggerIndexJobRequest,
    UpdateClassificationRequest, normalize_classification_terms, validate_classification_category,
    validate_classification_summary, validate_source_id_slug,
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
        topic: None,
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

#[tokio::test]
async fn invalid_json_and_query_params_return_unified_json_error() {
    let repo = Arc::new(InMemorySearchRepository::from_seed(None));
    let service = Arc::new(SearchService::new(repo.clone(), repo));
    let app = build_router(ApiState::new(service), None);

    let cases = [
        Request::post("/api/v1/search")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from("{bad"))
            .unwrap(),
        Request::post("/api/v1/search")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(r#"{"requested_scope":"private"}"#))
            .unwrap(),
        Request::get("/api/v1/documents/index-rollbacks/sections/body?cursor=abc")
            .body(Body::empty())
            .unwrap(),
        Request::get("/api/v1/documents/index-rollbacks/sections/body?max_chars=-5")
            .body(Body::empty())
            .unwrap(),
    ];

    for req in cases {
        let resp = app.clone().oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let content_type = resp
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        assert!(
            content_type.contains("application/json"),
            "expected application/json, got {content_type}"
        );
        let body: serde_json::Value =
            serde_json::from_slice(&resp.into_body().collect().await.unwrap().to_bytes()).unwrap();
        assert_eq!(body["error"]["code"], "INVALID_INPUT");
        assert!(body["error"]["message"].is_string());
    }
}

struct StubIndexJobs;

impl IndexJobPort for StubIndexJobs {
    fn list_jobs(&self) -> Result<IndexJobListResponse, dok3_core::Dok3Error> {
        Ok(IndexJobListResponse {
            jobs: vec![IndexJobRecord {
                job_id: "job-0001".to_string(),
                source_id: None,
                trigger: "startup".to_string(),
                mode: IndexJobMode::Incremental,
                state: IndexJobState::Succeeded,
                started_at_ms: 100,
                finished_at_ms: Some(120),
                indexed_documents: 1,
                rebuilt: true,
                active_generation: Some("gen-000001-seq-000001".to_string()),
                active_epoch: 1,
                skipped_files: vec![],
                error: None,
            }],
            active_generation: Some("gen-000001-seq-000001".to_string()),
            active_epoch: 1,
            refresh_interval_secs: Some(30),
        })
    }

    fn get_job(&self, job_id: &str) -> Result<IndexJobRecord, dok3_core::Dok3Error> {
        if job_id == "job-0001" {
            Ok(self.list_jobs()?.jobs.into_iter().next().unwrap())
        } else {
            Err(dok3_core::Dok3Error::NotFound(format!(
                "job '{job_id}' not found"
            )))
        }
    }

    fn trigger_job(
        &self,
        req: &TriggerIndexJobRequest,
    ) -> Result<IndexJobRecord, dok3_core::Dok3Error> {
        Ok(IndexJobRecord {
            job_id: "job-0002".to_string(),
            source_id: req.source_id.clone(),
            trigger: "manual".to_string(),
            mode: req.mode,
            state: IndexJobState::Succeeded,
            started_at_ms: 200,
            finished_at_ms: Some(250),
            indexed_documents: 2,
            rebuilt: req.mode == IndexJobMode::Full,
            active_generation: Some("gen-000002-seq-000001".to_string()),
            active_epoch: 2,
            skipped_files: vec![],
            error: None,
        })
    }
}

#[tokio::test]
async fn index_job_routes_enforce_admin_mode_and_payload_validation() {
    let repo = Arc::new(InMemorySearchRepository::from_seed(None));
    let service = Arc::new(SearchService::new(repo.clone(), repo));

    // 1. Non-admin caller always receives 403 FORBIDDEN before body-limit or UTF-8 path extraction.
    let public_app = build_router(
        ApiState::new(Arc::clone(&service)).with_index_jobs(Arc::new(StubIndexJobs)),
        None,
    );
    let oversized_body = vec![b'a'; 3 * 1024 * 1024];
    for req in [
        Request::get("/api/v1/index/jobs")
            .body(Body::empty())
            .unwrap(),
        Request::post("/api/v1/index/jobs")
            .body(Body::empty())
            .unwrap(),
        Request::post("/api/v1/index/jobs")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(oversized_body))
            .unwrap(),
        Request::get("/api/v1/index/jobs/job-0001")
            .body(Body::empty())
            .unwrap(),
        Request::get("/api/v1/index/jobs/%FF")
            .body(Body::empty())
            .unwrap(),
    ] {
        let resp = public_app.clone().oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    }

    // 2. Admin caller without persistent job runtime receives 400 INVALID_INPUT.
    let admin_unwired = build_router(
        ApiState::new(Arc::clone(&service)).with_principal(Principal::local_admin()),
        None,
    );
    let unwired_resp = admin_unwired
        .oneshot(
            Request::get("/api/v1/index/jobs")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(unwired_resp.status(), StatusCode::BAD_REQUEST);

    // 3. Admin caller enforces Origin / Sec-Fetch-Site / Content-Type guards and succeeds on valid requests.
    let admin_app = build_router(
        ApiState::new(service)
            .with_principal(Principal::local_admin())
            .with_index_jobs(Arc::new(StubIndexJobs)),
        None,
    );

    let bad_origin = admin_app
        .clone()
        .oneshot(
            Request::post("/api/v1/index/jobs")
                .header(header::ORIGIN, "https://evil.example")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bad_origin.status(), StatusCode::FORBIDDEN);

    let cross_site = admin_app
        .clone()
        .oneshot(
            Request::post("/api/v1/index/jobs")
                .header("sec-fetch-site", "cross-site")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(cross_site.status(), StatusCode::FORBIDDEN);

    let missing_ct = admin_app
        .clone()
        .oneshot(
            Request::post("/api/v1/index/jobs")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(missing_ct.status(), StatusCode::BAD_REQUEST);

    let form_ct = admin_app
        .clone()
        .oneshot(
            Request::post("/api/v1/index/jobs")
                .header(header::CONTENT_TYPE, "text/plain")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(form_ct.status(), StatusCode::BAD_REQUEST);

    let (status, _, list): (_, _, IndexJobListResponse) = response_json(
        admin_app
            .clone()
            .oneshot(
                Request::get("/api/v1/index/jobs")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(list.jobs.len(), 1);
    assert_eq!(list.refresh_interval_secs, Some(30));

    let (status, _, empty_post): (_, _, IndexJobRecord) = response_json(
        admin_app
            .clone()
            .oneshot(
                Request::post("/api/v1/index/jobs")
                    .header(header::ORIGIN, "http://127.0.0.1:5173")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(empty_post.mode, IndexJobMode::Incremental);
    assert!(!empty_post.rebuilt);

    let (status, _, full_post): (_, _, IndexJobRecord) = response_json(
        admin_app
            .clone()
            .oneshot(
                Request::post("/api/v1/index/jobs")
                    .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
                    .body(Body::from(r#"{"mode":"full"}"#))
                    .unwrap(),
            )
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(full_post.mode, IndexJobMode::Full);
    assert!(full_post.rebuilt);

    let bad_json = admin_app
        .clone()
        .oneshot(
            Request::post("/api/v1/index/jobs")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from("{invalid"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bad_json.status(), StatusCode::BAD_REQUEST);

    let missing_job = admin_app
        .oneshot(
            Request::get("/api/v1/index/jobs/job-9999")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(missing_job.status(), StatusCode::NOT_FOUND);
}

struct StubClassification {
    state: Mutex<DocumentClassification>,
}

impl StubClassification {
    fn new() -> Self {
        Self {
            state: Mutex::new(Self::rule_default()),
        }
    }

    fn rule_default() -> DocumentClassification {
        DocumentClassification {
            document_id: DocumentId::new("index-rollbacks"),
            summary: "Initial design summary".to_string(),
            category: "design".to_string(),
            tags: vec!["architecture".to_string()],
            keywords: vec!["rollback".to_string()],
            source: ClassificationSource::Rule,
            manual_locked: false,
            updated_at_ms: 100,
            content_hash: "abc".to_string(),
        }
    }
}

impl ClassificationPort for StubClassification {
    fn get_classification(
        &self,
        principal: &Principal,
        id: &DocumentId,
    ) -> Result<DocumentClassification, dok3_core::Dok3Error> {
        if id.as_str() == "private-secret" && !principal.is_admin {
            return Err(dok3_core::Dok3Error::Forbidden(
                "document access forbidden".to_string(),
            ));
        }
        if id.as_str() != "index-rollbacks" {
            return Err(dok3_core::Dok3Error::NotFound(format!(
                "document '{}' not found",
                id.as_str()
            )));
        }
        Ok(self.state.lock().unwrap().clone())
    }

    fn update_classification(
        &self,
        principal: &Principal,
        id: &DocumentId,
        req: &UpdateClassificationRequest,
    ) -> Result<DocumentClassification, dok3_core::Dok3Error> {
        if !principal.is_admin {
            return Err(dok3_core::Dok3Error::Forbidden(
                "admin required".to_string(),
            ));
        }
        if id.as_str() != "index-rollbacks" {
            return Err(dok3_core::Dok3Error::NotFound(format!(
                "document '{}' not found",
                id.as_str()
            )));
        }
        let mut guard = self.state.lock().unwrap();
        if req.reset_to_rule {
            *guard = Self::rule_default();
            return Ok(guard.clone());
        }
        if let Some(category) = req.category.as_deref() {
            guard.category = validate_classification_category(category)?;
        }
        if let Some(summary) = req.summary.as_deref() {
            guard.summary = validate_classification_summary(summary)?;
        }
        if let Some(tags) = req.tags.as_deref() {
            guard.tags = normalize_classification_terms("tags", tags)?;
        }
        if let Some(keywords) = req.keywords.as_deref() {
            guard.keywords = normalize_classification_terms("keywords", keywords)?;
        }
        guard.source = ClassificationSource::Manual;
        guard.manual_locked = true;
        guard.updated_at_ms += 1;
        Ok(guard.clone())
    }
}

#[tokio::test]
async fn classification_routes_enforce_acl_admin_guards_and_validation() {
    let repo = Arc::new(InMemorySearchRepository::from_seed(None));
    let service = Arc::new(SearchService::new(repo.clone(), repo));

    let unwired_admin = build_router(
        ApiState::new(Arc::clone(&service)).with_principal(Principal::local_admin()),
        None,
    );
    let unwired_get = unwired_admin
        .clone()
        .oneshot(
            Request::get("/api/v1/documents/index-rollbacks/classification")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(unwired_get.status(), StatusCode::BAD_REQUEST);

    let public_app = build_router(
        ApiState::new(Arc::clone(&service))
            .with_classification(Arc::new(StubClassification::new())),
        None,
    );
    let (status, _, initial): (_, _, DocumentClassification) = response_json(
        public_app
            .clone()
            .oneshot(
                Request::get("/api/v1/documents/index-rollbacks/classification")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(initial.source, ClassificationSource::Rule);
    assert!(!initial.manual_locked);

    let forbidden_doc = public_app
        .clone()
        .oneshot(
            Request::get("/api/v1/documents/private-secret/classification")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(forbidden_doc.status(), StatusCode::FORBIDDEN);

    let missing_doc = public_app
        .clone()
        .oneshot(
            Request::get("/api/v1/documents/missing-doc/classification")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(missing_doc.status(), StatusCode::NOT_FOUND);

    let oversized_body = vec![b'a'; 3 * 1024 * 1024];
    for req in [
        Request::patch("/api/v1/documents/index-rollbacks/classification")
            .body(Body::empty())
            .unwrap(),
        Request::patch("/api/v1/documents/%FF/classification")
            .body(Body::empty())
            .unwrap(),
        Request::patch("/api/v1/documents/index-rollbacks/classification")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(oversized_body))
            .unwrap(),
    ] {
        let resp = public_app.clone().oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    }

    let admin_app = build_router(
        ApiState::new(service)
            .with_principal(Principal::local_admin())
            .with_classification(Arc::new(StubClassification::new())),
        None,
    );

    let bad_origin = admin_app
        .clone()
        .oneshot(
            Request::patch("/api/v1/documents/index-rollbacks/classification")
                .header(header::ORIGIN, "https://evil.example")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bad_origin.status(), StatusCode::FORBIDDEN);

    let cross_site = admin_app
        .clone()
        .oneshot(
            Request::patch("/api/v1/documents/index-rollbacks/classification")
                .header("sec-fetch-site", "cross-site")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(cross_site.status(), StatusCode::FORBIDDEN);

    for req in [
        Request::patch("/api/v1/documents/index-rollbacks/classification")
            .body(Body::from("{}"))
            .unwrap(),
        Request::patch("/api/v1/documents/index-rollbacks/classification")
            .header(header::CONTENT_TYPE, "text/plain")
            .body(Body::from("{}"))
            .unwrap(),
        Request::patch("/api/v1/documents/index-rollbacks/classification")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from("{invalid"))
            .unwrap(),
        Request::patch("/api/v1/documents/index-rollbacks/classification")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(r#"{"category":"unknown"}"#))
            .unwrap(),
    ] {
        let resp = admin_app.clone().oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    let (status, _, updated): (_, _, DocumentClassification) = response_json(
        admin_app
            .clone()
            .oneshot(
                Request::patch("/api/v1/documents/index-rollbacks/classification")
                    .header(header::ORIGIN, "http://127.0.0.1:5173")
                    .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
                    .body(Body::from(
                        r#"{"category":"manual","summary":"수동 요약","tags":["ops","OPS"],"keywords":["runbook"]}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(updated.source, ClassificationSource::Manual);
    assert!(updated.manual_locked);
    assert_eq!(updated.category, "manual");
    assert_eq!(updated.summary, "수동 요약");
    assert_eq!(updated.tags, vec!["ops"]);
    assert_eq!(updated.keywords, vec!["runbook"]);

    let (status, _, reset): (_, _, DocumentClassification) = response_json(
        admin_app
            .oneshot(
                Request::patch("/api/v1/documents/index-rollbacks/classification")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(r#"{"reset_to_rule":true}"#))
                    .unwrap(),
            )
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(reset.source, ClassificationSource::Rule);
    assert!(!reset.manual_locked);
    assert_eq!(reset.category, "design");
}

struct StubSources {
    sources: Mutex<Vec<RegisteredSource>>,
}

impl StubSources {
    fn new() -> Self {
        Self {
            sources: Mutex::new(vec![RegisteredSource {
                source_id: "local-markdown".to_string(),
                label: "Local Markdown".to_string(),
                path: "/tmp/env-docs".to_string(),
                default_category: "guide".to_string(),
                default_tag: "docs".to_string(),
                managed_by_env: true,
                created_at_ms: 100,
                last_synced_at_ms: Some(110),
                document_count: 2,
                last_error: None,
            }]),
        }
    }
}

impl SourceRegistryPort for StubSources {
    fn list_sources(&self) -> Result<SourceListResponse, dok3_core::Dok3Error> {
        Ok(SourceListResponse {
            sources: self.sources.lock().unwrap().clone(),
        })
    }

    fn register_source(
        &self,
        req: &CreateSourceRequest,
    ) -> Result<RegisteredSource, dok3_core::Dok3Error> {
        let source_id = validate_source_id_slug(&req.source_id)?;
        let label = req.label.trim();
        if label.is_empty() {
            return Err(dok3_core::Dok3Error::InvalidInput(
                "source label must not be empty".to_string(),
            ));
        }
        let default_category = match req.default_category.as_deref() {
            Some(c) if !c.trim().is_empty() => validate_classification_category(c)?,
            _ => "guide".to_string(),
        };
        let mut guard = self.sources.lock().unwrap();
        if guard.iter().any(|s| s.source_id == source_id) {
            return Err(dok3_core::Dok3Error::InvalidInput(format!(
                "source '{source_id}' already exists"
            )));
        }
        let entry = RegisteredSource {
            source_id,
            label: label.to_string(),
            path: req.path.trim().to_string(),
            default_category,
            default_tag: req
                .default_tag
                .as_deref()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .unwrap_or("docs")
                .to_string(),
            managed_by_env: false,
            created_at_ms: 200,
            last_synced_at_ms: Some(210),
            document_count: 1,
            last_error: None,
        };
        guard.push(entry.clone());
        Ok(entry)
    }

    fn delete_source(&self, source_id: &str) -> Result<DeleteSourceResponse, dok3_core::Dok3Error> {
        let mut guard = self.sources.lock().unwrap();
        let Some(pos) = guard.iter().position(|s| s.source_id == source_id) else {
            return Err(dok3_core::Dok3Error::NotFound(format!(
                "source '{source_id}' not found"
            )));
        };
        if guard[pos].managed_by_env {
            return Err(dok3_core::Dok3Error::InvalidInput(
                "cannot delete environment-managed source".to_string(),
            ));
        }
        guard.remove(pos);
        Ok(DeleteSourceResponse {
            source_id: source_id.to_string(),
            deleted_documents: 1,
            active_generation: Some("gen-000002-seq-000004".to_string()),
            active_epoch: 2,
        })
    }
}

#[tokio::test]
async fn source_routes_enforce_admin_guards_and_crud() {
    let repo = Arc::new(InMemorySearchRepository::from_seed(None));
    let service = Arc::new(SearchService::new(repo.clone(), repo));

    let public_app = build_router(
        ApiState::new(Arc::clone(&service)).with_sources(Arc::new(StubSources::new())),
        None,
    );
    let oversized_body = vec![b'a'; 3 * 1024 * 1024];
    for req in [
        Request::get("/api/v1/sources").body(Body::empty()).unwrap(),
        Request::post("/api/v1/sources")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(oversized_body))
            .unwrap(),
        Request::delete("/api/v1/sources/%FF")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::empty())
            .unwrap(),
    ] {
        let resp = public_app.clone().oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    }

    let admin_unwired = build_router(
        ApiState::new(Arc::clone(&service)).with_principal(Principal::local_admin()),
        None,
    );
    let unwired_resp = admin_unwired
        .oneshot(Request::get("/api/v1/sources").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(unwired_resp.status(), StatusCode::BAD_REQUEST);

    let admin_app = build_router(
        ApiState::new(service)
            .with_principal(Principal::local_admin())
            .with_sources(Arc::new(StubSources::new())),
        None,
    );

    let bad_origin = admin_app
        .clone()
        .oneshot(
            Request::post("/api/v1/sources")
                .header(header::ORIGIN, "https://evil.example")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bad_origin.status(), StatusCode::FORBIDDEN);

    let cross_site = admin_app
        .clone()
        .oneshot(
            Request::delete("/api/v1/sources/team-notes")
                .header("sec-fetch-site", "cross-site")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(cross_site.status(), StatusCode::FORBIDDEN);

    let missing_delete_ct = admin_app
        .clone()
        .oneshot(
            Request::delete("/api/v1/sources/team-notes")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(missing_delete_ct.status(), StatusCode::BAD_REQUEST);

    let (status, _, initial_list): (_, _, SourceListResponse) = response_json(
        admin_app
            .clone()
            .oneshot(Request::get("/api/v1/sources").body(Body::empty()).unwrap())
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(initial_list.sources.len(), 1);
    assert_eq!(initial_list.sources[0].source_id, "local-markdown");

    let (status, _, created): (_, _, RegisteredSource) = response_json(
        admin_app
            .clone()
            .oneshot(
                Request::post("/api/v1/sources")
                    .header(header::ORIGIN, "http://127.0.0.1:5173")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        r#"{"source_id":"team-notes","label":"Team Notes","path":"/tmp/team","default_category":"manual","default_tag":"ops"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(created.source_id, "team-notes");
    assert_eq!(created.default_category, "manual");
    assert!(!created.managed_by_env);

    let delete_env = admin_app
        .clone()
        .oneshot(
            Request::delete("/api/v1/sources/local-markdown")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(delete_env.status(), StatusCode::BAD_REQUEST);

    let (status, _, deleted): (_, _, DeleteSourceResponse) = response_json(
        admin_app
            .clone()
            .oneshot(
                Request::delete("/api/v1/sources/team-notes")
                    .header(header::ORIGIN, "http://127.0.0.1:5173")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(deleted.source_id, "team-notes");
    assert_eq!(deleted.deleted_documents, 1);

    let delete_missing = admin_app
        .oneshot(
            Request::delete("/api/v1/sources/team-notes")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(delete_missing.status(), StatusCode::NOT_FOUND);
}
