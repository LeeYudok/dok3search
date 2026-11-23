# Module tower::layer

## Overview

A collection of `Layer` based tower services

## Items

- `util` — Utilities for combining layers
- `LayerFn` — A Layer implemented by a closure. See the docs for layer_fn for more details.
- `Layer` — Decorates a Service, transforming either the request or the response.
- `layer_fn` — Returns a new LayerFn that implements Layer by calling the given function.

### fn tower::layer::layer_fn

```rust
pub fn layer_fn<T>(f: T) -> LayerFn<T>
```

Returns a new `LayerFn` that implements `Layer` by calling the
given function.

The `Layer::layer` method takes a type implementing `Service` and
returns a different type implementing `Service`. In many cases, this can
be implemented by a function or a closure. The `LayerFn` helper allows
writing simple `Layer` implementations without needing the boilerplate of
a new struct implementing `Layer`.

**Example**

```rust
// A middleware that logs requests before forwarding them to another service
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
        // Log the request
        println!("request = {:?}, target = {:?}", request, self.target);

        self.service.call(request)
    }
}

// A `Layer` that wraps services in `LogService`
let log_layer = layer_fn(|service| {
    LogService {
        service,
        target: "tower-docs",
    }
});

// An example service. This one uppercases strings
let uppercase_service = tower::service_fn(|request: String| async move {
    Ok::<_, Infallible>(request.to_uppercase())
});

// Wrap our service in a `LogService` so requests are logged.
let wrapped_service = log_layer.layer(uppercase_service);
```

### struct tower::layer::LayerFn

```rust
pub struct LayerFn<F> { /* private fields */ }
```

A `Layer` implemented by a closure. See the docs for `layer_fn` for more details.

**Methods**

- `fn clone(&self) -> LayerFn<F>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn layer(&self, inner: S) -> <LayerFn<F> as Layer<S>>::Service`

### trait tower::layer::Layer

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
