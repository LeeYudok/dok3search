# Module tokio::net::unix

## Overview

Unix specific network types.

## Items

- `pipe` — Unix pipe types.
- `OwnedReadHalf` — Owned read half of a UnixStream, created by into_split.
- `OwnedWriteHalf` — Owned write half of a UnixStream, created by into_split.
- `ReadHalf` — Borrowed read half of a UnixStream, created by split.
- `ReuniteError` — Error indicating that two halves were not from the same socket, and thus could not be reunited.
- `SocketAddr` — An address associated with a Tokio Unix socket.
- `UCred` — Credentials of a process.
- `WriteHalf` — Borrowed write half of a UnixStream, created by split.
- `gid_t` — A type representing group ID.
- `pid_t` — A type representing process and process group IDs.
- `uid_t` — A type representing user ID.

### struct tokio::net::unix::OwnedReadHalf

```rust
pub struct OwnedReadHalf { /* private fields */ }
```

Owned read half of a `UnixStream`, created by `into_split`.

Reading from an `OwnedReadHalf` is usually done using the convenience methods found
on the `AsyncReadExt` trait.

**Methods**

- `pub fn reunite(self, other: OwnedWriteHalf) -> Result<UnixStream, ReuniteError>`
- `pub async fn ready(&self, interest: Interest) -> Result<Ready>`
- `pub async fn readable(&self) -> Result<()>`
- `pub fn try_read(&self, buf: &mut [u8]) -> Result<usize>`
- `pub fn try_read_buf<B: BufMut>(&self, buf: &mut B) -> Result<usize>`
- `pub fn try_read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> Result<usize>`
- `pub fn peer_addr(&self) -> Result<SocketAddr>`
- `pub fn local_addr(&self) -> Result<SocketAddr>`
- `fn as_ref(&self) -> &UnixStream`
- `fn poll_read( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<()>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tokio::net::unix::OwnedWriteHalf

```rust
pub struct OwnedWriteHalf { /* private fields */ }
```

Owned write half of a `UnixStream`, created by `into_split`.

Note that in the `AsyncWrite` implementation of this type,
`poll_shutdown` will shut down the stream in the write direction.
Dropping the write half will also shut down the write half of the stream.

Writing to an `OwnedWriteHalf` is usually done using the convenience methods
found on the `AsyncWriteExt` trait.

**Methods**

- `pub fn reunite(self, other: OwnedReadHalf) -> Result<UnixStream, ReuniteError>`
- `pub fn forget(self)`
- `pub async fn ready(&self, interest: Interest) -> Result<Ready>`
- `pub async fn writable(&self) -> Result<()>`
- `pub fn try_write(&self, buf: &[u8]) -> Result<usize>`
- `pub fn try_write_vectored(&self, buf: &[IoSlice<'_>]) -> Result<usize>`
- `pub fn peer_addr(&self) -> Result<SocketAddr>`
- `pub fn local_addr(&self) -> Result<SocketAddr>`
- `fn as_ref(&self) -> &UnixStream`
- `fn poll_write( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8], ) -> Poll<Result<usize>>`
- `fn poll_write_vectored( self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &[IoSlice<'_>], ) -> Poll<Result<usize>>`
- `fn is_write_vectored(&self) -> bool`
- `fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>>`
- `fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`

### struct tokio::net::unix::ReadHalf

```rust
pub struct ReadHalf<'a>(/* private fields */);
```

Borrowed read half of a `UnixStream`, created by `split`.

Reading from a `ReadHalf` is usually done using the convenience methods found on the
`AsyncReadExt` trait.

**Methods**

- `pub async fn ready(&self, interest: Interest) -> Result<Ready>`
- `pub async fn readable(&self) -> Result<()>`
- `pub fn try_read(&self, buf: &mut [u8]) -> Result<usize>`
- `pub fn try_read_buf<B: BufMut>(&self, buf: &mut B) -> Result<usize>`
- `pub fn try_read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> Result<usize>`
- `pub fn peer_addr(&self) -> Result<SocketAddr>`
- `pub fn local_addr(&self) -> Result<SocketAddr>`
- `fn as_ref(&self) -> &UnixStream`
- `fn poll_read( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<()>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tokio::net::unix::ReuniteError

```rust
pub struct ReuniteError(pub OwnedReadHalf, pub OwnedWriteHalf);
```

Error indicating that two halves were not from the same socket, and thus could
not be reunited.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`

### struct tokio::net::unix::SocketAddr

```rust
pub struct SocketAddr(/* private fields */);
```

An address associated with a Tokio Unix socket.

This type is a thin wrapper around `std::os::unix::net::SocketAddr`. You
can convert to and from the standard library `SocketAddr` type using the
`From` trait.

**Methods**

- `pub fn is_unnamed(&self) -> bool`
- `pub fn as_pathname(&self) -> Option<&Path>`
- `pub fn as_abstract_name(&self) -> Option<&[u8]>`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `fn from(value: SocketAddr) -> Self`
- `fn from(value: SocketAddr) -> Self`

### struct tokio::net::unix::UCred

```rust
pub struct UCred { /* private fields */ }
```

Credentials of a process.

**Methods**

- `pub fn uid(&self) -> uid_t`
- `pub fn gid(&self) -> gid_t`
- `pub fn pid(&self) -> Option<pid_t>`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn hash<__H: Hasher>(&self, state: &mut __H)`
- `1.3.0 · fn hash_slice<H>(data: &[Self], state: &mut H)where H: Hasher, Self: Sized,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### struct tokio::net::unix::WriteHalf

```rust
pub struct WriteHalf<'a>(/* private fields */);
```

Borrowed write half of a `UnixStream`, created by `split`.

Note that in the `AsyncWrite` implementation of this type, `poll_shutdown` will
shut down the `UnixStream` stream in the write direction.

Writing to an `WriteHalf` is usually done using the convenience methods found
on the `AsyncWriteExt` trait.

**Methods**

- `pub async fn ready(&self, interest: Interest) -> Result<Ready>`
- `pub async fn writable(&self) -> Result<()>`
- `pub fn try_write(&self, buf: &[u8]) -> Result<usize>`
- `pub fn try_write_vectored(&self, buf: &[IoSlice<'_>]) -> Result<usize>`
- `pub fn peer_addr(&self) -> Result<SocketAddr>`
- `pub fn local_addr(&self) -> Result<SocketAddr>`
- `fn as_ref(&self) -> &UnixStream`
- `fn poll_write( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8], ) -> Poll<Result<usize>>`
- `fn poll_write_vectored( self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &[IoSlice<'_>], ) -> Poll<Result<usize>>`
- `fn is_write_vectored(&self) -> bool`
- `fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>>`
- `fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### type tokio::net::unix::gid_t

```rust
pub type gid_t = u32;
```

A type representing group ID.

### type tokio::net::unix::pid_t

```rust
pub type pid_t = i32;
```

A type representing process and process group IDs.

### type tokio::net::unix::uid_t

```rust
pub type uid_t = u32;
```

A type representing user ID.
