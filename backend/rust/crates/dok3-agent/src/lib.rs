//! AI agent protocol adapters (MCP, A2A, ACP, and `/llms.txt`).

pub mod a2a;
pub mod acp;
pub mod http;
pub mod llms_txt;
pub mod protocol;
pub mod server;

pub use a2a::{A2aServer, AgentCapabilities, AgentCard, AgentSkill, default_agent_card};
pub use acp::{AcpAdapter, AcpRequest, AcpResponse};
pub use http::{
    MCP_METHOD_HEADER, MCP_NAME_HEADER, MCP_PROTOCOL_VERSION_HEADER, PrincipalResolver,
    build_agent_router, build_agent_router_with_mcp, build_mcp_router, build_mcp_router_with_auth,
};
pub use llms_txt::render_llms_txt;
pub use protocol::{
    DEFAULT_PROTOCOL_VERSION, JSONRPC_HEADER_MISMATCH, JSONRPC_INTERNAL_ERROR,
    JSONRPC_INVALID_PARAMS, JSONRPC_INVALID_REQUEST, JSONRPC_METHOD_NOT_FOUND, JSONRPC_PARSE_ERROR,
    JSONRPC_VERSION_MISMATCH, JsonRpcError, JsonRpcResponse, ListCategoriesResult,
    SUPPORTED_PROTOCOL_VERSIONS, ToolDefinition, tool_definitions,
};
pub use server::{MAX_JSONRPC_LINE_BYTES, McpServer};

/// Compatibility initialization hook retained for workspace smoke checks.
pub fn init() {}
