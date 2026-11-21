//! MCP Streamable HTTP (`POST /mcp`) transport adapter with version, header-body consistency,
//! loopback `Origin` guard, and per-request [`Principal`] authorization.

use std::sync::Arc;

use axum::Router;
use axum::body::Bytes;
use axum::extract::{DefaultBodyLimit, State};
use axum::http::{HeaderMap, HeaderName, HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use dok3_core::Dok3Error;
use dok3_domain::Principal;
use serde_json::{Value, json};

use crate::protocol::{
    DEFAULT_PROTOCOL_VERSION, JSONRPC_HEADER_MISMATCH, JSONRPC_INVALID_PARAMS,
    JSONRPC_INVALID_REQUEST, JSONRPC_PARSE_ERROR, JsonRpcResponse, SUPPORTED_PROTOCOL_VERSIONS,
    version_mismatch_error,
};
use crate::server::{MAX_JSONRPC_LINE_BYTES, McpServer};

/// Canonical HTTP header name carrying the MCP specification revision.
pub const MCP_PROTOCOL_VERSION_HEADER: &str = "mcp-protocol-version";
/// MCP `2026-07-28` routing header mirroring the JSON-RPC `method`.
pub const MCP_METHOD_HEADER: &str = "mcp-method";
/// MCP `2026-07-28` routing header mirroring `params.name` or `params.uri`.
pub const MCP_NAME_HEADER: &str = "mcp-name";

/// Resolves the caller [`Principal`] for each incoming HTTP request.
pub type PrincipalResolver = Arc<dyn Fn(&HeaderMap) -> Result<Principal, Dok3Error> + Send + Sync>;

#[derive(Clone)]
struct McpHttpState {
    server: Arc<McpServer>,
    resolve_principal: PrincipalResolver,
}

/// Builds a Streamable HTTP router (`/mcp`) binding every unauthenticated HTTP caller
/// strictly to [`Principal::anonymous_public()`] (never inherits privileged stdio defaults).
pub fn build_mcp_router(server: Arc<McpServer>) -> Router {
    let anon: PrincipalResolver = Arc::new(|_| Ok(Principal::anonymous_public()));
    build_mcp_router_with_auth(server, anon)
}

/// Builds a Streamable HTTP router (`/mcp`) evaluating per-request caller credentials via `resolve_principal`.
/// Rejections map fail-closed to HTTP `403` (`Dok3Error::Forbidden`) or `401 Unauthorized`.
pub fn build_mcp_router_with_auth(
    server: Arc<McpServer>,
    resolve_principal: PrincipalResolver,
) -> Router {
    let state = McpHttpState {
        server,
        resolve_principal,
    };
    Router::new()
        .route("/mcp", post(handle_post_mcp).get(handle_get_mcp))
        .layer(DefaultBodyLimit::max(MAX_JSONRPC_LINE_BYTES))
        .with_state(state)
}

async fn handle_get_mcp(headers: HeaderMap) -> Response {
    if let Err(resp) = validate_loopback_origin(&headers) {
        return *resp;
    }
    let header_ver = match parse_protocol_version_header(&headers) {
        Ok(Some(ver)) => ver,
        Ok(None) => DEFAULT_PROTOCOL_VERSION,
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
    State(state): State<McpHttpState>,
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
    let effective_hdr_ver = header_ver.unwrap_or(DEFAULT_PROTOCOL_VERSION);

    let principal = match (state.resolve_principal)(&headers) {
        Ok(p) => p,
        Err(err @ Dok3Error::Forbidden(_)) => {
            let err_resp = JsonRpcResponse::err(
                Value::Null,
                JSONRPC_INVALID_REQUEST,
                err.to_string(),
                Some(json!({ "code": err.code() })),
            );
            return json_response(StatusCode::FORBIDDEN, &err_resp, effective_hdr_ver);
        }
        Err(err) => {
            let err_resp = JsonRpcResponse::err(
                Value::Null,
                JSONRPC_INVALID_REQUEST,
                err.to_string(),
                Some(json!({ "code": err.code() })),
            );
            return json_response(StatusCode::UNAUTHORIZED, &err_resp, effective_hdr_ver);
        }
    };

    let Ok(text) = std::str::from_utf8(&body) else {
        let err_resp = JsonRpcResponse::err(
            Value::Null,
            JSONRPC_PARSE_ERROR,
            "request body must be valid UTF-8 JSON",
            None,
        );
        return json_response(StatusCode::BAD_REQUEST, &err_resp, effective_hdr_ver);
    };

    if text.trim().is_empty() {
        let err_resp = JsonRpcResponse::err(
            Value::Null,
            JSONRPC_INVALID_REQUEST,
            "empty JSON-RPC request body",
            None,
        );
        return json_response(StatusCode::BAD_REQUEST, &err_resp, effective_hdr_ver);
    }

    if let Ok(parsed) = serde_json::from_str::<Value>(text)
        && let Err(resp) = validate_header_body_consistency(&headers, header_ver, &parsed)
    {
        return *resp;
    }

    match state.server.handle_message_for(&principal, text) {
        None => empty_response(StatusCode::ACCEPTED, effective_hdr_ver),
        Some(rpc_resp) => {
            let negotiated = rpc_resp
                .result
                .as_ref()
                .and_then(|r| r.get("protocolVersion"))
                .and_then(Value::as_str)
                .unwrap_or(effective_hdr_ver);
            json_response(StatusCode::OK, &rpc_resp, negotiated)
        }
    }
}

fn validate_header_body_consistency(
    headers: &HeaderMap,
    header_ver: Option<&str>,
    body: &Value,
) -> Result<(), Box<Response>> {
    let Some(obj) = body.as_object() else {
        return Ok(());
    };
    let req_id = obj.get("id").cloned().unwrap_or(Value::Null);
    let err_ver = header_ver.unwrap_or(DEFAULT_PROTOCOL_VERSION);

    let mismatch = |message: String| -> Box<Response> {
        let err = JsonRpcResponse::err(req_id.clone(), JSONRPC_HEADER_MISMATCH, message, None);
        Box::new(json_response(StatusCode::BAD_REQUEST, &err, err_ver))
    };

    if let Some(hdr_ver) = header_ver
        && let Some(meta_ver) = obj
            .get("params")
            .and_then(Value::as_object)
            .and_then(|p| p.get("_meta"))
            .and_then(Value::as_object)
            .and_then(|m| {
                m.get("io.modelcontextprotocol/protocolVersion")
                    .or_else(|| m.get("protocolVersion"))
            })
            .and_then(Value::as_str)
        && meta_ver != hdr_ver
    {
        return Err(mismatch(format!(
            "Header mismatch: MCP-Protocol-Version '{hdr_ver}' does not match _meta protocolVersion '{meta_ver}'"
        )));
    }

    let body_method = obj.get("method").and_then(Value::as_str);
    if let Some(raw_method) = headers.get(MCP_METHOD_HEADER) {
        let Ok(hdr_method) = raw_method.to_str().map(str::trim) else {
            return Err(mismatch("invalid Mcp-Method header encoding".to_string()));
        };
        if hdr_method.is_empty() || body_method != Some(hdr_method) {
            return Err(mismatch(format!(
                "Header mismatch: Mcp-Method '{hdr_method}' does not match body method '{}'",
                body_method.unwrap_or("<missing>")
            )));
        }
    }

    if let Some(raw_name) = headers.get(MCP_NAME_HEADER) {
        let Ok(hdr_name) = raw_name.to_str().map(str::trim) else {
            return Err(mismatch("invalid Mcp-Name header encoding".to_string()));
        };
        let allows_name = matches!(
            body_method,
            Some("tools/call" | "resources/read" | "prompts/get")
        );
        let body_name = obj
            .get("params")
            .and_then(Value::as_object)
            .and_then(|p| p.get("name").or_else(|| p.get("uri")))
            .and_then(Value::as_str);
        if hdr_name.is_empty() || !allows_name || body_name != Some(hdr_name) {
            return Err(mismatch(format!(
                "Header mismatch: Mcp-Name '{hdr_name}' does not match body target '{}'",
                body_name.unwrap_or("<missing>")
            )));
        }
    }

    Ok(())
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

fn parse_protocol_version_header(
    headers: &HeaderMap,
) -> Result<Option<&'static str>, Box<Response>> {
    let Some(raw) = headers.get(MCP_PROTOCOL_VERSION_HEADER) else {
        return Ok(None);
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
        return Ok(Some(matched));
    }
    let err = version_mismatch_error(Value::Null, ver);
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
