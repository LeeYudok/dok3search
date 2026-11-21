# Implementations

[items.impl](#r-items.impl)

# Implementations

[items.impl.syntax](#r-items.impl.syntax)

**Syntax**\
[Implementation](#railroad-Implementation) → [InherentImpl](#grammar-InherentImpl) \| [TraitImpl](#grammar-TraitImpl)

[InherentImpl](#railroad-InherentImpl) →\
    impl [GenericParams](generics.md#grammar-GenericParams)? [Type](../types.md#grammar-Type) [WhereClause](generics.md#grammar-WhereClause)? {\
        [InnerAttribute](../attributes.md#grammar-InnerAttribute)\*\
        [AssociatedItem](associated-items.md#grammar-AssociatedItem)\*\
    }

[TraitImpl](#railroad-TraitImpl) →\
    unsafe? impl [GenericParams](generics.md#grammar-GenericParams)? !? [TypePath](../paths.md#grammar-TypePath) for [Type](../types.md#grammar-Type)\
    [WhereClause](generics.md#grammar-WhereClause)?\
    {\
        [InnerAttribute](../attributes.md#grammar-InnerAttribute)\*\
        [AssociatedItem](associated-items.md#grammar-AssociatedItem)\*\
    }

Show Railroad

![](../../assets/inline/implementations-1.svg) ![](../../assets/inline/implementations-2.svg) ![](../../assets/inline/implementations-3.svg) [items.impl.intro](#r-items.impl.intro)

An *implementation* is an item that associates items with an *implementing type*. Implementations are defined with the keyword `impl` and contain functions that belong to an instance of the type that is being implemented or to the type statically.

[items.impl.kinds](#r-items.impl.kinds)

There are two types of implementations:

- inherent implementations
- [trait](traits.md) implementations

[items.impl.inherent](#r-items.impl.inherent)

## Inherent implementations

[items.impl.inherent.intro](#r-items.impl.inherent.intro)

An inherent implementation is defined as the sequence of the `impl` keyword, generic type declarations, a path to a nominal type, a where clause, and a bracketed set of associable items.

[items.impl.inherent.implementing-type](#r-items.impl.inherent.implementing-type)

The nominal type is called the *implementing type* and the associable items are the *associated items* to the implementing type.

[items.impl.inherent.associated-items](#r-items.impl.inherent.associated-items)

Inherent implementations associate the contained items to the implementing type.

[items.impl.inherent.allowed-items](#r-items.impl.inherent.allowed-items)

Inherent implementations can contain [associated functions](associated-items.md#associated-functions-and-methods) (including [methods](associated-items.md#methods)) and [associated constants](associated-items.md#associated-constants).

[items.impl.inherent.type-alias](#r-items.impl.inherent.type-alias)

They cannot contain associated type aliases.

[items.impl.inherent.associated-item-path](#r-items.impl.inherent.associated-item-path)

The [path](../paths.md) to an associated item is any path to the implementing type, followed by the associated item’s identifier as the final path component.

[items.impl.inherent.coherence](#r-items.impl.inherent.coherence)

A type can also have multiple inherent implementations. An implementing type must be defined within the same crate as the original type definition.

``` playground
pub mod color {
    pub struct Color(pub u8, pub u8, pub u8);

    impl Color {
        pub const WHITE: Color = Color(255, 255, 255);
    }
}

mod values {
    use super::color::Color;
    impl Color {
        pub fn red() -> Color {
            Color(255, 0, 0)
        }
    }
}

pub use self::color::Color;
fn main() {
    // Actual path to the implementing type and impl in the same module.
    color::Color::WHITE;

    // Impl blocks in different modules are still accessed through a path to the type.
    color::Color::red();

    // Re-exported paths to the implementing type also work.
    Color::red();

    // Does not work, because use in `values` is not pub.
    // values::Color::red();
}
```

[items.impl.trait](#r-items.impl.trait)

## Trait implementations

[items.impl.trait.intro](#r-items.impl.trait.intro)

A *trait implementation* is defined like an inherent implementation except that the optional generic type declarations are followed by a [trait](traits.md), followed by the keyword `for`, followed by a path to a nominal type.

[items.impl.trait.implemented-trait](#r-items.impl.trait.implemented-trait)

The trait is known as the *implemented trait*. The implementing type implements the implemented trait.

[items.impl.trait.def-requirement](#r-items.impl.trait.def-requirement)

A trait implementation must define all non-default associated items declared by the implemented trait, may redefine default associated items defined by the implemented trait, and cannot define any other items.

[items.impl.trait.associated-item-path](#r-items.impl.trait.associated-item-path)

The path to the associated items is `<` followed by a path to the implementing type followed by `as` followed by a path to the trait followed by `>` as a path component followed by the associated item’s path component.

[items.impl.trait.safety](#r-items.impl.trait.safety)

[Unsafe traits](traits.md#unsafe-traits) require the trait implementation to begin with the `unsafe` keyword.

``` playground
#![allow(unused)]
fn main() {
#[derive(Copy, Clone)]
struct Point {x: f64, y: f64};
type Surface = i32;
struct BoundingBox {x: f64, y: f64, width: f64, height: f64};
trait Shape { fn draw(&self, s: Surface); fn bounding_box(&self) -> BoundingBox; }
fn do_draw_circle(s: Surface, c: Circle) { }
struct Circle {
    radius: f64,
    center: Point,
}

impl Copy for Circle {}

impl Clone for Circle {
    fn clone(&self) -> Circle { *self }
}

impl Shape for Circle {
    fn draw(&self, s: Surface) { do_draw_circle(s, *self); }
    fn bounding_box(&self) -> BoundingBox {
        let r = self.radius;
        BoundingBox {
            x: self.center.x - r,
            y: self.center.y - r,
            width: 2.0 * r,
            height: 2.0 * r,
        }
    }
}
}
```

[items.impl.trait.coherence](#r-items.impl.trait.coherence)

### Trait implementation coherence

[items.impl.trait.coherence.intro](#r-items.impl.trait.coherence.intro)

A trait implementation is considered incoherent if either the orphan rules check fails or there are overlapping implementation instances.

[items.impl.trait.coherence.overlapping](#r-items.impl.trait.coherence.overlapping)

Two trait implementations overlap when there is a non-empty intersection of the traits the implementation is for, the implementations can be instantiated with the same type.

[items.impl.trait.orphan-rule](#r-items.impl.trait.orphan-rule)

#### Orphan rules

[items.impl.trait.orphan-rule.intro](#r-items.impl.trait.orphan-rule.intro)

The *orphan rule* states that a trait implementation is only allowed if either the trait or at least one of the types in the implementation is defined in the current crate. It prevents conflicting trait implementations across different crates and is key to ensuring coherence.

An orphan implementation is one that implements a foreign trait for a foreign type. If these were freely allowed, two crates could implement the same trait for the same type in incompatible ways, creating a situation where adding or updating a dependency could break compilation due to conflicting implementations.

The orphan rule enables library authors to add new implementations to their traits without fear that they’ll break downstream code. Without these restrictions, a library couldn’t add an implementation like `impl<T: Display> MyTrait for T` without potentially conflicting with downstream implementations.

[items.impl.trait.orphan-rule.def](#r-items.impl.trait.orphan-rule.def)

Given `impl<P1..=Pn> Trait<T1..=Tn> for T0`, an `impl` is valid only if at least one of the following is true:

- `Trait` is a [local trait](../glossary.md#local-trait)
- All of
  - At least one of the types `T0..=Tn` must be a [local type](../glossary.md#local-type). Let `Ti` be the first such type.
  - No [uncovered type](../glossary.md#uncovered-type) parameters `P1..=Pn` may appear in `T0..Ti` (excluding `Ti`)

[items.impl.trait.uncovered-param](#r-items.impl.trait.uncovered-param)

Only the appearance of *uncovered* type parameters is restricted.

[items.impl.trait.fundamental](#r-items.impl.trait.fundamental)

Note that for the purposes of coherence, [fundamental types](../glossary.md#fundamental-type-constructors) are special. The `T` in `Box<T>` is not considered covered, and `Box<LocalType>` is considered local.

[items.impl.generics](#r-items.impl.generics)

## Generic implementations

[items.impl.generics.intro](#r-items.impl.generics.intro)

An implementation can take [generic parameters](generics.md), which can be used in the rest of the implementation. Implementation parameters are written directly after the `impl` keyword.

``` playground
#![allow(unused)]
fn main() {
trait Seq<T> { fn dummy(&self, _: T) { } }
impl<T> Seq<T> for Vec<T> {
    /* ... */
}
impl Seq<bool> for u32 {
    /* Treat the integer as a sequence of bits */
}
}
```

[items.impl.generics.use](#r-items.impl.generics.use)

Generic parameters *constrain* an implementation if the parameter appears at least once in one of:

- The implemented trait, if it has one
- The implementing type
- As an [associated type](associated-items.md#associated-types) in the [bounds](../trait-bounds.md) of a type that contains another parameter that constrains the implementation

[items.impl.generics.constrain](#r-items.impl.generics.constrain)

Type and const parameters must always constrain the implementation. Lifetimes must constrain the implementation if the lifetime is used in an associated type.

Examples of constraining situations:

``` playground
#![allow(unused)]
fn main() {
trait Trait{}
trait GenericTrait<T> {}
trait HasAssocType { type Ty; }
struct Struct;
struct GenericStruct<T>(T);
struct ConstGenericStruct<const N: usize>([(); N]);
// T constrains by being an argument to GenericTrait.
impl<T> GenericTrait<T> for i32 { /* ... */ }

// T constrains by being an argument to GenericStruct
impl<T> Trait for GenericStruct<T> { /* ... */ }

// Likewise, N constrains by being an argument to ConstGenericStruct
impl<const N: usize> Trait for ConstGenericStruct<N> { /* ... */ }

// T constrains by being in an associated type in a bound for type `U` which is
// itself a generic parameter constraining the trait.
impl<T, U> GenericTrait<U> for u32 where U: HasAssocType<Ty = T> { /* ... */ }

// Like previous, except the type is `(U, isize)`. `U` appears inside the type
// that includes `T`, and is not the type itself.
impl<T, U> GenericStruct<U> where (U, isize): HasAssocType<Ty = T> { /* ... */ }
}
```

Examples of non-constraining situations:

``` playground
#![allow(unused)]
fn main() {
// The rest of these are errors, since they have type or const parameters that
// do not constrain.

// T does not constrain since it does not appear at all.
impl<T> Struct { /* ... */ }

// N does not constrain for the same reason.
impl<const N: usize> Struct { /* ... */ }

// Usage of T inside the implementation does not constrain the impl.
impl<T> Struct {
    fn uses_t(t: &T) { /* ... */ }
}

// T is used as an associated type in the bounds for U, but U does not constrain.
impl<T, U> Struct where U: HasAssocType<Ty = T> { /* ... */ }

// T is used in the bounds, but not as an associated type, so it does not constrain.
impl<T, U> GenericTrait<U> for u32 where U: GenericTrait<T> {}
}
```

Example of an allowed unconstraining lifetime parameter:

``` playground
#![allow(unused)]
fn main() {
struct Struct;
impl<'a> Struct {}
}
```

Example of a disallowed unconstraining lifetime parameter:

``` playground
#![allow(unused)]
fn main() {
struct Struct;
trait HasAssocType { type Ty; }
impl<'a> HasAssocType for Struct {
    type Ty = &'a Struct;
}
}
```

[items.impl.attributes](#r-items.impl.attributes)

## Attributes on implementations

Implementations may contain outer [attributes](../attributes.md) before the `impl` keyword and inner [attributes](../attributes.md) inside the brackets that contain the associated items. Inner attributes must come before any associated items. The attributes that have meaning here are [`cfg`](../conditional-compilation.md), [`deprecated`](../attributes/diagnostics.md#the-deprecated-attribute), [`doc`](https://doc.rust-lang.org/1.100.0/rustdoc/the-doc-attribute.html), and [the lint check attributes](../attributes/diagnostics.md#lint-check-attributes).
