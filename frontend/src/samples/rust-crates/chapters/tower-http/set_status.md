# Module tower-http::set_status

## Overview

Middleware to override status codes.

**Example**

```rust
use tower_http::set_status::SetStatusLayer;
use http::{Request, Response, StatusCode};
use bytes::Bytes;
use http_body_util::Full;
use std::{iter::once, convert::Infallible};
use tower::{ServiceBuilder, Service, ServiceExt};

async fn handle(req: Request<Full<Bytes>>) -> Result<Response<Full<Bytes>>, Infallible> {
    // ...
}

let mut service = ServiceBuilder::new()
    // change the status to `404 Not Found` regardless what the inner service returns
    .layer(SetStatusLayer::new(StatusCode::NOT_FOUND))
    .service_fn(handle);

// Call the service.
let request = Request::builder().body(Full::default())?;

let response = service.ready().await?.call(request).await?;

assert_eq!(response.status(), StatusCode::NOT_FOUND);
```

## Items

- `ResponseFuture` — Response future for SetStatus.
- `SetStatus` — Middleware to override status codes.
- `SetStatusLayer` — Layer that applies SetStatus which overrides the status codes.

### struct tower-http::set_status::ResponseFuture

```rust
pub struct ResponseFuture<F> { /* private fields */ }
```

Response future for `SetStatus`.

**Methods**

- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`

### struct tower-http::set_status::SetStatus

```rust
pub struct SetStatus<S> { /* private fields */ }
```

Middleware to override status codes.

See the module docs for more details.

**Methods**

- `pub fn new(inner: S, status: StatusCode) -> Self`
- `pub fn get_ref(&self) -> &S`
- `pub fn get_mut(&mut self) -> &mut S`
- `pub fn into_inner(self) -> S`
- `pub fn layer(status: StatusCode) -> SetStatusLayer`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, req: Request<ReqBody>) -> Self::Future`

### struct tower-http::set_status::SetStatusLayer

```rust
pub struct SetStatusLayer { /* private fields */ }
```

Layer that applies `SetStatus` which overrides the status codes.

**Methods**

- `pub fn new(status: StatusCode) -> Self`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn layer(&self, inner: S) -> Self::Service`
