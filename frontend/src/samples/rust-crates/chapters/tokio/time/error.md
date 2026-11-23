# Module tokio::time::error

## Overview

Time error types.

## Items

- `Elapsed` — Errors returned by Timeout.
- `Error` — Errors encountered by the timer implementation.

### struct tokio::time::error::Elapsed

```rust
pub struct Elapsed(/* private fields */);
```

Errors returned by `Timeout`.

This error is returned when a timeout expires before the function was able
to finish.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(_err: Elapsed) -> Error`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### struct tokio::time::error::Error

```rust
pub struct Error(/* private fields */);
```

Errors encountered by the timer implementation.

Currently, there are two different errors that can occur:

- 

`shutdown` occurs when a timer operation is attempted, but the timer
instance has been dropped. In this case, the operation will never be able
to complete and the `shutdown` error is returned. This is a permanent
error, i.e., once this error is observed, timer operations will never
succeed in the future.

- 

`at_capacity` occurs when a timer operation is attempted, but the timer
instance is currently handling its maximum number of outstanding sleep instances.
In this case, the operation is not able to be performed at the current
moment, and `at_capacity` is returned. This is a transient error, i.e., at
some point in the future, if the operation is attempted again, it might
succeed. Callers that observe this error should attempt to shed load. One
way to do this would be dropping the future that issued the timer operation.

**Methods**

- `pub fn shutdown() -> Error`
- `pub fn is_shutdown(&self) -> bool`
- `pub fn at_capacity() -> Error`
- `pub fn is_at_capacity(&self) -> bool`
- `pub fn invalid() -> Error`
- `pub fn is_invalid(&self) -> bool`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
