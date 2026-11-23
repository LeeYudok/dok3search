# Module tower-http::body

## Overview

Body types.

All these are wrappers around other body types. You shouldn’t have to use them in your code.
Use `http-body-util` instead.

They exist because we don’t want to expose types from `http-body-util` in `tower-http`s public
API.

### struct tower-http::body::Full

```rust
pub struct Full { /* private fields */ }
```

**Methods**

- `fn poll_frame( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>>`
- `fn is_end_stream(&self) -> bool`
- `fn size_hint(&self) -> SizeHint`
- `fn default() -> Self`

### struct tower-http::body::Limited

```rust
pub struct Limited<B> { /* private fields */ }
```

**Methods**

- `fn poll_frame( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>>`
- `fn is_end_stream(&self) -> bool`
- `fn size_hint(&self) -> SizeHint`

### struct tower-http::body::UnsyncBoxBody

```rust
pub struct UnsyncBoxBody<D, E> { /* private fields */ }
```

**Methods**

- `pub fn new<B>(body: B) -> Selfwhere B: Body<Data = D, Error = E> + Send + 'static,`
- `fn poll_frame( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>>`
- `fn is_end_stream(&self) -> bool`
- `fn size_hint(&self) -> SizeHint`
- `fn default() -> Self`
- `fn from(body: ResponseBody) -> Self`
