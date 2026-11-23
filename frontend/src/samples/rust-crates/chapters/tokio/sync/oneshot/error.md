# Module tokio::sync::oneshot::error

## Overview

`Oneshot` error types.

## Items

- `RecvError` — Error returned by the Future implementation for Receiver.
- `TryRecvError` — Error returned by the try_recv function on Receiver.

### enum tokio::sync::oneshot::error::TryRecvError

```rust
pub enum TryRecvError {
    Empty,
    Closed,
}
```

Error returned by the `try_recv` function on `Receiver`.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### struct tokio::sync::oneshot::error::RecvError

```rust
pub struct RecvError(/* private fields */);
```

Error returned by the `Future` implementation for `Receiver`.

This error is returned by the receiver when the sender is dropped without sending.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
