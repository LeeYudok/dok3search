# Module tower-http::services::fs

## Overview

File system related services.

## Items

- `AsyncReadBody` — Adapter that turns an [impl AsyncRead][tokio::io::AsyncRead] to an [impl Body][http_body::Body].
- `DefaultServeDirFallback` — The default fallback service used with ServeDir.
- `ServeDir` — Service that serves files from a given directory and all its sub directories.
- `ServeFile` — Service that serves a file.
- `ServeFileSystemResponseBody` — Response body for ServeDir and ServeFile.
- `ServeFileSystemResponseFuture` — Response future of ServeDir::try_call().
- `TokioBackend` — Default Backend implementation using tokio::fs.
- `TokioFile` — Wrapper around [tokio::fs::File] implementing the File trait.
- `Backend` — Trait abstracting filesystem operations for ServeDir.
- `File` — Trait for an opened file.
- `Metadata` — Trait for file metadata.

### struct tower-http::services::fs::AsyncReadBody

```rust
pub struct AsyncReadBody<T> { /* private fields */ }
```

Adapter that turns an [`impl AsyncRead`][tokio::io::AsyncRead] to an [`impl Body`][http_body::Body].

**Methods**

- `pub fn with_capacity(read: T, capacity: usize) -> Self`
- `fn poll_frame( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>>`
- `fn is_end_stream(&self) -> bool`
- `fn size_hint(&self) -> SizeHint`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tower-http::services::fs::DefaultServeDirFallback

```rust
pub struct DefaultServeDirFallback(/* private fields */);
```

The default fallback service used with `ServeDir`.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>`
- `fn call(&mut self, _req: Request<ReqBody>) -> Self::Future`

### struct tower-http::services::fs::ServeDir

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

### struct tower-http::services::fs::ServeFile

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

### struct tower-http::services::fs::ServeFileSystemResponseBody

```rust
pub struct ServeFileSystemResponseBody { /* private fields */ }
```

Response body for `ServeDir` and `ServeFile`.

**Methods**

- `fn poll_frame( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>>`
- `fn is_end_stream(&self) -> bool`
- `fn size_hint(&self) -> SizeHint`
- `fn default() -> Self`
- `fn from(body: ResponseBody) -> Self`

### struct tower-http::services::fs::ServeFileSystemResponseFuture

```rust
pub struct ServeFileSystemResponseFuture<ReqBody, F = DefaultServeDirFallback> { /* private fields */ }
```

Response future of `ServeDir::try_call()`.

**Methods**

- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`

### struct tower-http::services::fs::TokioBackend

```rust
pub struct TokioBackend;
```

Default `Backend` implementation using `tokio::fs`.

**Methods**

- `fn open(&self, path: PathBuf) -> Self::OpenFuture`
- `fn metadata(&self, path: PathBuf) -> Self::MetadataFuture`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`

### struct tower-http::services::fs::TokioFile

```rust
pub struct TokioFile(/* private fields */);
```

Wrapper around [`tokio::fs::File`] implementing the `File` trait.

**Methods**

- `fn poll_read( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<()>>`
- `fn start_seek(self: Pin<&mut Self>, position: SeekFrom) -> Result<()>`
- `fn poll_complete( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<u64>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn metadata(&self) -> Self::MetadataFuture<'_>`

### trait tower-http::services::fs::Backend

```rust
pub trait Backend:
    Clone
    + Send
    + Sync
    + 'static {
    type File: File<Metadata = Self::Metadata>;
    type Metadata: Metadata;
    type OpenFuture: Future<Output = Result<Self::File>> + Send;
    type MetadataFuture: Future<Output = Result<Self::Metadata>> + Send;

    // Required methods
    fn open(&self, path: PathBuf) -> Self::OpenFuture;
    fn metadata(&self, path: PathBuf) -> Self::MetadataFuture;
}
```

Trait abstracting filesystem operations for `ServeDir`.

Implement this trait to serve files from non-filesystem sources.
The default implementation (`TokioBackend`) wraps `tokio::fs`.

**Methods**

- `type File: File<Metadata = Self::Metadata>`
- `type Metadata: Metadata`
- `type OpenFuture: Future<Output = Result<Self::File>> + Send`
- `type MetadataFuture: Future<Output = Result<Self::Metadata>> + Send`
- `fn open(&self, path: PathBuf) -> Self::OpenFuture`
- `fn metadata(&self, path: PathBuf) -> Self::MetadataFuture`

### trait tower-http::services::fs::File

```rust
pub trait File:
    AsyncRead
    + AsyncSeek
    + Unpin
    + Send
    + Sync {
    type Metadata: Metadata;
    type MetadataFuture<'a>: Future<Output = Result<Self::Metadata>> + Send
       where Self: 'a;

    // Required method
    fn metadata(&self) -> Self::MetadataFuture<'_>;
}
```

Trait for an opened file.

Must support async reading and seeking (for HTTP range requests).
In-memory backends can use `std::io::Cursor` to satisfy the `AsyncSeek` requirement.

**Methods**

- `type Metadata: Metadata`
- `type MetadataFuture<'a>: Future<Output = Result<Self::Metadata>> + Send where Self: 'a`
- `fn metadata(&self) -> Self::MetadataFuture<'_>`

### trait tower-http::services::fs::Metadata

```rust
pub trait Metadata: Send + 'static {
    // Required methods
    fn is_dir(&self) -> bool;
    fn modified(&self) -> Result<SystemTime>;
    fn len(&self) -> u64;

    // Provided method
    fn is_empty(&self) -> bool { ... }
}
```

Trait for file metadata.

This is the information `ServeDir` needs about a file or directory without opening it.

**Methods**

- `fn is_dir(&self) -> bool`
- `fn modified(&self) -> Result<SystemTime>`
- `fn len(&self) -> u64`
- `fn is_empty(&self) -> bool`
- `fn is_dir(&self) -> bool`
- `fn modified(&self) -> Result<SystemTime>`
- `fn len(&self) -> u64`
