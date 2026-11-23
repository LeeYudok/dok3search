# Module axum::body

## Overview

HTTP body utilities.

## Items

- `Body` — The body type used in axum requests and responses.
- `BodyDataStream` — A stream of data frames.
- `to_bytes` — Converts Body into [Bytes] and limits the maximum size of the body.

### fn axum::body::to_bytes

```rust
pub async fn to_bytes(body: Body, limit: usize) -> Result<Bytes, Error>
```

Converts `Body` into [`Bytes`] and limits the maximum size of the body.

**Example**

```rust
use axum::body::{to_bytes, Body};

let body = Body::from(vec![1, 2, 3]);
// Use `usize::MAX` if you don't care about the maximum size.
let bytes = to_bytes(body, usize::MAX).await?;
assert_eq!(&bytes[..], &[1, 2, 3]);
```

You can detect if the limit was hit by checking the source of the error:

```rust
use axum::body::{to_bytes, Body};
use http_body_util::LengthLimitError;

let body = Body::from(vec![1, 2, 3]);
match to_bytes(body, 1).await {
    Ok(_bytes) => panic!("should have hit the limit"),
    Err(err) => {
        let source = std::error::Error::source(&err).unwrap();
        assert!(source.is::<LengthLimitError>());
    }
}
```

### struct axum::body::Body

```rust
pub struct Body(/* private fields */);
```

The body type used in axum requests and responses.

**Methods**

- `pub fn new<B>(body: B) -> Bodywhere B: Body<Data = Bytes> + Send + 'static, <B as Body>::Error: Into<Box<dyn Error + Send + Sync>>,`
- `pub fn empty() -> Body`
- `pub fn from_stream<S>(stream: S) -> Bodywhere S: TryStream + Send + 'static, <S as TryStream>::Ok: Into<Bytes>, <S as TryStream>::Error: Into<Box<dyn Error + Send + Sync>>,`
- `pub fn into_data_stream(self) -> BodyDataStream`
- `fn poll_frame( self: Pin<&mut Body>, cx: &mut Context<'_>, ) -> Poll<Option<Result<Frame<<Body as Body>::Data>, <Body as Body>::Error>>>`
- `fn size_hint(&self) -> SizeHint`
- `fn is_end_stream(&self) -> bool`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn default() -> Body`
- `fn from(buf: &'static [u8]) -> Body`
- `fn from(buf: &'static str) -> Body`
- `fn from(_: ()) -> Body`
- `fn from(buf: Bytes) -> Body`
- `fn from(buf: Cow<'static, [u8]>) -> Body`
- `fn from(buf: Cow<'static, str>) -> Body`
- `fn from(buf: String) -> Body`
- `fn from(buf: Vec<u8>) -> Body`
- `async fn from_request( req: Request<Body>, _: &S, ) -> Result<Body, <Body as FromRequest<S>>::Rejection>`
- `fn into_response(self) -> Response<Body>`

### struct axum::body::BodyDataStream

```rust
pub struct BodyDataStream { /* private fields */ }
```

A stream of data frames.

Created with `Body::into_data_stream`.

**Methods**

- `fn poll_frame( self: Pin<&mut BodyDataStream>, cx: &mut Context<'_>, ) -> Poll<Option<Result<Frame<<BodyDataStream as Body>::Data>, <BodyDataStream as Body>::Error>>>`
- `fn is_end_stream(&self) -> bool`
- `fn size_hint(&self) -> SizeHint`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn poll_next( self: Pin<&mut BodyDataStream>, cx: &mut Context<'_>, ) -> Poll<Option<<BodyDataStream as Stream>::Item>>`
- `fn size_hint(&self) -> (usize, Option<usize>)`
