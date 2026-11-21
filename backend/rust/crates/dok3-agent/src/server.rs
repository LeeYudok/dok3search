//! Stateful & stateless MCP JSON-RPC 2.0 server backed by `dok3_search::SearchService`.

use std::io::{self, BufRead, Write};
use std::str::FromStr;
use std::sync::Arc;

use dok3_core::Dok3Error;
use dok3_domain::{
    AclScope, DocumentId, Principal, SearchFilters, SearchRequest, SectionId, SectionReadRequest,
    SourceId,
};
use dok3_search::SearchService;
use serde::Serialize;
use serde_json::{Map, Value, json};

use crate::protocol::{
    DEFAULT_PROTOCOL_VERSION, JSONRPC_INVALID_PARAMS, JSONRPC_INVALID_REQUEST,
    JSONRPC_METHOD_NOT_FOUND, JSONRPC_PARSE_ERROR, JsonRpcResponse, ListCategoriesResult,
    SUPPORTED_PROTOCOL_VERSIONS, tool_definitions, version_mismatch_error,
};

/// Maximum bytes accepted for a single newline-delimited JSON-RPC message on stdio.
pub const MAX_JSONRPC_LINE_BYTES: usize = 1024 * 1024;

/// MCP stdio JSON-RPC 2.0 server delegating all use cases to [`SearchService`].
#[derive(Clone)]
pub struct McpServer {
    search: Arc<SearchService>,
    principal: Principal,
    server_name: String,
    server_version: String,
}

impl McpServer {
    /// Creates an [`McpServer`] bound to an authenticated caller [`Principal`].
    #[must_use]
    pub fn new(search: Arc<SearchService>, principal: Principal) -> Self {
        Self {
            search,
            principal,
            server_name: "dok3search-mcp".to_string(),
            server_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// Overrides the server name and version reported during discovery/initialize.
    #[must_use]
    pub fn with_server_info(mut self, name: impl Into<String>, version: impl Into<String>) -> Self {
        self.server_name = name.into();
        self.server_version = version.into();
        self
    }

    /// Processes a single JSON-RPC 2.0 line under the server's configured stdio [`Principal`].
    #[must_use]
    pub fn handle_message(&self, raw_line: &str) -> Option<JsonRpcResponse> {
        self.handle_message_for(&self.principal, raw_line)
    }

    /// Processes a single JSON-RPC 2.0 line for an explicit per-request caller [`Principal`].
    /// Returns `None` for JSON-RPC notifications.
    #[must_use]
    pub fn handle_message_for(
        &self,
        principal: &Principal,
        raw_line: &str,
    ) -> Option<JsonRpcResponse> {
        if raw_line.len() > MAX_JSONRPC_LINE_BYTES {
            return Some(JsonRpcResponse::err(
                Value::Null,
                JSONRPC_INVALID_REQUEST,
                format!("request exceeds max size of {MAX_JSONRPC_LINE_BYTES} bytes"),
                None,
            ));
        }

        let value: Value = match serde_json::from_str(raw_line) {
            Ok(v) => v,
            Err(err) => {
                return Some(JsonRpcResponse::err(
                    Value::Null,
                    JSONRPC_PARSE_ERROR,
                    format!("invalid JSON payload: {err}"),
                    None,
                ));
            }
        };

        let Some(obj) = value.as_object() else {
            return Some(JsonRpcResponse::err(
                Value::Null,
                JSONRPC_INVALID_REQUEST,
                "JSON-RPC request must be an object",
                None,
            ));
        };

        let has_id = obj.contains_key("id");
        let id = obj.get("id").cloned().unwrap_or(Value::Null);
        if has_id && !matches!(id, Value::Null | Value::String(_) | Value::Number(_)) {
            return Some(JsonRpcResponse::err(
                Value::Null,
                JSONRPC_INVALID_REQUEST,
                "JSON-RPC id must be string, number, or null",
                None,
            ));
        }

        if obj.get("jsonrpc").and_then(Value::as_str) != Some("2.0") {
            return has_id.then(|| {
                JsonRpcResponse::err(
                    id,
                    JSONRPC_INVALID_REQUEST,
                    "jsonrpc version must be '2.0'",
                    None,
                )
            });
        }

        let Some(method) = obj.get("method").and_then(Value::as_str) else {
            return has_id.then(|| {
                JsonRpcResponse::err(
                    id,
                    JSONRPC_INVALID_REQUEST,
                    "missing or non-string JSON-RPC method",
                    None,
                )
            });
        };

        if !has_id || method.starts_with("notifications/") {
            return None;
        }

        let params = match obj.get("params") {
            None | Some(Value::Null) => Map::new(),
            Some(Value::Object(map)) => map.clone(),
            Some(_) => {
                return Some(JsonRpcResponse::err(
                    id,
                    JSONRPC_INVALID_PARAMS,
                    "params must be a JSON object",
                    None,
                ));
            }
        };

        if let Some(err_resp) = check_meta_protocol_version(&id, &params) {
            return Some(err_resp);
        }

        Some(match method {
            "initialize" => self.handle_initialize(id, &params),
            "server/discover" => self.handle_discover(id, &params),
            "ping" => JsonRpcResponse::ok(id, json!({})),
            "tools/list" => JsonRpcResponse::ok(id, json!({ "tools": tool_definitions() })),
            "tools/call" => self.handle_tools_call(principal, id, &params),
            _ => JsonRpcResponse::err(
                id,
                JSONRPC_METHOD_NOT_FOUND,
                format!("method not found: {method}"),
                None,
            ),
        })
    }

    /// Runs a newline-delimited JSON-RPC stdio session with bounded per-line buffering.
    pub fn serve_reader_writer<R: BufRead, W: Write>(
        &self,
        mut reader: R,
        writer: &mut W,
    ) -> io::Result<()> {
        let mut line_buf = Vec::with_capacity(4096);
        loop {
            let line = read_bounded_line(&mut reader, &mut line_buf, MAX_JSONRPC_LINE_BYTES)?;
            match line {
                BoundedLine::Eof => return Ok(()),
                BoundedLine::TooLarge => {
                    let resp = JsonRpcResponse::err(
                        Value::Null,
                        JSONRPC_INVALID_REQUEST,
                        format!("request exceeds max size of {MAX_JSONRPC_LINE_BYTES} bytes"),
                        None,
                    );
                    write_json_line(writer, &resp)?;
                }
                BoundedLine::Line(raw) => {
                    let trimmed = raw.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    if let Some(resp) = self.handle_message(trimmed) {
                        write_json_line(writer, &resp)?;
                    }
                }
            }
        }
    }

    /// Serves MCP JSON-RPC 2.0 over process standard input and standard output.
    pub fn serve_stdio(&self) -> io::Result<()> {
        let stdin = io::stdin();
        let stdout = io::stdout();
        let reader = stdin.lock();
        let mut writer = stdout.lock();
        self.serve_reader_writer(reader, &mut writer)
    }

    fn handle_initialize(&self, id: Value, params: &Map<String, Value>) -> JsonRpcResponse {
        let requested = params.get("protocolVersion").and_then(Value::as_str);
        let negotiated = match requested {
            Some(ver) if SUPPORTED_PROTOCOL_VERSIONS.contains(&ver) => ver,
            _ => DEFAULT_PROTOCOL_VERSION,
        };
        JsonRpcResponse::ok(
            id,
            json!({
                "protocolVersion": negotiated,
                "capabilities": {
                    "tools": { "listChanged": false }
                },
                "serverInfo": {
                    "name": self.server_name,
                    "version": self.server_version,
                }
            }),
        )
    }

    fn handle_discover(&self, id: Value, params: &Map<String, Value>) -> JsonRpcResponse {
        let requested = match params.get("protocolVersion") {
            None | Some(Value::Null) => params
                .get("_meta")
                .and_then(Value::as_object)
                .and_then(|m| m.get("io.modelcontextprotocol/protocolVersion"))
                .and_then(Value::as_str),
            Some(Value::String(ver)) => Some(ver.as_str()),
            Some(_) => {
                return JsonRpcResponse::err(
                    id,
                    JSONRPC_INVALID_PARAMS,
                    "protocolVersion must be a string",
                    None,
                );
            }
        };
        if let Some(ver) = requested
            && !SUPPORTED_PROTOCOL_VERSIONS.contains(&ver)
        {
            return version_mismatch_error(id, ver);
        }

        let server_info = json!({
            "name": self.server_name,
            "version": self.server_version,
        });
        JsonRpcResponse::ok(
            id,
            json!({
                "resultType": "complete",
                "supportedVersions": SUPPORTED_PROTOCOL_VERSIONS,
                "protocolVersion": requested.unwrap_or(DEFAULT_PROTOCOL_VERSION),
                "capabilities": {
                    "tools": { "listChanged": false }
                },
                "transport": {
                    "kind": "stdio",
                    "stateless": true
                },
                "serverInfo": server_info,
                "_meta": {
                    "io.modelcontextprotocol/serverInfo": {
                        "name": self.server_name,
                        "version": self.server_version,
                    }
                },
                "tools": tool_definitions(),
            }),
        )
    }

    fn handle_tools_call(
        &self,
        principal: &Principal,
        id: Value,
        params: &Map<String, Value>,
    ) -> JsonRpcResponse {
        let Some(name) = params
            .get("name")
            .and_then(Value::as_str)
            .filter(|n| !n.trim().is_empty())
        else {
            return JsonRpcResponse::err(
                id,
                JSONRPC_INVALID_PARAMS,
                "tools/call requires non-empty string 'name'",
                None,
            );
        };

        let args = match params.get("arguments") {
            None | Some(Value::Null) => Map::new(),
            Some(Value::Object(map)) => map.clone(),
            Some(_) => {
                return JsonRpcResponse::err(
                    id,
                    JSONRPC_INVALID_PARAMS,
                    "tools/call 'arguments' must be an object",
                    None,
                );
            }
        };

        match name {
            "kb_search" => self.call_kb_search(principal, id, &args),
            "kb_get_document" => self.call_kb_get_document(principal, id, &args),
            "kb_get_section" => self.call_kb_get_section(principal, id, &args),
            "kb_search_api" => {
                self.call_kb_search_preset(principal, id, &args, &["endpoint", "query"])
            }
            "kb_search_error" => {
                self.call_kb_search_preset(principal, id, &args, &["error_code", "query"])
            }
            "kb_search_code" => {
                self.call_kb_search_preset(principal, id, &args, &["symbol", "query"])
            }
            "kb_list_categories" => self.call_kb_list_categories(principal, id, &args),
            _ => JsonRpcResponse::err(
                id,
                JSONRPC_INVALID_PARAMS,
                format!("unknown tool '{name}'"),
                None,
            ),
        }
    }

    fn call_kb_search(
        &self,
        principal: &Principal,
        id: Value,
        args: &Map<String, Value>,
    ) -> JsonRpcResponse {
        let query = match optional_string(args, "query") {
            Ok(Some(q)) => q,
            Ok(None) => String::new(),
            Err(err) => return JsonRpcResponse::err(id, JSONRPC_INVALID_PARAMS, err, None),
        };
        let req = match build_search_request(query, args) {
            Ok(r) => r,
            Err(InvalidOrDomain::InvalidParams(msg)) => {
                return JsonRpcResponse::err(id, JSONRPC_INVALID_PARAMS, msg, None);
            }
            Err(InvalidOrDomain::Domain(dok3_err)) => return dok3_error_result(id, dok3_err),
        };
        tool_result(id, self.search.search(principal, &req))
    }

    fn call_kb_search_preset(
        &self,
        principal: &Principal,
        id: Value,
        args: &Map<String, Value>,
        text_keys: &[&str],
    ) -> JsonRpcResponse {
        let mut parts = Vec::with_capacity(text_keys.len());
        for key in text_keys {
            match optional_string(args, key) {
                Ok(Some(val)) => {
                    let trimmed = val.trim();
                    if !trimmed.is_empty() {
                        parts.push(trimmed.to_string());
                    }
                }
                Ok(None) => {}
                Err(msg) => return JsonRpcResponse::err(id, JSONRPC_INVALID_PARAMS, msg, None),
            }
        }
        if parts.is_empty() {
            return JsonRpcResponse::err(
                id,
                JSONRPC_INVALID_PARAMS,
                format!("at least one of {:?} must be non-empty", text_keys),
                None,
            );
        }
        let req = match build_search_request(parts.join(" "), args) {
            Ok(r) => r,
            Err(InvalidOrDomain::InvalidParams(msg)) => {
                return JsonRpcResponse::err(id, JSONRPC_INVALID_PARAMS, msg, None);
            }
            Err(InvalidOrDomain::Domain(dok3_err)) => return dok3_error_result(id, dok3_err),
        };
        tool_result(id, self.search.search(principal, &req))
    }

    fn call_kb_get_document(
        &self,
        principal: &Principal,
        id: Value,
        args: &Map<String, Value>,
    ) -> JsonRpcResponse {
        let doc_id = match required_string(args, "document_id") {
            Ok(v) => v,
            Err(msg) => return JsonRpcResponse::err(id, JSONRPC_INVALID_PARAMS, msg, None),
        };
        tool_result(
            id,
            self.search
                .get_document(principal, &DocumentId::new(doc_id)),
        )
    }

    fn call_kb_get_section(
        &self,
        principal: &Principal,
        id: Value,
        args: &Map<String, Value>,
    ) -> JsonRpcResponse {
        let document_id = match required_string(args, "document_id") {
            Ok(v) => v,
            Err(msg) => return JsonRpcResponse::err(id, JSONRPC_INVALID_PARAMS, msg, None),
        };
        let section_id = match required_string(args, "section_id") {
            Ok(v) => v,
            Err(msg) => return JsonRpcResponse::err(id, JSONRPC_INVALID_PARAMS, msg, None),
        };
        let cursor = match optional_usize(args, "cursor") {
            Ok(v) => v,
            Err(msg) => return JsonRpcResponse::err(id, JSONRPC_INVALID_PARAMS, msg, None),
        };
        let max_chars = match optional_usize(args, "max_chars") {
            Ok(Some(v)) => v,
            Ok(None) => dok3_domain::DEFAULT_SECTION_MAX_CHARS,
            Err(msg) => return JsonRpcResponse::err(id, JSONRPC_INVALID_PARAMS, msg, None),
        };
        let req = SectionReadRequest {
            document_id: DocumentId::new(document_id),
            section_id: SectionId::new(section_id),
            cursor,
            max_chars,
        };
        tool_result(id, self.search.get_section(principal, &req))
    }

    fn call_kb_list_categories(
        &self,
        principal: &Principal,
        id: Value,
        args: &Map<String, Value>,
    ) -> JsonRpcResponse {
        let mut req = match build_search_request(String::new(), args) {
            Ok(r) => r,
            Err(InvalidOrDomain::InvalidParams(msg)) => {
                return JsonRpcResponse::err(id, JSONRPC_INVALID_PARAMS, msg, None);
            }
            Err(InvalidOrDomain::Domain(dok3_err)) => return dok3_error_result(id, dok3_err),
        };
        // Category facets ignore req.filters.category; keep page=1, limit=1, snippets=false.
        req.filters.category = None;
        req.page = 1;
        req.limit = 1;
        req.include_snippets = false;

        let res = self
            .search
            .search(principal, &req)
            .map(|resp| ListCategoriesResult {
                total_documents: resp.total_unfiltered_hits,
                categories: resp.category_counts,
                snapshot: resp.snapshot,
            });
        tool_result(id, res)
    }
}

enum InvalidOrDomain {
    InvalidParams(String),
    Domain(Dok3Error),
}

fn build_search_request(
    query: String,
    args: &Map<String, Value>,
) -> Result<SearchRequest, InvalidOrDomain> {
    let category = optional_string(args, "category").map_err(InvalidOrDomain::InvalidParams)?;
    let language = optional_string(args, "language").map_err(InvalidOrDomain::InvalidParams)?;
    let source_id = optional_string(args, "source_id")
        .map_err(InvalidOrDomain::InvalidParams)?
        .filter(|s| !s.trim().is_empty())
        .map(SourceId::new);
    let tags = optional_string_vec(args, "tags").map_err(InvalidOrDomain::InvalidParams)?;
    let page = optional_usize(args, "page")
        .map_err(InvalidOrDomain::InvalidParams)?
        .unwrap_or(1);
    let limit = optional_usize(args, "limit")
        .map_err(InvalidOrDomain::InvalidParams)?
        .unwrap_or(15);
    let include_snippets = optional_bool(args, "include_snippets")
        .map_err(InvalidOrDomain::InvalidParams)?
        .unwrap_or(true);
    let requested_scope = match optional_string(args, "requested_scope")
        .map_err(InvalidOrDomain::InvalidParams)?
        .filter(|s| !s.trim().is_empty())
    {
        Some(raw) => Some(AclScope::from_str(&raw).map_err(InvalidOrDomain::Domain)?),
        None => None,
    };

    Ok(SearchRequest {
        query,
        filters: SearchFilters {
            category,
            language,
            source_id,
            tags,
        },
        page,
        limit,
        include_snippets,
        requested_scope,
    })
}

fn check_meta_protocol_version(id: &Value, params: &Map<String, Value>) -> Option<JsonRpcResponse> {
    let meta = params.get("_meta").and_then(Value::as_object)?;
    let raw = meta
        .get("io.modelcontextprotocol/protocolVersion")
        .or_else(|| meta.get("protocolVersion"))?;
    let Some(ver) = raw.as_str() else {
        return Some(JsonRpcResponse::err(
            id.clone(),
            JSONRPC_INVALID_PARAMS,
            "_meta protocolVersion must be a string",
            None,
        ));
    };
    if !SUPPORTED_PROTOCOL_VERSIONS.contains(&ver) {
        return Some(version_mismatch_error(id.clone(), ver));
    }
    None
}

fn required_string(args: &Map<String, Value>, key: &str) -> Result<String, String> {
    match args.get(key) {
        Some(Value::String(s)) if !s.trim().is_empty() => Ok(s.clone()),
        Some(Value::String(_)) | None | Some(Value::Null) => {
            Err(format!("missing or empty string argument '{key}'"))
        }
        Some(_) => Err(format!("argument '{key}' must be a string")),
    }
}

fn optional_string(args: &Map<String, Value>, key: &str) -> Result<Option<String>, String> {
    match args.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) => Ok(Some(s.clone())),
        Some(_) => Err(format!("argument '{key}' must be a string")),
    }
}

fn optional_string_vec(args: &Map<String, Value>, key: &str) -> Result<Vec<String>, String> {
    match args.get(key) {
        None | Some(Value::Null) => Ok(Vec::new()),
        Some(Value::Array(items)) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                let Some(s) = item.as_str() else {
                    return Err(format!("all elements of '{key}' must be strings"));
                };
                if !s.trim().is_empty() {
                    out.push(s.to_string());
                }
            }
            Ok(out)
        }
        Some(_) => Err(format!("argument '{key}' must be an array of strings")),
    }
}

fn optional_usize(args: &Map<String, Value>, key: &str) -> Result<Option<usize>, String> {
    match args.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Number(num)) => {
            let Some(u) = num.as_u64() else {
                return Err(format!("argument '{key}' must be a non-negative integer"));
            };
            let v = usize::try_from(u)
                .map_err(|_| format!("argument '{key}' exceeds integer bounds"))?;
            Ok(Some(v))
        }
        Some(_) => Err(format!("argument '{key}' must be a non-negative integer")),
    }
}

fn optional_bool(args: &Map<String, Value>, key: &str) -> Result<Option<bool>, String> {
    match args.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Bool(b)) => Ok(Some(*b)),
        Some(_) => Err(format!("argument '{key}' must be a boolean")),
    }
}

fn tool_result<T: Serialize>(id: Value, result: Result<T, Dok3Error>) -> JsonRpcResponse {
    match result {
        Ok(payload) => {
            let structured = match serde_json::to_value(&payload) {
                Ok(val) => val,
                Err(err) => {
                    return dok3_error_result(
                        id,
                        Dok3Error::Internal(format!("failed to serialize MCP tool output: {err}")),
                    );
                }
            };
            let text = serde_json::to_string(&structured).unwrap_or_else(|_| "{}".to_string());
            JsonRpcResponse::ok(
                id,
                json!({
                    "content": [{ "type": "text", "text": text }],
                    "structuredContent": structured,
                    "isError": false
                }),
            )
        }
        Err(err) => dok3_error_result(id, err),
    }
}

fn dok3_error_result(id: Value, err: Dok3Error) -> JsonRpcResponse {
    let code = err.code();
    let msg = err.to_string();
    JsonRpcResponse::ok(
        id,
        json!({
            "content": [{ "type": "text", "text": format!("{code}: {msg}") }],
            "structuredContent": {
                "error": {
                    "code": code,
                    "message": msg,
                }
            },
            "isError": true
        }),
    )
}

enum BoundedLine<'a> {
    Eof,
    TooLarge,
    Line(&'a str),
}

fn read_bounded_line<'a, R: BufRead>(
    reader: &mut R,
    buf: &'a mut Vec<u8>,
    max_bytes: usize,
) -> io::Result<BoundedLine<'a>> {
    buf.clear();
    let mut oversized = false;
    loop {
        let available = reader.fill_buf()?;
        if available.is_empty() {
            if oversized {
                return Ok(BoundedLine::TooLarge);
            }
            if buf.is_empty() {
                return Ok(BoundedLine::Eof);
            }
            return std::str::from_utf8(buf)
                .map(BoundedLine::Line)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e));
        }

        let newline_pos = available.iter().position(|&b| b == b'\n');
        let take_len = newline_pos.map_or(available.len(), |idx| idx + 1);

        if !oversized {
            if buf.len().saturating_add(take_len) > max_bytes {
                oversized = true;
                buf.clear();
            } else {
                buf.extend_from_slice(&available[..take_len]);
            }
        }

        reader.consume(take_len);
        if newline_pos.is_some() {
            if oversized {
                return Ok(BoundedLine::TooLarge);
            }
            return std::str::from_utf8(buf)
                .map(BoundedLine::Line)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e));
        }
    }
}

fn write_json_line<W: Write>(writer: &mut W, resp: &JsonRpcResponse) -> io::Result<()> {
    serde_json::to_writer(&mut *writer, resp)
        .map_err(|e| io::Error::other(format!("serialize JSON-RPC response: {e}")))?;
    writer.write_all(b"\n")?;
    writer.flush()
}
