# Module serde_json::de

## Overview

Deserialize JSON data to a Rust data structure.

## Items

- `Deserializer` — A structure that deserializes JSON into Rust values.
- `IoRead` — JSON input source that reads from a std::io input stream.
- `SliceRead` — JSON input source that reads from a slice of bytes.
- `StrRead` — JSON input source that reads from a UTF-8 string.
- `StreamDeserializer` — Iterator that deserializes a stream into multiple JSON values.
- `Read` — Trait used by the deserializer for iterating over input. This is manually “specialized” for iterating over &[u8]. Once feature(specialization) is stable we can use actual specialization.
- `from_reader` — Deserialize an instance of type T from an I/O stream of JSON.
- `from_slice` — Deserialize an instance of type T from bytes of JSON text.
- `from_str` — Deserialize an instance of type T from a string of JSON text.

### fn serde_json::de::from_reader

```rust
pub fn from_reader<R, T>(rdr: R) -> Result<T>where
    R: Read,
    T: DeserializeOwned,
```

Deserialize an instance of type `T` from an I/O stream of JSON.

The content of the I/O stream is deserialized directly from the stream
without being buffered in memory by serde_json.

When reading from a source against which short reads are not efficient, such
as a `File`, you will want to apply your own buffering because serde_json
will not buffer the input. See `std::io::BufReader`.

It is expected that the input stream ends after the deserialized object.
If the stream does not end, such as in the case of a persistent socket connection,
this function will not return. It is possible instead to deserialize from a prefix of an input
stream without looking for EOF by managing your own `Deserializer`.

Note that counter to intuition, this function is usually slower than
reading a file completely into memory and then applying `from_str`
or `from_slice` on it. See issue #160.

**Example**

Reading the contents of a file.

...

### fn serde_json::de::from_slice

```rust
pub fn from_slice<'a, T>(v: &'a [u8]) -> Result<T>where
    T: Deserialize<'a>,
```

Deserialize an instance of type `T` from bytes of JSON text.

**Example**

```rust
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

fn main() {
    // The type of `j` is `&[u8]`
    let j = b"
        {
            \"fingerprint\": \"0xF9BA143B95FF6D82\",
            \"location\": \"Menlo Park, CA\"
        }";

    let u: User = serde_json::from_slice(j).unwrap();
    println!("{:#?}", u);
}
```

**Errors**

This conversion can fail if the structure of the input does not match the
structure expected by `T`, for example if `T` is a struct type but the input
contains something other than a JSON map. It can also fail if the structure
is correct but `T`’s implementation of `Deserialize` decides that something
is wrong with the data, for example required struct fields are missing from
the JSON map or some number is too big to fit in the expected primitive
type.

### fn serde_json::de::from_str

```rust
pub fn from_str<'a, T>(s: &'a str) -> Result<T>where
    T: Deserialize<'a>,
```

Deserialize an instance of type `T` from a string of JSON text.

**Example**

```rust
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

fn main() {
    // The type of `j` is `&str`
    let j = "
        {
            \"fingerprint\": \"0xF9BA143B95FF6D82\",
            \"location\": \"Menlo Park, CA\"
        }";

    let u: User = serde_json::from_str(j).unwrap();
    println!("{:#?}", u);
}
```

**Errors**

This conversion can fail if the structure of the input does not match the
structure expected by `T`, for example if `T` is a struct type but the input
contains something other than a JSON map. It can also fail if the structure
is correct but `T`’s implementation of `Deserialize` decides that something
is wrong with the data, for example required struct fields are missing from
the JSON map or some number is too big to fit in the expected primitive
type.

### struct serde_json::de::Deserializer

```rust
pub struct Deserializer<R> { /* private fields */ }
```

A structure that deserializes JSON into Rust values.

**Methods**

- `pub fn new(read: R) -> Self`
- `pub fn from_reader(reader: R) -> Self`
- `pub fn from_slice(bytes: &'a [u8]) -> Self`
- `pub fn from_str(s: &'a str) -> Self`
- `pub fn end(&mut self) -> Result<()>`
- `pub fn into_iter<T>(self) -> StreamDeserializer<'de, R, T> ⓘwhere T: Deserialize<'de>,`
- `fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &str, visitor: V, ) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, _name: &str, _variants: &'static [&'static str], visitor: V, ) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, _name: &'static str, visitor: V, ) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, _name: &'static str, _len: usize, visitor: V, ) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, _name: &'static str, _fields: &'static [&'static str], visitor: V, ) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`

### struct serde_json::de::IoRead

```rust
pub struct IoRead<R>where
    R: Read,{ /* private fields */ }
```

JSON input source that reads from a std::io input stream.

**Methods**

- `pub fn new(reader: R) -> Self`

### struct serde_json::de::SliceRead

```rust
pub struct SliceRead<'a> { /* private fields */ }
```

JSON input source that reads from a slice of bytes.

**Methods**

- `pub fn new(slice: &'a [u8]) -> Self`

### struct serde_json::de::StrRead

```rust
pub struct StrRead<'a> { /* private fields */ }
```

JSON input source that reads from a UTF-8 string.

**Methods**

- `pub fn new(s: &'a str) -> Self`

### struct serde_json::de::StreamDeserializer

```rust
pub struct StreamDeserializer<'de, R, T> { /* private fields */ }
```

Iterator that deserializes a stream into multiple JSON values.

A stream deserializer can be created from any JSON deserializer using the
`Deserializer::into_iter` method.

The data can consist of any JSON value. Values need to be a self-delineating value e.g.
arrays, objects, or strings, or be followed by whitespace or a self-delineating value.

```rust
use serde_json::{Deserializer, Value};

fn main() {
    let data = "{\"k\": 3}1\"cool\"\"stuff\" 3{}  [0, 1, 2]";

    let stream = Deserializer::from_str(data).into_iter::<Value>();

    for value in stream {
        println!("{}", value.unwrap());
    }
}
```

**Methods**

- `pub fn new(read: R) -> Self`
- `pub fn byte_offset(&self) -> usize`
- `fn next(&mut self) -> Option<Result<T>>`
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

### trait serde_json::de::Read

```rust
pub trait Read<'de>: Sealed { }
```

Trait used by the deserializer for iterating over input. This is manually
“specialized” for iterating over `&[u8]`. Once feature(specialization) is
stable we can use actual specialization.

This trait is sealed and cannot be implemented for types outside of
`serde_json`.
