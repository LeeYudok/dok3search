use std::sync::Arc;

use axum::body::Body;
use axum::http::{HeaderMap, Method, Request, StatusCode, header};
use dok3_agent::{
    JSONRPC_HEADER_MISMATCH, JSONRPC_VERSION_MISMATCH, MAX_JSONRPC_LINE_BYTES, MCP_METHOD_HEADER,
    MCP_NAME_HEADER, MCP_PROTOCOL_VERSION_HEADER, McpServer, build_mcp_router,
    build_mcp_router_with_auth,
};
use dok3_core::Dok3Error;
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
            topic: None,
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
            topic: None,
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

    // 2026-07-28 stateless discovery + required supportedVersions / resultType fields
    let discover_req = Request::builder()
        .method(Method::POST)
        .uri("/mcp")
        .header(header::CONTENT_TYPE, "application/json")
        .header(MCP_PROTOCOL_VERSION_HEADER, "2026-07-28")
        .header(MCP_METHOD_HEADER, "server/discover")
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
    assert_eq!(discover_json["result"]["resultType"], "complete");
    assert!(
        discover_json["result"]["supportedVersions"]
            .as_array()
            .unwrap()
            .contains(&json!("2026-07-28"))
    );
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

    // Unsupported MCP-Protocol-Version header -> 400 + -32022 with data.supported & data.requested
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
    assert_eq!(bad_ver_json["error"]["data"]["requested"], "1999-01-01");
    assert!(
        bad_ver_json["error"]["data"]["supported"]
            .as_array()
            .unwrap()
            .contains(&json!("2026-07-28"))
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
async fn streamable_http_rejects_header_body_mismatches_and_enforces_per_request_acl() {
    // Even if constructed from a privileged stdio McpServer, default build_mcp_router stays anonymous_public.
    let privileged_server = fixture_server(Principal {
        subject: "local-admin".to_string(),
        allowed_scopes: vec![AclScope::Public, AclScope::internal("security")],
        is_admin: true,
    });
    let anon_app = build_mcp_router(privileged_server.clone());

    let denied_doc = Request::builder()
        .method(Method::POST)
        .uri("/mcp")
        .body(Body::from(
            r#"{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"kb_get_document","arguments":{"document_id":"sec-doc"}}}"#,
        ))
        .unwrap();
    let denied_resp = anon_app.clone().oneshot(denied_doc).await.unwrap();
    assert_eq!(denied_resp.status(), StatusCode::OK);
    let denied_json = read_json(denied_resp).await;
    assert_eq!(denied_json["result"]["isError"], true);
    assert_eq!(
        denied_json["result"]["structuredContent"]["error"]["code"],
        "FORBIDDEN"
    );

    // Authenticated per-request resolver grants internal access only to valid bearer token,
    // failing closed (401/403) otherwise.
    let auth_app = build_mcp_router_with_auth(
        privileged_server,
        Arc::new(|headers: &HeaderMap| {
            match headers
                .get(header::AUTHORIZATION)
                .and_then(|v| v.to_str().ok())
            {
                Some("Bearer sec-token") => Ok(Principal {
                    subject: "sec-user".to_string(),
                    allowed_scopes: vec![AclScope::Public, AclScope::internal("security")],
                    is_admin: false,
                }),
                Some(_) => Err(Dok3Error::Forbidden("invalid bearer token".to_string())),
                None => Err(Dok3Error::InvalidInput("missing authorization".to_string())),
            }
        }),
    );

    let unauth_req = Request::builder()
        .method(Method::POST)
        .uri("/mcp")
        .body(Body::from(r#"{"jsonrpc":"2.0","id":2,"method":"ping"}"#))
        .unwrap();
    assert_eq!(
        auth_app.clone().oneshot(unauth_req).await.unwrap().status(),
        StatusCode::UNAUTHORIZED
    );

    let bad_tok_req = Request::builder()
        .method(Method::POST)
        .uri("/mcp")
        .header(header::AUTHORIZATION, "Bearer wrong")
        .body(Body::from(r#"{"jsonrpc":"2.0","id":3,"method":"ping"}"#))
        .unwrap();
    assert_eq!(
        auth_app
            .clone()
            .oneshot(bad_tok_req)
            .await
            .unwrap()
            .status(),
        StatusCode::FORBIDDEN
    );

    let sec_req = Request::builder()
        .method(Method::POST)
        .uri("/mcp")
        .header(header::AUTHORIZATION, "Bearer sec-token")
        .header(MCP_PROTOCOL_VERSION_HEADER, "2026-07-28")
        .header(MCP_METHOD_HEADER, "tools/call")
        .header(MCP_NAME_HEADER, "kb_get_document")
        .body(Body::from(
            r#"{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"kb_get_document","arguments":{"document_id":"sec-doc"},"_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28"}}}"#,
        ))
        .unwrap();
    let sec_resp = auth_app.oneshot(sec_req).await.unwrap();
    assert_eq!(sec_resp.status(), StatusCode::OK);
    let sec_json = read_json(sec_resp).await;
    assert_eq!(sec_json["result"]["isError"], false);
    assert_eq!(sec_json["result"]["structuredContent"]["id"], "sec-doc");

    // Header-body mismatches (MCP-Protocol-Version vs _meta, Mcp-Method vs body.method, Mcp-Name vs params.name)
    for (hdr_k, hdr_v, body_str) in [
        (
            MCP_PROTOCOL_VERSION_HEADER,
            "2025-11-25",
            r#"{"jsonrpc":"2.0","id":5,"method":"tools/list","params":{"_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28"}}}"#,
        ),
        (
            MCP_PROTOCOL_VERSION_HEADER,
            "2026-07-28",
            r#"{"jsonrpc":"2.0","id":55,"method":"server/discover","params":{"protocolVersion":"2025-11-25","_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28"}}}"#,
        ),
        (
            MCP_METHOD_HEADER,
            "tools/list",
            r#"{"jsonrpc":"2.0","id":6,"method":"tools/call","params":{"name":"kb_list_categories"}}"#,
        ),
        (
            MCP_NAME_HEADER,
            "kb_search",
            r#"{"jsonrpc":"2.0","id":7,"method":"tools/call","params":{"name":"kb_get_document","arguments":{"document_id":"pub-doc"}}}"#,
        ),
    ] {
        let req = Request::builder()
            .method(Method::POST)
            .uri("/mcp")
            .header(hdr_k, hdr_v)
            .body(Body::from(body_str))
            .unwrap();
        let resp = anon_app.clone().oneshot(req).await.unwrap();
        assert_eq!(
            resp.status(),
            StatusCode::BAD_REQUEST,
            "mismatch {hdr_k}={hdr_v}"
        );
        let json_body = read_json(resp).await;
        assert_eq!(json_body["error"]["code"], json!(JSONRPC_HEADER_MISMATCH));
    }

    // Loopback Origin guard & 413 payload limit still enforced
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
            .body(Body::from(r#"{"jsonrpc":"2.0","id":8,"method":"ping"}"#))
            .unwrap();
        assert_eq!(
            anon_app.clone().oneshot(req).await.unwrap().status(),
            StatusCode::FORBIDDEN
        );
    }

    let oversized = format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":9,\"method\":\"ping\",\"pad\":\"{}\"}}",
        "x".repeat(MAX_JSONRPC_LINE_BYTES + 128)
    );
    let oversized_resp = anon_app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/mcp")
                .body(Body::from(oversized))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(oversized_resp.status(), StatusCode::PAYLOAD_TOO_LARGE);
}
