# Derive

[attributes.derive](#r-attributes.derive)

# Derive

[attributes.derive.intro](#r-attributes.derive.intro)

The *`derive` [attribute](../attributes.md#r-attributes)* invokes one or more [derive macros](../procedural-macros.md#r-macro.proc.derive), allowing new [items](../items.md) to be automatically generated for data structures. You can create `derive` macros with [procedural macros](../procedural-macros.md#r-macro.proc.derive).

> ![](../../assets/inline/derive-1.svg)Example
>
> The [`PartialEq`](../../core/cmp/derive.PartialEq.html) derive macro emits an [implementation](../items/implementations.md) of [`PartialEq`](../../core/cmp/trait.PartialEq.html) for `Foo<T> where T: PartialEq`. The [`Clone`](../../core/clone/derive.Clone.html) derive macro does likewise for [`Clone`](../../core/clone/trait.Clone.html).
>
> ``` playground
> #![allow(unused)]
> fn main() {
> #[derive(PartialEq, Clone)]
> struct Foo {
>     a: i32,
>     b: T,
> }
> }
> ```
>
> The generated `impl` items are equivalent to:
>
> ``` playground
> #![allow(unused)]
> fn main() {
> struct Foo { a: i32, b: T }
> impl PartialEq for Foo {
>     fn eq(&self, other: &Foo) -> bool {
>         self.a == other.a && self.b == other.b
>     }
> }
>
> impl Clone for Foo {
>     fn clone(&self) -> Self {
>         Foo { a: self.a.clone(), b: self.b.clone() }
>     }
> }
> }
> ```

[attributes.derive.syntax](#r-attributes.derive.syntax)

The `derive` attribute uses the [MetaListPaths](../attributes.md#grammar-MetaListPaths) syntax to specify a list of paths to [derive macros](../procedural-macros.md#r-macro.proc.derive) to invoke.

[attributes.derive.allowed-positions](#r-attributes.derive.allowed-positions)

The `derive` attribute may only be applied to [structs](../items/structs.md#r-items.struct), [enums](../items/enumerations.md#r-items.enum), and [unions](../items/unions.md#r-items.union).

[attributes.derive.duplicates](#r-attributes.derive.duplicates)

The `derive` attribute may be used any number of times on an item. All derive macros listed in all attributes are invoked.

[attributes.derive.stdlib](#r-attributes.derive.stdlib)

The `derive` attribute is exported in the standard library as:

- [`core::derive`](../../core/macros/builtin/attr.derive.html)
- [`std::derive`](../../core/macros/builtin/attr.derive.html)
- [`core::prelude::v1::derive`](../../core/macros/builtin/attr.derive.html)
- [`std::prelude::v1::derive`](../../core/macros/builtin/attr.derive.html)

[attributes.derive.built-in](#r-attributes.derive.built-in)

Built-in derives are defined in the [language prelude](../names/preludes.md#r-names.preludes.lang). The list of built-in derives are:

- [`Clone`](../../core/clone/trait.Clone.html)
- [`Copy`](../../core/marker/trait.Copy.html)
- [`Debug`](../../core/fmt/macros/derive.Debug.html)
- [`Default`](../../core/default/trait.Default.html)
- [`Eq`](../../core/cmp/trait.Eq.html)
- [`Hash`](../../core/hash/macros/derive.Hash.html)
- [`Ord`](../../core/cmp/trait.Ord.html)
- [`PartialEq`](../../core/cmp/trait.PartialEq.html)
- [`PartialOrd`](../../core/cmp/trait.PartialOrd.html)

[attributes.derive.built-in-automatically_derived](#r-attributes.derive.built-in-automatically_derived)

The built-in derives include the [`automatically_derived` attribute](#r-attributes.derive.automatically_derived) on the implementations they generate.

[attributes.derive.behavior](#r-attributes.derive.behavior)

During macro expansion, for each element in the list of derives, the corresponding derive macro expands to zero or more [items](../items.md).

[attributes.derive.automatically_derived](#r-attributes.derive.automatically_derived)

## The `automatically_derived` attribute

[attributes.derive.automatically_derived.intro](#r-attributes.derive.automatically_derived.intro)

The *`automatically_derived` [attribute](../attributes.md#r-attributes)* is used to annotate an [implementation](../items/implementations.md) to indicate that it was automatically created by a [derive macro](../procedural-macros.md#r-macro.proc.derive). It has no direct effect, but it may be used by tools and diagnostic lints to detect these automatically generated implementations.

> ![](../../assets/inline/derive-2.svg)Example
>
> Given [`#[derive(Clone)]`](../../core/clone/derive.Clone.html) on `struct Example`, the [derive macro](../procedural-macros.md#r-macro.proc.derive) may produce:
>
> ``` playground
> #![allow(unused)]
> fn main() {
> struct Example;
> #[automatically_derived]
> impl ::core::clone::Clone for Example {
>     #[inline]
>     fn clone(&self) -> Self {
>         Example
>     }
> }
> }
> ```

[attributes.derive.automatically_derived.syntax](#r-attributes.derive.automatically_derived.syntax)

The `automatically_derived` attribute uses the [MetaWord](../attributes.md#grammar-MetaWord) syntax.

[attributes.derive.automatically_derived.allowed-positions](#r-attributes.derive.automatically_derived.allowed-positions)

The `automatically_derived` attribute may only be applied to an [implementation](../items/implementations.md).

> ![](../../assets/inline/derive-3.svg)Note
>
> `rustc` ignores use in other positions but lints against it. This may become an error in the future.

[attributes.derive.automatically_derived.duplicates](#r-attributes.derive.automatically_derived.duplicates)

Using `automatically_derived` more than once on an implementation has the same effect as using it once.

> ![](../../assets/inline/derive-4.svg)Note
>
> `rustc` lints against any use following the first.

[attributes.derive.automatically_derived.behavior](#r-attributes.derive.automatically_derived.behavior)

The `automatically_derived` attribute has no behavior.
