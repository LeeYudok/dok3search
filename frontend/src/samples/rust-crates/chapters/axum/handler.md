# Module axum::handler

## Overview

Async functions that can be used to handle requests.

In axum a “handler” is an async function that accepts zero or more
“extractors” as arguments and returns something that
can be converted into a response.

Handlers are where your application logic lives and axum applications are built
by routing between handlers.

Some examples of handlers:

```rust
use axum::{body::Bytes, http::StatusCode};

// Handler that immediately returns an empty `200 OK` response.
async fn unit_handler() {}

// Handler that immediately returns a `200 OK` response with a plain text
// body.
async fn string_handler() -> String {
    "Hello, World!".to_string()
}

// Handler that buffers the request body and returns it.
//
// This works because `Bytes` implements `FromRequest`
// and therefore can be used as an extractor.
//
// `String` and `StatusCode` both implement `IntoResponse` and
// therefore `Result<String, StatusCode>` also implements `IntoResponse`
async fn echo(body: Bytes) -> Result<String, StatusCode> {
    if let Ok(string) = String::from_utf8(body.to_vec()) {
        Ok(string)
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
```

Instead of a direct `StatusCode`, it makes sense to use intermediate error type
that can ultimately be converted to `Response`. This allows using `?` operator
in handlers. See those examples:

- `anyhow-error-response` for generic boxed errors

- `error-handling` for application-specific detailed errors

**Debugging handler type errors**

For a function to be used as a handler it must implement the `Handler` trait.
axum provides blanket implementations for functions that:

- Are `async fn`s.

- Take no more than 16 arguments that all implement `Send`.

- All except the last argument implement `FromRequestParts`.

- The last argument implements `FromRequest`.

- Returns something that implements `IntoResponse`.

- If a closure is used it must implement `Clone + Send` and be
`'static`.

- Returns a future that is `Send`. The most common way to accidentally make a
future `!Send` is to hold a `!Send` type across an await.

Unfortunately Rust gives poor error messages if you try to use a function
that doesn’t quite match what’s required by `Handler`.

You might get an error like this:

...

## Items

- `future` — Handler future types.
- `HandlerService` — An adapter that makes a Handler into a [Service].
- `Layered` — A [Service] created from a Handler by applying a Tower middleware.
- `Handler` — Trait for async functions that can be used to handle requests.
- `HandlerWithoutStateExt` — Extension trait for Handlers that don’t have state.

### struct axum::handler::HandlerService

```rust
pub struct HandlerService<H, T, S> { /* private fields */ }
```

An adapter that makes a `Handler` into a [`Service`].

Created with `Handler::with_state` or `HandlerWithoutStateExt::into_service`.

**Methods**

- `pub fn state(&self) -> &S`
- `pub fn into_make_service(self) -> IntoMakeService<HandlerService<H, T, S>>`
- `pub fn into_make_service_with_connect_info<C>( self, ) -> IntoMakeServiceWithConnectInfo<HandlerService<H, T, S>, C>`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, _req: IncomingStream<'_, L>) -> Self::Future`
- `fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, req: Request<B>) -> Self::Future`

### struct axum::handler::Layered

```rust
pub struct Layered<L, H, T, S> { /* private fields */ }
```

A [`Service`] created from a `Handler` by applying a Tower middleware.

Created with `Handler::layer`. See that method for more details.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn call(self, req: Request, state: S) -> Self::Future`
- `fn layer<L>(self, layer: L) -> Layered<L, Self, T, S>where L: Layer<HandlerService<Self, T, S>> + Clone, L::Service: Service<Request>,`
- `fn with_state(self, state: S) -> HandlerService<Self, T, S>`

### trait axum::handler::Handler

```rust
pub trait Handler<T, S>:
    Clone
    + Send
    + Sync
    + Sized
    + 'static {
    type Future: Future<Output = Response> + Send + 'static;

    // Required method
    fn call(self, req: Request, state: S) -> Self::Future;

    // Provided methods
    fn layer<L>(self, layer: L) -> Layered<L, Self, T, S>
       where L: Layer<HandlerService<Self, T, S>> + Clone,
             L::Service: Service<Request> { ... }
    fn with_state(self, state: S) -> HandlerService<Self, T, S> { ... }
}
```

Trait for async functions that can be used to handle requests.

You shouldn’t need to depend on this trait directly. It is automatically
implemented to closures of the right types.

See the module docs for more details.

**Converting `Handler`s into [`Service`]s**

To convert `Handler`s into [`Service`]s you have to call either
`HandlerWithoutStateExt::into_service` or `Handler::with_state`:

```rust
use tower::Service;
use axum::{
    extract::{State, Request},
    body::Body,
    handler::{HandlerWithoutStateExt, Handler},
};

// this handler doesn't require any state
async fn one() {}
// so it can be converted to a service with `HandlerWithoutStateExt::into_service`
assert_service(one.into_service());

// this handler requires state
async fn two(_: State<String>) {}
// so we have to provide it
let handler_with_state = two.with_state(String::new());
// which gives us a `Service`
assert_service(handler_with_state);

// helper to check that a value implements `Service`
fn assert_service<S>(service: S)
where
    S: Service<Request>,
{}
```

**Debugging handler type errors**

For a function to be used as a handler it must implement the `Handler` trait.
axum provides blanket implementations for functions that:

- Are `async fn`s.

- Take no more than 16 arguments that all implement `Send`.

- All except the last argument implement `FromRequestParts`.

- The last argument implements `FromRequest`.

- Returns something that implements `IntoResponse`.

- If a closure is used it must implement `Clone + Send` and be
`'static`.

- Returns a future that is `Send`. The most common way

...

**Methods**

- `type Future: Future<Output = Response> + Send + 'static`
- `fn call(self, req: Request, state: S) -> Self::Future`
- `fn layer<L>(self, layer: L) -> Layered<L, Self, T, S>where L: Layer<HandlerService<Self, T, S>> + Clone, L::Service: Service<Request>,`
- `fn with_state(self, state: S) -> HandlerService<Self, T, S>`

### trait axum::handler::HandlerWithoutStateExt

```rust
pub trait HandlerWithoutStateExt<T>: Handler<T, ()> {
    // Required methods
    fn into_service(self) -> HandlerService<Self, T, ()>;
    fn into_make_service(self) -> IntoMakeService<HandlerService<Self, T, ()>>;
    fn into_make_service_with_connect_info<C>(
        self,
    ) -> IntoMakeServiceWithConnectInfo<HandlerService<Self, T, ()>, C>;
}
```

Extension trait for `Handler`s that don’t have state.

This provides convenience methods to convert the `Handler` into a [`Service`] or `MakeService`.

**Methods**

- `fn into_service(self) -> HandlerService<Self, T, ()>`
- `fn into_make_service(self) -> IntoMakeService<HandlerService<Self, T, ()>>`
- `fn into_make_service_with_connect_info<C>( self, ) -> IntoMakeServiceWithConnectInfo<HandlerService<Self, T, ()>, C>`
