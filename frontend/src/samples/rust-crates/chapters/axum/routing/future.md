# Module axum::routing::future

## Overview

Future types.

## Items

- `InfallibleRouteFuture` — A RouteFuture that always yields a Response.
- `IntoMakeServiceFuture` — Response future for IntoMakeService.
- `RouteFuture` — Response future for Route.

### struct axum::routing::future::InfallibleRouteFuture

```rust
pub struct InfallibleRouteFuture { /* private fields */ }
```

A `RouteFuture` that always yields a `Response`.

**Methods**

- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`

### struct axum::routing::future::IntoMakeServiceFuture

```rust
pub struct IntoMakeServiceFuture<S> { /* private fields */ }
```

Response future for `IntoMakeService`.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`

### struct axum::routing::future::RouteFuture

```rust
pub struct RouteFuture<E> { /* private fields */ }
```

Response future for `Route`.

**Methods**

- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`
