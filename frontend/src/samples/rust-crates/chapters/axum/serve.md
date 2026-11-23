# Module axum::serve

## Overview

Serve services.

## Items

- `IncomingStream` — An incoming stream.
- `Serve` — Future returned by serve.
- `TapIo` — Return type of ListenerExt::tap_io.
- `WithGracefulShutdown` — Serve future with graceful shutdown enabled.
- `Listener` — Types that can listen for connections.
- `ListenerExt` — Extensions to Listener.
- `serve` — Serve the service with the supplied listener.

### fn axum::serve::serve

```rust
pub fn serve<L, M, S>(listener: L, make_service: M) -> Serve<L, M, S>where
    L: Listener,
    M: for<'a> Service<IncomingStream<'a, L>, Error = Infallible, Response = S>,
    S: Service<Request, Response = Response, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send,
```

Serve the service with the supplied listener.

This method of running a service is intentionally simple and doesn’t support any configuration.
Use hyper or hyper-util if you need configuration.

It supports both HTTP/1 as well as HTTP/2.

**Examples**

Serving a `Router`:

```rust
use axum::{Router, routing::get};

let router = Router::new().route("/", get(|| async { "Hello, World!" }));

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, router).await.unwrap();
```

See also `Router::into_make_service_with_connect_info`.

Serving a `MethodRouter`:

```rust
use axum::routing::get;

let router = get(|| async { "Hello, World!" });

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, router).await.unwrap();
```

See also `MethodRouter::into_make_service_with_connect_info`.

Serving a `Handler`:

```rust
use axum::handler::HandlerWithoutStateExt;

async fn handler() -> &'static str {
    "Hello, World!"
}

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, handler.into_make_service()).await.unwrap();
```

See also `HandlerWithoutStateExt::into_make_service_with_connect_info` and
`HandlerService::into_make_service_with_connect_info`.

**Return Value**

Although this future resolves to `io::Result<()>`, it will never actually complete or return an
error. Errors on the TCP socket will be handled by sleeping for a short while (currently, one
second).

### struct axum::serve::IncomingStream

```rust
pub struct IncomingStream<'a, L>where
    L: Listener,{ /* private fields */ }
```

An incoming stream.

Used with `serve` and `IntoMakeServiceWithConnectInfo`.

**Methods**

- `pub fn io(&self) -> &L::Io`
- `pub fn remote_addr(&self) -> &L::Addr`
- `fn connect_info(stream: IncomingStream<'_, TcpListener>) -> Self`
- `fn connect_info(stream: IncomingStream<'a, TapIo<L, F>>) -> Self`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, _req: IncomingStream<'_, L>) -> Self::Future`
- `fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, _req: IncomingStream<'_, L>) -> Self::Future`
- `fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, _req: IncomingStream<'_, L>) -> Self::Future`

### struct axum::serve::Serve

```rust
pub struct Serve<L, M, S> { /* private fields */ }
```

Future returned by `serve`.

**Methods**

- `pub fn with_graceful_shutdown<F>( self, signal: F, ) -> WithGracefulShutdown<L, M, S, F>where F: Future<Output = ()> + Send + 'static,`
- `pub fn local_addr(&self) -> Result<L::Addr>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn into_future(self) -> Self::IntoFuture`

### struct axum::serve::TapIo

```rust
pub struct TapIo<L, F> { /* private fields */ }
```

Return type of `ListenerExt::tap_io`.

See that method for details.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `async fn accept(&mut self) -> (Self::Io, Self::Addr)`
- `fn local_addr(&self) -> Result<Self::Addr>`

### struct axum::serve::WithGracefulShutdown

```rust
pub struct WithGracefulShutdown<L, M, S, F> { /* private fields */ }
```

Serve future with graceful shutdown enabled.

**Methods**

- `pub fn local_addr(&self) -> Result<L::Addr>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn into_future(self) -> Self::IntoFuture`

### trait axum::serve::Listener

```rust
pub trait Listener: Send + 'static {
    type Io: AsyncRead + AsyncWrite + Unpin + Send + 'static;
    type Addr: Send;

    // Required methods
    fn accept(&mut self) -> impl Future<Output = (Self::Io, Self::Addr)> + Send;
    fn local_addr(&self) -> Result<Self::Addr>;
}
```

Types that can listen for connections.

**Methods**

- `type Io: AsyncRead + AsyncWrite + Unpin + Send + 'static`
- `type Addr: Send`
- `fn accept(&mut self) -> impl Future<Output = (Self::Io, Self::Addr)> + Send`
- `fn local_addr(&self) -> Result<Self::Addr>`
- `async fn accept(&mut self) -> (Self::Io, Self::Addr)`
- `fn local_addr(&self) -> Result<Self::Addr>`
- `async fn accept(&mut self) -> (Self::Io, Self::Addr)`
- `fn local_addr(&self) -> Result<Self::Addr>`

### trait axum::serve::ListenerExt

```rust
pub trait ListenerExt: Listener + Sized {
    // Provided method
    fn tap_io<F>(self, tap_fn: F) -> TapIo<Self, F>
       where F: FnMut(&mut Self::Io) + Send + 'static { ... }
}
```

Extensions to `Listener`.

**Methods**

- `fn tap_io<F>(self, tap_fn: F) -> TapIo<Self, F>where F: FnMut(&mut Self::Io) + Send + 'static,`
