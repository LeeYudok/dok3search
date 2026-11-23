# Module axum::error_handling

## Overview

Error handling model and utilities

**axum’s error handling model**

axum is based on `tower::Service` which bundles errors through its associated
`Error` type. If you have a [`Service`] that produces an error and that error
makes it all the way up to hyper, the connection will be terminated without
sending a response. This is generally not desirable so axum makes sure you
always produce a response by relying on the type system.

axum does this by requiring all services have `Infallible` as their error
type. `Infallible` is the error type for errors that can never happen.

This means if you define a handler like:

```rust
use axum::http::StatusCode;

async fn handler() -> Result<String, StatusCode> {
    // ...
}
```

While it looks like it might fail with a `StatusCode` this actually isn’t an
“error”. If this handler returns `Err(some_status_code)` that will still be
converted into a `Response` and sent back to the client. This is done
through `StatusCode`’s `IntoResponse` implementation.

It doesn’t matter whether you return `Err(StatusCode::NOT_FOUND)` or
`Err(StatusCode::INTERNAL_SERVER_ERROR)`. These are not considered errors in
axum.

Instead of a direct `StatusCode`, it makes sense to use intermediate error type
that can ultimately be converted to `Response`. This allows using `?` operator
in handlers. See those examples:

- `anyhow-error-response` for generic boxed errors

- `error-handling` for application-specific detailed errors

This also applies to extractors. If an extractor doesn’t match the request the
request will be rejected and a response will be returned without calling your
handler. See `extract` to learn more about handling extractor
failures.

**Routing to fallible services**

You generally don’t have to think about errors if you’re only using async
functions as handlers. However if you’re embedding general `Service`s or
applying middleware, which might produce errors you have to tell axum how to
convert those errors into responses.

...

## Items

- `future` — Future types.
- `HandleError` — A [Service] adapter that handles errors by converting them into responses.
- `HandleErrorLayer` — [Layer] that applies HandleError which is a [Service] adapter that handles errors by converting them into responses.

### struct axum::error_handling::HandleError

```rust
pub struct HandleError<S, F, T> { /* private fields */ }
```

A [`Service`] adapter that handles errors by converting them into responses.

See module docs for more details on axum’s error handling model.

**Methods**

- `pub fn new(inner: S, f: F) -> Self`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, req: Request<B>) -> Self::Future`
- `fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, req: Request<B>) -> Self::Future`
- `fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, req: Request<B>) -> Self::Future`
- `fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, req: Request<B>) -> Self::Future`
- `fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, req: Request<B>) -> Self::Future`
- `fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, req: Request<B>) -> Self::Future`
- `fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, req: Request<B>) -> Self::Future`
- `fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, req: Request<B>) -> Self::Future`
- `fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, req: Request<B>) -> Self::Future`
- `fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, req: Request<B>) -> Self::Future`
- `fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, req: Request<B>) -> Self::Future`
- `fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, req: Request<B>) -> Self::Future`
- `fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, req: Request<B>) -> Self::Future`
- `fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, req: Request<B>) -> Self::Future`
- `fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, req: Request<B>) -> Self::Future`
- `fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, req: Request<B>) -> Self::Future`
- `fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, req: Request<B>) -> Self::Future`

### struct axum::error_handling::HandleErrorLayer

```rust
pub struct HandleErrorLayer<F, T> { /* private fields */ }
```

[`Layer`] that applies `HandleError` which is a [`Service`] adapter
that handles errors by converting them into responses.

See module docs for more details on axum’s error handling model.

**Methods**

- `pub fn new(f: F) -> Self`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn layer(&self, inner: S) -> Self::Service`
