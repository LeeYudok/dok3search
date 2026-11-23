# Crate tower

## Overview

`async fn(Request) -> Result<Response, Error>`

**Overview**

Tower is a library of modular and reusable components for building
robust networking clients and servers.

Tower provides a simple core abstraction, the `Service` trait, which
represents an asynchronous function taking a request and returning either a
response or an error. This abstraction can be used to model both clients and
servers.

Generic components, like [`timeout`], rate limiting, and load balancing,
can be modeled as `Service`s that wrap some inner service and apply
additional behavior before or after the inner service is called. This allows
implementing these components in a protocol-agnostic, composable way. Typically,
such services are referred to as middleware.

An additional abstraction, the `Layer` trait, is used to compose
middleware with `Service`s. If a `Service` can be thought of as an
asynchronous function from a request type to a response type, a `Layer` is
a function taking a `Service` of one type and returning a `Service` of a
different type. The `ServiceBuilder` type is used to add middleware to a
service by composing it with multiple `Layer`s.

**The Tower Ecosystem**

Tower is made up of the following crates:

- `tower` (this crate)

- `tower-service`

- `tower-layer`

- `tower-test`

Since the `Service` and `Layer` traits are important integration points
for all libraries using Tower, they are kept as stable as possible, and
breaking changes are made rarely. Therefore, they are defined in separate
crates, `tower-service` and `tower-layer`. This crate contains
re-exports of those core traits, implementations of commonly-used
middleware, and utilities for working with `Service`s and `Layer`s.
Finally, the `tower-test` crate provides tools for testing programs using
Tower.

**Usage**

Tower provides an abstraction layer, and generic implementations of various
middleware. This means that the `tower` crate on its own does not provide
a working implementation of a network client or server. Instead, Tower’s
`Service` trait provides an integration point between
application code, libraries providing middleware implementations, and
libraries that implement servers and/or clients for various network
protocols.

Depending on your particular use case, you might use Tower in several ways:

- 

Implementing application logic for a networked program. You might
use the `Service` trait to

...

## Items

- `builder` — Builder types to compose layers and services
- `layer` — A collection of Layer based tower services
- `make` — Trait aliases for Services that produce specific types of Responses.
- `util` — Various utility types and functions that are generally used with Tower.
- `ServiceBuilder` — Declaratively construct Service values.
- `Layer` — Decorates a Service, transforming either the request or the response.
- `MakeService` — Creates new Service values.
- `Service` — An asynchronous function from a Request to a Response.
- `ServiceExt` — An extension trait for Services that provides a variety of convenient adapters
- `service_fn` — Returns a new ServiceFn with the given closure.
- `BoxError` — Alias for a type-erased error type.

### fn tower::service_fn

```rust
pub fn service_fn<T>(f: T) -> ServiceFn<T>
```

Returns a new `ServiceFn` with the given closure.

This lets you build a `Service` from an async function that returns a `Result`.

**Example**

```rust
use tower::{service_fn, Service, ServiceExt, BoxError};

async fn handle(request: Request) -> Result<Response, BoxError> {
    let response = Response::new("Hello, World!");
    Ok(response)
}

let mut service = service_fn(handle);

let response = service
    .ready()
    .await?
    .call(Request::new())
    .await?;

assert_eq!("Hello, World!", response.into_body());
```

### struct tower::ServiceBuilder

```rust
pub struct ServiceBuilder<L> { /* private fields */ }
```

Declaratively construct `Service` values.

`ServiceBuilder` provides a builder-like interface for composing
layers to be applied to a `Service`.

**Service**

A `Service` is a trait representing an asynchronous function of a request
to a response. It is similar to `async fn(Request) -> Result<Response, Error>`.

A `Service` is typically bound to a single transport, such as a TCP
connection.  It defines how all inbound or outbound requests are handled
by that connection.

**Order**

The order in which layers are added impacts how requests are handled. Layers
that are added first will be called with the request first. The argument to
`service` will be last to see the request.

```rust
ServiceBuilder::new()
    .buffer(100)
    .concurrency_limit(10)
    .service(svc)
```

In the above example, the buffer layer receives the request first followed
by `concurrency_limit`. `buffer` enables up to 100 request to be in-flight
on top of the requests that have already been forwarded to the next
layer. Combined with `concurrency_limit`, this allows up to 110 requests to be
in-flight.

```rust
ServiceBuilder::new()
    .concurrency_limit(10)
    .buffer(100)
    .service(svc)
```

The above example is similar, but the order of layers is reversed. Now,
`concurrency_limit` applies first and only allows 10 requests to be in-flight
total.

**Examples**

A `Service` stack with a single layer:

```rust
ServiceBuilder::new()
    .concurrency_limit(5)
    .service(svc);
```

A `Service` stack with multiple layers that contain rate limiting,
in-flight request limits, and a channel-backed, clonab

...

**Methods**

- `pub const fn new() -> Self`
- `pub fn layer<T>(self, layer: T) -> ServiceBuilder<Stack<T, L>>`
- `pub fn option_layer<T>( self, layer: Option<T>, ) -> ServiceBuilder<Stack<Either<T, Identity>, L>>`
- `pub fn layer_fn<F>(self, f: F) -> ServiceBuilder<Stack<LayerFn<F>, L>>`
- `pub fn map_request<F, R1, R2>( self, f: F, ) -> ServiceBuilder<Stack<MapRequestLayer<F>, L>>where F: FnMut(R1) -> R2 + Clone,`
- `pub fn map_response<F>( self, f: F, ) -> ServiceBuilder<Stack<MapResponseLayer<F>, L>>`
- `pub fn map_err<F>(self, f: F) -> ServiceBuilder<Stack<MapErrLayer<F>, L>>`
- `pub fn map_future<F>(self, f: F) -> ServiceBuilder<Stack<MapFutureLayer<F>, L>>`
- `pub fn then<F>(self, f: F) -> ServiceBuilder<Stack<ThenLayer<F>, L>>`
- `pub fn and_then<F>(self, f: F) -> ServiceBuilder<Stack<AndThenLayer<F>, L>>`
- `pub fn map_result<F>(self, f: F) -> ServiceBuilder<Stack<MapResultLayer<F>, L>>`
- `pub fn into_inner(self) -> L`
- `pub fn service<S>(&self, service: S) -> L::Servicewhere L: Layer<S>,`
- `pub fn service_fn<F>(self, f: F) -> L::Servicewhere L: Layer<ServiceFn<F>>,`
- `pub fn check_clone(self) -> Selfwhere Self: Clone,`
- `pub fn check_service_clone<S>(self) -> Selfwhere L: Layer<S>, L::Service: Clone,`
- `pub fn check_service<S, T, U, E>(self) -> Selfwhere L: Layer<S>, L::Service: Service<T, Response = U, Error = E>,`
- `pub fn boxed<S, R>( self, ) -> ServiceBuilder<Stack<LayerFn<fn(L::Service) -> BoxService<R, <L::Service as Service<R>>::Response, <L::Service as Service<R>>::Error>>, L>>where L: Layer<S>, L::Service: Service<R> + Send + 'static, <L::Service as Service<R>>::Future: Send + 'static,`
- `pub fn boxed_clone<S, R>( self, ) -> ServiceBuilder<Stack<LayerFn<fn(L::Service) -> BoxCloneService<R, <L::Service as Service<R>>::Response, <L::Service as Service<R>>::Error>>, L>>where L: Layer<S>, L::Service: Service<R> + Clone + Send + 'static, <L::Service as Service<R>>::Future: Send + 'static,`
- `pub fn boxed_clone_sync<S, R>( self, ) -> ServiceBuilder<Stack<BoxCloneSyncServiceLayer<S, R, <L::Service as Service<R>>::Response, <L::Service as Service<R>>::Error>, Identity>>where L: Layer<S> + Send + Sync + 'static, L::Service: Service<R> + Clone + Send + Sync + 'static, <L::Service as Service<R>>::Future: Send + Sync + 'static,`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn layer(&self, inner: S) -> Self::Service`

### trait tower::Layer

```rust
pub trait Layer<S> {
    type Service;

    // Required method
    fn layer(&self, inner: S) -> Self::Service;
}
```

Decorates a `Service`, transforming either the request or the response.

Often, many of the pieces needed for writing network applications can be
reused across multiple services. The `Layer` trait can be used to write
reusable components that can be applied to very different kinds of services;
for example, it can be applied to services operating on different protocols,
and to both the client and server side of a network transaction.

**Log**

Take request logging as an example:

```rust

pub struct LogLayer {
    target: &'static str,
}

impl<S> Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(&self, service: S) -> Self::Service {
        LogService {
            target: self.target,
            service
        }
    }
}

// This service implements the Log behavior
pub struct LogService<S> {
    target: &'static str,
    service: S,
}

impl<S, Request> Service<Request> for LogService<S>
where
    S: Service<Request>,
    Request: fmt::Debug,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        // Insert log statement here or other functionality
        println!("request = {:?}, target = {:?}", request, self.target);
        self.service.call(request)
    }
}
```

The above log implementation is decoupled from the underlying protocol and
is also decoupled from client or server concerns. In other words, the same
log middle

...

**Methods**

- `type Service`
- `fn layer(&self, inner: S) -> Self::Service`
- `fn layer(&self, inner: S) -> <&'a T as Layer<S>>::Service`
- `fn layer( &self, service: S, ) -> <(L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13, L14, L15, L16) as Layer<S>>::Service`
- `fn layer( &self, service: S, ) -> <(L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13, L14, L15) as Layer<S>>::Service`
- `fn layer( &self, service: S, ) -> <(L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13, L14) as Layer<S>>::Service`
- `fn layer( &self, service: S, ) -> <(L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13) as Layer<S>>::Service`
- `fn layer( &self, service: S, ) -> <(L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12) as Layer<S>>::Service`
- `fn layer( &self, service: S, ) -> <(L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11) as Layer<S>>::Service`
- `fn layer( &self, service: S, ) -> <(L1, L2, L3, L4, L5, L6, L7, L8, L9, L10) as Layer<S>>::Service`
- `fn layer( &self, service: S, ) -> <(L1, L2, L3, L4, L5, L6, L7, L8, L9) as Layer<S>>::Service`
- `fn layer( &self, service: S, ) -> <(L1, L2, L3, L4, L5, L6, L7, L8) as Layer<S>>::Service`
- `fn layer( &self, service: S, ) -> <(L1, L2, L3, L4, L5, L6, L7) as Layer<S>>::Service`
- `fn layer(&self, service: S) -> <(L1, L2, L3, L4, L5, L6) as Layer<S>>::Service`
- `fn layer(&self, service: S) -> <(L1, L2, L3, L4, L5) as Layer<S>>::Service`
- `fn layer(&self, service: S) -> <(L1, L2, L3, L4) as Layer<S>>::Service`
- `fn layer(&self, service: S) -> <(L1, L2, L3) as Layer<S>>::Service`
- `fn layer(&self, service: S) -> <(L1, L2) as Layer<S>>::Service`
- `fn layer(&self, service: S) -> <(L1,) as Layer<S>>::Service`
- `fn layer(&self, service: S) -> <() as Layer<S>>::Service`

### trait tower::MakeService

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

### trait tower::Service

```rust
pub trait Service<Request> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    // Required methods
    fn poll_ready(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>>;
    fn call(&mut self, req: Request) -> Self::Future;
}
```

An asynchronous function from a `Request` to a `Response`.

The `Service` trait is a simplified interface making it easy to write
network applications in a modular and reusable way, decoupled from the
underlying protocol. It is one of Tower’s fundamental abstractions.

**Functional**

A `Service` is a function of a `Request`. It immediately returns a
`Future` representing the eventual completion of processing the
request. The actual request processing may happen at any time in the
future, on any thread or executor. The processing may depend on calling
other services. At some point in the future, the processing will complete,
and the `Future` will resolve to a response or error.

At a high level, the `Service::call` function represents an RPC request. The
`Service` value can be a server or a client.

**Server**

An RPC server implements the `Service` trait. Requests received by the
server over the network are deserialized and then passed as an argument to the
server value. The returned response is sent back over the network.

As an example, here is how an HTTP request is processed by a server:

...

**Methods**

- `type Response`
- `type Error`
- `type Future: Future<Output = Result<Self::Response, Self::Error>>`
- `fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, req: Request) -> Self::Future`
- `fn poll_ready( &mut self, cx: &mut Context<'_>, ) -> Poll<Result<(), <S as Service<Request>>::Error>>`
- `fn call(&mut self, request: Request) -> <S as Service<Request>>::Future`
- `fn poll_ready( &mut self, cx: &mut Context<'_>, ) -> Poll<Result<(), <S as Service<Request>>::Error>>`
- `fn call(&mut self, request: Request) -> <S as Service<Request>>::Future`

### trait tower::ServiceExt

```rust
pub trait ServiceExt<Request>: Service<Request> {
    // Provided methods
    fn ready(&mut self) -> Ready<'_, Self, Request> ⓘ
       where Self: Sized { ... }
    fn ready_oneshot(self) -> ReadyOneshot<Self, Request> ⓘ
       where Self: Sized { ... }
    fn oneshot(self, req: Request) -> Oneshot<Self, Request> ⓘ
       where Self: Sized { ... }
    fn call_all<S>(self, reqs: S) -> CallAll<Self, S>
       where Self: Sized,
             S: Stream<Item = Request> { ... }
    fn and_then<F>(self, f: F) -> AndThen<Self, F>
       where Self: Sized,
             F: Clone { ... }
    fn map_response<F, Response>(self, f: F) -> MapResponse<Self, F>
       where Self: Sized,
             F: FnOnce(Self::Response) -> Response + Clone { ... }
    fn map_err<F, Error>(self, f: F) -> MapErr<Self, F>
       where Self: Sized,
             F: FnOnce(Self::Error) -> Error + Clone { ... }
    fn map_result<F, Response, Error>(self, f: F) -> MapResult<Self, F>
       where Self: Sized,
             Error: From<Self::Error>,
             F: FnOnce(Result<Self::Response, Self::Error>) -> Result<Response, Error> + Clone { ... }
    fn map_request<F, NewRequest>(self, f: F) -> MapRequest<Self, F>
       where Self: Sized,
             F: FnMut(NewRequest) -> Request { ... }
    fn then<F, Response, Error, Fut>(self, f: F) -> Then<Self, F>
       where Self: Sized,
             Error: From<Self::Error>,
             F: FnOnce(Result<Self::Response, Self::Error>) -> Fut + Clone,
             Fut: Future<Output = Result<Response, Error>> { ... }
    fn map_future<F, Fut, Response, Error>(self, f: F) -> MapFuture<Self, F>
       where Self: Sized,
             F: FnMut(Self::Future) -> Fut,
             Error: From<Self::Error>,
             Fut: Future<Output = Result<Response, Error>> { ... }
    fn boxed(self) -> BoxService<Request, Self::Response, Self::Error>
       where Self: Sized + Send + 'static,
             Self::Future: Send + 'static { ... }
    fn boxed_clone(
        self,
    ) -> BoxCloneService<Request, Self::Response, Self::Error>
       where Self: Clone + Sized + Send + 'static,
             Self::Future: Send + 'static { ... }
}
```

An extension trait for `Service`s that provides a variety of convenient
adapters

**Methods**

- `fn ready(&mut self) -> Ready<'_, Self, Request> ⓘwhere Self: Sized,`
- `fn ready_oneshot(self) -> ReadyOneshot<Self, Request> ⓘwhere Self: Sized,`
- `fn oneshot(self, req: Request) -> Oneshot<Self, Request> ⓘwhere Self: Sized,`
- `fn call_all<S>(self, reqs: S) -> CallAll<Self, S>where Self: Sized, S: Stream<Item = Request>,`
- `fn and_then<F>(self, f: F) -> AndThen<Self, F>where Self: Sized, F: Clone,`
- `fn map_response<F, Response>(self, f: F) -> MapResponse<Self, F>where Self: Sized, F: FnOnce(Self::Response) -> Response + Clone,`
- `fn map_err<F, Error>(self, f: F) -> MapErr<Self, F>where Self: Sized, F: FnOnce(Self::Error) -> Error + Clone,`
- `fn map_result<F, Response, Error>(self, f: F) -> MapResult<Self, F>where Self: Sized, Error: From<Self::Error>, F: FnOnce(Result<Self::Response, Self::Error>) -> Result<Response, Error> + Clone,`
- `fn map_request<F, NewRequest>(self, f: F) -> MapRequest<Self, F>where Self: Sized, F: FnMut(NewRequest) -> Request,`
- `fn then<F, Response, Error, Fut>(self, f: F) -> Then<Self, F>where Self: Sized, Error: From<Self::Error>, F: FnOnce(Result<Self::Response, Self::Error>) -> Fut + Clone, Fut: Future<Output = Result<Response, Error>>,`
- `fn map_future<F, Fut, Response, Error>(self, f: F) -> MapFuture<Self, F>where Self: Sized, F: FnMut(Self::Future) -> Fut, Error: From<Self::Error>, Fut: Future<Output = Result<Response, Error>>,`
- `fn boxed(self) -> BoxService<Request, Self::Response, Self::Error>where Self: Sized + Send + 'static, Self::Future: Send + 'static,`
- `fn boxed_clone(self) -> BoxCloneService<Request, Self::Response, Self::Error>where Self: Clone + Sized + Send + 'static, Self::Future: Send + 'static,`

### type tower::BoxError

```rust
pub type BoxError = Box<dyn Error + Send + Sync>;
```

Alias for a type-erased error type.
