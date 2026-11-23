# Module axum::routing

## Overview

Routing between [`Service`]s and handlers.

## Items

- `future` — Future types.
- `method_routing` — Route to services and handlers based on HTTP methods.
- `IntoMakeService` — A MakeService that produces axum router services.
- `MethodFilter` — A filter that matches one or more HTTP methods.
- `Route` — How routes are stored inside a Router.
- `Router` — The router type for composing handlers and services.
- `RouterAsService` — A Router converted into a borrowed [Service] with a fixed body type.
- `RouterIntoService` — A Router converted into an owned [Service] with a fixed body type.

### struct axum::routing::IntoMakeService

```rust
pub struct IntoMakeService<S> { /* private fields */ }
```

A `MakeService` that produces axum router services.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, _target: T) -> Self::Future`

### struct axum::routing::MethodFilter

```rust
pub struct MethodFilter(/* private fields */);
```

A filter that matches one or more HTTP methods.

**Methods**

- `pub const fn or(self, other: Self) -> Self`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn try_from(m: Method) -> Result<Self, NoMatchingMethodFilter>`

### struct axum::routing::Route

```rust
pub struct Route<E = Infallible>(/* private fields */);
```

How routes are stored inside a `Router`.

You normally shouldn’t need to care about this type. It’s used in
`Router::layer`.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, req: Request<B>) -> Self::Future`

### struct axum::routing::Router

```rust
pub struct Router<S = ()> { /* private fields */ }
```

The router type for composing handlers and services.

`Router<S>` means a router that is missing a state of type `S` to be able
to handle requests. Thus, only `Router<()>` (i.e. without missing state) can
be passed to `serve`. See `Router::with_state` for more details.

**Methods**

- `pub fn new() -> Self`
- `pub fn without_v07_checks(self) -> Self`
- `pub fn route(self, path: &str, method_router: MethodRouter<S>) -> Self`
- `pub fn route_service<T>(self, path: &str, service: T) -> Selfwhere T: Service<Request, Error = Infallible> + Clone + Send + Sync + 'static, T::Response: IntoResponse, T::Future: Send + 'static,`
- `pub fn nest(self, path: &str, router: Router<S>) -> Self`
- `pub fn nest_service<T>(self, path: &str, service: T) -> Selfwhere T: Service<Request, Error = Infallible> + Clone + Send + Sync + 'static, T::Response: IntoResponse, T::Future: Send + 'static,`
- `pub fn merge<R>(self, other: R) -> Selfwhere R: Into<Router<S>>,`
- `pub fn layer<L>(self, layer: L) -> Router<S>where L: Layer<Route> + Clone + Send + Sync + 'static, L::Service: Service<Request> + Clone + Send + Sync + 'static, <L::Service as Service<Request>>::Response: IntoResponse + 'static, <L::Service as Service<Request>>::Error: Into<Infallible> + 'static, <L::Service as Service<Request>>::Future: Send + 'static,`
- `pub fn route_layer<L>(self, layer: L) -> Selfwhere L: Layer<Route> + Clone + Send + Sync + 'static, L::Service: Service<Request> + Clone + Send + Sync + 'static, <L::Service as Service<Request>>::Response: IntoResponse + 'static, <L::Service as Service<Request>>::Error: Into<Infallible> + 'static, <L::Service as Service<Request>>::Future: Send + 'static,`
- `pub fn has_routes(&self) -> bool`
- `pub fn fallback<H, T>(self, handler: H) -> Selfwhere H: Handler<T, S>, T: 'static,`
- `pub fn fallback_service<T>(self, service: T) -> Selfwhere T: Service<Request, Error = Infallible> + Clone + Send + Sync + 'static, T::Response: IntoResponse, T::Future: Send + 'static,`
- `pub fn method_not_allowed_fallback<H, T>(self, handler: H) -> Selfwhere H: Handler<T, S>, T: 'static,`
- `pub fn reset_fallback(self) -> Self`
- `pub fn with_state<S2>(self, state: S) -> Router<S2>`
- `pub fn as_service<B>(&mut self) -> RouterAsService<'_, B, S>`
- `pub fn into_service<B>(self) -> RouterIntoService<B, S>`
- `pub fn into_make_service(self) -> IntoMakeService<Self>`
- `pub fn into_make_service_with_connect_info<C>( self, ) -> IntoMakeServiceWithConnectInfo<Self, C>`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, _req: IncomingStream<'_, L>) -> Self::Future`
- `fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, req: Request<B>) -> Self::Future`

### struct axum::routing::RouterAsService

```rust
pub struct RouterAsService<'a, B, S = ()> { /* private fields */ }
```

A `Router` converted into a borrowed [`Service`] with a fixed body type.

See `Router::as_service` for more details.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, req: Request<B>) -> Self::Future`

### struct axum::routing::RouterIntoService

```rust
pub struct RouterIntoService<B, S = ()> { /* private fields */ }
```

A `Router` converted into an owned [`Service`] with a fixed body type.

See `Router::into_service` for more details.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, req: Request<B>) -> Self::Future`
