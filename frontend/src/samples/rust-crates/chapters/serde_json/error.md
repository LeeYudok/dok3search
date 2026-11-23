# Module serde_json::error

## Overview

When serializing or deserializing JSON goes wrong.

## Items

- `Error` — This type represents all possible errors that can occur when serializing or deserializing JSON data.
- `Category` — Categorizes the cause of a serde_json::Error.
- `Result` — Alias for a Result with the error type serde_json::Error.

### enum serde_json::error::Category

```rust
pub enum Category {
    Io,
    Syntax,
    Data,
    Eof,
}
```

Categorizes the cause of a `serde_json::Error`.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### struct serde_json::error::Error

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

### type serde_json::error::Result

```rust
pub type Result<T> = Result<T, Error>;
```

Alias for a `Result` with the error type `serde_json::Error`.
