# Limits

[attributes.limits](#r-attributes.limits)

# Limits

[attributes.limits.intro](#r-attributes.limits.intro)

The following [attributes](../attributes.md) affect compile-time limits.

[attributes.limits.recursion_limit](#r-attributes.limits.recursion_limit)

## The `recursion_limit` attribute

[attributes.limits.recursion_limit.intro](#r-attributes.limits.recursion_limit.intro)

The *`recursion_limit` attribute* may be applied at the [crate](../crates-and-source-files.md) level to set the maximum depth for potentially infinitely-recursive compile-time operations like macro expansion or auto-dereference.

[attributes.limits.recursion_limit.syntax](#r-attributes.limits.recursion_limit.syntax)

It uses the [MetaNameValueStr](../attributes.md#grammar-MetaNameValueStr) syntax to specify the recursion depth.

> ![](../../assets/inline/limits-1.svg)Note
>
> The default in `rustc` is 128.

``` playground
#![allow(unused)]
#![recursion_limit = "4"]

fn main() {
macro_rules! a {
    () => { a!(1); };
    (1) => { a!(2); };
    (2) => { a!(3); };
    (3) => { a!(4); };
    (4) => { };
}

// This fails to expand because it requires a recursion depth greater than 4.
a!{}
}
```

``` playground
#![allow(unused)]
#![recursion_limit = "1"]

fn main() {
// This fails because it requires two recursive steps to auto-dereference.
(|_: &u8| {})(&&&1);
}
```

[attributes.limits.type_length_limit](#r-attributes.limits.type_length_limit)

## The `type_length_limit` attribute

[attributes.limits.type_length_limit.intro](#r-attributes.limits.type_length_limit.intro)

The *`type_length_limit` [attribute](../attributes.md)* sets the maximum number of type substitutions allowed when constructing a concrete type during monomorphization.

> ![](../../assets/inline/limits-2.svg)Note
>
> `rustc` only enforces the limit when the nightly `-Zenforce-type-length-limit` flag is active.
>
> For more information, see [Rust PR \#127670](https://github.com/rust-lang/rust/pull/127670).

> ![](../../assets/inline/limits-3.svg)Example
>
> ``` rust
> #![type_length_limit = "4"]
>
> fn f(x: T) {}
>
> // This fails to compile because monomorphizing to
> // `f::<((((i32,), i32), i32), i32)>` requires more
> // than 4 type elements.
> f(((((1,), 2), 3), 4));
> ```

> ![](../../assets/inline/limits-4.svg)Note
>
> The default value in `rustc` is `1048576`.

[attributes.limits.type_length_limit.syntax](#r-attributes.limits.type_length_limit.syntax)

The `type_length_limit` attribute uses the [MetaNameValueStr](../attributes.md#grammar-MetaNameValueStr) syntax. The value in the string must be a non-negative number.

[attributes.limits.type_length_limit.allowed-positions](#r-attributes.limits.type_length_limit.allowed-positions)

The `type_length_limit` attribute may only be applied to the crate root.

> ![](../../assets/inline/limits-5.svg)Note
>
> `rustc` ignores use in other positions but lints against it. This may become an error in the future.

[attributes.limits.type_length_limit.duplicates](#r-attributes.limits.type_length_limit.duplicates)

Only the first use of `type_length_limit` on an item has effect.

> ![](../../assets/inline/limits-6.svg)Note
>
> `rustc` lints against any use following the first. This may become an error in the future.
