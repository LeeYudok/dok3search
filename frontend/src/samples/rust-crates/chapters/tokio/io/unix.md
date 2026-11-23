# Module tokio::io::unix

## Overview

Asynchronous IO structures specific to Unix-like operating systems.

## Items

- `AsyncFd` — Associates an IO object backed by a Unix file descriptor with the tokio reactor, allowing for readiness to be polled. The file descriptor must be of a type that can be used with the OS polling facilities (ie, poll, epoll, kqueue, etc), such as a network socket or pipe, and the file descriptor must have the nonblocking mode set to true.
- `AsyncFdReadyGuard` — Represents an IO-ready event detected on a particular file descriptor that has not yet been acknowledged. This is a must_use structure to help ensure that you do not forget to explicitly clear (or not clear) the event.
- `AsyncFdReadyMutGuard` — Represents an IO-ready event detected on a particular file descriptor that has not yet been acknowledged. This is a must_use structure to help ensure that you do not forget to explicitly clear (or not clear) the event.
- `AsyncFdRegisterError` — Error returned by register or register_with_interest.
- `TryIoError` — The error type returned by try_io.

### struct tokio::io::unix::AsyncFd

```rust
pub struct AsyncFd<T: AsRawFd> { /* private fields */ }
```

Associates an IO object backed by a Unix file descriptor with the tokio
reactor, allowing for readiness to be polled. The file descriptor must be of
a type that can be used with the OS polling facilities (ie, `poll`, `epoll`,
`kqueue`, etc), such as a network socket or pipe, and the file descriptor
must have the nonblocking mode set to true.

Creating an `AsyncFd` registers the file descriptor with the current tokio
Reactor, allowing you to directly await the file descriptor being readable
or writable. Once registered, the file descriptor remains registered until
the `AsyncFd` is dropped.

The `AsyncFd` takes ownership of an arbitrary object to represent the IO
object. It is intended that the inner object will handle closing the file
descriptor when it is dropped, avoiding resource leaks and ensuring that the
`AsyncFd` can clean up the registration before closing the file descriptor.
The `AsyncFd::into_inner` function can be used to extract the inner object
to retake control from the tokio IO reactor. The `OwnedFd` type is often
used as the inner object, as it is the simplest type that closes the fd on
drop.

The inner object is required to implement `AsRawFd`. This file descriptor
must remain open and must not change while `AsyncFd` owns the inner
object, i.e. the `AsRawFd::as_raw_fd` method on the inner type must always
return the same valid file descriptor referring to the same underlying file
description when called multiple times. Violating this requirement leads to
undefined behavior.

Polling for readiness is done by calling the async functions `readable`
and `writab

...

**Methods**

- `pub unsafe fn register(inner: T) -> Result<Self, AsyncFdRegisterError<T>>`
- `pub unsafe fn register_with_interest( inner: T, interest: Interest, ) -> Result<Self, AsyncFdRegisterError<T>>`
- `pub fn get_ref(&self) -> &T`
- `pub fn get_mut(&mut self) -> &mut T`
- `pub fn into_inner(self) -> T`
- `pub fn poll_read_ready<'a>( &'a self, cx: &mut Context<'_>, ) -> Poll<Result<AsyncFdReadyGuard<'a, T>>>`
- `pub fn poll_read_ready_mut<'a>( &'a mut self, cx: &mut Context<'_>, ) -> Poll<Result<AsyncFdReadyMutGuard<'a, T>>>`
- `pub fn poll_write_ready<'a>( &'a self, cx: &mut Context<'_>, ) -> Poll<Result<AsyncFdReadyGuard<'a, T>>>`
- `pub fn poll_write_ready_mut<'a>( &'a mut self, cx: &mut Context<'_>, ) -> Poll<Result<AsyncFdReadyMutGuard<'a, T>>>`
- `pub async fn ready( &self, interest: Interest, ) -> Result<AsyncFdReadyGuard<'_, T>>`
- `pub async fn ready_mut( &mut self, interest: Interest, ) -> Result<AsyncFdReadyMutGuard<'_, T>>`
- `pub async fn readable<'a>(&'a self) -> Result<AsyncFdReadyGuard<'a, T>>`
- `pub async fn readable_mut<'a>( &'a mut self, ) -> Result<AsyncFdReadyMutGuard<'a, T>>`
- `pub async fn writable<'a>(&'a self) -> Result<AsyncFdReadyGuard<'a, T>>`
- `pub async fn writable_mut<'a>( &'a mut self, ) -> Result<AsyncFdReadyMutGuard<'a, T>>`
- `pub async fn async_io<R>( &self, interest: Interest, f: impl FnMut(&T) -> Result<R>, ) -> Result<R>`
- `pub async fn async_io_mut<R>( &mut self, interest: Interest, f: impl FnMut(&mut T) -> Result<R>, ) -> Result<R>`
- `pub fn try_io<R>( &self, interest: Interest, f: impl FnOnce(&T) -> Result<R>, ) -> Result<R>`
- `pub fn try_io_mut<R>( &mut self, interest: Interest, f: impl FnOnce(&mut T) -> Result<R>, ) -> Result<R>`
- `pub fn new(inner: T) -> Result<Self>`
- `pub fn with_interest(inner: T, interest: Interest) -> Result<Self>`
- `pub fn try_new(inner: T) -> Result<Self, AsyncFdRegisterError<T>>`
- `pub fn try_with_interest( inner: T, interest: Interest, ) -> Result<Self, AsyncFdRegisterError<T>>`
- `fn as_fd(&self) -> BorrowedFd<'_>`
- `fn as_raw_fd(&self) -> RawFd`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`

### struct tokio::io::unix::AsyncFdReadyGuard

```rust
pub struct AsyncFdReadyGuard<'a, T: AsRawFd> { /* private fields */ }
```

Represents an IO-ready event detected on a particular file descriptor that
has not yet been acknowledged. This is a `must_use` structure to help ensure
that you do not forget to explicitly clear (or not clear) the event.

This type exposes an immutable reference to the underlying IO object.

**Methods**

- `pub fn clear_ready(&mut self)`
- `pub fn clear_ready_matching(&mut self, ready: Ready)`
- `pub fn retain_ready(&mut self)`
- `pub fn ready(&self) -> Ready`
- `pub fn try_io<R>( &mut self, f: impl FnOnce(&'a AsyncFd<Inner>) -> Result<R>, ) -> Result<Result<R>, TryIoError>`
- `pub fn get_ref(&self) -> &'a AsyncFd<Inner>`
- `pub fn get_inner(&self) -> &'a Inner`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tokio::io::unix::AsyncFdReadyMutGuard

```rust
pub struct AsyncFdReadyMutGuard<'a, T: AsRawFd> { /* private fields */ }
```

Represents an IO-ready event detected on a particular file descriptor that
has not yet been acknowledged. This is a `must_use` structure to help ensure
that you do not forget to explicitly clear (or not clear) the event.

This type exposes a mutable reference to the underlying IO object.

**Methods**

- `pub fn clear_ready(&mut self)`
- `pub fn clear_ready_matching(&mut self, ready: Ready)`
- `pub fn retain_ready(&mut self)`
- `pub fn ready(&self) -> Ready`
- `pub fn try_io<R>( &mut self, f: impl FnOnce(&mut AsyncFd<Inner>) -> Result<R>, ) -> Result<Result<R>, TryIoError>`
- `pub fn get_ref(&self) -> &AsyncFd<Inner>`
- `pub fn get_mut(&mut self) -> &mut AsyncFd<Inner>`
- `pub fn get_inner(&self) -> &Inner`
- `pub fn get_inner_mut(&mut self) -> &mut Inner`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tokio::io::unix::AsyncFdRegisterError

```rust
pub struct AsyncFdRegisterError<T> { /* private fields */ }
```

Error returned by `register` or `register_with_interest`.

**Methods**

- `pub fn into_parts(self) -> (T, Error)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(value: AsyncFdRegisterError<T>) -> Self`

### struct tokio::io::unix::TryIoError

```rust
pub struct TryIoError(/* private fields */);
```

The error type returned by `try_io`.

This error indicates that the IO resource returned a `WouldBlock` error.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
