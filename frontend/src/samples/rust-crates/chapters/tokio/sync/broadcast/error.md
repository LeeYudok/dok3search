# Module tokio::sync::broadcast::error

## Overview

Broadcast error types

## Items

- `SendError` — Error returned by the send function on a Sender.
- `RecvError` — An error returned from the recv function on a Receiver.
- `TryRecvError` — An error returned from the try_recv function on a Receiver.

### enum tokio::sync::broadcast::error::RecvError

```rust
pub enum RecvError {
    Closed,
    Lagged(u64),
}
```

An error returned from the `recv` function on a `Receiver`.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### enum tokio::sync::broadcast::error::TryRecvError

```rust
pub enum TryRecvError {
    Empty,
    Closed,
    Lagged(u64),
}
```

An error returned from the `try_recv` function on a `Receiver`.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### struct tokio::sync::broadcast::error::SendError

```rust
pub struct SendError<T>(pub T);
```

Error returned by the `send` function on a `Sender`.

A send operation can only fail if there are no active receivers,
implying that the message could never be received. The error contains the
message being sent as a payload so it can be recovered.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
