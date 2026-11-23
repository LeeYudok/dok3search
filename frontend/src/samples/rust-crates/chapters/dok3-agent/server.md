# Module dok3-agent::server

## Overview

Stateful & stateless MCP JSON-RPC 2.0 server backed by `dok3_search::SearchService`.

## Items

- `McpServer` — MCP stdio JSON-RPC 2.0 server delegating all use cases to SearchService.
- `MAX_JSONRPC_LINE_BYTES` — Maximum bytes accepted for a single newline-delimited JSON-RPC message on stdio.

### constant dok3-agent::server::MAX_JSONRPC_LINE_BYTES

```rust
pub const MAX_JSONRPC_LINE_BYTES: usize = _; // 1_048_576usize
```

Maximum bytes accepted for a single newline-delimited JSON-RPC message on stdio.

### struct dok3-agent::server::McpServer

```rust
pub struct McpServer { /* private fields */ }
```

MCP stdio JSON-RPC 2.0 server delegating all use cases to `SearchService`.

**Methods**

- `pub fn new(search: Arc<SearchService>, principal: Principal) -> Self`
- `pub fn with_server_info( self, name: impl Into<String>, version: impl Into<String>, ) -> Self`
- `pub fn handle_message(&self, raw_line: &str) -> Option<JsonRpcResponse>`
- `pub fn handle_message_for( &self, principal: &Principal, raw_line: &str, ) -> Option<JsonRpcResponse>`
- `pub fn serve_reader_writer<R: BufRead, W: Write>( &self, reader: R, writer: &mut W, ) -> Result<()>`
- `pub fn serve_stdio(&self) -> Result<()>`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
