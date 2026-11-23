# Module tower::util::future::optional

## Overview

Future types for `Optional`.

## Items

- `ResponseFuture` — Response future returned by Optional.

### struct tower::util::future::optional::ResponseFuture

```rust
pub struct ResponseFuture<T> { /* private fields */ }
```

Response future returned by `Optional`.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`
