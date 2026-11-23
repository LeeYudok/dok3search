# Module tower-http::classify

## Overview

Tools for classifying responses as either success or failure.

## Items

- `GrpcEosErrorsAsFailures` — The ClassifyEos for GrpcErrorsAsFailures.
- `GrpcErrorsAsFailures` — Response classifier for gRPC responses.
- `GrpcStatus` — A gRPC status extracted from response headers/trailers.
- `MapFailureClass` — Response classifier that transforms the failure class of some other classifier.
- `NeverClassifyEos` — A ClassifyEos type that can be used in ClassifyResponse implementations that never have to classify streaming responses.
- `ServerErrorsAsFailures` — The default classifier used for normal HTTP responses.
- `SharedClassifier` — A MakeClassifier that produces new classifiers by cloning an inner classifier.
- `StatusInRangeAsFailures` — Response classifier that considers responses with a status code within some range to be failures.
- `ClassifiedResponse` — Result of doing a classification.
- `GrpcCode` — gRPC status codes.
- `GrpcFailureClass` — The failure class for GrpcErrorsAsFailures.
- `ServerErrorsFailureClass` — The failure class for ServerErrorsAsFailures.
- `StatusInRangeFailureClass` — The failure class for StatusInRangeAsFailures.
- `ClassifyEos` — Trait for classifying end of streams (EOS) as either success or failure.
- `ClassifyResponse` — Trait for classifying responses as either success or failure. Designed to support both unary requests (single request for a single response) as well as streaming responses.
- `MakeClassifier` — Trait for producing response classifiers from a request.

### enum tower-http::classify::ClassifiedResponse

```rust
pub enum ClassifiedResponse<FailureClass, ClassifyEos> {
    Ready(Result<(), FailureClass>),
    RequiresEos(ClassifyEos),
}
```

Result of doing a classification.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### enum tower-http::classify::GrpcCode

```rust
#[non_exhaustive]#[repr(i32)]pub enum GrpcCode {
    Ok = 0,
    Cancelled = 1,
    Unknown = 2,
    InvalidArgument = 3,
    DeadlineExceeded = 4,
    NotFound = 5,
    AlreadyExists = 6,
    PermissionDenied = 7,
    ResourceExhausted = 8,
    FailedPrecondition = 9,
    Aborted = 10,
    OutOfRange = 11,
    Unimplemented = 12,
    Internal = 13,
    Unavailable = 14,
    DataLoss = 15,
    Unauthenticated = 16,
}
```

gRPC status codes.

These variants match the gRPC status codes.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn from(value: NonZeroI32) -> Self`
- `fn from(value: i32) -> Self`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### enum tower-http::classify::GrpcFailureClass

```rust
#[non_exhaustive]pub enum GrpcFailureClass {
    Status(GrpcStatus),
    Error(String),
}
```

The failure class for `GrpcErrorsAsFailures`.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`

### enum tower-http::classify::ServerErrorsFailureClass

```rust
pub enum ServerErrorsFailureClass {
    StatusCode(StatusCode),
    Error(String),
}
```

The failure class for `ServerErrorsAsFailures`.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### enum tower-http::classify::StatusInRangeFailureClass

```rust
pub enum StatusInRangeFailureClass {
    StatusCode(StatusCode),
    Error(String),
}
```

The failure class for `StatusInRangeAsFailures`.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tower-http::classify::GrpcEosErrorsAsFailures

```rust
pub struct GrpcEosErrorsAsFailures { /* private fields */ }
```

The `ClassifyEos` for `GrpcErrorsAsFailures`.

**Methods**

- `fn classify_eos( self, trailers: Option<&HeaderMap>, ) -> Result<(), Self::FailureClass>`
- `fn classify_error<E>(self, error: &E) -> Self::FailureClasswhere E: Display + 'static,`
- `fn map_failure_class<F, NewClass>(self, f: F) -> MapFailureClass<Self, F>where Self: Sized, F: FnOnce(Self::FailureClass) -> NewClass,`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tower-http::classify::GrpcErrorsAsFailures

```rust
pub struct GrpcErrorsAsFailures { /* private fields */ }
```

Response classifier for gRPC responses.

gRPC doesn’t use normal HTTP statuses for indicating success or failure but instead a special
header that might appear in a trailer.

Responses are considered successful if

- `grpc-status` header value contains a success value.

- `grpc-status` header is missing.

- `grpc-status` header value isn’t a valid `String`.

- `grpc-status` header value can’t parsed into an `i32`.

All others are considered failures.

**Methods**

- `pub fn new() -> Self`
- `pub fn with_success(self, code: GrpcCode) -> Self`
- `pub fn make_classifier() -> SharedClassifier<Self>`
- `fn classify_response<B>( self, res: &Response<B>, ) -> ClassifiedResponse<Self::FailureClass, Self::ClassifyEos>`
- `fn classify_error<E>(self, error: &E) -> Self::FailureClasswhere E: Display + 'static,`
- `fn map_failure_class<F, NewClass>(self, f: F) -> MapFailureClass<Self, F>where Self: Sized, F: FnOnce(Self::FailureClass) -> NewClass,`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`

### struct tower-http::classify::GrpcStatus

```rust
pub struct GrpcStatus { /* private fields */ }
```

A gRPC status extracted from response headers/trailers.

**Methods**

- `pub fn code(&self) -> Option<GrpcCode>`
- `pub fn code_raw(&self) -> i32`
- `pub fn message(&self) -> Option<&str>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### struct tower-http::classify::MapFailureClass

```rust
pub struct MapFailureClass<C, F> { /* private fields */ }
```

Response classifier that transforms the failure class of some other
classifier.

Created with `ClassifyResponse::map_failure_class` or
`ClassifyEos::map_failure_class`.

**Methods**

- `fn classify_eos( self, trailers: Option<&HeaderMap>, ) -> Result<(), Self::FailureClass>`
- `fn classify_error<E>(self, error: &E) -> Self::FailureClasswhere E: Display + 'static,`
- `fn map_failure_class<F, NewClass>(self, f: F) -> MapFailureClass<Self, F>where Self: Sized, F: FnOnce(Self::FailureClass) -> NewClass,`
- `fn classify_response<B>( self, res: &Response<B>, ) -> ClassifiedResponse<Self::FailureClass, Self::ClassifyEos>`
- `fn classify_error<E>(self, error: &E) -> Self::FailureClasswhere E: Display + 'static,`
- `fn map_failure_class<F, NewClass>(self, f: F) -> MapFailureClass<Self, F>where Self: Sized, F: FnOnce(Self::FailureClass) -> NewClass,`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tower-http::classify::NeverClassifyEos

```rust
pub struct NeverClassifyEos<T> { /* private fields */ }
```

A `ClassifyEos` type that can be used in `ClassifyResponse` implementations that never have
to classify streaming responses.

`NeverClassifyEos` exists only as type.  `NeverClassifyEos` values cannot be constructed.

**Methods**

- `fn classify_eos( self, _trailers: Option<&HeaderMap>, ) -> Result<(), Self::FailureClass>`
- `fn classify_error<E>(self, _error: &E) -> Self::FailureClasswhere E: Display + 'static,`
- `fn map_failure_class<F, NewClass>(self, f: F) -> MapFailureClass<Self, F>where Self: Sized, F: FnOnce(Self::FailureClass) -> NewClass,`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tower-http::classify::ServerErrorsAsFailures

```rust
pub struct ServerErrorsAsFailures { /* private fields */ }
```

The default classifier used for normal HTTP responses.

Responses with a `5xx` status code are considered failures, all others are considered
successes.

**Methods**

- `pub fn new() -> Self`
- `pub fn make_classifier() -> SharedClassifier<Self>`
- `fn classify_response<B>( self, res: &Response<B>, ) -> ClassifiedResponse<Self::FailureClass, Self::ClassifyEos>`
- `fn classify_error<E>(self, error: &E) -> Self::FailureClasswhere E: Display + 'static,`
- `fn map_failure_class<F, NewClass>(self, f: F) -> MapFailureClass<Self, F>where Self: Sized, F: FnOnce(Self::FailureClass) -> NewClass,`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`

### struct tower-http::classify::SharedClassifier

```rust
pub struct SharedClassifier<C> { /* private fields */ }
```

A `MakeClassifier` that produces new classifiers by cloning an inner classifier.

When a type implementing `ClassifyResponse` doesn’t depend on information
from the request, `SharedClassifier` can be used to turn an instance of that type
into a `MakeClassifier`.

**Example**

```rust
use std::fmt;
use tower_http::classify::{
    ClassifyResponse, ClassifiedResponse, NeverClassifyEos,
    SharedClassifier, MakeClassifier,
};
use http::Response;

// A response classifier that only considers errors to be failures.
#[derive(Clone, Copy)]
struct MyClassifier;

impl ClassifyResponse for MyClassifier {
    type FailureClass = String;
    type ClassifyEos = NeverClassifyEos<Self::FailureClass>;

    fn classify_response<B>(
        self,
        _res: &Response<B>,
    ) -> ClassifiedResponse<Self::FailureClass, Self::ClassifyEos> {
        ClassifiedResponse::Ready(Ok(()))
    }

    fn classify_error<E>(self, error: &E) -> Self::FailureClass
    where
        E: fmt::Display + 'static,
    {
        error.to_string()
    }
}

// Some function that requires a `MakeClassifier`
fn use_make_classifier<M: MakeClassifier>(make: M) {
    // ...
}

// `MyClassifier` doesn't implement `MakeClassifier` but since it doesn't
// care about the incoming request we can make `MyClassifier`s by cloning.
// That is what `SharedClassifier` does.
let make_classifier = SharedClassifier::new(MyClassifier);

// We now have a `MakeClassifier`!
use_make_classifier(make_classifier);
```

**Methods**

- `pub fn new(classifier: C) -> Selfwhere C: ClassifyResponse + Clone,`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn make_classifier<B>(&self, _req: &Request<B>) -> Self::Classifier`

### struct tower-http::classify::StatusInRangeAsFailures

```rust
pub struct StatusInRangeAsFailures { /* private fields */ }
```

Response classifier that considers responses with a status code within some range to be
failures.

**Example**

A client with tracing where server errors and client errors are considered failures.

```rust
use tower_http::{trace::TraceLayer, classify::StatusInRangeAsFailures};
use tower::{ServiceBuilder, Service, ServiceExt};
use http::{Request, Method};
use http_body_util::Full;
use bytes::Bytes;
use hyper_util::{rt::TokioExecutor, client::legacy::Client};

let classifier = StatusInRangeAsFailures::new(400..=599);

let client = Client::builder(TokioExecutor::new()).build_http();
let mut client = ServiceBuilder::new()
    .layer(TraceLayer::new(classifier.into_make_classifier()))
    .service(client);

let request = Request::builder()
    .method(Method::GET)
    .uri("https://example.com")
    .body(Full::<Bytes>::default())
    .unwrap();

let response = client.ready().await?.call(request).await?;
```

**Methods**

- `pub fn new(range: RangeInclusive<u16>) -> Self`
- `pub fn new_for_client_and_server_errors() -> Self`
- `pub fn into_make_classifier(self) -> SharedClassifier<Self>`
- `fn classify_response<B>( self, res: &Response<B>, ) -> ClassifiedResponse<Self::FailureClass, Self::ClassifyEos>`
- `fn classify_error<E>(self, error: &E) -> Self::FailureClasswhere E: Display + 'static,`
- `fn map_failure_class<F, NewClass>(self, f: F) -> MapFailureClass<Self, F>where Self: Sized, F: FnOnce(Self::FailureClass) -> NewClass,`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### trait tower-http::classify::ClassifyEos

```rust
pub trait ClassifyEos {
    type FailureClass;

    // Required methods
    fn classify_eos(
        self,
        trailers: Option<&HeaderMap>,
    ) -> Result<(), Self::FailureClass>;
    fn classify_error<E>(self, error: &E) -> Self::FailureClass
       where E: Display + 'static;

    // Provided method
    fn map_failure_class<F, NewClass>(self, f: F) -> MapFailureClass<Self, F>
       where Self: Sized,
             F: FnOnce(Self::FailureClass) -> NewClass { ... }
}
```

Trait for classifying end of streams (EOS) as either success or failure.

**Methods**

- `type FailureClass`
- `fn classify_eos( self, trailers: Option<&HeaderMap>, ) -> Result<(), Self::FailureClass>`
- `fn classify_error<E>(self, error: &E) -> Self::FailureClasswhere E: Display + 'static,`
- `fn map_failure_class<F, NewClass>(self, f: F) -> MapFailureClass<Self, F>where Self: Sized, F: FnOnce(Self::FailureClass) -> NewClass,`

### trait tower-http::classify::ClassifyResponse

```rust
pub trait ClassifyResponse {
    type FailureClass;
    type ClassifyEos: ClassifyEos<FailureClass = Self::FailureClass>;

    // Required methods
    fn classify_response<B>(
        self,
        res: &Response<B>,
    ) -> ClassifiedResponse<Self::FailureClass, Self::ClassifyEos>;
    fn classify_error<E>(self, error: &E) -> Self::FailureClass
       where E: Display + 'static;

    // Provided method
    fn map_failure_class<F, NewClass>(self, f: F) -> MapFailureClass<Self, F>
       where Self: Sized,
             F: FnOnce(Self::FailureClass) -> NewClass { ... }
}
```

Trait for classifying responses as either success or failure. Designed to support both unary
requests (single request for a single response) as well as streaming responses.

Response classifiers are used in cases where middleware needs to determine
whether a response completed successfully or failed. For example, they may
be used by logging or metrics middleware to record failures differently
from successes.

Furthermore, when a response fails, a response classifier may provide
additional information about the failure. This can, for example, be used to
build retry policies by indicating whether or not a particular failure is
retryable.

**Methods**

- `type FailureClass`
- `type ClassifyEos: ClassifyEos<FailureClass = Self::FailureClass>`
- `fn classify_response<B>( self, res: &Response<B>, ) -> ClassifiedResponse<Self::FailureClass, Self::ClassifyEos>`
- `fn classify_error<E>(self, error: &E) -> Self::FailureClasswhere E: Display + 'static,`
- `fn map_failure_class<F, NewClass>(self, f: F) -> MapFailureClass<Self, F>where Self: Sized, F: FnOnce(Self::FailureClass) -> NewClass,`

### trait tower-http::classify::MakeClassifier

```rust
pub trait MakeClassifier {
    type Classifier: ClassifyResponse<FailureClass = Self::FailureClass, ClassifyEos = Self::ClassifyEos>;
    type FailureClass;
    type ClassifyEos: ClassifyEos<FailureClass = Self::FailureClass>;

    // Required method
    fn make_classifier<B>(&self, req: &Request<B>) -> Self::Classifier;
}
```

Trait for producing response classifiers from a request.

This is useful when a classifier depends on data from the request. For example, this could
include the URI or HTTP method.

This trait is generic over the `Error` type of the `Service`s used with the classifier.
This is necessary for `ClassifyResponse::classify_error`.

**Methods**

- `type Classifier: ClassifyResponse<FailureClass = Self::FailureClass, ClassifyEos = Self::ClassifyEos>`
- `type FailureClass`
- `type ClassifyEos: ClassifyEos<FailureClass = Self::FailureClass>`
- `fn make_classifier<B>(&self, req: &Request<B>) -> Self::Classifier`
