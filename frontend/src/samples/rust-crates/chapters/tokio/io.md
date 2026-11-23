# Module tokio::io

## Overview

Traits, helpers, and type definitions for asynchronous I/O functionality.

This module is the asynchronous version of `std::io`. Primarily, it
defines two traits, `AsyncRead` and `AsyncWrite`, which are asynchronous
versions of the `Read` and `Write` traits in the standard library.

**`AsyncRead` and `AsyncWrite`**

Like the standard library’s `Read` and `Write` traits, `AsyncRead` and
`AsyncWrite` provide the most general interface for reading and writing
input and output. Unlike the standard library’s traits, however, they are
asynchronous — meaning that reading from or writing to a `tokio::io`
type will yield to the Tokio scheduler when IO is not ready, rather than
blocking. This allows other tasks to run while waiting on IO.

Another difference is that `AsyncRead` and `AsyncWrite` only contain
core methods needed to provide asynchronous reading and writing
functionality. Instead, utility methods are defined in the `AsyncReadExt`
and `AsyncWriteExt` extension traits. These traits are automatically
implemented for all values that implement `AsyncRead` and `AsyncWrite`
respectively.

End users will rarely interact directly with `AsyncRead` and
`AsyncWrite`. Instead, they will use the async functions defined in the
extension traits. Library authors are expected to implement `AsyncRead`
and `AsyncWrite` in order to provide types that behave like byte streams.

Even with these differences, Tokio’s `AsyncRead` and `AsyncWrite` traits
can be used in almost exactly the same manner as the standard library’s
`Read` and `Write`. Most types in the standard library that implement `Read`
and `Write` have asynchronous equivalents in `tokio` that implement
`AsyncRead` and `AsyncWrite`, such as `File` and `TcpStream`.

For example, the standard library documentation introduces `Read` by
demonstrating reading some bytes from a `std::fs::File`. We
can do the same with `tokio::fs::File`:

```rust
use tokio::io::{self, AsyncReadExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0; 10];

    // read up to 10 bytes
    let n = f.read(&mut buffer).await?;

    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}
```

**Buffered Readers and Writers**

Byte-based interfaces are unwieldy and can be inefficient, as we’d need to be
making near-constant calls to the operating system. To

...

## Items

- `unix` — Asynchronous IO structures specific to Unix-like operating systems.
- `BufReader` — The BufReader struct adds buffering to any reader.
- `BufStream` — Wraps a type that is AsyncWrite and AsyncRead, and buffers its input and output.
- `BufWriter` — Wraps a writer and buffers its output.
- `Chain` — Stream for the chain method.
- `DuplexStream` — A bidirectional pipe to read and write bytes in memory.
- `Empty` — Empty ignores any data written via AsyncWrite, and will always be empty (returning zero bytes) when read via AsyncRead.
- `Interest` — Readiness event interest.
- `Join` — Joins two values implementing AsyncRead and AsyncWrite into a single handle.
- `Lines` — Reads lines from an AsyncBufRead.
- `ReadBuf` — A wrapper around a byte buffer that is incrementally filled and initialized.
- `ReadHalf` — The readable half of a value returned from split.
- `Ready` — Describes the readiness state of an I/O resources.
- `Repeat` — An async reader which yields one byte over and over and over and over and over and…
- `SimplexStream` — A unidirectional pipe to read and write bytes in memory.
- `Sink` — An async writer which will move data into the void.
- `Split` — Splitter for the split method.
- `Take` — Stream for the take method.
- `WriteHalf` — The writable half of a value returned from split.
- `AsyncBufRead` — Reads bytes asynchronously.
- `AsyncBufReadExt` — An extension trait which adds utility methods to AsyncBufRead types.
- `AsyncRead` — Reads bytes from a source.
- `AsyncReadExt` — Reads bytes from a source.
- `AsyncSeek` — Seek bytes asynchronously.
- `AsyncSeekExt` — An extension trait that adds utility methods to AsyncSeek types.
- `AsyncWrite` — Writes bytes asynchronously.
- `AsyncWriteExt` — Writes bytes to a sink.
- `copy` — Asynchronously copies the entire contents of a reader into a writer.
- `copy_bidirectional` — Copies data in both directions between a and b.
- `copy_bidirectional_with_sizes` — Copies data in both directions between a and b using buffers of the specified size.
- `copy_buf` — Asynchronously copies the entire contents of a reader into a writer.
- `duplex` — Create a new pair of DuplexStreams that act like a pair of connected sockets.
- `empty` — Creates a value that is always at EOF for reads, and ignores all data written.
- `join` — Join two values implementing AsyncRead and AsyncWrite into a single handle.
- `repeat` — Creates an instance of an async reader that infinitely repeats one byte.
- `simplex` — Creates unidirectional buffer that acts like in memory pipe.
- `sink` — Creates an instance of an async writer which will successfully consume all data.
- `split` — Splits a single value implementing AsyncRead + AsyncWrite into separate AsyncRead and AsyncWrite handles.

### fn tokio::io::copy

```rust
pub async fn copy<'a, R, W>(reader: &'a mut R, writer: &'a mut W) -> Result<u64>where
    R: AsyncRead + Unpin + ?Sized,
    W: AsyncWrite + Unpin + ?Sized,
```

Asynchronously copies the entire contents of a reader into a writer.

This function returns a future that will continuously read data from
`reader` and then write it into `writer` in a streaming fashion until
`reader` returns EOF or fails.

On success, the total number of bytes that were copied from `reader` to
`writer` is returned.

This is an asynchronous version of `std::io::copy`.

A heap-allocated copy buffer with 8 KB is created to take data from the
reader to the writer, check `copy_buf` if you want an alternative for
`AsyncBufRead`. You can use `copy_buf` with `BufReader` to change the
buffer capacity.

**When to use async alternatives instead of `SyncIoBridge`**

If you are looking to use `std::io::copy` with a synchronous consumer
(like a `hasher` or compressor), consider using async alternatives instead of
wrapping the reader with `SyncIoBridge`.
See the `SyncIoBridge` documentation for detailed examples and guidance.

**Errors**

The returned future will return an error immediately if any call to
`poll_read` or `poll_write` returns an error other than
`io::ErrorKind::Interrupted`, which is retried. Errors from flushing
the writer are returned without retrying.

**Examples**

```rust
use tokio::io;

let mut reader: &[u8] = b"hello";
let mut writer: Vec<u8> = vec![];

io::copy(&mut reader, &mut writer).await?;

assert_eq!(&b"hello"[..], &writer[..]);
```

### fn tokio::io::copy_bidirectional

```rust
pub async fn copy_bidirectional<A, B>(
    a: &mut A,
    b: &mut B,
) -> Result<(u64, u64)>where
    A: AsyncRead + AsyncWrite + Unpin + ?Sized,
    B: AsyncRead + AsyncWrite + Unpin + ?Sized,
```

Copies data in both directions between `a` and `b`.

This function returns a future that will read from both streams,
writing any data read to the opposing stream.
This happens in both directions concurrently.

If an EOF is observed on one stream, `shutdown()` will be invoked on
the other, and reading from that stream will stop. Copying of data in
the other direction will continue.

The future will complete successfully once both directions of communication has been shut down.
A direction is shut down when the reader reports EOF,
at which point `shutdown()` is called on the corresponding writer. When finished,
it will return a tuple of the number of bytes copied from a to b
and the number of bytes copied from b to a, in that order.

It uses two 8 KB buffers for transferring bytes between `a` and `b` by default.
To set your own buffers sizes use `copy_bidirectional_with_sizes()`.

**Errors**

Interrupted reads and writes are retried. The future will immediately
return any other read or write error, or any flush or shutdown error. Some
data read from either stream may be lost (not written to the other stream)
in this case.

**Return value**

Returns a tuple of bytes copied `a` to `b` and bytes copied `b` to `a`.

### fn tokio::io::copy_bidirectional_with_sizes

```rust
pub async fn copy_bidirectional_with_sizes<A, B>(
    a: &mut A,
    b: &mut B,
    a_to_b_buf_size: usize,
    b_to_a_buf_size: usize,
) -> Result<(u64, u64)>where
    A: AsyncRead + AsyncWrite + Unpin + ?Sized,
    B: AsyncRead + AsyncWrite + Unpin + ?Sized,
```

Copies data in both directions between `a` and `b` using buffers of the specified size.

This method is the same as the `copy_bidirectional()`, except that it allows you to set the
size of the internal buffers used when copying data.

**Panics**

Panics if either buffer size is zero.

### fn tokio::io::copy_buf

```rust
pub async fn copy_buf<'a, R, W>(
    reader: &'a mut R,
    writer: &'a mut W,
) -> Result<u64>where
    R: AsyncBufRead + Unpin + ?Sized,
    W: AsyncWrite + Unpin + ?Sized,
```

Asynchronously copies the entire contents of a reader into a writer.

This function returns a future that will continuously read data from
`reader` and then write it into `writer` in a streaming fashion until
`reader` returns EOF or fails.

On success, the total number of bytes that were copied from `reader` to
`writer` is returned.

This is a `tokio::io::copy` alternative for `AsyncBufRead` readers
with no extra buffer allocation, since `AsyncBufRead` allow access
to the reader’s inner buffer.

**When to use async alternatives instead of `SyncIoBridge`**

If you are looking to use `std::io::copy` with a synchronous consumer
(like a `hasher` or compressor), consider using async alternatives instead of
wrapping the reader with `SyncIoBridge`. See the `SyncIoBridge`
documentation for detailed examples and guidance on hashing, compression,
and data parsing.

**Errors**

Errors from `poll_fill_buf` and `poll_write` are returned immediately,
except `io::ErrorKind::Interrupted`, which is retried. Errors from
flushing the writer are returned without retrying.

**Examples**

```rust
use tokio::io;

let mut reader: &[u8] = b"hello";
let mut writer: Vec<u8> = vec![];

io::copy_buf(&mut reader, &mut writer).await?;

assert_eq!(b"hello", &writer[..]);
```

### fn tokio::io::duplex

```rust
pub fn duplex(max_buf_size: usize) -> (DuplexStream, DuplexStream)
```

Create a new pair of `DuplexStream`s that act like a pair of connected sockets.

The `max_buf_size` argument is the maximum amount of bytes that can be
written to a side before the write returns `Poll::Pending`.

**Panics**

This function panics if `max_buf_size` is 0.

### fn tokio::io::empty

```rust
pub fn empty() -> Empty
```

Creates a value that is always at EOF for reads, and ignores all data written.

All writes on the returned instance will return `Poll::Ready(Ok(buf.len()))`
and the contents of the buffer will not be inspected.

All reads from the returned instance will return `Poll::Ready(Ok(0))`.

This is an asynchronous version of `std::io::empty`.

**Examples**

A slightly sad example of not reading anything into a buffer:

```rust
use tokio::io::{self, AsyncReadExt};

    let mut buffer = String::new();
    io::empty().read_to_string(&mut buffer).await.unwrap();
    assert!(buffer.is_empty());
```

A convoluted way of getting the length of a buffer:

```rust
use tokio::io::{self, AsyncWriteExt};

let buffer = vec![1, 2, 3, 5, 8];
let num_bytes = io::empty().write(&buffer).await.unwrap();
assert_eq!(num_bytes, 5);
```

### fn tokio::io::join

```rust
pub fn join<R, W>(reader: R, writer: W) -> Join<R, W>where
    R: AsyncRead,
    W: AsyncWrite,
```

Join two values implementing `AsyncRead` and `AsyncWrite` into a
single handle.

### fn tokio::io::repeat

```rust
pub fn repeat(byte: u8) -> Repeat
```

Creates an instance of an async reader that infinitely repeats one byte.

All reads from this reader will succeed by filling the specified buffer with
the given byte.

This is an asynchronous version of `std::io::repeat`.

**Examples**

```rust
use tokio::io::{self, AsyncReadExt};

let mut buffer = [0; 3];
io::repeat(0b101).read_exact(&mut buffer).await.unwrap();
assert_eq!(buffer, [0b101, 0b101, 0b101]);
```

### fn tokio::io::simplex

```rust
pub fn simplex(
    max_buf_size: usize,
) -> (ReadHalf<SimplexStream>, WriteHalf<SimplexStream>)
```

Creates unidirectional buffer that acts like in memory pipe.

The `max_buf_size` argument is the maximum amount of bytes that can be
written to a buffer before it returns `Poll::Pending`.

**Unify reader and writer**

The reader and writer half can be unified into a single structure
of `SimplexStream` that supports both reading and writing or
the `SimplexStream` can be already created as unified structure
using `SimplexStream::new_unsplit()`.

```rust
let (reader, writer) = tokio::io::simplex(64);
let mut simplex_stream = reader.unsplit(writer);
simplex_stream.write_all(b"hello").await?;

let mut buf = [0u8; 5];
simplex_stream.read_exact(&mut buf).await?;
assert_eq!(&buf, b"hello");
```

**Panics**

This function panics if `max_buf_size` is 0.

### fn tokio::io::sink

```rust
pub fn sink() -> Sink
```

Creates an instance of an async writer which will successfully consume all
data.

All calls to `poll_write` on the returned instance will return
`Poll::Ready(Ok(buf.len()))` and the contents of the buffer will not be
inspected.

This is an asynchronous version of `std::io::sink`.

**Examples**

```rust
use tokio::io::{self, AsyncWriteExt};

let buffer = vec![1, 2, 3, 5, 8];
let num_bytes = io::sink().write(&buffer).await?;
assert_eq!(num_bytes, 5);
Ok(())
```

### fn tokio::io::split

```rust
pub fn split<T>(stream: T) -> (ReadHalf<T>, WriteHalf<T>)where
    T: AsyncRead + AsyncWrite,
```

Splits a single value implementing `AsyncRead + AsyncWrite` into separate
`AsyncRead` and `AsyncWrite` handles.

To restore this read/write object from its `ReadHalf` and
`WriteHalf` use `unsplit`.

### struct tokio::io::BufReader

```rust
pub struct BufReader<R> { /* private fields */ }
```

The `BufReader` struct adds buffering to any reader.

It can be excessively inefficient to work directly with a `AsyncRead`
instance. A `BufReader` performs large, infrequent reads on the underlying
`AsyncRead` and maintains an in-memory buffer of the results.

`BufReader` can improve the speed of programs that make small and
repeated read calls to the same file or network socket. It does not
help when reading very large amounts at once, or reading just one or a few
times. It also provides no advantage when reading from a source that is
already in memory, like a `Vec<u8>`.

When the `BufReader` is dropped, the contents of its buffer will be
discarded. Creating multiple instances of a `BufReader` on the same
stream can cause data loss.

**Methods**

- `pub fn new(inner: R) -> Self`
- `pub fn with_capacity(capacity: usize, inner: R) -> Self`
- `pub fn get_ref(&self) -> &R`
- `pub fn get_mut(&mut self) -> &mut R`
- `pub fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut R>`
- `pub fn into_inner(self) -> R`
- `pub fn buffer(&self) -> &[u8]`
- `fn poll_fill_buf( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<&[u8]>>`
- `fn consume(self: Pin<&mut Self>, amt: usize)`
- `fn poll_read( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<()>>`
- `fn start_seek(self: Pin<&mut Self>, pos: SeekFrom) -> Result<()>`
- `fn poll_complete( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<u64>>`
- `fn poll_write( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8], ) -> Poll<Result<usize>>`
- `fn poll_write_vectored( self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &[IoSlice<'_>], ) -> Poll<Result<usize>>`
- `fn is_write_vectored(&self) -> bool`
- `fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>>`
- `fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn from(b: BufReader<BufWriter<RW>>) -> Self`

### struct tokio::io::BufStream

```rust
pub struct BufStream<RW> { /* private fields */ }
```

Wraps a type that is `AsyncWrite` and `AsyncRead`, and buffers its input and output.

It can be excessively inefficient to work directly with something that implements `AsyncWrite`
and `AsyncRead`. For example, every `write`, however small, has to traverse the syscall
interface, and similarly, every read has to do the same. The `BufWriter` and `BufReader`
types aid with these problems respectively, but do so in only one direction. `BufStream` wraps
one in the other so that both directions are buffered. See their documentation for details.

**Methods**

- `pub fn new(stream: RW) -> BufStream<RW>`
- `pub fn with_capacity( reader_capacity: usize, writer_capacity: usize, stream: RW, ) -> BufStream<RW>`
- `pub fn get_ref(&self) -> &RW`
- `pub fn get_mut(&mut self) -> &mut RW`
- `pub fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut RW>`
- `pub fn into_inner(self) -> RW`
- `fn poll_fill_buf( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<&[u8]>>`
- `fn consume(self: Pin<&mut Self>, amt: usize)`
- `fn poll_read( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<()>>`
- `fn start_seek(self: Pin<&mut Self>, position: SeekFrom) -> Result<()>`
- `fn poll_complete( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<u64>>`
- `fn poll_write( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8], ) -> Poll<Result<usize>>`
- `fn poll_write_vectored( self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &[IoSlice<'_>], ) -> Poll<Result<usize>>`
- `fn is_write_vectored(&self) -> bool`
- `fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>>`
- `fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn from(b: BufReader<BufWriter<RW>>) -> Self`
- `fn from(b: BufWriter<BufReader<RW>>) -> Self`

### struct tokio::io::BufWriter

```rust
pub struct BufWriter<W> { /* private fields */ }
```

Wraps a writer and buffers its output.

It can be excessively inefficient to work directly with something that
implements `AsyncWrite`. A `BufWriter` keeps an in-memory buffer of data and
writes it to an underlying writer in large, infrequent batches.

`BufWriter` can improve the speed of programs that make small and
repeated write calls to the same file or network socket. It does not
help when writing very large amounts at once, or writing just one or a few
times. It also provides no advantage when writing to a destination that is
in memory, like a `Vec<u8>`.

When the `BufWriter` is dropped, the contents of its buffer will be
discarded. Creating multiple instances of a `BufWriter` on the same
stream can cause data loss. If you need to write out the contents of its
buffer, you must manually call flush before the writer is dropped.

**Methods**

- `pub fn new(inner: W) -> Self`
- `pub fn with_capacity(cap: usize, inner: W) -> Self`
- `pub fn get_ref(&self) -> &W`
- `pub fn get_mut(&mut self) -> &mut W`
- `pub fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut W>`
- `pub fn into_inner(self) -> W`
- `pub fn buffer(&self) -> &[u8]`
- `fn poll_fill_buf( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<&[u8]>>`
- `fn consume(self: Pin<&mut Self>, amt: usize)`
- `fn poll_read( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<()>>`
- `fn start_seek(self: Pin<&mut Self>, pos: SeekFrom) -> Result<()>`
- `fn poll_complete( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<u64>>`
- `fn poll_write( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8], ) -> Poll<Result<usize>>`
- `fn poll_write_vectored( self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &[IoSlice<'_>], ) -> Poll<Result<usize>>`
- `fn is_write_vectored(&self) -> bool`
- `fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>>`
- `fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn from(b: BufWriter<BufReader<RW>>) -> Self`

### struct tokio::io::Chain

```rust
pub struct Chain<T, U> { /* private fields */ }
```

Stream for the `chain` method.

**Methods**

- `pub fn get_ref(&self) -> (&T, &U)`
- `pub fn get_mut(&mut self) -> (&mut T, &mut U)`
- `pub fn get_pin_mut(self: Pin<&mut Self>) -> (Pin<&mut T>, Pin<&mut U>)`
- `pub fn into_inner(self) -> (T, U)`
- `fn poll_fill_buf( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<&[u8]>>`
- `fn consume(self: Pin<&mut Self>, amt: usize)`
- `fn poll_read( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<()>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tokio::io::DuplexStream

```rust
pub struct DuplexStream { /* private fields */ }
```

A bidirectional pipe to read and write bytes in memory.

A pair of `DuplexStream`s are created together, and they act as a “channel”
that can be used as in-memory IO types. Writing to one of the pairs will
allow that data to be read from the other, and vice versa.

**Closing a `DuplexStream`**

If one end of the `DuplexStream` channel is dropped, any pending reads on
the other side will continue to read data until the buffer is drained, then
they will signal EOF by returning 0 bytes. Any writes to the other side,
including pending ones (that are waiting for free space in the buffer) will
return `Err(BrokenPipe)` immediately.

**Example**

```rust
let (mut client, mut server) = tokio::io::duplex(64);

client.write_all(b"ping").await?;

let mut buf = [0u8; 4];
server.read_exact(&mut buf).await?;
assert_eq!(&buf, b"ping");

server.write_all(b"pong").await?;

client.read_exact(&mut buf).await?;
assert_eq!(&buf, b"pong");
```

**Methods**

- `fn poll_read( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<()>>`
- `fn poll_write( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8], ) -> Poll<Result<usize>>`
- `fn poll_write_vectored( self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &[IoSlice<'_>], ) -> Poll<Result<usize, Error>>`
- `fn is_write_vectored(&self) -> bool`
- `fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>>`
- `fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`

### struct tokio::io::Empty

```rust
pub struct Empty { /* private fields */ }
```

`Empty` ignores any data written via `AsyncWrite`, and will always be empty
(returning zero bytes) when read via `AsyncRead`.

This struct is generally created by calling `empty`. Please see
the documentation of `empty()` for more details.

This is an asynchronous version of `std::io::empty`.

**Methods**

- `fn poll_fill_buf( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<&[u8]>>`
- `fn consume(self: Pin<&mut Self>, _: usize)`
- `fn poll_read( self: Pin<&mut Self>, cx: &mut Context<'_>, _: &mut ReadBuf<'_>, ) -> Poll<Result<()>>`
- `fn start_seek(self: Pin<&mut Self>, _position: SeekFrom) -> Result<()>`
- `fn poll_complete( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<u64>>`
- `fn poll_write( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8], ) -> Poll<Result<usize>>`
- `fn poll_flush( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<(), Error>>`
- `fn poll_shutdown( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<(), Error>>`
- `fn is_write_vectored(&self) -> bool`
- `fn poll_write_vectored( self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &[IoSlice<'_>], ) -> Poll<Result<usize, Error>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tokio::io::Interest

```rust
pub struct Interest(/* private fields */);
```

Readiness event interest.

Specifies the readiness events the caller is interested in when awaiting on
I/O resource readiness states.

**Methods**

- `pub const fn is_readable(self) -> bool`
- `pub const fn is_writable(self) -> bool`
- `pub const fn is_error(self) -> bool`
- `pub const fn is_priority(self) -> bool`
- `pub const fn add(self, other: Interest) -> Interest`
- `pub fn remove(self, other: Interest) -> Option<Interest>`
- `fn bitor(self, other: Self) -> Self`
- `fn bitor_assign(&mut self, other: Self)`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### struct tokio::io::Join

```rust
pub struct Join<R, W> { /* private fields */ }
```

Joins two values implementing `AsyncRead` and `AsyncWrite` into a
single handle.

**Methods**

- `pub fn into_inner(self) -> (R, W)`
- `pub fn reader(&self) -> &R`
- `pub fn writer(&self) -> &W`
- `pub fn reader_mut(&mut self) -> &mut R`
- `pub fn writer_mut(&mut self) -> &mut W`
- `pub fn reader_pin_mut(self: Pin<&mut Self>) -> Pin<&mut R>`
- `pub fn writer_pin_mut(self: Pin<&mut Self>) -> Pin<&mut W>`
- `fn poll_fill_buf( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<&[u8]>>`
- `fn consume(self: Pin<&mut Self>, amt: usize)`
- `fn poll_read( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<(), Error>>`
- `fn poll_write( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8], ) -> Poll<Result<usize, Error>>`
- `fn poll_flush( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<(), Error>>`
- `fn poll_shutdown( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<(), Error>>`
- `fn poll_write_vectored( self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &[IoSlice<'_>], ) -> Poll<Result<usize, Error>>`
- `fn is_write_vectored(&self) -> bool`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tokio::io::Lines

```rust
pub struct Lines<R> { /* private fields */ }
```

Reads lines from an `AsyncBufRead`.

A `Lines` can be turned into a `Stream` with `LinesStream`.

This type is usually created using the `lines` method.

**Methods**

- `pub async fn next_line(&mut self) -> Result<Option<String>>`
- `pub fn get_mut(&mut self) -> &mut R`
- `pub fn get_ref(&mut self) -> &R`
- `pub fn into_inner(self) -> R`
- `pub fn poll_next_line( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<Option<String>>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tokio::io::ReadBuf

```rust
pub struct ReadBuf<'a> { /* private fields */ }
```

A wrapper around a byte buffer that is incrementally filled and initialized.

This type is a sort of “double cursor”. It tracks three regions in the
buffer: a region at the beginning of the buffer that has been logically
filled with data, a region that has been initialized at some point but not
yet logically filled, and a region at the end that may be uninitialized.
The filled region is guaranteed to be a subset of the initialized region.

In summary, the contents of the buffer can be visualized as:

```rust
[             capacity              ]
[ filled |         unfilled         ]
[    initialized    | uninitialized ]
```

It is undefined behavior to de-initialize any bytes from the uninitialized
region, since it is merely unknown whether this region is uninitialized or
not, and if part of it turns out to be initialized, it must stay initialized.

**Methods**

- `pub fn new(buf: &'a mut [u8]) -> ReadBuf<'a>`
- `pub fn uninit(buf: &'a mut [MaybeUninit<u8>]) -> ReadBuf<'a>`
- `pub fn capacity(&self) -> usize`
- `pub fn filled(&self) -> &[u8]`
- `pub fn filled_mut(&mut self) -> &mut [u8]`
- `pub fn take(&mut self, n: usize) -> ReadBuf<'_>`
- `pub fn initialized(&self) -> &[u8]`
- `pub fn initialized_mut(&mut self) -> &mut [u8]`
- `pub unsafe fn inner_mut(&mut self) -> &mut [MaybeUninit<u8>]`
- `pub unsafe fn unfilled_mut(&mut self) -> &mut [MaybeUninit<u8>]`
- `pub fn initialize_unfilled(&mut self) -> &mut [u8]`
- `pub fn initialize_unfilled_to(&mut self, n: usize) -> &mut [u8]`
- `pub fn remaining(&self) -> usize`
- `pub fn clear(&mut self)`
- `pub fn advance(&mut self, n: usize)`
- `pub fn set_filled(&mut self, n: usize)`
- `pub unsafe fn assume_init(&mut self, n: usize)`
- `pub fn put_slice(&mut self, buf: &[u8])`
- `fn remaining_mut(&self) -> usize`
- `unsafe fn advance_mut(&mut self, cnt: usize)`
- `fn chunk_mut(&mut self) -> &mut UninitSlice`
- `fn has_remaining_mut(&self) -> bool`
- `fn put<T>(&mut self, src: T)where T: Buf, Self: Sized,`
- `fn put_slice(&mut self, src: &[u8])`
- `fn put_bytes(&mut self, val: u8, cnt: usize)`
- `fn put_u8(&mut self, n: u8)`
- `fn put_i8(&mut self, n: i8)`
- `fn put_u16(&mut self, n: u16)`
- `fn put_u16_le(&mut self, n: u16)`
- `fn put_u16_ne(&mut self, n: u16)`
- `fn put_i16(&mut self, n: i16)`
- `fn put_i16_le(&mut self, n: i16)`
- `fn put_i16_ne(&mut self, n: i16)`
- `fn put_u32(&mut self, n: u32)`
- `fn put_u32_le(&mut self, n: u32)`
- `fn put_u32_ne(&mut self, n: u32)`
- `fn put_i32(&mut self, n: i32)`
- `fn put_i32_le(&mut self, n: i32)`
- `fn put_i32_ne(&mut self, n: i32)`
- `fn put_u64(&mut self, n: u64)`

### struct tokio::io::ReadHalf

```rust
pub struct ReadHalf<T> { /* private fields */ }
```

The readable half of a value returned from `split`.

**Methods**

- `pub fn is_pair_of(&self, other: &WriteHalf<T>) -> bool`
- `pub fn unsplit(self, wr: WriteHalf<T>) -> Twhere T: Unpin,`
- `fn poll_read( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<()>>`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`

### struct tokio::io::Ready

```rust
pub struct Ready(/* private fields */);
```

Describes the readiness state of an I/O resources.

`Ready` tracks which operation an I/O resource is ready to perform.

**Methods**

- `pub fn is_empty(self) -> bool`
- `pub fn is_readable(self) -> bool`
- `pub fn is_writable(self) -> bool`
- `pub fn is_read_closed(self) -> bool`
- `pub fn is_write_closed(self) -> bool`
- `pub fn is_priority(self) -> bool`
- `pub fn is_error(self) -> bool`
- `fn bitand(self, other: Ready) -> Ready`
- `fn bitor(self, other: Ready) -> Ready`
- `fn bitor_assign(&mut self, other: Ready)`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `fn cmp(&self, other: &Self) -> Ordering`
- `1.21.0 (const: unstable) · fn max(self, other: Self) -> Selfwhere Self: Sized,`
- `1.21.0 (const: unstable) · fn min(self, other: Self) -> Selfwhere Self: Sized,`
- `1.50.0 (const: unstable) · fn clamp(self, min: Self, max: Self) -> Selfwhere Self: Sized,`
- `fn clamp_to<R>(self, range: R) -> Selfwhere Self: Sized, R: ClampBounds<Self>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn partial_cmp(&self, other: &Self) -> Option<Ordering>`
- `1.0.0 (const: unstable) · fn lt(&self, other: &Rhs) -> bool`
- `1.0.0 (const: unstable) · fn le(&self, other: &Rhs) -> bool`
- `1.0.0 (const: unstable) · fn gt(&self, other: &Rhs) -> bool`
- `1.0.0 (const: unstable) · fn ge(&self, other: &Rhs) -> bool`
- `fn sub(self, other: Ready) -> Ready`

### struct tokio::io::Repeat

```rust
pub struct Repeat { /* private fields */ }
```

An async reader which yields one byte over and over and over and over and
over and…

This struct is generally created by calling `repeat`. Please
see the documentation of `repeat()` for more details.

This is an asynchronous version of `std::io::Repeat`.

**Methods**

- `fn poll_read( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<()>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tokio::io::SimplexStream

```rust
pub struct SimplexStream { /* private fields */ }
```

A unidirectional pipe to read and write bytes in memory.

It can be constructed by `simplex` function which will create a pair of
reader and writer or by calling `SimplexStream::new_unsplit` that will
create a handle for both reading and writing.

**Example**

```rust
let (mut receiver, mut sender) = tokio::io::simplex(64);

sender.write_all(b"ping").await?;

let mut buf = [0u8; 4];
receiver.read_exact(&mut buf).await?;
assert_eq!(&buf, b"ping");
```

**Methods**

- `pub fn new_unsplit(max_buf_size: usize) -> SimplexStream`
- `fn poll_read( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<()>>`
- `fn poll_write( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8], ) -> Poll<Result<usize>>`
- `fn poll_write_vectored( self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &[IoSlice<'_>], ) -> Poll<Result<usize, Error>>`
- `fn is_write_vectored(&self) -> bool`
- `fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>>`
- `fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tokio::io::Sink

```rust
pub struct Sink { /* private fields */ }
```

An async writer which will move data into the void.

This struct is generally created by calling `sink`. Please
see the documentation of `sink()` for more details.

This is an asynchronous version of `std::io::Sink`.

**Methods**

- `fn poll_write( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8], ) -> Poll<Result<usize, Error>>`
- `fn is_write_vectored(&self) -> bool`
- `fn poll_write_vectored( self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &[IoSlice<'_>], ) -> Poll<Result<usize, Error>>`
- `fn poll_flush( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<(), Error>>`
- `fn poll_shutdown( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<(), Error>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tokio::io::Split

```rust
pub struct Split<R> { /* private fields */ }
```

Splitter for the `split` method.

A `Split` can be turned into a `Stream` with `SplitStream`.

**Methods**

- `pub async fn next_segment(&mut self) -> Result<Option<Vec<u8>>>`
- `pub fn poll_next_segment( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<Option<Vec<u8>>>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tokio::io::Take

```rust
pub struct Take<R> { /* private fields */ }
```

Stream for the `take` method.

**Methods**

- `pub fn limit(&self) -> u64`
- `pub fn set_limit(&mut self, limit: u64)`
- `pub fn get_ref(&self) -> &R`
- `pub fn get_mut(&mut self) -> &mut R`
- `pub fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut R>`
- `pub fn into_inner(self) -> R`
- `fn poll_fill_buf( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<&[u8]>>`
- `fn consume(self: Pin<&mut Self>, amt: usize)`
- `fn poll_read( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<(), Error>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tokio::io::WriteHalf

```rust
pub struct WriteHalf<T> { /* private fields */ }
```

The writable half of a value returned from `split`.

**Methods**

- `pub fn is_pair_of(&self, other: &ReadHalf<T>) -> bool`
- `fn poll_write( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8], ) -> Poll<Result<usize, Error>>`
- `fn poll_flush( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<(), Error>>`
- `fn poll_shutdown( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<(), Error>>`
- `fn poll_write_vectored( self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &[IoSlice<'_>], ) -> Poll<Result<usize, Error>>`
- `fn is_write_vectored(&self) -> bool`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`

### trait tokio::io::AsyncBufRead

```rust
pub trait AsyncBufRead: AsyncRead {
    // Required methods
    fn poll_fill_buf(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<&[u8]>>;
    fn consume(self: Pin<&mut Self>, amt: usize);
}
```

Reads bytes asynchronously.

This trait is analogous to `std::io::BufRead`, but integrates with
the asynchronous task system. In particular, the `poll_fill_buf` method,
unlike `BufRead::fill_buf`, will automatically queue the current task for wakeup
and return if data is not yet available, rather than blocking the calling
thread.

Utilities for working with `AsyncBufRead` values are provided by
`AsyncBufReadExt`.

**Methods**

- `fn poll_fill_buf( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<&[u8]>>`
- `fn consume(self: Pin<&mut Self>, amt: usize)`
- `fn poll_fill_buf( self: Pin<&mut Self>, _cx: &mut Context<'_>, ) -> Poll<Result<&[u8]>>`
- `fn consume(self: Pin<&mut Self>, amt: usize)`
- `fn poll_fill_buf( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<&[u8]>>`
- `fn consume(self: Pin<&mut Self>, amt: usize)`
- `fn poll_fill_buf( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<&[u8]>>`
- `fn consume(self: Pin<&mut Self>, amt: usize)`
- `fn poll_fill_buf( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<&[u8]>>`
- `fn consume(self: Pin<&mut Self>, amt: usize)`
- `fn poll_fill_buf( self: Pin<&mut Self>, _cx: &mut Context<'_>, ) -> Poll<Result<&[u8]>>`
- `fn consume(self: Pin<&mut Self>, amt: usize)`

### trait tokio::io::AsyncBufReadExt

```rust
pub trait AsyncBufReadExt: AsyncBufRead {
    // Provided methods
    fn read_until<'a>(
        &'a mut self,
        byte: u8,
        buf: &'a mut Vec<u8>,
    ) -> ReadUntil<'a, Self>
       where Self: Unpin { ... }
    fn read_line<'a>(&'a mut self, buf: &'a mut String) -> ReadLine<'a, Self>
       where Self: Unpin { ... }
    fn split(self, byte: u8) -> Split<Self>
       where Self: Sized + Unpin { ... }
    fn fill_buf(&mut self) -> FillBuf<'_, Self>
       where Self: Unpin { ... }
    fn consume(&mut self, amt: usize)
       where Self: Unpin { ... }
    fn lines(self) -> Lines<Self>
       where Self: Sized { ... }
}
```

An extension trait which adds utility methods to `AsyncBufRead` types.

**Methods**

- `fn read_until<'a>( &'a mut self, byte: u8, buf: &'a mut Vec<u8>, ) -> ReadUntil<'a, Self>where Self: Unpin,`
- `fn read_line<'a>(&'a mut self, buf: &'a mut String) -> ReadLine<'a, Self>where Self: Unpin,`
- `fn split(self, byte: u8) -> Split<Self>where Self: Sized + Unpin,`
- `fn fill_buf(&mut self) -> FillBuf<'_, Self>where Self: Unpin,`
- `fn consume(&mut self, amt: usize)where Self: Unpin,`
- `fn lines(self) -> Lines<Self>where Self: Sized,`

### trait tokio::io::AsyncRead

```rust
pub trait AsyncRead {
    // Required method
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<()>>;
}
```

Reads bytes from a source.

This trait is analogous to the `std::io::Read` trait, but integrates with
the asynchronous task system. In particular, the `poll_read` method,
unlike `Read::read`, will automatically queue the current task for wakeup
and return if data is not yet available, rather than blocking the calling
thread.

Specifically, this means that the `poll_read` function will return one of
the following:

- 

`Poll::Ready(Ok(()))` means that data was immediately read and placed into
the output buffer. The amount of data read can be determined by the
increase in the length of the slice returned by `ReadBuf::filled`. If the
difference is 0, either EOF has been reached, or the output buffer had zero
capacity (i.e. `buf.remaining()` == 0).

- 

`Poll::Pending` means that no data was read into the buffer
provided. The I/O object is not currently readable but may become readable
in the future. Most importantly, the current future’s task is scheduled
to get unparked when the object is readable. This means that like
`Future::poll` you’ll receive a notification when the I/O object is
readable again.

- 

`Poll::Ready(Err(e))` for other errors are standard I/O errors coming from the
underlying object.

This trait importantly means that the `read` method only works in the
context of a future’s task. The object may panic if used outside of a task.

Utilities for working with `AsyncRead` values are provided by
`AsyncReadExt`.

**Methods**

- `fn poll_read( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<()>>`
- `fn poll_read( self: Pin<&mut Self>, _cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<()>>`
- `fn poll_read( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<()>>`
- `fn poll_read( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<()>>`
- `fn poll_read( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<()>>`
- `fn poll_read( self: Pin<&mut Self>, _cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<()>>`

### trait tokio::io::AsyncReadExt

```rust
pub trait AsyncReadExt: AsyncRead {
    // Provided methods
    fn chain<R>(self, next: R) -> Chain<Self, R>
       where Self: Sized,
             R: AsyncRead { ... }
    fn read<'a>(&'a mut self, buf: &'a mut [u8]) -> Read<'a, Self>
       where Self: Unpin { ... }
    fn read_buf<'a, B>(&'a mut self, buf: &'a mut B) -> ReadBuf<'a, Self, B>
       where Self: Unpin,
             B: BufMut + ?Sized { ... }
    fn read_exact<'a>(&'a mut self, buf: &'a mut [u8]) -> ReadExact<'a, Self>
       where Self: Unpin { ... }
    fn read_u8(&mut self) -> ReadU8<&mut Self>
       where Self: Unpin { ... }
    fn read_i8(&mut self) -> ReadI8<&mut Self>
       where Self: Unpin { ... }
    fn read_u16(&mut self) -> ReadU16<&mut Self>
       where Self: Unpin { ... }
    fn read_i16(&mut self) -> ReadI16<&mut Self>
       where Self: Unpin { ... }
    fn read_u32(&mut self) -> ReadU32<&mut Self>
       where Self: Unpin { ... }
    fn read_i32(&mut self) -> ReadI32<&mut Self>
       where Self: Unpin { ... }
    fn read_u64(&mut self) -> ReadU64<&mut Self>
       where Self: Unpin { ... }
    fn read_i64(&mut self) -> ReadI64<&mut Self>
       where Self: Unpin { ... }
    fn read_u128(&mut self) -> ReadU128<&mut Self>
       where Self: Unpin { ... }
    fn read_i128(&mut self) -> ReadI128<&mut Self>
       where Self: Unpin { ... }
    fn read_f32(&mut self) -> ReadF32<&mut Self>
       where Self: Unpin { ... }
    fn read_f64(&mut self) -> ReadF64<&mut Self>
       where Self: Unpin { ... }
    fn read_u16_le(&mut self) -> ReadU16Le<&mut Self>
       where Self: Unpin { ... }
    fn read_i16_le(&mut self) -> ReadI16Le<&mut Self>
       where Self: Unpin { ... }
    fn read_u32_le(&mut self) -> ReadU32Le<&mut Self>
       where Self: Unpin { ... }
    fn read_i32_le(&mut self) -> ReadI32Le<&mut Self>
       where Self: Unpin { ... }
    fn read_u64_le(&mut self) -> ReadU64Le<&mut Self>
       where Self: Unpin { ... }
    fn read_i64_le(&mut self) -> ReadI64Le<&mut Self>
       where Self: Unpin { ... }
    fn read_u128_le(&mut self) -> ReadU128Le<&mut Self>
       where Self: Unpin { ... }
    fn read_i128_le(&mut self) -> ReadI128Le<&mut Self>
       where Self: Unpin { ... }
    fn read_f32_le(&mut self) -> ReadF32Le<&mut Self>
       where Self: Unpin { ... }
    fn read_f64_le(&mut self) -> ReadF64Le<&mut Self>
       where Self: Unpin { ... }
    fn read_to_end<'a>(
        &'a mut self,
        buf: &'a mut Vec<u8>,
    ) -> ReadToEnd<'a, Self>
       where Self: Unpin { ... }
    fn read_to_string<'a>(
        &'a mut self,
        dst: &'a mut String,
    ) -> ReadToString<'a, Self>
       where Self: Unpin { ... }
    fn take(self, limit: u64) -> Take<Self>
       where Self: Sized { ... }
}
```

Reads bytes from a source.

Implemented as an extension trait, adding utility methods to all
`AsyncRead` types. Callers will tend to import this trait instead of
`AsyncRead`.

```rust
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0; 10];

    // The `read` method is defined by this trait.
    let n = f.read(&mut buffer[..]).await?;

    Ok(())
}
```

See module documentation for more details.

**Methods**

- `fn chain<R>(self, next: R) -> Chain<Self, R>where Self: Sized, R: AsyncRead,`
- `fn read<'a>(&'a mut self, buf: &'a mut [u8]) -> Read<'a, Self>where Self: Unpin,`
- `fn read_buf<'a, B>(&'a mut self, buf: &'a mut B) -> ReadBuf<'a, Self, B>where Self: Unpin, B: BufMut + ?Sized,`
- `fn read_exact<'a>(&'a mut self, buf: &'a mut [u8]) -> ReadExact<'a, Self>where Self: Unpin,`
- `fn read_u8(&mut self) -> ReadU8<&mut Self>where Self: Unpin,`
- `fn read_i8(&mut self) -> ReadI8<&mut Self>where Self: Unpin,`
- `fn read_u16(&mut self) -> ReadU16<&mut Self>where Self: Unpin,`
- `fn read_i16(&mut self) -> ReadI16<&mut Self>where Self: Unpin,`
- `fn read_u32(&mut self) -> ReadU32<&mut Self>where Self: Unpin,`
- `fn read_i32(&mut self) -> ReadI32<&mut Self>where Self: Unpin,`
- `fn read_u64(&mut self) -> ReadU64<&mut Self>where Self: Unpin,`
- `fn read_i64(&mut self) -> ReadI64<&mut Self>where Self: Unpin,`
- `fn read_u128(&mut self) -> ReadU128<&mut Self>where Self: Unpin,`
- `fn read_i128(&mut self) -> ReadI128<&mut Self>where Self: Unpin,`
- `fn read_f32(&mut self) -> ReadF32<&mut Self>where Self: Unpin,`
- `fn read_f64(&mut self) -> ReadF64<&mut Self>where Self: Unpin,`
- `fn read_u16_le(&mut self) -> ReadU16Le<&mut Self>where Self: Unpin,`
- `fn read_i16_le(&mut self) -> ReadI16Le<&mut Self>where Self: Unpin,`
- `fn read_u32_le(&mut self) -> ReadU32Le<&mut Self>where Self: Unpin,`
- `fn read_i32_le(&mut self) -> ReadI32Le<&mut Self>where Self: Unpin,`
- `fn read_u64_le(&mut self) -> ReadU64Le<&mut Self>where Self: Unpin,`
- `fn read_i64_le(&mut self) -> ReadI64Le<&mut Self>where Self: Unpin,`
- `fn read_u128_le(&mut self) -> ReadU128Le<&mut Self>where Self: Unpin,`
- `fn read_i128_le(&mut self) -> ReadI128Le<&mut Self>where Self: Unpin,`
- `fn read_f32_le(&mut self) -> ReadF32Le<&mut Self>where Self: Unpin,`
- `fn read_f64_le(&mut self) -> ReadF64Le<&mut Self>where Self: Unpin,`
- `fn read_to_end<'a>(&'a mut self, buf: &'a mut Vec<u8>) -> ReadToEnd<'a, Self>where Self: Unpin,`
- `fn read_to_string<'a>( &'a mut self, dst: &'a mut String, ) -> ReadToString<'a, Self>where Self: Unpin,`
- `fn take(self, limit: u64) -> Take<Self>where Self: Sized,`

### trait tokio::io::AsyncSeek

```rust
pub trait AsyncSeek {
    // Required methods
    fn start_seek(self: Pin<&mut Self>, position: SeekFrom) -> Result<()>;
    fn poll_complete(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<u64>>;
}
```

Seek bytes asynchronously.

This trait is analogous to the `std::io::Seek` trait, but integrates
with the asynchronous task system. In particular, the `start_seek`
method, unlike `Seek::seek`, will not block the calling thread.

Utilities for working with `AsyncSeek` values are provided by
`AsyncSeekExt`.

**Methods**

- `fn start_seek(self: Pin<&mut Self>, position: SeekFrom) -> Result<()>`
- `fn poll_complete( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<u64>>`
- `fn start_seek(self: Pin<&mut Self>, pos: SeekFrom) -> Result<()>`
- `fn poll_complete( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<u64>>`
- `fn start_seek(self: Pin<&mut Self>, pos: SeekFrom) -> Result<()>`
- `fn poll_complete( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<u64>>`
- `fn start_seek(self: Pin<&mut Self>, pos: SeekFrom) -> Result<()>`
- `fn poll_complete( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<u64>>`
- `fn start_seek(self: Pin<&mut Self>, pos: SeekFrom) -> Result<()>`
- `fn poll_complete(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<u64>>`

### trait tokio::io::AsyncSeekExt

```rust
pub trait AsyncSeekExt: AsyncSeek {
    // Provided methods
    fn seek(&mut self, pos: SeekFrom) -> Seek<'_, Self>
       where Self: Unpin { ... }
    fn rewind(&mut self) -> Seek<'_, Self>
       where Self: Unpin { ... }
    fn stream_position(&mut self) -> Seek<'_, Self>
       where Self: Unpin { ... }
}
```

An extension trait that adds utility methods to `AsyncSeek` types.

**Examples**

```rust
use std::io::{self, Cursor, SeekFrom};
use tokio::io::{AsyncSeekExt, AsyncReadExt};

let mut cursor = Cursor::new(b"abcdefg");

// the `seek` method is defined by this trait
cursor.seek(SeekFrom::Start(3)).await?;

let mut buf = [0; 1];
let n = cursor.read(&mut buf).await?;
assert_eq!(n, 1);
assert_eq!(buf, [b'd']);

Ok(())
```

See module documentation for more details.

**Methods**

- `fn seek(&mut self, pos: SeekFrom) -> Seek<'_, Self>where Self: Unpin,`
- `fn rewind(&mut self) -> Seek<'_, Self>where Self: Unpin,`
- `fn stream_position(&mut self) -> Seek<'_, Self>where Self: Unpin,`

### trait tokio::io::AsyncWrite

```rust
pub trait AsyncWrite {
    // Required methods
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize>>;
    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<()>>;
    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<()>>;

    // Provided methods
    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[IoSlice<'_>],
    ) -> Poll<Result<usize>> { ... }
    fn is_write_vectored(&self) -> bool { ... }
}
```

Writes bytes asynchronously.

This trait is analogous to the `std::io::Write` trait, but integrates with
the asynchronous task system. In particular, the `poll_write` method,
unlike `Write::write`, will automatically queue the current task for wakeup
and return if data is not yet available, rather than blocking the calling
thread.

Specifically, this means that the `poll_write` function will return one of
the following:

- 

`Poll::Ready(Ok(n))` means that `n` bytes of data was immediately
written.

- 

`Poll::Pending` means that no data was written from the buffer
provided. The I/O object is not currently writable but may become writable
in the future. Most importantly, the current future’s task is scheduled
to get unparked when the object is writable. This means that like
`Future::poll` you’ll receive a notification when the I/O object is
writable again.

- 

`Poll::Ready(Err(e))` for other errors are standard I/O errors coming from the
underlying object.

Utilities for working with `AsyncWrite` values are provided by
`AsyncWriteExt`. Most users will interact with `AsyncWrite` types through
these extension methods, which provide ergonomic async functions such as
`write_all` and `flush`.

**Methods**

- `fn poll_write( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8], ) -> Poll<Result<usize>>`
- `fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>>`
- `fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>>`
- `fn poll_write_vectored( self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &[IoSlice<'_>], ) -> Poll<Result<usize>>`
- `fn is_write_vectored(&self) -> bool`
- `fn poll_write( self: Pin<&mut Self>, _: &mut Context<'_>, buf: &[u8], ) -> Poll<Result<usize>>`
- `fn poll_write_vectored( self: Pin<&mut Self>, _: &mut Context<'_>, bufs: &[IoSlice<'_>], ) -> Poll<Result<usize>>`
- `fn is_write_vectored(&self) -> bool`
- `fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>>`
- `fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>>`
- `fn poll_write( self: Pin<&mut Self>, _: &mut Context<'_>, buf: &[u8], ) -> Poll<Result<usize>>`
- `fn poll_write_vectored( self: Pin<&mut Self>, _: &mut Context<'_>, bufs: &[IoSlice<'_>], ) -> Poll<Result<usize>>`
- `fn is_write_vectored(&self) -> bool`
- `fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>>`
- `fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>>`
- `fn poll_write( self: Pin<&mut Self>, _: &mut Context<'_>, buf: &[u8], ) -> Poll<Result<usize>>`
- `fn poll_write_vectored( self: Pin<&mut Self>, _: &mut Context<'_>, bufs: &[IoSlice<'_>], ) -> Poll<Result<usize>>`
- `fn is_write_vectored(&self) -> bool`
- `fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>>`
- `fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>>`
- `fn poll_write( self: Pin<&mut Self>, _: &mut Context<'_>, buf: &[u8], ) -> Poll<Result<usize>>`
- `fn poll_write_vectored( self: Pin<&mut Self>, _: &mut Context<'_>, bufs: &[IoSlice<'_>], ) -> Poll<Result<usize>>`
- `fn is_write_vectored(&self) -> bool`
- `fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>>`
- `fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>>`
- `fn poll_write( self: Pin<&mut Self>, _cx: &mut Context<'_>, buf: &[u8], ) -> Poll<Result<usize>>`
- `fn poll_write_vectored( self: Pin<&mut Self>, _: &mut Context<'_>, bufs: &[IoSlice<'_>], ) -> Poll<Result<usize>>`
- `fn is_write_vectored(&self) -> bool`
- `fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<()>>`
- `fn poll_shutdown( self: Pin<&mut Self>, _cx: &mut Context<'_>, ) -> Poll<Result<()>>`
- `fn poll_write( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8], ) -> Poll<Result<usize>>`
- `fn poll_write_vectored( self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &[IoSlice<'_>], ) -> Poll<Result<usize>>`
- `fn is_write_vectored(&self) -> bool`
- `fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>>`
- `fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>>`
- `fn poll_write( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8], ) -> Poll<Result<usize>>`
- `fn poll_write_vectored( self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &[IoSlice<'_>], ) -> Poll<Result<usize>>`
- `fn is_write_vectored(&self) -> bool`
- `fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>>`
- `fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>>`

### trait tokio::io::AsyncWriteExt

```rust
pub trait AsyncWriteExt: AsyncWrite {
    // Provided methods
    fn write<'a>(&'a mut self, src: &'a [u8]) -> Write<'a, Self>
       where Self: Unpin { ... }
    fn write_vectored<'a, 'b>(
        &'a mut self,
        bufs: &'a [IoSlice<'b>],
    ) -> WriteVectored<'a, 'b, Self>
       where Self: Unpin { ... }
    fn write_buf<'a, B>(&'a mut self, src: &'a mut B) -> WriteBuf<'a, Self, B>
       where Self: Sized + Unpin,
             B: Buf { ... }
    fn write_all_buf<'a, B>(
        &'a mut self,
        src: &'a mut B,
    ) -> WriteAllBuf<'a, Self, B>
       where Self: Sized + Unpin,
             B: Buf { ... }
    fn write_all<'a>(&'a mut self, src: &'a [u8]) -> WriteAll<'a, Self>
       where Self: Unpin { ... }
    fn write_u8(&mut self, n: u8) -> WriteU8<&mut Self>
       where Self: Unpin { ... }
    fn write_i8(&mut self, n: i8) -> WriteI8<&mut Self>
       where Self: Unpin { ... }
    fn write_u16(&mut self, n: u16) -> WriteU16<&mut Self>
       where Self: Unpin { ... }
    fn write_i16(&mut self, n: i16) -> WriteI16<&mut Self>
       where Self: Unpin { ... }
    fn write_u32(&mut self, n: u32) -> WriteU32<&mut Self>
       where Self: Unpin { ... }
    fn write_i32(&mut self, n: i32) -> WriteI32<&mut Self>
       where Self: Unpin { ... }
    fn write_u64(&mut self, n: u64) -> WriteU64<&mut Self>
       where Self: Unpin { ... }
    fn write_i64(&mut self, n: i64) -> WriteI64<&mut Self>
       where Self: Unpin { ... }
    fn write_u128(&mut self, n: u128) -> WriteU128<&mut Self>
       where Self: Unpin { ... }
    fn write_i128(&mut self, n: i128) -> WriteI128<&mut Self>
       where Self: Unpin { ... }
    fn write_f32(&mut self, n: f32) -> WriteF32<&mut Self>
       where Self: Unpin { ... }
    fn write_f64(&mut self, n: f64) -> WriteF64<&mut Self>
       where Self: Unpin { ... }
    fn write_u16_le(&mut self, n: u16) -> WriteU16Le<&mut Self>
       where Self: Unpin { ... }
    fn write_i16_le(&mut self, n: i16) -> WriteI16Le<&mut Self>
       where Self: Unpin { ... }
    fn write_u32_le(&mut self, n: u32) -> WriteU32Le<&mut Self>
       where Self: Unpin { ... }
    fn write_i32_le(&mut self, n: i32) -> WriteI32Le<&mut Self>
       where Self: Unpin { ... }
    fn write_u64_le(&mut self, n: u64) -> WriteU64Le<&mut Self>
       where Self: Unpin { ... }
    fn write_i64_le(&mut self, n: i64) -> WriteI64Le<&mut Self>
       where Self: Unpin { ... }
    fn write_u128_le(&mut self, n: u128) -> WriteU128Le<&mut Self>
       where Self: Unpin { ... }
    fn write_i128_le(&mut self, n: i128) -> WriteI128Le<&mut Self>
       where Self: Unpin { ... }
    fn write_f32_le(&mut self, n: f32) -> WriteF32Le<&mut Self>
       where Self: Unpin { ... }
    fn write_f64_le(&mut self, n: f64) -> WriteF64Le<&mut Self>
       where Self: Unpin { ... }
    fn flush(&mut self) -> Flush<'_, Self>
       where Self: Unpin { ... }
    fn shutdown(&mut self) -> Shutdown<'_, Self>
       where Self: Unpin { ... }
}
```

Writes bytes to a sink.

Implemented as an extension trait, adding utility methods to all
`AsyncWrite` types. Callers will tend to import this trait instead of
`AsyncWrite`.

```rust
use tokio::io::{self, AsyncWriteExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let data = b"some bytes";

    let mut pos = 0;
    let mut buffer = File::create("foo.txt").await?;

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..]).await?;
        pos += bytes_written;
    }

    Ok(())
}
```

See module documentation for more details.

**Methods**

- `fn write<'a>(&'a mut self, src: &'a [u8]) -> Write<'a, Self>where Self: Unpin,`
- `fn write_vectored<'a, 'b>( &'a mut self, bufs: &'a [IoSlice<'b>], ) -> WriteVectored<'a, 'b, Self>where Self: Unpin,`
- `fn write_buf<'a, B>(&'a mut self, src: &'a mut B) -> WriteBuf<'a, Self, B>where Self: Sized + Unpin, B: Buf,`
- `fn write_all_buf<'a, B>( &'a mut self, src: &'a mut B, ) -> WriteAllBuf<'a, Self, B>where Self: Sized + Unpin, B: Buf,`
- `fn write_all<'a>(&'a mut self, src: &'a [u8]) -> WriteAll<'a, Self>where Self: Unpin,`
- `fn write_u8(&mut self, n: u8) -> WriteU8<&mut Self>where Self: Unpin,`
- `fn write_i8(&mut self, n: i8) -> WriteI8<&mut Self>where Self: Unpin,`
- `fn write_u16(&mut self, n: u16) -> WriteU16<&mut Self>where Self: Unpin,`
- `fn write_i16(&mut self, n: i16) -> WriteI16<&mut Self>where Self: Unpin,`
- `fn write_u32(&mut self, n: u32) -> WriteU32<&mut Self>where Self: Unpin,`
- `fn write_i32(&mut self, n: i32) -> WriteI32<&mut Self>where Self: Unpin,`
- `fn write_u64(&mut self, n: u64) -> WriteU64<&mut Self>where Self: Unpin,`
- `fn write_i64(&mut self, n: i64) -> WriteI64<&mut Self>where Self: Unpin,`
- `fn write_u128(&mut self, n: u128) -> WriteU128<&mut Self>where Self: Unpin,`
- `fn write_i128(&mut self, n: i128) -> WriteI128<&mut Self>where Self: Unpin,`
- `fn write_f32(&mut self, n: f32) -> WriteF32<&mut Self>where Self: Unpin,`
- `fn write_f64(&mut self, n: f64) -> WriteF64<&mut Self>where Self: Unpin,`
- `fn write_u16_le(&mut self, n: u16) -> WriteU16Le<&mut Self>where Self: Unpin,`
- `fn write_i16_le(&mut self, n: i16) -> WriteI16Le<&mut Self>where Self: Unpin,`
- `fn write_u32_le(&mut self, n: u32) -> WriteU32Le<&mut Self>where Self: Unpin,`
- `fn write_i32_le(&mut self, n: i32) -> WriteI32Le<&mut Self>where Self: Unpin,`
- `fn write_u64_le(&mut self, n: u64) -> WriteU64Le<&mut Self>where Self: Unpin,`
- `fn write_i64_le(&mut self, n: i64) -> WriteI64Le<&mut Self>where Self: Unpin,`
- `fn write_u128_le(&mut self, n: u128) -> WriteU128Le<&mut Self>where Self: Unpin,`
- `fn write_i128_le(&mut self, n: i128) -> WriteI128Le<&mut Self>where Self: Unpin,`
- `fn write_f32_le(&mut self, n: f32) -> WriteF32Le<&mut Self>where Self: Unpin,`
- `fn write_f64_le(&mut self, n: f64) -> WriteF64Le<&mut Self>where Self: Unpin,`
- `fn flush(&mut self) -> Flush<'_, Self>where Self: Unpin,`
- `fn shutdown(&mut self) -> Shutdown<'_, Self>where Self: Unpin,`
