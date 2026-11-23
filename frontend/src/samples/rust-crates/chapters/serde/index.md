# Crate serde

## Overview

**Serde**

Serde is a framework for serializing and deserializing Rust data
structures efficiently and generically.

The Serde ecosystem consists of data structures that know how to serialize
and deserialize themselves along with data formats that know how to
serialize and deserialize other things. Serde provides the layer by which
these two groups interact with each other, allowing any supported data
structure to be serialized and deserialized using any supported data format.

See the Serde website https://serde.rs for additional documentation and
usage examples.

**Design**

Where many other languages rely on runtime reflection for serializing data,
Serde is instead built on Rust’s powerful trait system. A data structure
that knows how to serialize and deserialize itself is one that implements
Serde’s `Serialize` and `Deserialize` traits (or uses Serde’s derive
attribute to automatically generate implementations at compile time). This
avoids any overhead of reflection or runtime type information. In fact in
many situations the interaction between data structure and data format can
be completely optimized away by the Rust compiler, leaving Serde
serialization to perform the same speed as a handwritten serializer for the
specific selection of data structure and data format.

**Data formats**

The following is a partial list of data formats that have been implemented
for Serde by the community.

- JSON, the ubiquitous JavaScript Object Notation used by many HTTP APIs.

- Postcard, a no_std and embedded-systems friendly compact binary format.

- CBOR, a Concise Binary Object Representation designed for small message
size without the need for version negotiation.

- YAML, a self-proclaimed human-friendly configuration language that ain’t
markup language.

- MessagePack, an efficient binary format that resembles a compact JSON.

- TOML, a minimal configuration format used by Cargo.

- Pickle, a format common in the Python world.

- RON, a Rusty Object Notation.

- BSON, the data storage and network transfer format used by MongoDB.

- Avro, a binary format used within Apache Hadoop, with support for schema
definition.

- JSON5, a superset of JSON including some productions from ES5.

- URL query strings, in the x-www-form-urlencoded format.

- Starlark, the format used for describing build targets by the Bazel and
Buck build systems. (serialization only)

- Envy,

...

## Items

- `de` — Generic data structure deserialization framework.
- `ser` — Generic data structure serialization framework.
- `forward_to_deserialize_any` — Helper macro when implementing the Deserializer part of a new data format for Serde.
- `Deserialize` — A data structure that can be deserialized from any data format supported by Serde.
- `Deserializer` — A data format that can deserialize any data structure supported by Serde.
- `Serialize` — A data structure that can be serialized into any data format supported by Serde.
- `Serializer` — A data format that can serialize any data structure supported by Serde.
- `Deserialize` — Derive macro available if serde is built with features = ["derive"].
- `Serialize` — Derive macro available if serde is built with features = ["derive"].

### macro serde::forward_to_deserialize_any

```rust
macro_rules! forward_to_deserialize_any {
    (<$visitor:ident: Visitor<$lifetime:tt>> $($func:ident)*) => { ... };
    ($($func:ident)*) => { ... };
}
```

Helper macro when implementing the `Deserializer` part of a new data format
for Serde.

Some `Deserializer` implementations for self-describing formats do not
care what hint the `Visitor` gives them, they just want to blindly call
the `Visitor` method corresponding to the data they can tell is in the
input. This requires repetitive implementations of all the `Deserializer`
trait methods.

ⓘ
```rust
#[inline]
fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
where
    V: Visitor<'de>,
{
    self.deserialize_any(visitor)
}
```

The `forward_to_deserialize_any!` macro implements these simple forwarding
methods so that they forward directly to `Deserializer::deserialize_any`.
You can choose which methods to forward.

ⓘ
```rust
impl<'de> Deserializer<'de> for MyDeserializer {
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        /* ... */
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
```

The macro assumes the convention that your `Deserializer` lifetime parameter
is called `'de` and that the `Visitor` type parameters on each method are
called `V`. A different type parameter and a different lifetime can be
specified explicitly if necessary.

ⓘ

...

### trait serde::Deserialize

```rust
pub trait Deserialize<'de>: Sized {
    // Required method
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, <D as Deserializer<'de>>::Error>
       where D: Deserializer<'de>;
}
```

A data structure that can be deserialized from any data format supported
by Serde.

Serde provides `Deserialize` implementations for many Rust primitive and
standard library types. The complete list is here. All of these
can be deserialized using Serde out of the box.

Additionally, Serde provides a procedural macro called `serde_derive` to
automatically generate `Deserialize` implementations for structs and enums
in your program. See the derive section of the manual for how to
use this.

In rare cases it may be necessary to implement `Deserialize` manually for
some type in your program. See the Implementing
`Deserialize` section of the manual for more about this.

Third-party crates may provide `Deserialize` implementations for types that
they expose. For example the `linked-hash-map` crate provides a
`LinkedHashMap<K, V>` type that is deserializable by Serde because the crate
provides an implementation of `Deserialize` for it.

**Lifetime**

The `'de` lifetime of this trait is the lifetime of data that may be
borrowed by `Self` when deserialized. See the page Understanding
deserializer lifetimes for a more detailed explanation of these lifetimes.

**Methods**

- `fn deserialize<D>( deserializer: D, ) -> Result<Self, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<Cow<'a, T>, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<&'a Path, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<&'a [u8], <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<&'a str, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<Range<Idx>, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<RangeFrom<Idx>, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<RangeInclusive<Idx>, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<RangeTo<Idx>, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<HashMap<K, V, S>, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<BTreeMap<K, V>, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8), <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7), <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<(T0, T1, T2, T3, T4, T5, T6), <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<(T0, T1, T2, T3, T4, T5), <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<(T0, T1, T2, T3, T4), <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<(T0, T1, T2, T3), <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<(T0, T1, T2), <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<(T0, T1), <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<Result<T, E>, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<HashSet<T, S>, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<(T,), <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<BTreeSet<T>, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<BinaryHeap<T>, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<Bound<T>, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<Box<T>, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<Box<[T]>, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<Cell<T>, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<LinkedList<T>, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<Mutex<T>, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<Option<T>, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<PhantomData<T>, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( deserializer: D, ) -> Result<RefCell<T>, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`

### trait serde::Deserializer

```rust
pub trait Deserializer<'de>: Sized {
    type Error: Error;

    // Required methods
    fn deserialize_any<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;
    fn deserialize_bool<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;
    fn deserialize_i8<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;
    fn deserialize_i16<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;
    fn deserialize_i32<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;
    fn deserialize_i64<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;
    fn deserialize_u8<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;
    fn deserialize_u16<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;
    fn deserialize_u32<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;
    fn deserialize_u64<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;
    fn deserialize_f32<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;
    fn deserialize_f64<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;
    fn deserialize_char<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;
    fn deserialize_str<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;
    fn deserialize_string<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;
    fn deserialize_bytes<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;
    fn deserialize_byte_buf<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;
    fn deserialize_option<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;
    fn deserialize_unit<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;
    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;
    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;
    fn deserialize_seq<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;
    fn deserialize_tuple<V>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;
    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;
    fn deserialize_map<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;
    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;
    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;
    fn deserialize_identifier<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;
    fn deserialize_ignored_any<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;

    // Provided methods
    fn deserialize_i128<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de> { ... }
    fn deserialize_u128<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de> { ... }
    fn is_human_readable(&self) -> bool { ... }
}
```

A data format that can deserialize any data structure supported by
Serde.

The role of this trait is to define the deserialization half of the Serde
data model, which is a way to categorize every Rust data type into one of
29 possible types. Each method of the `Deserializer` trait corresponds to one
of the types of the data model.

Implementations of `Deserialize` map themselves into this data model by
passing to the `Deserializer` a `Visitor` implementation that can receive
these various types.

The types that make up the Serde data model are:

- 14 primitive types

- bool

- i8, i16, i32, i64, i128

- u8, u16, u32, u64, u128

- f32, f64

- char

- string

- UTF-8 bytes with a length and no null terminator.

- When serializing, all strings are handled equally. When deserializing,
there are three flavors of strings: transient, owned, and borrowed.

- byte array - [u8]

- Similar to strings, during deserialization byte arrays can be
transient, owned, or borrowed.

- option

- Either none or some value.

- unit

- The type of `()` in Rust. It represents an anonymous value containing
no data.

- unit_struct

- For example `struct Unit` or `PhantomData<T>`. It represents a named
value containing no data.

- unit_variant

- For example the `E::A` and `E::B` in `enum E { A, B }`.

- newtype_struct

- For example `struct Millimeters(u8)`.

- newtype_variant

- For example the `E::N` in `enum E { N(u8) }`.

- seq

- A variably sized heterogeneous sequence of values, for example `Vec<T>`
or `HashSet<T>`. When serializing, the length may or may not be known
before iterating through a

...

**Methods**

- `type Error: Error`
- `fn deserialize_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_f32<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_f64<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_char<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_str<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_string<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_bytes<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_byte_buf<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_option<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_unit<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_seq<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_map<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, name: &'static str, variants: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_identifier<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_u128<V>( self, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn is_human_readable(&self) -> bool`

### trait serde::Serialize

```rust
pub trait Serialize {
    // Required method
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
       where S: Serializer;
}
```

A data structure that can be serialized into any data format supported
by Serde.

Serde provides `Serialize` implementations for many Rust primitive and
standard library types. The complete list is here. All of
these can be serialized using Serde out of the box.

Additionally, Serde provides a procedural macro called `serde_derive` to
automatically generate `Serialize` implementations for structs and enums in
your program. See the derive section of the manual for how to use this.

In rare cases it may be necessary to implement `Serialize` manually for some
type in your program. See the Implementing `Serialize` section of the
manual for more about this.

Third-party crates may provide `Serialize` implementations for types that
they expose. For example the `linked-hash-map` crate provides a
`LinkedHashMap<K, V>` type that is serializable by Serde because the crate
provides an implementation of `Serialize` for it.

**Methods**

- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`
- `fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where S: Serializer,`

### trait serde::Serializer

```rust
pub trait Serializer: Sized {
    type Ok;
    type Error: Error;
    type SerializeSeq: SerializeSeq<Ok = Self::Ok, Error = Self::Error>;
    type SerializeTuple: SerializeTuple<Ok = Self::Ok, Error = Self::Error>;
    type SerializeTupleStruct: SerializeTupleStruct<Ok = Self::Ok, Error = Self::Error>;
    type SerializeTupleVariant: SerializeTupleVariant<Ok = Self::Ok, Error = Self::Error>;
    type SerializeMap: SerializeMap<Ok = Self::Ok, Error = Self::Error>;
    type SerializeStruct: SerializeStruct<Ok = Self::Ok, Error = Self::Error>;
    type SerializeStructVariant: SerializeStructVariant<Ok = Self::Ok, Error = Self::Error>;

    // Required methods
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error>;
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error>;
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error>;
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error>;
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error>;
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error>;
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error>;
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error>;
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error>;
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error>;
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error>;
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error>;
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error>;
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error>;
    fn serialize_none(self) -> Result<Self::Ok, Self::Error>;
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
       where T: Serialize + ?Sized;
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error>;
    fn serialize_unit_struct(
        self,
        name: &'static str,
    ) -> Result<Self::Ok, Self::Error>;
    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error>;
    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
       where T: Serialize + ?Sized;
    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
       where T: Serialize + ?Sized;
    fn serialize_seq(
        self,
        len: Option<usize>,
    ) -> Result<Self::SerializeSeq, Self::Error>;
    fn serialize_tuple(
        self,
        len: usize,
    ) -> Result<Self::SerializeTuple, Self::Error>;
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error>;
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error>;
    fn serialize_map(
        self,
        len: Option<usize>,
    ) -> Result<Self::SerializeMap, Self::Error>;
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error>;
    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error>;

    // Provided methods
    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> { ... }
    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> { ... }
    fn collect_seq<I>(self, iter: I) -> Result<Self::Ok, Self::Error>
       where I: IntoIterator,
             <I as IntoIterator>::Item: Serialize { ... }
    fn collect_map<K, V, I>(self, iter: I) -> Result<Self::Ok, Self::Error>
       where K: Serialize,
             V: Serialize,
             I: IntoIterator<Item = (K, V)> { ... }
    fn collect_str<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
       where T: Display + ?Sized { ... }
    fn is_human_readable(&self) -> bool { ... }
}
```

A data format that can serialize any data structure supported by Serde.

The role of this trait is to define the serialization half of the Serde
data model, which is a way to categorize every Rust data structure into one
of 29 possible types. Each method of the `Serializer` trait corresponds to
one of the types of the data model.

Implementations of `Serialize` map themselves into this data model by
invoking exactly one of the `Serializer` methods.

The types that make up the Serde data model are:

- 14 primitive types

- bool

- i8, i16, i32, i64, i128

- u8, u16, u32, u64, u128

- f32, f64

- char

- string

- UTF-8 bytes with a length and no null terminator.

- When serializing, all strings are handled equally. When deserializing,
there are three flavors of strings: transient, owned, and borrowed.

- byte array - [u8]

- Similar to strings, during deserialization byte arrays can be
transient, owned, or borrowed.

- option

- Either none or some value.

- unit

- The type of `()` in Rust. It represents an anonymous value containing
no data.

- unit_struct

- For example `struct Unit` or `PhantomData<T>`. It represents a named
value containing no data.

- unit_variant

- For example the `E::A` and `E::B` in `enum E { A, B }`.

- newtype_struct

- For example `struct Millimeters(u8)`.

- newtype_variant

- For example the `E::N` in `enum E { N(u8) }`.

- seq

- A variably sized heterogeneous sequence of values, for example
`Vec<T>` or `HashSet<T>`. When serializing, the length may or may not
be known before iterating through all the data. When deserializing,
the length is d

...

**Methods**

- `type Ok`
- `type Error: Error`
- `type SerializeSeq: SerializeSeq<Ok = Self::Ok, Error = Self::Error>`
- `type SerializeTuple: SerializeTuple<Ok = Self::Ok, Error = Self::Error>`
- `type SerializeTupleStruct: SerializeTupleStruct<Ok = Self::Ok, Error = Self::Error>`
- `type SerializeTupleVariant: SerializeTupleVariant<Ok = Self::Ok, Error = Self::Error>`
- `type SerializeMap: SerializeMap<Ok = Self::Ok, Error = Self::Error>`
- `type SerializeStruct: SerializeStruct<Ok = Self::Ok, Error = Self::Error>`
- `type SerializeStructVariant: SerializeStructVariant<Ok = Self::Ok, Error = Self::Error>`
- `fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error>`
- `fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error>`
- `fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error>`
- `fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error>`
- `fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error>`
- `fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error>`
- `fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error>`
- `fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error>`
- `fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error>`
- `fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error>`
- `fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error>`
- `fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error>`
- `fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error>`
- `fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error>`
- `fn serialize_none(self) -> Result<Self::Ok, Self::Error>`
- `fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>where T: Serialize + ?Sized,`
- `fn serialize_unit(self) -> Result<Self::Ok, Self::Error>`
- `fn serialize_unit_struct( self, name: &'static str, ) -> Result<Self::Ok, Self::Error>`
- `fn serialize_unit_variant( self, name: &'static str, variant_index: u32, variant: &'static str, ) -> Result<Self::Ok, Self::Error>`
- `fn serialize_newtype_struct<T>( self, name: &'static str, value: &T, ) -> Result<Self::Ok, Self::Error>where T: Serialize + ?Sized,`
- `fn serialize_newtype_variant<T>( self, name: &'static str, variant_index: u32, variant: &'static str, value: &T, ) -> Result<Self::Ok, Self::Error>where T: Serialize + ?Sized,`
- `fn serialize_seq( self, len: Option<usize>, ) -> Result<Self::SerializeSeq, Self::Error>`
- `fn serialize_tuple( self, len: usize, ) -> Result<Self::SerializeTuple, Self::Error>`
- `fn serialize_tuple_struct( self, name: &'static str, len: usize, ) -> Result<Self::SerializeTupleStruct, Self::Error>`
- `fn serialize_tuple_variant( self, name: &'static str, variant_index: u32, variant: &'static str, len: usize, ) -> Result<Self::SerializeTupleVariant, Self::Error>`
- `fn serialize_map( self, len: Option<usize>, ) -> Result<Self::SerializeMap, Self::Error>`
- `fn serialize_struct( self, name: &'static str, len: usize, ) -> Result<Self::SerializeStruct, Self::Error>`
- `fn serialize_struct_variant( self, name: &'static str, variant_index: u32, variant: &'static str, len: usize, ) -> Result<Self::SerializeStructVariant, Self::Error>`
- `fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error>`
- `fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error>`
- `fn collect_seq<I>(self, iter: I) -> Result<Self::Ok, Self::Error>where I: IntoIterator, <I as IntoIterator>::Item: Serialize,`
