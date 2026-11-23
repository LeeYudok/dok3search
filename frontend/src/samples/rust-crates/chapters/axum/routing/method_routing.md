# Module axum::routing::method_routing

## Overview

Route to services and handlers based on HTTP methods.

## Items

- `MethodRouter` — A [Service] that accepts requests based on a MethodFilter and allows chaining additional handlers and services.
- `any` — Route requests with the given handler regardless of the method.
- `any_service` — Route requests to the given service regardless of its method.
- `connect` — Route CONNECT requests to the given handler.
- `connect_service` — Route CONNECT requests to the given service.
- `delete` — Route DELETE requests to the given handler.
- `delete_service` — Route DELETE requests to the given service.
- `get` — Route GET requests to the given handler.
- `get_service` — Route GET requests to the given service.
- `head` — Route HEAD requests to the given handler.
- `head_service` — Route HEAD requests to the given service.
- `on` — Route requests with the given method to the handler.
- `on_service` — Route requests with the given method to the service.
- `options` — Route OPTIONS requests to the given handler.
- `options_service` — Route OPTIONS requests to the given service.
- `patch` — Route PATCH requests to the given handler.
- `patch_service` — Route PATCH requests to the given service.
- `post` — Route POST requests to the given handler.
- `post_service` — Route POST requests to the given service.
- `put` — Route PUT requests to the given handler.
- `put_service` — Route PUT requests to the given service.
- `trace` — Route TRACE requests to the given handler.
- `trace_service` — Route TRACE requests to the given service.

### fn axum::routing::method_routing::any

```rust
pub fn any<H, T, S>(handler: H) -> MethodRouter<S, Infallible>where
    H: Handler<T, S>,
    T: 'static,
    S: Clone + Send + Sync + 'static,
```

Route requests with the given handler regardless of the method.

**Example**

```rust
use axum::{
    routing::any,
    Router,
};

async fn handler() {}

// All requests to `/` will go to `handler`.
let app = Router::new().route("/", any(handler));
```

Additional methods can still be chained:

```rust
use axum::{
    routing::any,
    Router,
};

async fn handler() {}

async fn other_handler() {}

// `POST /` goes to `other_handler`. All other requests go to `handler`
let app = Router::new().route("/", any(handler).post(other_handler));
```

### fn axum::routing::method_routing::any_service

```rust
pub fn any_service<T, S>(svc: T) -> MethodRouter<S, T::Error>where
    T: Service<Request> + Clone + Send + Sync + 'static,
    T::Response: IntoResponse + 'static,
    T::Future: Send + 'static,
    S: Clone,
```

Route requests to the given service regardless of its method.

**Example**

```rust
use axum::{
    extract::Request,
    Router,
    routing::any_service,
    body::Body,
};
use http::Response;
use std::convert::Infallible;

let service = tower::service_fn(|request: Request| async {
    Ok::<_, Infallible>(Response::new(Body::empty()))
});

// All requests to `/` will go to `service`.
let app = Router::new().route("/", any_service(service));
```

Additional methods can still be chained:

```rust
use axum::{
    extract::Request,
    Router,
    routing::any_service,
    body::Body,
};
use http::Response;
use std::convert::Infallible;

let service = tower::service_fn(|request: Request| async {
    // ...
});

let other_service = tower::service_fn(|request: Request| async {
    // ...
});

// `POST /` goes to `other_service`. All other requests go to `service`
let app = Router::new().route("/", any_service(service).post_service(other_service));
```

### fn axum::routing::method_routing::connect

```rust
pub fn connect<H, T, S>(handler: H) -> MethodRouter<S, Infallible>where
    H: Handler<T, S>,
    T: 'static,
    S: Clone + Send + Sync + 'static,
```

Route `CONNECT` requests to the given handler.

See `MethodFilter::CONNECT` for when you’d want to use this,
and `get` for an example.

### fn axum::routing::method_routing::connect_service

```rust
pub fn connect_service<T, S>(svc: T) -> MethodRouter<S, T::Error>where
    T: Service<Request> + Clone + Send + Sync + 'static,
    T::Response: IntoResponse + 'static,
    T::Future: Send + 'static,
    S: Clone,
```

Route `CONNECT` requests to the given service.

See `MethodFilter::CONNECT` for when you’d want to use this,
and `get_service` for an example.

### fn axum::routing::method_routing::delete

```rust
pub fn delete<H, T, S>(handler: H) -> MethodRouter<S, Infallible>where
    H: Handler<T, S>,
    T: 'static,
    S: Clone + Send + Sync + 'static,
```

Route `DELETE` requests to the given handler.

See `get` for an example.

### fn axum::routing::method_routing::delete_service

```rust
pub fn delete_service<T, S>(svc: T) -> MethodRouter<S, T::Error>where
    T: Service<Request> + Clone + Send + Sync + 'static,
    T::Response: IntoResponse + 'static,
    T::Future: Send + 'static,
    S: Clone,
```

Route `DELETE` requests to the given service.

See `get_service` for an example.

### fn axum::routing::method_routing::get

```rust
pub fn get<H, T, S>(handler: H) -> MethodRouter<S, Infallible>where
    H: Handler<T, S>,
    T: 'static,
    S: Clone + Send + Sync + 'static,
```

Route `GET` requests to the given handler.

**Example**

```rust
use axum::{
    routing::get,
    Router,
};

async fn handler() {}

// Requests to `GET /` will go to `handler`.
let app = Router::new().route("/", get(handler));
```

Note that `get` routes will also be called for `HEAD` requests but will have
the response body removed. Make sure to add explicit `HEAD` routes
afterwards.

### fn axum::routing::method_routing::get_service

```rust
pub fn get_service<T, S>(svc: T) -> MethodRouter<S, T::Error>where
    T: Service<Request> + Clone + Send + Sync + 'static,
    T::Response: IntoResponse + 'static,
    T::Future: Send + 'static,
    S: Clone,
```

Route `GET` requests to the given service.

**Example**

```rust
use axum::{
    extract::Request,
    Router,
    routing::get_service,
    body::Body,
};
use http::Response;
use std::convert::Infallible;

let service = tower::service_fn(|request: Request| async {
    Ok::<_, Infallible>(Response::new(Body::empty()))
});

// Requests to `GET /` will go to `service`.
let app = Router::new().route("/", get_service(service));
```

Note that `get` routes will also be called for `HEAD` requests but will have
the response body removed. Make sure to add explicit `HEAD` routes
afterwards.

### fn axum::routing::method_routing::head

```rust
pub fn head<H, T, S>(handler: H) -> MethodRouter<S, Infallible>where
    H: Handler<T, S>,
    T: 'static,
    S: Clone + Send + Sync + 'static,
```

Route `HEAD` requests to the given handler.

See `get` for an example.

### fn axum::routing::method_routing::head_service

```rust
pub fn head_service<T, S>(svc: T) -> MethodRouter<S, T::Error>where
    T: Service<Request> + Clone + Send + Sync + 'static,
    T::Response: IntoResponse + 'static,
    T::Future: Send + 'static,
    S: Clone,
```

Route `HEAD` requests to the given service.

See `get_service` for an example.

### fn axum::routing::method_routing::on

```rust
pub fn on<H, T, S>(
    filter: MethodFilter,
    handler: H,
) -> MethodRouter<S, Infallible>where
    H: Handler<T, S>,
    T: 'static,
    S: Clone + Send + Sync + 'static,
```

Route requests with the given method to the handler.

**Example**

```rust
use axum::{
    routing::on,
    Router,
    routing::MethodFilter,
};

async fn handler() {}

// Requests to `POST /` will go to `handler`.
let app = Router::new().route("/", on(MethodFilter::POST, handler));
```

### fn axum::routing::method_routing::on_service

```rust
pub fn on_service<T, S>(
    filter: MethodFilter,
    svc: T,
) -> MethodRouter<S, T::Error>where
    T: Service<Request> + Clone + Send + Sync + 'static,
    T::Response: IntoResponse + 'static,
    T::Future: Send + 'static,
    S: Clone,
```

Route requests with the given method to the service.

**Example**

```rust
use axum::{
    extract::Request,
    routing::on,
    Router,
    body::Body,
    routing::{MethodFilter, on_service},
};
use http::Response;
use std::convert::Infallible;

let service = tower::service_fn(|request: Request| async {
    Ok::<_, Infallible>(Response::new(Body::empty()))
});

// Requests to `POST /` will go to `service`.
let app = Router::new().route("/", on_service(MethodFilter::POST, service));
```

### fn axum::routing::method_routing::options

```rust
pub fn options<H, T, S>(handler: H) -> MethodRouter<S, Infallible>where
    H: Handler<T, S>,
    T: 'static,
    S: Clone + Send + Sync + 'static,
```

Route `OPTIONS` requests to the given handler.

See `get` for an example.

### fn axum::routing::method_routing::options_service

```rust
pub fn options_service<T, S>(svc: T) -> MethodRouter<S, T::Error>where
    T: Service<Request> + Clone + Send + Sync + 'static,
    T::Response: IntoResponse + 'static,
    T::Future: Send + 'static,
    S: Clone,
```

Route `OPTIONS` requests to the given service.

See `get_service` for an example.

### fn axum::routing::method_routing::patch

```rust
pub fn patch<H, T, S>(handler: H) -> MethodRouter<S, Infallible>where
    H: Handler<T, S>,
    T: 'static,
    S: Clone + Send + Sync + 'static,
```

Route `PATCH` requests to the given handler.

See `get` for an example.

### fn axum::routing::method_routing::patch_service

```rust
pub fn patch_service<T, S>(svc: T) -> MethodRouter<S, T::Error>where
    T: Service<Request> + Clone + Send + Sync + 'static,
    T::Response: IntoResponse + 'static,
    T::Future: Send + 'static,
    S: Clone,
```

Route `PATCH` requests to the given service.

See `get_service` for an example.

### fn axum::routing::method_routing::post

```rust
pub fn post<H, T, S>(handler: H) -> MethodRouter<S, Infallible>where
    H: Handler<T, S>,
    T: 'static,
    S: Clone + Send + Sync + 'static,
```

Route `POST` requests to the given handler.

See `get` for an example.

### fn axum::routing::method_routing::post_service

```rust
pub fn post_service<T, S>(svc: T) -> MethodRouter<S, T::Error>where
    T: Service<Request> + Clone + Send + Sync + 'static,
    T::Response: IntoResponse + 'static,
    T::Future: Send + 'static,
    S: Clone,
```

Route `POST` requests to the given service.

See `get_service` for an example.

### fn axum::routing::method_routing::put

```rust
pub fn put<H, T, S>(handler: H) -> MethodRouter<S, Infallible>where
    H: Handler<T, S>,
    T: 'static,
    S: Clone + Send + Sync + 'static,
```

Route `PUT` requests to the given handler.

See `get` for an example.

### fn axum::routing::method_routing::put_service

```rust
pub fn put_service<T, S>(svc: T) -> MethodRouter<S, T::Error>where
    T: Service<Request> + Clone + Send + Sync + 'static,
    T::Response: IntoResponse + 'static,
    T::Future: Send + 'static,
    S: Clone,
```

Route `PUT` requests to the given service.

See `get_service` for an example.

### fn axum::routing::method_routing::trace

```rust
pub fn trace<H, T, S>(handler: H) -> MethodRouter<S, Infallible>where
    H: Handler<T, S>,
    T: 'static,
    S: Clone + Send + Sync + 'static,
```

Route `TRACE` requests to the given handler.

See `get` for an example.

### fn axum::routing::method_routing::trace_service

```rust
pub fn trace_service<T, S>(svc: T) -> MethodRouter<S, T::Error>where
    T: Service<Request> + Clone + Send + Sync + 'static,
    T::Response: IntoResponse + 'static,
    T::Future: Send + 'static,
    S: Clone,
```

Route `TRACE` requests to the given service.

See `get_service` for an example.

### struct axum::routing::method_routing::MethodRouter

```rust
pub struct MethodRouter<S = (), E = Infallible> { /* private fields */ }
```

A [`Service`] that accepts requests based on a `MethodFilter` and
allows chaining additional handlers and services.

**When does `MethodRouter` implement [`Service`]?**

Whether or not `MethodRouter` implements [`Service`] depends on the state type it requires.

```rust
use tower::Service;
use axum::{routing::get, extract::{State, Request}, body::Body};

// this `MethodRouter` doesn't require any state, i.e. the state is `()`,
let method_router = get(|| async {});
// and thus it implements `Service`
assert_service(method_router);

// this requires a `String` and doesn't implement `Service`
let method_router = get(|_: State<String>| async {});
// until you provide the `String` with `.with_state(...)`
let method_router_with_state = method_router.with_state(String::new());
// and then it implements `Service`
assert_service(method_router_with_state);

// helper to check that a value implements `Service`
fn assert_service<S>(service: S)
where
    S: Service<Request>,
{}
```

**Methods**

- `pub fn on<H, T>(self, filter: MethodFilter, handler: H) -> Selfwhere H: Handler<T, S>, T: 'static, S: Send + Sync + 'static,`
- `pub fn connect<H, T>(self, handler: H) -> Selfwhere H: Handler<T, S>, T: 'static, S: Send + Sync + 'static,`
- `pub fn delete<H, T>(self, handler: H) -> Selfwhere H: Handler<T, S>, T: 'static, S: Send + Sync + 'static,`
- `pub fn get<H, T>(self, handler: H) -> Selfwhere H: Handler<T, S>, T: 'static, S: Send + Sync + 'static,`
- `pub fn head<H, T>(self, handler: H) -> Selfwhere H: Handler<T, S>, T: 'static, S: Send + Sync + 'static,`
- `pub fn options<H, T>(self, handler: H) -> Selfwhere H: Handler<T, S>, T: 'static, S: Send + Sync + 'static,`
- `pub fn patch<H, T>(self, handler: H) -> Selfwhere H: Handler<T, S>, T: 'static, S: Send + Sync + 'static,`
- `pub fn post<H, T>(self, handler: H) -> Selfwhere H: Handler<T, S>, T: 'static, S: Send + Sync + 'static,`
- `pub fn put<H, T>(self, handler: H) -> Selfwhere H: Handler<T, S>, T: 'static, S: Send + Sync + 'static,`
- `pub fn trace<H, T>(self, handler: H) -> Selfwhere H: Handler<T, S>, T: 'static, S: Send + Sync + 'static,`
- `pub fn fallback<H, T>(self, handler: H) -> Selfwhere H: Handler<T, S>, T: 'static, S: Send + Sync + 'static,`
- `pub fn into_make_service(self) -> IntoMakeService<Self>`
- `pub fn into_make_service_with_connect_info<C>( self, ) -> IntoMakeServiceWithConnectInfo<Self, C>`
- `pub fn new() -> Self`
- `pub fn with_state<S2>(self, state: S) -> MethodRouter<S2, E>`
- `pub fn on_service<T>(self, filter: MethodFilter, svc: T) -> Selfwhere T: Service<Request, Error = E> + Clone + Send + Sync + 'static, T::Response: IntoResponse + 'static, T::Future: Send + 'static,`
- `pub fn connect_service<T>(self, svc: T) -> Selfwhere T: Service<Request, Error = E> + Clone + Send + Sync + 'static, T::Response: IntoResponse + 'static, T::Future: Send + 'static,`
- `pub fn delete_service<T>(self, svc: T) -> Selfwhere T: Service<Request, Error = E> + Clone + Send + Sync + 'static, T::Response: IntoResponse + 'static, T::Future: Send + 'static,`
- `pub fn get_service<T>(self, svc: T) -> Selfwhere T: Service<Request, Error = E> + Clone + Send + Sync + 'static, T::Response: IntoResponse + 'static, T::Future: Send + 'static,`
- `pub fn head_service<T>(self, svc: T) -> Selfwhere T: Service<Request, Error = E> + Clone + Send + Sync + 'static, T::Response: IntoResponse + 'static, T::Future: Send + 'static,`
- `pub fn options_service<T>(self, svc: T) -> Selfwhere T: Service<Request, Error = E> + Clone + Send + Sync + 'static, T::Response: IntoResponse + 'static, T::Future: Send + 'static,`
- `pub fn patch_service<T>(self, svc: T) -> Selfwhere T: Service<Request, Error = E> + Clone + Send + Sync + 'static, T::Response: IntoResponse + 'static, T::Future: Send + 'static,`
- `pub fn post_service<T>(self, svc: T) -> Selfwhere T: Service<Request, Error = E> + Clone + Send + Sync + 'static, T::Response: IntoResponse + 'static, T::Future: Send + 'static,`
- `pub fn put_service<T>(self, svc: T) -> Selfwhere T: Service<Request, Error = E> + Clone + Send + Sync + 'static, T::Response: IntoResponse + 'static, T::Future: Send + 'static,`
- `pub fn trace_service<T>(self, svc: T) -> Selfwhere T: Service<Request, Error = E> + Clone + Send + Sync + 'static, T::Response: IntoResponse + 'static, T::Future: Send + 'static,`
- `pub fn fallback_service<T>(self, svc: T) -> Selfwhere T: Service<Request, Error = E> + Clone + Send + Sync + 'static, T::Response: IntoResponse + 'static, T::Future: Send + 'static,`
- `pub fn layer<L, NewError>(self, layer: L) -> MethodRouter<S, NewError>where L: Layer<Route<E>> + Clone + Send + Sync + 'static, L::Service: Service<Request> + Clone + Send + Sync + 'static, <L::Service as Service<Request>>::Response: IntoResponse + 'static, <L::Service as Service<Request>>::Error: Into<NewError> + 'static, <L::Service as Service<Request>>::Future: Send + 'static, E: 'static, S: 'static, NewError: 'static,`
- `pub fn route_layer<L>(self, layer: L) -> MethodRouter<S, E>where L: Layer<Route<E>> + Clone + Send + Sync + 'static, L::Service: Service<Request, Error = E> + Clone + Send + Sync + 'static, <L::Service as Service<Request>>::Response: IntoResponse + 'static, <L::Service as Service<Request>>::Future: Send + 'static, E: 'static, S: 'static,`
- `pub fn merge(self, other: MethodRouter<S, E>) -> Self`
- `pub fn handle_error<F, T>(self, f: F) -> MethodRouter<S, Infallible>where F: Clone + Send + Sync + 'static, HandleError<Route<E>, F, T>: Service<Request, Error = Infallible>, <HandleError<Route<E>, F, T> as Service<Request>>::Future: Send, <HandleError<Route<E>, F, T> as Service<Request>>::Response: IntoResponse + Send, T: 'static, E: 'static, S: 'static,`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn call(self, req: Request, state: S) -> Self::Future`
- `fn layer<L>(self, layer: L) -> Layered<L, Self, T, S>where L: Layer<HandlerService<Self, T, S>> + Clone, L::Service: Service<Request>,`
- `fn with_state(self, state: S) -> HandlerService<Self, T, S>`
- `fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, _req: IncomingStream<'_, L>) -> Self::Future`
- `fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
