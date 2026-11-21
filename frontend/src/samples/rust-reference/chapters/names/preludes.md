# Preludes

[names.preludes](#r-names.preludes)

# Preludes

[names.preludes.intro](#r-names.preludes.intro)

A *prelude* is a collection of names that are automatically brought into scope of every module in a crate.

These prelude names are not part of the module itself: they are implicitly queried during [name resolution](name-resolution.md). For example, even though something like [`Box`](https://doc.rust-lang.org/1.100.0/alloc/boxed/struct.Box.html) is in scope in every module, you cannot refer to it as `self::Box` because it is not a member of the current module.

[names.preludes.kinds](#r-names.preludes.kinds)

There are several different preludes:

- [Standard library prelude](#r-names.preludes.std)
- [Extern prelude](#extern-prelude)
- [Language prelude](#language-prelude)
- [`macro_use` prelude](#macro_use-prelude)
- [Tool prelude](#tool-prelude)

[names.preludes.std](#r-names.preludes.std)

## Standard library prelude

[names.preludes.std.intro](#r-names.preludes.std.intro)

Each crate has a standard library prelude, which consists of the names from a single standard library module.

[names.preludes.std.module](#r-names.preludes.std.module)

The module used depends on the crate’s edition, and on whether the [`no_std` attribute](#the-no_std-attribute) is applied to the crate:

| Edition | `no_std` not applied | `no_std` applied |
|----|----|----|
| 2015 | [`std::prelude::rust_2015`](https://doc.rust-lang.org/1.100.0/std/prelude/rust_2015/index.html) | [`core::prelude::rust_2015`](https://doc.rust-lang.org/1.100.0/core/prelude/rust_2015/index.html) |
| 2018 | [`std::prelude::rust_2018`](https://doc.rust-lang.org/1.100.0/std/prelude/rust_2018/index.html) | [`core::prelude::rust_2018`](https://doc.rust-lang.org/1.100.0/core/prelude/rust_2018/index.html) |
| 2021 | [`std::prelude::rust_2021`](https://doc.rust-lang.org/1.100.0/std/prelude/rust_2021/index.html) | [`core::prelude::rust_2021`](https://doc.rust-lang.org/1.100.0/core/prelude/rust_2021/index.html) |
| 2024 | [`std::prelude::rust_2024`](https://doc.rust-lang.org/1.100.0/std/prelude/rust_2024/index.html) | [`core::prelude::rust_2024`](https://doc.rust-lang.org/1.100.0/core/prelude/rust_2024/index.html) |

> ![](../../assets/inline/preludes-1.svg)Note
>
> [`std::prelude::rust_2015`](https://doc.rust-lang.org/1.100.0/std/prelude/rust_2015/index.html) and [`std::prelude::rust_2018`](https://doc.rust-lang.org/1.100.0/std/prelude/rust_2018/index.html) have the same contents as [`std::prelude::v1`](https://doc.rust-lang.org/1.100.0/std/prelude/v1/index.html).
>
> [`core::prelude::rust_2015`](https://doc.rust-lang.org/1.100.0/core/prelude/rust_2015/index.html) and [`core::prelude::rust_2018`](https://doc.rust-lang.org/1.100.0/core/prelude/rust_2018/index.html) have the same contents as [`core::prelude::v1`](https://doc.rust-lang.org/1.100.0/core/prelude/v1/index.html).

> ![](../../assets/inline/preludes-2.svg)Note
>
> When one of [`core::panic!`](https://doc.rust-lang.org/1.100.0/core/macro.panic.html) or [`std::panic!`](https://doc.rust-lang.org/1.100.0/std/macro.panic.html) is brought into scope due to the [standard library prelude](#r-names.preludes.std), and a user-written [glob import](../items/use-declarations.md#r-items.use.glob) brings the other into scope, `rustc` currently allows use of `panic!`, even though it is ambiguous. The user-written glob import takes precedence to resolve this ambiguity.
>
> For details, see [names.resolution.expansion.imports.ambiguity.panic-hack](name-resolution.md#r-names.resolution.expansion.imports.ambiguity.panic-hack).

[names.preludes.extern](#r-names.preludes.extern)

## Extern prelude

[names.preludes.extern.intro](#r-names.preludes.extern.intro)

External crates imported with [`extern crate`](../items/extern-crates.md) in the root module or provided to the compiler (as with the `--extern` flag with `rustc`) are added to the *extern prelude*. If imported with an alias such as `extern crate orig_name as new_name`, then the symbol `new_name` is instead added to the prelude.

[names.preludes.extern.core](#r-names.preludes.extern.core)

The [`core`](https://doc.rust-lang.org/1.100.0/core/index.html) crate is always added to the extern prelude.

[names.preludes.extern.std](#r-names.preludes.extern.std)

The [`std`](https://doc.rust-lang.org/1.100.0/std/index.html) crate is added as long as the [`no_std` attribute](#the-no_std-attribute) is not specified in the crate root.

[names.preludes.extern.edition2018](#r-names.preludes.extern.edition2018)

> 2018 Edition differences
>
> In the 2015 edition, crates in the extern prelude cannot be referenced via [use declarations](../items/use-declarations.md), so it is generally standard practice to include `extern crate` declarations to bring them into scope.
>
> Beginning in the 2018 edition, [use declarations](../items/use-declarations.md) can reference crates in the extern prelude, so it is considered unidiomatic to use `extern crate`.

> ![](../../assets/inline/preludes-3.svg)Note
>
> Additional crates that ship with `rustc`, such as [`alloc`](https://doc.rust-lang.org/1.100.0/alloc/index.html), and [`test`](https://doc.rust-lang.org/1.100.0/test/index.html), are not automatically included with the `--extern` flag when using Cargo. They must be brought into scope with an `extern crate` declaration, even in the 2018 edition.
>
> ``` playground
> #![allow(unused)]
> fn main() {
> extern crate alloc;
> use alloc::rc::Rc;
> }
> ```
>
> Cargo does bring in `proc_macro` to the extern prelude for proc-macro crates only.

[names.preludes.extern.no_std](#r-names.preludes.extern.no_std)

### The `no_std` attribute

[names.preludes.extern.no_std.intro](#r-names.preludes.extern.no_std.intro)

The *`no_std` [attribute](../attributes.md#r-attributes)* causes the [`std`](https://doc.rust-lang.org/1.100.0/std/index.html) crate to not be linked automatically and the [standard library prelude](#r-names.preludes.std) to instead use the `core` prelude.

> ![](../../assets/inline/preludes-4.svg)Example
>
> ``` rust
> #![no_std]
> ```

> ![](../../assets/inline/preludes-5.svg)Note
>
> Using `no_std` is useful when either the crate is targeting a platform that does not support the standard library or is purposefully not using the capabilities of the standard library. Those capabilities are mainly dynamic memory allocation (e.g. `Box` and `Vec`) and file and network capabilities (e.g. `std::fs` and `std::io`).

> ![](../../assets/inline/preludes-6.svg)Warning
>
> Using `no_std` does not prevent the standard library from being linked. It is still valid to write `extern crate std` in the crate or in one of its dependencies; this will cause the compiler to link the `std` crate into the program.

[names.preludes.extern.no_std.syntax](#r-names.preludes.extern.no_std.syntax)

The `no_std` attribute uses the [MetaWord](../attributes.md#grammar-MetaWord) syntax.

[names.preludes.extern.no_std.allowed-positions](#r-names.preludes.extern.no_std.allowed-positions)

The `no_std` attribute may only be applied to the crate root.

[names.preludes.extern.no_std.duplicates](#r-names.preludes.extern.no_std.duplicates)

The `no_std` attribute may be used any number of times on a form.

> ![](../../assets/inline/preludes-7.svg)Note
>
> `rustc` lints against any use following the first.

[names.preludes.extern.no_std.module](#r-names.preludes.extern.no_std.module)

The `no_std` attribute changes the [standard library prelude](#r-names.preludes.std) to use the `core` prelude instead of the `std` prelude.

[names.preludes.extern.no_std.edition2018](#r-names.preludes.extern.no_std.edition2018)

> 2018 Edition differences
>
> Before the 2018 edition, `std` is injected into the crate root by default. If `no_std` is specified, `core` is injected instead. Starting with the 2018 edition, regardless of `no_std` being specified, neither is injected into the crate root.

[names.preludes.lang](#r-names.preludes.lang)

## Language prelude

[names.preludes.lang.intro](#r-names.preludes.lang.intro)

The language prelude includes names of types and attributes that are built-in to the language. The language prelude is always in scope.

[names.preludes.lang.entities](#r-names.preludes.lang.entities)

It includes the following:

- [Type namespace](namespaces.md)
  - [Boolean type](../types/boolean.md) — `bool`
  - [`char`](../types/char.md)
  - [`str`](../types/str.md)
  - [Integer types](../types/numeric.md#integer-types) — `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`
  - [Machine-dependent integer types](../types/numeric.md#machine-dependent-integer-types) — `usize` and `isize`
  - [floating-point types](../types/numeric.md#floating-point-types) — `f32` and `f64`
- [Macro namespace](namespaces.md)
  - [Built-in attributes](../attributes.md#built-in-attributes-index)
  - [Built-in derive macros](../attributes/derive.md#r-attributes.derive.built-in)

[names.preludes.macro_use](#r-names.preludes.macro_use)

## `macro_use` prelude

[names.preludes.macro_use.intro](#r-names.preludes.macro_use.intro)

The `macro_use` prelude includes macros from external crates that were imported by the [`macro_use` attribute](../macros-by-example.md#the-macro_use-attribute) applied to an [`extern crate`](../items/extern-crates.md).

[names.preludes.tool](#r-names.preludes.tool)

## Tool prelude

[names.preludes.tool.intro](#r-names.preludes.tool.intro)

The tool prelude includes tool names for external tools in the [type namespace](namespaces.md). See the [tool attributes](../attributes.md#tool-attributes) section for more details.

[names.preludes.no_implicit_prelude](#r-names.preludes.no_implicit_prelude)

## The `no_implicit_prelude` attribute

[names.preludes.no_implicit_prelude.intro](#r-names.preludes.no_implicit_prelude.intro)

The *`no_implicit_prelude` [attribute](../attributes.md)* is used to prevent implicit preludes from being brought into scope.

> ![](../../assets/inline/preludes-8.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> // The attribute can be applied to the crate root to affect
> // all modules.
> #![no_implicit_prelude]
>
> // Or it can be applied to a module to only affect that module
> // and its descendants.
> #[no_implicit_prelude]
> mod example {
>     // ...
> }
> }
> ```

[names.preludes.no_implicit_prelude.syntax](#r-names.preludes.no_implicit_prelude.syntax)

The `no_implicit_prelude` attribute uses the [MetaWord](../attributes.md#grammar-MetaWord) syntax.

[names.preludes.no_implicit_prelude.allowed-positions](#r-names.preludes.no_implicit_prelude.allowed-positions)

The `no_implicit_prelude` attribute may only be applied to the crate or to a module.

> ![](../../assets/inline/preludes-9.svg)Note
>
> `rustc` ignores use in other positions but lints against it. This may become an error in the future.

[names.preludes.no_implicit_prelude.duplicates](#r-names.preludes.no_implicit_prelude.duplicates)

The `no_implicit_prelude` attribute may be used any number of times on a form.

> ![](../../assets/inline/preludes-10.svg)Note
>
> `rustc` lints against any use following the first.

[names.preludes.no_implicit_prelude.excluded-preludes](#r-names.preludes.no_implicit_prelude.excluded-preludes)

The `no_implicit_prelude` attribute prevents the [standard library prelude](#r-names.preludes.std), [extern prelude](#extern-prelude), [`macro_use` prelude](#macro_use-prelude), and the [tool prelude](#tool-prelude) from being brought into scope for the module and its descendants.

[names.preludes.no_implicit_prelude.implicitly-imported-macros](#r-names.preludes.no_implicit_prelude.implicitly-imported-macros)

> ![](../../assets/inline/preludes-11.svg)Note
>
> Despite `#![no_implicit_prelude]`, `rustc` currently brings certain macros implicitly into scope. Those macros are:
>
> - [`assert!`](https://doc.rust-lang.org/1.100.0/core/macro.assert.html)
> - [`cfg!`](https://doc.rust-lang.org/1.100.0/core/macro.cfg.html)
> - [`cfg_select!`](https://doc.rust-lang.org/1.100.0/core/macros/macro.cfg_select.html)
> - [`column!`](https://doc.rust-lang.org/1.100.0/core/macro.column.html)
> - [`compile_error!`](https://doc.rust-lang.org/1.100.0/core/macro.compile_error.html)
> - [`concat!`](https://doc.rust-lang.org/1.100.0/core/macro.concat.html)
> - [`concat_bytes!`](https://doc.rust-lang.org/1.100.0/core/macro.concat_bytes.html)
> - [`env!`](https://doc.rust-lang.org/1.100.0/core/macro.env.html)
> - [`file!`](https://doc.rust-lang.org/1.100.0/core/macro.file.html)
> - [`format_args!`](https://doc.rust-lang.org/1.100.0/core/macro.format_args.html)
> - [`include!`](https://doc.rust-lang.org/1.100.0/core/macro.include.html)
> - [`include_bytes!`](https://doc.rust-lang.org/1.100.0/core/macro.include_bytes.html)
> - [`include_str!`](https://doc.rust-lang.org/1.100.0/core/macro.include_str.html)
> - [`line!`](https://doc.rust-lang.org/1.100.0/core/macro.line.html)
> - [`module_path!`](https://doc.rust-lang.org/1.100.0/core/macro.module_path.html)
> - [`option_env!`](https://doc.rust-lang.org/1.100.0/core/macro.option_env.html)
> - [`panic!`](https://doc.rust-lang.org/1.100.0/std/macro.panic.html)
> - [`stringify!`](https://doc.rust-lang.org/1.100.0/core/macro.stringify.html)
> - [`unreachable!`](https://doc.rust-lang.org/1.100.0/core/macro.unreachable.html)
>
> E.g., this works:
>
> ``` playground
> #![no_implicit_prelude]
> fn main() { assert!(true); }
> ```
>
> Don’t rely on this behavior; it may be removed in the future. Always bring the items you need into scope explicitly when using `#![no_implicit_prelude]`.
>
> For details, see [Rust PR \#62086](https://github.com/rust-lang/rust/pull/62086) and [Rust PR \#139493](https://github.com/rust-lang/rust/pull/139493).

[names.preludes.no_implicit_prelude.lang](#r-names.preludes.no_implicit_prelude.lang)

The `no_implicit_prelude` attribute does not affect the [language prelude](#language-prelude).

[names.preludes.no_implicit_prelude.edition2018](#r-names.preludes.no_implicit_prelude.edition2018)

> 2018 Edition differences
>
> In the 2015 edition, the `no_implicit_prelude` attribute does not affect the [`macro_use` prelude](#macro_use-prelude), and all macros exported from the standard library are still included in the `macro_use` prelude. Starting in the 2018 edition, the attribute does remove the `macro_use` prelude.
