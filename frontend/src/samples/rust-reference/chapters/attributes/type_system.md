# Type system attributes

[attributes.type-system](#r-attributes.type-system)

# Type system attributes

[attributes.type-system.intro](#r-attributes.type-system.intro)

The following [attributes](../attributes.md) are used for changing how a type can be used.

[attributes.type-system.non_exhaustive](#r-attributes.type-system.non_exhaustive)

## The `non_exhaustive` attribute

[attributes.type-system.non_exhaustive.intro](#r-attributes.type-system.non_exhaustive.intro)

The *`non_exhaustive` attribute* indicates that a type or variant may have more fields or variants added in the future.

[attributes.type-system.non_exhaustive.allowed-positions](#r-attributes.type-system.non_exhaustive.allowed-positions)

It can be applied to [`struct`s](../items/structs.md), [`enum`s](../items/enumerations.md), and `enum` variants.

[attributes.type-system.non_exhaustive.syntax](#r-attributes.type-system.non_exhaustive.syntax)

The `non_exhaustive` attribute uses the [MetaWord](../attributes.md#grammar-MetaWord) syntax and thus does not take any inputs.

[attributes.type-system.non_exhaustive.same-crate](#r-attributes.type-system.non_exhaustive.same-crate)

Within the defining crate, `non_exhaustive` has no effect.

``` playground
#![allow(unused)]
fn main() {
#[non_exhaustive]
pub struct Config {
    pub window_width: u16,
    pub window_height: u16,
}

#[non_exhaustive]
pub struct Token;

#[non_exhaustive]
pub struct Id(pub u64);

#[non_exhaustive]
pub enum Error {
    Message(String),
    Other,
}

pub enum Message {
    #[non_exhaustive] Send { from: u32, to: u32, contents: String },
    #[non_exhaustive] Reaction(u32),
    #[non_exhaustive] Quit,
}

// Non-exhaustive structs can be constructed as normal within the defining crate.
let config = Config { window_width: 640, window_height: 480 };
let token = Token;
let id = Id(4);

// Non-exhaustive structs can be matched on exhaustively within the defining crate.
let Config { window_width, window_height } = config;
let Token = token;
let Id(id_number) = id;

let error = Error::Other;
let message = Message::Reaction(3);

// Non-exhaustive enums can be matched on exhaustively within the defining crate.
match error {
    Error::Message(ref s) => { },
    Error::Other => { },
}

match message {
    // Non-exhaustive variants can be matched on exhaustively within the defining crate.
    Message::Send { from, to, contents } => { },
    Message::Reaction(id) => { },
    Message::Quit => { },
}
}
```

[attributes.type-system.non_exhaustive.external-crate](#r-attributes.type-system.non_exhaustive.external-crate)

Outside of the defining crate, types annotated with `non_exhaustive` have limitations that preserve backwards compatibility when new fields or variants are added.

[attributes.type-system.non_exhaustive.construction](#r-attributes.type-system.non_exhaustive.construction)

Non-exhaustive types cannot be constructed outside of the defining crate:

- Non-exhaustive variants ([`struct`](../items/structs.md) or [`enum` variant](../items/enumerations.md)) cannot be constructed with a [StructExpression](../expressions/struct-expr.md#grammar-StructExpression) (including with [functional update syntax](../expressions/struct-expr.md#functional-update-syntax)).
- The implicitly defined same-named constant of a [unit-like struct](../items/structs.md), or the same-named constructor function of a [tuple struct](../items/structs.md), has a [visibility](../visibility-and-privacy.md) no greater than `pub(crate)`. That is, if the struct’s visibility is `pub`, then the constant or constructor’s visibility is `pub(crate)`, and otherwise the visibility of the two items is the same (as is the case without `#[non_exhaustive]`).
- [`enum`](../items/enumerations.md) instances can be constructed.

The following examples of construction do not compile when outside the defining crate:

``` rust
// These are types defined in an upstream crate that have been annotated as
// `#[non_exhaustive]`.
use upstream::{Config, Token, Id, Error, Message};

// Cannot construct an instance of `Config`; if new fields were added in
// a new version of `upstream` then this would fail to compile, so it is
// disallowed.
let config = Config { window_width: 640, window_height: 480 };

// Cannot construct an instance of `Token`; if new fields were added, then
// it would not be a unit-like struct any more, so the same-named constant
// created by it being a unit-like struct is not public outside the crate;
// this code fails to compile.
let token = Token;

// Cannot construct an instance of `Id`; if new fields were added, then
// its constructor function signature would change, so its constructor
// function is not public outside the crate; this code fails to compile.
let id = Id(5);

// Can construct an instance of `Error`; new variants being introduced would
// not result in this failing to compile.
let error = Error::Message("foo".to_string());

// Cannot construct an instance of `Message::Send` or `Message::Reaction`;
// if new fields were added in a new version of `upstream` then this would
// fail to compile, so it is disallowed.
let message = Message::Send { from: 0, to: 1, contents: "foo".to_string(), };
let message = Message::Reaction(0);

// Cannot construct an instance of `Message::Quit`; if this were converted to
// a tuple enum variant `upstream`, this would fail to compile.
let message = Message::Quit;
```

[attributes.type-system.non_exhaustive.match](#r-attributes.type-system.non_exhaustive.match)

There are limitations when matching on non-exhaustive types outside of the defining crate:

- When pattern matching on a non-exhaustive variant ([`struct`](../items/structs.md) or [`enum` variant](../items/enumerations.md)), a [StructPattern](../patterns.md#grammar-StructPattern) must be used which must include a `..`. A tuple enum variant’s constructor’s [visibility](../visibility-and-privacy.md) is reduced to be no greater than `pub(crate)`.
- When pattern matching on a non-exhaustive [`enum`](../items/enumerations.md), matching on a variant does not contribute towards the exhaustiveness of the arms. The following examples of matching do not compile when outside the defining crate:

``` rust
// These are types defined in an upstream crate that have been annotated as
// `#[non_exhaustive]`.
use upstream::{Config, Token, Id, Error, Message};

// Cannot match on a non-exhaustive enum without including a wildcard arm.
match error {
  Error::Message(ref s) => { },
  Error::Other => { },
  // would compile with: `_ => {},`
}

// Cannot match on a non-exhaustive struct without a wildcard.
if let Ok(Config { window_width, window_height }) = config {
    // would compile with: `..`
}

// Cannot match a non-exhaustive unit-like or tuple struct except by using
// braced struct syntax with a wildcard.
// This would compile as `let Token { .. } = token;`
let Token = token;
// This would compile as `let Id { 0: id_number, .. } = id;`
let Id(id_number) = id;

match message {
  // Cannot match on a non-exhaustive struct enum variant without including a wildcard.
  Message::Send { from, to, contents } => { },
  // Cannot match on a non-exhaustive tuple or unit enum variant.
  Message::Reaction(type) => { },
  Message::Quit => { },
}
```

It’s also not allowed to use numeric casts (`as`) on enums that contain any non-exhaustive variants.

For example, the following enum can be cast because it doesn’t contain any non-exhaustive variants:

``` playground
#![allow(unused)]
fn main() {
#[non_exhaustive]
pub enum Example {
    First,
    Second,
}
}
```

However, if the enum contains even a single non-exhaustive variant, casting will result in an error. Consider this modified version of the same enum:

``` playground
#![allow(unused)]
fn main() {
#[non_exhaustive]
pub enum EnumWithNonExhaustiveVariants {
    First,
    #[non_exhaustive]
    Second,
}
}
```

``` rust
use othercrate::EnumWithNonExhaustiveVariants;

// Error: cannot cast an enum with a non-exhaustive variant when it's defined in another crate
let _ = EnumWithNonExhaustiveVariants::First as u8;
```

Non-exhaustive types are always considered inhabited in downstream crates.
