# Diagnostic attributes

[attributes.diagnostics](#r-attributes.diagnostics)

# Diagnostic attributes

[attributes.diagnostics.intro](#r-attributes.diagnostics.intro)

The following [attributes](../attributes.md) are used for controlling or generating diagnostic messages during compilation.

[attributes.diagnostics.lint](#r-attributes.diagnostics.lint)

## Lint check attributes

[attributes.diagnostics.lint.intro](#r-attributes.diagnostics.lint.intro)

A lint check names a potentially undesirable coding pattern, such as unreachable code or omitted documentation.

[attributes.diagnostics.lint.level](#r-attributes.diagnostics.lint.level)

The lint attributes `allow`, `expect`, `warn`, `deny`, and `forbid` use the [MetaListPaths](../attributes.md#grammar-MetaListPaths) syntax to specify a list of lint names to change the lint level for the entity to which the attribute applies.

For any lint check `C`:

[attributes.diagnostics.lint.allow](#r-attributes.diagnostics.lint.allow)

- `#[allow(C)]` overrides the check for `C` so that violations will go unreported.

[attributes.diagnostics.lint.expect](#r-attributes.diagnostics.lint.expect)

- `#[expect(C)]` indicates that lint `C` is expected to be emitted. The attribute will suppress the emission of `C` or issue a warning, if the expectation is unfulfilled.

[attributes.diagnostics.lint.warn](#r-attributes.diagnostics.lint.warn)

- `#[warn(C)]` warns about violations of `C` but continues compilation.

[attributes.diagnostics.lint.deny](#r-attributes.diagnostics.lint.deny)

- `#[deny(C)]` signals an error after encountering a violation of `C`,

[attributes.diagnostics.lint.forbid](#r-attributes.diagnostics.lint.forbid)

- `#[forbid(C)]` is the same as `deny(C)`, but also forbids changing the lint level afterwards,

> ![](../../assets/inline/diagnostics-1.svg)Note
>
> The lint checks supported by `rustc` can be found via `rustc -W help`, along with their default settings and are documented in the [rustc book](../../rustc/lints/index.html).

``` playground
#![allow(unused)]
fn main() {
pub mod m1 {
    // Missing documentation is ignored here
    #[allow(missing_docs)]
    pub fn undocumented_one() -> i32 { 1 }

    // Missing documentation signals a warning here
    #[warn(missing_docs)]
    pub fn undocumented_too() -> i32 { 2 }

    // Missing documentation signals an error here
    #[deny(missing_docs)]
    pub fn undocumented_end() -> i32 { 3 }
}
}
```

[attributes.diagnostics.lint.override](#r-attributes.diagnostics.lint.override)

Lint attributes can override the level specified from a previous attribute, as long as the level does not attempt to change a forbidden lint (except for `deny`, which is allowed inside a `forbid` context, but ignored). Previous attributes are those from a higher level in the syntax tree, or from a previous attribute on the same entity as listed in left-to-right source order.

This example shows how one can use `allow` and `warn` to toggle a particular check on and off:

``` playground
#![allow(unused)]
fn main() {
#[warn(missing_docs)]
pub mod m2 {
    #[allow(missing_docs)]
    pub mod nested {
        // Missing documentation is ignored here
        pub fn undocumented_one() -> i32 { 1 }

        // Missing documentation signals a warning here,
        // despite the allow above.
        #[warn(missing_docs)]
        pub fn undocumented_two() -> i32 { 2 }
    }

    // Missing documentation signals a warning here
    pub fn undocumented_too() -> i32 { 3 }
}
}
```

This example shows how one can use `forbid` to disallow uses of `allow` or `expect` for that lint check:

``` playground
#![allow(unused)]
fn main() {
#[forbid(missing_docs)]
pub mod m3 {
    // Attempting to toggle warning signals an error here
    #[allow(missing_docs)]
    /// Returns 2.
    pub fn undocumented_too() -> i32 { 2 }
}
}
```

> ![](../../assets/inline/diagnostics-2.svg)Note
>
> `rustc` allows setting lint levels on the [command-line](../../rustc/lints/levels.html#via-compiler-flag), and also supports [setting caps](../../rustc/lints/levels.html#capping-lints) on the lints that are reported.

[attributes.diagnostics.lint.reason](#r-attributes.diagnostics.lint.reason)

### Lint reasons

All lint attributes support an additional `reason` parameter, to give context why a certain attribute was added. This reason will be displayed as part of the lint message if the lint is emitted at the defined level.

``` playground
#![allow(unused)]
fn main() {
// `keyword_idents` is allowed by default. Here we deny it to
// avoid migration of identifiers when we update the edition.
#![deny(
    keyword_idents,
    reason = "we want to avoid these idents to be future compatible"
)]

// This name was allowed in Rust's 2015 edition. We still aim to avoid
// this to be future compatible and not confuse end users.
fn dyn() {}
}
```

Here is another example, where the lint is allowed with a reason:

``` playground
#![allow(unused)]
fn main() {
use std::path::PathBuf;

pub fn get_path() -> PathBuf {
    // The `reason` parameter on `allow` attributes acts as documentation for the reader.
    #[allow(unused_mut, reason = "this is only modified on some platforms")]
    let mut file_name = PathBuf::from("git");

    #[cfg(target_os = "windows")]
    file_name.set_extension("exe");

    file_name
}
}
```

[attributes.diagnostics.expect](#r-attributes.diagnostics.expect)

### The `#[expect]` attribute

[attributes.diagnostics.expect.intro](#r-attributes.diagnostics.expect.intro)

The `#[expect(C)]` attribute creates a lint expectation for lint `C`. The expectation will be fulfilled, if a `#[warn(C)]` attribute at the same location would result in a lint emission. If the expectation is unfulfilled, because lint `C` would not be emitted, the `unfulfilled_lint_expectations` lint will be emitted at the attribute.

``` playground
fn main() {
    // This `#[expect]` attribute creates a lint expectation, that the `unused_variables`
    // lint would be emitted by the following statement. This expectation is
    // unfulfilled, since the `question` variable is used by the `println!` macro.
    // Therefore, the `unfulfilled_lint_expectations` lint will be emitted at the
    // attribute.
    #[expect(unused_variables)]
    let question = "who lives in a pineapple under the sea?";
    println!("{question}");

    // This `#[expect]` attribute creates a lint expectation that will be fulfilled, since
    // the `answer` variable is never used. The `unused_variables` lint, that would usually
    // be emitted, is suppressed. No warning will be issued for the statement or attribute.
    #[expect(unused_variables)]
    let answer = "SpongeBob SquarePants!";
}
```

[attributes.diagnostics.expect.fulfillment](#r-attributes.diagnostics.expect.fulfillment)

The lint expectation is only fulfilled by lint emissions which have been suppressed by the `expect` attribute. If the lint level is modified in the scope with other level attributes like `allow` or `warn`, the lint emission will be handled accordingly and the expectation will remain unfulfilled.

``` playground
#![allow(unused)]
fn main() {
#[expect(unused_variables)]
fn select_song() {
    // This will emit the `unused_variables` lint at the warn level
    // as defined by the `warn` attribute. This will not fulfill the
    // expectation above the function.
    #[warn(unused_variables)]
    let song_name = "Crab Rave";

    // The `allow` attribute suppresses the lint emission. This will not
    // fulfill the expectation as it has been suppressed by the `allow`
    // attribute and not the `expect` attribute above the function.
    #[allow(unused_variables)]
    let song_creator = "Noisestorm";

    // This `expect` attribute will suppress the `unused_variables` lint emission
    // at the variable. The `expect` attribute above the function will still not
    // be fulfilled, since this lint emission has been suppressed by the local
    // expect attribute.
    #[expect(unused_variables)]
    let song_version = "Monstercat Release";
}
}
```

[attributes.diagnostics.expect.independent](#r-attributes.diagnostics.expect.independent)

If the `expect` attribute contains several lints, each one is expected separately. For a lint group it’s enough if one lint inside the group has been emitted:

``` playground
#![allow(unused)]
fn main() {
// This expectation will be fulfilled by the unused value inside the function
// since the emitted `unused_variables` lint is inside the `unused` lint group.
#[expect(unused)]
pub fn thoughts() {
    let unused = "I'm running out of examples";
}

pub fn another_example() {
    // This attribute creates two lint expectations. The `unused_mut` lint will be
    // suppressed and with that fulfill the first expectation. The `unused_variables`
    // wouldn't be emitted, since the variable is used. That expectation will therefore
    // be unsatisfied, and a warning will be emitted.
    #[expect(unused_mut, unused_variables)]
    let mut link = "https://www.rust-lang.org/";

    println!("Welcome to our community: {link}");
}
}
```

> ![](../../assets/inline/diagnostics-3.svg)Note
>
> The behavior of `#[expect(unfulfilled_lint_expectations)]` is currently defined to always generate the `unfulfilled_lint_expectations` lint.

[attributes.diagnostics.lint.group](#r-attributes.diagnostics.lint.group)

### Lint groups

[attributes.diagnostics.lint.group.intro](#r-attributes.diagnostics.lint.group.intro)

Lints may be organized into named groups so that the level of related lints can be adjusted together.

[attributes.diagnostics.lint.group.equivalence](#r-attributes.diagnostics.lint.group.equivalence)

Using a named group is equivalent to listing out the lints within that group.

``` playground
#![allow(unused)]
fn main() {
// This allows all lints in the "unused" group.
#[allow(unused)]
// This overrides the "unused_must_use" lint from the "unused"
// group to deny.
#[deny(unused_must_use)]
fn example() {
    // This does not generate a warning because the "unused_variables"
    // lint is in the "unused" group.
    let x = 1;
    // This generates an error because the result is unused and
    // "unused_must_use" is marked as "deny".
    std::fs::remove_file("some_file"); // ERROR: unused `Result` that must be used
}
}
```

[attributes.diagnostics.lint.group.warnings](#r-attributes.diagnostics.lint.group.warnings)

There is a special group named “warnings” which includes all lints at the “warn” level. The “warnings” group ignores attribute order and applies to all lints that would otherwise warn within the entity.

``` playground
#![allow(unused)]
fn main() {
unsafe fn an_unsafe_fn() {}
// The order of these two attributes does not matter.
#[deny(warnings)]
// The unsafe_code lint is normally "allow" by default.
#[warn(unsafe_code)]
fn example_err() {
    // This is an error because the `unsafe_code` warning has
    // been lifted to "deny".
    unsafe { an_unsafe_fn() } // ERROR: use of `unsafe` block
}
}
```

[attributes.diagnostics.lint.tool](#r-attributes.diagnostics.lint.tool)

### Tool lint attributes

[attributes.diagnostics.lint.tool.intro](#r-attributes.diagnostics.lint.tool.intro)

Tool lints allows using scoped lints, to `allow`, `warn`, `deny` or `forbid` lints of certain tools.

[attributes.diagnostics.lint.tool.activation](#r-attributes.diagnostics.lint.tool.activation)

Tool lints only get checked when the associated tool is active. If a lint attribute, such as `allow`, references a nonexistent tool lint, the compiler will not warn about the nonexistent lint until you use the tool.

Otherwise, they work just like regular lint attributes:

``` playground
// set the entire `pedantic` clippy lint group to warn
#![warn(clippy::pedantic)]
// silence warnings from the `filter_map` clippy lint
#![allow(clippy::filter_map)]

fn main() {
    // ...
}

// silence the `cmp_nan` clippy lint just for this function
#[allow(clippy::cmp_nan)]
fn foo() {
    // ...
}
```

> ![](../../assets/inline/diagnostics-4.svg)Note
>
> `rustc` currently recognizes the tool lints for “[clippy](https://github.com/rust-lang/rust-clippy)” and “[rustdoc](../../rustdoc/lints.html)”.

[attributes.diagnostics.deprecated](#r-attributes.diagnostics.deprecated)

## The `deprecated` attribute

[attributes.diagnostics.deprecated.intro](#r-attributes.diagnostics.deprecated.intro)

The *`deprecated` attribute* marks an item as deprecated. `rustc` will issue warnings on use of `#[deprecated]` items. `rustdoc` will show item deprecation, including the `since` version and `note`, if available.

[attributes.diagnostics.deprecated.syntax](#r-attributes.diagnostics.deprecated.syntax)

The `deprecated` attribute has several forms:

- `deprecated` — Issues a generic message.
- `deprecated = "message"` — Includes the given string in the deprecation message.
- [MetaListNameValueStr](../attributes.md#grammar-MetaListNameValueStr) syntax with two optional fields:
  - `since` — Specifies a version number when the item was deprecated. `rustc` does not currently interpret the string, but external tools like [Clippy](https://github.com/rust-lang/rust-clippy) may check the validity of the value.
  - `note` — Specifies a string that should be included in the deprecation message. This is typically used to provide an explanation about the deprecation and preferred alternatives.

[attributes.diagnostics.deprecated.allowed-positions](#r-attributes.diagnostics.deprecated.allowed-positions)

The `deprecated` attribute may be applied to any [item](../items.md), [trait item](../items/traits.md), [enum variant](../items/enumerations.md), [struct field](../items/structs.md), [external block item](../items/external-blocks.md), or [macro definition](../macros-by-example.md). It cannot be applied to [trait implementation items](../items/implementations.md#trait-implementations). When applied to an item containing other items, such as a [module](../items/modules.md) or [implementation](../items/implementations.md), all child items inherit the deprecation attribute.

Here is an example:

``` playground
#![allow(unused)]
fn main() {
#[deprecated(since = "5.2.0", note = "foo was rarely used. Users should instead use bar")]
pub fn foo() {}

pub fn bar() {}
}
```

The [RFC](https://github.com/rust-lang/rfcs/blob/master/text/1270-deprecation.md) contains motivations and more details.

[attributes.diagnostics.must_use](#r-attributes.diagnostics.must_use)

## The `must_use` attribute

[attributes.diagnostics.must_use.intro](#r-attributes.diagnostics.must_use.intro)

The *`must_use` [attribute](../attributes.md)* marks a value that should be used.

[attributes.diagnostics.must_use.syntax](#r-attributes.diagnostics.must_use.syntax)

The `must_use` attribute uses the [MetaWord](../attributes.md#grammar-MetaWord) and [MetaNameValueStr](../attributes.md#grammar-MetaNameValueStr) syntaxes.

> ![](../../assets/inline/diagnostics-5.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> #[must_use]
> fn use_me1() -> u8 { 0 }
>
> #[must_use = "explanation of why it should be used"]
> fn use_me2() -> u8 { 0 }
> }
> ```

[attributes.diagnostics.must_use.allowed-positions](#r-attributes.diagnostics.must_use.allowed-positions)

The `must_use` attribute may be applied to a:

- [Struct](../items/structs.md)
- [Enumeration](../items/enumerations.md)
- [Union](../items/unions.md)
- [Function](../items/functions.md#grammar-Function)
- [Trait](../items/traits.md#grammar-Trait)

> ![](../../assets/inline/diagnostics-6.svg)Note
>
> `rustc` ignores use in other positions but lints against it. This may become an error in the future.

[attributes.diagnostics.must_use.duplicates](#r-attributes.diagnostics.must_use.duplicates)

The `must_use` attribute may be used only once on an item.

> ![](../../assets/inline/diagnostics-7.svg)Note
>
> `rustc` lints against any use following the first. This may become an error in the future.

[attributes.diagnostics.must_use.message](#r-attributes.diagnostics.must_use.message)

The `must_use` attribute may include a message by using the [MetaNameValueStr](../attributes.md#grammar-MetaNameValueStr) syntax, e.g., `#[must_use = "example message"]`. The message may be emitted as part of the lint.

[attributes.diagnostics.must_use.type](#r-attributes.diagnostics.must_use.type)

When the attribute is applied to a [struct](../items/structs.md), [enumeration](../items/enumerations.md), or [union](../items/unions.md), if the [expression](../expressions.md) of an [expression statement](../statements.md#expression-statements) has that type, the use triggers the `unused_must_use` lint.

``` playground
#![allow(unused)]
#![deny(unused_must_use)]
fn main() {
#[must_use]
struct MustUse();
MustUse(); // ERROR: Unused value that must be used.
}
```

[attributes.diagnostics.must_use.type-uninhabited](#r-attributes.diagnostics.must_use.type-uninhabited)

As an exception to [attributes.diagnostics.must_use.type](#r-attributes.diagnostics.must_use.type), the lint does not fire for `Result<(), E>` when `E` is [uninhabited](../glossary.md#r-glossary.uninhabited) or for `ControlFlow<B, ()>` when `B` is [uninhabited](../glossary.md#r-glossary.uninhabited). A `#[non_exhaustive]` type from an external crate is not considered uninhabited for this purpose, because it may gain constructors in the future.

``` playground
#![allow(unused)]
#![deny(unused_must_use)]
fn main() {
use core::ops::ControlFlow;
fn f1() -> Result<(), !> { Ok(()) }
f1(); // OK: `!` is uninhabited.
fn f2() -> ControlFlow<!, ()> { ControlFlow::Continue(()) }
f2(); // OK: `!` is uninhabited.
}
```

[attributes.diagnostics.must_use.fn](#r-attributes.diagnostics.must_use.fn)

If the [expression](../expressions.md) of an [expression statement](../statements.md#expression-statements) is a [call expression](../expressions/call-expr.md) or [method call expression](../expressions/method-call-expr.md) whose function operand is a function to which the attribute is applied, the use triggers the `unused_must_use` lint.

``` playground
#![allow(unused)]
#![deny(unused_must_use)]
fn main() {
#[must_use]
fn f() {}
f(); // ERROR: Unused return value that must be used.
}
```

[attributes.diagnostics.must_use.trait](#r-attributes.diagnostics.must_use.trait)

If the [expression](../expressions.md) of an [expression statement](../statements.md#expression-statements) is a [call expression](../expressions/call-expr.md) or [method call expression](../expressions/method-call-expr.md) whose function operand is a function that returns an [impl trait](../types/impl-trait.md) or a [dyn trait](../types/trait-object.md) type where one or more traits in the bound are marked with the attribute, the use triggers the `unused_must_use` lint.

``` playground
#![allow(unused)]
#![deny(unused_must_use)]
fn main() {
#[must_use]
trait Tr {}
impl Tr for () {}
fn f() -> impl Tr {}
f(); // ERROR: Unused implementor that must be used.
}
```

[attributes.diagnostics.must_use.trait-function](#r-attributes.diagnostics.must_use.trait-function)

When the attribute is applied to a function in a trait declaration, the rules described in [attributes.diagnostics.must_use.fn](#r-attributes.diagnostics.must_use.fn) also apply when the function operand of the [call expression](../expressions/call-expr.md) or [method call expression](../expressions/method-call-expr.md) is an implementation of that function.

``` playground
#![allow(unused)]
#![deny(unused_must_use)]
fn main() {
trait Tr {
    #[must_use]
    fn use_me(&self);
}

impl Tr for () {
    fn use_me(&self) {}
}

().use_me(); // ERROR: Unused return value that must be used.
}
```

``` playground
#![allow(unused)]
fn main() {
#![deny(unused_must_use)]
trait Tr {
    #[must_use]
    fn use_me(&self);
}

impl Tr for () {
    fn use_me(&self) {}
}

<() as Tr>::use_me(&());
//          ^^^^^^^^^^^ ERROR: Unused return value that must be used.
}
```

[attributes.diagnostics.must_use.block-expr](#r-attributes.diagnostics.must_use.block-expr)

When checking the [expression](../expressions.md) of an [expression statement](../statements.md#expression-statements) for [attributes.diagnostics.must_use.type](#r-attributes.diagnostics.must_use.type), [attributes.diagnostics.must_use.fn](#r-attributes.diagnostics.must_use.fn), [attributes.diagnostics.must_use.trait](#r-attributes.diagnostics.must_use.trait), and [attributes.diagnostics.must_use.trait-function](#r-attributes.diagnostics.must_use.trait-function), the lint looks through [block expressions](../expressions/block-expr.md) (including [`unsafe` blocks](../expressions/block-expr.md#unsafe-blocks) and [labeled block expressions](../expressions/block-expr.md#labeled-block-expressions)) to the trailing expression of each. This applies recursively for nested block expressions.

``` playground
#![allow(unused)]
#![deny(unused_must_use)]
fn main() {
#[must_use]
fn f() {}

{ f() };        // ERROR: The lint looks through block expressions.
unsafe { f() }; // ERROR: The lint looks through `unsafe` blocks.
{ { f() } };    // ERROR: The lint looks through nested blocks.
}
```

[attributes.diagnostics.must_use.trait-impl-function](#r-attributes.diagnostics.must_use.trait-impl-function)

When used on a function in a trait implementation, the attribute does nothing.

``` playground
#![allow(unused)]
#![deny(unused_must_use)]
fn main() {
trait Tr {
    fn f(&self);
}

impl Tr for () {
    #[must_use] // This has no effect.
    fn f(&self) {}
}

().f(); // OK.
}
```

> ![](../../assets/inline/diagnostics-8.svg)Note
>
> `rustc` lints against use on functions in trait implementations. This may become an error in the future.

[attributes.diagnostics.must_use.wrapping-suppression](#r-attributes.diagnostics.must_use.wrapping-suppression)

> ![](../../assets/inline/diagnostics-9.svg)Note
>
> Wrapping the result of a `#[must_use]` function in certain expressions can suppress the [fn-based check](#r-attributes.diagnostics.must_use.fn), because the [expression](../expressions.md) of the [expression statement](../statements.md#expression-statements) is not a [call expression](../expressions/call-expr.md) or [method call expression](../expressions/method-call-expr.md) to a `#[must_use]` function. The [type-based check](#r-attributes.diagnostics.must_use.type) still applies if the type of the overall expression is `#[must_use]`.
>
> ``` playground
> #![allow(unused)]
> #![deny(unused_must_use)]
> fn main() {
> #[must_use]
> fn f() {}
>
> // The fn-based check does not fire for any of these, because the
> // expression of the expression statement is not a call to a
> // `#[must_use]` function.
> (f(),);                    // Expression is a tuple, not a call.
> Some(f());                 // Callee `Some` is not `#[must_use]`.
> if true { f() } else {};   // Expression is an `if`, not a call.
> match true {               // Expression is a `match`, not a call.
>     _ => f()
> };
> }
> ```
>
> ``` playground
> #![allow(unused)]
> #![deny(unused_must_use)]
> fn main() {
> #[must_use]
> struct MustUse;
> fn g() -> MustUse { MustUse }
>
> // Despite the `if` expression not being a call, the type-based check
> // fires because the type of the expression is `MustUse`, which has
> // the `#[must_use]` attribute.
> if true { g() } else { MustUse }; // ERROR: Must be used.
> }
> ```

[attributes.diagnostics.must_use.underscore-idiom](#r-attributes.diagnostics.must_use.underscore-idiom)

> ![](../../assets/inline/diagnostics-10.svg)Note
>
> Using a [let statement](../statements.md#let-statements) or [destructuring assignment](../expressions/operator-expr.md#r-expr.assign.destructure) with a pattern of `_` when a must-used value is purposely discarded is idiomatic.
>
> ``` playground
> #![allow(unused)]
> #![deny(unused_must_use)]
> fn main() {
> #[must_use]
> fn f() {}
> let _ = f(); // OK.
> _ = f(); // OK.
> }
> ```

[attributes.diagnostic.namespace](#r-attributes.diagnostic.namespace)

## The `diagnostic` tool attribute namespace

[attributes.diagnostic.namespace.intro](#r-attributes.diagnostic.namespace.intro)

The `#[diagnostic]` attribute namespace is a home for attributes to influence compile-time error messages. The hints provided by these attributes are not guaranteed to be used.

[attributes.diagnostic.namespace.unknown-invalid-syntax](#r-attributes.diagnostic.namespace.unknown-invalid-syntax)\
Tests Tests with this rule:

- [tests/ui/diagnostic_namespace/deny_malformed_attribute.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/diagnostic_namespace/deny_malformed_attribute.rs)
- [tests/ui/diagnostic_namespace/non_existing_attributes_accepted.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/diagnostic_namespace/non_existing_attributes_accepted.rs)
- [tests/ui/diagnostic_namespace/suggest_typos.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/diagnostic_namespace/suggest_typos.rs)

Unknown attributes in this namespace are accepted, though they may emit warnings for unused attributes. Additionally, invalid inputs to known attributes will typically be a warning (see the attribute definitions for details). This is meant to allow adding or discarding attributes and changing inputs in the future to allow changes without the need to keep the non-meaningful attributes or options working.

[attributes.diagnostic.on_unimplemented](#r-attributes.diagnostic.on_unimplemented)

### The `diagnostic::on_unimplemented` attribute

[attributes.diagnostic.on_unimplemented.intro](#r-attributes.diagnostic.on_unimplemented.intro)\
Tests Tests with this rule:

- [tests/ui/diagnostic_namespace/on_unimplemented/custom-on-unimplemented-diagnostic.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/diagnostic_namespace/on_unimplemented/custom-on-unimplemented-diagnostic.rs)
- [tests/ui/diagnostic_namespace/on_unimplemented/error_is_shown_in_downstream_crates.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/diagnostic_namespace/on_unimplemented/error_is_shown_in_downstream_crates.rs)
- [tests/ui/diagnostic_namespace/on_unimplemented/on_unimplemented_simple.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/diagnostic_namespace/on_unimplemented/on_unimplemented_simple.rs)
- [tests/ui/traits/negative-bounds/on-unimplemented.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/traits/negative-bounds/on-unimplemented.rs)

The `#[diagnostic::on_unimplemented]` attribute is a hint to the compiler to supplement the error message that would normally be generated in scenarios where a trait is required but not implemented on a type.

[attributes.diagnostic.on_unimplemented.allowed-positions](#r-attributes.diagnostic.on_unimplemented.allowed-positions)

The attribute should be placed on a [trait declaration](../items/traits.md), though it is not an error to be located in other positions.

[attributes.diagnostic.on_unimplemented.syntax](#r-attributes.diagnostic.on_unimplemented.syntax)\
Tests Tests with this rule:

- [tests/ui/diagnostic_namespace/malformed_foreign_on_unimplemented.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/diagnostic_namespace/malformed_foreign_on_unimplemented.rs)
- [tests/ui/diagnostic_namespace/on_unimplemented/do_not_accept_options_of_the_internal_rustc_attribute.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/diagnostic_namespace/on_unimplemented/do_not_accept_options_of_the_internal_rustc_attribute.rs)
- [tests/ui/diagnostic_namespace/on_unimplemented/do_not_fail_parsing_on_invalid_options_1.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/diagnostic_namespace/on_unimplemented/do_not_fail_parsing_on_invalid_options_1.rs)

The attribute uses the [MetaListNameValueStr](../attributes.md#grammar-MetaListNameValueStr) syntax to specify its inputs, though any malformed input to the attribute is not considered as an error to provide both forwards and backwards compatibility.

[attributes.diagnostic.on_unimplemented.keys](#r-attributes.diagnostic.on_unimplemented.keys)\
Tests Tests with this rule:

- [tests/ui/diagnostic_namespace/on_unimplemented/custom-on-unimplemented-diagnostic.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/diagnostic_namespace/on_unimplemented/custom-on-unimplemented-diagnostic.rs)
- [tests/ui/diagnostic_namespace/on_unimplemented/do_not_accept_options_of_the_internal_rustc_attribute.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/diagnostic_namespace/on_unimplemented/do_not_accept_options_of_the_internal_rustc_attribute.rs)
- [tests/ui/diagnostic_namespace/on_unimplemented/on_unimplemented_simple.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/diagnostic_namespace/on_unimplemented/on_unimplemented_simple.rs)

The following keys have the given meaning:

- `message` — The text for the top level error message.
- `label` — The text for the label shown inline in the broken code in the error message.
- `note` — Provides additional notes.

[attributes.diagnostic.on_unimplemented.note-repetition](#r-attributes.diagnostic.on_unimplemented.note-repetition)\
Tests Tests with this rule:

- [tests/ui/diagnostic_namespace/on_unimplemented/multiple_notes.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/diagnostic_namespace/on_unimplemented/multiple_notes.rs)

The `note` option can appear several times, which results in several note messages being emitted.

[attributes.diagnostic.on_unimplemented.repetition](#r-attributes.diagnostic.on_unimplemented.repetition)\
Tests Tests with this rule:

- [tests/ui/diagnostic_namespace/on_unimplemented/report_warning_on_duplicated_options.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/diagnostic_namespace/on_unimplemented/report_warning_on_duplicated_options.rs)

If any of the other options appears several times the first occurrence of the relevant option specifies the actually used value. Subsequent occurrences generates a warning.

[attributes.diagnostic.on_unimplemented.unknown-keys](#r-attributes.diagnostic.on_unimplemented.unknown-keys)\
Tests Tests with this rule:

- [tests/ui/diagnostic_namespace/on_unimplemented/do_not_fail_parsing_on_invalid_options_1.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/diagnostic_namespace/on_unimplemented/do_not_fail_parsing_on_invalid_options_1.rs)

A warning is generated for any unknown keys.

[attributes.diagnostic.on_unimplemented.format-string](#r-attributes.diagnostic.on_unimplemented.format-string)

All three options accept a string as an argument, interpreted using the same formatting as a [`std::fmt`](../../alloc/fmt/index.html) string.

[attributes.diagnostic.on_unimplemented.format-parameters](#r-attributes.diagnostic.on_unimplemented.format-parameters)\
Tests Tests with this rule:

- [tests/ui/diagnostic_namespace/on_unimplemented/do_not_accept_options_of_the_internal_rustc_attribute.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/diagnostic_namespace/on_unimplemented/do_not_accept_options_of_the_internal_rustc_attribute.rs)

Format parameters with the given named parameter will be replaced with the following text:

- `{Self}` — The name of the type implementing the trait.
- `{` *GenericParameterName* `}` — The name of the generic argument’s type for the given generic parameter.

[attributes.diagnostic.on_unimplemented.invalid-formats](#r-attributes.diagnostic.on_unimplemented.invalid-formats)\
Tests Tests with this rule:

- [tests/ui/diagnostic_namespace/on_unimplemented/do_not_accept_options_of_the_internal_rustc_attribute.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/diagnostic_namespace/on_unimplemented/do_not_accept_options_of_the_internal_rustc_attribute.rs)

Any other format parameter will generate a warning, but will otherwise be included in the string as-is.

[attributes.diagnostic.on_unimplemented.invalid-string](#r-attributes.diagnostic.on_unimplemented.invalid-string)\
Tests Tests with this rule:

- [tests/ui/diagnostic_namespace/on_unimplemented/broken_format.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/diagnostic_namespace/on_unimplemented/broken_format.rs)

Invalid format strings may generate a warning, but are otherwise allowed, but may not display as intended. Format specifiers may generate a warning, but are otherwise ignored.

In this example:

``` playground
#[diagnostic::on_unimplemented(
    message = "My Message for `ImportantTrait<{A}>` implemented for `{Self}`",
    label = "My Label",
    note = "Note 1",
    note = "Note 2"
)]
trait ImportantTrait<A> {}

fn use_my_trait(_: impl ImportantTrait<i32>) {}

fn main() {
    use_my_trait(String::new());
}
```

the compiler may generate an error message which looks like this:

``` text
error[E0277]: My Message for `ImportantTrait<i32>` implemented for `String`
  --> src/main.rs:14:18
   |
14 |     use_my_trait(String::new());
   |     ------------ ^^^^^^^^^^^^^ My Label
   |     |
   |     required by a bound introduced by this call
   |
   = help: the trait `ImportantTrait<i32>` is not implemented for `String`
   = note: Note 1
   = note: Note 2
```

[attributes.diagnostic.do_not_recommend](#r-attributes.diagnostic.do_not_recommend)

### The `diagnostic::do_not_recommend` attribute

[attributes.diagnostic.do_not_recommend.intro](#r-attributes.diagnostic.do_not_recommend.intro)\
Tests Tests with this rule:

- [tests/ui/diagnostic_namespace/do_not_recommend/as_expression.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/diagnostic_namespace/do_not_recommend/as_expression.rs)
- [tests/ui/diagnostic_namespace/do_not_recommend/nested.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/diagnostic_namespace/do_not_recommend/nested.rs)
- [tests/ui/diagnostic_namespace/do_not_recommend/simple.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/diagnostic_namespace/do_not_recommend/simple.rs)
- [tests/ui/diagnostic_namespace/do_not_recommend/single-impl-candidate.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/diagnostic_namespace/do_not_recommend/single-impl-candidate.rs)
- [tests/ui/diagnostic_namespace/do_not_recommend/stacked.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/diagnostic_namespace/do_not_recommend/stacked.rs)
- [tests/ui/diagnostic_namespace/do_not_recommend/supress_suggestions_in_help.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/diagnostic_namespace/do_not_recommend/supress_suggestions_in_help.rs)
- [tests/ui/diagnostic_namespace/do_not_recommend/type_mismatch.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/diagnostic_namespace/do_not_recommend/type_mismatch.rs)
- [tests/ui/diagnostic_namespace/do_not_recommend/with_lifetime.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/diagnostic_namespace/do_not_recommend/with_lifetime.rs)

The `#[diagnostic::do_not_recommend]` attribute is a hint to the compiler to not show the annotated trait implementation as part of a diagnostic message.

> ![](../../assets/inline/diagnostics-11.svg)Note
>
> Suppressing the recommendation can be useful if you know that the recommendation would normally not be useful to the programmer. This often occurs with broad, blanket impls. The recommendation may send the programmer down the wrong path, or the trait implementation may be an internal detail that you don’t want to expose, or the bounds may not be able to be satisfied by the programmer.
>
> For example, in an error message about a type not implementing a required trait, the compiler may find a trait implementation that would satisfy the requirements if it weren’t for specific bounds in the trait implementation. The compiler may tell the user that there is an impl, but the problem is the bounds in the trait implementation. The `#[diagnostic::do_not_recommend]` attribute can be used to tell the compiler to *not* tell the user about the trait implementation, and instead simply tell the user the type doesn’t implement the required trait.

[attributes.diagnostic.do_not_recommend.allowed-positions](#r-attributes.diagnostic.do_not_recommend.allowed-positions)\
Tests Tests with this rule:

- [tests/ui/diagnostic_namespace/do_not_recommend/incorrect-locations.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/diagnostic_namespace/do_not_recommend/incorrect-locations.rs)

The attribute should be placed on a [trait implementation item](../items/implementations.md#trait-implementations), though it is not an error to be located in other positions.

[attributes.diagnostic.do_not_recommend.syntax](#r-attributes.diagnostic.do_not_recommend.syntax)\
Tests Tests with this rule:

- [tests/ui/diagnostic_namespace/do_not_recommend/does_not_acccept_args.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/diagnostic_namespace/do_not_recommend/does_not_acccept_args.rs)
- [tests/ui/diagnostic_namespace/do_not_recommend/malformed-diagnostic-attributes-if-expression.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/diagnostic_namespace/do_not_recommend/malformed-diagnostic-attributes-if-expression.rs)

The attribute does not accept any arguments, though unexpected arguments are not considered as an error.

In the following example, there is a trait called `AsExpression` which is used for casting arbitrary types to the `Expression` type used in an SQL library. There is a method called `check` which takes an `AsExpression`.

``` playground
pub trait Expression {
    type SqlType;
}

pub trait AsExpression<ST> {
    type Expression: Expression<SqlType = ST>;
}

pub struct Text;
pub struct Integer;

pub struct Bound<T>(T);
pub struct SelectInt;

impl Expression for SelectInt {
    type SqlType = Integer;
}

impl<T> Expression for Bound<T> {
    type SqlType = T;
}

impl AsExpression<Integer> for i32 {
    type Expression = Bound<Integer>;
}

impl AsExpression<Text> for &'_ str {
    type Expression = Bound<Text>;
}

impl<T> Foo for T where T: Expression {}

// Uncomment this line to change the recommendation.
// #[diagnostic::do_not_recommend]
impl<T, ST> AsExpression<ST> for T
where
    T: Expression<SqlType = ST>,
{
    type Expression = T;
}

trait Foo: Expression + Sized {
    fn check<T>(&self, _: T) -> <T as AsExpression<<Self as Expression>::SqlType>>::Expression
    where
        T: AsExpression<Self::SqlType>,
    {
        todo!()
    }
}

fn main() {
    SelectInt.check("bar");
}
```

The `SelectInt` type’s `check` method is expecting an `Integer` type. Calling it with an i32 type works, as it gets converted to an `Integer` by the `AsExpression` trait. However, calling it with a string does not, and generates a an error that may look like this:

``` text
error[E0277]: the trait bound `&str: Expression` is not satisfied
  --> src/main.rs:53:15
   |
53 |     SelectInt.check("bar");
   |               ^^^^^ the trait `Expression` is not implemented for `&str`
   |
   = help: the following other types implement trait `Expression`:
             Bound<T>
             SelectInt
note: required for `&str` to implement `AsExpression<Integer>`
  --> src/main.rs:45:13
   |
45 | impl<T, ST> AsExpression<ST> for T
   |             ^^^^^^^^^^^^^^^^     ^
46 | where
47 |     T: Expression<SqlType = ST>,
   |        ------------------------ unsatisfied trait bound introduced here
```

By adding the `#[diagnostic::do_not_recommend]` attribute to the blanket `impl` for `AsExpression`, the message changes to:

``` text
error[E0277]: the trait bound `&str: AsExpression<Integer>` is not satisfied
  --> src/main.rs:53:15
   |
53 |     SelectInt.check("bar");
   |               ^^^^^ the trait `AsExpression<Integer>` is not implemented for `&str`
   |
   = help: the trait `AsExpression<Integer>` is not implemented for `&str`
           but trait `AsExpression<Text>` is implemented for it
   = help: for that trait implementation, expected `Text`, found `Integer`
```

The first error message includes a somewhat confusing error message about the relationship of `&str` and `Expression`, as well as the unsatisfied trait bound in the blanket impl. After adding `#[diagnostic::do_not_recommend]`, it no longer considers the blanket impl for the recommendation. The message should be a little clearer, with an indication that a string cannot be converted to an `Integer`.
