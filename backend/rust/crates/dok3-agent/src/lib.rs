//! AI agent protocol adapters (MCP first, A2A and ACP follow-up).

pub mod protocol;
pub mod server;

pub use protocol::{
    DEFAULT_PROTOCOL_VERSION, JSONRPC_INTERNAL_ERROR, JSONRPC_INVALID_PARAMS,
    JSONRPC_INVALID_REQUEST, JSONRPC_METHOD_NOT_FOUND, JSONRPC_PARSE_ERROR,
    JSONRPC_VERSION_MISMATCH, JsonRpcError, JsonRpcResponse, ListCategoriesResult,
    SUPPORTED_PROTOCOL_VERSIONS, ToolDefinition, tool_definitions,
};
pub use server::{MAX_JSONRPC_LINE_BYTES, McpServer};

/// Compatibility initialization hook retained for workspace smoke checks.
pub fn init() {}
