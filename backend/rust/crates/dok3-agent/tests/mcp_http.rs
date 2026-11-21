use std::sync::Arc;

use axum::body::Body;
use axum::http::{Method, Request, StatusCode, header};
use dok3_agent::{
    JSONRPC_VERSION_MISMATCH, MAX_JSONRPC_LINE_BYTES, MCP_PROTOCOL_VERSION_HEADER, McpServer,
    build_mcp_router,
};
use dok3_domain::{
    AclScope, Document, DocumentSection, Principal, RevisionId, SectionId, SourceId,
};
use dok3_search::{InMemorySearchRepository, SearchService};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tower::ServiceExt;

fn fixture_server(principal: Principal) -> Arc<McpServer> {
    let docs = vec![
        Document {
            id: "pub-doc".into(),
            revision_id: RevisionId::new("rev-1"),
            source_id: SourceId::new("src"),
            source: "공개 가이드".to_string(),
            title: "Rust 소유권 입문".to_string(),
            summary: "소유권과 빌림 기본".to_string(),
            category: "guide".to_string(),
            tag: "Rust".to_string(),
            tags: vec!["Rust".to_string()],
            keywords: vec![],
            language: "ko".to_string(),
            version: "1.0".to_string(),
            date: Some("2026-11-22".to_string()),
            source_uri: "test://pub-doc".to_string(),
            sample_path: None,
            acl_scope: AclScope::Public,
            content_hash: "h1".to_string(),
            body: "소유권 이동 규칙 설명".to_string(),
            markdown: None,
            sections: vec![DocumentSection {
                section_id: SectionId::new("s1"),
                heading: "소유권".to_string(),
                anchor: "s1".to_string(),
                order: 1,
                line_start: 1,
                line_end: 2,
                body: "소유권 이동 규칙 설명".to_string(),
            }],
        },
        Document {
            id: "sec-doc".into(),
            revision_id: RevisionId::new("rev-2"),
            source_id: SourceId::new("src"),
            source: "내부 보안".to_string(),
            title: "대외비 장애 조치".to_string(),
            summary: "내부 전용".to_string(),
            category: "policy".to_string(),
            tag: "보안".to_string(),
            tags: vec!["보안".to_string()],
            keywords: vec![],
            language: "ko".to_string(),
            version: "1.0".to_string(),
            date: Some("2026-11-22".to_string()),
            source_uri: "test://sec-doc".to_string(),
            sample_path: None,
            acl_scope: AclScope::internal("security"),
            content_hash: "h2".to_string(),
            body: "비밀 복구 절차".to_string(),
            markdown: None,
            sections: vec![],
        },
    ];
    let repo = Arc::new(InMemorySearchRepository::new(docs, vec![]));
    let svc = Arc::new(SearchService::new(repo.clone(), repo));
    Arc::new(McpServer::new(svc, principal))
}

async fn read_json(resp: axum::response::Response) -> Value {
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

#[tokio::test]
async fn streamable_http_supports_2026_stateless_and_2025_initialize_matrix() {
    let app = build_mcp_router(fixture_server(Principal::anonymous_public()));

    // 2026-07-28 stateless discovery + tool call over single POST /mcp
    let discover_req = Request::builder()
        .method(Method::POST)
        .uri("/mcp")
        .header(header::CONTENT_TYPE, "application/json")
        .header(MCP_PROTOCOL_VERSION_HEADER, "2026-07-28")
        .body(Body::from(
            r#"{"jsonrpc":"2.0","id":1,"method":"server/discover"}"#,
        ))
        .unwrap();
    let discover_resp = app.clone().oneshot(discover_req).await.unwrap();
    assert_eq!(discover_resp.status(), StatusCode::OK);
    assert_eq!(
        discover_resp
            .headers()
            .get(MCP_PROTOCOL_VERSION_HEADER)
            .unwrap(),
        "2026-07-28"
    );
    let discover_json = read_json(discover_resp).await;
    assert_eq!(
        discover_json["result"]["tools"].as_array().unwrap().len(),
        7
    );

    // 2025-11-25 initialize + notifications/initialized -> 202 Accepted + tools/call
    let init_req = Request::builder()
        .method(Method::POST)
        .uri("/mcp")
        .header(header::CONTENT_TYPE, "application/json")
        .header(MCP_PROTOCOL_VERSION_HEADER, "2025-11-25")
        .body(Body::from(
            r#"{"jsonrpc":"2.0","id":2,"method":"initialize","params":{"protocolVersion":"2025-11-25"}}"#,
        ))
        .unwrap();
    let init_resp = app.clone().oneshot(init_req).await.unwrap();
    assert_eq!(init_resp.status(), StatusCode::OK);
    assert_eq!(
        init_resp
            .headers()
            .get(MCP_PROTOCOL_VERSION_HEADER)
            .unwrap(),
        "2025-11-25"
    );

    let notif_req = Request::builder()
        .method(Method::POST)
        .uri("/mcp")
        .header(MCP_PROTOCOL_VERSION_HEADER, "2025-11-25")
        .body(Body::from(
            r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        ))
        .unwrap();
    let notif_resp = app.clone().oneshot(notif_req).await.unwrap();
    assert_eq!(notif_resp.status(), StatusCode::ACCEPTED);
    assert_eq!(
        notif_resp
            .headers()
            .get(MCP_PROTOCOL_VERSION_HEADER)
            .unwrap(),
        "2025-11-25"
    );

    // Unsupported MCP-Protocol-Version header -> 400 + -32022
    let bad_ver_req = Request::builder()
        .method(Method::POST)
        .uri("/mcp")
        .header(MCP_PROTOCOL_VERSION_HEADER, "1999-01-01")
        .body(Body::from(r#"{"jsonrpc":"2.0","id":3,"method":"ping"}"#))
        .unwrap();
    let bad_ver_resp = app.clone().oneshot(bad_ver_req).await.unwrap();
    assert_eq!(bad_ver_resp.status(), StatusCode::BAD_REQUEST);
    let bad_ver_json = read_json(bad_ver_resp).await;
    assert_eq!(
        bad_ver_json["error"]["code"],
        json!(JSONRPC_VERSION_MISMATCH)
    );

    // GET /mcp -> 405 Method Not Allowed + Allow: POST
    let get_resp = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/mcp")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(get_resp.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(get_resp.headers().get(header::ALLOW).unwrap(), "POST");
}

#[tokio::test]
async fn streamable_http_enforces_loopback_origin_body_cap_and_acl() {
    let app = build_mcp_router(fixture_server(Principal::anonymous_public()));

    // Reject non-loopback / spoofed Origin headers
    for evil_origin in [
        "https://evil.example",
        "http://127.0.0.1.evil.example",
        "http://localhost@evil.example",
        "null",
    ] {
        let req = Request::builder()
            .method(Method::POST)
            .uri("/mcp")
            .header(header::ORIGIN, evil_origin)
            .body(Body::from(r#"{"jsonrpc":"2.0","id":1,"method":"ping"}"#))
            .unwrap();
        let resp = app.clone().oneshot(req).await.unwrap();
        assert_eq!(
            resp.status(),
            StatusCode::FORBIDDEN,
            "origin: {evil_origin}"
        );
    }

    // Accept loopback Origin and execute ACL-enforced document read
    let allowed_req = Request::builder()
        .method(Method::POST)
        .uri("/mcp")
        .header(header::ORIGIN, "http://127.0.0.1:18443")
        .body(Body::from(
            r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"kb_get_document","arguments":{"document_id":"sec-doc"}}}"#,
        ))
        .unwrap();
    let allowed_resp = app.clone().oneshot(allowed_req).await.unwrap();
    assert_eq!(allowed_resp.status(), StatusCode::OK);
    let body = read_json(allowed_resp).await;
    assert_eq!(body["result"]["isError"], true);
    assert_eq!(
        body["result"]["structuredContent"]["error"]["code"],
        "FORBIDDEN"
    );

    // Oversized payload hits Axum DefaultBodyLimit -> 413 Payload Too Large
    let oversized = format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":3,\"method\":\"ping\",\"pad\":\"{}\"}}",
        "x".repeat(MAX_JSONRPC_LINE_BYTES + 128)
    );
    let oversized_req = Request::builder()
        .method(Method::POST)
        .uri("/mcp")
        .body(Body::from(oversized))
        .unwrap();
    let oversized_resp = app.oneshot(oversized_req).await.unwrap();
    assert_eq!(oversized_resp.status(), StatusCode::PAYLOAD_TOO_LARGE);
}
