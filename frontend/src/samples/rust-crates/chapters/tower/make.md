# Module tower::make

## Overview

Trait aliases for Services that produce specific types of Responses.

## Items

- `future` — Future types
- `AsService` — Service returned by MakeService::as_service.
- `IntoService` — Service returned by MakeService::into_service.
- `Shared` — A MakeService that produces services by cloning an inner service.
- `MakeConnection` — The MakeConnection trait is used to create transports.
- `MakeService` — Creates new Service values.

### struct tower::make::AsService

```rust
pub struct AsService<'a, M, Request> { /* private fields */ }
```

Service returned by `MakeService::as_service`.

See the documentation on `as_service` for details.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, target: Target) -> Self::Future`

### struct tower::make::IntoService

```rust
pub struct IntoService<M, Request> { /* private fields */ }
```

Service returned by `MakeService::into_service`.

See the documentation on `into_service` for details.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, target: Target) -> Self::Future`

### struct tower::make::Shared

```rust
pub struct Shared<S> { /* private fields */ }
```

A `MakeService` that produces services by cloning an inner service.

**Example**

```rust
use std::future::{Ready, ready};
use tower::make::{MakeService, Shared};
use tower::buffer::Buffer;
use tower::Service;

// An example connection type
struct Connection {}

// An example request type
struct Request {}

// An example response type
struct Response {}

// Some service that doesn't implement `Clone`
struct MyService;

impl Service<Request> for MyService {
    type Response = Response;
    type Error = Infallible;
    type Future = Ready<Result<Response, Infallible>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request) -> Self::Future {
        ready(Ok(Response {}))
    }
}

// Example function that runs a service by accepting new connections and using
// `Make` to create new services that might be bound to the connection.
//
// This is similar to what you might find in hyper.
async fn serve_make_service<Make>(make: Make)
where
    Make: MakeService<Connection, Request>
{
    // ...
}

// Our service
let svc = MyService;

// Make it `Clone` by putting a channel in front
let buffered = Buffer::new(svc, 1024);

// Convert it into a `MakeService`
let make = Shared::new(buffered);

// Run the service and just ignore the `Connection`s as `MyService` doesn't need them
serve_make_service(make).await;
```

**Methods**

- `pub const fn new(service: S) -> Self`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, _target: T) -> Self::Future`

### trait tower::make::MakeConnection

```rust
pub trait MakeConnection<Target>: Sealed<(Target,)> {
    type Connection: AsyncRead + AsyncWrite;
    type Error;
    type Future: Future<Output = Result<Self::Connection, Self::Error>>;

    // Required methods
    fn poll_ready(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>>;
    fn make_connection(&mut self, target: Target) -> Self::Future;
}
```

The `MakeConnection` trait is used to create transports.

The goal of this service is to allow composable methods for creating
`AsyncRead + AsyncWrite` transports. This could mean creating a TLS
based connection or using some other method to authenticate the connection.

**Methods**

- `type Connection: AsyncRead + AsyncWrite`
- `type Error`
- `type Future: Future<Output = Result<Self::Connection, Self::Error>>`
- `fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn make_connection(&mut self, target: Target) -> Self::Future`

### trait tower::make::MakeService

```rust
pub trait MakeService<Target, Request>: Sealed<(Target, Request)> {
    type Response;
    type Error;
    type Service: Service<Request, Response = Self::Response, Error = Self::Error>;
    type MakeError;
    type Future: Future<Output = Result<Self::Service, Self::MakeError>>;

    // Required methods
    fn poll_ready(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::MakeError>>;
    fn make_service(&mut self, target: Target) -> Self::Future;

    // Provided methods
    fn into_service(self) -> IntoService<Self, Request>
       where Self: Sized { ... }
    fn as_service(&mut self) -> AsService<'_, Self, Request>
       where Self: Sized { ... }
}
```

Creates new `Service` values.

Acts as a service factory. This is useful for cases where new `Service`
values must be produced. One case is a TCP server listener. The listener
accepts new TCP streams, obtains a new `Service` value using the
`MakeService` trait, and uses that new `Service` value to process inbound
requests on that new TCP stream.

This is essentially a trait alias for a `Service` of `Service`s.

**Methods**

- `type Response`
- `type Error`
- `type Service: Service<Request, Response = Self::Response, Error = Self::Error>`
- `type MakeError`
- `type Future: Future<Output = Result<Self::Service, Self::MakeError>>`
- `fn poll_ready( &mut self, cx: &mut Context<'_>, ) -> Poll<Result<(), Self::MakeError>>`
- `fn make_service(&mut self, target: Target) -> Self::Future`
- `fn into_service(self) -> IntoService<Self, Request>where Self: Sized,`
- `fn as_service(&mut self) -> AsService<'_, Self, Request>where Self: Sized,`
