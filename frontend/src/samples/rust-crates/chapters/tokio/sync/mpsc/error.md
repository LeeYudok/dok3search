# Module tokio::sync::mpsc::error

## Overview

Channel error types.

## Items

- `SendError` — Error returned by Sender::send.
- `SendTimeoutError` — Error returned by Sender::send_timeout.
- `TryRecvError` — Error returned by Receiver::try_recv.
- `TrySendError` — Error returned by Sender::try_send.

### enum tokio::sync::mpsc::error::SendTimeoutError

```rust
pub enum SendTimeoutError<T> {
    Timeout(T),
    Closed(T),
}
```

Error returned by `Sender::send_timeout`.

**Methods**

- `pub fn into_inner(self) -> T`
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

### enum tokio::sync::mpsc::error::TryRecvError

```rust
pub enum TryRecvError {
    Empty,
    Disconnected,
}
```

Error returned by `Receiver::try_recv`.

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

### enum tokio::sync::mpsc::error::TrySendError

```rust
pub enum TrySendError<T> {
    Full(T),
    Closed(T),
}
```

Error returned by `Sender::try_send`.

**Methods**

- `pub fn into_inner(self) -> T`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(src: SendError<T>) -> TrySendError<T>`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### struct tokio::sync::mpsc::error::SendError

```rust
pub struct SendError<T>(pub T);
```

Error returned by `Sender::send`.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(src: SendError<T>) -> TrySendError<T>`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
