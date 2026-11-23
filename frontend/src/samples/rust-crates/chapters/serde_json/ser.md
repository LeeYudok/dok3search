# Module serde_json::ser

## Overview

Serialize a Rust data structure into JSON data.

## Items

- `CompactFormatter` — This structure compacts a JSON value with no extra whitespace.
- `PrettyFormatter` — This structure pretty prints a JSON value to make it human readable.
- `Serializer` — A structure for serializing Rust values into JSON.
- `CharEscape` — Represents a character escape code in a type-safe manner.
- `Formatter` — This trait abstracts away serializing the JSON control characters, which allows the user to optionally pretty print the JSON output.
- `to_string` — Serialize the given data structure as a String of JSON.
- `to_string_pretty` — Serialize the given data structure as a pretty-printed String of JSON.
- `to_vec` — Serialize the given data structure as a JSON byte vector.
- `to_vec_pretty` — Serialize the given data structure as a pretty-printed JSON byte vector.
- `to_writer` — Serialize the given data structure as JSON into the I/O stream.
- `to_writer_pretty` — Serialize the given data structure as pretty-printed JSON into the I/O stream.

### enum serde_json::ser::CharEscape

```rust
pub enum CharEscape {
    Quote,
    ReverseSolidus,
    Solidus,
    Backspace,
    FormFeed,
    LineFeed,
    CarriageReturn,
    Tab,
    AsciiControl(u8),
}
```

Represents a character escape code in a type-safe manner.

### fn serde_json::ser::to_string

```rust
pub fn to_string<T>(value: &T) -> Result<String>where
    T: ?Sized + Serialize,
```

Serialize the given data structure as a String of JSON.

**Errors**

Serialization can fail if `T`’s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

### fn serde_json::ser::to_string_pretty

```rust
pub fn to_string_pretty<T>(value: &T) -> Result<String>where
    T: ?Sized + Serialize,
```

Serialize the given data structure as a pretty-printed String of JSON.

**Errors**

Serialization can fail if `T`’s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

### fn serde_json::ser::to_vec

```rust
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>where
    T: ?Sized + Serialize,
```

Serialize the given data structure as a JSON byte vector.

**Errors**

Serialization can fail if `T`’s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

### fn serde_json::ser::to_vec_pretty

```rust
pub fn to_vec_pretty<T>(value: &T) -> Result<Vec<u8>>where
    T: ?Sized + Serialize,
```

Serialize the given data structure as a pretty-printed JSON byte vector.

**Errors**

Serialization can fail if `T`’s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

### fn serde_json::ser::to_writer

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

### fn serde_json::ser::to_writer_pretty

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

### struct serde_json::ser::CompactFormatter

```rust
pub struct CompactFormatter;
```

This structure compacts a JSON value with no extra whitespace.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn write_null<W>(&mut self, writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn write_bool<W>(&mut self, writer: &mut W, value: bool) -> Result<()>where W: ?Sized + Write,`
- `fn write_i8<W>(&mut self, writer: &mut W, value: i8) -> Result<()>where W: ?Sized + Write,`
- `fn write_i16<W>(&mut self, writer: &mut W, value: i16) -> Result<()>where W: ?Sized + Write,`
- `fn write_i32<W>(&mut self, writer: &mut W, value: i32) -> Result<()>where W: ?Sized + Write,`
- `fn write_i64<W>(&mut self, writer: &mut W, value: i64) -> Result<()>where W: ?Sized + Write,`
- `fn write_i128<W>(&mut self, writer: &mut W, value: i128) -> Result<()>where W: ?Sized + Write,`
- `fn write_u8<W>(&mut self, writer: &mut W, value: u8) -> Result<()>where W: ?Sized + Write,`
- `fn write_u16<W>(&mut self, writer: &mut W, value: u16) -> Result<()>where W: ?Sized + Write,`
- `fn write_u32<W>(&mut self, writer: &mut W, value: u32) -> Result<()>where W: ?Sized + Write,`
- `fn write_u64<W>(&mut self, writer: &mut W, value: u64) -> Result<()>where W: ?Sized + Write,`
- `fn write_u128<W>(&mut self, writer: &mut W, value: u128) -> Result<()>where W: ?Sized + Write,`
- `fn write_f32<W>(&mut self, writer: &mut W, value: f32) -> Result<()>where W: ?Sized + Write,`
- `fn write_f64<W>(&mut self, writer: &mut W, value: f64) -> Result<()>where W: ?Sized + Write,`
- `fn write_number_str<W>(&mut self, writer: &mut W, value: &str) -> Result<()>where W: ?Sized + Write,`
- `fn begin_string<W>(&mut self, writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn end_string<W>(&mut self, writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn write_string_fragment<W>( &mut self, writer: &mut W, fragment: &str, ) -> Result<()>where W: ?Sized + Write,`
- `fn write_char_escape<W>( &mut self, writer: &mut W, char_escape: CharEscape, ) -> Result<()>where W: ?Sized + Write,`
- `fn write_byte_array<W>(&mut self, writer: &mut W, value: &[u8]) -> Result<()>where W: ?Sized + Write,`
- `fn begin_array<W>(&mut self, writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn end_array<W>(&mut self, writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> Result<()>where W: ?Sized + Write,`
- `fn end_array_value<W>(&mut self, _writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn begin_object<W>(&mut self, writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn end_object<W>(&mut self, writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> Result<()>where W: ?Sized + Write,`
- `fn end_object_key<W>(&mut self, _writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn begin_object_value<W>(&mut self, writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn end_object_value<W>(&mut self, _writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn write_raw_fragment<W>( &mut self, writer: &mut W, fragment: &str, ) -> Result<()>where W: ?Sized + Write,`

### struct serde_json::ser::PrettyFormatter

```rust
pub struct PrettyFormatter<'a> { /* private fields */ }
```

This structure pretty prints a JSON value to make it human readable.

**Methods**

- `pub fn new() -> Self`
- `pub fn with_indent(indent: &'a [u8]) -> Self`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn begin_array<W>(&mut self, writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn end_array<W>(&mut self, writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> Result<()>where W: ?Sized + Write,`
- `fn end_array_value<W>(&mut self, _writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn begin_object<W>(&mut self, writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn end_object<W>(&mut self, writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> Result<()>where W: ?Sized + Write,`
- `fn begin_object_value<W>(&mut self, writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn end_object_value<W>(&mut self, _writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn write_null<W>(&mut self, writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn write_bool<W>(&mut self, writer: &mut W, value: bool) -> Result<()>where W: ?Sized + Write,`
- `fn write_i8<W>(&mut self, writer: &mut W, value: i8) -> Result<()>where W: ?Sized + Write,`
- `fn write_i16<W>(&mut self, writer: &mut W, value: i16) -> Result<()>where W: ?Sized + Write,`
- `fn write_i32<W>(&mut self, writer: &mut W, value: i32) -> Result<()>where W: ?Sized + Write,`
- `fn write_i64<W>(&mut self, writer: &mut W, value: i64) -> Result<()>where W: ?Sized + Write,`
- `fn write_i128<W>(&mut self, writer: &mut W, value: i128) -> Result<()>where W: ?Sized + Write,`
- `fn write_u8<W>(&mut self, writer: &mut W, value: u8) -> Result<()>where W: ?Sized + Write,`
- `fn write_u16<W>(&mut self, writer: &mut W, value: u16) -> Result<()>where W: ?Sized + Write,`
- `fn write_u32<W>(&mut self, writer: &mut W, value: u32) -> Result<()>where W: ?Sized + Write,`
- `fn write_u64<W>(&mut self, writer: &mut W, value: u64) -> Result<()>where W: ?Sized + Write,`
- `fn write_u128<W>(&mut self, writer: &mut W, value: u128) -> Result<()>where W: ?Sized + Write,`
- `fn write_f32<W>(&mut self, writer: &mut W, value: f32) -> Result<()>where W: ?Sized + Write,`
- `fn write_f64<W>(&mut self, writer: &mut W, value: f64) -> Result<()>where W: ?Sized + Write,`
- `fn write_number_str<W>(&mut self, writer: &mut W, value: &str) -> Result<()>where W: ?Sized + Write,`
- `fn begin_string<W>(&mut self, writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn end_string<W>(&mut self, writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn write_string_fragment<W>( &mut self, writer: &mut W, fragment: &str, ) -> Result<()>where W: ?Sized + Write,`
- `fn write_char_escape<W>( &mut self, writer: &mut W, char_escape: CharEscape, ) -> Result<()>where W: ?Sized + Write,`
- `fn write_byte_array<W>(&mut self, writer: &mut W, value: &[u8]) -> Result<()>where W: ?Sized + Write,`
- `fn end_object_key<W>(&mut self, _writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn write_raw_fragment<W>( &mut self, writer: &mut W, fragment: &str, ) -> Result<()>where W: ?Sized + Write,`

### struct serde_json::ser::Serializer

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

### trait serde_json::ser::Formatter

```rust
pub trait Formatter {
    // Provided methods
    fn write_null<W>(&mut self, writer: &mut W) -> Result<()>
       where W: ?Sized + Write { ... }
    fn write_bool<W>(&mut self, writer: &mut W, value: bool) -> Result<()>
       where W: ?Sized + Write { ... }
    fn write_i8<W>(&mut self, writer: &mut W, value: i8) -> Result<()>
       where W: ?Sized + Write { ... }
    fn write_i16<W>(&mut self, writer: &mut W, value: i16) -> Result<()>
       where W: ?Sized + Write { ... }
    fn write_i32<W>(&mut self, writer: &mut W, value: i32) -> Result<()>
       where W: ?Sized + Write { ... }
    fn write_i64<W>(&mut self, writer: &mut W, value: i64) -> Result<()>
       where W: ?Sized + Write { ... }
    fn write_i128<W>(&mut self, writer: &mut W, value: i128) -> Result<()>
       where W: ?Sized + Write { ... }
    fn write_u8<W>(&mut self, writer: &mut W, value: u8) -> Result<()>
       where W: ?Sized + Write { ... }
    fn write_u16<W>(&mut self, writer: &mut W, value: u16) -> Result<()>
       where W: ?Sized + Write { ... }
    fn write_u32<W>(&mut self, writer: &mut W, value: u32) -> Result<()>
       where W: ?Sized + Write { ... }
    fn write_u64<W>(&mut self, writer: &mut W, value: u64) -> Result<()>
       where W: ?Sized + Write { ... }
    fn write_u128<W>(&mut self, writer: &mut W, value: u128) -> Result<()>
       where W: ?Sized + Write { ... }
    fn write_f32<W>(&mut self, writer: &mut W, value: f32) -> Result<()>
       where W: ?Sized + Write { ... }
    fn write_f64<W>(&mut self, writer: &mut W, value: f64) -> Result<()>
       where W: ?Sized + Write { ... }
    fn write_number_str<W>(&mut self, writer: &mut W, value: &str) -> Result<()>
       where W: ?Sized + Write { ... }
    fn begin_string<W>(&mut self, writer: &mut W) -> Result<()>
       where W: ?Sized + Write { ... }
    fn end_string<W>(&mut self, writer: &mut W) -> Result<()>
       where W: ?Sized + Write { ... }
    fn write_string_fragment<W>(
        &mut self,
        writer: &mut W,
        fragment: &str,
    ) -> Result<()>
       where W: ?Sized + Write { ... }
    fn write_char_escape<W>(
        &mut self,
        writer: &mut W,
        char_escape: CharEscape,
    ) -> Result<()>
       where W: ?Sized + Write { ... }
    fn write_byte_array<W>(
        &mut self,
        writer: &mut W,
        value: &[u8],
    ) -> Result<()>
       where W: ?Sized + Write { ... }
    fn begin_array<W>(&mut self, writer: &mut W) -> Result<()>
       where W: ?Sized + Write { ... }
    fn end_array<W>(&mut self, writer: &mut W) -> Result<()>
       where W: ?Sized + Write { ... }
    fn begin_array_value<W>(
        &mut self,
        writer: &mut W,
        first: bool,
    ) -> Result<()>
       where W: ?Sized + Write { ... }
    fn end_array_value<W>(&mut self, _writer: &mut W) -> Result<()>
       where W: ?Sized + Write { ... }
    fn begin_object<W>(&mut self, writer: &mut W) -> Result<()>
       where W: ?Sized + Write { ... }
    fn end_object<W>(&mut self, writer: &mut W) -> Result<()>
       where W: ?Sized + Write { ... }
    fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> Result<()>
       where W: ?Sized + Write { ... }
    fn end_object_key<W>(&mut self, _writer: &mut W) -> Result<()>
       where W: ?Sized + Write { ... }
    fn begin_object_value<W>(&mut self, writer: &mut W) -> Result<()>
       where W: ?Sized + Write { ... }
    fn end_object_value<W>(&mut self, _writer: &mut W) -> Result<()>
       where W: ?Sized + Write { ... }
    fn write_raw_fragment<W>(
        &mut self,
        writer: &mut W,
        fragment: &str,
    ) -> Result<()>
       where W: ?Sized + Write { ... }
}
```

This trait abstracts away serializing the JSON control characters, which allows the user to
optionally pretty print the JSON output.

**Methods**

- `fn write_null<W>(&mut self, writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn write_bool<W>(&mut self, writer: &mut W, value: bool) -> Result<()>where W: ?Sized + Write,`
- `fn write_i8<W>(&mut self, writer: &mut W, value: i8) -> Result<()>where W: ?Sized + Write,`
- `fn write_i16<W>(&mut self, writer: &mut W, value: i16) -> Result<()>where W: ?Sized + Write,`
- `fn write_i32<W>(&mut self, writer: &mut W, value: i32) -> Result<()>where W: ?Sized + Write,`
- `fn write_i64<W>(&mut self, writer: &mut W, value: i64) -> Result<()>where W: ?Sized + Write,`
- `fn write_i128<W>(&mut self, writer: &mut W, value: i128) -> Result<()>where W: ?Sized + Write,`
- `fn write_u8<W>(&mut self, writer: &mut W, value: u8) -> Result<()>where W: ?Sized + Write,`
- `fn write_u16<W>(&mut self, writer: &mut W, value: u16) -> Result<()>where W: ?Sized + Write,`
- `fn write_u32<W>(&mut self, writer: &mut W, value: u32) -> Result<()>where W: ?Sized + Write,`
- `fn write_u64<W>(&mut self, writer: &mut W, value: u64) -> Result<()>where W: ?Sized + Write,`
- `fn write_u128<W>(&mut self, writer: &mut W, value: u128) -> Result<()>where W: ?Sized + Write,`
- `fn write_f32<W>(&mut self, writer: &mut W, value: f32) -> Result<()>where W: ?Sized + Write,`
- `fn write_f64<W>(&mut self, writer: &mut W, value: f64) -> Result<()>where W: ?Sized + Write,`
- `fn write_number_str<W>(&mut self, writer: &mut W, value: &str) -> Result<()>where W: ?Sized + Write,`
- `fn begin_string<W>(&mut self, writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn end_string<W>(&mut self, writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn write_string_fragment<W>( &mut self, writer: &mut W, fragment: &str, ) -> Result<()>where W: ?Sized + Write,`
- `fn write_char_escape<W>( &mut self, writer: &mut W, char_escape: CharEscape, ) -> Result<()>where W: ?Sized + Write,`
- `fn write_byte_array<W>(&mut self, writer: &mut W, value: &[u8]) -> Result<()>where W: ?Sized + Write,`
- `fn begin_array<W>(&mut self, writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn end_array<W>(&mut self, writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> Result<()>where W: ?Sized + Write,`
- `fn end_array_value<W>(&mut self, _writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn begin_object<W>(&mut self, writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn end_object<W>(&mut self, writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> Result<()>where W: ?Sized + Write,`
- `fn end_object_key<W>(&mut self, _writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn begin_object_value<W>(&mut self, writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn end_object_value<W>(&mut self, _writer: &mut W) -> Result<()>where W: ?Sized + Write,`
- `fn write_raw_fragment<W>( &mut self, writer: &mut W, fragment: &str, ) -> Result<()>where W: ?Sized + Write,`
