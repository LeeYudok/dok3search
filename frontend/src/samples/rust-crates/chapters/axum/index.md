# Crate axum

## Overview

axum is an HTTP routing and request-handling library that focuses on ergonomics and modularity.

**High-level features**

- Route requests to handlers with a macro-free API.

- Declaratively parse requests using extractors.

- Simple and predictable error handling model.

- Generate responses with minimal boilerplate.

- Take full advantage of the `tower` and `tower-http` ecosystem of
middleware, services, and utilities.

In particular, the last point is what sets `axum` apart from other libraries / frameworks.
`axum` doesn’t have its own middleware system but instead uses
`tower::Service`. This means `axum` gets timeouts, tracing, compression,
authorization, and more, for free. It also enables you to share middleware with
applications written using `hyper` or `tonic`.

**Compatibility**

axum is designed to work with [tokio] and [hyper]. Runtime and
transport layer independence is not a goal, at least for the time being.

**Example**

The “Hello, World!” of axum is:

```rust
use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

Note using `#[tokio::main]` requires you enable tokio’s `macros` and `rt-multi-thread` features
or just `full` to enable all features (`cargo add tokio --features macros,rt-multi-thread`).

**Routing**

`Router` is used to set up which paths go to which services:

```rust
use axum::{Router, routing::get};

// our router
let app = Router::new()
    .route("/", get(root))
    .route("/foo", get(get_foo).post(post_foo))
    .route("/foo/bar", get(foo_bar));

// which calls one of these handlers
async fn root() {}
async fn get_foo() {}
async fn post_foo() {}
async fn foo_bar() {}
```

See `Router` for more details on routing.

**Handlers**

In axum a “handler” is an async function that accepts zero or more
“extractors” as arguments and returns something that
can be converted into a response.

Handlers are where your application logic lives and axum applications are built
by routing between handlers.

See `handler` for more details on handlers.

**Extractors**

An extractor is a type that implemen

...

## Items

- `body` — HTTP body utilities.
- `error_handling` — Error handling model and utilities
- `extract` — Types and traits for extracting data from requests.
- `handler` — Async functions that can be used to handle requests.
- `middleware` — Utilities for writing middleware
- `response` — Types and traits for generating responses.
- `routing` — Routing between [Service]s and handlers.
- `serve` — Serve services.
- `Error` — Errors that can happen when using axum.
- `Extension` — Extractor and response for extensions.
- `Form` — URL encoded extractor and response.
- `Json` — JSON Extractor / Response.
- `Router` — The router type for composing handlers and services.
- `RequestExt` — Extension trait that adds additional methods to Request.
- `RequestPartsExt` — Extension trait that adds additional methods to [Parts].
- `ServiceExt` — Extension trait that adds additional methods to any [Service].
- `serve` — Serve the service with the supplied listener.
- `BoxError` — Alias for a type-erased error type.

### fn axum::serve

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

### struct axum::Error

```rust
pub struct Error { /* private fields */ }
```

Errors that can happen when using axum.

**Methods**

- `pub fn new(error: impl Into<Box<dyn Error + Send + Sync>>) -> Error`
- `pub fn into_inner(self) -> Box<dyn Error + Send + Sync>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`

### struct axum::Extension

```rust
pub struct Extension<T>(pub T);
```

Extractor and response for extensions.

**As extractor**

This is commonly used to share state across handlers.

```rust
use axum::{
    Router,
    Extension,
    routing::get,
};
use std::sync::Arc;

// Some shared state used throughout our application
struct State {
    // ...
}

async fn handler(state: Extension<Arc<State>>) {
    // ...
}

let state = Arc::new(State { /* ... */ });

let app = Router::new().route("/", get(handler))
    // Add middleware that inserts the state into all incoming request's
    // extensions.
    .layer(Extension(state));
```

If the extension is missing it will reject the request with a `500 Internal Server Error` response. Alternatively, you can use `Option<Extension<T>>` to
make the extension extractor optional.

**As response**

Response extensions can be used to share state with middleware.

```rust
use axum::{
    Extension,
    response::IntoResponse,
};

async fn handler() -> (Extension<Foo>, &'static str) {
    (
        Extension(Foo("foo")),
        "Hello, World!"
    )
}

#[derive(Clone)]
struct Foo(&'static str);
```

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn deref(&self) -> &Self::Target`
- `fn deref_mut(&mut self) -> &mut Self::Target`
- `async fn from_request_parts( req: &mut Parts, _state: &S, ) -> Result<Self, Self::Rejection>`
- `fn into_response(self) -> Response`
- `fn into_response_parts( self, res: ResponseParts, ) -> Result<ResponseParts, Self::Error>`
- `fn layer(&self, inner: S) -> Self::Service`
- `async fn from_request_parts( req: &mut Parts, _state: &S, ) -> Result<Option<Self>, Self::Rejection>`

### struct axum::Form

```rust
pub struct Form<T>(pub T);
```

URL encoded extractor and response.

**As extractor**

If used as an extractor, `Form` will deserialize form data from the request,
specifically:

- If the request has a method of `GET` or `HEAD`, the form data will be read
from the query string (same as with `Query`)

- If the request has a different method, the form will be read from the body
of the request. It must have a `content-type` of
`application/x-www-form-urlencoded` for this to work. If you want to parse
`multipart/form-data` request bodies, use `Multipart` instead.

This matches how HTML forms are sent by browsers by default.
In both cases, the inner type `T` must implement [`serde::Deserialize`].

⚠️ Since parsing form data might require consuming the request body, the `Form` extractor must be
last if there are multiple extractors in a handler. See “the order of
extractors”

```rust
use axum::Form;
use serde::Deserialize;

#[derive(Deserialize)]
struct SignUp {
    username: String,
    password: String,
}

async fn accept_form(Form(sign_up): Form<SignUp>) {
    // ...
}
```

**As response**

`Form` can also be used to encode any type that implements
[`serde::Serialize`] as `application/x-www-form-urlencoded`

```rust
use axum::Form;
use serde::Serialize;

#[derive(Serialize)]
struct Payload {
    value: String,
}

async fn handler() -> Form<Payload> {
    Form(Payload { value: "foo".to_owned() })
}
```

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn deref(&self) -> &Self::Target`
- `fn deref_mut(&mut self) -> &mut Self::Target`
- `async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection>`
- `fn into_response(self) -> Response`

### struct axum::Json

```rust
pub struct Json<T>(pub T);
```

JSON Extractor / Response.

When used as an extractor, it can deserialize request bodies into some type that
implements [`serde::de::DeserializeOwned`]. The request will be rejected (and a `JsonRejection` will
be returned) if:

- The request doesn’t have a `Content-Type: application/json` (or similar) header.

- The body doesn’t contain syntactically valid JSON.

- The body contains syntactically valid JSON, but it couldn’t be deserialized into the target type.

- Buffering the request body fails.

⚠️ Since parsing JSON requires consuming the request body, the `Json` extractor must be
last if there are multiple extractors in a handler.
See “the order of extractors”

See `JsonRejection` for more details.

**Extractor example**

```rust
use axum::{
    extract,
    routing::post,
    Router,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateUser {
    email: String,
    password: String,
}

async fn create_user(extract::Json(payload): extract::Json<CreateUser>) {
    // payload is a `CreateUser`
}

let app = Router::new().route("/users", post(create_user));
```

When used as a response, it can serialize any type that implements [`serde::Serialize`] to
`JSON`, and will automatically set `Content-Type: application/json` header.

If the `Serialize` implementation decides to fail
or if a map with non-string keys is used,
a 500 response will be issued
whose body is the error message in UTF-8.

**Response example**

...

**Methods**

- `pub fn from_bytes(bytes: &[u8]) -> Result<Self, JsonRejection>`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn deref(&self) -> &Self::Target`
- `fn deref_mut(&mut self) -> &mut Self::Target`
- `fn from(inner: T) -> Self`
- `async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection>`
- `fn into_response(self) -> Response`
- `async fn from_request( req: Request, state: &S, ) -> Result<Option<Self>, Self::Rejection>`

### struct axum::Router

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

### trait axum::RequestExt

```rust
pub trait RequestExt: Sized + Sealed {
    // Required methods
    fn extract<E, M>(
        self,
    ) -> impl Future<Output = Result<E, <E as FromRequest<(), M>>::Rejection>> + Send
       where E: FromRequest<(), M> + 'static,
             M: 'static;
    fn extract_with_state<E, S, M>(
        self,
        state: &S,
    ) -> impl Future<Output = Result<E, <E as FromRequest<S, M>>::Rejection>> + Send
       where E: FromRequest<S, M> + 'static,
             S: Send + Sync;
    fn extract_parts<E>(
        &mut self,
    ) -> impl Future<Output = Result<E, <E as FromRequestParts<()>>::Rejection>> + Send
       where E: FromRequestParts<()> + 'static;
    fn extract_parts_with_state<'a, E, S>(
        &'a mut self,
        state: &'a S,
    ) -> impl Future<Output = Result<E, <E as FromRequestParts<S>>::Rejection>> + Send + 'a
       where E: FromRequestParts<S> + 'static,
             S: Send + Sync;
    fn with_limited_body(self) -> Request<Body>;
    fn into_limited_body(self) -> Body;
}
```

Extension trait that adds additional methods to `Request`.

**Methods**

- `fn extract<E, M>( self, ) -> impl Future<Output = Result<E, <E as FromRequest<(), M>>::Rejection>> + Sendwhere E: FromRequest<(), M> + 'static, M: 'static,`
- `fn extract_with_state<E, S, M>( self, state: &S, ) -> impl Future<Output = Result<E, <E as FromRequest<S, M>>::Rejection>> + Sendwhere E: FromRequest<S, M> + 'static, S: Send + Sync,`
- `fn extract_parts<E>( &mut self, ) -> impl Future<Output = Result<E, <E as FromRequestParts<()>>::Rejection>> + Sendwhere E: FromRequestParts<()> + 'static,`
- `fn extract_parts_with_state<'a, E, S>( &'a mut self, state: &'a S, ) -> impl Future<Output = Result<E, <E as FromRequestParts<S>>::Rejection>> + Send + 'awhere E: FromRequestParts<S> + 'static, S: Send + Sync,`
- `fn with_limited_body(self) -> Request<Body>`
- `fn into_limited_body(self) -> Body`
- `fn extract<E, M>( self, ) -> impl Future<Output = Result<E, <E as FromRequest<(), M>>::Rejection>> + Sendwhere E: FromRequest<(), M> + 'static, M: 'static,`
- `fn extract_with_state<E, S, M>( self, state: &S, ) -> impl Future<Output = Result<E, <E as FromRequest<S, M>>::Rejection>> + Sendwhere E: FromRequest<S, M> + 'static, S: Send + Sync,`
- `fn extract_parts<E>( &mut self, ) -> impl Future<Output = Result<E, <E as FromRequestParts<()>>::Rejection>> + Sendwhere E: FromRequestParts<()> + 'static,`
- `async fn extract_parts_with_state<'a, E, S>( &'a mut self, state: &'a S, ) -> Result<E, <E as FromRequestParts<S>>::Rejection>where E: FromRequestParts<S> + 'static, S: Send + Sync,`
- `fn with_limited_body(self) -> Request<Body>`
- `fn into_limited_body(self) -> Body`

### trait axum::RequestPartsExt

```rust
pub trait RequestPartsExt: Sized + Sealed {
    // Required methods
    fn extract<E>(
        &mut self,
    ) -> impl Future<Output = Result<E, <E as FromRequestParts<()>>::Rejection>> + Send
       where E: FromRequestParts<()> + 'static;
    fn extract_with_state<'a, E, S>(
        &'a mut self,
        state: &'a S,
    ) -> impl Future<Output = Result<E, <E as FromRequestParts<S>>::Rejection>> + Send + 'a
       where E: FromRequestParts<S> + 'static,
             S: Send + Sync;
}
```

Extension trait that adds additional methods to [`Parts`].

**Methods**

- `fn extract<E>( &mut self, ) -> impl Future<Output = Result<E, <E as FromRequestParts<()>>::Rejection>> + Sendwhere E: FromRequestParts<()> + 'static,`
- `fn extract_with_state<'a, E, S>( &'a mut self, state: &'a S, ) -> impl Future<Output = Result<E, <E as FromRequestParts<S>>::Rejection>> + Send + 'awhere E: FromRequestParts<S> + 'static, S: Send + Sync,`
- `fn extract<E>( &mut self, ) -> impl Future<Output = Result<E, <E as FromRequestParts<()>>::Rejection>> + Sendwhere E: FromRequestParts<()> + 'static,`
- `fn extract_with_state<'a, E, S>( &'a mut self, state: &'a S, ) -> impl Future<Output = Result<E, <E as FromRequestParts<S>>::Rejection>> + Send + 'awhere E: FromRequestParts<S> + 'static, S: Send + Sync,`

### trait axum::ServiceExt

```rust
pub trait ServiceExt<R>: Service<R> + Sized {
    // Required methods
    fn into_make_service(self) -> IntoMakeService<Self>;
    fn into_make_service_with_connect_info<C>(
        self,
    ) -> IntoMakeServiceWithConnectInfo<Self, C>;

    // Provided method
    fn handle_error<F, T>(self, f: F) -> HandleError<Self, F, T> { ... }
}
```

Extension trait that adds additional methods to any [`Service`].

**Methods**

- `fn into_make_service(self) -> IntoMakeService<Self>`
- `fn into_make_service_with_connect_info<C>( self, ) -> IntoMakeServiceWithConnectInfo<Self, C>`
- `fn handle_error<F, T>(self, f: F) -> HandleError<Self, F, T>`

### type axum::BoxError

```rust
pub type BoxError = Box<dyn Error + Send + Sync>;
```

Alias for a type-erased error type.
