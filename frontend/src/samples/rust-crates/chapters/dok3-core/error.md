# Module dok3-core::error

## Overview

Shared error definitions for dok3search crates.

## Items

- `Dok3Error` — Domain and infrastructure boundary errors mapped consistently across API and adapters.

### enum dok3-core::error::Dok3Error

```rust
pub enum Dok3Error {
    InvalidInput(String),
    NotFound(String),
    Forbidden(String),
    Parse(String),
    Index(String),
    Storage(String),
    Internal(String),
}
```

Domain and infrastructure boundary errors mapped consistently across API and adapters.

**Methods**

- `pub fn code(&self) -> &'static str`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, __formatter: &mut Formatter<'_>) -> Result`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
