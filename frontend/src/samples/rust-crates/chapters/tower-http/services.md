# Module tower-http::services

## Overview

`Service`s that return responses without wrapping other `Service`s.

These kinds of services are also referred to as “leaf services” since they sit at the leaves of
a tree of services.

## Items

- `fs` — File system related services.
- `ServeDir` — Service that serves files from a given directory and all its sub directories.
- `ServeFile` — Service that serves a file.

### struct tower-http::services::ServeDir

```rust
pub struct ServeDir<F = DefaultServeDirFallback, B = TokioBackend> { /* private fields */ }
```

Service that serves files from a given directory and all its sub directories.

The `Content-Type` will be guessed from the file extension.

An empty response with status `404 Not Found` will be returned if:

- The file doesn’t exist

- Any segment of the path contains `..`

- Any segment of the path contains a backslash

- On unix, any segment of the path referenced as directory is actually an
existing file (`/file.html/something`)

- We don’t have necessary permissions to read the file

**Example**

```rust
use tower_http::services::ServeDir;

// This will serve files in the "assets" directory and
// its subdirectories
let service = ServeDir::new("assets");
```

**Methods**

- `pub fn new<P>(path: P) -> Selfwhere P: AsRef<Path>,`
- `pub fn with_backend<P>(path: P, backend: B) -> Selfwhere P: AsRef<Path>,`
- `pub fn append_index_html_on_directories(self, append: bool) -> Self`
- `pub fn redirect_to_trailing_slash(self, redirect: bool) -> Self`
- `pub fn html_as_default_extension(self, append: bool) -> Self`
- `pub fn redirect_path_prefix(self, prefix: impl Into<String>) -> Self`
- `pub fn with_buf_chunk_size(self, chunk_size: usize) -> Self`
- `pub fn ignore_multi_range_requests(self, ignore: bool) -> Self`
- `pub fn precompressed_gzip(self) -> Self`
- `pub fn precompressed_br(self) -> Self`
- `pub fn precompressed_deflate(self) -> Self`
- `pub fn precompressed_zstd(self) -> Self`
- `pub fn fallback<F2>(self, new_fallback: F2) -> ServeDir<F2, B>`
- `pub fn not_found_service<F2>( self, new_fallback: F2, ) -> ServeDir<SetStatus<F2>, B>`
- `pub fn call_fallback_on_method_not_allowed(self, call_fallback: bool) -> Self`
- `pub fn try_call<ReqBody, FResBody>( &mut self, req: Request<ReqBody>, ) -> ResponseFuture<ReqBody, F> ⓘwhere F: Service<Request<ReqBody>, Response = Response<FResBody>, Error = Infallible> + Clone, F::Future: Send + 'static, FResBody: Body<Data = Bytes> + Send + 'static, FResBody::Error: Into<Box<dyn Error + Send + Sync>>,`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, req: Request<ReqBody>) -> Self::Future`

### struct tower-http::services::ServeFile

```rust
pub struct ServeFile(/* private fields */);
```

Service that serves a file.

**Methods**

- `pub fn new<P: AsRef<Path>>(path: P) -> Self`
- `pub fn new_with_mime<P: AsRef<Path>>(path: P, mime: &Mime) -> Self`
- `pub fn precompressed_gzip(self) -> Self`
- `pub fn precompressed_br(self) -> Self`
- `pub fn precompressed_deflate(self) -> Self`
- `pub fn precompressed_zstd(self) -> Self`
- `pub fn with_buf_chunk_size(self, chunk_size: usize) -> Self`
- `pub fn ignore_multi_range_requests(self, ignore: bool) -> Self`
- `pub fn try_call<ReqBody>( &mut self, req: Request<ReqBody>, ) -> ResponseFuture<ReqBody> ⓘwhere ReqBody: Send + 'static,`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, req: Request<ReqBody>) -> Self::Future`
