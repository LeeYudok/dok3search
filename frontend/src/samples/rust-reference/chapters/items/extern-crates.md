# Extern crate declarations

[items.extern-crate](#r-items.extern-crate)

# Extern crate declarations

[items.extern-crate.syntax](#r-items.extern-crate.syntax)

**Syntax**\
[ExternCrate](#railroad-ExternCrate) → extern crate [CrateRef](#grammar-CrateRef) [AsClause](#grammar-AsClause)? ;

[CrateRef](#railroad-CrateRef) → [IDENTIFIER](../identifiers.md#grammar-IDENTIFIER) \| self

[AsClause](#railroad-AsClause) → as ( [IDENTIFIER](../identifiers.md#grammar-IDENTIFIER) \| \_ )

Show Railroad

![](../../assets/inline/extern-crates-1.svg) ![](../../assets/inline/extern-crates-2.svg) ![](../../assets/inline/extern-crates-3.svg) [items.extern-crate.intro](#r-items.extern-crate.intro)

An *`extern crate` declaration* specifies a dependency on an external crate.

[items.extern-crate.namespace](#r-items.extern-crate.namespace)

The external crate is then bound into the declaring scope as the given [identifier](../identifiers.md) in the [type namespace](../names/namespaces.md).

[items.extern-crate.extern-prelude](#r-items.extern-crate.extern-prelude)

Additionally, if the `extern crate` appears in the crate root, then the crate name is also added to the [extern prelude](../names/preludes.md#extern-prelude), making it automatically in scope in all modules.

[items.extern-crate.as](#r-items.extern-crate.as)\
Tests Tests with this rule:

- [tests/ui/extern/extern-crate-rename.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/extern/extern-crate-rename.rs)

The `as` clause can be used to bind the imported crate to a different name.

[items.extern-crate.lookup](#r-items.extern-crate.lookup)

The external crate is resolved to a specific `soname` at compile time, and a runtime linkage requirement to that `soname` is passed to the linker for loading at runtime. The `soname` is resolved at compile time by scanning the compiler’s library path and matching the optional `crate_name` provided against the [`crate_name` attributes](../crates-and-source-files.md#the-crate_name-attribute) that were declared on the external crate when it was compiled. If no `crate_name` is provided, a default `name` attribute is assumed, equal to the [identifier](../identifiers.md) given in the `extern crate` declaration.

[items.extern-crate.self](#r-items.extern-crate.self)

The `self` crate may be imported which creates a binding to the current crate. In this case the `as` clause must be used to specify the name to bind it to.

Three examples of `extern crate` declarations:

``` rust
extern crate pcre;

extern crate std; // equivalent to: extern crate std as std;

extern crate std as ruststd; // linking to 'std' under another name
```

[items.extern-crate.name-restrictions](#r-items.extern-crate.name-restrictions)

When naming Rust crates, hyphens are disallowed. However, Cargo packages may make use of them. In such case, when `Cargo.toml` doesn’t specify a crate name, Cargo will transparently replace `-` with `_` (Refer to [RFC 940](https://github.com/rust-lang/rfcs/blob/master/text/0940-hyphens-considered-harmful.md) for more details).

Here is an example:

``` rust
// Importing the Cargo package hello-world
extern crate hello_world; // hyphen replaced with an underscore
```

[items.extern-crate.underscore](#r-items.extern-crate.underscore)

## Underscore imports

[items.extern-crate.underscore.intro](#r-items.extern-crate.underscore.intro)

An external crate dependency can be declared without binding its name in scope by using an underscore with the form `extern crate foo as _`. This may be useful for crates that only need to be linked, but are never referenced, and will avoid being reported as unused.

[items.extern-crate.underscore.macro_use](#r-items.extern-crate.underscore.macro_use)

The [`macro_use` attribute](../macros-by-example.md#the-macro_use-attribute) works as usual and imports the macro names into the [`macro_use` prelude](../names/preludes.md#macro_use-prelude).

[items.extern-crate.no_link](#r-items.extern-crate.no_link)

## The `no_link` attribute

[items.extern-crate.no_link.intro](#r-items.extern-crate.no_link.intro)

The *`no_link` [attribute](../attributes.md#r-attributes)* may be applied to an `extern crate` item to prevent linking the crate.

> ![](../../assets/inline/extern-crates-4.svg)Note
>
> This is helpful, e.g., when only the macros of a crate are needed.

> ![](../../assets/inline/extern-crates-5.svg)Example
>
> ``` rust
> #[no_link]
> extern crate other_crate;
>
> other_crate::some_macro!();
> ```

[items.extern-crate.no_link.syntax](#r-items.extern-crate.no_link.syntax)

The `no_link` attribute uses the [MetaWord](../attributes.md#grammar-MetaWord) syntax.

[items.extern-crate.no_link.allowed-positions](#r-items.extern-crate.no_link.allowed-positions)

The `no_link` attribute may only be applied to an `extern crate` declaration.

> ![](../../assets/inline/extern-crates-6.svg)Note
>
> `rustc` ignores use in other positions but lints against it. This may become an error in the future.

[items.extern-crate.no_link.duplicates](#r-items.extern-crate.no_link.duplicates)

Only the first use of `no_link` on an `extern crate` declaration has effect.

> ![](../../assets/inline/extern-crates-7.svg)Note
>
> `rustc` lints against any use following the first. This may become an error in the future.
