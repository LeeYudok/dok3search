# Module tokio::net::unix::pipe

## Overview

Unix pipe types.

## Items

- `OpenOptions` — Options and flags which can be used to configure how a FIFO file is opened.
- `Receiver` — Reading end of a Unix pipe.
- `Sender` — Writing end of a Unix pipe.
- `pipe` — Creates a new anonymous Unix pipe.

### fn tokio::net::unix::pipe::pipe

```rust
pub fn pipe() -> Result<(Sender, Receiver)>
```

Creates a new anonymous Unix pipe.

This function will open a new pipe and associate both pipe ends with the default
event loop.

If you need to create a pipe for communication with a spawned process, you can
use `Stdio::piped()` instead.

**Errors**

If creating a pipe fails, this function will return with the related OS error.

**Examples**

Create a pipe and pass the writing end to a spawned process.

```rust
use tokio::net::unix::pipe;
use tokio::process::Command;

let (tx, mut rx) = pipe::pipe()?;
let mut buffer = String::new();

let status = Command::new("echo")
    .arg("Hello, world!")
    .stdout(tx.into_blocking_fd()?)
    .status();
rx.read_to_string(&mut buffer).await?;

assert!(status.await?.success());
assert_eq!(buffer, "Hello, world!\n");
```

**Panics**

This function panics if it is not called from within a runtime with
IO enabled.

The runtime is usually set implicitly when this function is called
from a future driven by a tokio runtime, otherwise runtime can be set
explicitly with `Runtime::enter` function.

### struct tokio::net::unix::pipe::OpenOptions

```rust
pub struct OpenOptions { /* private fields */ }
```

Options and flags which can be used to configure how a FIFO file is opened.

This builder allows configuring how to create a pipe end from a FIFO file.
Generally speaking, when using `OpenOptions`, you’ll first call `new`,
then chain calls to methods to set each option, then call either
`open_receiver` or `open_sender`, passing the path of the FIFO file you
are trying to open. This will give you a `io::Result` with a pipe end
inside that you can further operate on.

**Examples**

Opening a pair of pipe ends from a FIFO file:

```rust
use tokio::net::unix::pipe;

const FIFO_NAME: &str = "path/to/a/fifo";

let rx = pipe::OpenOptions::new().open_receiver(FIFO_NAME)?;
let tx = pipe::OpenOptions::new().open_sender(FIFO_NAME)?;
```

Opening a `Sender` on Linux when you are sure the file is a FIFO:

ⓘ
```rust
use tokio::net::unix::pipe;
use nix::{unistd::mkfifo, sys::stat::Mode};

// Our program has exclusive access to this path.
const FIFO_NAME: &str = "path/to/a/new/fifo";

mkfifo(FIFO_NAME, Mode::S_IRWXU)?;
let tx = pipe::OpenOptions::new()
    .read_write(true)
    .unchecked(true)
    .open_sender(FIFO_NAME)?;
```

**Methods**

- `pub fn new() -> OpenOptions`
- `pub fn read_write(&mut self, value: bool) -> &mut Self`
- `pub fn unchecked(&mut self, value: bool) -> &mut Self`
- `pub fn open_receiver<P: AsRef<Path>>(&self, path: P) -> Result<Receiver>`
- `pub fn open_sender<P: AsRef<Path>>(&self, path: P) -> Result<Sender>`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> OpenOptions`

### struct tokio::net::unix::pipe::Receiver

```rust
pub struct Receiver { /* private fields */ }
```

Reading end of a Unix pipe.

It can be constructed from a FIFO file with `OpenOptions::open_receiver`.

**Examples**

Receiving messages from a named pipe in a loop:

```rust
use tokio::net::unix::pipe;
use tokio::io::{self, AsyncReadExt};

const FIFO_NAME: &str = "path/to/a/fifo";

let mut rx = pipe::OpenOptions::new().open_receiver(FIFO_NAME)?;
loop {
    let mut msg = vec![0; 256];
    match rx.read_exact(&mut msg).await {
        Ok(_) => {
            /* handle the message */
        }
        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
            // Writing end has been closed, we should reopen the pipe.
            rx = pipe::OpenOptions::new().open_receiver(FIFO_NAME)?;
        }
        Err(e) => return Err(e.into()),
    }
}
```

On Linux, you can use a `Receiver` in read-write access mode to implement
resilient reading from a named pipe. Unlike `Receiver` opened in read-only
mode, read from a pipe in read-write mode will not fail with `UnexpectedEof`
when the writing end is closed. This way, a `Receiver` can asynchronously
wait for the next writer to open the pipe.

You should not use functions waiting for EOF such as `read_to_end` with
a `Receiver` in read-write access mode, since it may wait forever.
`Receiver` in this mode also holds an open writing end, which prevents
receiving EOF.

To set the read-write access mode you can use `OpenOptions::read_write`.
Note that using read-write access mode with FIFO files is not defined by
the POSIX standard and it is only guaranteed to work on Linux.

ⓘ

...

**Methods**

- `pub fn from_file(file: File) -> Result<Receiver>`
- `pub fn from_owned_fd(owned_fd: OwnedFd) -> Result<Receiver>`
- `pub fn from_file_unchecked(file: File) -> Result<Receiver>`
- `pub fn from_owned_fd_unchecked(owned_fd: OwnedFd) -> Result<Receiver>`
- `pub async fn ready(&self, interest: Interest) -> Result<Ready>`
- `pub async fn readable(&self) -> Result<()>`
- `pub fn poll_read_ready(&self, cx: &mut Context<'_>) -> Poll<Result<()>>`
- `pub fn try_read(&self, buf: &mut [u8]) -> Result<usize>`
- `pub fn try_read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> Result<usize>`
- `pub fn try_io<R>(&self, f: impl FnOnce() -> Result<R>) -> Result<R>`
- `pub fn try_read_buf<B: BufMut>(&self, buf: &mut B) -> Result<usize>`
- `pub fn into_blocking_fd(self) -> Result<OwnedFd>`
- `pub fn into_nonblocking_fd(self) -> Result<OwnedFd>`
- `fn as_fd(&self) -> BorrowedFd<'_>`
- `fn as_raw_fd(&self) -> RawFd`
- `fn poll_read( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>, ) -> Poll<Result<()>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tokio::net::unix::pipe::Sender

```rust
pub struct Sender { /* private fields */ }
```

Writing end of a Unix pipe.

It can be constructed from a FIFO file with `OpenOptions::open_sender`.

Opening a named pipe for writing involves a few steps.
Call to `OpenOptions::open_sender` might fail with an error indicating
different things:

- `io::ErrorKind::NotFound` - There is no file at the specified path.

- `io::ErrorKind::InvalidInput` - The file exists, but it is not a FIFO.

- `ENXIO` - The file is a FIFO, but no process has it open for reading.
Sleep for a while and try again.

- Other OS errors not specific to opening FIFO files.

Opening a `Sender` from a FIFO file should look like this:

```rust
use tokio::net::unix::pipe;
use tokio::time::{self, Duration};

const FIFO_NAME: &str = "path/to/a/fifo";

// Wait for a reader to open the file.
let tx = loop {
    match pipe::OpenOptions::new().open_sender(FIFO_NAME) {
        Ok(tx) => break tx,
        Err(e) if e.raw_os_error() == Some(libc::ENXIO) => {},
        Err(e) => return Err(e.into()),
    }

    time::sleep(Duration::from_millis(50)).await;
};
```

On Linux, it is possible to create a `Sender` without waiting in a sleeping
loop. This is done by opening a named pipe in read-write access mode with
`OpenOptions::read_write`. This way, a `Sender` can at the same time hold
both a writing end and a reading end, and the latter allows to open a FIFO
without `ENXIO` error since the pipe is open for reading as well.

`Sender` cannot be used to read from a pipe, so in practice the read access
is only used when a FIFO is opened. However, using a `Sender` in read-write
mode may lead to lost data, because written

...

**Methods**

- `pub fn from_file(file: File) -> Result<Sender>`
- `pub fn from_owned_fd(owned_fd: OwnedFd) -> Result<Sender>`
- `pub fn from_file_unchecked(file: File) -> Result<Sender>`
- `pub fn from_owned_fd_unchecked(owned_fd: OwnedFd) -> Result<Sender>`
- `pub async fn ready(&self, interest: Interest) -> Result<Ready>`
- `pub async fn writable(&self) -> Result<()>`
- `pub fn poll_write_ready(&self, cx: &mut Context<'_>) -> Poll<Result<()>>`
- `pub fn try_write(&self, buf: &[u8]) -> Result<usize>`
- `pub fn try_write_vectored(&self, buf: &[IoSlice<'_>]) -> Result<usize>`
- `pub fn try_io<R>(&self, f: impl FnOnce() -> Result<R>) -> Result<R>`
- `pub fn into_blocking_fd(self) -> Result<OwnedFd>`
- `pub fn into_nonblocking_fd(self) -> Result<OwnedFd>`
- `fn as_fd(&self) -> BorrowedFd<'_>`
- `fn as_raw_fd(&self) -> RawFd`
- `fn poll_write( self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8], ) -> Poll<Result<usize>>`
- `fn poll_write_vectored( self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &[IoSlice<'_>], ) -> Poll<Result<usize>>`
- `fn is_write_vectored(&self) -> bool`
- `fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>>`
- `fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
