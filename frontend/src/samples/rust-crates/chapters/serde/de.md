# Module serde::de

## Overview

Generic data structure deserialization framework.

The two most important traits in this module are `Deserialize` and
`Deserializer`.

- A type that implements `Deserialize` is a data structure that can be
deserialized from any data format supported by Serde, and conversely

- A type that implements `Deserializer` is a data format that can
deserialize any data structure supported by Serde.

**The Deserialize trait**

Serde provides `Deserialize` implementations for many Rust primitive and
standard library types. The complete list is below. All of these can be
deserialized using Serde out of the box.

Additionally, Serde provides a procedural macro called `serde_derive` to
automatically generate `Deserialize` implementations for structs and enums
in your program. See the derive section of the manual for how to use this.

In rare cases it may be necessary to implement `Deserialize` manually for
some type in your program. See the Implementing `Deserialize` section of
the manual for more about this.

Third-party crates may provide `Deserialize` implementations for types
that they expose. For example the `linked-hash-map` crate provides a
`LinkedHashMap<K, V>` type that is deserializable by Serde because the
crate provides an implementation of `Deserialize` for it.

**The Deserializer trait**

`Deserializer` implementations are provided by third-party crates, for
example `serde_json`, `serde_yaml` and `postcard`.

A partial list of well-maintained formats is given on the Serde
website.

**Implementations of Deserialize provided by Serde**

This is a slightly different set of types than what is supported for
serialization. Some types can be serialized by Serde but not deserialized.
One example is `OsStr`.

- Primitive types:

- bool

- i8, i16, i32, i64, i128, isize

- u8, u16, u32, u64, u128, usize

- f32, f64

- char

- Compound types:

- [T; 0] through [T; 32]

- tuples up to size 16

- Common standard library types:

- String

- Option<T>

- Result<T, E>

- PhantomData<T>

- Wrapper types:

- Box<T>

- Box<[T]>

- Box<str>

- Cow<’a, T>

- Cell<T>

- RefCell<T>

- Mutex<T>

- RwLock<T>

- Rc<T> (if features = [“rc”] is enabled)

- Arc<T> (if features = [“rc”] is enabled)

- Collection types:

- BTreeMap<K, V>

- BTreeSet<T>

- BinaryHeap<T>

- HashMap<K, V, H>

- HashSet<T, H>

- LinkedList<T>

- VecDeque<T>

- Vec<T>

- Zero-copy types:

- &str

- &[u8]

- FF

...

## Items

- `value` — Building blocks for deserializing basic values using the IntoDeserializer trait.
- `IgnoredAny` — An efficient way of discarding data from a deserializer.
- `Unexpected` — Unexpected represents an unexpected invocation of any one of the Visitor trait methods.
- `Deserialize` — A data structure that can be deserialized from any data format supported by Serde.
- `DeserializeOwned` — A data structure that can be deserialized without borrowing any data from the deserializer.
- `DeserializeSeed` — DeserializeSeed is the stateful form of the Deserialize trait. If you ever find yourself looking for a way to pass data into a Deserialize impl, this trait is the way to do it.
- `Deserializer` — A data format that can deserialize any data structure supported by Serde.
- `EnumAccess` — Provides a Visitor access to the data of an enum in the input.
- `Error` — The Error trait allows Deserialize implementations to create descriptive error messages belonging to the Deserializer against which they are currently running.
- `Expected` — Expected represents an explanation of what data a Visitor was expecting to receive.
- `IntoDeserializer` — Converts an existing value into a Deserializer from which other values can be deserialized.
- `MapAccess` — Provides a Visitor access to each entry of a map in the input.
- `SeqAccess` — Provides a Visitor access to each element of a sequence in the input.
- `VariantAccess` — VariantAccess is a visitor that is created by the Deserializer and passed to the Deserialize to deserialize the content of a particular enum variant.
- `Visitor` — This trait represents a visitor that walks through a deserializer.

### enum serde::de::Unexpected

```rust
pub enum Unexpected<'a> {
    Bool(bool),
    Unsigned(u64),
    Signed(i64),
    Float(f64),
    Char(char),
    Str(&'a str),
    Bytes(&'a [u8]),
    Unit,
    Option,
    NewtypeStruct,
    Seq,
    Map,
    Enum,
    UnitVariant,
    NewtypeVariant,
    TupleVariant,
    StructVariant,
    Other(&'a str),
}
```

`Unexpected` represents an unexpected invocation of any one of the `Visitor`
trait methods.

This is used as an argument to the `invalid_type`, `invalid_value`, and
`invalid_length` methods of the `Error` trait to build error messages.

ⓘ
```rust
fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
where
    E: de::Error,
{
    Err(de::Error::invalid_type(Unexpected::Bool(v), &self))
}
```

**Methods**

- `fn clone(&self) -> Unexpected<'a>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn eq(&self, other: &Unexpected<'a>) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### struct serde::de::IgnoredAny

```rust
pub struct IgnoredAny;
```

An efficient way of discarding data from a deserializer.

Think of this like `serde_json::Value` in that it can be deserialized from
any type, except that it does not store any information about the data that
gets deserialized.

ⓘ
```rust
use serde::de::{
    self, Deserialize, DeserializeSeed, Deserializer, IgnoredAny, SeqAccess, Visitor,
};
use std::fmt;
use std::marker::PhantomData;

/// A seed that can be used to deserialize only the `n`th element of a sequence
/// while efficiently discarding elements of any type before or after index `n`.
///
/// For example to deserialize only the element at index 3:
///
/// ```
/// NthElement::new(3).deserialize(deserializer)
///

...

**Methods**

- `fn clone(&self) -> IgnoredAny`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn default() -> IgnoredAny`
- `fn deserialize<D>( deserializer: D, ) -> Result<IgnoredAny, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn eq(&self, other: &IgnoredAny) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn expecting(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn visit_bool<E>( self, x: bool, ) -> Result<<IgnoredAny as Visitor<'de>>::Value, E>`
- `fn visit_i64<E>(self, x: i64) -> Result<<IgnoredAny as Visitor<'de>>::Value, E>`
- `fn visit_i128<E>( self, x: i128, ) -> Result<<IgnoredAny as Visitor<'de>>::Value, E>`
- `fn visit_u64<E>(self, x: u64) -> Result<<IgnoredAny as Visitor<'de>>::Value, E>`
- `fn visit_u128<E>( self, x: u128, ) -> Result<<IgnoredAny as Visitor<'de>>::Value, E>`
- `fn visit_f64<E>(self, x: f64) -> Result<<IgnoredAny as Visitor<'de>>::Value, E>`
- `fn visit_str<E>(self, s: &str) -> Result<<IgnoredAny as Visitor<'de>>::Value, E>where E: Error,`
- `fn visit_none<E>(self) -> Result<<IgnoredAny as Visitor<'de>>::Value, E>`
- `fn visit_some<D>( self, deserializer: D, ) -> Result<<IgnoredAny as Visitor<'de>>::Value, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn visit_newtype_struct<D>( self, deserializer: D, ) -> Result<<IgnoredAny as Visitor<'de>>::Value, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn visit_unit<E>(self) -> Result<<IgnoredAny as Visitor<'de>>::Value, E>`
- `fn visit_seq<A>( self, seq: A, ) -> Result<<IgnoredAny as Visitor<'de>>::Value, <A as SeqAccess<'de>>::Error>where A: SeqAccess<'de>,`
- `fn visit_map<A>( self, map: A, ) -> Result<<IgnoredAny as Visitor<'de>>::Value, <A as MapAccess<'de>>::Error>where A: MapAccess<'de>,`
- `fn visit_bytes<E>( self, bytes: &[u8], ) -> Result<<IgnoredAny as Visitor<'de>>::Value, E>where E: Error,`
- `fn visit_enum<A>( self, data: A, ) -> Result<<IgnoredAny as Visitor<'de>>::Value, <A as EnumAccess<'de>>::Error>where A: EnumAccess<'de>,`
- `fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>where E: Error,`
- `fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>where E: Error,`
- `fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>where E: Error,`
- `fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>where E: Error,`
- `fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>where E: Error,`
- `fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>where E: Error,`
- `fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>where E: Error,`
- `fn visit_char<E>(self, v: char) -> Result<Self::Value, E>where E: Error,`
- `fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>where E: Error,`
- `fn visit_string<E>(self, v: String) -> Result<Self::Value, E>where E: Error,`
- `fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>where E: Error,`
- `fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>where E: Error,`

### trait serde::de::Deserialize

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

### trait serde::de::DeserializeOwned

```rust
pub trait DeserializeOwned: for<'de> Deserialize<'de> { }
```

A data structure that can be deserialized without borrowing any data from
the deserializer.

This is primarily useful for trait bounds on functions. For example a
`from_str` function may be able to deserialize a data structure that borrows
from the input string, but a `from_reader` function may only deserialize
owned data.

ⓘ
```rust
fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>;

fn from_reader<R, T>(rdr: R) -> Result<T>
where
    R: Read,
    T: DeserializeOwned;
```

**Lifetime**

The relationship between `Deserialize` and `DeserializeOwned` in trait
bounds is explained in more detail on the page Understanding deserializer
lifetimes.

### trait serde::de::DeserializeSeed

```rust
pub trait DeserializeSeed<'de>: Sized {
    type Value;

    // Required method
    fn deserialize<D>(
        self,
        deserializer: D,
    ) -> Result<Self::Value, <D as Deserializer<'de>>::Error>
       where D: Deserializer<'de>;
}
```

`DeserializeSeed` is the stateful form of the `Deserialize` trait. If you
ever find yourself looking for a way to pass data into a `Deserialize` impl,
this trait is the way to do it.

As one example of stateful deserialization consider deserializing a JSON
array into an existing buffer. Using the `Deserialize` trait we could
deserialize a JSON array into a `Vec<T>` but it would be a freshly allocated
`Vec<T>`; there is no way for `Deserialize` to reuse a previously allocated
buffer. Using `DeserializeSeed` instead makes this possible as in the
example code below.

The canonical API for stateless deserialization looks like this:

ⓘ
```rust
fn func<'de, T: Deserialize<'de>>() -> Result<T, Error>
```

Adjusting an API like this to support stateful deserialization is a matter
of accepting a seed as input:

ⓘ
```rust
fn func_seed<'de, T: DeserializeSeed<'de>>(seed: T) -> Result<T::Value, Error>
```

In practice the majority of deserialization is stateless. An API expecting a
seed can be appeased by passing `std::marker::PhantomData` as a seed in the
case of stateless deserialization.

**Lifetime**

The `'de` lifetime of this trait is the lifetime of data that may be
borrowed by `Self::Value` when deserialized. See the page Understanding
deserializer lifetimes for a more detailed explanation of these lifetimes.

**Example**

Suppose we have JSON that looks like `[[1, 2], [3, 4, 5], [6]]` and we need
to deserialize it into a flat representation like `vec![1, 2, 3, 4, 5, 6]`.
Allocating a brand new `Vec<T>` for each subarray would be slow. Instead we
would like to allocate a single

...

**Methods**

- `type Value`
- `fn deserialize<D>( self, deserializer: D, ) -> Result<Self::Value, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn deserialize<D>( self, deserializer: D, ) -> Result<T, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`

### trait serde::de::Deserializer

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

### trait serde::de::EnumAccess

```rust
pub trait EnumAccess<'de>: Sized {
    type Error: Error;
    type Variant: VariantAccess<'de, Error = Self::Error>;

    // Required method
    fn variant_seed<V>(
        self,
        seed: V,
    ) -> Result<(<V as DeserializeSeed<'de>>::Value, Self::Variant), Self::Error>
       where V: DeserializeSeed<'de>;

    // Provided method
    fn variant<V>(self) -> Result<(V, Self::Variant), Self::Error>
       where V: Deserialize<'de> { ... }
}
```

Provides a `Visitor` access to the data of an enum in the input.

`EnumAccess` is created by the `Deserializer` and passed to the
`Visitor` in order to identify which variant of an enum to deserialize.

**Lifetime**

The `'de` lifetime of this trait is the lifetime of data that may be
borrowed by the deserialized enum variant. See the page Understanding
deserializer lifetimes for a more detailed explanation of these lifetimes.

**Example implementation**

The example data format presented on the website demonstrates an
implementation of `EnumAccess` for a basic JSON data format.

**Methods**

- `type Error: Error`
- `type Variant: VariantAccess<'de, Error = Self::Error>`
- `fn variant_seed<V>( self, seed: V, ) -> Result<(<V as DeserializeSeed<'de>>::Value, Self::Variant), Self::Error>where V: DeserializeSeed<'de>,`
- `fn variant<V>(self) -> Result<(V, Self::Variant), Self::Error>where V: Deserialize<'de>,`

### trait serde::de::Error

```rust
pub trait Error: Sized + Error {
    // Required method
    fn custom<T>(msg: T) -> Self
       where T: Display;

    // Provided methods
    fn invalid_type(unexp: Unexpected<'_>, exp: &dyn Expected) -> Self { ... }
    fn invalid_value(unexp: Unexpected<'_>, exp: &dyn Expected) -> Self { ... }
    fn invalid_length(len: usize, exp: &dyn Expected) -> Self { ... }
    fn unknown_variant(variant: &str, expected: &'static [&'static str]) -> Self { ... }
    fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self { ... }
    fn missing_field(field: &'static str) -> Self { ... }
    fn duplicate_field(field: &'static str) -> Self { ... }
}
```

The `Error` trait allows `Deserialize` implementations to create descriptive
error messages belonging to the `Deserializer` against which they are
currently running.

Every `Deserializer` declares an `Error` type that encompasses both
general-purpose deserialization errors as well as errors specific to the
particular deserialization format. For example the `Error` type of
`serde_json` can represent errors like an invalid JSON escape sequence or an
unterminated string literal, in addition to the error cases that are part of
this trait.

Most deserializers should only need to provide the `Error::custom` method
and inherit the default behavior for the other methods.

**Example implementation**

The example data format presented on the website shows an error
type appropriate for a basic JSON data format.

**Methods**

- `fn custom<T>(msg: T) -> Selfwhere T: Display,`
- `fn invalid_type(unexp: Unexpected<'_>, exp: &dyn Expected) -> Self`
- `fn invalid_value(unexp: Unexpected<'_>, exp: &dyn Expected) -> Self`
- `fn invalid_length(len: usize, exp: &dyn Expected) -> Self`
- `fn unknown_variant(variant: &str, expected: &'static [&'static str]) -> Self`
- `fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self`
- `fn missing_field(field: &'static str) -> Self`
- `fn duplicate_field(field: &'static str) -> Self`

### trait serde::de::Expected

```rust
pub trait Expected {
    // Required method
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>;
}
```

`Expected` represents an explanation of what data a `Visitor` was expecting
to receive.

This is used as an argument to the `invalid_type`, `invalid_value`, and
`invalid_length` methods of the `Error` trait to build error messages. The
message should be a noun or noun phrase that completes the sentence “This
Visitor expects to receive …”, for example the message could be “an
integer between 0 and 64”. The message should not be capitalized and should
not end with a period.

Within the context of a `Visitor` implementation, the `Visitor` itself
(`&self`) is an implementation of this trait.

ⓘ
```rust
fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
where
    E: de::Error,
{
    Err(de::Error::invalid_type(Unexpected::Bool(v), &self))
}
```

Outside of a `Visitor`, `&"..."` can be used.

ⓘ
```rust
return Err(de::Error::invalid_type(
    Unexpected::Bool(v),
    &"a negative integer",
));
```

**Methods**

- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`

### trait serde::de::IntoDeserializer

```rust
pub trait IntoDeserializer<'de, E = Error>where
    E: Error,{
    type Deserializer: Deserializer<'de, Error = E>;

    // Required method
    fn into_deserializer(self) -> Self::Deserializer;
}
```

Converts an existing value into a `Deserializer` from which other values can
be deserialized.

**Lifetime**

The `'de` lifetime of this trait is the lifetime of data that may be
borrowed from the resulting `Deserializer`. See the page Understanding
deserializer lifetimes for a more detailed explanation of these lifetimes.

**Example**

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

**Methods**

- `type Deserializer: Deserializer<'de, Error = E>`
- `fn into_deserializer(self) -> Self::Deserializer`
- `fn into_deserializer(self) -> BytesDeserializer<'a, E>`
- `fn into_deserializer(self) -> StrDeserializer<'a, E>`
- `fn into_deserializer(self) -> CowStrDeserializer<'a, E>`
- `fn into_deserializer(self) -> UnitDeserializer<E>`
- `fn into_deserializer(self) -> StringDeserializer<E>`
- `fn into_deserializer(self) -> BoolDeserializer<E>`
- `fn into_deserializer(self) -> CharDeserializer<E>`
- `fn into_deserializer(self) -> F32Deserializer<E>`
- `fn into_deserializer(self) -> F64Deserializer<E>`
- `fn into_deserializer(self) -> I8Deserializer<E>`
- `fn into_deserializer(self) -> I16Deserializer<E>`
- `fn into_deserializer(self) -> I32Deserializer<E>`
- `fn into_deserializer(self) -> I64Deserializer<E>`
- `fn into_deserializer(self) -> I128Deserializer<E>`
- `fn into_deserializer(self) -> IsizeDeserializer<E>`
- `fn into_deserializer(self) -> U8Deserializer<E>`
- `fn into_deserializer(self) -> U16Deserializer<E>`
- `fn into_deserializer(self) -> U32Deserializer<E>`
- `fn into_deserializer(self) -> U64Deserializer<E>`
- `fn into_deserializer(self) -> U128Deserializer<E>`
- `fn into_deserializer(self) -> UsizeDeserializer<E>`
- `fn into_deserializer( self, ) -> <BTreeMap<K, V> as IntoDeserializer<'de, E>>::Deserializer`
- `fn into_deserializer( self, ) -> <HashMap<K, V, S> as IntoDeserializer<'de, E>>::Deserializer`
- `fn into_deserializer( self, ) -> <BTreeSet<T> as IntoDeserializer<'de, E>>::Deserializer`
- `fn into_deserializer(self) -> <Vec<T> as IntoDeserializer<'de, E>>::Deserializer`
- `fn into_deserializer( self, ) -> <HashSet<T, S> as IntoDeserializer<'de, E>>::Deserializer`

### trait serde::de::MapAccess

```rust
pub trait MapAccess<'de> {
    type Error: Error;

    // Required methods
    fn next_key_seed<K>(
        &mut self,
        seed: K,
    ) -> Result<Option<<K as DeserializeSeed<'de>>::Value>, Self::Error>
       where K: DeserializeSeed<'de>;
    fn next_value_seed<V>(
        &mut self,
        seed: V,
    ) -> Result<<V as DeserializeSeed<'de>>::Value, Self::Error>
       where V: DeserializeSeed<'de>;

    // Provided methods
    fn next_entry_seed<K, V>(
        &mut self,
        kseed: K,
        vseed: V,
    ) -> Result<Option<(<K as DeserializeSeed<'de>>::Value, <V as DeserializeSeed<'de>>::Value)>, Self::Error>
       where K: DeserializeSeed<'de>,
             V: DeserializeSeed<'de> { ... }
    fn next_key<K>(&mut self) -> Result<Option<K>, Self::Error>
       where K: Deserialize<'de> { ... }
    fn next_value<V>(&mut self) -> Result<V, Self::Error>
       where V: Deserialize<'de> { ... }
    fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, Self::Error>
       where K: Deserialize<'de>,
             V: Deserialize<'de> { ... }
    fn size_hint(&self) -> Option<usize> { ... }
}
```

Provides a `Visitor` access to each entry of a map in the input.

This is a trait that a `Deserializer` passes to a `Visitor` implementation.

**Lifetime**

The `'de` lifetime of this trait is the lifetime of data that may be
borrowed by deserialized map entries. See the page Understanding
deserializer lifetimes for a more detailed explanation of these lifetimes.

**Example implementation**

The example data format presented on the website demonstrates an
implementation of `MapAccess` for a basic JSON data format.

**Methods**

- `type Error: Error`
- `fn next_key_seed<K>( &mut self, seed: K, ) -> Result<Option<<K as DeserializeSeed<'de>>::Value>, Self::Error>where K: DeserializeSeed<'de>,`
- `fn next_value_seed<V>( &mut self, seed: V, ) -> Result<<V as DeserializeSeed<'de>>::Value, Self::Error>where V: DeserializeSeed<'de>,`
- `fn next_entry_seed<K, V>( &mut self, kseed: K, vseed: V, ) -> Result<Option<(<K as DeserializeSeed<'de>>::Value, <V as DeserializeSeed<'de>>::Value)>, Self::Error>where K: DeserializeSeed<'de>, V: DeserializeSeed<'de>,`
- `fn next_key<K>(&mut self) -> Result<Option<K>, Self::Error>where K: Deserialize<'de>,`
- `fn next_value<V>(&mut self) -> Result<V, Self::Error>where V: Deserialize<'de>,`
- `fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, Self::Error>where K: Deserialize<'de>, V: Deserialize<'de>,`
- `fn size_hint(&self) -> Option<usize>`
- `fn next_key_seed<K>( &mut self, seed: K, ) -> Result<Option<<K as DeserializeSeed<'de>>::Value>, <&mut A as MapAccess<'de>>::Error>where K: DeserializeSeed<'de>,`
- `fn next_value_seed<V>( &mut self, seed: V, ) -> Result<<V as DeserializeSeed<'de>>::Value, <&mut A as MapAccess<'de>>::Error>where V: DeserializeSeed<'de>,`
- `fn next_entry_seed<K, V>( &mut self, kseed: K, vseed: V, ) -> Result<Option<(<K as DeserializeSeed<'de>>::Value, <V as DeserializeSeed<'de>>::Value)>, <&mut A as MapAccess<'de>>::Error>where K: DeserializeSeed<'de>, V: DeserializeSeed<'de>,`
- `fn next_entry<K, V>( &mut self, ) -> Result<Option<(K, V)>, <&mut A as MapAccess<'de>>::Error>where K: Deserialize<'de>, V: Deserialize<'de>,`
- `fn next_key<K>( &mut self, ) -> Result<Option<K>, <&mut A as MapAccess<'de>>::Error>where K: Deserialize<'de>,`
- `fn next_value<V>(&mut self) -> Result<V, <&mut A as MapAccess<'de>>::Error>where V: Deserialize<'de>,`
- `fn size_hint(&self) -> Option<usize>`

### trait serde::de::SeqAccess

```rust
pub trait SeqAccess<'de> {
    type Error: Error;

    // Required method
    fn next_element_seed<T>(
        &mut self,
        seed: T,
    ) -> Result<Option<<T as DeserializeSeed<'de>>::Value>, Self::Error>
       where T: DeserializeSeed<'de>;

    // Provided methods
    fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
       where T: Deserialize<'de> { ... }
    fn size_hint(&self) -> Option<usize> { ... }
}
```

Provides a `Visitor` access to each element of a sequence in the input.

This is a trait that a `Deserializer` passes to a `Visitor` implementation,
which deserializes each item in a sequence.

**Lifetime**

The `'de` lifetime of this trait is the lifetime of data that may be
borrowed by deserialized sequence elements. See the page Understanding
deserializer lifetimes for a more detailed explanation of these lifetimes.

**Example implementation**

The example data format presented on the website demonstrates an
implementation of `SeqAccess` for a basic JSON data format.

**Methods**

- `type Error: Error`
- `fn next_element_seed<T>( &mut self, seed: T, ) -> Result<Option<<T as DeserializeSeed<'de>>::Value>, Self::Error>where T: DeserializeSeed<'de>,`
- `fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>where T: Deserialize<'de>,`
- `fn size_hint(&self) -> Option<usize>`
- `fn next_element_seed<T>( &mut self, seed: T, ) -> Result<Option<<T as DeserializeSeed<'de>>::Value>, <&mut A as SeqAccess<'de>>::Error>where T: DeserializeSeed<'de>,`
- `fn next_element<T>( &mut self, ) -> Result<Option<T>, <&mut A as SeqAccess<'de>>::Error>where T: Deserialize<'de>,`
- `fn size_hint(&self) -> Option<usize>`

### trait serde::de::VariantAccess

```rust
pub trait VariantAccess<'de>: Sized {
    type Error: Error;

    // Required methods
    fn unit_variant(self) -> Result<(), Self::Error>;
    fn newtype_variant_seed<T>(
        self,
        seed: T,
    ) -> Result<<T as DeserializeSeed<'de>>::Value, Self::Error>
       where T: DeserializeSeed<'de>;
    fn tuple_variant<V>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;
    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
       where V: Visitor<'de>;

    // Provided method
    fn newtype_variant<T>(self) -> Result<T, Self::Error>
       where T: Deserialize<'de> { ... }
}
```

`VariantAccess` is a visitor that is created by the `Deserializer` and
passed to the `Deserialize` to deserialize the content of a particular enum
variant.

**Lifetime**

The `'de` lifetime of this trait is the lifetime of data that may be
borrowed by the deserialized enum variant. See the page Understanding
deserializer lifetimes for a more detailed explanation of these lifetimes.

**Example implementation**

The example data format presented on the website demonstrates an
implementation of `VariantAccess` for a basic JSON data format.

**Methods**

- `type Error: Error`
- `fn unit_variant(self) -> Result<(), Self::Error>`
- `fn newtype_variant_seed<T>( self, seed: T, ) -> Result<<T as DeserializeSeed<'de>>::Value, Self::Error>where T: DeserializeSeed<'de>,`
- `fn tuple_variant<V>( self, len: usize, visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn struct_variant<V>( self, fields: &'static [&'static str], visitor: V, ) -> Result<<V as Visitor<'de>>::Value, Self::Error>where V: Visitor<'de>,`
- `fn newtype_variant<T>(self) -> Result<T, Self::Error>where T: Deserialize<'de>,`

### trait serde::de::Visitor

```rust
pub trait Visitor<'de>: Sized {
    type Value;

    // Required method
    fn expecting(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>;

    // Provided methods
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
       where E: Error { ... }
    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
       where E: Error { ... }
    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
       where E: Error { ... }
    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
       where E: Error { ... }
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
       where E: Error { ... }
    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
       where E: Error { ... }
    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
       where E: Error { ... }
    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
       where E: Error { ... }
    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
       where E: Error { ... }
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
       where E: Error { ... }
    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
       where E: Error { ... }
    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
       where E: Error { ... }
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
       where E: Error { ... }
    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
       where E: Error { ... }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
       where E: Error { ... }
    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
       where E: Error { ... }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
       where E: Error { ... }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
       where E: Error { ... }
    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
       where E: Error { ... }
    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
       where E: Error { ... }
    fn visit_none<E>(self) -> Result<Self::Value, E>
       where E: Error { ... }
    fn visit_some<D>(
        self,
        deserializer: D,
    ) -> Result<Self::Value, <D as Deserializer<'de>>::Error>
       where D: Deserializer<'de> { ... }
    fn visit_unit<E>(self) -> Result<Self::Value, E>
       where E: Error { ... }
    fn visit_newtype_struct<D>(
        self,
        deserializer: D,
    ) -> Result<Self::Value, <D as Deserializer<'de>>::Error>
       where D: Deserializer<'de> { ... }
    fn visit_seq<A>(
        self,
        seq: A,
    ) -> Result<Self::Value, <A as SeqAccess<'de>>::Error>
       where A: SeqAccess<'de> { ... }
    fn visit_map<A>(
        self,
        map: A,
    ) -> Result<Self::Value, <A as MapAccess<'de>>::Error>
       where A: MapAccess<'de> { ... }
    fn visit_enum<A>(
        self,
        data: A,
    ) -> Result<Self::Value, <A as EnumAccess<'de>>::Error>
       where A: EnumAccess<'de> { ... }
}
```

This trait represents a visitor that walks through a deserializer.

**Lifetime**

The `'de` lifetime of this trait is the requirement for lifetime of data
that may be borrowed by `Self::Value`. See the page Understanding
deserializer lifetimes for a more detailed explanation of these lifetimes.

**Example**

ⓘ
```rust
/// A visitor that deserializes a long string - a string containing at least
/// some minimum number of bytes.
struct LongString {
    min: usize,
}

impl<'de> Visitor<'de> for LongString {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string containing at least {} bytes", self.min)
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if s.len() >= self.min {
            Ok(s.to_owned())
        } else {
            Err(de::Error::invalid_value(Unexpected::Str(s), &self))
        }
    }
}
```

**Methods**

- `type Value`
- `fn expecting(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>where E: Error,`
- `fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>where E: Error,`
- `fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>where E: Error,`
- `fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>where E: Error,`
- `fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>where E: Error,`
- `fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>where E: Error,`
- `fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>where E: Error,`
- `fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>where E: Error,`
- `fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>where E: Error,`
- `fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>where E: Error,`
- `fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>where E: Error,`
- `fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>where E: Error,`
- `fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>where E: Error,`
- `fn visit_char<E>(self, v: char) -> Result<Self::Value, E>where E: Error,`
- `fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>where E: Error,`
- `fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>where E: Error,`
- `fn visit_string<E>(self, v: String) -> Result<Self::Value, E>where E: Error,`
- `fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>where E: Error,`
- `fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>where E: Error,`
- `fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>where E: Error,`
- `fn visit_none<E>(self) -> Result<Self::Value, E>where E: Error,`
- `fn visit_some<D>( self, deserializer: D, ) -> Result<Self::Value, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn visit_unit<E>(self) -> Result<Self::Value, E>where E: Error,`
- `fn visit_newtype_struct<D>( self, deserializer: D, ) -> Result<Self::Value, <D as Deserializer<'de>>::Error>where D: Deserializer<'de>,`
- `fn visit_seq<A>( self, seq: A, ) -> Result<Self::Value, <A as SeqAccess<'de>>::Error>where A: SeqAccess<'de>,`
- `fn visit_map<A>( self, map: A, ) -> Result<Self::Value, <A as MapAccess<'de>>::Error>where A: MapAccess<'de>,`
- `fn visit_enum<A>( self, data: A, ) -> Result<Self::Value, <A as EnumAccess<'de>>::Error>where A: EnumAccess<'de>,`
