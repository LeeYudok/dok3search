# Crate serde_json

## Overview

**Serde JSON**

JSON is a ubiquitous open-standard format that uses human-readable text to
transmit data objects consisting of key-value pairs.

```rust
{
    "name": "John Doe",
    "age": 43,
    "address": {
        "street": "10 Downing Street",
        "city": "London"
    },
    "phones": [
        "+44 1234567",
        "+44 2345678"
    ]
}
```

There are three common ways that you might find yourself needing to work
with JSON data in Rust.

- As text data. An unprocessed string of JSON data that you receive on
an HTTP endpoint, read from a file, or prepare to send to a remote
server.

- As an untyped or loosely typed representation. Maybe you want to
check that some JSON data is valid before passing it on, but without
knowing the structure of what it contains. Or you want to do very basic
manipulations like insert a key in a particular spot.

- As a strongly typed Rust data structure. When you expect all or most
of your data to conform to a particular structure and want to get real
work done without JSON’s loosey-goosey nature tripping you up.

Serde JSON provides efficient, flexible, safe ways of converting data
between each of these representations.

**Operating on untyped JSON values**

Any valid JSON data can be manipulated in the following recursive enum
representation. This data structure is `serde_json::Value`.

```rust
enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(Map<String, Value>),
}
```

A string of JSON data can be parsed into a `serde_json::Value` by the
`serde_json::from_str` function. There is also `from_slice`
for parsing from a byte slice `&[u8]` and `from_reader` for parsing from
any `io::Read` like a File or a TCP stream.

```rust
use serde_json::{Result, Value};

fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}
```

The result of square bracket indexing li

...

## Items

- `de` — Deserialize JSON data to a Rust data structure.
- `error` — When serializing or deserializing JSON goes wrong.
- `map` — A map of String to serde_json::Value.
- `ser` — Serialize a Rust data structure into JSON data.
- `value` — The Value enum, a loosely typed way of representing any valid JSON value.
- `json` — Construct a serde_json::Value from a JSON literal.
- `Deserializer` — A structure that deserializes JSON into Rust values.
- `Error` — This type represents all possible errors that can occur when serializing or deserializing JSON data.
- `Map` — Represents a JSON key/value type.
- `Number` — Represents a JSON number, whether integer or floating point.
- `Serializer` — A structure for serializing Rust values into JSON.
- `StreamDeserializer` — Iterator that deserializes a stream into multiple JSON values.
- `Value` — Represents any valid JSON value.
- `from_reader` — Deserialize an instance of type T from an I/O stream of JSON.
- `from_slice` — Deserialize an instance of type T from bytes of JSON text.
- `from_str` — Deserialize an instance of type T from a string of JSON text.
- `from_value` — Interpret a serde_json::Value as an instance of type T.
- `to_string` — Serialize the given data structure as a String of JSON.
- `to_string_pretty` — Serialize the given data structure as a pretty-printed String of JSON.
- `to_value` — Convert a T into serde_json::Value which is an enum that can represent any valid JSON data.
- `to_vec` — Serialize the given data structure as a JSON byte vector.
- `to_vec_pretty` — Serialize the given data structure as a pretty-printed JSON byte vector.
- `to_writer` — Serialize the given data structure as JSON into the I/O stream.
- `to_writer_pretty` — Serialize the given data structure as pretty-printed JSON into the I/O stream.
- `Result` — Alias for a Result with the error type serde_json::Error.

### enum serde_json::Value

```rust
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(Map<String, Value>),
}
```

Represents any valid JSON value.

See the `serde_json::value` module documentation for usage examples.

**Methods**

- `pub fn get<I: Index>(&self, index: I) -> Option<&Value>`
- `pub fn get_mut<I: Index>(&mut self, index: I) -> Option<&mut Value>`
- `pub fn is_object(&self) -> bool`
- `pub fn as_object(&self) -> Option<&Map<String, Value>>`
- `pub fn as_object_mut(&mut self) -> Option<&mut Map<String, Value>>`
- `pub fn is_array(&self) -> bool`
- `pub fn as_array(&self) -> Option<&Vec<Value>>`
- `pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>>`
- `pub fn is_string(&self) -> bool`
- `pub fn as_str(&self) -> Option<&str>`
- `pub fn is_number(&self) -> bool`
- `pub fn as_number(&self) -> Option<&Number>`
- `pub fn is_i64(&self) -> bool`
- `pub fn is_u64(&self) -> bool`
- `pub fn is_f64(&self) -> bool`
- `pub fn as_i64(&self) -> Option<i64>`
- `pub fn as_u64(&self) -> Option<u64>`
- `pub fn as_f64(&self) -> Option<f64>`
- `pub fn is_boolean(&self) -> bool`
- `pub fn as_bool(&self) -> Option<bool>`
- `pub fn is_null(&self) -> bool`
- `pub fn as_null(&self) -> Option<()>`
- `pub fn pointer(&self, pointer: &str) -> Option<&Value>`
- `pub fn pointer_mut(&mut self, pointer: &str) -> Option<&mut Value>`
- `pub fn take(&mut self) -> Value`
- `pub fn sort_all_objects(&mut self)`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result`
- `fn default() -> Value`
- `fn default() -> Self`
- `fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>where D: Deserializer<'de>,`
- `fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`

### fn serde_json::from_reader

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

### fn serde_json::from_slice

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

### fn serde_json::from_str

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

### fn serde_json::from_value

```rust
pub fn from_value<T>(value: Value) -> Result<T, Error>where
    T: DeserializeOwned,
```

Interpret a `serde_json::Value` as an instance of type `T`.

**Example**

```rust
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

fn main() {
    // The type of `j` is `serde_json::Value`
    let j = json!({
        "fingerprint": "0xF9BA143B95FF6D82",
        "location": "Menlo Park, CA"
    });

    let u: User = serde_json::from_value(j).unwrap();
    println!("{:#?}", u);
}
```

**Errors**

This conversion can fail if the structure of the Value does not match the
structure expected by `T`, for example if `T` is a struct type but the Value
contains something other than a JSON map. It can also fail if the structure
is correct but `T`’s implementation of `Deserialize` decides that something
is wrong with the data, for example required struct fields are missing from
the JSON map or some number is too big to fit in the expected primitive
type.

### fn serde_json::to_string

```rust
pub fn to_string<T>(value: &T) -> Result<String>where
    T: ?Sized + Serialize,
```

Serialize the given data structure as a String of JSON.

**Errors**

Serialization can fail if `T`’s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

### fn serde_json::to_string_pretty

```rust
pub fn to_string_pretty<T>(value: &T) -> Result<String>where
    T: ?Sized + Serialize,
```

Serialize the given data structure as a pretty-printed String of JSON.

**Errors**

Serialization can fail if `T`’s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

### fn serde_json::to_value

```rust
pub fn to_value<T>(value: T) -> Result<Value, Error>where
    T: Serialize,
```

Convert a `T` into `serde_json::Value` which is an enum that can represent
any valid JSON data.

**Example**

```rust
use serde::Serialize;
use serde_json::json;
use std::error::Error;

#[derive(Serialize)]
struct User {
    fingerprint: String,
    location: String,
}

fn compare_json_values() -> Result<(), Box<dyn Error>> {
    let u = User {
        fingerprint: "0xF9BA143B95FF6D82".to_owned(),
        location: "Menlo Park, CA".to_owned(),
    };

    // The type of `expected` is `serde_json::Value`
    let expected = json!({
        "fingerprint": "0xF9BA143B95FF6D82",
        "location": "Menlo Park, CA",
    });

    let v = serde_json::to_value(u).unwrap();
    assert_eq!(v, expected);

    Ok(())
}
```

**Errors**

This conversion can fail if `T`’s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

```rust
use std::collections::BTreeMap;

fn main() {
    // The keys in this map are vectors, not strings.
    let mut map = BTreeMap::new();
    map.insert(vec![32, 64], "x86");

    println!("{}", serde_json::to_value(map).unwrap_err());
}
```

### fn serde_json::to_vec

```rust
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>where
    T: ?Sized + Serialize,
```

Serialize the given data structure as a JSON byte vector.

**Errors**

Serialization can fail if `T`’s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

### fn serde_json::to_vec_pretty

```rust
pub fn to_vec_pretty<T>(value: &T) -> Result<Vec<u8>>where
    T: ?Sized + Serialize,
```

Serialize the given data structure as a pretty-printed JSON byte vector.

**Errors**

Serialization can fail if `T`’s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

### fn serde_json::to_writer

```rust
pub fn to_writer<W, T>(writer: W, value: &T) -> Result<()>where
    W: Write,
    T: ?Sized + Serialize,
```

Serialize the given data structure as JSON into the I/O stream.

Serialization guarantees it only feeds valid UTF-8 sequences to the writer.

**Errors**

Serialization can fail if `T`’s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

### fn serde_json::to_writer_pretty

```rust
pub fn to_writer_pretty<W, T>(writer: W, value: &T) -> Result<()>where
    W: Write,
    T: ?Sized + Serialize,
```

Serialize the given data structure as pretty-printed JSON into the I/O
stream.

Serialization guarantees it only feeds valid UTF-8 sequences to the writer.

**Errors**

Serialization can fail if `T`’s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

### macro serde_json::json

```rust
macro_rules! json {
    ($($json:tt)+) => { ... };
}
```

Construct a `serde_json::Value` from a JSON literal.

```rust
let value = json!({
    "code": 200,
    "success": true,
    "payload": {
        "features": [
            "serde",
            "json"
        ],
        "homepage": null
    }
});
```

Variables or expressions can be interpolated into the JSON literal. Any type
interpolated into an array element or object value must implement Serde’s
`Serialize` trait, while any type interpolated into an object key must
implement `Into<String>`. If the `Serialize` implementation of the
interpolated type decides to fail, or if the interpolated type contains a
map with non-string keys, the `json!` macro will panic.

```rust
let code = 200;
let features = vec!["serde", "json"];

let value = json!({
    "code": code,
    "success": code == 200,
    "payload": {
        features[0]: features[1]
    }
});
```

Trailing commas are allowed inside both arrays and objects.

```rust
let value = json!([
    "notice",
    "the",
    "trailing",
    "comma -->",
]);
```

### struct serde_json::Deserializer

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

### struct serde_json::Error

```rust
pub struct Error { /* private fields */ }
```

This type represents all possible errors that can occur when serializing or
deserializing JSON data.

**Methods**

- `pub fn line(&self) -> usize`
- `pub fn column(&self) -> usize`
- `pub fn classify(&self) -> Category`
- `pub fn is_io(&self) -> bool`
- `pub fn is_syntax(&self) -> bool`
- `pub fn is_data(&self) -> bool`
- `pub fn is_eof(&self) -> bool`
- `pub fn io_error_kind(&self) -> Option<ErrorKind>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn custom<T: Display>(msg: T) -> Error`
- `fn invalid_type(unexp: Unexpected<'_>, exp: &dyn Expected) -> Self`
- `fn invalid_value(unexp: Unexpected<'_>, exp: &dyn Expected) -> Self`
- `fn invalid_length(len: usize, exp: &dyn Expected) -> Self`
- `fn unknown_variant(variant: &str, expected: &'static [&'static str]) -> Self`
- `fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self`
- `fn missing_field(field: &'static str) -> Self`
- `fn duplicate_field(field: &'static str) -> Self`
- `fn custom<T: Display>(msg: T) -> Error`
- `fn from(j: Error) -> Self`
- `fn into_deserializer(self) -> Self::Deserializer`
- `fn into_deserializer(self) -> Self::Deserializer`
- `fn into_deserializer(self) -> Self::Deserializer`
- `fn into_deserializer(self) -> Self::Deserializer`
- `fn into_deserializer(self) -> Self::Deserializer`

### struct serde_json::Map

```rust
pub struct Map<K, V> { /* private fields */ }
```

Represents a JSON key/value type.

**Methods**

- `pub fn new() -> Self`
- `pub fn with_capacity(capacity: usize) -> Self`
- `pub fn clear(&mut self)`
- `pub fn get<Q>(&self, key: &Q) -> Option<&Value>where String: Borrow<Q>, Q: ?Sized + Ord + Eq + Hash,`
- `pub fn contains_key<Q>(&self, key: &Q) -> boolwhere String: Borrow<Q>, Q: ?Sized + Ord + Eq + Hash,`
- `pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut Value>where String: Borrow<Q>, Q: ?Sized + Ord + Eq + Hash,`
- `pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&String, &Value)>where String: Borrow<Q>, Q: ?Sized + Ord + Eq + Hash,`
- `pub fn insert(&mut self, k: String, v: Value) -> Option<Value>`
- `pub fn remove<Q>(&mut self, key: &Q) -> Option<Value>where String: Borrow<Q>, Q: ?Sized + Ord + Eq + Hash,`
- `pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(String, Value)>where String: Borrow<Q>, Q: ?Sized + Ord + Eq + Hash,`
- `pub fn append(&mut self, other: &mut Self)`
- `pub fn entry<S>(&mut self, key: S) -> Entry<'_>where S: Into<String>,`
- `pub fn len(&self) -> usize`
- `pub fn is_empty(&self) -> bool`
- `pub fn iter(&self) -> Iter<'_>`
- `pub fn iter_mut(&mut self) -> IterMut<'_>`
- `pub fn keys(&self) -> Keys<'_>`
- `pub fn values(&self) -> Values<'_>`
- `pub fn values_mut(&mut self) -> ValuesMut<'_>`
- `pub fn into_values(self) -> IntoValues`
- `pub fn retain<F>(&mut self, f: F)where F: FnMut(&String, &mut Value) -> bool,`
- `pub fn sort_keys(&mut self)`
- `fn clone(&self) -> Self`
- `fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn default() -> Self`
- `fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>where D: Deserializer<'de>,`
- `fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, _name: &'static str, _variants: &'static [&'static str], visitor: V, ) -> Result<V::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`

### struct serde_json::Number

```rust
pub struct Number { /* private fields */ }
```

Represents a JSON number, whether integer or floating point.

**Methods**

- `pub fn is_i64(&self) -> bool`
- `pub fn is_u64(&self) -> bool`
- `pub fn is_f64(&self) -> bool`
- `pub fn as_i64(&self) -> Option<i64>`
- `pub fn as_u64(&self) -> Option<u64>`
- `pub fn as_f64(&self) -> Option<f64>`
- `pub fn from_f64(f: f64) -> Option<Number>`
- `pub fn as_i128(&self) -> Option<i128>`
- `pub fn as_u128(&self) -> Option<u128>`
- `pub fn from_i128(i: i128) -> Option<Number>`
- `pub fn from_u128(i: u128) -> Option<Number>`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result`
- `fn deserialize<D>(deserializer: D) -> Result<Number, D::Error>where D: Deserializer<'de>,`
- `fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`

### struct serde_json::Serializer

```rust
pub struct Serializer<W, F = CompactFormatter> { /* private fields */ }
```

A structure for serializing Rust values into JSON.

**Methods**

- `pub fn new(writer: W) -> Self`
- `pub fn pretty(writer: W) -> Self`
- `pub fn with_formatter(writer: W, formatter: F) -> Self`
- `pub fn into_inner(self) -> W`
- `fn serialize_newtype_struct<T>( self, _name: &'static str, value: &T, ) -> Result<()>where T: ?Sized + Serialize,`
- `fn serialize_bool(self, value: bool) -> Result<()>`
- `fn serialize_i8(self, value: i8) -> Result<()>`
- `fn serialize_i16(self, value: i16) -> Result<()>`
- `fn serialize_i32(self, value: i32) -> Result<()>`
- `fn serialize_i64(self, value: i64) -> Result<()>`
- `fn serialize_i128(self, value: i128) -> Result<()>`
- `fn serialize_u8(self, value: u8) -> Result<()>`
- `fn serialize_u16(self, value: u16) -> Result<()>`
- `fn serialize_u32(self, value: u32) -> Result<()>`
- `fn serialize_u64(self, value: u64) -> Result<()>`
- `fn serialize_u128(self, value: u128) -> Result<()>`
- `fn serialize_f32(self, value: f32) -> Result<()>`
- `fn serialize_f64(self, value: f64) -> Result<()>`
- `fn serialize_char(self, value: char) -> Result<()>`
- `fn serialize_str(self, value: &str) -> Result<()>`
- `fn serialize_bytes(self, value: &[u8]) -> Result<()>`
- `fn serialize_unit(self) -> Result<()>`
- `fn serialize_unit_struct(self, _name: &'static str) -> Result<()>`
- `fn serialize_unit_variant( self, _name: &'static str, _variant_index: u32, variant: &'static str, ) -> Result<()>`
- `fn serialize_newtype_variant<T>( self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T, ) -> Result<()>where T: ?Sized + Serialize,`
- `fn serialize_none(self) -> Result<()>`
- `fn serialize_some<T>(self, value: &T) -> Result<()>where T: ?Sized + Serialize,`
- `fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq>`
- `fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple>`
- `fn serialize_tuple_struct( self, _name: &'static str, len: usize, ) -> Result<Self::SerializeTupleStruct>`
- `fn serialize_tuple_variant( self, _name: &'static str, _variant_index: u32, variant: &'static str, len: usize, ) -> Result<Self::SerializeTupleVariant>`
- `fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap>`
- `fn serialize_struct( self, name: &'static str, len: usize, ) -> Result<Self::SerializeStruct>`
- `fn serialize_struct_variant( self, _name: &'static str, _variant_index: u32, variant: &'static str, len: usize, ) -> Result<Self::SerializeStructVariant>`
- `fn collect_str<T>(self, value: &T) -> Result<()>where T: ?Sized + Display,`
- `fn collect_seq<I>(self, iter: I) -> Result<Self::Ok, Self::Error>where I: IntoIterator, <I as IntoIterator>::Item: Serialize,`
- `fn collect_map<K, V, I>(self, iter: I) -> Result<Self::Ok, Self::Error>where K: Serialize, V: Serialize, I: IntoIterator<Item = (K, V)>,`
- `fn is_human_readable(&self) -> bool`

### struct serde_json::StreamDeserializer

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

### type serde_json::Result

```rust
pub type Result<T> = Result<T, Error>;
```

Alias for a `Result` with the error type `serde_json::Error`.
