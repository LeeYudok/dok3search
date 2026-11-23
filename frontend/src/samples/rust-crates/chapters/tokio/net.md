# Module tokio::net

## Overview

TCP/UDP/Unix bindings for `tokio`.

This module contains the TCP/UDP/Unix networking types, similar to the standard
library, which can be used to implement networking protocols.

**Organization**

- `TcpListener` and `TcpStream` provide functionality for communication over TCP

- `UdpSocket` provides functionality for communication over UDP

- `UnixListener` and `UnixStream` provide functionality for communication over a
Unix Domain Stream Socket (available on Unix only)

- `UnixDatagram` provides functionality for communication
over Unix Domain Datagram Socket (available on Unix only)

- `tokio::net::unix::pipe` for FIFO pipes (available on Unix only)

- `tokio::net::windows::named_pipe` for Named Pipes (available on Windows only)

For IO resources not available in `tokio::net`, you can use `AsyncFd`.

## Items

- `tcp` — TCP utility types.
- `unix` — Unix specific network types.
- `TcpListener` — A TCP socket server, listening for connections.
- `TcpSocket` — A TCP socket that has not yet been converted to a TcpStream or TcpListener.
- `TcpStream` — A TCP stream between a local and a remote socket.
- `UdpSocket` — A UDP socket.
- `UnixDatagram` — An I/O object representing a Unix datagram socket.
- `UnixListener` — A Unix socket which can accept connections from other Unix sockets.
- `UnixSocket` — A Unix socket that has not yet been converted to a UnixStream, UnixDatagram, or UnixListener.
- `UnixStream` — A structure representing a connected Unix socket.
- `ToSocketAddrs` — Converts or resolves without blocking to one or more SocketAddr values.
- `lookup_host` — Performs a DNS resolution.

### fn tokio::net::lookup_host

```rust
pub async fn lookup_host<T>(host: T) -> Result<impl Iterator<Item = SocketAddr>>where
    T: ToSocketAddrs,
```

Performs a DNS resolution.

The returned iterator may not actually yield any values depending on the
outcome of any resolution performed.

This API is not intended to cover all DNS use cases. Anything beyond the
basic use case should be done with a specialized library.

**Examples**

To resolve a DNS entry:

```rust
use tokio::net;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    for addr in net::lookup_host("localhost:3000").await? {
        println!("socket address is {}", addr);
    }

    Ok(())
}
```

### struct tokio::net::TcpListener

```rust
pub struct TcpListener { /* private fields */ }
```

A TCP socket server, listening for connections.

You can accept a new connection by using the `accept`
method.

A `TcpListener` can be turned into a `Stream` with `TcpListenerStream`.

The socket will be closed when the value is dropped.

**Errors**

Note that accepting a connection can lead to various errors and not all
of them are necessarily fatal ‒ for example having too many open file
descriptors or the other side closing the connection while it waits in
an accept queue. These would terminate the stream if not handled in any
way.

**Examples**

Using `accept`:

```rust
use tokio::net::TcpListener;

use std::io;

async fn process_socket<T>(socket: T) {
    // do work with socket here
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        process_socket(socket).await;
    }
}
```

**Methods**

- `pub async fn bind<A: ToSocketAddrs>(addr: A) -> Result<TcpListener>`
- `pub async fn accept(&self) -> Result<(TcpStream, SocketAddr)>`
- `pub fn poll_accept( &self, cx: &mut Context<'_>, ) -> Poll<Result<(TcpStream, SocketAddr)>>`
- `pub fn from_std(listener: TcpListener) -> Result<TcpListener>`
- `pub fn into_std(self) -> Result<TcpListener>`
- `pub fn local_addr(&self) -> Result<SocketAddr>`
- `pub fn ttl(&self) -> Result<u32>`
- `pub fn set_ttl(&self, ttl: u32) -> Result<()>`
- `fn as_fd(&self) -> BorrowedFd<'_>`
- `fn as_raw_fd(&self) -> RawFd`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn try_from(stream: TcpListener) -> Result<Self, Self::Error>`

### struct tokio::net::TcpSocket

```rust
pub struct TcpSocket { /* private fields */ }
```

A TCP socket that has not yet been converted to a `TcpStream` or
`TcpListener`.

`TcpSocket` wraps an operating system socket and enables the caller to
configure the socket before establishing a TCP connection or accepting
inbound connections. The caller is able to set socket option and explicitly
bind the socket with a socket address.

The underlying socket is closed when the `TcpSocket` value is dropped.

`TcpSocket` should only be used directly if the default configuration used
by `TcpStream::connect` and `TcpListener::bind` does not meet the required
use case.

Calling `TcpStream::connect("127.0.0.1:8080")` is equivalent to:

```rust
use tokio::net::TcpSocket;

use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = "127.0.0.1:8080".parse().unwrap();

    let socket = TcpSocket::new_v4()?;
    let stream = socket.connect(addr).await?;

    Ok(())
}
```

Calling `TcpListener::bind("127.0.0.1:8080")` is equivalent to:

...

**Methods**

- `pub fn new_v4() -> Result<TcpSocket>`
- `pub fn new_v6() -> Result<TcpSocket>`
- `pub fn set_keepalive(&self, keepalive: bool) -> Result<()>`
- `pub fn keepalive(&self) -> Result<bool>`
- `pub fn set_reuseaddr(&self, reuseaddr: bool) -> Result<()>`
- `pub fn reuseaddr(&self) -> Result<bool>`
- `pub fn set_reuseport(&self, reuseport: bool) -> Result<()>`
- `pub fn reuseport(&self) -> Result<bool>`
- `pub fn set_send_buffer_size(&self, size: u32) -> Result<()>`
- `pub fn send_buffer_size(&self) -> Result<u32>`
- `pub fn set_recv_buffer_size(&self, size: u32) -> Result<()>`
- `pub fn recv_buffer_size(&self) -> Result<u32>`
- `pub fn set_linger(&self, dur: Option<Duration>) -> Result<()>`
- `pub fn set_zero_linger(&self) -> Result<()>`
- `pub fn linger(&self) -> Result<Option<Duration>>`
- `pub fn set_nodelay(&self, nodelay: bool) -> Result<()>`
- `pub fn nodelay(&self) -> Result<bool>`
- `pub fn tclass_v6(&self) -> Result<u32>`
- `pub fn set_tclass_v6(&self, tclass: u32) -> Result<()>`
- `pub fn tos_v4(&self) -> Result<u32>`
- `pub fn set_tos_v4(&self, tos: u32) -> Result<()>`
- `pub fn device(&self) -> Result<Option<Vec<u8>>>`
- `pub fn bind_device(&self, interface: Option<&[u8]>) -> Result<()>`
- `pub fn local_addr(&self) -> Result<SocketAddr>`
- `pub fn take_error(&self) -> Result<Option<Error>>`
- `pub fn bind(&self, addr: SocketAddr) -> Result<()>`
- `pub async fn connect(self, addr: SocketAddr) -> Result<TcpStream>`
- `pub fn listen(self, backlog: u32) -> Result<TcpListener>`
- `pub fn from_std_stream(std_stream: TcpStream) -> TcpSocket`
- `fn as_fd(&self) -> BorrowedFd<'_>`
- `fn as_raw_fd(&self) -> RawFd`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `unsafe fn from_raw_fd(fd: RawFd) -> TcpSocket`
- `fn into_raw_fd(self) -> RawFd`

### struct tokio::net::TcpStream

```rust
pub struct TcpStream { /* private fields */ }
```

A TCP stream between a local and a remote socket.

A TCP stream can either be created by connecting to an endpoint, via the
`connect` method, or by accepting a connection from a listener. A
TCP stream can also be created via the `TcpSocket` type.

Reading and writing to a `TcpStream` is usually done using the
convenience methods found on the `AsyncReadExt` and `AsyncWriteExt`
traits.

**Examples**

```rust
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    // Write some data.
    stream.write_all(b"hello world!").await?;

    Ok(())
}
```

The `write_all` method is defined on the `AsyncWriteExt` trait.

To shut down the stream in the write direction, you can call the
`shutdown()` method. This will cause the other peer to receive a read of
length 0, indicating that no more data will be sent. This only closes
the stream in one direction.

**Methods**

- `pub async fn connect<A: ToSocketAddrs>(addr: A) -> Result<TcpStream>`
- `pub fn from_std(stream: TcpStream) -> Result<TcpStream>`
- `pub fn into_std(self) -> Result<TcpStream>`
- `pub fn local_addr(&self) -> Result<SocketAddr>`
- `pub fn take_error(&self) -> Result<Option<Error>>`
- `pub fn peer_addr(&self) -> Result<SocketAddr>`
- `pub fn poll_peek( &self, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<usize>>`
- `pub async fn ready(&self, interest: Interest) -> Result<Ready>`
- `pub async fn readable(&self) -> Result<()>`
- `pub fn poll_read_ready(&self, cx: &mut Context<'_>) -> Poll<Result<()>>`
- `pub fn try_read(&self, buf: &mut [u8]) -> Result<usize>`
- `pub fn try_read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> Result<usize>`
- `pub fn try_read_buf<B: BufMut>(&self, buf: &mut B) -> Result<usize>`
- `pub async fn writable(&self) -> Result<()>`
- `pub fn poll_write_ready(&self, cx: &mut Context<'_>) -> Poll<Result<()>>`
- `pub fn try_write(&self, buf: &[u8]) -> Result<usize>`
- `pub fn try_write_vectored(&self, bufs: &[IoSlice<'_>]) -> Result<usize>`
- `pub fn try_io<R>( &self, interest: Interest, f: impl FnOnce() -> Result<R>, ) -> Result<R>`
- `pub async fn async_io<R>( &self, interest: Interest, f: impl FnMut() -> Result<R>, ) -> Result<R>`
- `pub async fn peek(&self, buf: &mut [u8]) -> Result<usize>`
- `pub fn nodelay(&self) -> Result<bool>`
- `pub fn set_nodelay(&self, nodelay: bool) -> Result<()>`
- `pub fn quickack(&self) -> Result<bool>`
- `pub fn set_quickack(&self, quickack: bool) -> Result<()>`
- `pub fn linger(&self) -> Result<Option<Duration>>`
- `pub fn set_linger(&self, dur: Option<Duration>) -> Result<()>`
- `pub fn set_zero_linger(&self) -> Result<()>`
- `pub fn ttl(&self) -> Result<u32>`
- `pub fn set_ttl(&self, ttl: u32) -> Result<()>`
- `pub fn split<'a>(&'a mut self) -> (ReadHalf<'a>, WriteHalf<'a>)`
- `pub fn into_split(self) -> (OwnedReadHalf, OwnedWriteHalf)`
- `fn as_fd(&self) -> BorrowedFd<'_>`
- `fn as_raw_fd(&self) -> RawFd`
- `fn as_ref(&self) -> &TcpStream`
- `fn as_ref(&self) -> &TcpStream`
- `fn as_ref(&self) -> &TcpStream`
- `fn as_ref(&self) -> &TcpStream`
- `fn as_ref(&self) -> &Self`
- `fn poll_read( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<()>>`
- `fn poll_write( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8], ) -> Poll<Result<usize>>`

### struct tokio::net::UdpSocket

```rust
pub struct UdpSocket { /* private fields */ }
```

A UDP socket.

UDP is “connectionless”, unlike TCP. Meaning, regardless of what address you’ve bound to, a `UdpSocket`
is free to communicate with many different remotes. In tokio there are basically two main ways to use `UdpSocket`:

- one to many: `bind` and use `send_to`
and `recv_from` to communicate with many different addresses

- one to one: `connect` and associate with a single address, using `send`
and `recv` to communicate only with that remote address

This type does not provide a `split` method, because this functionality
can be achieved by instead wrapping the socket in an `Arc`. Note that
you do not need a `Mutex` to share the `UdpSocket` — an `Arc<UdpSocket>`
is enough. This is because all of the methods take `&self` instead of
`&mut self`. Once you have wrapped it in an `Arc`, you can call
`.clone()` on the `Arc<UdpSocket>` to get multiple shared handles to the
same socket. An example of such usage can be found further down.

**Streams**

If you need to listen over UDP and produce a `Stream`, you can look
at `UdpFramed`.

**Example: one to many (bind)**

Using `bind` we can create a simple echo server that sends and recv’s with many different clients:

...

**Methods**

- `pub async fn bind<A: ToSocketAddrs>(addr: A) -> Result<UdpSocket>`
- `pub fn from_std(socket: UdpSocket) -> Result<UdpSocket>`
- `pub fn into_std(self) -> Result<UdpSocket>`
- `pub fn local_addr(&self) -> Result<SocketAddr>`
- `pub fn peer_addr(&self) -> Result<SocketAddr>`
- `pub async fn connect<A: ToSocketAddrs>(&self, addr: A) -> Result<()>`
- `pub async fn ready(&self, interest: Interest) -> Result<Ready>`
- `pub async fn writable(&self) -> Result<()>`
- `pub fn poll_send_ready(&self, cx: &mut Context<'_>) -> Poll<Result<()>>`
- `pub async fn send(&self, buf: &[u8]) -> Result<usize>`
- `pub fn poll_send(&self, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize>>`
- `pub fn try_send(&self, buf: &[u8]) -> Result<usize>`
- `pub async fn readable(&self) -> Result<()>`
- `pub fn poll_recv_ready(&self, cx: &mut Context<'_>) -> Poll<Result<()>>`
- `pub async fn recv(&self, buf: &mut [u8]) -> Result<usize>`
- `pub fn poll_recv( &self, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<()>>`
- `pub fn try_recv(&self, buf: &mut [u8]) -> Result<usize>`
- `pub fn try_recv_buf<B: BufMut>(&self, buf: &mut B) -> Result<usize>`
- `pub async fn recv_buf<B: BufMut>(&self, buf: &mut B) -> Result<usize>`
- `pub fn try_recv_buf_from<B: BufMut>( &self, buf: &mut B, ) -> Result<(usize, SocketAddr)>`
- `pub async fn recv_buf_from<B: BufMut>( &self, buf: &mut B, ) -> Result<(usize, SocketAddr)>`
- `pub async fn send_to<A: ToSocketAddrs>( &self, buf: &[u8], addr: A, ) -> Result<usize>`
- `pub fn poll_send_to( &self, cx: &mut Context<'_>, buf: &[u8], target: SocketAddr, ) -> Poll<Result<usize>>`
- `pub fn try_send_to(&self, buf: &[u8], target: SocketAddr) -> Result<usize>`
- `pub async fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr)>`
- `pub fn poll_recv_from( &self, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<SocketAddr>>`
- `pub fn try_recv_from(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr)>`
- `pub fn try_io<R>( &self, interest: Interest, f: impl FnOnce() -> Result<R>, ) -> Result<R>`
- `pub async fn async_io<R>( &self, interest: Interest, f: impl FnMut() -> Result<R>, ) -> Result<R>`
- `pub async fn peek(&self, buf: &mut [u8]) -> Result<usize>`
- `pub fn poll_peek( &self, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<()>>`
- `pub fn try_peek(&self, buf: &mut [u8]) -> Result<usize>`
- `pub async fn peek_from(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr)>`
- `pub fn poll_peek_from( &self, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<SocketAddr>>`
- `pub fn try_peek_from(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr)>`
- `pub async fn peek_sender(&self) -> Result<SocketAddr>`
- `pub fn poll_peek_sender(&self, cx: &mut Context<'_>) -> Poll<Result<SocketAddr>>`
- `pub fn try_peek_sender(&self) -> Result<SocketAddr>`
- `pub fn broadcast(&self) -> Result<bool>`
- `pub fn set_broadcast(&self, on: bool) -> Result<()>`

### struct tokio::net::UnixDatagram

```rust
pub struct UnixDatagram { /* private fields */ }
```

An I/O object representing a Unix datagram socket.

A socket can be either named (associated with a filesystem path) or
unnamed.

This type does not provide a `split` method, because this functionality
can be achieved by wrapping the socket in an `Arc`. Note that you do
not need a `Mutex` to share the `UnixDatagram` — an `Arc<UnixDatagram>`
is enough. This is because all of the methods take `&self` instead of
`&mut self`.

Note: named sockets are persisted even after the object is dropped
and the program has exited, and cannot be reconnected. It is advised
that you either check for and unlink the existing socket if it exists,
or use a temporary file that is guaranteed to not already exist.

**Examples**

Using named sockets, associated with a filesystem path:

```rust
use tokio::net::UnixDatagram;
use tempfile::tempdir;

// We use a temporary directory so that the socket
// files left by the bound sockets will get cleaned up.
let tmp = tempdir()?;

// Bind each socket to a filesystem path
let tx_path = tmp.path().join("tx");
let tx = UnixDatagram::bind(&tx_path)?;
let rx_path = tmp.path().join("rx");
let rx = UnixDatagram::bind(&rx_path)?;

let bytes = b"hello world";
tx.send_to(bytes, &rx_path).await?;

let mut buf = vec![0u8; 24];
let (size, addr) = rx.recv_from(&mut buf).await?;

let dgram = &buf[..size];
assert_eq!(dgram, bytes);
assert_eq!(addr.as_pathname().unwrap(), &tx_path);

```

Using unnamed sockets, created as a pair

...

**Methods**

- `pub async fn ready(&self, interest: Interest) -> Result<Ready>`
- `pub async fn writable(&self) -> Result<()>`
- `pub fn poll_send_ready(&self, cx: &mut Context<'_>) -> Poll<Result<()>>`
- `pub async fn readable(&self) -> Result<()>`
- `pub fn poll_recv_ready(&self, cx: &mut Context<'_>) -> Poll<Result<()>>`
- `pub fn bind<P>(path: P) -> Result<UnixDatagram>where P: AsRef<Path>,`
- `pub fn pair() -> Result<(UnixDatagram, UnixDatagram)>`
- `pub fn from_std(datagram: UnixDatagram) -> Result<UnixDatagram>`
- `pub fn into_std(self) -> Result<UnixDatagram>`
- `pub fn unbound() -> Result<UnixDatagram>`
- `pub fn connect<P: AsRef<Path>>(&self, path: P) -> Result<()>`
- `pub async fn send(&self, buf: &[u8]) -> Result<usize>`
- `pub fn try_send(&self, buf: &[u8]) -> Result<usize>`
- `pub fn try_send_to<P>(&self, buf: &[u8], target: P) -> Result<usize>where P: AsRef<Path>,`
- `pub async fn recv(&self, buf: &mut [u8]) -> Result<usize>`
- `pub fn try_recv(&self, buf: &mut [u8]) -> Result<usize>`
- `pub fn try_recv_buf_from<B: BufMut>( &self, buf: &mut B, ) -> Result<(usize, SocketAddr)>`
- `pub async fn recv_buf_from<B: BufMut>( &self, buf: &mut B, ) -> Result<(usize, SocketAddr)>`
- `pub fn try_recv_buf<B: BufMut>(&self, buf: &mut B) -> Result<usize>`
- `pub async fn recv_buf<B: BufMut>(&self, buf: &mut B) -> Result<usize>`
- `pub async fn send_to<P>(&self, buf: &[u8], target: P) -> Result<usize>where P: AsRef<Path>,`
- `pub async fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr)>`
- `pub fn poll_recv_from( &self, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<SocketAddr>>`
- `pub fn poll_send_to<P>( &self, cx: &mut Context<'_>, buf: &[u8], target: P, ) -> Poll<Result<usize>>where P: AsRef<Path>,`
- `pub fn poll_send(&self, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize>>`
- `pub fn poll_recv( &self, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<()>>`
- `pub fn try_recv_from(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr)>`
- `pub fn try_io<R>( &self, interest: Interest, f: impl FnOnce() -> Result<R>, ) -> Result<R>`
- `pub async fn async_io<R>( &self, interest: Interest, f: impl FnMut() -> Result<R>, ) -> Result<R>`
- `pub fn local_addr(&self) -> Result<SocketAddr>`
- `pub fn peer_addr(&self) -> Result<SocketAddr>`
- `pub fn take_error(&self) -> Result<Option<Error>>`
- `pub fn shutdown(&self, how: Shutdown) -> Result<()>`
- `fn as_fd(&self) -> BorrowedFd<'_>`
- `fn as_raw_fd(&self) -> RawFd`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn try_from(stream: UnixDatagram) -> Result<Self, Self::Error>`

### struct tokio::net::UnixListener

```rust
pub struct UnixListener { /* private fields */ }
```

A Unix socket which can accept connections from other Unix sockets.

You can accept a new connection by using the `accept` method.

A `UnixListener` can be turned into a `Stream` with `UnixListenerStream`.

**Errors**

Note that accepting a connection can lead to various errors and not all
of them are necessarily fatal ‒ for example having too many open file
descriptors or the other side closing the connection while it waits in
an accept queue. These would terminate the stream if not handled in any
way.

**Examples**

```rust
use tokio::net::UnixListener;

#[tokio::main]
async fn main() {
    let listener = UnixListener::bind("/path/to/the/socket").unwrap();
    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                println!("new client!");
            }
            Err(e) => { /* connection failed */ }
        }
    }
}
```

**Methods**

- `pub fn bind<P>(path: P) -> Result<UnixListener>where P: AsRef<Path>,`
- `pub fn bind_addr(socket_addr: &SocketAddr) -> Result<UnixListener>`
- `pub fn from_std(listener: UnixListener) -> Result<UnixListener>`
- `pub fn into_std(self) -> Result<UnixListener>`
- `pub fn local_addr(&self) -> Result<SocketAddr>`
- `pub fn take_error(&self) -> Result<Option<Error>>`
- `pub async fn accept(&self) -> Result<(UnixStream, SocketAddr)>`
- `pub fn poll_accept( &self, cx: &mut Context<'_>, ) -> Poll<Result<(UnixStream, SocketAddr)>>`
- `fn as_fd(&self) -> BorrowedFd<'_>`
- `fn as_raw_fd(&self) -> RawFd`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn try_from(stream: UnixListener) -> Result<Self>`

### struct tokio::net::UnixSocket

```rust
pub struct UnixSocket { /* private fields */ }
```

A Unix socket that has not yet been converted to a `UnixStream`, `UnixDatagram`, or
`UnixListener`.

`UnixSocket` wraps an operating system socket and enables the caller to
configure the socket before establishing a connection or accepting
inbound connections. The caller is able to set socket option and explicitly
bind the socket with a socket address.

The underlying socket is closed when the `UnixSocket` value is dropped.

`UnixSocket` should only be used directly if the default configuration used
by `UnixStream::connect`, `UnixDatagram::bind`, and `UnixListener::bind`
does not meet the required use case.

Calling `UnixStream::connect(path)` effectively performs the same function as:

```rust
use tokio::net::UnixSocket;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("bind_path");
    let socket = UnixSocket::new_stream()?;

    let stream = socket.connect(path).await?;

    Ok(())
}
```

Calling `UnixDatagram::bind(path)` effectively performs the same function as:

```rust
use tokio::net::UnixSocket;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("bind_path");
    let socket = UnixSocket::new_datagram()?;
    socket.bind(path)?;

    let datagram = socket.datagram()?;

    Ok(())
}
```

Calling `UnixListener::bind(path)` effectively performs the same function as:

...

**Methods**

- `pub fn new_datagram() -> Result<UnixSocket>`
- `pub fn new_stream() -> Result<UnixSocket>`
- `pub fn bind(&self, path: impl AsRef<Path>) -> Result<()>`
- `pub fn listen(self, backlog: u32) -> Result<UnixListener>`
- `pub async fn connect(self, path: impl AsRef<Path>) -> Result<UnixStream>`
- `pub fn datagram(self) -> Result<UnixDatagram>`
- `fn as_fd(&self) -> BorrowedFd<'_>`
- `fn as_raw_fd(&self) -> RawFd`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `unsafe fn from_raw_fd(fd: RawFd) -> UnixSocket`
- `fn into_raw_fd(self) -> RawFd`

### struct tokio::net::UnixStream

```rust
pub struct UnixStream { /* private fields */ }
```

A structure representing a connected Unix socket.

This socket can be connected directly with `UnixStream::connect` or accepted
from a listener with `UnixListener::accept`. Additionally, a pair of
anonymous Unix sockets can be created with `UnixStream::pair`.

To shut down the stream in the write direction, you can call the
`shutdown()` method. This will cause the other peer to receive a read of
length 0, indicating that no more data will be sent. This only closes
the stream in one direction.

**Methods**

- `pub async fn connect<P>(path: P) -> Result<UnixStream>where P: AsRef<Path>,`
- `pub async fn connect_addr(socket_addr: &SocketAddr) -> Result<UnixStream>`
- `pub async fn ready(&self, interest: Interest) -> Result<Ready>`
- `pub async fn readable(&self) -> Result<()>`
- `pub fn poll_read_ready(&self, cx: &mut Context<'_>) -> Poll<Result<()>>`
- `pub fn try_read(&self, buf: &mut [u8]) -> Result<usize>`
- `pub fn try_read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> Result<usize>`
- `pub fn try_read_buf<B: BufMut>(&self, buf: &mut B) -> Result<usize>`
- `pub async fn writable(&self) -> Result<()>`
- `pub fn poll_write_ready(&self, cx: &mut Context<'_>) -> Poll<Result<()>>`
- `pub fn try_write(&self, buf: &[u8]) -> Result<usize>`
- `pub fn try_write_vectored(&self, buf: &[IoSlice<'_>]) -> Result<usize>`
- `pub fn try_io<R>( &self, interest: Interest, f: impl FnOnce() -> Result<R>, ) -> Result<R>`
- `pub async fn async_io<R>( &self, interest: Interest, f: impl FnMut() -> Result<R>, ) -> Result<R>`
- `pub fn from_std(stream: UnixStream) -> Result<UnixStream>`
- `pub fn into_std(self) -> Result<UnixStream>`
- `pub fn pair() -> Result<(UnixStream, UnixStream)>`
- `pub fn local_addr(&self) -> Result<SocketAddr>`
- `pub fn peer_addr(&self) -> Result<SocketAddr>`
- `pub fn peer_cred(&self) -> Result<UCred>`
- `pub fn take_error(&self) -> Result<Option<Error>>`
- `pub fn split<'a>(&'a mut self) -> (ReadHalf<'a>, WriteHalf<'a>)`
- `pub fn into_split(self) -> (OwnedReadHalf, OwnedWriteHalf)`
- `fn as_fd(&self) -> BorrowedFd<'_>`
- `fn as_raw_fd(&self) -> RawFd`
- `fn as_ref(&self) -> &UnixStream`
- `fn as_ref(&self) -> &UnixStream`
- `fn as_ref(&self) -> &UnixStream`
- `fn as_ref(&self) -> &UnixStream`
- `fn as_ref(&self) -> &Self`
- `fn poll_read( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<()>>`
- `fn poll_write( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8], ) -> Poll<Result<usize>>`
- `fn poll_write_vectored( self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &[IoSlice<'_>], ) -> Poll<Result<usize>>`
- `fn is_write_vectored(&self) -> bool`
- `fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>>`
- `fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn try_from(stream: UnixStream) -> Result<Self>`

### trait tokio::net::ToSocketAddrs

```rust
pub trait ToSocketAddrs: ToSocketAddrsPriv { }
```

Converts or resolves without blocking to one or more `SocketAddr` values.

**DNS**

Implementations of `ToSocketAddrs` for string types require a DNS lookup.

**Calling**

Currently, this trait is only used as an argument to Tokio functions that
need to reference a target socket address. To perform a `SocketAddr`
conversion directly, use `lookup_host()`.

This trait is sealed and is intended to be opaque. The details of the trait
will change. Stabilization is pending enhancements to the Rust language.
