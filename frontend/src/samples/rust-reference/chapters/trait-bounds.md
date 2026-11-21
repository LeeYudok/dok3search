# Trait and lifetime bounds

[bound](#r-bound)

# Trait and lifetime bounds

[bound.syntax](#r-bound.syntax)

**Syntax**\
[Bounds](#railroad-Bounds) → [Bound](#grammar-Bound) ( + [Bound](#grammar-Bound) )\* +?

[Bound](#railroad-Bound) → [Lifetime](#grammar-Lifetime) \| [TraitBound](#grammar-TraitBound) \| [UseBound](#grammar-UseBound)

[TraitBound](#railroad-TraitBound) →\
      ( ? \| [ForLifetimes](#grammar-ForLifetimes) )? [TypePath](paths.md#grammar-TypePath)\
    \| ( ( ? \| [ForLifetimes](#grammar-ForLifetimes) )? [TypePath](paths.md#grammar-TypePath) )

[LifetimeBounds](#railroad-LifetimeBounds) → ( [Lifetime](#grammar-Lifetime) + )\* [Lifetime](#grammar-Lifetime)?

[Lifetime](#railroad-Lifetime) →\
      [LIFETIME_OR_LABEL](tokens.md#grammar-LIFETIME_OR_LABEL)\
    \| 'static\
    \| '\_

[UseBound](#railroad-UseBound) → use [UseBoundGenericArgs](#grammar-UseBoundGenericArgs)

[UseBoundGenericArgs](#railroad-UseBoundGenericArgs) →\
      \< \>\
    \| \< ( [UseBoundGenericArg](#grammar-UseBoundGenericArg) , )\* [UseBoundGenericArg](#grammar-UseBoundGenericArg) ,? \>

[UseBoundGenericArg](#railroad-UseBoundGenericArg) →\
      [Lifetime](#grammar-Lifetime)\
    \| [IDENTIFIER](identifiers.md#grammar-IDENTIFIER)\
    \| Self

Show Railroad

![](../assets/inline/trait-bounds-1.svg) ![](../assets/inline/trait-bounds-2.svg) ![](../assets/inline/trait-bounds-3.svg) ![](../assets/inline/trait-bounds-4.svg) ![](../assets/inline/trait-bounds-5.svg) ![](../assets/inline/trait-bounds-6.svg) ![](../assets/inline/trait-bounds-7.svg) ![](../assets/inline/trait-bounds-8.svg) [bound.intro](#r-bound.intro)

[Trait](items/traits.md#trait-bounds) and lifetime bounds provide a way for [generic items](items/generics.md) to restrict which types and lifetimes are used as their parameters. Bounds can be provided on any type in a [where clause](items/generics.md#where-clauses). There are also shorter forms for certain common cases:

- Bounds written after declaring a [generic parameter](items/generics.md): `fn f<A: Copy>() {}` is the same as `fn f<A>() where A: Copy {}`.
- In trait declarations as [supertraits](items/traits.md#supertraits): `trait Circle : Shape {}` is equivalent to `trait Circle where Self : Shape {}`.
- In trait declarations as bounds on [associated types](items/associated-items.md#associated-types): `trait A { type B: Copy; }` is equivalent to `trait A where Self::B: Copy { type B; }`.

[bound.satisfaction](#r-bound.satisfaction)

Bounds on an item must be satisfied when using the item. When type checking and borrow checking a generic item, the bounds can be used to determine that a trait is implemented for a type. For example, given `Ty: Trait`

- In the body of a generic function, methods from `Trait` can be called on `Ty` values. Likewise associated constants on the `Trait` can be used.
- Associated types from `Trait` can be used.
- Generic functions and types with a `T: Trait` bounds can be used with `Ty` being used for `T`.

``` playground
#![allow(unused)]
fn main() {
type Surface = i32;
trait Shape {
    fn draw(&self, surface: Surface);
    fn name() -> &'static str;
}

fn draw_twice<T: Shape>(surface: Surface, sh: T) {
    sh.draw(surface);           // Can call method because T: Shape
    sh.draw(surface);
}

fn copy_and_draw_twice<T: Copy>(surface: Surface, sh: T) where T: Shape {
    let shape_copy = sh;        // doesn't move sh because T: Copy
    draw_twice(surface, sh);    // Can use generic function because T: Shape
}

struct Figure<S: Shape>(S, S);

fn name_figure<U: Shape>(
    figure: Figure<U>,          // Type Figure<U> is well-formed because U: Shape
) {
    println!(
        "Figure of two {}",
        U::name(),              // Can use associated function
    );
}
}
```

[bound.trivial](#r-bound.trivial)

Bounds that don’t use the item’s parameters or [higher-ranked lifetimes](#higher-ranked-trait-bounds) are checked when the item is defined. It is an error for such a bound to be false.

[bound.special](#r-bound.special)

[`Copy`](../core/marker/trait.Copy.html), [`Clone`](../core/clone/trait.Clone.html), and [`Sized`](../core/marker/trait.Sized.html) bounds are also checked for certain generic types when using the item, even if the use does not provide a concrete type. It is an error to have `Copy` or `Clone` as a bound on a mutable reference, [trait object](types/trait-object.md), or [slice](types/slice.md). It is an error to have `Sized` as a bound on a trait object or slice.

``` playground
#![allow(unused)]
fn main() {
struct A<'a, T>
where
    i32: Default,           // Allowed, but not useful
    i32: Iterator,          // Error: `i32` is not an iterator
    &'a mut T: Copy,        // (at use) Error: the trait bound is not satisfied
    [T]: Sized,             // (at use) Error: size cannot be known at compilation
{
    f: &'a T,
}
struct UsesA<'a, T>(A<'a, T>);
}
```

[bound.trait-object](#r-bound.trait-object)

Trait and lifetime bounds are also used to name [trait objects](types/trait-object.md).

[bound.sized](#r-bound.sized)

## `?Sized`

`?` is only used to relax the implicit [`Sized`](../core/marker/trait.Sized.html) trait bound for [type parameters](types/parameters.md) or [associated types](items/associated-items.md#associated-types). `?Sized` may not be used as a bound for other types.

[bound.lifetime](#r-bound.lifetime)

## Lifetime bounds

[bound.lifetime.intro](#r-bound.lifetime.intro)

Lifetime bounds can be applied to types or to other lifetimes.

[bound.lifetime.outlive-lifetime](#r-bound.lifetime.outlive-lifetime)

The bound `'a: 'b` is usually read as `'a` *outlives* `'b`. `'a: 'b` means that `'a` lasts at least as long as `'b`, so a reference `&'a ()` is valid whenever `&'b ()` is valid.

``` playground
#![allow(unused)]
fn main() {
fn f<'a, 'b>(x: &'a i32, mut y: &'b i32) where 'a: 'b {
    y = x;                      // &'a i32 is a subtype of &'b i32 because 'a: 'b
    let r: &'b &'a i32 = &&0;   // &'b &'a i32 is well formed because 'a: 'b
}
}
```

[bound.lifetime.outlive-type](#r-bound.lifetime.outlive-type)

`T: 'a` means that all lifetime parameters of `T` outlive `'a`. For example, if `'a` is an unconstrained lifetime parameter, then `i32: 'static` and `&'static str: 'a` are satisfied, but `Vec<&'a ()>: 'static` is not.

[bound.higher-ranked](#r-bound.higher-ranked)

## Higher-ranked trait bounds

[bound.higher-ranked.syntax](#r-bound.higher-ranked.syntax)

**Syntax**\
[ForLifetimes](#railroad-ForLifetimes) → for [GenericParams](items/generics.md#grammar-GenericParams)

Show Railroad

![](../assets/inline/trait-bounds-9.svg) [bound.higher-ranked.intro](#r-bound.higher-ranked.intro)

Trait bounds may be *higher ranked* over lifetimes. These bounds specify a bound that is true *for all* lifetimes. For example, a bound such as `for<'a> &'a T: PartialEq<i32>` would require an implementation like

``` playground
#![allow(unused)]
fn main() {
struct T;
impl<'a> PartialEq<i32> for &'a T {
    // ...
   fn eq(&self, other: &i32) -> bool {true}
}
}
```

and could then be used to compare a `&'a T` with any lifetime to an `i32`.

Only a higher-ranked bound can be used here, because the lifetime of the reference is shorter than any possible lifetime parameter on the function:

``` playground
#![allow(unused)]
fn main() {
fn call_on_ref_zero<F>(f: F) where for<'a> F: Fn(&'a i32) {
    let zero = 0;
    f(&zero);
}
}
```

[bound.higher-ranked.trait](#r-bound.higher-ranked.trait)

Higher-ranked lifetimes may also be specified just before the trait: the only difference is the [scope](names/scopes.md#higher-ranked-trait-bound-scopes) of the lifetime parameter, which extends only to the end of the following trait instead of the whole bound. This function is equivalent to the last one.

``` playground
#![allow(unused)]
fn main() {
fn call_on_ref_zero<F>(f: F) where F: for<'a> Fn(&'a i32) {
    let zero = 0;
    f(&zero);
}
}
```

[bound.implied](#r-bound.implied)

## Implied bounds

[bound.implied.intro](#r-bound.implied.intro)

Lifetime bounds required for types to be well-formed are sometimes inferred.

``` playground
#![allow(unused)]
fn main() {
fn requires_t_outlives_a<'a, T>(x: &'a T) {}
}
```

The type parameter `T` is required to outlive `'a` for the type `&'a T` to be well-formed. This is inferred because the function signature contains the type `&'a T` which is only valid if `T: 'a` holds.

[bound.implied.context](#r-bound.implied.context)

Implied bounds are added for all parameters and outputs of functions. Inside of `requires_t_outlives_a` you can assume `T: 'a` to hold even if you don’t explicitly specify this:

``` playground
#![allow(unused)]
fn main() {
fn requires_t_outlives_a_not_implied<'a, T: 'a>() {}

fn requires_t_outlives_a<'a, T>(x: &'a T) {
    // This compiles, because `T: 'a` is implied by
    // the reference type `&'a T`.
    requires_t_outlives_a_not_implied::<'a, T>();
}
}
```

``` playground
#![allow(unused)]
fn main() {
fn requires_t_outlives_a_not_implied<'a, T: 'a>() {}
fn not_implied<'a, T>() {
    // This errors, because `T: 'a` is not implied by
    // the function signature.
    requires_t_outlives_a_not_implied::<'a, T>();
}
}
```

[bound.implied.trait](#r-bound.implied.trait)

Only lifetime bounds are implied, trait bounds still have to be explicitly added. The following example therefore causes an error:

``` playground
#![allow(unused)]
fn main() {
use std::fmt::Debug;
struct IsDebug<T: Debug>(T);
// error[E0277]: `T` doesn't implement `Debug`
fn doesnt_specify_t_debug<T>(x: IsDebug<T>) {}
}
```

[bound.implied.def](#r-bound.implied.def)

Lifetime bounds are also inferred for type definitions and impl blocks for any type:

``` playground
#![allow(unused)]
fn main() {
struct Struct<'a, T> {
    // This requires `T: 'a` to be well-formed
    // which is inferred by the compiler.
    field: &'a T,
}

enum Enum<'a, T> {
    // This requires `T: 'a` to be well-formed,
    // which is inferred by the compiler.
    //
    // Note that `T: 'a` is required even when only
    // using `Enum::OtherVariant`.
    SomeVariant(&'a T),
    OtherVariant,
}

trait Trait<'a, T: 'a> {}

// This would error because `T: 'a` is not implied by any type
// in the impl header.
//     impl<'a, T> Trait<'a, T> for () {}

// This compiles as `T: 'a` is implied by the self type `&'a T`.
impl<'a, T> Trait<'a, T> for &'a T {}
}
```

[bound.use](#r-bound.use)

## Use bounds

Certain bounds lists may include a `use<..>` bound to control which generic parameters are captured by the `impl Trait` [abstract return type](types/impl-trait.md#abstract-return-types). See [precise capturing](types/impl-trait.md#precise-capturing) for more details.
