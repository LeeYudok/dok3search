//! MCP Streamable HTTP (`POST /mcp`) transport adapter with version and loopback Origin checks.

use std::sync::Arc;

use axum::Router;
use axum::body::Bytes;
use axum::extract::{DefaultBodyLimit, State};
use axum::http::{HeaderMap, HeaderName, HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use serde_json::{Value, json};

use crate::protocol::{
    DEFAULT_PROTOCOL_VERSION, JSONRPC_INVALID_PARAMS, JSONRPC_INVALID_REQUEST, JSONRPC_PARSE_ERROR,
    JSONRPC_VERSION_MISMATCH, JsonRpcResponse, SUPPORTED_PROTOCOL_VERSIONS,
};
use crate::server::{MAX_JSONRPC_LINE_BYTES, McpServer};

/// Canonical HTTP header name carrying the MCP specification revision.
pub const MCP_PROTOCOL_VERSION_HEADER: &str = "mcp-protocol-version";

/// Builds an Axum router exposing the Streamable HTTP endpoint at `/mcp`.
///
/// Enforces single-POST `2026-07-28` / `2025-11-25` version negotiation, loopback-only `Origin`
/// when supplied, and a [`MAX_JSONRPC_LINE_BYTES`] body cap.
pub fn build_mcp_router(server: Arc<McpServer>) -> Router {
    Router::new()
        .route("/mcp", post(handle_post_mcp).get(handle_get_mcp))
        .layer(DefaultBodyLimit::max(MAX_JSONRPC_LINE_BYTES))
        .with_state(server)
}

async fn handle_get_mcp(headers: HeaderMap) -> Response {
    if let Err(resp) = validate_loopback_origin(&headers) {
        return *resp;
    }
    let header_ver = match parse_protocol_version_header(&headers) {
        Ok(ver) => ver,
        Err(resp) => return *resp,
    };

    let body = JsonRpcResponse::err(
        Value::Null,
        JSONRPC_INVALID_REQUEST,
        "GET /mcp SSE stream is not supported in stateless Streamable HTTP mode; use POST /mcp",
        Some(json!({ "supportedMethods": ["POST"] })),
    );
    let mut resp = json_response(StatusCode::METHOD_NOT_ALLOWED, &body, header_ver);
    resp.headers_mut()
        .insert(header::ALLOW, HeaderValue::from_static("POST"));
    resp
}

async fn handle_post_mcp(
    State(server): State<Arc<McpServer>>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    if let Err(resp) = validate_loopback_origin(&headers) {
        return *resp;
    }
    let header_ver = match parse_protocol_version_header(&headers) {
        Ok(ver) => ver,
        Err(resp) => return *resp,
    };

    let Ok(text) = std::str::from_utf8(&body) else {
        let err_resp = JsonRpcResponse::err(
            Value::Null,
            JSONRPC_PARSE_ERROR,
            "request body must be valid UTF-8 JSON",
            None,
        );
        return json_response(StatusCode::BAD_REQUEST, &err_resp, header_ver);
    };

    if text.trim().is_empty() {
        let err_resp = JsonRpcResponse::err(
            Value::Null,
            JSONRPC_INVALID_REQUEST,
            "empty JSON-RPC request body",
            None,
        );
        return json_response(StatusCode::BAD_REQUEST, &err_resp, header_ver);
    }

    match server.handle_message(text) {
        None => empty_response(StatusCode::ACCEPTED, header_ver),
        Some(rpc_resp) => {
            let effective_ver = rpc_resp
                .result
                .as_ref()
                .and_then(|r| r.get("protocolVersion"))
                .and_then(Value::as_str)
                .unwrap_or(header_ver);
            json_response(StatusCode::OK, &rpc_resp, effective_ver)
        }
    }
}

fn validate_loopback_origin(headers: &HeaderMap) -> Result<(), Box<Response>> {
    let Some(raw) = headers.get(header::ORIGIN) else {
        return Ok(());
    };
    let Ok(origin) = raw.to_str() else {
        let err = JsonRpcResponse::err(
            Value::Null,
            JSONRPC_INVALID_REQUEST,
            "invalid Origin header",
            None,
        );
        return Err(Box::new(json_response(
            StatusCode::FORBIDDEN,
            &err,
            DEFAULT_PROTOCOL_VERSION,
        )));
    };
    if is_allowed_loopback_origin(origin.trim()) {
        Ok(())
    } else {
        let err = JsonRpcResponse::err(
            Value::Null,
            JSONRPC_INVALID_REQUEST,
            format!("origin '{origin}' is rejected by loopback guard"),
            None,
        );
        Err(Box::new(json_response(
            StatusCode::FORBIDDEN,
            &err,
            DEFAULT_PROTOCOL_VERSION,
        )))
    }
}

fn is_allowed_loopback_origin(origin: &str) -> bool {
    let rest = origin
        .strip_prefix("http://")
        .or_else(|| origin.strip_prefix("https://"));
    let Some(authority) = rest else {
        return false;
    };
    if authority.is_empty() || authority.contains(['/', '?', '#', '@', '\\']) {
        return false;
    }
    if let Some(port_str) = authority.strip_prefix("[::1]") {
        return port_str.is_empty()
            || port_str
                .strip_prefix(':')
                .is_some_and(|p| !p.is_empty() && p.parse::<u16>().is_ok());
    }
    let (host, port_opt) = match authority.split_once(':') {
        Some((h, p)) => (h, Some(p)),
        None => (authority, None),
    };
    let valid_host = host.eq_ignore_ascii_case("localhost") || host == "127.0.0.1";
    let valid_port = port_opt.is_none_or(|p| !p.is_empty() && p.parse::<u16>().is_ok());
    valid_host && valid_port
}

fn parse_protocol_version_header(headers: &HeaderMap) -> Result<&'static str, Box<Response>> {
    let Some(raw) = headers.get(MCP_PROTOCOL_VERSION_HEADER) else {
        return Ok(DEFAULT_PROTOCOL_VERSION);
    };
    let Ok(ver) = raw.to_str().map(str::trim) else {
        let err = JsonRpcResponse::err(
            Value::Null,
            JSONRPC_INVALID_PARAMS,
            "MCP-Protocol-Version header must be valid ASCII",
            None,
        );
        return Err(Box::new(json_response(
            StatusCode::BAD_REQUEST,
            &err,
            DEFAULT_PROTOCOL_VERSION,
        )));
    };
    if let Some(&matched) = SUPPORTED_PROTOCOL_VERSIONS.iter().find(|&&v| v == ver) {
        return Ok(matched);
    }
    let err = JsonRpcResponse::err(
        Value::Null,
        JSONRPC_VERSION_MISMATCH,
        format!("unsupported MCP-Protocol-Version '{ver}'"),
        Some(json!({ "supportedProtocolVersions": SUPPORTED_PROTOCOL_VERSIONS })),
    );
    Err(Box::new(json_response(
        StatusCode::BAD_REQUEST,
        &err,
        DEFAULT_PROTOCOL_VERSION,
    )))
}

fn apply_common_headers(resp: &mut Response, version: &str) {
    let headers = resp.headers_mut();
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    if let Ok(val) = HeaderValue::from_str(version) {
        headers.insert(HeaderName::from_static(MCP_PROTOCOL_VERSION_HEADER), val);
    }
}

fn json_response(status: StatusCode, payload: &JsonRpcResponse, version: &str) -> Response {
    let body = serde_json::to_string(payload).unwrap_or_else(|_| "{}".to_string());
    let mut resp = (status, body).into_response();
    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    apply_common_headers(&mut resp, version);
    resp
}

fn empty_response(status: StatusCode, version: &str) -> Response {
    let mut resp = status.into_response();
    apply_common_headers(&mut resp, version);
    resp
}
