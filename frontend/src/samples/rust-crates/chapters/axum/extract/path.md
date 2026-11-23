# Module axum::extract::path

## Overview

Extractor that will get captures from the URL and parse them using
[`serde`].

## Items

- `FailedToDeserializePathParams` — Rejection type for Path if the captured routes params couldn’t be deserialized into the expected type.
- `InvalidUtf8InPathParam` — Rejection used by RawPathParams if a parameter contained text that, once percent decoded, wasn’t valid UTF-8.
- `Path` — Extractor that will get captures from the URL and parse them using serde.
- `RawPathParams` — Extractor that will get captures from the URL without deserializing them.
- `RawPathParamsIter` — An iterator over raw path parameters.
- `ErrorKind` — The kinds of errors that can happen we deserializing into a Path.

### enum axum::extract::path::ErrorKind

```rust
#[non_exhaustive]pub enum ErrorKind {
    WrongNumberOfParameters {
        got: usize,
        expected: usize,
    },
    ParseErrorAtKey {
        key: String,
        value: String,
        expected_type: &'static str,
    },
    ParseErrorAtIndex {
        index: usize,
        value: String,
        expected_type: &'static str,
    },
    ParseError {
        value: String,
        expected_type: &'static str,
    },
    InvalidUtf8InPathParam {
        key: String,
    },
    UnsupportedType {
        name: &'static str,
    },
    DeserializeError {
        key: String,
        value: String,
        message: String,
    },
    Message(String),
}
```

The kinds of errors that can happen we deserializing into a `Path`.

This type is obtained through `FailedToDeserializePathParams::kind` or
`FailedToDeserializePathParams::into_kind` and is useful for building
more precise error messages.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### struct axum::extract::path::FailedToDeserializePathParams

```rust
pub struct FailedToDeserializePathParams(/* private fields */);
```

Rejection type for `Path` if the captured routes params couldn’t be deserialized
into the expected type.

**Methods**

- `pub fn kind(&self) -> &ErrorKind`
- `pub fn into_kind(self) -> ErrorKind`
- `pub fn body_text(&self) -> String`
- `pub fn status(&self) -> StatusCode`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(inner: FailedToDeserializePathParams) -> Self`
- `fn into_response(self) -> Response`

### struct axum::extract::path::InvalidUtf8InPathParam

```rust
pub struct InvalidUtf8InPathParam { /* private fields */ }
```

Rejection used by `RawPathParams` if a parameter contained text that, once percent decoded,
wasn’t valid UTF-8.

**Methods**

- `pub fn body_text(&self) -> String`
- `pub fn status(&self) -> StatusCode`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(inner: InvalidUtf8InPathParam) -> Self`
- `fn into_response(self) -> Response`

### struct axum::extract::path::Path

```rust
pub struct Path<T>(pub T);
```

Extractor that will get captures from the URL and parse them using
`serde`.

Any percent encoded parameters will be automatically decoded. The decoded
parameters must be valid UTF-8, otherwise `Path` will fail and return a `400 Bad Request` response.

**`Option<Path<T>>` behavior**

You can use `Option<Path<T>>` as an extractor to allow the same handler to
be used in a route with parameters that deserialize to `T`, and another
route with no parameters at all.

**Example**

These examples assume the `serde` feature of the `uuid` crate is enabled.

One `Path` can extract multiple captures. It is not necessary (and does
not work) to give a handler more than one `Path` argument.

```rust
use axum::{
    extract::Path,
    routing::get,
    Router,
};
use uuid::Uuid;

async fn users_teams_show(
    Path((user_id, team_id)): Path<(Uuid, Uuid)>,
) {
    // ...
}

let app = Router::new().route("/users/{user_id}/team/{team_id}", get(users_teams_show));
```

If the path contains only one parameter, then you can omit the tuple.

```rust
use axum::{
    extract::Path,
    routing::get,
    Router,
};
use uuid::Uuid;

async fn user_info(Path(user_id): Path<Uuid>) {
    // ...
}

let app = Router::new().route("/users/{user_id}", get(user_info));
```

Path segments also can be deserialized into any type that implements
`serde::Deserialize`. This includes tuples and structs:

...

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deref(&self) -> &Self::Target`
- `fn deref_mut(&mut self) -> &mut Self::Target`
- `async fn from_request_parts( parts: &mut Parts, _state: &S, ) -> Result<Self, Self::Rejection>`
- `async fn from_request_parts( parts: &mut Parts, _state: &S, ) -> Result<Option<Self>, Self::Rejection>`

### struct axum::extract::path::RawPathParams

```rust
pub struct RawPathParams(/* private fields */);
```

Extractor that will get captures from the URL without deserializing them.

In general you should prefer to use `Path` as it is higher level, however `RawPathParams` is
suitable if just want the raw params without deserializing them and thus saving some
allocations.

Any percent encoded parameters will be automatically decoded. The decoded parameters must be
valid UTF-8, otherwise `RawPathParams` will fail and return a `400 Bad Request` response.

**Example**

```rust
use axum::{
    extract::RawPathParams,
    routing::get,
    Router,
};

async fn users_teams_show(params: RawPathParams) {
    for (key, value) in ¶ms {
        println!("{key:?} = {value:?}");
    }
}

let app = Router::new().route("/users/{user_id}/team/{team_id}", get(users_teams_show));
```

**Methods**

- `pub fn iter(&self) -> RawPathParamsIter<'_>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `async fn from_request_parts( parts: &mut Parts, _state: &S, ) -> Result<Self, Self::Rejection>`
- `fn into_iter(self) -> Self::IntoIter`

### struct axum::extract::path::RawPathParamsIter

```rust
pub struct RawPathParamsIter<'a>(/* private fields */);
```

An iterator over raw path parameters.

Created with `RawPathParams::iter`.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn next(&mut self) -> Option<Self::Item>`
- `fn next_chunk<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,`
- `1.0.0 (const: unstable) · fn size_hint(&self) -> (usize, Option<usize>)`
- `1.0.0 (const: unstable) · fn count(self) -> usizewhere Self: Sized,`
- `1.0.0 (const: unstable) · fn last(self) -> Option<Self::Item>where Self: Sized,`
- `fn advance_by(&mut self, n: usize) -> Result<(), NonZero<usize>>`
- `1.0.0 (const: unstable) · fn nth(&mut self, n: usize) -> Option<Self::Item>`
- `1.28.0 (const: unstable) · fn step_by(self, step: usize) -> StepBy<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn chain<U>(self, other: U) -> Chain<Self, <U as IntoIterator>::IntoIter> ⓘwhere Self: Sized, U: IntoIterator<Item = Self::Item>,`
- `1.0.0 (const: unstable) · fn zip<U>(self, other: U) -> Zip<Self, <U as IntoIterator>::IntoIter> ⓘwhere Self: Sized, U: IntoIterator,`
- `fn intersperse(self, separator: Self::Item) -> Intersperse<Self> ⓘwhere Self: Sized, Self::Item: Clone,`
- `fn intersperse_with<G>(self, separator: G) -> IntersperseWith<Self, G> ⓘwhere Self: Sized, G: FnMut() -> Self::Item,`
- `1.0.0 (const: unstable) · fn map<B, F>(self, f: F) -> Map<Self, F> ⓘwhere Self: Sized, F: FnMut(Self::Item) -> B,`
- `1.21.0 (const: unstable) · fn for_each<F>(self, f: F)where Self: Sized, F: FnMut(Self::Item),`
- `1.0.0 (const: unstable) · fn filter<P>(self, predicate: P) -> Filter<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.0.0 (const: unstable) · fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F> ⓘwhere Self: Sized, F: FnMut(Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn enumerate(self) -> Enumerate<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn peekable(self) -> Peekable<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.0.0 (const: unstable) · fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.57.0 (const: unstable) · fn map_while<B, P>(self, predicate: P) -> MapWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn skip(self, n: usize) -> Skip<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn take(self, n: usize) -> Take<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn scan<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F> ⓘwhere Self: Sized, F: FnMut(&mut St, Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F> ⓘwhere Self: Sized, U: IntoIterator, F: FnMut(Self::Item) -> U,`
- `1.29.0 (const: unstable) · fn flatten(self) -> Flatten<Self> ⓘwhere Self: Sized, Self::Item: IntoIterator,`
- `fn map_windows<F, R, const N: usize>(self, f: F) -> MapWindows<Self, F, N> ⓘwhere Self: Sized, F: FnMut(&[Self::Item; N]) -> R,`
- `1.0.0 (const: unstable) · fn fuse(self) -> Fuse<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn inspect<F>(self, f: F) -> Inspect<Self, F> ⓘwhere Self: Sized, F: FnMut(&Self::Item),`
- `1.0.0 (const: unstable) · fn by_ref(&mut self) -> &mut Selfwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn collect<B>(self) -> Bwhere B: FromIterator<Self::Item>, Self: Sized,`
- `fn try_collect<B>( &mut self, ) -> <<Self::Item as Try>::Residual as Residual<B>>::TryTypewhere Self: Sized, Self::Item: Try, <Self::Item as Try>::Residual: Residual<B>, B: FromIterator<<Self::Item as Try>::Output>,`
- `fn collect_into<E>(self, collection: &mut E) -> &mut Ewhere E: Extend<Self::Item>, Self: Sized,`
- `1.0.0 (const: unstable) · fn partition<B, F>(self, f: F) -> (B, B)where Self: Sized, B: Default + Extend<Self::Item>, F: FnMut(&Self::Item) -> bool,`
- `fn is_partitioned<P>(self, predicate: P) -> boolwhere Self: Sized, P: FnMut(Self::Item) -> bool,`
- `1.27.0 (const: unstable) · fn try_fold<B, F, R>(&mut self, init: B, f: F) -> Rwhere Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Output = B>,`
- `1.27.0 (const: unstable) · fn try_for_each<F, R>(&mut self, f: F) -> Rwhere Self: Sized, F: FnMut(Self::Item) -> R, R: Try<Output = ()>,`
- `1.0.0 (const: unstable) · fn fold<B, F>(self, init: B, f: F) -> Bwhere Self: Sized, F: FnMut(B, Self::Item) -> B,`
- `1.51.0 (const: unstable) · fn reduce<F>(self, f: F) -> Option<Self::Item>where Self: Sized, F: FnMut(Self::Item, Self::Item) -> Self::Item,`
