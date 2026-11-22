//! MCP runtime wiring for the `dok3search` binary.
//!
//! - HTTP: mounts the `dok3-agent` Streamable HTTP router (`POST /mcp`) on the same Axum app as
//!   `/api/v1/*` and `/health/ready`, sharing one [`SearchService`].
//! - stdio: `--mcp-stdio` serves newline-delimited JSON-RPC on stdin/stdout instead of HTTP.
//!
//! Both transports bind every caller to [`Principal::anonymous_public()`] (fail-closed) until an
//! authentication layer exists; neither inherits a privileged default principal.

use std::path::PathBuf;
use std::sync::Arc;

use dok3_agent::{McpServer, build_mcp_router};
use dok3_api::{ApiState, Router, build_router_with_extra};
use dok3_domain::Principal;
use dok3_search::SearchService;

/// CLI flag selecting stdio MCP mode instead of the HTTP server.
pub const MCP_STDIO_FLAG: &str = "--mcp-stdio";

/// Parses process arguments (excluding `argv[0]`). Returns `true` when stdio MCP mode is requested.
/// Unknown arguments are rejected so typos never silently fall back to HTTP mode.
pub fn stdio_requested<I>(args: I) -> Result<bool, String>
where
    I: IntoIterator<Item = String>,
{
    let mut stdio = false;
    for arg in args {
        if arg == MCP_STDIO_FLAG {
            stdio = true;
        } else {
            return Err(format!(
                "unknown argument '{arg}' (supported: {MCP_STDIO_FLAG})"
            ));
        }
    }
    Ok(stdio)
}

/// Builds the anonymous-public MCP server over the shared search service.
pub fn anonymous_server(search: &Arc<SearchService>) -> McpServer {
    McpServer::new(Arc::clone(search), Principal::anonymous_public())
}

/// Builds the full HTTP app: REST API, health, optional SPA, and `/mcp`, all sharing `state.search`.
pub fn build_app(state: ApiState, static_dir: Option<PathBuf>) -> Router {
    let mcp = build_mcp_router(Arc::new(anonymous_server(&state.search)));
    build_router_with_extra(state, static_dir, Some(mcp))
}

/// Serves MCP over stdin/stdout until EOF. Diagnostics must go to stderr only.
pub async fn serve_stdio(search: Arc<SearchService>) -> Result<(), String> {
    let server = anonymous_server(&search);
    tokio::task::spawn_blocking(move || server.serve_stdio())
        .await
        .map_err(|err| format!("MCP stdio task failed: {err}"))?
        .map_err(|err| format!("MCP stdio I/O error: {err}"))
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use std::sync::Arc;

    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode, header};
    use dok3_agent::MCP_PROTOCOL_VERSION_HEADER;
    use dok3_api::ApiState;
    use dok3_domain::{AclScope, Document, DocumentSection, RevisionId, SectionId, SourceId};
    use dok3_search::{InMemorySearchRepository, SearchService};
    use http_body_util::BodyExt;
    use serde_json::{Value, json};
    use tower::ServiceExt;

    use super::{anonymous_server, build_app, stdio_requested};

    fn doc(id: &str, title: &str, body: &str, acl_scope: AclScope) -> Document {
        Document {
            id: id.into(),
            revision_id: RevisionId::new("rev-1"),
            source_id: SourceId::new("src"),
            source: "테스트".to_string(),
            title: title.to_string(),
            summary: title.to_string(),
            category: "guide".to_string(),
            topic: None,
            tag: "Rust".to_string(),
            tags: vec!["Rust".to_string()],
            keywords: vec![],
            language: "ko".to_string(),
            version: "1.0".to_string(),
            date: Some("2026-11-22".to_string()),
            source_uri: format!("test://{id}"),
            sample_path: None,
            acl_scope,
            content_hash: format!("hash-{id}"),
            body: body.to_string(),
            markdown: None,
            sections: vec![DocumentSection {
                section_id: SectionId::new("s1"),
                heading: title.to_string(),
                anchor: "s1".to_string(),
                order: 1,
                line_start: 1,
                line_end: 2,
                body: body.to_string(),
            }],
        }
    }

    fn fixture_search() -> Arc<SearchService> {
        let docs = vec![
            doc(
                "pub-doc",
                "Rust 소유권 입문",
                "소유권 이동 규칙 설명",
                AclScope::Public,
            ),
            doc(
                "sec-doc",
                "대외비 소유권 장애 조치",
                "소유권 비밀 복구 절차",
                AclScope::internal("security"),
            ),
        ];
        let repo = Arc::new(InMemorySearchRepository::new(docs, vec![]));
        Arc::new(SearchService::new(repo.clone(), repo))
    }

    fn app_with_static(static_dir: Option<std::path::PathBuf>) -> axum::Router {
        build_app(ApiState::new(fixture_search()), static_dir)
    }

    fn mcp_post(body: Value) -> Request<Body> {
        Request::builder()
            .method(Method::POST)
            .uri("/mcp")
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::ACCEPT, "application/json, text/event-stream")
            .body(Body::from(body.to_string()))
            .unwrap()
    }

    async fn read_json(resp: axum::response::Response) -> Value {
        let bytes = resp.into_body().collect().await.unwrap().to_bytes();
        serde_json::from_slice(&bytes).unwrap()
    }

    fn assert_security_headers(resp: &axum::response::Response) {
        let headers = resp.headers();
        assert_eq!(headers[header::X_CONTENT_TYPE_OPTIONS], "nosniff");
        assert_eq!(headers[header::X_FRAME_OPTIONS], "DENY");
        assert_eq!(headers[header::REFERRER_POLICY], "no-referrer");
        assert!(
            headers[header::CONTENT_SECURITY_POLICY]
                .to_str()
                .unwrap()
                .contains("frame-ancestors 'none'")
        );
    }

    fn hit_ids(result: &Value) -> Vec<String> {
        result["result"]["structuredContent"]["hits"]
            .as_array()
            .unwrap()
            .iter()
            .map(|hit| hit["document_id"].as_str().unwrap().to_string())
            .collect()
    }

    #[test]
    fn cli_parses_mcp_stdio_flag_and_rejects_unknown_args() {
        assert_eq!(stdio_requested(Vec::<String>::new()), Ok(false));
        assert_eq!(stdio_requested(["--mcp-stdio".to_string()]), Ok(true));
        assert!(stdio_requested(["--mcp".to_string()]).is_err());
        let extra = ["--mcp-stdio".to_string(), "x".to_string()];
        assert!(stdio_requested(extra).is_err());
    }

    #[tokio::test]
    async fn mcp_initialize_list_and_kb_search_over_shared_app() {
        let app = app_with_static(None);

        let init = app
            .clone()
            .oneshot(mcp_post(json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": {
                    "protocolVersion": "2025-11-25",
                    "capabilities": {},
                    "clientInfo": { "name": "test", "version": "0" }
                }
            })))
            .await
            .unwrap();
        assert_eq!(init.status(), StatusCode::OK);
        assert_security_headers(&init);
        assert_eq!(init.headers()[MCP_PROTOCOL_VERSION_HEADER], "2025-11-25");
        let init_json = read_json(init).await;
        assert_eq!(init_json["result"]["protocolVersion"], "2025-11-25");
        assert_eq!(init_json["result"]["serverInfo"]["name"], "dok3search-mcp");

        let list = app
            .clone()
            .oneshot(mcp_post(json!({
                "jsonrpc": "2.0",
                "id": 2,
                "method": "tools/list"
            })))
            .await
            .unwrap();
        assert_eq!(list.status(), StatusCode::OK);
        let list_json = read_json(list).await;
        let names: Vec<&str> = list_json["result"]["tools"]
            .as_array()
            .unwrap()
            .iter()
            .map(|tool| tool["name"].as_str().unwrap())
            .collect();
        assert!(names.contains(&"kb_search"), "tools: {names:?}");

        let call = app
            .oneshot(mcp_post(json!({
                "jsonrpc": "2.0",
                "id": 3,
                "method": "tools/call",
                "params": { "name": "kb_search", "arguments": { "query": "소유권" } }
            })))
            .await
            .unwrap();
        assert_eq!(call.status(), StatusCode::OK);
        let call_json = read_json(call).await;
        assert_eq!(call_json["result"]["isError"], false);
        // Anonymous HTTP caller sees only public documents (fail-closed ACL).
        assert_eq!(hit_ids(&call_json), vec!["pub-doc".to_string()]);
    }

    #[tokio::test]
    async fn mcp_get_is_405_and_foreign_origin_is_403() {
        let app = app_with_static(None);

        let get = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/mcp")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(get.status(), StatusCode::METHOD_NOT_ALLOWED);
        assert_eq!(get.headers()[header::ALLOW], "POST");
        assert_security_headers(&get);

        let mut foreign = mcp_post(json!({ "jsonrpc": "2.0", "id": 1, "method": "ping" }));
        foreign
            .headers_mut()
            .insert(header::ORIGIN, "https://evil.example".parse().unwrap());
        let foreign_resp = app.clone().oneshot(foreign).await.unwrap();
        assert_eq!(foreign_resp.status(), StatusCode::FORBIDDEN);
        assert_security_headers(&foreign_resp);

        let mut loopback = mcp_post(json!({ "jsonrpc": "2.0", "id": 1, "method": "ping" }));
        loopback
            .headers_mut()
            .insert(header::ORIGIN, "http://127.0.0.1:8080".parse().unwrap());
        let loopback_resp = app.oneshot(loopback).await.unwrap();
        assert_eq!(loopback_resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn rest_api_and_health_coexist_with_mcp() {
        let app = app_with_static(None);

        let health = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/health/ready")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(health.status(), StatusCode::OK);
        assert_security_headers(&health);
        let health_json = read_json(health).await;
        assert_eq!(health_json["status"], "ready");

        let search = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/v1/search")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(json!({ "query": "소유권" }).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(search.status(), StatusCode::OK);
        let search_json = read_json(search).await;
        let ids: Vec<&str> = search_json["hits"]
            .as_array()
            .unwrap()
            .iter()
            .map(|hit| hit["document_id"].as_str().unwrap())
            .collect();
        assert_eq!(ids, vec!["pub-doc"]);
    }

    #[tokio::test]
    async fn mcp_route_wins_over_spa_fallback() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("index.html"), "<html>spa</html>").unwrap();
        let app = app_with_static(Some(dir.path().to_path_buf()));

        let get = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/mcp")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(get.status(), StatusCode::METHOD_NOT_ALLOWED);

        let ping_req = mcp_post(json!({ "jsonrpc": "2.0", "id": 9, "method": "ping" }));
        let ping = app.clone().oneshot(ping_req).await.unwrap();
        assert_eq!(ping.status(), StatusCode::OK);
        assert_eq!(read_json(ping).await["id"], 9);

        let spa = app
            .oneshot(
                Request::builder()
                    .uri("/some/client/route")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(spa.status(), StatusCode::OK);
        let html = spa.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&html[..], b"<html>spa</html>");
    }

    #[test]
    fn stdio_server_is_anonymous_public() {
        let server = anonymous_server(&fixture_search());
        let input = concat!(
            r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25"}}"#,
            "\n",
            r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
            "\n",
            r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"kb_search","arguments":{"query":"소유권"}}}"#,
            "\n",
        );
        let mut out = Vec::new();
        server
            .serve_reader_writer(Cursor::new(input.as_bytes()), &mut out)
            .unwrap();
        let lines: Vec<Value> = String::from_utf8(out)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        assert_eq!(lines.len(), 2, "notification must not produce output");
        assert_eq!(lines[0]["result"]["protocolVersion"], "2025-11-25");
        assert_eq!(hit_ids(&lines[1]), vec!["pub-doc".to_string()]);
    }
}
