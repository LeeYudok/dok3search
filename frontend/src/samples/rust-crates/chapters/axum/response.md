# Module axum::response

## Overview

Types and traits for generating responses.

**Building responses**

Anything that implements `IntoResponse` can be returned from a handler. axum
provides implementations for common types:

```rust
use axum::{
    Json,
    response::{Html, IntoResponse},
    http::{StatusCode, Uri, header::{self, HeaderMap, HeaderName}},
};

// `()` gives an empty response
async fn empty() {}

// String will get a `text/plain; charset=utf-8` content-type
async fn plain_text(uri: Uri) -> String {
    format!("Hi from {}", uri.path())
}

// Bytes will get a `application/octet-stream` content-type
async fn bytes() -> Vec<u8> {
    vec![1, 2, 3, 4]
}

// `Json` will get a `application/json` content-type and work with anything that
// implements `serde::Serialize`
async fn json() -> Json<Vec<String>> {
    Json(vec!["foo".to_owned(), "bar".to_owned()])
}

// `Html` will get a `text/html` content-type
async fn html() -> Html<&'static str> {
    Html("<p>Hello, World!</p>")
}

// `StatusCode` gives an empty response with that status code
async fn status() -> StatusCode {
    StatusCode::NOT_FOUND
}

// `HeaderMap` gives an empty response with some headers
async fn headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(header::SERVER, "axum".parse().unwrap());
    headers
}

// An array of tuples also gives headers
async fn array_headers() -> [(HeaderName, &'static str); 2] {
    [
        (header::SERVER, "axum"),
        (header::CONTENT_TYPE, "text/plain")
    ]
}

// Use `impl IntoResponse` to avoid writing the whole type
async fn impl_trait() -> impl IntoResponse {
    [
        (header::SERVER, "axum"),
        (header::CONTENT_TYPE, "text/plain")
    ]
}
```

Additionally you can return tuples to build more complex responses from
individual parts.

...

## Items

- `sse` — Server-Sent Events (SSE) responses.
- `AppendHeaders` — Append headers to a response.
- `ErrorResponse` — An IntoResponse-based error type
- `Html` — An HTML response.
- `NoContent` — An empty response with 204 No Content status.
- `Redirect` — Response that redirects the request to another location.
- `ResponseParts` — Parts of a response.
- `Sse` — An SSE response
- `IntoResponse` — Trait for generating responses.
- `IntoResponseParts` — Trait for adding headers and extensions to a response.
- `Response` — Type alias for [http::Response] whose body type defaults to Body, the most common body type used with axum.
- `Result` — An IntoResponse-based result type that uses ErrorResponse as the error type.

### struct axum::response::AppendHeaders

```rust
pub struct AppendHeaders<I>(pub I);
```

Append headers to a response.

Returning something like `[("content-type", "foo=bar")]` from a handler will override any
existing `content-type` headers. If instead you want to append headers, use `AppendHeaders`:

```rust
use axum::{
    response::{AppendHeaders, IntoResponse},
    http::header::SET_COOKIE,
};

async fn handler() -> impl IntoResponse {
    // something that sets the `set-cookie` header
    let set_some_cookies = /* ... */

    (
        set_some_cookies,
        // append two `set-cookie` headers to the response
        // without overriding the ones added by `set_some_cookies`
        AppendHeaders([
            (SET_COOKIE, "foo=bar"),
            (SET_COOKIE, "baz=qux"),
        ])
    )
}
```

**Methods**

- `fn clone(&self) -> AppendHeaders<I>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response_parts( self, res: ResponseParts, ) -> Result<ResponseParts, <AppendHeaders<I> as IntoResponseParts>::Error>`

### struct axum::response::ErrorResponse

```rust
pub struct ErrorResponse(/* private fields */);
```

An `IntoResponse`-based error type

See `Result` for more details.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn from(value: T) -> ErrorResponse`

### struct axum::response::Html

```rust
pub struct Html<T>(pub T);
```

An HTML response.

Will automatically get `Content-Type: text/html`.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn from(inner: T) -> Self`
- `fn into_response(self) -> Response`

### struct axum::response::NoContent

```rust
pub struct NoContent;
```

An empty response with 204 No Content status.

Due to historical and implementation reasons, the `IntoResponse` implementation of `()`
(unit type) returns an empty response with 200 [`StatusCode::OK`] status.
If you specifically want a 204 [`StatusCode::NO_CONTENT`] status, you can use either `StatusCode` type
directly, or this shortcut struct for self-documentation.

```rust
use axum::{extract::Path, response::NoContent};

async fn delete_user(Path(user): Path<String>) -> Result<NoContent, String> {
    // ...access database...
    Ok(NoContent)
}
```

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn into_response(self) -> Response`

### struct axum::response::Redirect

```rust
pub struct Redirect { /* private fields */ }
```

Response that redirects the request to another location.

**Example**

```rust
use axum::{
    routing::get,
    response::Redirect,
    Router,
};

let app = Router::new()
    .route("/old", get(|| async { Redirect::permanent("/new") }))
    .route("/new", get(|| async { "Hello!" }));
```

**Methods**

- `pub fn to(uri: &str) -> Self`
- `pub fn temporary(uri: &str) -> Self`
- `pub fn permanent(uri: &str) -> Self`
- `pub fn status_code(&self) -> StatusCode`
- `pub fn location(&self) -> &str`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn into_response(self) -> Response`

### struct axum::response::ResponseParts

```rust
pub struct ResponseParts { /* private fields */ }
```

Parts of a response.

Used with `IntoResponseParts`.

**Methods**

- `pub fn headers(&self) -> &HeaderMap`
- `pub fn headers_mut(&mut self) -> &mut HeaderMap`
- `pub fn extensions(&self) -> &Extensions`
- `pub fn extensions_mut(&mut self) -> &mut Extensions`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`

### struct axum::response::Sse

```rust
pub struct Sse<S> { /* private fields */ }
```

An SSE response

**Methods**

- `pub fn new(stream: S) -> Selfwhere S: TryStream<Ok = Event> + Send + 'static, S::Error: Into<BoxError>,`
- `pub fn keep_alive(self, keep_alive: KeepAlive) -> Sse<KeepAliveStream<S>>`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn into_response(self) -> Response`

### trait axum::response::IntoResponse

```rust
pub trait IntoResponse {
    // Required method
    fn into_response(self) -> Response<Body>;
}
```

Trait for generating responses.

Types that implement `IntoResponse` can be returned from handlers.

**Implementing `IntoResponse`**

You generally shouldn’t have to implement `IntoResponse` manually, as axum
provides implementations for many common types.

However it might be necessary if you have a custom error type that you want
to return from handlers:

```rust
use axum::{
    Router,
    body::{self, Bytes},
    routing::get,
    http::StatusCode,
    response::{IntoResponse, Response},
};

enum MyError {
    SomethingWentWrong,
    SomethingElseWentWrong,
}

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        let body = match self {
            MyError::SomethingWentWrong => "something went wrong",
            MyError::SomethingElseWentWrong => "something else went wrong",
        };

        // it's often easiest to implement `IntoResponse` by calling other implementations
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

// `Result<impl IntoResponse, MyError>` can now be returned from handlers
let app = Router::new().route("/", get(handler));

async fn handler() -> Result<(), MyError> {
    Err(MyError::SomethingWentWrong)
}
```

Or if you have a custom body type you’ll also need to implement
`IntoResponse` for it:

...

**Methods**

- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`
- `fn into_response(self) -> Response<Body>`

### trait axum::response::IntoResponseParts

```rust
pub trait IntoResponseParts {
    type Error: IntoResponse;

    // Required method
    fn into_response_parts(
        self,
        res: ResponseParts,
    ) -> Result<ResponseParts, Self::Error>;
}
```

Trait for adding headers and extensions to a response.

**Example**

...

**Methods**

- `type Error: IntoResponse`
- `fn into_response_parts( self, res: ResponseParts, ) -> Result<ResponseParts, Self::Error>`
- `fn into_response_parts( self, res: ResponseParts, ) -> Result<ResponseParts, <() as IntoResponseParts>::Error>`
- `fn into_response_parts( self, res: ResponseParts, ) -> Result<ResponseParts, <Extensions as IntoResponseParts>::Error>`
- `fn into_response_parts( self, res: ResponseParts, ) -> Result<ResponseParts, <HeaderMap as IntoResponseParts>::Error>`
- `fn into_response_parts( self, res: ResponseParts, ) -> Result<ResponseParts, <[(K, V); N] as IntoResponseParts>::Error>`
- `fn into_response_parts( self, res: ResponseParts, ) -> Result<ResponseParts, <(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) as IntoResponseParts>::Error>`
- `fn into_response_parts( self, res: ResponseParts, ) -> Result<ResponseParts, <(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) as IntoResponseParts>::Error>`
- `fn into_response_parts( self, res: ResponseParts, ) -> Result<ResponseParts, <(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) as IntoResponseParts>::Error>`
- `fn into_response_parts( self, res: ResponseParts, ) -> Result<ResponseParts, <(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) as IntoResponseParts>::Error>`
- `fn into_response_parts( self, res: ResponseParts, ) -> Result<ResponseParts, <(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) as IntoResponseParts>::Error>`
- `fn into_response_parts( self, res: ResponseParts, ) -> Result<ResponseParts, <(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) as IntoResponseParts>::Error>`
- `fn into_response_parts( self, res: ResponseParts, ) -> Result<ResponseParts, <(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) as IntoResponseParts>::Error>`
- `fn into_response_parts( self, res: ResponseParts, ) -> Result<ResponseParts, <(T1, T2, T3, T4, T5, T6, T7, T8, T9) as IntoResponseParts>::Error>`
- `fn into_response_parts( self, res: ResponseParts, ) -> Result<ResponseParts, <(T1, T2, T3, T4, T5, T6, T7, T8) as IntoResponseParts>::Error>`
- `fn into_response_parts( self, res: ResponseParts, ) -> Result<ResponseParts, <(T1, T2, T3, T4, T5, T6, T7) as IntoResponseParts>::Error>`
- `fn into_response_parts( self, res: ResponseParts, ) -> Result<ResponseParts, <(T1, T2, T3, T4, T5, T6) as IntoResponseParts>::Error>`
- `fn into_response_parts( self, res: ResponseParts, ) -> Result<ResponseParts, <(T1, T2, T3, T4, T5) as IntoResponseParts>::Error>`
- `fn into_response_parts( self, res: ResponseParts, ) -> Result<ResponseParts, <(T1, T2, T3, T4) as IntoResponseParts>::Error>`
- `fn into_response_parts( self, res: ResponseParts, ) -> Result<ResponseParts, <(T1, T2, T3) as IntoResponseParts>::Error>`
- `fn into_response_parts( self, res: ResponseParts, ) -> Result<ResponseParts, <(T1, T2) as IntoResponseParts>::Error>`
- `fn into_response_parts( self, res: ResponseParts, ) -> Result<ResponseParts, <(T1,) as IntoResponseParts>::Error>`
- `fn into_response_parts( self, res: ResponseParts, ) -> Result<ResponseParts, <Option<T> as IntoResponseParts>::Error>`

### type axum::response::Response

```rust
pub type Response<T = Body> = Response<T>;
```

Type alias for [`http::Response`] whose body type defaults to `Body`, the most common body
type used with axum.

### type axum::response::Result

```rust
pub type Result<T, E = ErrorResponse> = Result<T, E>;
```

An `IntoResponse`-based result type that uses `ErrorResponse` as the error type.

All types which implement `IntoResponse` can be converted to an `ErrorResponse`. This makes
it useful as a general purpose error type for functions which combine multiple distinct error
types that all implement `IntoResponse`.

**Example**

```rust
use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
};

// two fallible functions with different error types
fn try_something() -> Result<(), ErrorA> {
    // ...
}

fn try_something_else() -> Result<(), ErrorB> {
    // ...
}

// each error type implements `IntoResponse`
struct ErrorA;

impl IntoResponse for ErrorA {
    fn into_response(self) -> Response {
        // ...
    }
}

enum ErrorB {
    SomethingWentWrong,
}

impl IntoResponse for ErrorB {
    fn into_response(self) -> Response {
        // ...
    }
}

// we can combine them using `axum::response::Result` and still use `?`
async fn handler() -> axum::response::Result<&'static str> {
    // the errors are automatically converted to `ErrorResponse`
    try_something()?;
    try_something_else()?;

    Ok("it worked!")
}
```

**As a replacement for `std::result::Result`**

Since `axum::response::Result` has a default error type you only have to specify the `Ok` type:

...
