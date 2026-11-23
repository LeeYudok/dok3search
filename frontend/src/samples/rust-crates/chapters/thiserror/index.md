# Crate thiserror

## Overview

This library provides a convenient derive macro for the standard library’s
`std::error::Error` trait.

**Example**

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown data store error")]
    Unknown,
}
```

**Details**

- 

Thiserror deliberately does not appear in your public API. You get the
same thing as if you had written an implementation of
`std::error::Error` by hand, and switching from handwritten impls to
thiserror or vice versa is not a breaking change.

- 

Errors may be enums, structs with named fields, tuple structs, or unit
structs.

- 

A `Display` impl is generated for your error if you provide
`#[error("...")]` messages on the struct or each variant of your enum, as
shown above in the example.

The messages support a shorthand for interpolating fields from the error.

- `#[error("{var}")]` ⟶ `write!("{}", self.var)`

- `#[error("{0}")]` ⟶ `write!("{}", self.0)`

- `#[error("{var:?}")]` ⟶ `write!("{:?}", self.var)`

- `#[error("{0:?}")]` ⟶ `write!("{:?}", self.0)`

These shorthands can be used together with any additional format args,
which may be arbitrary expressions. For example:

```rust
#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid rdo_lookahead_frames {0} (expected < {max})", max = i32::MAX)]
    InvalidLookahead(u32),
}
```

If one of the additional expression arguments needs to refer to a field of
the struct or enum, then refer to named fields as `.var` and tuple fields
as `.0`.

```rust
#[derive(Error, Debug)]
pub enum Error {
    #[error("first letter must be lowercase but was {:?}", first_char(.0))]
    WrongCase(String),
    #[error("invalid index {idx}, expected at least {} and at most {}", .limits.lo, .limits.hi)]
    OutOfBounds { idx: usize, limits: Limits },
}
```

- 

A `From` impl is generated for each variant that contains a `#[from]`
attribute.

The variant using `#[from]` must not contain any other fields beyond the
source error (and possibly a backtrace — see below). Usually
`#[from]` fields are unnamed, but `#[from]` is allowed on a named field
too.

...
