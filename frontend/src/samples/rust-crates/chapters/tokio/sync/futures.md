# Module tokio::sync::futures

## Overview

Named future types.

## Items

- `Notified` — Future returned from Notify::notified().
- `OwnedNotified` — Future returned from Notify::notified_owned().

### struct tokio::sync::futures::Notified

```rust
pub struct Notified<'a> { /* private fields */ }
```

Future returned from `Notify::notified()`.

This future is fused, so once it has completed, any future calls to poll
will immediately return `Poll::Ready`.

**Methods**

- `pub fn enable(self: Pin<&mut Self>) -> bool`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`
- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()>`

### struct tokio::sync::futures::OwnedNotified

```rust
pub struct OwnedNotified { /* private fields */ }
```

Future returned from `Notify::notified_owned()`.

This future is fused, so once it has completed, any future calls to poll
will immediately return `Poll::Ready`.

**Methods**

- `pub fn enable(self: Pin<&mut Self>) -> bool`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`
- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()>`
