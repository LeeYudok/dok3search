# Module tower::util

## Overview

Various utility types and functions that are generally used with Tower.

## Items

- `error` — Error types
- `future` — Future types
- `rng` — PRNG utilities for tower middleware.
- `AndThen` — Service returned by the and_then combinator.
- `AndThenLayer` — A Layer that produces a AndThen service.
- `BoxCloneService` — A Clone + Send boxed Service.
- `BoxCloneServiceLayer` — A Clone + Send boxed Layer.
- `BoxCloneSyncService` — A Clone + Send + Sync boxed Service.
- `BoxCloneSyncServiceLayer` — A Clone + Send + Sync boxed Layer.
- `BoxLayer` — A boxed Layer trait object.
- `BoxService` — A boxed Service + Send trait object.
- `CallAll` — This is a Stream of responses resulting from calling the wrapped Service for each request received on the wrapped Stream.
- `CallAllUnordered` — A stream of responses received from the inner service in received order.
- `FutureService` — A type that implements Service for a Future that produces a Service.
- `MapErr` — Service returned by the map_err combinator.
- `MapErrLayer` — A Layer that produces MapErr services.
- `MapFuture` — Service returned by the map_future combinator.
- `MapFutureLayer` — A Layer that produces a MapFuture service.
- `MapRequest` — Service returned by the MapRequest combinator.
- `MapRequestLayer` — A Layer that produces MapRequest services.
- `MapResponse` — Service returned by the map_response combinator.
- `MapResponseLayer` — A Layer that produces a MapResponse service.
- `MapResult` — Service returned by the map_result combinator.
- `MapResultLayer` — A Layer that produces a MapResult service.
- `Oneshot` — A Future consuming a Service and request, waiting until the Service is ready, and then calling Service::call with the request, and waiting for that Future.
- `Optional` — Optionally forwards requests to an inner service.
- `Ready` — A future that yields a mutable reference to the service when it is ready to accept a request.
- `ReadyOneshot` — A Future that yields the service when it is ready to accept a request.
- `ServiceFn` — A Service implemented by a closure.
- `Then` — Service returned by the then combinator.
- `ThenLayer` — A Layer that produces a Then service.
- `UnsyncBoxService` — A boxed Service trait object.
- `Either` — Combine two different service types into a single type.
- `ServiceExt` — An extension trait for Services that provides a variety of convenient adapters
- `future_service` — Returns a new FutureService for the given future.
- `option_layer` — Convert an Option<Layer> into a Layer.
- `service_fn` — Returns a new ServiceFn with the given closure.

### enum tower::util::Either

```rust
pub enum Either<A, B> {
    Left(A),
    Right(B),
}
```

Combine two different service types into a single type.

Both services must be of the same request, response, and error types.
`Either` is useful for handling conditional branching in service middleware
to different inner service types.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn layer(&self, inner: S) -> Self::Service`
- `fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, request: Request) -> Self::Future`

### fn tower::util::future_service

```rust
pub fn future_service<F, S, R, E>(future: F) -> FutureService<F, S>where
    F: Future<Output = Result<S, E>> + Unpin,
    S: Service<R, Error = E>,
```

Returns a new `FutureService` for the given future.

A `FutureService` allows you to treat a future that resolves to a service as a service. This
can be useful for services that are created asynchronously.

**Example**

```rust
use tower::{service_fn, Service, ServiceExt};
use tower::util::future_service;
use std::convert::Infallible;

// A future which outputs a type implementing `Service`.
let future_of_a_service = async {
    let svc = service_fn(|_req: ()| async { Ok::<_, Infallible>("ok") });
    Ok::<_, Infallible>(svc)
};

// Wrap the future with a `FutureService`, allowing it to be used
// as a service without awaiting the future's completion:
let mut svc = future_service(Box::pin(future_of_a_service));

// Now, when we wait for the service to become ready, it will
// drive the future to completion internally.
let svc = svc.ready().await.unwrap();
let res = svc.call(()).await.unwrap();
```

**Regarding the `Unpin` bound**

The `Unpin` bound on `F` is necessary because the future will be polled in
`Service::poll_ready` which doesn’t have a pinned receiver (it takes `&mut self` and not `self: Pin<&mut Self>`). So we cannot put the future into a `Pin` without requiring `Unpin`.

This will most likely come up if you’re calling `future_service` with an async block. In that
case you can use `Box::pin(async { ... })` as shown in the example.

### fn tower::util::option_layer

```rust
pub fn option_layer<L>(layer: Option<L>) -> Either<L, Identity>
```

Convert an `Option<Layer>` into a `Layer`.

```rust
use tower::util::option_layer;
// Layer to apply a timeout if configured
let maybe_timeout = option_layer(timeout.map(TimeoutLayer::new));

ServiceBuilder::new()
    .layer(maybe_timeout)
    .service(svc);
```

### fn tower::util::service_fn

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

### struct tower::util::AndThen

```rust
pub struct AndThen<S, F> { /* private fields */ }
```

Service returned by the `and_then` combinator.

**Methods**

- `pub const fn new(inner: S, f: F) -> Self`
- `pub fn layer(f: F) -> AndThenLayer<F>`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, request: Request) -> Self::Future`

### struct tower::util::AndThenLayer

```rust
pub struct AndThenLayer<F> { /* private fields */ }
```

A `Layer` that produces a `AndThen` service.

**Methods**

- `pub const fn new(f: F) -> Self`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn layer(&self, inner: S) -> Self::Service`

### struct tower::util::BoxCloneService

```rust
pub struct BoxCloneService<T, U, E>(/* private fields */);
```

A `Clone` + `Send` boxed `Service`.

`BoxCloneService` turns a service into a trait object, allowing the
response future type to be dynamic, and allowing the service to be cloned.

This is similar to `BoxService` except the resulting
service implements `Clone`.

**Example**

```rust
use tower::{Service, ServiceBuilder, BoxError, util::BoxCloneService};
use std::time::Duration;

// This service has a complex type that is hard to name
let service = ServiceBuilder::new()
    .map_request(|req| {
        println!("received request");
        req
    })
    .map_response(|res| {
        println!("response produced");
        res
    })
    .load_shed()
    .concurrency_limit(64)
    .timeout(Duration::from_secs(10))
    .service_fn(|req: Request| async {
        Ok::<_, BoxError>(Response::new())
    });

// `BoxCloneService` will erase the type so it's nameable
let service: BoxCloneService<Request, Response, BoxError> = BoxCloneService::new(service);

// And we can still clone the service
let cloned_service = service.clone();
```

**Methods**

- `pub fn new<S>(inner: S) -> Selfwhere S: Service<T, Response = U, Error = E> + Clone + Send + 'static, S::Future: Send + 'static,`
- `pub fn layer<S>() -> LayerFn<fn(S) -> Self>where S: Service<T, Response = U, Error = E> + Clone + Send + 'static, S::Future: Send + 'static,`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), E>>`
- `fn call(&mut self, request: T) -> Self::Future`

### struct tower::util::BoxCloneServiceLayer

```rust
pub struct BoxCloneServiceLayer<In, T, U, E> { /* private fields */ }
```

A `Clone` + `Send` boxed `Layer`.

`BoxCloneServiceLayer` turns a layer into a trait object, allowing both the `Layer` itself
and the output `Service` to be dynamic, while having consistent types.

This `Layer` produces `BoxCloneService` instances erasing the type of the
`Service` produced by the wrapped `Layer`.

This is similar to `BoxLayer` except the layer and resulting
service implements `Clone`.

**Example**

`BoxCloneServiceLayer` can, for example, be useful to create layers dynamically that otherwise wouldn’t have
the same types, when the underlying service must be clone (for example, when building a MakeService)
In this example, we include a `Timeout` layer only if an environment variable is set. We can use
`BoxCloneService` to return a consistent type regardless of runtime configuration:

...

**Methods**

- `pub fn new<L>(inner_layer: L) -> Selfwhere L: Layer<In> + Send + Sync + 'static, L::Service: Service<T, Response = U, Error = E> + Send + Clone + 'static, <L::Service as Service<T>>::Future: Send + 'static,`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `fn layer(&self, inner: In) -> Self::Service`

### struct tower::util::BoxCloneSyncService

```rust
pub struct BoxCloneSyncService<T, U, E>(/* private fields */);
```

A `Clone` + `Send` + `Sync` boxed `Service`.

`BoxCloneSyncService` turns a service into a trait object, allowing the
response future type to be dynamic, and allowing the service to be cloned and shared.

This is similar to `BoxCloneService` except the resulting
service implements `Sync`.

**Methods**

- `pub fn new<S>(inner: S) -> Selfwhere S: Service<T, Response = U, Error = E> + Clone + Send + Sync + 'static, S::Future: Send + 'static,`
- `pub fn layer<S>() -> LayerFn<fn(S) -> Self>where S: Service<T, Response = U, Error = E> + Clone + Send + Sync + 'static, S::Future: Send + 'static,`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), E>>`
- `fn call(&mut self, request: T) -> Self::Future`

### struct tower::util::BoxCloneSyncServiceLayer

```rust
pub struct BoxCloneSyncServiceLayer<In, T, U, E> { /* private fields */ }
```

A `Clone` + `Send` + `Sync` boxed `Layer`.

`BoxCloneSyncServiceLayer` turns a layer into a trait object, allowing both the `Layer` itself
and the output `Service` to be dynamic, while having consistent types.

This `Layer` produces `BoxCloneSyncService` instances erasing the type of the
`Service` produced by the wrapped `Layer`.

This is similar to `BoxCloneServiceLayer` except the layer and resulting
service implements `Sync`.

**Example**

`BoxCloneSyncServiceLayer` can, for example, be useful to create layers dynamically that otherwise wouldn’t have
the same types, when the underlying service must be clone and sync (for example, when building a Hyper connector).
In this example, we include a `Timeout` layer only if an environment variable is set. We can use
`BoxCloneSyncServiceLayer` to return a consistent type regardless of runtime configuration:

...

**Methods**

- `pub fn new<L>(inner_layer: L) -> Selfwhere L: Layer<In> + Send + Sync + 'static, L::Service: Service<T, Response = U, Error = E> + Send + Sync + Clone + 'static, <L::Service as Service<T>>::Future: Send + 'static,`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `fn layer(&self, inner: In) -> Self::Service`

### struct tower::util::BoxLayer

```rust
pub struct BoxLayer<In, T, U, E> { /* private fields */ }
```

A boxed `Layer` trait object.

`BoxLayer` turns a layer into a trait object, allowing both the `Layer` itself
and the output `Service` to be dynamic, while having consistent types.

This `Layer` produces `BoxService` instances erasing the type of the
`Service` produced by the wrapped `Layer`.

**Example**

`BoxLayer` can, for example, be useful to create layers dynamically that otherwise wouldn’t have
the same types. In this example, we include a `Timeout` layer
only if an environment variable is set. We can use `BoxLayer`
to return a consistent type regardless of runtime configuration:

```rust
use std::time::Duration;
use tower::{Service, ServiceBuilder, BoxError, util::BoxLayer};

fn common_layer<S, T>() -> BoxLayer<S, T, S::Response, BoxError>
where
    S: Service<T> + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<BoxError> + 'static,
{
    let builder = ServiceBuilder::new()
        .concurrency_limit(100);

    if std::env::var("SET_TIMEOUT").is_ok() {
        let layer = builder
            .timeout(Duration::from_secs(30))
            .into_inner();

        BoxLayer::new(layer)
    } else {
        let layer = builder
            .map_err(Into::into)
            .into_inner();

        BoxLayer::new(layer)
    }
}
```

**Methods**

- `pub fn new<L>(inner_layer: L) -> Selfwhere L: Layer<In> + Send + Sync + 'static, L::Service: Service<T, Response = U, Error = E> + Send + 'static, <L::Service as Service<T>>::Future: Send + 'static,`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `fn layer(&self, inner: In) -> Self::Service`

### struct tower::util::BoxService

```rust
pub struct BoxService<T, U, E> { /* private fields */ }
```

A boxed `Service + Send` trait object.

`BoxService` turns a service into a trait object, allowing the response
future type to be dynamic. This type requires both the service and the
response future to be `Send`.

If you need a boxed `Service` that implements `Clone` consider using
`BoxCloneService`.

Dynamically dispatched `Service` objects allow for erasing the underlying
`Service` type and using the `Service` instances as opaque handles. This can
be useful when the service instance cannot be explicitly named for whatever
reason.

**Examples**

```rust
use std::future::ready;
// Respond to requests using a closure, but closures cannot be named...
let svc = service_fn(|mut request: String| {
    request.push_str(" response");
    ready(Ok(request))
});

let service: BoxService<String, String, ()> = BoxService::new(svc);
}
```

**Methods**

- `pub fn new<S>(inner: S) -> Selfwhere S: Service<T, Response = U, Error = E> + Send + 'static, S::Future: Send + 'static,`
- `pub fn layer<S>() -> LayerFn<fn(S) -> Self>where S: Service<T, Response = U, Error = E> + Send + 'static, S::Future: Send + 'static,`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), E>>`
- `fn call( &mut self, request: T, ) -> Pin<Box<dyn Future<Output = Result<U, E>> + Send>>`

### struct tower::util::CallAll

```rust
pub struct CallAll<Svc, S>where
    Svc: Service<S::Item>,
    S: Stream,{ /* private fields */ }
```

This is a `Stream` of responses resulting from calling the wrapped `Service` for each
request received on the wrapped `Stream`.

...

**Methods**

- `pub fn new(service: Svc, stream: S) -> CallAll<Svc, S>`
- `pub fn into_inner(self) -> Svc`
- `pub fn take_service(self: Pin<&mut Self>) -> Svc`
- `pub fn unordered(self) -> CallAllUnordered<Svc, S>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_next( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Option<Self::Item>>`
- `fn size_hint(&self) -> (usize, Option<usize>)`

### struct tower::util::CallAllUnordered

```rust
pub struct CallAllUnordered<Svc, S>where
    Svc: Service<S::Item>,
    S: Stream,{ /* private fields */ }
```

A stream of responses received from the inner service in received order.

Similar to `CallAll` except, instead of yielding responses in request order,
responses are returned as they are available.

**Methods**

- `pub fn new(service: Svc, stream: S) -> CallAllUnordered<Svc, S>`
- `pub fn into_inner(self) -> Svc`
- `pub fn take_service(self: Pin<&mut Self>) -> Svc`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_next( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Option<Self::Item>>`
- `fn size_hint(&self) -> (usize, Option<usize>)`

### struct tower::util::FutureService

```rust
pub struct FutureService<F, S> { /* private fields */ }
```

A type that implements `Service` for a `Future` that produces a `Service`.

See `future_service` for more details.

**Methods**

- `pub const fn new(future: F) -> Self`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, req: R) -> Self::Future`

### struct tower::util::MapErr

```rust
pub struct MapErr<S, F> { /* private fields */ }
```

Service returned by the `map_err` combinator.

**Methods**

- `pub const fn new(inner: S, f: F) -> Self`
- `pub fn layer(f: F) -> MapErrLayer<F>`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, request: Request) -> Self::Future`

### struct tower::util::MapErrLayer

```rust
pub struct MapErrLayer<F> { /* private fields */ }
```

A `Layer` that produces `MapErr` services.

**Methods**

- `pub const fn new(f: F) -> Self`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn layer(&self, inner: S) -> Self::Service`

### struct tower::util::MapFuture

```rust
pub struct MapFuture<S, F> { /* private fields */ }
```

`Service` returned by the `map_future` combinator.

**Methods**

- `pub const fn new(inner: S, f: F) -> Self`
- `pub fn layer(f: F) -> MapFutureLayer<F>`
- `pub fn get_ref(&self) -> &S`
- `pub fn get_mut(&mut self) -> &mut S`
- `pub fn into_inner(self) -> S`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, req: R) -> Self::Future`

### struct tower::util::MapFutureLayer

```rust
pub struct MapFutureLayer<F> { /* private fields */ }
```

A `Layer` that produces a `MapFuture` service.

**Methods**

- `pub const fn new(f: F) -> Self`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn layer(&self, inner: S) -> Self::Service`

### struct tower::util::MapRequest

```rust
pub struct MapRequest<S, F> { /* private fields */ }
```

Service returned by the `MapRequest` combinator.

**Methods**

- `pub const fn new(inner: S, f: F) -> Self`
- `pub fn layer(f: F) -> MapRequestLayer<F>`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), S::Error>>`
- `fn call(&mut self, request: R1) -> S::Future`

### struct tower::util::MapRequestLayer

```rust
pub struct MapRequestLayer<F> { /* private fields */ }
```

A `Layer` that produces `MapRequest` services.

**Methods**

- `pub const fn new(f: F) -> Self`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn layer(&self, inner: S) -> Self::Service`

### struct tower::util::MapResponse

```rust
pub struct MapResponse<S, F> { /* private fields */ }
```

Service returned by the `map_response` combinator.

**Methods**

- `pub const fn new(inner: S, f: F) -> Self`
- `pub fn layer(f: F) -> MapResponseLayer<F>`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, request: Request) -> Self::Future`

### struct tower::util::MapResponseLayer

```rust
pub struct MapResponseLayer<F> { /* private fields */ }
```

A `Layer` that produces a `MapResponse` service.

**Methods**

- `pub const fn new(f: F) -> Self`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn layer(&self, inner: S) -> Self::Service`

### struct tower::util::MapResult

```rust
pub struct MapResult<S, F> { /* private fields */ }
```

Service returned by the `map_result` combinator.

**Methods**

- `pub const fn new(inner: S, f: F) -> Self`
- `pub fn layer(f: F) -> MapResultLayer<F>`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, request: Request) -> Self::Future`

### struct tower::util::MapResultLayer

```rust
pub struct MapResultLayer<F> { /* private fields */ }
```

A `Layer` that produces a `MapResult` service.

**Methods**

- `pub const fn new(f: F) -> Self`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn layer(&self, inner: S) -> Self::Service`

### struct tower::util::Oneshot

```rust
pub struct Oneshot<S: Service<Req>, Req> { /* private fields */ }
```

A `Future` consuming a `Service` and request, waiting until the `Service`
is ready, and then calling `Service::call` with the request, and
waiting for that `Future`.

**Methods**

- `pub const fn new(svc: S, req: Req) -> Self`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`

### struct tower::util::Optional

```rust
pub struct Optional<T> { /* private fields */ }
```

Optionally forwards requests to an inner service.

If the inner service is `None`, `optional::None` is returned as the response.

**Methods**

- `pub const fn new<Request>(inner: Option<T>) -> Optional<T>where T: Service<Request>, T::Error: Into<BoxError>,`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, request: Request) -> Self::Future`

### struct tower::util::Ready

```rust
pub struct Ready<'a, T, Request>(/* private fields */);
```

A future that yields a mutable reference to the service when it is ready to accept a request.

`Ready` values are produced by `ServiceExt::ready`.

**Methods**

- `pub fn new(service: &'a mut T) -> Self`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`

### struct tower::util::ReadyOneshot

```rust
pub struct ReadyOneshot<T, Request> { /* private fields */ }
```

A `Future` that yields the service when it is ready to accept a request.

`ReadyOneshot` values are produced by `ServiceExt::ready_oneshot`.

**Methods**

- `pub const fn new(service: T) -> Self`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`

### struct tower::util::ServiceFn

```rust
pub struct ServiceFn<T> { /* private fields */ }
```

A `Service` implemented by a closure.

See `service_fn` for more details.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), E>>`
- `fn call(&mut self, req: Request) -> Self::Future`

### struct tower::util::Then

```rust
pub struct Then<S, F> { /* private fields */ }
```

`Service` returned by the `then` combinator.

**Methods**

- `pub const fn new(inner: S, f: F) -> Self`
- `pub fn layer(f: F) -> ThenLayer<F>`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, request: Request) -> Self::Future`

### struct tower::util::ThenLayer

```rust
pub struct ThenLayer<F> { /* private fields */ }
```

A `Layer` that produces a `Then` service.

**Methods**

- `pub const fn new(f: F) -> Self`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn layer(&self, inner: S) -> Self::Service`

### struct tower::util::UnsyncBoxService

```rust
pub struct UnsyncBoxService<T, U, E> { /* private fields */ }
```

A boxed `Service` trait object.

**Methods**

- `pub fn new<S>(inner: S) -> Selfwhere S: Service<T, Response = U, Error = E> + 'static, S::Future: 'static,`
- `pub fn layer<S>() -> LayerFn<fn(S) -> Self>where S: Service<T, Response = U, Error = E> + 'static, S::Future: 'static,`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), E>>`
- `fn call(&mut self, request: T) -> Pin<Box<dyn Future<Output = Result<U, E>>>>`

### trait tower::util::ServiceExt

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
