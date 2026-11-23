# Module tower::layer::util

## Overview

Utilities for combining layers

## Items

- `Identity` — A no-op middleware.
- `Stack` — Two middlewares chained together.

### struct tower::layer::util::Identity

```rust
pub struct Identity { /* private fields */ }
```

A no-op middleware.

When wrapping a `Service`, the `Identity` layer returns the provided
service without modifying it.

**Methods**

- `pub const fn new() -> Identity`
- `fn clone(&self) -> Identity`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn default() -> Identity`
- `fn layer(&self, inner: S) -> <Identity as Layer<S>>::Service`

### struct tower::layer::util::Stack

```rust
pub struct Stack<Inner, Outer> { /* private fields */ }
```

Two middlewares chained together.

**Methods**

- `pub const fn new(inner: Inner, outer: Outer) -> Stack<Inner, Outer>`
- `fn clone(&self) -> Stack<Inner, Outer>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn layer(&self, service: S) -> <Stack<Inner, Outer> as Layer<S>>::Service`
