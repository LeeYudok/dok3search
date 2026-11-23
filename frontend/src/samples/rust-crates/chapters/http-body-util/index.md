# Crate http-body-util

## Overview

Utilities for [`http_body::Body`].

`BodyExt` adds extensions to the common trait.

`Empty` and `Full` provide simple implementations.

## Items

- `combinators` — Combinators for the Body trait.
- `BodyDataStream` — A data stream created from a [Body].
- `BodyStream` — A stream created from a [Body].
- `Collected` — A collected body produced by BodyExt::collect which collects all the DATA frames and trailers.
- `Empty` — A body that is always empty.
- `Full` — A body that consists of a single chunk.
- `LengthLimitError` — An error returned when body length exceeds the configured limit.
- `Limited` — A length limited body.
- `StreamBody` — A body created from a [Stream].
- `Either` — Sum type with two cases: Left and Right, used if a body can be one of two distinct types.
- `BodyExt` — An extension trait for [http_body::Body] adding various combinators and adapters

### enum http-body-util::Either

```rust
pub enum Either<L, R> {
    Left(L),
    Right(R),
}
```

Sum type with two cases: `Left` and `Right`, used if a body can be one of
two distinct types.

**Methods**

- `pub fn into_inner(self) -> L`
- `fn poll_frame( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>>`
- `fn is_end_stream(&self) -> bool`
- `fn size_hint(&self) -> SizeHint`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct http-body-util::BodyDataStream

```rust
pub struct BodyDataStream<B> { /* private fields */ }
```

A data stream created from a [`Body`].

**Methods**

- `pub fn new(body: B) -> Self`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_next( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Option<Self::Item>>`
- `fn size_hint(&self) -> (usize, Option<usize>)`

### struct http-body-util::BodyStream

```rust
pub struct BodyStream<B> { /* private fields */ }
```

A stream created from a [`Body`].

**Methods**

- `pub fn new(body: B) -> Self`
- `fn poll_frame( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>>`
- `fn is_end_stream(&self) -> bool`
- `fn size_hint(&self) -> SizeHint`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_next( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Option<Self::Item>>`
- `fn size_hint(&self) -> (usize, Option<usize>)`

### struct http-body-util::Collected

```rust
pub struct Collected<B> { /* private fields */ }
```

A collected body produced by `BodyExt::collect` which collects all the DATA frames
and trailers.

**Methods**

- `pub fn trailers(&self) -> Option<&HeaderMap>`
- `pub fn aggregate(self) -> impl Buf`
- `pub fn to_bytes(self) -> Bytes`
- `fn poll_frame( self: Pin<&mut Self>, _: &mut Context<'_>, ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>>`
- `fn is_end_stream(&self) -> bool`
- `fn size_hint(&self) -> SizeHint`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`

### struct http-body-util::Empty

```rust
pub struct Empty<D> { /* private fields */ }
```

A body that is always empty.

**Methods**

- `pub const fn new() -> Self`
- `fn poll_frame( self: Pin<&mut Self>, _cx: &mut Context<'_>, ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>>`
- `fn is_end_stream(&self) -> bool`
- `fn size_hint(&self) -> SizeHint`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`

### struct http-body-util::Full

```rust
pub struct Full<D> { /* private fields */ }
```

A body that consists of a single chunk.

**Methods**

- `pub fn new(data: D) -> Self`
- `pub fn into_inner(self) -> Option<D>`
- `fn poll_frame( self: Pin<&mut Self>, _cx: &mut Context<'_>, ) -> Poll<Option<Result<Frame<D>, Self::Error>>>`
- `fn is_end_stream(&self) -> bool`
- `fn size_hint(&self) -> SizeHint`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn from(slice: &'static [u8]) -> Self`
- `fn from(slice: &'static str) -> Self`
- `fn from(bytes: Bytes) -> Self`
- `fn from(cow: Cow<'static, B>) -> Self`
- `fn from(s: String) -> Self`
- `fn from(vec: Vec<u8>) -> Self`

### struct http-body-util::LengthLimitError

```rust
#[non_exhaustive]pub struct LengthLimitError;
```

An error returned when body length exceeds the configured limit.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`

### struct http-body-util::Limited

```rust
pub struct Limited<B> { /* private fields */ }
```

A length limited body.

This body will return an error if more than the configured number
of bytes are returned on polling the wrapped body.

**Methods**

- `pub fn new(inner: B, limit: usize) -> Self`
- `fn poll_frame( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>>`
- `fn is_end_stream(&self) -> bool`
- `fn size_hint(&self) -> SizeHint`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct http-body-util::StreamBody

```rust
pub struct StreamBody<S> { /* private fields */ }
```

A body created from a [`Stream`].

**Methods**

- `pub fn new(stream: S) -> Self`
- `fn poll_frame( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>>`
- `fn is_end_stream(&self) -> bool`
- `fn size_hint(&self) -> SizeHint`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_next( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Option<Self::Item>>`
- `fn size_hint(&self) -> (usize, Option<usize>)`

### trait http-body-util::BodyExt

```rust
pub trait BodyExt: Body {
    // Provided methods
    fn frame(&mut self) -> Frame<'_, Self> ⓘ
       where Self: Unpin { ... }
    fn map_frame<F, B>(self, f: F) -> MapFrame<Self, F>
       where Self: Sized,
             F: FnMut(Frame<Self::Data>) -> Frame<B>,
             B: Buf { ... }
    fn inspect_frame<F>(self, f: F) -> InspectFrame<Self, F>
       where Self: Sized,
             F: FnMut(&Frame<Self::Data>) { ... }
    fn map_err<F, E>(self, f: F) -> MapErr<Self, F>
       where Self: Sized,
             F: FnMut(Self::Error) -> E { ... }
    fn inspect_err<F>(self, f: F) -> InspectErr<Self, F>
       where Self: Sized,
             F: FnMut(&Self::Error) { ... }
    fn boxed(self) -> BoxBody<Self::Data, Self::Error>
       where Self: Sized + Send + Sync + 'static { ... }
    fn boxed_unsync(self) -> UnsyncBoxBody<Self::Data, Self::Error>
       where Self: Sized + Send + 'static { ... }
    fn collect(self) -> Collect<Self> ⓘ
       where Self: Sized { ... }
    fn with_trailers<F>(self, trailers: F) -> WithTrailers<Self, F>
       where Self: Sized,
             F: Future<Output = Option<Result<HeaderMap, Self::Error>>> { ... }
    fn into_stream(self) -> BodyStream<Self>
       where Self: Sized { ... }
    fn into_data_stream(self) -> BodyDataStream<Self>
       where Self: Sized { ... }
    fn fuse(self) -> Fuse<Self>
       where Self: Sized { ... }
}
```

An extension trait for [`http_body::Body`] adding various combinators and adapters

**Methods**

- `fn frame(&mut self) -> Frame<'_, Self> ⓘwhere Self: Unpin,`
- `fn map_frame<F, B>(self, f: F) -> MapFrame<Self, F>where Self: Sized, F: FnMut(Frame<Self::Data>) -> Frame<B>, B: Buf,`
- `fn inspect_frame<F>(self, f: F) -> InspectFrame<Self, F>where Self: Sized, F: FnMut(&Frame<Self::Data>),`
- `fn map_err<F, E>(self, f: F) -> MapErr<Self, F>where Self: Sized, F: FnMut(Self::Error) -> E,`
- `fn inspect_err<F>(self, f: F) -> InspectErr<Self, F>where Self: Sized, F: FnMut(&Self::Error),`
- `fn boxed(self) -> BoxBody<Self::Data, Self::Error>where Self: Sized + Send + Sync + 'static,`
- `fn boxed_unsync(self) -> UnsyncBoxBody<Self::Data, Self::Error>where Self: Sized + Send + 'static,`
- `fn collect(self) -> Collect<Self> ⓘwhere Self: Sized,`
- `fn with_trailers<F>(self, trailers: F) -> WithTrailers<Self, F>where Self: Sized, F: Future<Output = Option<Result<HeaderMap, Self::Error>>>,`
- `fn into_stream(self) -> BodyStream<Self>where Self: Sized,`
- `fn into_data_stream(self) -> BodyDataStream<Self>where Self: Sized,`
- `fn fuse(self) -> Fuse<Self>where Self: Sized,`
