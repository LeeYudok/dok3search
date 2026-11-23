# Crate tower-http

## Overview

`async fn(HttpRequest) -> Result<HttpResponse, Error>`

**Overview**

tower-http is a library that provides HTTP-specific middleware and utilities built on top of
tower.

All middleware uses the http and http-body crates as the HTTP abstractions. That means
they’re compatible with any library or framework that also uses those crates, such as
hyper, tonic, and warp.

**Example server**

This example shows how to apply middleware from `tower-http` to a `Service` and then run
that service using hyper and hyper-util.

```rust
use tower_http::{
    compression::CompressionLayer,
    trace::TraceLayer,
};
use tower::{ServiceBuilder, service_fn, BoxError};
use http::{Request, Response};
use std::net::{SocketAddr, Ipv6Addr};
use bytes::Bytes;
use http_body_util::Full;
use hyper_util::rt::{TokioIo, TokioExecutor};
use hyper_util::service::TowerToHyperService;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    let addr = SocketAddr::from((Ipv6Addr::LOCALHOST, 8000));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let socket = TokioIo::new(socket);

            // Build our middleware stack
            let service = ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .service_fn(handler);

            // Wrap it for hyper
            let hyper_service = TowerToHyperService::new(service);

            if let Err(err) = hyper_util::server::conn::auto::Builder::new(TokioExecutor::new())
                .serve_connection(socket, hyper_service)
                .await
            {
                eprintln!("failed to serve connection: {err:#}");
            }
        });
    }
}
```

Keep in mind that while this example uses hyper, tower-http supports any HTTP
client/server implementation that uses the http and http-body crates.

**Example client**

tower-http middleware can also be applied to HTTP clients:

...

## Items

- `body` — Body types.
- `classify` — Tools for classifying responses as either success or failure.
- `services` — Services that return responses without wrapping other Services.
- `set_status` — Middleware to override status codes.
- `LatencyUnit` — The latency unit used to report latencies by middleware.
- `BoxError` — Alias for a type-erased error type.

### enum tower-http::LatencyUnit

```rust
#[non_exhaustive]pub enum LatencyUnit {
    Seconds,
    Millis,
    Micros,
    Nanos,
}
```

The latency unit used to report latencies by middleware.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### type tower-http::BoxError

```rust
pub type BoxError = Box<dyn Error + Send + Sync>;
```

Alias for a type-erased error type.
