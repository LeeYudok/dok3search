# Module serde::ser

## Overview

Generic data structure serialization framework.

The two most important traits in this module are `Serialize` and
`Serializer`.

- A type that implements `Serialize` is a data structure that can be
serialized to any data format supported by Serde, and conversely

- A type that implements `Serializer` is a data format that can
serialize any data structure supported by Serde.

**The Serialize trait**

Serde provides `Serialize` implementations for many Rust primitive and
standard library types. The complete list is below. All of these can be
serialized using Serde out of the box.

Additionally, Serde provides a procedural macro called `serde_derive` to
automatically generate `Serialize` implementations for structs and enums
in your program. See the derive section of the manual for how to use this.

In rare cases it may be necessary to implement `Serialize` manually for
some type in your program. See the Implementing `Serialize` section of the
manual for more about this.

Third-party crates may provide `Serialize` implementations for types that
they expose. For example the `linked-hash-map` crate provides a
`LinkedHashMap<K, V>` type that is serializable by Serde because the crate
provides an implementation of `Serialize` for it.

**The Serializer trait**

`Serializer` implementations are provided by third-party crates, for
example `serde_json`, `serde_yaml` and `postcard`.

A partial list of well-maintained formats is given on the Serde
website.

**Implementations of Serialize provided by Serde**

- Primitive types:

- bool

- i8, i16, i32, i64, i128, isize

- u8, u16, u32, u64, u128, usize

- f32, f64

- char

- str

- &T and &mut T

- Compound types:

- [T]

- [T; 0] through [T; 32]

- tuples up to size 16

- Common standard library types:

- String

- Option<T>

- Result<T, E>

- PhantomData<T>

- Wrapper types:

- Box<T>

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

- FFI types:

- CStr

- CString

- OsStr

- OsString

- Miscellaneous standard library types:

- Duration

- SystemTime

- Path

- PathBuf

- Range<T>

- RangeInclusive<T>

- Bound<T>

- num::NonZero*

- `!` (unstable)

- Net types:

- IpAddr

...

## Items

- `Impossible` — Helper type for implementing a Serializer that does not support serializing one of the compound types.
- `Error` — Trait used by Serialize implementations to generically construct errors belonging to the Serializer against which they are currently running.
- `Serialize` — A data structure that can be serialized into any data format supported by Serde.
- `SerializeMap` — Returned from Serializer::serialize_map.
- `SerializeSeq` — Returned from Serializer::serialize_seq.
- `SerializeStruct` — Returned from Serializer::serialize_struct.
- `SerializeStructVariant` — Returned from Serializer::serialize_struct_variant.
- `SerializeTuple` — Returned from Serializer::serialize_tuple.
- `SerializeTupleStruct` — Returned from Serializer::serialize_tuple_struct.
- `SerializeTupleVariant` — Returned from Serializer::serialize_tuple_variant.
- `Serializer` — A data format that can serialize any data structure supported by Serde.

### struct serde::ser::Impossible

```rust
pub struct Impossible<Ok, Error> { /* private fields */ }
```

Helper type for implementing a `Serializer` that does not support
serializing one of the compound types.

This type cannot be instantiated, but implements every one of the traits
corresponding to the `Serializer` compound types: `SerializeSeq`,
`SerializeTuple`, `SerializeTupleStruct`, `SerializeTupleVariant`,
`SerializeMap`, `SerializeStruct`, and `SerializeStructVariant`.

ⓘ
```rust
impl Serializer for MySerializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Impossible<(), Error>;
    /* other associated types */

    /// This data format does not support serializing sequences.
    fn serialize_seq(self,
                     len: Option<usize>)
                     -> Result<Self::SerializeSeq, Error> {
        // Given Impossible cannot be instantiated, the only
        // thing we can do here is to return an error.
        Err(...)
    }

    /* other Serializer methods */
}
```

**Methods**

- `fn serialize_key<T>(&mut self, key: &T) -> Result<(), Error>where T: Serialize + ?Sized,`
- `fn serialize_value<T>(&mut self, value: &T) -> Result<(), Error>where T: Serialize + ?Sized,`
- `fn end(self) -> Result<Ok, Error>`
- `fn serialize_entry<K, V>( &mut self, key: &K, value: &V, ) -> Result<(), Self::Error>where K: Serialize + ?Sized, V: Serialize + ?Sized,`
- `fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>where T: Serialize + ?Sized,`
- `fn end(self) -> Result<Ok, Error>`
- `fn serialize_field<T>( &mut self, key: &'static str, value: &T, ) -> Result<(), Error>where T: Serialize + ?Sized,`
- `fn end(self) -> Result<Ok, Error>`
- `fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error>`
- `fn serialize_field<T>( &mut self, key: &'static str, value: &T, ) -> Result<(), Error>where T: Serialize + ?Sized,`
- `fn end(self) -> Result<Ok, Error>`
- `fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error>`
- `fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>where T: Serialize + ?Sized,`
- `fn end(self) -> Result<Ok, Error>`
- `fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>where T: Serialize + ?Sized,`
- `fn end(self) -> Result<Ok, Error>`
- `fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>where T: Serialize + ?Sized,`
- `fn end(self) -> Result<Ok, Error>`

### trait serde::ser::Error

```rust
pub trait Error: Sized + Error {
    // Required method
    fn custom<T>(msg: T) -> Self
       where T: Display;
}
```

Trait used by `Serialize` implementations to generically construct
errors belonging to the `Serializer` against which they are
currently running.

**Example implementation**

The example data format presented on the website shows an error
type appropriate for a basic JSON data format.

**Methods**

- `fn custom<T>(msg: T) -> Selfwhere T: Display,`
- `fn custom<T>(_msg: T) -> Errorwhere T: Display,`

### trait serde::ser::Serialize

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

### trait serde::ser::SerializeMap

```rust
pub trait SerializeMap {
    type Ok;
    type Error: Error;

    // Required methods
    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
       where T: Serialize + ?Sized;
    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
       where T: Serialize + ?Sized;
    fn end(self) -> Result<Self::Ok, Self::Error>;

    // Provided method
    fn serialize_entry<K, V>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), Self::Error>
       where K: Serialize + ?Sized,
             V: Serialize + ?Sized { ... }
}
```

Returned from `Serializer::serialize_map`.

**Example use**

ⓘ
```rust
use serde::ser::{Serialize, SerializeMap, Serializer};

impl<K, V> Serialize for HashMap<K, V>
where
    K: Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
```

**Example implementation**

The example data format presented on the website demonstrates an
implementation of `SerializeMap` for a basic JSON data format.

**Methods**

- `type Ok`
- `type Error: Error`
- `fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>where T: Serialize + ?Sized,`
- `fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>where T: Serialize + ?Sized,`
- `fn end(self) -> Result<Self::Ok, Self::Error>`
- `fn serialize_entry<K, V>( &mut self, key: &K, value: &V, ) -> Result<(), Self::Error>where K: Serialize + ?Sized, V: Serialize + ?Sized,`

### trait serde::ser::SerializeSeq

```rust
pub trait SerializeSeq {
    type Ok;
    type Error: Error;

    // Required methods
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
       where T: Serialize + ?Sized;
    fn end(self) -> Result<Self::Ok, Self::Error>;
}
```

Returned from `Serializer::serialize_seq`.

**Example use**

ⓘ
```rust
use serde::ser::{Serialize, SerializeSeq, Serializer};

impl<T> Serialize for Vec<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for element in self {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}
```

**Example implementation**

The example data format presented on the website demonstrates an
implementation of `SerializeSeq` for a basic JSON data format.

**Methods**

- `type Ok`
- `type Error: Error`
- `fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>where T: Serialize + ?Sized,`
- `fn end(self) -> Result<Self::Ok, Self::Error>`

### trait serde::ser::SerializeStruct

```rust
pub trait SerializeStruct {
    type Ok;
    type Error: Error;

    // Required methods
    fn serialize_field<T>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
       where T: Serialize + ?Sized;
    fn end(self) -> Result<Self::Ok, Self::Error>;

    // Provided method
    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> { ... }
}
```

Returned from `Serializer::serialize_struct`.

**Example use**

ⓘ
```rust
use serde::ser::{Serialize, SerializeStruct, Serializer};

struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Serialize for Rgb {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut rgb = serializer.serialize_struct("Rgb", 3)?;
        rgb.serialize_field("r", &self.r)?;
        rgb.serialize_field("g", &self.g)?;
        rgb.serialize_field("b", &self.b)?;
        rgb.end()
    }
}
```

**Example implementation**

The example data format presented on the website demonstrates an
implementation of `SerializeStruct` for a basic JSON data format.

**Methods**

- `type Ok`
- `type Error: Error`
- `fn serialize_field<T>( &mut self, key: &'static str, value: &T, ) -> Result<(), Self::Error>where T: Serialize + ?Sized,`
- `fn end(self) -> Result<Self::Ok, Self::Error>`
- `fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error>`

### trait serde::ser::SerializeStructVariant

```rust
pub trait SerializeStructVariant {
    type Ok;
    type Error: Error;

    // Required methods
    fn serialize_field<T>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
       where T: Serialize + ?Sized;
    fn end(self) -> Result<Self::Ok, Self::Error>;

    // Provided method
    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> { ... }
}
```

Returned from `Serializer::serialize_struct_variant`.

**Example use**

ⓘ
```rust
use serde::ser::{Serialize, SerializeStructVariant, Serializer};

enum E {
    S { r: u8, g: u8, b: u8 },
}

impl Serialize for E {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            E::S {
                ref r,
                ref g,
                ref b,
            } => {
                let mut sv = serializer.serialize_struct_variant("E", 0, "S", 3)?;
                sv.serialize_field("r", r)?;
                sv.serialize_field("g", g)?;
                sv.serialize_field("b", b)?;
                sv.end()
            }
        }
    }
}
```

**Example implementation**

The example data format presented on the website demonstrates an
implementation of `SerializeStructVariant` for a basic JSON data format.

**Methods**

- `type Ok`
- `type Error: Error`
- `fn serialize_field<T>( &mut self, key: &'static str, value: &T, ) -> Result<(), Self::Error>where T: Serialize + ?Sized,`
- `fn end(self) -> Result<Self::Ok, Self::Error>`
- `fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error>`

### trait serde::ser::SerializeTuple

```rust
pub trait SerializeTuple {
    type Ok;
    type Error: Error;

    // Required methods
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
       where T: Serialize + ?Sized;
    fn end(self) -> Result<Self::Ok, Self::Error>;
}
```

Returned from `Serializer::serialize_tuple`.

**Example use**

ⓘ
```rust
use serde::ser::{Serialize, SerializeTuple, Serializer};

impl<A, B, C> Serialize for (A, B, C)
where
    A: Serialize,
    B: Serialize,
    C: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tup = serializer.serialize_tuple(3)?;
        tup.serialize_element(&self.0)?;
        tup.serialize_element(&self.1)?;
        tup.serialize_element(&self.2)?;
        tup.end()
    }
}
```

ⓘ
```rust
use serde::ser::{Serialize, SerializeTuple, Serializer};

impl<T> Serialize for [T; 16]
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_tuple(16)?;
        for element in self {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}
```

**Example implementation**

The example data format presented on the website demonstrates an
implementation of `SerializeTuple` for a basic JSON data format.

**Methods**

- `type Ok`
- `type Error: Error`
- `fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>where T: Serialize + ?Sized,`
- `fn end(self) -> Result<Self::Ok, Self::Error>`

### trait serde::ser::SerializeTupleStruct

```rust
pub trait SerializeTupleStruct {
    type Ok;
    type Error: Error;

    // Required methods
    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
       where T: Serialize + ?Sized;
    fn end(self) -> Result<Self::Ok, Self::Error>;
}
```

Returned from `Serializer::serialize_tuple_struct`.

**Example use**

ⓘ
```rust
use serde::ser::{Serialize, SerializeTupleStruct, Serializer};

struct Rgb(u8, u8, u8);

impl Serialize for Rgb {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ts = serializer.serialize_tuple_struct("Rgb", 3)?;
        ts.serialize_field(&self.0)?;
        ts.serialize_field(&self.1)?;
        ts.serialize_field(&self.2)?;
        ts.end()
    }
}
```

**Example implementation**

The example data format presented on the website demonstrates an
implementation of `SerializeTupleStruct` for a basic JSON data format.

**Methods**

- `type Ok`
- `type Error: Error`
- `fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>where T: Serialize + ?Sized,`
- `fn end(self) -> Result<Self::Ok, Self::Error>`

### trait serde::ser::SerializeTupleVariant

```rust
pub trait SerializeTupleVariant {
    type Ok;
    type Error: Error;

    // Required methods
    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
       where T: Serialize + ?Sized;
    fn end(self) -> Result<Self::Ok, Self::Error>;
}
```

Returned from `Serializer::serialize_tuple_variant`.

**Example use**

ⓘ
```rust
use serde::ser::{Serialize, SerializeTupleVariant, Serializer};

enum E {
    T(u8, u8),
    U(String, u32, u32),
}

impl Serialize for E {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            E::T(ref a, ref b) => {
                let mut tv = serializer.serialize_tuple_variant("E", 0, "T", 2)?;
                tv.serialize_field(a)?;
                tv.serialize_field(b)?;
                tv.end()
            }
            E::U(ref a, ref b, ref c) => {
                let mut tv = serializer.serialize_tuple_variant("E", 1, "U", 3)?;
                tv.serialize_field(a)?;
                tv.serialize_field(b)?;
                tv.serialize_field(c)?;
                tv.end()
            }
        }
    }
}
```

**Example implementation**

The example data format presented on the website demonstrates an
implementation of `SerializeTupleVariant` for a basic JSON data format.

**Methods**

- `type Ok`
- `type Error: Error`
- `fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>where T: Serialize + ?Sized,`
- `fn end(self) -> Result<Self::Ok, Self::Error>`

### trait serde::ser::Serializer

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
