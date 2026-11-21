# Application binary interface (ABI)

[abi](#r-abi)

# Application binary interface (ABI)

[abi.intro](#r-abi.intro)

This section documents features that affect the ABI of the compiled output of a crate.

See *[extern functions](items/functions.md#extern-function-qualifier)* for information on specifying the ABI for exporting functions. See *[external blocks](items/external-blocks.md)* for information on specifying the ABI for linking external libraries.

[abi.used](#r-abi.used)

## The `used` attribute

[abi.used.intro](#r-abi.used.intro)

The *`used` [attribute](attributes.md)* forces a [static](items/static-items.md) to be kept in the output object file (.o, .rlib, etc., excluding final binaries) even if it’s never used or referenced by any other item in the crate. The linker, however, is still free to remove it.

> ![](../assets/inline/abi-1.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> // lib.rs
>
> // This is kept because of `#[used]`.
> #[used]
> static S1: u8 = 0;
>
> // This is removable because it's unused.
> #[allow(dead_code)]
> static S2: u8 = 0;
>
> // This is kept because it's publicly reachable.
> pub static S3: u8 = 0;
>
> // This is kept because it's referenced by a publicly
> // reachable function.
> static S4: u8 = 0;
> #[unsafe(no_mangle)] pub fn f4() -> &'static u8 { &S4 }
>
> // This is removable because it's referenced only by a
> // private, unused (dead) function.
> static S5: u8 = 0;
> #[allow(dead_code)]
> fn f5() -> &'static u8 { &S5 }
> }
> ```
>
> ``` console
> $ rustc -O --emit=obj --crate-type=rlib lib.rs
> $ LC_ALL=C nm -C lib.o
> 0000000000000000 R lib::S1
> 0000000000000000 R lib::S3
> 0000000000000000 r lib::S4
> 0000000000000000 T f4
> ```

[abi.used.syntax](#r-abi.used.syntax)

The `used` attribute uses the [MetaWord](attributes.md#grammar-MetaWord) syntax.

[abi.used.allowed-positions](#r-abi.used.allowed-positions)

The `used` attribute may only be applied to [`static` items](items/static-items.md#r-items.static).

[abi.used.duplicates](#r-abi.used.duplicates)

Only the first use of `used` on an item has effect.

> ![](../assets/inline/abi-2.svg)Note
>
> `rustc` lints against any use following the first.

[abi.no_mangle](#r-abi.no_mangle)

## The `no_mangle` attribute

[abi.no_mangle.intro](#r-abi.no_mangle.intro)

The *`no_mangle` [attribute](attributes.md)* disables the standard symbol name mangling on a [function](items/functions.md) or [static](items/static-items.md). The symbol for the item is the item’s identifier.

> ![](../assets/inline/abi-3.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> #[unsafe(no_mangle)]
> extern "C" fn foo() {}
> }
> ```

[abi.no_mangle.syntax](#r-abi.no_mangle.syntax)

The `no_mangle` attribute uses the [MetaWord](attributes.md#grammar-MetaWord) syntax.

[abi.no_mangle.allowed-positions](#r-abi.no_mangle.allowed-positions)

The `no_mangle` attribute may only be applied to:

- [Static items](items/static-items.md#r-items.static)
- [Free functions](items/functions.md#r-items.fn)
- [Inherent associated functions](items/associated-items.md#r-items.associated.fn)
- [Trait impl functions](items/implementations.md#r-items.impl.trait)

> ![](../assets/inline/abi-4.svg)Note
>
> `rustc` lints against use in other positions. This may become an error in the future.

[abi.no_mangle.closures](#r-abi.no_mangle.closures)

The `no_mangle` attribute may not be used with a [closure](expressions/closure-expr.md#r-expr.closure).

[abi.no_mangle.generics](#r-abi.no_mangle.generics)

The `no_mangle` attribute may not be used on an item with generic parameters.

[abi.no_mangle.duplicates](#r-abi.no_mangle.duplicates)

Only the first use of `no_mangle` on an item has effect.

> ![](../assets/inline/abi-5.svg)Note
>
> `rustc` lints against any use following the first.

[abi.no_mangle.export_name](#r-abi.no_mangle.export_name)

When the `no_mangle` and [`export_name`](#r-abi.export_name) attributes are both applied to the same item, the symbol name from `export_name` is used, and `no_mangle` has no effect.

> ![](../assets/inline/abi-6.svg)Note
>
> `rustc` lints against this combination.

[abi.no_mangle.unsafe](#r-abi.no_mangle.unsafe)

The `no_mangle` attribute must be marked with [`unsafe`](attributes.md#r-attributes.safety) because an unmangled symbol may collide with another symbol with the same name (or with a well-known symbol), leading to undefined behavior.

[abi.no_mangle.edition2024](#r-abi.no_mangle.edition2024)

> 2024 Edition differences
>
> Before the 2024 edition, it is allowed to use the `no_mangle` attribute without `unsafe`.

[abi.no_mangle.publicly-exported](#r-abi.no_mangle.publicly-exported)

In addition to disabling name mangling, the `no_mangle` attribute causes the symbol to be publicly exported from the produced library or object file, similar to the [`used`](#r-abi.used) attribute.

[abi.no_mangle.ascii-only](#r-abi.no_mangle.ascii-only)

The `no_mangle` attribute may only be used on items with a name that contains only ASCII characters.

[abi.link_section](#r-abi.link_section)

## The `link_section` attribute

[abi.link_section.intro](#r-abi.link_section.intro)

The *`link_section` attribute* specifies the section of the object file that a [function](items/functions.md) or [static](items/static-items.md)’s content will be placed into.

[abi.link_section.syntax](#r-abi.link_section.syntax)

The `link_section` attribute uses the [MetaNameValueStr](attributes.md#grammar-MetaNameValueStr) syntax to specify the section name.

``` playground
#![allow(unused)]
fn main() {
#[cfg(target_os = "linux")] {
#[unsafe(no_mangle)]
#[unsafe(link_section = ".example_section")]
pub static VAR1: u32 = 1;
}
}
```

[abi.link_section.unsafe](#r-abi.link_section.unsafe)

This attribute is unsafe as it allows users to place data and code into sections of memory not expecting them, such as mutable data into read-only areas.

[abi.link_section.duplicates](#r-abi.link_section.duplicates)

Only the first use of `link_section` on an item has effect.

> ![](../assets/inline/abi-7.svg)Note
>
> `rustc` lints against any use following the first with a future-compatibility warning. This may become an error in the future.

[abi.link_section.edition2024](#r-abi.link_section.edition2024)

> 2024 Edition differences
>
> Before the 2024 edition it is allowed to use the `link_section` attribute without the `unsafe` qualification.

[abi.export_name](#r-abi.export_name)

## The `export_name` attribute

[abi.export_name.intro](#r-abi.export_name.intro)

The *`export_name` attribute* specifies the name of the symbol that will be exported on a [function](items/functions.md) or [static](items/static-items.md).

[abi.export_name.syntax](#r-abi.export_name.syntax)

The `export_name `attribute uses the [MetaNameValueStr](attributes.md#grammar-MetaNameValueStr) syntax to specify the symbol name.

``` playground
#![allow(unused)]
fn main() {
#[unsafe(export_name = "exported_symbol_name")]
pub fn name_in_rust() { }
}
```

[abi.export_name.unsafe](#r-abi.export_name.unsafe)

This attribute is unsafe as a symbol with a custom name may collide with another symbol with the same name (or with a well-known symbol), leading to undefined behavior.

[abi.export_name.duplicates](#r-abi.export_name.duplicates)

Only the first use of `export_name` on an item has effect.

> ![](../assets/inline/abi-8.svg)Note
>
> `rustc` lints against any use following the first with a future-compatibility warning. This may become an error in the future.

[abi.export_name.edition2024](#r-abi.export_name.edition2024)

> 2024 Edition differences
>
> Before the 2024 edition it is allowed to use the `export_name` attribute without the `unsafe` qualification.
