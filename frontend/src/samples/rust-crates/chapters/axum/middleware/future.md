# Module axum::middleware::future

## Overview

Future types.

## Items

- `FromExtractorResponseFuture` — Response future for FromExtractor.
- `FromFnResponseFuture` — Response future for FromFn.
- `MapRequestResponseFuture` — Response future for MapRequest.
- `MapResponseResponseFuture` — Response future for MapResponse.

### struct axum::middleware::future::FromExtractorResponseFuture

```rust
pub struct FromExtractorResponseFuture<B, T, E, S>where
    E: FromRequestParts<S>,
    T: Service<Request<B>>,{ /* private fields */ }
```

Response future for `FromExtractor`.

**Methods**

- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`

### struct axum::middleware::future::FromFnResponseFuture

```rust
pub struct FromFnResponseFuture { /* private fields */ }
```

Response future for `FromFn`.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`

### struct axum::middleware::future::MapRequestResponseFuture

```rust
pub struct MapRequestResponseFuture { /* private fields */ }
```

Response future for `MapRequest`.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`

### struct axum::middleware::future::MapResponseResponseFuture

```rust
pub struct MapResponseResponseFuture { /* private fields */ }
```

Response future for `MapResponse`.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`
