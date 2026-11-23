# Module axum::extract

## Overview

Types and traits for extracting data from requests.

**Intro**

A handler function is an async function that takes any number of
“extractors” as arguments. An extractor is a type that implements
`FromRequest` or `FromRequestParts`.

For example, `Json` is an extractor that consumes the request body and
deserializes it as JSON into some target type:

```rust
use axum::{
    extract::Json,
    routing::post,
    handler::Handler,
    Router,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateUser {
    email: String,
    password: String,
}

async fn create_user(Json(payload): Json<CreateUser>) {
    // ...
}

let app = Router::new().route("/users", post(create_user));
```

**Common extractors**

Some commonly used extractors are:

```rust
use axum::{
    extract::{Request, Json, Path, Extension, Query},
    routing::post,
    http::header::HeaderMap,
    body::{Bytes, Body},
    Router,
};
use serde_json::Value;
use std::collections::HashMap;

// `Path` gives you the path parameters and deserializes them. See its docs for
// more details
async fn path(Path(user_id): Path<u32>) {}

// `Query` gives you the query parameters and deserializes them.
async fn query(Query(params): Query<HashMap<String, String>>) {}

// `HeaderMap` gives you all the headers
async fn headers(headers: HeaderMap) {}

// `String` consumes the request body and ensures it is valid utf-8
async fn string(body: String) {}

// `Bytes` gives you the raw request body
async fn bytes(body: Bytes) {}

// We've already seen `Json` for parsing the request body as json
async fn json(Json(payload): Json<Value>) {}

// `Request` gives you the whole request for maximum control
async fn request(request: Request) {}

// `Extension` extracts data from "request extensions"
// This is commonly used to share state with handlers
async fn extension(Extension(state): Extension<State>) {}

#[derive(Clone)]
struct State { /* ... */ }

let app = Router::new()
    .route("/path/{user_id}", post(path))
    .route("/query", post(query))
    .route("/string", post(string))
    .route("/bytes", post(bytes))
    .route("/json", post(json))
    .route("/request", post(request))
    .route("/extension", post(extension));
```

**Applying multiple extractors**

You can also apply multiple extractors:

...

## Items

- `connect_info` — Extractor for getting connection information from a client.
- `path` — Extractor that will get captures from the URL and parse them using [serde].
- `rejection` — Rejection response types.
- `ConnectInfo` — Extractor for getting connection information produced by a Connected.
- `DefaultBodyLimit` — Layer for configuring the default request body limit.
- `MatchedPath` — Access the path in the router that matches the request.
- `NestedPath` — Access the path the matched the route is nested at.
- `OriginalUri` — Extractor that gets the original request URI regardless of nesting.
- `Path` — Extractor that will get captures from the URL and parse them using serde.
- `Query` — Extractor that deserializes query strings into some type.
- `RawForm` — Extractor that extracts raw form requests.
- `RawPathParams` — Extractor that will get captures from the URL without deserializing them.
- `RawQuery` — Extractor that extracts the raw query string, without parsing it.
- `State` — Extractor for state.
- `FromRef` — Used to do reference-to-value conversions thus not consuming the input value.
- `FromRequest` — Types that can be created from requests.
- `FromRequestParts` — Types that can be created from request parts.
- `OptionalFromRequest` — Customize the behavior of Option<Self> as a FromRequest extractor.
- `OptionalFromRequestParts` — Customize the behavior of Option<Self> as a FromRequestParts extractor.
- `Request` — Type alias for [http::Request] whose body type defaults to Body, the most common body type used with axum.

### struct axum::extract::ConnectInfo

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

### struct axum::extract::DefaultBodyLimit

```rust
pub struct DefaultBodyLimit { /* private fields */ }
```

Layer for configuring the default request body limit.

For security reasons, `Bytes` will, by default, not accept bodies larger than 2MB. This also
applies to extractors that uses `Bytes` internally such as `String`, `Json`, and `Form`.

This middleware provides ways to configure that.

Note that if an extractor consumes the body directly with `Body::poll_frame`, or similar, the
default limit is not applied.

**Difference between `DefaultBodyLimit` and `RequestBodyLimit`**

`DefaultBodyLimit` and `RequestBodyLimit` serve similar functions but in different ways.

`DefaultBodyLimit` is local in that it only applies to `FromRequest` implementations that
explicitly apply it (or call another extractor that does). You can apply the limit with
`RequestExt::with_limited_body` or `RequestExt::into_limited_body`

`RequestBodyLimit` is applied globally to all requests, regardless of which extractors are
used or how the body is consumed.

**Example**

```rust
use axum::{
    Router,
    routing::post,
    body::Body,
    extract::{Request, DefaultBodyLimit},
};

let app = Router::new()
    .route("/", post(|request: Request| async {}))
    // change the default limit
    .layer(DefaultBodyLimit::max(1024));
```

In general using `DefaultBodyLimit` is recommended but if you need to use third party
extractors and want to make sure a limit is also applied there then `RequestBodyLimit` should
be used.

**Different limits for different routes**

`DefaultBodyLimit` can also be selectively applied to have different limits for different
routes:

...

**Methods**

- `pub const fn disable() -> DefaultBodyLimit`
- `pub const fn max(limit: usize) -> DefaultBodyLimit`
- `pub fn apply<B>(self, req: &mut Request<B>)`
- `fn clone(&self) -> DefaultBodyLimit`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn layer(&self, inner: S) -> <DefaultBodyLimit as Layer<S>>::Service`

### struct axum::extract::MatchedPath

```rust
pub struct MatchedPath(/* private fields */);
```

Access the path in the router that matches the request.

```rust
use axum::{
    Router,
    extract::MatchedPath,
    routing::get,
};

let app = Router::new().route(
    "/users/{id}",
    get(|path: MatchedPath| async move {
        let path = path.as_str();
        // `path` will be "/users/{id}"
    })
);
```

**Accessing `MatchedPath` via extensions**

`MatchedPath` can also be accessed from middleware via request extensions.

This is useful for example with `Trace` to
create a span that contains the matched path:

```rust
use axum::{
    Router,
    extract::{Request, MatchedPath},
    routing::get,
};
use tower_http::trace::TraceLayer;

let app = Router::new()
    .route("/users/{id}", get(|| async { /* ... */ }))
    .layer(
        TraceLayer::new_for_http().make_span_with(|req: &Request<_>| {
            let path = if let Some(path) = req.extensions().get::<MatchedPath>() {
                path.as_str()
            } else {
                req.uri().path()
            };
            tracing::info_span!("http-request", %path)
        }),
    );
```

**Methods**

- `pub fn as_str(&self) -> &str`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `async fn from_request_parts( parts: &mut Parts, _state: &S, ) -> Result<Self, Self::Rejection>`
- `async fn from_request_parts( parts: &mut Parts, _state: &S, ) -> Result<Option<Self>, Self::Rejection>`

### struct axum::extract::NestedPath

```rust
pub struct NestedPath(/* private fields */);
```

Access the path the matched the route is nested at.

This can for example be used when doing redirects.

**Example**

```rust
use axum::{
    Router,
    extract::NestedPath,
    routing::get,
};

let api = Router::new().route(
    "/users",
    get(|path: NestedPath| async move {
        // `path` will be "/api" because that's what this
        // router is nested at when we build `app`
        let path = path.as_str();
    })
);

let app = Router::new().nest("/api", api);
```

**Methods**

- `pub fn as_str(&self) -> &str`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `async fn from_request_parts( parts: &mut Parts, _state: &S, ) -> Result<Self, Self::Rejection>`

### struct axum::extract::OriginalUri

```rust
pub struct OriginalUri(pub Uri);
```

Extractor that gets the original request URI regardless of nesting.

This is necessary since `Uri`, when used as an extractor, will
have the prefix stripped if used in a nested service.

**Example**

```rust
use axum::{
    routing::get,
    Router,
    extract::OriginalUri,
    http::Uri
};

let api_routes = Router::new()
    .route(
        "/users",
        get(|uri: Uri, OriginalUri(original_uri): OriginalUri| async {
            // `uri` is `/users`
            // `original_uri` is `/api/users`
        }),
    );

let app = Router::new().nest("/api", api_routes);
```

**Extracting via request extensions**

`OriginalUri` can also be accessed from middleware via request extensions.
This is useful for example with `Trace` to
create a span that contains the full path, if your service might be nested:

...

**Methods**

- `pub fn path_and_query(&self) -> Option<&PathAndQuery>`
- `pub fn path(&self) -> &str`
- `pub fn scheme(&self) -> Option<&Scheme>`
- `pub fn scheme_str(&self) -> Option<&str>`
- `pub fn authority(&self) -> Option<&Authority>`
- `pub fn host(&self) -> Option<&str>`
- `pub fn port(&self) -> Option<Port<&str>>`
- `pub fn port_u16(&self) -> Option<u16>`
- `pub fn query(&self) -> Option<&str>`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deref(&self) -> &Self::Target`
- `fn deref_mut(&mut self) -> &mut Self::Target`
- `async fn from_request_parts( parts: &mut Parts, state: &S, ) -> Result<Self, Self::Rejection>`

### struct axum::extract::Path

```rust
pub struct Path<T>(pub T);
```

Extractor that will get captures from the URL and parse them using
`serde`.

Any percent encoded parameters will be automatically decoded. The decoded
parameters must be valid UTF-8, otherwise `Path` will fail and return a `400 Bad Request` response.

**`Option<Path<T>>` behavior**

You can use `Option<Path<T>>` as an extractor to allow the same handler to
be used in a route with parameters that deserialize to `T`, and another
route with no parameters at all.

**Example**

These examples assume the `serde` feature of the `uuid` crate is enabled.

One `Path` can extract multiple captures. It is not necessary (and does
not work) to give a handler more than one `Path` argument.

```rust
use axum::{
    extract::Path,
    routing::get,
    Router,
};
use uuid::Uuid;

async fn users_teams_show(
    Path((user_id, team_id)): Path<(Uuid, Uuid)>,
) {
    // ...
}

let app = Router::new().route("/users/{user_id}/team/{team_id}", get(users_teams_show));
```

If the path contains only one parameter, then you can omit the tuple.

```rust
use axum::{
    extract::Path,
    routing::get,
    Router,
};
use uuid::Uuid;

async fn user_info(Path(user_id): Path<Uuid>) {
    // ...
}

let app = Router::new().route("/users/{user_id}", get(user_info));
```

Path segments also can be deserialized into any type that implements
`serde::Deserialize`. This includes tuples and structs:

...

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deref(&self) -> &Self::Target`
- `fn deref_mut(&mut self) -> &mut Self::Target`
- `async fn from_request_parts( parts: &mut Parts, _state: &S, ) -> Result<Self, Self::Rejection>`
- `async fn from_request_parts( parts: &mut Parts, _state: &S, ) -> Result<Option<Self>, Self::Rejection>`

### struct axum::extract::Query

```rust
pub struct Query<T>(pub T);
```

Extractor that deserializes query strings into some type.

`T` is expected to implement [`serde::Deserialize`].

**Examples**

```rust
use axum::{
    extract::Query,
    routing::get,
    Router,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Pagination {
    page: usize,
    per_page: usize,
}

// This will parse query strings like `?page=2&per_page=30` into `Pagination`
// structs.
async fn list_things(pagination: Query<Pagination>) {
    let pagination: Pagination = pagination.0;

    // ...
}

let app = Router::new().route("/list_things", get(list_things));
```

If the query string cannot be parsed it will reject the request with a `400 Bad Request` response.

For handling values being empty vs missing see the query-params-with-empty-strings
example.

For handling multiple values for the same query parameter, in a `?foo=1&foo=2&foo=3`
fashion, use `axum_extra::extract::Query` instead.

**Methods**

- `pub fn try_from_uri(value: &Uri) -> Result<Self, QueryRejection>`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn deref(&self) -> &Self::Target`
- `fn deref_mut(&mut self) -> &mut Self::Target`
- `async fn from_request_parts( parts: &mut Parts, _state: &S, ) -> Result<Self, Self::Rejection>`

### struct axum::extract::RawForm

```rust
pub struct RawForm(pub Bytes);
```

Extractor that extracts raw form requests.

For `GET` requests it will extract the raw query. For other methods it extracts the raw
`application/x-www-form-urlencoded` encoded request body.

**Example**

```rust
use axum::{
    extract::RawForm,
    routing::get,
    Router
};

async fn handler(RawForm(form): RawForm) {}

let app = Router::new().route("/", get(handler));
```

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection>`

### struct axum::extract::RawPathParams

```rust
pub struct RawPathParams(/* private fields */);
```

Extractor that will get captures from the URL without deserializing them.

In general you should prefer to use `Path` as it is higher level, however `RawPathParams` is
suitable if just want the raw params without deserializing them and thus saving some
allocations.

Any percent encoded parameters will be automatically decoded. The decoded parameters must be
valid UTF-8, otherwise `RawPathParams` will fail and return a `400 Bad Request` response.

**Example**

```rust
use axum::{
    extract::RawPathParams,
    routing::get,
    Router,
};

async fn users_teams_show(params: RawPathParams) {
    for (key, value) in ¶ms {
        println!("{key:?} = {value:?}");
    }
}

let app = Router::new().route("/users/{user_id}/team/{team_id}", get(users_teams_show));
```

**Methods**

- `pub fn iter(&self) -> RawPathParamsIter<'_>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `async fn from_request_parts( parts: &mut Parts, _state: &S, ) -> Result<Self, Self::Rejection>`
- `fn into_iter(self) -> Self::IntoIter`

### struct axum::extract::RawQuery

```rust
pub struct RawQuery(pub Option<String>);
```

Extractor that extracts the raw query string, without parsing it.

**Example**

```rust
use axum::{
    extract::RawQuery,
    routing::get,
    Router,
};
use futures_util::StreamExt;

async fn handler(RawQuery(query): RawQuery) {
    // ...
}

let app = Router::new().route("/users", get(handler));
```

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `async fn from_request_parts( parts: &mut Parts, _state: &S, ) -> Result<Self, Self::Rejection>`

### struct axum::extract::State

```rust
pub struct State<S>(pub S);
```

Extractor for state.

See “Sharing state with handlers” for an overview of all approaches to
sharing state, including when to use `Arc`, how `FromRef` substates work, and what the
`Router<S>` type parameter means.

See “Accessing state in middleware” for how to
access state in middleware.

State is global and used in every request a router with state receives.
For accessing data derived from requests, such as authorization data, see `Extension`.

**With `Router`**

```rust
use axum::{Router, routing::get, extract::State};

// the application state
//
// here you can put configuration, database connection pools, or whatever
// state you need
#[derive(Clone)]
struct AppState {}

let state = AppState {};

// create a `Router` that holds our state
let app = Router::new()
    .route("/", get(handler))
    // provide the state so the router can access it
    .with_state(state);

async fn handler(
    // access the state via the `State` extractor
    // extracting a state of the wrong type results in a compile error
    State(state): State<AppState>,
) {
    // use `state`...
}
```

Note that `State` is an extractor, so be sure to put it before any body
extractors, see “the order of extractors”.

**Combining stateful routers**

Multiple `Router`s can be combined with `Router::nest` or `Router::merge`
When combining `Router`s with one of these methods, the `Router`s must have
the same state type. Generally, this can be inferred automatically:

...

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn deref(&self) -> &Self::Target`
- `fn deref_mut(&mut self) -> &mut Self::Target`
- `async fn from_request_parts( _parts: &mut Parts, state: &OuterState, ) -> Result<Self, Self::Rejection>`

### trait axum::extract::FromRef

```rust
pub trait FromRef<T> {
    // Required method
    fn from_ref(input: &T) -> Self;
}
```

Used to do reference-to-value conversions thus not consuming the input value.

This is mainly used with `State` to extract “substates” from a reference to main application
state.

See `State` for more details on how library authors should use this trait.

This trait can be derived using `#[derive(FromRef)]`.

**Methods**

- `fn from_ref(input: &T) -> Self`

### trait axum::extract::FromRequest

```rust
pub trait FromRequest<S, M = ViaRequest>: Sized {
    type Rejection: IntoResponse;

    // Required method
    fn from_request(
        req: Request<Body>,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send;
}
```

Types that can be created from requests.

Extractors that implement `FromRequest` can consume the request body and can thus only be run
once for handlers.

If your extractor doesn’t need to consume the request body then you should implement
`FromRequestParts` and not `FromRequest`.

See `axum::extract` for more general docs about extractors.

**Methods**

- `type Rejection: IntoResponse`
- `fn from_request( req: Request<Body>, state: &S, ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send`
- `fn from_request( req: Request<Body>, state: &S, ) -> impl Future<Output = Result<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), <(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) as FromRequest<S>>::Rejection>>`
- `fn from_request( req: Request<Body>, state: &S, ) -> impl Future<Output = Result<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), <(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) as FromRequest<S>>::Rejection>>`
- `fn from_request( req: Request<Body>, state: &S, ) -> impl Future<Output = Result<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), <(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) as FromRequest<S>>::Rejection>>`
- `fn from_request( req: Request<Body>, state: &S, ) -> impl Future<Output = Result<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), <(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) as FromRequest<S>>::Rejection>>`
- `fn from_request( req: Request<Body>, state: &S, ) -> impl Future<Output = Result<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), <(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) as FromRequest<S>>::Rejection>>`
- `fn from_request( req: Request<Body>, state: &S, ) -> impl Future<Output = Result<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), <(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) as FromRequest<S>>::Rejection>>`
- `fn from_request( req: Request<Body>, state: &S, ) -> impl Future<Output = Result<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), <(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) as FromRequest<S>>::Rejection>>`
- `fn from_request( req: Request<Body>, state: &S, ) -> impl Future<Output = Result<(T1, T2, T3, T4, T5, T6, T7, T8, T9), <(T1, T2, T3, T4, T5, T6, T7, T8, T9) as FromRequest<S>>::Rejection>>`
- `fn from_request( req: Request<Body>, state: &S, ) -> impl Future<Output = Result<(T1, T2, T3, T4, T5, T6, T7, T8), <(T1, T2, T3, T4, T5, T6, T7, T8) as FromRequest<S>>::Rejection>>`
- `fn from_request( req: Request<Body>, state: &S, ) -> impl Future<Output = Result<(T1, T2, T3, T4, T5, T6, T7), <(T1, T2, T3, T4, T5, T6, T7) as FromRequest<S>>::Rejection>>`
- `fn from_request( req: Request<Body>, state: &S, ) -> impl Future<Output = Result<(T1, T2, T3, T4, T5, T6), <(T1, T2, T3, T4, T5, T6) as FromRequest<S>>::Rejection>>`
- `fn from_request( req: Request<Body>, state: &S, ) -> impl Future<Output = Result<(T1, T2, T3, T4, T5), <(T1, T2, T3, T4, T5) as FromRequest<S>>::Rejection>>`
- `fn from_request( req: Request<Body>, state: &S, ) -> impl Future<Output = Result<(T1, T2, T3, T4), <(T1, T2, T3, T4) as FromRequest<S>>::Rejection>>`
- `fn from_request( req: Request<Body>, state: &S, ) -> impl Future<Output = Result<(T1, T2, T3), <(T1, T2, T3) as FromRequest<S>>::Rejection>>`
- `fn from_request( req: Request<Body>, state: &S, ) -> impl Future<Output = Result<(T1, T2), <(T1, T2) as FromRequest<S>>::Rejection>>`
- `fn from_request( req: Request<Body>, state: &S, ) -> impl Future<Output = Result<(T1,), <(T1,) as FromRequest<S>>::Rejection>>`
- `async fn from_request( req: Request<Body>, state: &S, ) -> Result<Option<T>, <Option<T> as FromRequest<S>>::Rejection>`
- `async fn from_request( req: Request<Body>, state: &S, ) -> Result<Result<T, <T as FromRequest<S>>::Rejection>, <Result<T, <T as FromRequest<S>>::Rejection> as FromRequest<S>>::Rejection>`
- `async fn from_request( req: Request<Body>, _: &S, ) -> Result<Bytes, <Bytes as FromRequest<S>>::Rejection>`
- `async fn from_request( req: Request<Body>, _: &S, ) -> Result<BytesMut, <BytesMut as FromRequest<S>>::Rejection>`
- `async fn from_request( req: Request<Body>, _: &S, ) -> Result<Request<Body>, <Request<Body> as FromRequest<S>>::Rejection>`
- `async fn from_request( req: Request<Body>, state: &S, ) -> Result<String, <String as FromRequest<S>>::Rejection>`

### trait axum::extract::FromRequestParts

```rust
pub trait FromRequestParts<S>: Sized {
    type Rejection: IntoResponse;

    // Required method
    fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send;
}
```

Types that can be created from request parts.

Extractors that implement `FromRequestParts` cannot consume the request body and can thus be
run in any order for handlers.

If your extractor needs to consume the request body then you should implement `FromRequest`
and not `FromRequestParts`.

See `axum::extract` for more general docs about extractors.

**Methods**

- `type Rejection: IntoResponse`
- `fn from_request_parts( parts: &mut Parts, state: &S, ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send`
- `async fn from_request_parts( parts: &mut Parts, state: &S, ) -> Result<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), <(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) as FromRequestParts<S>>::Rejection>`
- `async fn from_request_parts( parts: &mut Parts, state: &S, ) -> Result<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), <(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) as FromRequestParts<S>>::Rejection>`
- `async fn from_request_parts( parts: &mut Parts, state: &S, ) -> Result<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), <(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) as FromRequestParts<S>>::Rejection>`
- `async fn from_request_parts( parts: &mut Parts, state: &S, ) -> Result<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), <(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) as FromRequestParts<S>>::Rejection>`
- `async fn from_request_parts( parts: &mut Parts, state: &S, ) -> Result<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), <(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) as FromRequestParts<S>>::Rejection>`
- `async fn from_request_parts( parts: &mut Parts, state: &S, ) -> Result<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), <(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) as FromRequestParts<S>>::Rejection>`
- `async fn from_request_parts( parts: &mut Parts, state: &S, ) -> Result<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), <(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) as FromRequestParts<S>>::Rejection>`
- `async fn from_request_parts( parts: &mut Parts, state: &S, ) -> Result<(T1, T2, T3, T4, T5, T6, T7, T8, T9), <(T1, T2, T3, T4, T5, T6, T7, T8, T9) as FromRequestParts<S>>::Rejection>`
- `async fn from_request_parts( parts: &mut Parts, state: &S, ) -> Result<(T1, T2, T3, T4, T5, T6, T7, T8), <(T1, T2, T3, T4, T5, T6, T7, T8) as FromRequestParts<S>>::Rejection>`
- `async fn from_request_parts( parts: &mut Parts, state: &S, ) -> Result<(T1, T2, T3, T4, T5, T6, T7), <(T1, T2, T3, T4, T5, T6, T7) as FromRequestParts<S>>::Rejection>`
- `async fn from_request_parts( parts: &mut Parts, state: &S, ) -> Result<(T1, T2, T3, T4, T5, T6), <(T1, T2, T3, T4, T5, T6) as FromRequestParts<S>>::Rejection>`
- `async fn from_request_parts( parts: &mut Parts, state: &S, ) -> Result<(T1, T2, T3, T4, T5), <(T1, T2, T3, T4, T5) as FromRequestParts<S>>::Rejection>`
- `async fn from_request_parts( parts: &mut Parts, state: &S, ) -> Result<(T1, T2, T3, T4), <(T1, T2, T3, T4) as FromRequestParts<S>>::Rejection>`
- `async fn from_request_parts( parts: &mut Parts, state: &S, ) -> Result<(T1, T2, T3), <(T1, T2, T3) as FromRequestParts<S>>::Rejection>`
- `async fn from_request_parts( parts: &mut Parts, state: &S, ) -> Result<(T1, T2), <(T1, T2) as FromRequestParts<S>>::Rejection>`
- `async fn from_request_parts( parts: &mut Parts, state: &S, ) -> Result<(T1,), <(T1,) as FromRequestParts<S>>::Rejection>`
- `fn from_request_parts( parts: &mut Parts, state: &S, ) -> impl Future<Output = Result<Option<T>, <Option<T> as FromRequestParts<S>>::Rejection>>`
- `async fn from_request_parts( parts: &mut Parts, state: &S, ) -> Result<Result<T, <T as FromRequestParts<S>>::Rejection>, <Result<T, <T as FromRequestParts<S>>::Rejection> as FromRequestParts<S>>::Rejection>`
- `async fn from_request_parts( _: &mut Parts, _: &S, ) -> Result<(), <() as FromRequestParts<S>>::Rejection>`
- `async fn from_request_parts( parts: &mut Parts, _state: &S, ) -> Result<Extensions, <Extensions as FromRequestParts<S>>::Rejection>`
- `async fn from_request_parts( parts: &mut Parts, _: &S, ) -> Result<HeaderMap, <HeaderMap as FromRequestParts<S>>::Rejection>`
- `async fn from_request_parts( parts: &mut Parts, _: &S, ) -> Result<Method, <Method as FromRequestParts<S>>::Rejection>`
- `async fn from_request_parts( parts: &mut Parts, _state: &S, ) -> Result<Parts, <Parts as FromRequestParts<S>>::Rejection>`
- `async fn from_request_parts( parts: &mut Parts, _: &S, ) -> Result<Uri, <Uri as FromRequestParts<S>>::Rejection>`
- `async fn from_request_parts( parts: &mut Parts, _: &S, ) -> Result<Version, <Version as FromRequestParts<S>>::Rejection>`

### trait axum::extract::OptionalFromRequest

```rust
pub trait OptionalFromRequest<S, M = ViaRequest>: Sized {
    type Rejection: IntoResponse;

    // Required method
    fn from_request(
        req: Request<Body>,
        state: &S,
    ) -> impl Future<Output = Result<Option<Self>, Self::Rejection>> + Send;
}
```

Customize the behavior of `Option<Self>` as a `FromRequest` extractor.

**Methods**

- `type Rejection: IntoResponse`
- `fn from_request( req: Request<Body>, state: &S, ) -> impl Future<Output = Result<Option<Self>, Self::Rejection>> + Send`

### trait axum::extract::OptionalFromRequestParts

```rust
pub trait OptionalFromRequestParts<S>: Sized {
    type Rejection: IntoResponse;

    // Required method
    fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> impl Future<Output = Result<Option<Self>, Self::Rejection>> + Send;
}
```

Customize the behavior of `Option<Self>` as a `FromRequestParts`
extractor.

**Methods**

- `type Rejection: IntoResponse`
- `fn from_request_parts( parts: &mut Parts, state: &S, ) -> impl Future<Output = Result<Option<Self>, Self::Rejection>> + Send`

### type axum::extract::Request

```rust
pub type Request<T = Body> = Request<T>;
```

Type alias for [`http::Request`] whose body type defaults to `Body`, the most common body
type used with axum.
