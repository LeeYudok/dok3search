# Crate dok3-agent

## Overview

AI agent protocol adapters (MCP first, A2A and ACP follow-up).

## Items

- `http` — MCP Streamable HTTP (POST /mcp) transport adapter with version, header-body consistency, loopback Origin guard, and per-request Principal authorization.
- `protocol` — MCP JSON-RPC 2.0 wire types, protocol version constants, and SSOT tool schemas.
- `server` — Stateful & stateless MCP JSON-RPC 2.0 server backed by dok3_search::SearchService.
- `init` — Compatibility initialization hook retained for workspace smoke checks.

### fn dok3-agent::init

```rust
pub fn init()
```

Compatibility initialization hook retained for workspace smoke checks.
