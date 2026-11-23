# Module axum::handler::future

## Overview

Handler future types.

## Items

- `IntoServiceFuture` — The response future for IntoService.
- `LayeredFuture` — The response future for Layered.

### struct axum::handler::future::IntoServiceFuture

```rust
pub struct IntoServiceFuture<F> { /* private fields */ }
```

The response future for `IntoService`.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`

### struct axum::handler::future::LayeredFuture

```rust
pub struct LayeredFuture<S>where
    S: Service<Request>,{ /* private fields */ }
```

The response future for `Layered`.

**Methods**

- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`
