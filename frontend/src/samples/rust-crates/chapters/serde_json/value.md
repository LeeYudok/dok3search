# Module serde_json::value

## Overview

The Value enum, a loosely typed way of representing any valid JSON value.

**Constructing JSON**

Serde JSON provides a `json!` macro to build `serde_json::Value`
objects with very natural JSON syntax.

```rust
use serde_json::json;

fn main() {
    // The type of `john` is `serde_json::Value`
    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    println!("first phone number: {}", john["phones"][0]);

    // Convert to a string of JSON and print it out
    println!("{}", john.to_string());
}
```

The `Value::to_string()` function converts a `serde_json::Value` into a
`String` of JSON text.

One neat thing about the `json!` macro is that variables and expressions can
be interpolated directly into the JSON value as you are building it. Serde
will check at compile time that the value you are interpolating is able to
be represented as JSON.

```rust
let full_name = "John Doe";
let age_last_year = 42;

// The type of `john` is `serde_json::Value`
let john = json!({
    "name": full_name,
    "age": age_last_year + 1,
    "phones": [
        format!("+44 {}", random_phone())
    ]
});
```

A string of JSON data can be parsed into a `serde_json::Value` by the
`serde_json::from_str` function. There is also
`from_slice` for parsing from a byte slice `&[u8]` and
`from_reader` for parsing from any `io::Read` like a File or
a TCP stream.

```rust
use serde_json::{json, Value, Error};

fn untyped_example() -> Result<(), Error> {
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

## Items

- `Number` — Represents a JSON number, whether integer or floating point.
- `RawValue` — Reference to a range of bytes encompassing a single valid JSON value in the input data.
- `Serializer` — Serializer whose output is a Value.
- `Value` — Represents any valid JSON value.
- `Index` — A type that can be used to index into a serde_json::Value.
- `from_value` — Interpret a serde_json::Value as an instance of type T.
- `to_raw_value` — Convert a T into a boxed RawValue.
- `to_value` — Convert a T into serde_json::Value which is an enum that can represent any valid JSON data.

### enum serde_json::value::Value

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

### fn serde_json::value::from_value

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

### fn serde_json::value::to_raw_value

```rust
pub fn to_raw_value<T>(value: &T) -> Result<Box<RawValue>, Error>where
    T: ?Sized + Serialize,
```

Convert a `T` into a boxed `RawValue`.

**Example**

```rust
// Upstream crate
pub struct Thing {
    foo: String,
    bar: Option<String>,
    extra_data: Box<RawValue>,
}

// Local crate
use serde::Serialize;
use serde_json::value::{to_raw_value, RawValue};

#[derive(Serialize)]
struct MyExtraData {
    a: u32,
    b: u32,
}

let my_thing = Thing {
    foo: "FooVal".into(),
    bar: None,
    extra_data: to_raw_value(&MyExtraData { a: 1, b: 2 }).unwrap(),
};
```

**Errors**

This conversion can fail if `T`’s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

```rust
use std::collections::BTreeMap;

// The keys in this map are vectors, not strings.
let mut map = BTreeMap::new();
map.insert(vec![32, 64], "x86");

println!("{}", serde_json::value::to_raw_value(&map).unwrap_err());
```

### fn serde_json::value::to_value

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

### struct serde_json::value::Number

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

### struct serde_json::value::RawValue

```rust
pub struct RawValue { /* private fields */ }
```

Reference to a range of bytes encompassing a single valid JSON value in the
input data.

A `RawValue` can be used to defer parsing parts of a payload until later,
or to avoid parsing it at all in the case that part of the payload just
needs to be transferred verbatim into a different output object.

When serializing, a value of this type will retain its original formatting
and will not be minified or pretty-printed.

**Note**

`RawValue` is only available if serde_json is built with the `"raw_value"`
feature.

```rust
[dependencies]
serde_json = { version = "1.0", features = ["raw_value"] }
```

**Example**

...

**Methods**

- `pub fn from_string(json: String) -> Result<Box<Self>, Error>`
- `pub unsafe fn from_string_unchecked(json: String) -> Box<Self>`
- `pub fn get(&self) -> &str`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>where D: Deserializer<'de>,`
- `fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
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
- `fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, name: &'static str, variants: &'static [&'static str], visitor: V, ) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Error>where V: Visitor<'de>,`

### struct serde_json::value::Serializer

```rust
pub struct Serializer;
```

Serializer whose output is a `Value`.

This is the serializer that backs `serde_json::to_value`.
Unlike the main serde_json serializer which goes from some serializable
value of type `T` to JSON text, this one goes from `T` to
`serde_json::Value`.

The `to_value` function is implementable as:

```rust
use serde::Serialize;
use serde_json::{Error, Value};

pub fn to_value<T>(input: T) -> Result<Value, Error>
where
    T: Serialize,
{
    input.serialize(serde_json::value::Serializer)
}
```

**Methods**

- `fn serialize_bool(self, value: bool) -> Result<Value>`
- `fn serialize_i8(self, value: i8) -> Result<Value>`
- `fn serialize_i16(self, value: i16) -> Result<Value>`
- `fn serialize_i32(self, value: i32) -> Result<Value>`
- `fn serialize_i64(self, value: i64) -> Result<Value>`
- `fn serialize_i128(self, value: i128) -> Result<Value>`
- `fn serialize_u8(self, value: u8) -> Result<Value>`
- `fn serialize_u16(self, value: u16) -> Result<Value>`
- `fn serialize_u32(self, value: u32) -> Result<Value>`
- `fn serialize_u64(self, value: u64) -> Result<Value>`
- `fn serialize_u128(self, value: u128) -> Result<Value>`
- `fn serialize_f32(self, float: f32) -> Result<Value>`
- `fn serialize_f64(self, float: f64) -> Result<Value>`
- `fn serialize_char(self, value: char) -> Result<Value>`
- `fn serialize_str(self, value: &str) -> Result<Value>`
- `fn serialize_bytes(self, value: &[u8]) -> Result<Value>`
- `fn serialize_unit(self) -> Result<Value>`
- `fn serialize_unit_struct(self, _name: &'static str) -> Result<Value>`
- `fn serialize_unit_variant( self, _name: &'static str, _variant_index: u32, variant: &'static str, ) -> Result<Value>`
- `fn serialize_newtype_struct<T>( self, _name: &'static str, value: &T, ) -> Result<Value>where T: ?Sized + Serialize,`
- `fn serialize_newtype_variant<T>( self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T, ) -> Result<Value>where T: ?Sized + Serialize,`
- `fn serialize_none(self) -> Result<Value>`
- `fn serialize_some<T>(self, value: &T) -> Result<Value>where T: ?Sized + Serialize,`
- `fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq>`
- `fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple>`
- `fn serialize_tuple_struct( self, _name: &'static str, len: usize, ) -> Result<Self::SerializeTupleStruct>`
- `fn serialize_tuple_variant( self, _name: &'static str, _variant_index: u32, variant: &'static str, len: usize, ) -> Result<Self::SerializeTupleVariant>`
- `fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap>`
- `fn serialize_struct( self, name: &'static str, len: usize, ) -> Result<Self::SerializeStruct>`
- `fn serialize_struct_variant( self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize, ) -> Result<Self::SerializeStructVariant>`
- `fn collect_str<T>(self, value: &T) -> Result<Value>where T: ?Sized + Display,`
- `fn collect_seq<I>(self, iter: I) -> Result<Self::Ok, Self::Error>where I: IntoIterator, <I as IntoIterator>::Item: Serialize,`
- `fn collect_map<K, V, I>(self, iter: I) -> Result<Self::Ok, Self::Error>where K: Serialize, V: Serialize, I: IntoIterator<Item = (K, V)>,`
- `fn is_human_readable(&self) -> bool`

### trait serde_json::value::Index

```rust
pub trait Index: Sealed { }
```

A type that can be used to index into a `serde_json::Value`.

The `get` and `get_mut` methods of `Value` accept any type that
implements `Index`, as does the square-bracket indexing operator. This
trait is implemented for strings which are used as the index into a JSON
map, and for `usize` which is used as the index into a JSON array.

This trait is sealed and cannot be implemented for types outside of
`serde_json`.

**Examples**

```rust
let data = json!({ "inner": [1, 2, 3] });

// Data is a JSON map so it can be indexed with a string.
let inner = &data["inner"];

// Inner is a JSON array so it can be indexed with an integer.
let first = &inner[0];

assert_eq!(first, 1);
```
