//! MCP JSON-RPC 2.0 wire types, protocol version constants, and SSOT tool schemas.

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use dok3_domain::{CategoryCount, SearchSnapshotMeta};

/// Latest MCP protocol revision supported alongside stateful legacy handshakes.
pub const DEFAULT_PROTOCOL_VERSION: &str = "2026-07-28";
/// Supported MCP protocol revisions (`2026-07-28` stateless + `2025-11-25`/`2024-11-05` initialize).
pub const SUPPORTED_PROTOCOL_VERSIONS: &[&str] = &["2026-07-28", "2025-11-25", "2024-11-05"];

pub const JSONRPC_PARSE_ERROR: i32 = -32700;
pub const JSONRPC_INVALID_REQUEST: i32 = -32600;
pub const JSONRPC_METHOD_NOT_FOUND: i32 = -32601;
pub const JSONRPC_INVALID_PARAMS: i32 = -32602;
pub const JSONRPC_INTERNAL_ERROR: i32 = -32603;
/// MCP `2026-07-28` HTTP header/body mismatch error code.
pub const JSONRPC_HEADER_MISMATCH: i32 = -32020;
/// MCP `2026-07-28` incompatible protocol version error code.
pub const JSONRPC_VERSION_MISMATCH: i32 = -32022;

/// Builds a canonical `-32022` unsupported protocol version response with `data.supported` and `data.requested`.
#[must_use]
pub(crate) fn version_mismatch_error(id: Value, requested: &str) -> JsonRpcResponse {
    JsonRpcResponse::err(
        id,
        JSONRPC_VERSION_MISMATCH,
        format!("unsupported MCP protocol version '{requested}'"),
        Some(json!({
            "supported": SUPPORTED_PROTOCOL_VERSIONS,
            "requested": requested,
        })),
    )
}

/// Standard JSON-RPC 2.0 response envelope.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

/// JSON-RPC 2.0 error object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl JsonRpcResponse {
    /// Builds a successful JSON-RPC response for `id`.
    #[must_use]
    pub fn ok(id: Value, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    /// Builds a JSON-RPC error response for `id`.
    #[must_use]
    pub fn err(id: Value, code: i32, message: impl Into<String>, data: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(JsonRpcError {
                code,
                message: message.into(),
                data,
            }),
        }
    }
}

/// MCP tool descriptor exposed via `tools/list` and `server/discover`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

/// Structured output returned by `kb_list_categories`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListCategoriesResult {
    pub total_documents: usize,
    pub categories: Vec<CategoryCount>,
    pub snapshot: SearchSnapshotMeta,
}

fn search_common_properties() -> serde_json::Map<String, Value> {
    let mut props = serde_json::Map::new();
    props.insert(
        "category".to_string(),
        json!({ "type": "string", "description": "Optional category ID filter (e.g. guide, manual, code, policy, design)." }),
    );
    props.insert(
        "language".to_string(),
        json!({ "type": "string", "description": "Optional language filter (e.g. ko, en, rust)." }),
    );
    props.insert(
        "source_id".to_string(),
        json!({ "type": "string", "description": "Optional ingested source identifier filter." }),
    );
    props.insert(
        "tags".to_string(),
        json!({ "type": "array", "items": { "type": "string" }, "description": "Optional tag filters (all specified tags must match)." }),
    );
    props.insert(
        "page".to_string(),
        json!({ "type": "integer", "minimum": 1, "description": "1-based page number (default 1)." }),
    );
    props.insert(
        "limit".to_string(),
        json!({ "type": "integer", "minimum": 1, "maximum": 100, "description": "Page size (default 15, max 100)." }),
    );
    props.insert(
        "include_snippets".to_string(),
        json!({ "type": "boolean", "description": "Include hit snippets in results (default true)." }),
    );
    props.insert(
        "requested_scope".to_string(),
        json!({ "type": "string", "description": "Advisory ACL narrowing scope ('public' or 'internal:<name>'). Cannot escalate caller privileges." }),
    );
    props
}

fn schema_with(extra: Vec<(&str, Value)>, required: &[&str], include_common_search: bool) -> Value {
    let mut properties = if include_common_search {
        search_common_properties()
    } else {
        serde_json::Map::new()
    };
    for (k, v) in extra {
        properties.insert(k.to_string(), v);
    }
    json!({
        "type": "object",
        "properties": Value::Object(properties),
        "required": required,
        "additionalProperties": false,
    })
}

/// Returns all 7 SSOT MCP tool definitions documented in architecture §8.2.
#[must_use]
pub fn tool_definitions() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: "kb_search".to_string(),
            description: "Search authorized knowledge base documents with filters, pagination, and category facets."
                .to_string(),
            input_schema: schema_with(
                vec![(
                    "query",
                    json!({ "type": "string", "description": "Search query text (max 512 chars)." }),
                )],
                &[],
                true,
            ),
        },
        ToolDefinition {
            name: "kb_get_document".to_string(),
            description: "Retrieve document metadata, body, section TOC, and paired format links if authorized."
                .to_string(),
            input_schema: schema_with(
                vec![(
                    "document_id",
                    json!({ "type": "string", "description": "Logical document ID." }),
                )],
                &["document_id"],
                false,
            ),
        },
        ToolDefinition {
            name: "kb_get_section".to_string(),
            description: "Read a bounded, cursor-paginated slice of a specific document section with source citations."
                .to_string(),
            input_schema: schema_with(
                vec![
                    (
                        "document_id",
                        json!({ "type": "string", "description": "Logical document ID." }),
                    ),
                    (
                        "section_id",
                        json!({ "type": "string", "description": "Section ID within the document." }),
                    ),
                    (
                        "cursor",
                        json!({ "type": "integer", "minimum": 0, "description": "Character offset cursor for pagination." }),
                    ),
                    (
                        "max_chars",
                        json!({ "type": "integer", "minimum": 1, "maximum": 100000, "description": "Maximum characters to return (default 12000)." }),
                    ),
                ],
                &["document_id", "section_id"],
                false,
            ),
        },
        ToolDefinition {
            name: "kb_search_api".to_string(),
            description: "Search REST/API contracts, endpoints, and schema specifications."
                .to_string(),
            input_schema: schema_with(
                vec![
                    (
                        "query",
                        json!({ "type": "string", "description": "API contract or operation query." }),
                    ),
                    (
                        "endpoint",
                        json!({ "type": "string", "description": "Optional API path or endpoint keyword (e.g. /api/v1/search)." }),
                    ),
                ],
                &[],
                true,
            ),
        },
        ToolDefinition {
            name: "kb_search_error".to_string(),
            description: "Search error codes, diagnostics, recovery guides, and troubleshooting runbooks."
                .to_string(),
            input_schema: schema_with(
                vec![
                    (
                        "error_code",
                        json!({ "type": "string", "description": "Exact error code or status token (e.g. ORA-00904, INVALID_INPUT)." }),
                    ),
                    (
                        "query",
                        json!({ "type": "string", "description": "Additional diagnostic context or symptom text." }),
                    ),
                ],
                &[],
                true,
            ),
        },
        ToolDefinition {
            name: "kb_search_code".to_string(),
            description: "Search code symbols, identifiers, namespaces, and programming examples."
                .to_string(),
            input_schema: schema_with(
                vec![
                    (
                        "symbol",
                        json!({ "type": "string", "description": "Code symbol or namespace (e.g. SearchService, tokio::spawn)." }),
                    ),
                    (
                        "query",
                        json!({ "type": "string", "description": "Additional code or topic query." }),
                    ),
                ],
                &[],
                true,
            ),
        },
        ToolDefinition {
            name: "kb_list_categories".to_string(),
            description: "List category document counts visible under the caller's effective ACL."
                .to_string(),
            input_schema: schema_with(
                vec![
                    (
                        "language",
                        json!({ "type": "string", "description": "Optional language filter." }),
                    ),
                    (
                        "source_id",
                        json!({ "type": "string", "description": "Optional source ID filter." }),
                    ),
                    (
                        "tags",
                        json!({ "type": "array", "items": { "type": "string" }, "description": "Optional tag filters." }),
                    ),
                    (
                        "requested_scope",
                        json!({ "type": "string", "description": "Advisory ACL narrowing scope." }),
                    ),
                ],
                &[],
                false,
            ),
        },
    ]
}
