# Module rusqlite::types

## Overview

Traits dealing with SQLite data types.

SQLite uses a dynamic type system. Implementations of
the `ToSql` and `FromSql` traits are provided for the basic types that
SQLite provides methods for:

- Strings (`String` and `&str`)

- Blobs (`Vec<u8>` and `&[u8]`)

- Numbers

The number situation is a little complicated due to the fact that all
numbers in SQLite are stored as `INTEGER` (`i64`) or `REAL` (`f64`).

`ToSql` and `FromSql` are implemented for all primitive number types.
`FromSql` has different behaviour depending on the SQL and Rust types, and
the value.

- `INTEGER` to integer: returns an
`Error::IntegralValueOutOfRange`
error if the value does not fit in the Rust type.

- `REAL` to integer: always returns an
`Error::InvalidColumnType` error.

- `INTEGER` to float: casts using `as` operator. Never fails.

- `REAL` to float: casts using `as` operator. Never fails.

`ToSql` always succeeds except when storing a `u64` or `usize` value that
cannot fit in an `INTEGER` (`i64`). Also note that SQLite ignores column
types, so if you store an `i64` in a column with type `REAL` it will be
stored as an `INTEGER`, not a `REAL` (unless the column is part of a
STRICT table).

If the `time` feature is enabled, implementations are
provided for `time::OffsetDateTime` that use the RFC 3339 date/time format,
`"%Y-%m-%dT%H:%M:%S.%fZ"`, to store time values as strings.  These values
can be parsed by SQLite’s builtin
datetime functions.  If you
want different storage for datetimes, you can use a newtype.
`ToSql` and `FromSql` are also implemented for `Option<T>` where `T`
implements `ToSql` or `FromSql` for the cases where you want to know if
a value was NULL (which gets translated to `None`).

## Items

- `Null` — Empty struct that can be used to fill in a query parameter as NULL.
- `FromSqlError` — Enum listing possible errors from FromSql trait.
- `ToSqlOutput` — ToSqlOutput represents the possible output types for implementers of the ToSql trait.
- `Type` — SQLite data types. See Fundamental Datatypes.
- `Value` — Owning dynamic type value. Value’s type is typically dictated by SQLite (not by the caller).
- `ValueRef` — A non-owning dynamic type value. Typically, the memory backing this value is owned by SQLite.
- `FromSql` — A trait for types that can be created from a SQLite value.
- `ToSql` — A trait for types that can be converted into SQLite values. Returns crate::Error::ToSqlConversionFailure if the conversion fails.
- `FromSqlResult` — Result type for implementors of the FromSql trait.

### enum rusqlite::types::FromSqlError

```rust
#[non_exhaustive]pub enum FromSqlError {
    InvalidType,
    OutOfRange(i64),
    Utf8Error(Utf8Error),
    InvalidBlobSize {
        expected_size: usize,
        blob_size: usize,
    },
    Other(Box<dyn Error + Send + Sync + 'static>),
}
```

Enum listing possible errors from `FromSql` trait.

**Methods**

- `pub fn other<E: Error + Send + Sync + 'static>(error: E) -> Self`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(err: FromSqlError) -> Self`
- `fn from(err: Utf8Error) -> Self`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### enum rusqlite::types::ToSqlOutput

```rust
#[non_exhaustive]pub enum ToSqlOutput<'a> {
    Borrowed(ValueRef<'a>),
    Owned(Value),
}
```

`ToSqlOutput` represents the possible output types for implementers of the
`ToSql` trait.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn from(t: &'a T) -> Self`
- `fn from(t: NonZeroI8) -> Self`
- `fn from(t: NonZeroI16) -> Self`
- `fn from(t: NonZeroI32) -> Self`
- `fn from(t: NonZeroI64) -> Self`
- `fn from(t: NonZeroIsize) -> Self`
- `fn from(t: NonZeroU8) -> Self`
- `fn from(t: NonZeroU16) -> Self`
- `fn from(t: NonZeroU32) -> Self`
- `fn from(t: Null) -> Self`
- `fn from(t: String) -> Self`
- `fn from(t: Vec<u8>) -> Self`
- `fn from(t: bool) -> Self`
- `fn from(t: f32) -> Self`
- `fn from(t: f64) -> Self`
- `fn from(t: i8) -> Self`
- `fn from(t: i16) -> Self`
- `fn from(t: i32) -> Self`
- `fn from(t: i64) -> Self`
- `fn from(t: isize) -> Self`
- `fn from(t: u8) -> Self`
- `fn from(t: u16) -> Self`
- `fn from(t: u32) -> Self`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`

### enum rusqlite::types::Type

```rust
pub enum Type {
    Null,
    Integer,
    Real,
    Text,
    Blob,
}
```

SQLite data types.
See Fundamental Datatypes.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### enum rusqlite::types::Value

```rust
pub enum Value {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
}
```

Owning dynamic type value. Value’s type is typically
dictated by SQLite (not by the caller).

See `ValueRef` for a non-owning dynamic type
value.

**Methods**

- `pub fn data_type(&self) -> Type`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn from(value: &'a Value) -> Self`
- `fn from(_: Null) -> Self`
- `fn from(v: Option<T>) -> Self`
- `fn from(s: String) -> Self`
- `fn from(v: Vec<u8>) -> Self`
- `fn from(i: bool) -> Self`
- `fn from(f: f32) -> Self`
- `fn from(f: f64) -> Self`
- `fn from(i: i8) -> Value`
- `fn from(i: i16) -> Value`
- `fn from(i: i32) -> Value`
- `fn from(i: i64) -> Self`
- `fn from(i: isize) -> Self`
- `fn from(i: u8) -> Value`
- `fn from(i: u16) -> Value`
- `fn from(i: u32) -> Value`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn try_from(borrowed: ValueRef<'_>) -> Result<Self, Self::Error>`

### enum rusqlite::types::ValueRef

```rust
pub enum ValueRef<'a> {
    Null,
    Integer(i64),
    Real(f64),
    Text(&'a [u8]),
    Blob(&'a [u8]),
}
```

A non-owning dynamic type value. Typically, the
memory backing this value is owned by SQLite.

See `Value` for an owning dynamic type value.

**Methods**

- `pub fn data_type(&self) -> Type`
- `pub fn as_i64(&self) -> FromSqlResult<i64>`
- `pub fn as_i64_or_null(&self) -> FromSqlResult<Option<i64>>`
- `pub fn as_f64(&self) -> FromSqlResult<f64>`
- `pub fn as_f64_or_null(&self) -> FromSqlResult<Option<f64>>`
- `pub fn as_str(&self) -> FromSqlResult<&'a str>`
- `pub fn as_str_or_null(&self) -> FromSqlResult<Option<&'a str>>`
- `pub fn as_blob(&self) -> FromSqlResult<&'a [u8]>`
- `pub fn as_blob_or_null(&self) -> FromSqlResult<Option<&'a [u8]>>`
- `pub fn as_bytes(&self) -> FromSqlResult<&'a [u8]>`
- `pub fn as_bytes_or_null(&self) -> FromSqlResult<Option<&'a [u8]>>`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn from(value: &'a Value) -> Self`
- `fn from(s: &[u8]) -> ValueRef<'_>`
- `fn from(s: &str) -> ValueRef<'_>`
- `fn from(s: Option<T>) -> Self`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn try_from(borrowed: ValueRef<'_>) -> Result<Self, Self::Error>`

### struct rusqlite::types::Null

```rust
pub struct Null;
```

Empty struct that can be used to fill in a query parameter as `NULL`.

**Example**

```rust

fn insert_null(conn: &Connection) -> Result<usize> {
    conn.execute("INSERT INTO people (name) VALUES (?1)", [Null])
}
```

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn from(t: Null) -> Self`
- `fn from(_: Null) -> Self`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`

### trait rusqlite::types::FromSql

```rust
pub trait FromSql: Sized {
    // Required method
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>;
}
```

A trait for types that can be created from a SQLite value.

**Methods**

- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`
- `fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>`

### trait rusqlite::types::ToSql

```rust
pub trait ToSql {
    // Required method
    fn to_sql(&self) -> Result<ToSqlOutput<'_>>;
}
```

A trait for types that can be converted into SQLite values. Returns
`crate::Error::ToSqlConversionFailure` if the conversion fails.

**Methods**

- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`
- `fn to_sql(&self) -> Result<ToSqlOutput<'_>>`

### type rusqlite::types::FromSqlResult

```rust
pub type FromSqlResult<T> = Result<T, FromSqlError>;
```

Result type for implementors of the `FromSql` trait.
