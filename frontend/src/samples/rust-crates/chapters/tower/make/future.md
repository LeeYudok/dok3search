# Module tower::make::future

## Overview

Future types

## Items

- `SharedFuture` — Response future from Shared services.

### struct tower::make::future::SharedFuture

```rust
pub struct SharedFuture<S> { /* private fields */ }
```

Response future from `Shared` services.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`
