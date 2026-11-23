# Module tower::builder

## Overview

Builder types to compose layers and services

## Items

- `ServiceBuilder` — Declaratively construct Service values.

### struct tower::builder::ServiceBuilder

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
