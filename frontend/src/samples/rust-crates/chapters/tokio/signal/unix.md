# Module tokio::signal::unix

## Overview

Unix-specific types for signal handling.

This module is only defined on Unix platforms and contains the primary
`Signal` type for receiving notifications of signals.

## Items

- `Signal` — An listener for receiving a particular type of OS signal.
- `SignalKind` — Represents the specific kind of signal to listen for.
- `signal` — Creates a new listener which will receive notifications when the current process receives the specified signal kind.

### fn tokio::signal::unix::signal

```rust
pub fn signal(kind: SignalKind) -> Result<Signal>
```

Creates a new listener which will receive notifications when the current
process receives the specified signal `kind`.

This function will create a new stream which binds to the default reactor.
The `Signal` stream is an infinite stream which will receive
notifications whenever a signal is received. More documentation can be
found on `Signal` itself, but to reiterate:

- Signals may be coalesced beyond what the kernel already does.

- Once a signal handler is registered with the process the underlying
libc signal handler is never unregistered.

A `Signal` stream can be created for a particular signal number
multiple times. When a signal is received then all the associated
channels will receive the signal notification.

**Errors**

- If the lower-level C functions fail for some reason.

- If the previous initialization of this specific signal failed.

- If the signal is one of
`signal_hook::FORBIDDEN`

**Panics**

This function panics if there is no current reactor set, or if the `rt`
feature flag is not enabled.

### struct tokio::signal::unix::Signal

```rust
pub struct Signal { /* private fields */ }
```

An listener for receiving a particular type of OS signal.

The listener can be turned into a `Stream` using `SignalStream`.

In general signal handling on Unix is a pretty tricky topic, and this
structure is no exception! There are some important limitations to keep in
mind when using `Signal` streams:

- 

Signals handling in Unix already necessitates coalescing signals
together sometimes. This `Signal` stream is also no exception here in
that it will also coalesce signals. That is, even if the signal handler
for this process runs multiple times, the `Signal` stream may only return
one signal notification. Specifically, before `poll` is called, all
signal notifications are coalesced into one item returned from `poll`.
Once `poll` has been called, however, a further signal is guaranteed to
be yielded as an item.

Put another way, any element pulled off the returned listener corresponds to
at least one signal, but possibly more.

- 

Signal handling in general is relatively inefficient. Although some
improvements are possible in this crate, it’s recommended to not plan on
having millions of signal channels open.

If you’ve got any questions about this feel free to open an issue on the
repo! New approaches to alleviate some of these limitations are always
appreciated!

**Caveats**

The first time that a `Signal` instance is registered for a particular
signal kind, an OS signal-handler is installed which replaces the default
platform behavior when that signal is received, for the duration of the
entire process.

For example, Unix systems will terminate a process by default whe

...

**Methods**

- `pub async fn recv(&mut self) -> Option<()>`
- `pub fn poll_recv(&mut self, cx: &mut Context<'_>) -> Poll<Option<()>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tokio::signal::unix::SignalKind

```rust
pub struct SignalKind(/* private fields */);
```

Represents the specific kind of signal to listen for.

**Methods**

- `pub const fn from_raw(signum: c_int) -> Self`
- `pub const fn as_raw_value(&self) -> c_int`
- `pub const fn alarm() -> Self`
- `pub const fn child() -> Self`
- `pub const fn hangup() -> Self`
- `pub const fn interrupt() -> Self`
- `pub const fn io() -> Self`
- `pub const fn pipe() -> Self`
- `pub const fn quit() -> Self`
- `pub const fn terminate() -> Self`
- `pub const fn user_defined1() -> Self`
- `pub const fn user_defined2() -> Self`
- `pub const fn window_change() -> Self`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn from(kind: SignalKind) -> Self`
- `fn from(signum: c_int) -> Self`
- `fn hash<__H: Hasher>(&self, state: &mut __H)`
- `1.3.0 · fn hash_slice<H>(data: &[Self], state: &mut H)where H: Hasher, Self: Sized,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
