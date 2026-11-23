# Module http-body-util::combinators

## Overview

Combinators for the `Body` trait.

## Items

- `BoxBody` — A boxed [Body] trait object.
- `Collect` — Future that resolves into a Collected.
- `Frame` — Future that resolves to the next frame from a [Body].
- `Fuse` — A “fused” [Body].
- `InspectErr` — Body returned by the inspect_err() combinator.
- `InspectFrame` — Body returned by the inspect_frame() combinator.
- `MapErr` — Body returned by the map_err combinator.
- `MapFrame` — Body returned by the map_frame combinator.
- `UnsyncBoxBody` — A boxed [Body] trait object that is !Sync.
- `WithTrailers` — Adds trailers to a body.

### struct http-body-util::combinators::BoxBody

```rust
pub struct BoxBody<D, E> { /* private fields */ }
```

A boxed [`Body`] trait object.

**Methods**

- `pub fn new<B>(body: B) -> Selfwhere B: Body<Data = D, Error = E> + Send + Sync + 'static, D: Buf,`
- `fn poll_frame( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>>`
- `fn is_end_stream(&self) -> bool`
- `fn size_hint(&self) -> SizeHint`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`

### struct http-body-util::combinators::Collect

```rust
pub struct Collect<T>where
    T: Body + ?Sized,{ /* private fields */ }
```

Future that resolves into a `Collected`.

**Methods**

- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`

### struct http-body-util::combinators::Frame

```rust
pub struct Frame<'a, T: ?Sized>(/* private fields */);
```

Future that resolves to the next frame from a [`Body`].

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output>`

### struct http-body-util::combinators::Fuse

```rust
pub struct Fuse<B> { /* private fields */ }
```

A “fused” [`Body`].

This [`Body`] yields `Poll::Ready(None)` forever after the underlying body yields
`Poll::Ready(None)`, or an error `Poll::Ready(Some(Err(_)))`, once.

Bodies should ideally continue to return `Poll::Ready(None)` indefinitely after the end of
the stream is reached. `Fuse<B>` avoids polling its underlying body `B` further after the
underlying stream as ended, which can be useful for implementation that cannot uphold this
guarantee.

This is akin to the functionality that `std::iter::Iterator::fuse()` provides for
`Iterator`s.

**Methods**

- `pub fn new(body: B) -> Self`
- `fn poll_frame( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Option<Result<Frame<B::Data>, B::Error>>>`
- `fn is_end_stream(&self) -> bool`
- `fn size_hint(&self) -> SizeHint`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct http-body-util::combinators::InspectErr

```rust
pub struct InspectErr<B, F> { /* private fields */ }
```

Body returned by the `inspect_err()` combinator.

**Methods**

- `pub fn get_ref(&self) -> &B`
- `pub fn get_mut(&mut self) -> &mut B`
- `pub fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut B>`
- `pub fn into_inner(self) -> B`
- `fn poll_frame( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>>`
- `fn size_hint(&self) -> SizeHint`
- `fn is_end_stream(&self) -> bool`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct http-body-util::combinators::InspectFrame

```rust
pub struct InspectFrame<B, F> { /* private fields */ }
```

Body returned by the `inspect_frame()` combinator.

**Methods**

- `pub fn get_ref(&self) -> &B`
- `pub fn get_mut(&mut self) -> &mut B`
- `pub fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut B>`
- `pub fn into_inner(self) -> B`
- `fn poll_frame( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>>`
- `fn size_hint(&self) -> SizeHint`
- `fn is_end_stream(&self) -> bool`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct http-body-util::combinators::MapErr

```rust
pub struct MapErr<B, F> { /* private fields */ }
```

Body returned by the `map_err` combinator.

**Methods**

- `pub fn get_ref(&self) -> &B`
- `pub fn get_mut(&mut self) -> &mut B`
- `pub fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut B>`
- `pub fn into_inner(self) -> B`
- `fn poll_frame( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>>`
- `fn is_end_stream(&self) -> bool`
- `fn size_hint(&self) -> SizeHint`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct http-body-util::combinators::MapFrame

```rust
pub struct MapFrame<B, F> { /* private fields */ }
```

Body returned by the `map_frame` combinator.

**Methods**

- `pub fn get_ref(&self) -> &B`
- `pub fn get_mut(&mut self) -> &mut B`
- `pub fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut B>`
- `pub fn into_inner(self) -> B`
- `fn poll_frame( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>>`
- `fn is_end_stream(&self) -> bool`
- `fn size_hint(&self) -> SizeHint`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct http-body-util::combinators::UnsyncBoxBody

```rust
pub struct UnsyncBoxBody<D, E> { /* private fields */ }
```

A boxed [`Body`] trait object that is !Sync.

**Methods**

- `pub fn new<B>(body: B) -> Selfwhere B: Body<Data = D, Error = E> + Send + 'static, D: Buf,`
- `fn poll_frame( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>>`
- `fn is_end_stream(&self) -> bool`
- `fn size_hint(&self) -> SizeHint`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`

### struct http-body-util::combinators::WithTrailers

```rust
pub struct WithTrailers<T, F> { /* private fields */ }
```

Adds trailers to a body.

See [`BodyExt::with_trailers`] for more details.

**Methods**

- `fn poll_frame( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>>`
- `fn size_hint(&self) -> SizeHint`
- `fn is_end_stream(&self) -> bool`
