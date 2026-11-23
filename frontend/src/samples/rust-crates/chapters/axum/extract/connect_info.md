# Module axum::extract::connect_info

## Overview

Extractor for getting connection information from a client.

See `Router::into_make_service_with_connect_info` for more details.

## Items

- `ConnectInfo` — Extractor for getting connection information produced by a Connected.
- `IntoMakeServiceWithConnectInfo` — A MakeService created from a router.
- `MockConnectInfo` — Middleware used to mock ConnectInfo during tests.
- `ResponseFuture` — Response future for IntoMakeServiceWithConnectInfo.
- `Connected` — Trait that connected IO resources implement and use to produce information about the connection.

### struct axum::extract::connect_info::ConnectInfo

```rust
pub struct ConnectInfo<T>(pub T);
```

Extractor for getting connection information produced by a `Connected`.

Note this extractor requires you to use
`Router::into_make_service_with_connect_info` to run your app
otherwise it will fail at runtime.

See `Router::into_make_service_with_connect_info` for more details.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deref(&self) -> &Self::Target`
- `fn deref_mut(&mut self) -> &mut Self::Target`
- `async fn from_request_parts( parts: &mut Parts, state: &S, ) -> Result<Self, Self::Rejection>`

### struct axum::extract::connect_info::IntoMakeServiceWithConnectInfo

```rust
pub struct IntoMakeServiceWithConnectInfo<S, C> { /* private fields */ }
```

A `MakeService` created from a router.

See `Router::into_make_service_with_connect_info` for more details.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, target: T) -> Self::Future`

### struct axum::extract::connect_info::MockConnectInfo

```rust
pub struct MockConnectInfo<T>(pub T);
```

Middleware used to mock `ConnectInfo` during tests.

If you’re accidentally using `MockConnectInfo` and
`Router::into_make_service_with_connect_info` at the same time then
`Router::into_make_service_with_connect_info` takes precedence.

**Example**

```rust
use axum::{
    Router,
    extract::connect_info::{MockConnectInfo, ConnectInfo},
    body::Body,
    routing::get,
    http::{Request, StatusCode},
};
use std::net::SocketAddr;
use tower::ServiceExt;

async fn handler(ConnectInfo(addr): ConnectInfo<SocketAddr>) {}

// this router you can run with `app.into_make_service_with_connect_info::<SocketAddr>()`
fn app() -> Router {
    Router::new().route("/", get(handler))
}

// use this router for tests
fn test_app() -> Router {
    app().layer(MockConnectInfo(SocketAddr::from(([0, 0, 0, 0], 1337))))
}

// #[tokio::test]
async fn some_test() {
    let app = test_app();

    let request = Request::new(Body::empty());
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
```

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn layer(&self, inner: S) -> Self::Service`

### struct axum::extract::connect_info::ResponseFuture

```rust
pub struct ResponseFuture<S, C> { /* private fields */ }
```

Response future for `IntoMakeServiceWithConnectInfo`.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`

### trait axum::extract::connect_info::Connected

```rust
pub trait Connected<T>:
    Clone
    + Send
    + Sync
    + 'static {
    // Required method
    fn connect_info(stream: T) -> Self;
}
```

Trait that connected IO resources implement and use to produce information
about the connection.

The goal for this trait is to allow users to implement custom IO types that
can still provide the same connection metadata.

See `Router::into_make_service_with_connect_info` for more details.

**Methods**

- `fn connect_info(stream: T) -> Self`
- `fn connect_info(stream: IncomingStream<'_, TcpListener>) -> Self`
- `fn connect_info(remote_addr: SocketAddr) -> Self`
