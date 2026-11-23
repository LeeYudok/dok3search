# Module axum::error_handling::future

## Overview

Future types.

## Items

- `HandleErrorFuture` — Response future for [HandleError].

### struct axum::error_handling::future::HandleErrorFuture

```rust
pub struct HandleErrorFuture { /* private fields */ }
```

Response future for [`HandleError`].

**Methods**

- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`
