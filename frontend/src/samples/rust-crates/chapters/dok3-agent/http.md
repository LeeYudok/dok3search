# Module dok3-agent::http

## Overview

MCP Streamable HTTP (`POST /mcp`) transport adapter with version, header-body consistency,
loopback `Origin` guard, and per-request `Principal` authorization.

## Items

- `MCP_METHOD_HEADER` — MCP 2026-07-28 routing header mirroring the JSON-RPC method.
- `MCP_NAME_HEADER` — MCP 2026-07-28 routing header mirroring params.name or params.uri.
- `MCP_PROTOCOL_VERSION_HEADER` — Canonical HTTP header name carrying the MCP specification revision.
- `build_mcp_router` — Builds a Streamable HTTP router (/mcp) binding every unauthenticated HTTP caller strictly to Principal::anonymous_public() (never inherits privileged stdio defaults).
- `build_mcp_router_with_auth` — Builds a Streamable HTTP router (/mcp) evaluating per-request caller credentials via resolve_principal. Rejections map fail-closed to HTTP 403 (Dok3Error::Forbidden) or 401 Unauthorized.
- `PrincipalResolver` — Resolves the caller Principal for each incoming HTTP request.

### constant dok3-agent::http::MCP_METHOD_HEADER

```rust
pub const MCP_METHOD_HEADER: &str = "mcp-method";
```

MCP `2026-07-28` routing header mirroring the JSON-RPC `method`.

### constant dok3-agent::http::MCP_NAME_HEADER

```rust
pub const MCP_NAME_HEADER: &str = "mcp-name";
```

MCP `2026-07-28` routing header mirroring `params.name` or `params.uri`.

### constant dok3-agent::http::MCP_PROTOCOL_VERSION_HEADER

```rust
pub const MCP_PROTOCOL_VERSION_HEADER: &str = "mcp-protocol-version";
```

Canonical HTTP header name carrying the MCP specification revision.

### fn dok3-agent::http::build_mcp_router

```rust
pub fn build_mcp_router(server: Arc<McpServer>) -> Router
```

Builds a Streamable HTTP router (`/mcp`) binding every unauthenticated HTTP caller
strictly to `Principal::anonymous_public()` (never inherits privileged stdio defaults).

### fn dok3-agent::http::build_mcp_router_with_auth

```rust
pub fn build_mcp_router_with_auth(
    server: Arc<McpServer>,
    resolve_principal: PrincipalResolver,
) -> Router
```

Builds a Streamable HTTP router (`/mcp`) evaluating per-request caller credentials via `resolve_principal`.
Rejections map fail-closed to HTTP `403` (`Dok3Error::Forbidden`) or `401 Unauthorized`.

### type dok3-agent::http::PrincipalResolver

```rust
pub type PrincipalResolver = Arc<dyn Fn(&HeaderMap) -> Result<Principal, Dok3Error> + Send + Sync>;
```

Resolves the caller `Principal` for each incoming HTTP request.
