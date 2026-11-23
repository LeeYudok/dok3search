# Module dok3-agent::protocol

## Overview

MCP JSON-RPC 2.0 wire types, protocol version constants, and SSOT tool schemas.

## Items

- `JsonRpcError` — JSON-RPC 2.0 error object.
- `JsonRpcResponse` — Standard JSON-RPC 2.0 response envelope.
- `ListCategoriesResult` — Structured output returned by kb_list_categories.
- `ToolDefinition` — MCP tool descriptor exposed via tools/list and server/discover.
- `DEFAULT_PROTOCOL_VERSION` — Latest MCP protocol revision supported alongside stateful legacy handshakes.
- `JSONRPC_HEADER_MISMATCH` — MCP 2026-07-28 HTTP header/body mismatch error code.
- `JSONRPC_VERSION_MISMATCH` — MCP 2026-07-28 incompatible protocol version error code.
- `SUPPORTED_PROTOCOL_VERSIONS` — Supported MCP protocol revisions (2026-07-28 stateless + 2025-11-25/2024-11-05 initialize).
- `tool_definitions` — Returns all 7 SSOT MCP tool definitions documented in architecture §8.2.

### constant dok3-agent::protocol::DEFAULT_PROTOCOL_VERSION

```rust
pub const DEFAULT_PROTOCOL_VERSION: &str = "2026-07-28";
```

Latest MCP protocol revision supported alongside stateful legacy handshakes.

### constant dok3-agent::protocol::JSONRPC_HEADER_MISMATCH

```rust
pub const JSONRPC_HEADER_MISMATCH: i32 = -32020;
```

MCP `2026-07-28` HTTP header/body mismatch error code.

### constant dok3-agent::protocol::JSONRPC_INTERNAL_ERROR

```rust
pub const JSONRPC_INTERNAL_ERROR: i32 = -32603;
```

### constant dok3-agent::protocol::JSONRPC_INVALID_PARAMS

```rust
pub const JSONRPC_INVALID_PARAMS: i32 = -32602;
```

### constant dok3-agent::protocol::JSONRPC_INVALID_REQUEST

```rust
pub const JSONRPC_INVALID_REQUEST: i32 = -32600;
```

### constant dok3-agent::protocol::JSONRPC_METHOD_NOT_FOUND

```rust
pub const JSONRPC_METHOD_NOT_FOUND: i32 = -32601;
```

### constant dok3-agent::protocol::JSONRPC_PARSE_ERROR

```rust
pub const JSONRPC_PARSE_ERROR: i32 = -32700;
```

### constant dok3-agent::protocol::JSONRPC_VERSION_MISMATCH

```rust
pub const JSONRPC_VERSION_MISMATCH: i32 = -32022;
```

MCP `2026-07-28` incompatible protocol version error code.

### constant dok3-agent::protocol::SUPPORTED_PROTOCOL_VERSIONS

```rust
pub const SUPPORTED_PROTOCOL_VERSIONS: &[&str];
```

Supported MCP protocol revisions (`2026-07-28` stateless + `2025-11-25`/`2024-11-05` initialize).

### fn dok3-agent::protocol::tool_definitions

```rust
pub fn tool_definitions() -> Vec<ToolDefinition>
```

Returns all 7 SSOT MCP tool definitions documented in architecture §8.2.

### struct dok3-agent::protocol::JsonRpcError

```rust
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    pub data: Option<Value>,
}
```

JSON-RPC 2.0 error object.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-agent::protocol::JsonRpcResponse

```rust
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Value,
    pub result: Option<Value>,
    pub error: Option<JsonRpcError>,
}
```

Standard JSON-RPC 2.0 response envelope.

**Methods**

- `pub fn ok(id: Value, result: Value) -> Self`
- `pub fn err( id: Value, code: i32, message: impl Into<String>, data: Option<Value>, ) -> Self`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-agent::protocol::ListCategoriesResult

```rust
pub struct ListCategoriesResult {
    pub total_documents: usize,
    pub categories: Vec<CategoryCount>,
    pub snapshot: SearchSnapshotMeta,
}
```

Structured output returned by `kb_list_categories`.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-agent::protocol::ToolDefinition

```rust
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}
```

MCP tool descriptor exposed via `tools/list` and `server/discover`.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`
