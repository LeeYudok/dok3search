# Module serde::de::value

## Overview

Building blocks for deserializing basic values using the `IntoDeserializer`
trait.

ⓘ
```rust
use serde::de::{value, Deserialize, IntoDeserializer};
use serde_derive::Deserialize;
use std::str::FromStr;

#[derive(Deserialize)]
enum Setting {
    On,
    Off,
}

impl FromStr for Setting {
    type Err = value::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
```

## Items

- `BoolDeserializer` — A deserializer holding a bool.
- `BorrowedBytesDeserializer` — A deserializer holding a &[u8] with a lifetime tied to another deserializer. Always calls Visitor::visit_borrowed_bytes.
- `BorrowedStrDeserializer` — A deserializer holding a &str with a lifetime tied to another deserializer.
- `BytesDeserializer` — A deserializer holding a &[u8]. Always calls Visitor::visit_bytes.
- `CharDeserializer` — A deserializer holding a char.
- `CowStrDeserializer` — A deserializer holding a Cow<str>.
- `EnumAccessDeserializer` — A deserializer holding an EnumAccess.
- `Error` — A minimal representation of all possible errors that can occur using the IntoDeserializer trait.
- `F32Deserializer` — A deserializer holding an f32.
- `F64Deserializer` — A deserializer holding an f64.
- `I8Deserializer` — A deserializer holding an i8.
- `I16Deserializer` — A deserializer holding an i16.
- `I32Deserializer` — A deserializer holding an i32.
- `I64Deserializer` — A deserializer holding an i64.
- `I128Deserializer` — A deserializer holding an i128.
- `IsizeDeserializer` — A deserializer holding an isize.
- `MapAccessDeserializer` — A deserializer holding a MapAccess.
- `MapDeserializer` — A deserializer that iterates over a map.
- `SeqAccessDeserializer` — A deserializer holding a SeqAccess.
- `SeqDeserializer` — A deserializer that iterates over a sequence.
- `StrDeserializer` — A deserializer holding a &str.
- `StringDeserializer` — A deserializer holding a String.
- `U8Deserializer` — A deserializer holding a u8.
- `U16Deserializer` — A deserializer holding a u16.
- `U32Deserializer` — A deserializer holding a u32.
- `U64Deserializer` — A deserializer holding a u64.
- `U128Deserializer` — A deserializer holding a u128.
- `UnitDeserializer` — A deserializer holding a ().
- `UsizeDeserializer` — A deserializer holding a usize.

### struct serde::de::value::BoolDeserializer

```rust
pub struct BoolDeserializer<E> { /* private fields */ }
```

A deserializer holding
a `bool`.

**Methods**

- `pub fn new(value: bool) -> BoolDeserializer<E>`
- `fn clone(&self) -> BoolDeserializer<E>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, name: &'static str, variants: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BoolDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`
- `fn into_deserializer(self) -> BoolDeserializer<E>`

### struct serde::de::value::BorrowedBytesDeserializer

```rust
pub struct BorrowedBytesDeserializer<'de, E> { /* private fields */ }
```

A deserializer holding a `&[u8]` with a lifetime tied to another
deserializer. Always calls `Visitor::visit_borrowed_bytes`.

**Methods**

- `pub fn new(value: &'de [u8]) -> BorrowedBytesDeserializer<'de, E>`
- `fn clone(&self) -> BorrowedBytesDeserializer<'de, E>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn deserialize_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, name: &'static str, variants: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedBytesDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`
- `fn into_deserializer(self) -> BorrowedBytesDeserializer<'de, E>`

### struct serde::de::value::BorrowedStrDeserializer

```rust
pub struct BorrowedStrDeserializer<'de, E> { /* private fields */ }
```

A deserializer holding a `&str` with a lifetime tied to another
deserializer.

**Methods**

- `pub fn new(value: &'de str) -> BorrowedStrDeserializer<'de, E>`
- `fn clone(&self) -> BorrowedStrDeserializer<'de, E>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn deserialize_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, name: &str, variants: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BorrowedStrDeserializer<'de, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`
- `fn variant_seed<T>( self, seed: T, ) -> Result<(<T as DeserializeSeed<'de>>::Value, <BorrowedStrDeserializer<'de, E> as EnumAccess<'de>>::Variant), <BorrowedStrDeserializer<'de, E> as EnumAccess<'de>>::Error>where T: DeserializeSeed<'de>,`
- `fn variant<V>(self) -> Result<(V, Self::Variant), Self::Error>where V: Deserialize<'de>,`
- `fn into_deserializer(self) -> BorrowedStrDeserializer<'de, E>`

### struct serde::de::value::BytesDeserializer

```rust
pub struct BytesDeserializer<'a, E> { /* private fields */ }
```

A deserializer holding a `&[u8]`. Always calls `Visitor::visit_bytes`.

**Methods**

- `pub fn new(value: &'a [u8]) -> BytesDeserializer<'a, E>`
- `fn clone(&self) -> BytesDeserializer<'a, E>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn deserialize_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, name: &'static str, variants: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <BytesDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`
- `fn into_deserializer(self) -> BytesDeserializer<'a, E>`

### struct serde::de::value::CharDeserializer

```rust
pub struct CharDeserializer<E> { /* private fields */ }
```

A deserializer holding
a `char`.

**Methods**

- `pub fn new(value: char) -> CharDeserializer<E>`
- `fn clone(&self) -> CharDeserializer<E>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, name: &'static str, variants: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CharDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`
- `fn into_deserializer(self) -> CharDeserializer<E>`

### struct serde::de::value::CowStrDeserializer

```rust
pub struct CowStrDeserializer<'a, E> { /* private fields */ }
```

A deserializer holding a `Cow<str>`.

**Methods**

- `pub fn new(value: Cow<'a, str>) -> CowStrDeserializer<'a, E>`
- `fn clone(&self) -> CowStrDeserializer<'a, E>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn deserialize_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, name: &str, variants: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <CowStrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`
- `fn variant_seed<T>( self, seed: T, ) -> Result<(<T as DeserializeSeed<'de>>::Value, <CowStrDeserializer<'a, E> as EnumAccess<'de>>::Variant), <CowStrDeserializer<'a, E> as EnumAccess<'de>>::Error>where T: DeserializeSeed<'de>,`
- `fn variant<V>(self) -> Result<(V, Self::Variant), Self::Error>where V: Deserialize<'de>,`
- `fn into_deserializer(self) -> CowStrDeserializer<'a, E>`

### struct serde::de::value::EnumAccessDeserializer

```rust
pub struct EnumAccessDeserializer<A> { /* private fields */ }
```

A deserializer holding an `EnumAccess`.

**Methods**

- `pub fn new(access: A) -> EnumAccessDeserializer<A>`
- `fn clone(&self) -> EnumAccessDeserializer<A>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn deserialize_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, name: &'static str, variants: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <EnumAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`
- `fn into_deserializer(self) -> EnumAccessDeserializer<A>`

### struct serde::de::value::Error

```rust
pub struct Error { /* private fields */ }
```

A minimal representation of all possible errors that can occur using the
`IntoDeserializer` trait.

**Methods**

- `fn clone(&self) -> Error`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn custom<T>(msg: T) -> Errorwhere T: Display,`
- `fn invalid_type(unexp: Unexpected<'_>, exp: &dyn Expected) -> Self`
- `fn invalid_value(unexp: Unexpected<'_>, exp: &dyn Expected) -> Self`
- `fn invalid_length(len: usize, exp: &dyn Expected) -> Self`
- `fn unknown_variant(variant: &str, expected: &'static [&'static str]) -> Self`
- `fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self`
- `fn missing_field(field: &'static str) -> Self`
- `fn duplicate_field(field: &'static str) -> Self`
- `fn custom<T>(msg: T) -> Errorwhere T: Display,`
- `fn description(&self) -> &str`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn eq(&self, other: &Error) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### struct serde::de::value::F32Deserializer

```rust
pub struct F32Deserializer<E> { /* private fields */ }
```

A deserializer holding
an `f32`.

**Methods**

- `pub fn new(value: f32) -> F32Deserializer<E>`
- `fn clone(&self) -> F32Deserializer<E>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, name: &'static str, variants: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`
- `fn into_deserializer(self) -> F32Deserializer<E>`

### struct serde::de::value::F64Deserializer

```rust
pub struct F64Deserializer<E> { /* private fields */ }
```

A deserializer holding
an `f64`.

**Methods**

- `pub fn new(value: f64) -> F64Deserializer<E>`
- `fn clone(&self) -> F64Deserializer<E>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, name: &'static str, variants: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <F64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`
- `fn into_deserializer(self) -> F64Deserializer<E>`

### struct serde::de::value::I128Deserializer

```rust
pub struct I128Deserializer<E> { /* private fields */ }
```

A deserializer holding
an `i128`.

**Methods**

- `pub fn new(value: i128) -> I128Deserializer<E>`
- `fn clone(&self) -> I128Deserializer<E>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, name: &'static str, variants: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`
- `fn into_deserializer(self) -> I128Deserializer<E>`

### struct serde::de::value::I16Deserializer

```rust
pub struct I16Deserializer<E> { /* private fields */ }
```

A deserializer holding
an `i16`.

**Methods**

- `pub fn new(value: i16) -> I16Deserializer<E>`
- `fn clone(&self) -> I16Deserializer<E>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, name: &'static str, variants: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`
- `fn into_deserializer(self) -> I16Deserializer<E>`

### struct serde::de::value::I32Deserializer

```rust
pub struct I32Deserializer<E> { /* private fields */ }
```

A deserializer holding
an `i32`.

**Methods**

- `pub fn new(value: i32) -> I32Deserializer<E>`
- `fn clone(&self) -> I32Deserializer<E>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, name: &'static str, variants: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`
- `fn into_deserializer(self) -> I32Deserializer<E>`

### struct serde::de::value::I64Deserializer

```rust
pub struct I64Deserializer<E> { /* private fields */ }
```

A deserializer holding
an `i64`.

**Methods**

- `pub fn new(value: i64) -> I64Deserializer<E>`
- `fn clone(&self) -> I64Deserializer<E>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, name: &'static str, variants: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`
- `fn into_deserializer(self) -> I64Deserializer<E>`

### struct serde::de::value::I8Deserializer

```rust
pub struct I8Deserializer<E> { /* private fields */ }
```

A deserializer holding
an `i8`.

**Methods**

- `pub fn new(value: i8) -> I8Deserializer<E>`
- `fn clone(&self) -> I8Deserializer<E>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, name: &'static str, variants: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <I8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`
- `fn into_deserializer(self) -> I8Deserializer<E>`

### struct serde::de::value::IsizeDeserializer

```rust
pub struct IsizeDeserializer<E> { /* private fields */ }
```

A deserializer holding
an `isize`.

**Methods**

- `pub fn new(value: isize) -> IsizeDeserializer<E>`
- `fn clone(&self) -> IsizeDeserializer<E>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, name: &'static str, variants: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <IsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`
- `fn into_deserializer(self) -> IsizeDeserializer<E>`

### struct serde::de::value::MapAccessDeserializer

```rust
pub struct MapAccessDeserializer<A> { /* private fields */ }
```

A deserializer holding a `MapAccess`.

**Methods**

- `pub fn new(map: A) -> MapAccessDeserializer<A>`
- `fn clone(&self) -> MapAccessDeserializer<A>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn deserialize_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, _name: &str, _variants: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`
- `fn variant_seed<T>( self, seed: T, ) -> Result<(<T as DeserializeSeed<'de>>::Value, <MapAccessDeserializer<A> as EnumAccess<'de>>::Variant), <MapAccessDeserializer<A> as EnumAccess<'de>>::Error>where T: DeserializeSeed<'de>,`
- `fn variant<V>(self) -> Result<(V, Self::Variant), Self::Error>where V: Deserialize<'de>,`
- `fn into_deserializer(self) -> MapAccessDeserializer<A>`

### struct serde::de::value::MapDeserializer

```rust
pub struct MapDeserializer<'de, I, E>where
    I: Iterator,
    <I as Iterator>::Item: Pair,{ /* private fields */ }
```

A deserializer that iterates over a map.

**Methods**

- `pub fn new(iter: I) -> MapDeserializer<'de, I, E>`
- `pub fn end(self) -> Result<(), E>`
- `fn clone(&self) -> MapDeserializer<'de, I, E>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn deserialize_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, name: &'static str, variants: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <MapDeserializer<'de, I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`
- `fn into_deserializer(self) -> MapDeserializer<'de, I, E>`
- `fn next_key_seed<T>( &mut self, seed: T, ) -> Result<Option<<T as DeserializeSeed<'de>>::Value>, <MapDeserializer<'de, I, E> as MapAccess<'de>>::Error>where T: DeserializeSeed<'de>,`
- `fn next_value_seed<T>( &mut self, seed: T, ) -> Result<<T as DeserializeSeed<'de>>::Value, <MapDeserializer<'de, I, E> as MapAccess<'de>>::Error>where T: DeserializeSeed<'de>,`

### struct serde::de::value::SeqAccessDeserializer

```rust
pub struct SeqAccessDeserializer<A> { /* private fields */ }
```

A deserializer holding a `SeqAccess`.

**Methods**

- `pub fn new(seq: A) -> SeqAccessDeserializer<A>`
- `fn clone(&self) -> SeqAccessDeserializer<A>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn deserialize_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, name: &'static str, variants: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqAccessDeserializer<A> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`
- `fn into_deserializer(self) -> SeqAccessDeserializer<A>`

### struct serde::de::value::SeqDeserializer

```rust
pub struct SeqDeserializer<I, E> { /* private fields */ }
```

A deserializer that iterates over a sequence.

**Methods**

- `pub fn new(iter: I) -> SeqDeserializer<I, E>`
- `pub fn end(self) -> Result<(), E>`
- `fn clone(&self) -> SeqDeserializer<I, E>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn deserialize_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, name: &'static str, variants: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <SeqDeserializer<I, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`
- `fn into_deserializer(self) -> SeqDeserializer<I, E>`
- `fn next_element_seed<V>( &mut self, seed: V, ) -> Result<Option<<V as DeserializeSeed<'de>>::Value>, <SeqDeserializer<I, E> as SeqAccess<'de>>::Error>where V: DeserializeSeed<'de>,`
- `fn size_hint(&self) -> Option<usize>`

### struct serde::de::value::StrDeserializer

```rust
pub struct StrDeserializer<'a, E> { /* private fields */ }
```

A deserializer holding a `&str`.

**Methods**

- `pub fn new(value: &'a str) -> StrDeserializer<'a, E>`
- `fn clone(&self) -> StrDeserializer<'de, E>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn deserialize_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, name: &str, variants: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StrDeserializer<'a, E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`
- `fn variant_seed<T>( self, seed: T, ) -> Result<(<T as DeserializeSeed<'de>>::Value, <StrDeserializer<'a, E> as EnumAccess<'de>>::Variant), <StrDeserializer<'a, E> as EnumAccess<'de>>::Error>where T: DeserializeSeed<'de>,`
- `fn variant<V>(self) -> Result<(V, Self::Variant), Self::Error>where V: Deserialize<'de>,`
- `fn into_deserializer(self) -> StrDeserializer<'a, E>`

### struct serde::de::value::StringDeserializer

```rust
pub struct StringDeserializer<E> { /* private fields */ }
```

A deserializer holding a `String`.

**Methods**

- `pub fn new(value: String) -> StringDeserializer<E>`
- `fn clone(&self) -> StringDeserializer<E>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn deserialize_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, name: &str, variants: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <StringDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`
- `fn variant_seed<T>( self, seed: T, ) -> Result<(<T as DeserializeSeed<'de>>::Value, <StringDeserializer<E> as EnumAccess<'de>>::Variant), <StringDeserializer<E> as EnumAccess<'de>>::Error>where T: DeserializeSeed<'de>,`
- `fn variant<V>(self) -> Result<(V, Self::Variant), Self::Error>where V: Deserialize<'de>,`
- `fn into_deserializer(self) -> StringDeserializer<E>`

### struct serde::de::value::U128Deserializer

```rust
pub struct U128Deserializer<E> { /* private fields */ }
```

A deserializer holding
a `u128`.

**Methods**

- `pub fn new(value: u128) -> U128Deserializer<E>`
- `fn clone(&self) -> U128Deserializer<E>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, name: &'static str, variants: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U128Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`
- `fn into_deserializer(self) -> U128Deserializer<E>`

### struct serde::de::value::U16Deserializer

```rust
pub struct U16Deserializer<E> { /* private fields */ }
```

A deserializer holding
a `u16`.

**Methods**

- `pub fn new(value: u16) -> U16Deserializer<E>`
- `fn clone(&self) -> U16Deserializer<E>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, name: &'static str, variants: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U16Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`
- `fn into_deserializer(self) -> U16Deserializer<E>`

### struct serde::de::value::U32Deserializer

```rust
pub struct U32Deserializer<E> { /* private fields */ }
```

A deserializer holding a `u32`.

**Methods**

- `pub fn new(value: u32) -> U32Deserializer<E>`
- `fn clone(&self) -> U32Deserializer<E>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, name: &str, variants: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U32Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`
- `fn variant_seed<T>( self, seed: T, ) -> Result<(<T as DeserializeSeed<'de>>::Value, <U32Deserializer<E> as EnumAccess<'de>>::Variant), <U32Deserializer<E> as EnumAccess<'de>>::Error>where T: DeserializeSeed<'de>,`
- `fn variant<V>(self) -> Result<(V, Self::Variant), Self::Error>where V: Deserialize<'de>,`
- `fn into_deserializer(self) -> U32Deserializer<E>`

### struct serde::de::value::U64Deserializer

```rust
pub struct U64Deserializer<E> { /* private fields */ }
```

A deserializer holding
a `u64`.

**Methods**

- `pub fn new(value: u64) -> U64Deserializer<E>`
- `fn clone(&self) -> U64Deserializer<E>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, name: &'static str, variants: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U64Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`
- `fn into_deserializer(self) -> U64Deserializer<E>`

### struct serde::de::value::U8Deserializer

```rust
pub struct U8Deserializer<E> { /* private fields */ }
```

A deserializer holding
a `u8`.

**Methods**

- `pub fn new(value: u8) -> U8Deserializer<E>`
- `fn clone(&self) -> U8Deserializer<E>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, name: &'static str, variants: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <U8Deserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`
- `fn into_deserializer(self) -> U8Deserializer<E>`

### struct serde::de::value::UnitDeserializer

```rust
pub struct UnitDeserializer<E> { /* private fields */ }
```

A deserializer holding a `()`.

**Methods**

- `pub fn new() -> UnitDeserializer<E>`
- `fn clone(&self) -> UnitDeserializer<E>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, name: &'static str, variants: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UnitDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`
- `fn into_deserializer(self) -> UnitDeserializer<E>`

### struct serde::de::value::UsizeDeserializer

```rust
pub struct UsizeDeserializer<E> { /* private fields */ }
```

A deserializer holding
a `usize`.

**Methods**

- `pub fn new(value: usize) -> UsizeDeserializer<E>`
- `fn clone(&self) -> UsizeDeserializer<E>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, name: &'static str, variants: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, <UsizeDeserializer<E> as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`
- `fn into_deserializer(self) -> UsizeDeserializer<E>`
