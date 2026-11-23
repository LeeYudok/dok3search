# Module tokio::net::tcp

## Overview

TCP utility types.

## Items

- `OwnedReadHalf` — Owned read half of a TcpStream, created by into_split.
- `OwnedWriteHalf` — Owned write half of a TcpStream, created by into_split.
- `ReadHalf` — Borrowed read half of a TcpStream, created by split.
- `ReuniteError` — Error indicating that two halves were not from the same socket, and thus could not be reunited.
- `WriteHalf` — Borrowed write half of a TcpStream, created by split.

### struct tokio::net::tcp::OwnedReadHalf

```rust
pub struct OwnedReadHalf { /* private fields */ }
```

Owned read half of a `TcpStream`, created by `into_split`.

Reading from an `OwnedReadHalf` is usually done using the convenience methods found
on the `AsyncReadExt` trait.

**Methods**

- `pub fn reunite(self, other: OwnedWriteHalf) -> Result<TcpStream, ReuniteError>`
- `pub fn poll_peek( &mut self, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<usize>>`
- `pub async fn peek(&mut self, buf: &mut [u8]) -> Result<usize>`
- `pub async fn ready(&self, interest: Interest) -> Result<Ready>`
- `pub async fn readable(&self) -> Result<()>`
- `pub fn try_read(&self, buf: &mut [u8]) -> Result<usize>`
- `pub fn try_read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> Result<usize>`
- `pub fn try_read_buf<B: BufMut>(&self, buf: &mut B) -> Result<usize>`
- `pub fn peer_addr(&self) -> Result<SocketAddr>`
- `pub fn local_addr(&self) -> Result<SocketAddr>`
- `fn as_ref(&self) -> &TcpStream`
- `fn poll_read( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<()>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tokio::net::tcp::OwnedWriteHalf

```rust
pub struct OwnedWriteHalf { /* private fields */ }
```

Owned write half of a `TcpStream`, created by `into_split`.

Note that in the `AsyncWrite` implementation of this type, `poll_shutdown` will
shut down the TCP stream in the write direction.  Dropping the write half
will also shut down the write half of the TCP stream.

Writing to an `OwnedWriteHalf` is usually done using the convenience methods found
on the `AsyncWriteExt` trait.

**Methods**

- `pub fn reunite(self, other: OwnedReadHalf) -> Result<TcpStream, ReuniteError>`
- `pub fn forget(self)`
- `pub async fn ready(&self, interest: Interest) -> Result<Ready>`
- `pub async fn writable(&self) -> Result<()>`
- `pub fn try_write(&self, buf: &[u8]) -> Result<usize>`
- `pub fn try_write_vectored(&self, bufs: &[IoSlice<'_>]) -> Result<usize>`
- `pub fn peer_addr(&self) -> Result<SocketAddr>`
- `pub fn local_addr(&self) -> Result<SocketAddr>`
- `fn as_ref(&self) -> &TcpStream`
- `fn poll_write( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8], ) -> Poll<Result<usize>>`
- `fn poll_write_vectored( self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &[IoSlice<'_>], ) -> Poll<Result<usize>>`
- `fn is_write_vectored(&self) -> bool`
- `fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>>`
- `fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`

### struct tokio::net::tcp::ReadHalf

```rust
pub struct ReadHalf<'a>(/* private fields */);
```

Borrowed read half of a `TcpStream`, created by `split`.

Reading from a `ReadHalf` is usually done using the convenience methods found on the
`AsyncReadExt` trait.

**Methods**

- `pub fn poll_peek( &mut self, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<usize>>`
- `pub async fn peek(&mut self, buf: &mut [u8]) -> Result<usize>`
- `pub async fn ready(&self, interest: Interest) -> Result<Ready>`
- `pub async fn readable(&self) -> Result<()>`
- `pub fn try_read(&self, buf: &mut [u8]) -> Result<usize>`
- `pub fn try_read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> Result<usize>`
- `pub fn try_read_buf<B: BufMut>(&self, buf: &mut B) -> Result<usize>`
- `pub fn peer_addr(&self) -> Result<SocketAddr>`
- `pub fn local_addr(&self) -> Result<SocketAddr>`
- `fn as_ref(&self) -> &TcpStream`
- `fn poll_read( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<()>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tokio::net::tcp::ReuniteError

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

### struct tokio::net::tcp::WriteHalf

```rust
pub struct WriteHalf<'a>(/* private fields */);
```

Borrowed write half of a `TcpStream`, created by `split`.

Note that in the `AsyncWrite` implementation of this type, `poll_shutdown` will
shut down the TCP stream in the write direction.

Writing to an `WriteHalf` is usually done using the convenience methods found
on the `AsyncWriteExt` trait.

**Methods**

- `pub async fn ready(&self, interest: Interest) -> Result<Ready>`
- `pub async fn writable(&self) -> Result<()>`
- `pub fn try_write(&self, buf: &[u8]) -> Result<usize>`
- `pub fn try_write_vectored(&self, bufs: &[IoSlice<'_>]) -> Result<usize>`
- `pub fn peer_addr(&self) -> Result<SocketAddr>`
- `pub fn local_addr(&self) -> Result<SocketAddr>`
- `fn as_ref(&self) -> &TcpStream`
- `fn poll_write( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8], ) -> Poll<Result<usize>>`
- `fn poll_write_vectored( self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &[IoSlice<'_>], ) -> Poll<Result<usize>>`
- `fn is_write_vectored(&self) -> bool`
- `fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>>`
- `fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
