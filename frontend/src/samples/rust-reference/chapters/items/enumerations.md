# Enumerations

[items.enum](#r-items.enum)

# Enumerations

[items.enum.syntax](#r-items.enum.syntax)

**Syntax**\
[Enumeration](#railroad-Enumeration) →\
    enum [IDENTIFIER](../identifiers.md#grammar-IDENTIFIER) [GenericParams](generics.md#grammar-GenericParams)? [WhereClause](generics.md#grammar-WhereClause)? { [EnumVariants](#grammar-EnumVariants)? }

[EnumVariants](#railroad-EnumVariants) → [EnumVariant](#grammar-EnumVariant) ( , [EnumVariant](#grammar-EnumVariant) )\* ,?

[EnumVariant](#railroad-EnumVariant) →\
    [OuterAttribute](../attributes.md#grammar-OuterAttribute)\* [Visibility](../visibility-and-privacy.md#grammar-Visibility)?\
    [IDENTIFIER](../identifiers.md#grammar-IDENTIFIER) ( [EnumVariantTuple](#grammar-EnumVariantTuple) \| [EnumVariantStruct](#grammar-EnumVariantStruct) )? [EnumVariantDiscriminant](#grammar-EnumVariantDiscriminant)?

[EnumVariantTuple](#railroad-EnumVariantTuple) → ( [TupleFields](structs.md#grammar-TupleFields)? )

[EnumVariantStruct](#railroad-EnumVariantStruct) → { [StructFields](structs.md#grammar-StructFields)? }

[EnumVariantDiscriminant](#railroad-EnumVariantDiscriminant) → = [Expression](../expressions.md#grammar-Expression)

Show Railroad

![](../../assets/inline/enumerations-1.svg) ![](../../assets/inline/enumerations-2.svg) ![](../../assets/inline/enumerations-3.svg) ![](../../assets/inline/enumerations-4.svg) ![](../../assets/inline/enumerations-5.svg) ![](../../assets/inline/enumerations-6.svg) [items.enum.intro](#r-items.enum.intro)

An *enumeration*, also referred to as an *enum*, is a simultaneous definition of a nominal [enumerated type](../types/enum.md) as well as a set of *constructors*, that can be used to create or pattern-match values of the corresponding enumerated type.

[items.enum.decl](#r-items.enum.decl)

Enumerations are declared with the keyword `enum`.

[items.enum.namespace](#r-items.enum.namespace)

The `enum` declaration defines the enumeration type in the [type namespace](../names/namespaces.md) of the module or block where it is located.

An example of an `enum` item and its use:

``` playground
#![allow(unused)]
fn main() {
enum Animal {
    Dog,
    Cat,
}

let mut a: Animal = Animal::Dog;
a = Animal::Cat;
}
```

[items.enum.constructor](#r-items.enum.constructor)

Enum constructors can have either named or unnamed fields:

``` playground
#![allow(unused)]
fn main() {
enum Animal {
    Dog(String, f64),
    Cat { name: String, weight: f64 },
}

let mut a: Animal = Animal::Dog("Cocoa".to_string(), 37.2);
a = Animal::Cat { name: "Spotty".to_string(), weight: 2.7 };
}
```

In this example, `Cat` is a *struct-like enum variant*, whereas `Dog` is simply called an enum variant.

[items.enum.fieldless](#r-items.enum.fieldless)

An enum where no constructors contain fields is called a *field-less enum*. For example, this is a fieldless enum:

``` playground
#![allow(unused)]
fn main() {
enum Fieldless {
    Tuple(),
    Struct{},
    Unit,
}
}
```

[items.enum.unit-only](#r-items.enum.unit-only)

If a field-less enum only contains unit variants, the enum is called an *unit-only enum*. For example:

``` playground
#![allow(unused)]
fn main() {
enum Enum {
    Foo = 3,
    Bar = 2,
    Baz = 1,
}
}
```

[items.enum.constructor-names](#r-items.enum.constructor-names)

Variant constructors are similar to [struct](structs.md) definitions, and can be referenced by a path from the enumeration name, including in [use declarations](use-declarations.md).

[items.enum.constructor-namespace](#r-items.enum.constructor-namespace)

Each variant defines its type in the [type namespace](../names/namespaces.md), though that type cannot be used as a type specifier. Tuple-like and unit-like variants also define a constructor in the [value namespace](../names/namespaces.md).

[items.enum.struct-expr](#r-items.enum.struct-expr)

A struct-like variant can be instantiated with a [struct expression](../expressions/struct-expr.md).

[items.enum.tuple-expr](#r-items.enum.tuple-expr)

A tuple-like variant can be instantiated with a [call expression](../expressions/call-expr.md) or a [struct expression](../expressions/struct-expr.md).

[items.enum.path-expr](#r-items.enum.path-expr)

A unit-like variant can be instantiated with a [path expression](../expressions/path-expr.md) or a [struct expression](../expressions/struct-expr.md). For example:

``` playground
#![allow(unused)]
fn main() {
enum Examples {
    UnitLike,
    TupleLike(i32),
    StructLike { value: i32 },
}

use Examples::*; // Creates aliases to all variants.
let x = UnitLike; // Path expression of the const item.
let x = UnitLike {}; // Struct expression.
let y = TupleLike(123); // Call expression.
let y = TupleLike { 0: 123 }; // Struct expression using integer field names.
let z = StructLike { value: 123 }; // Struct expression.
}
```

[items.enum.discriminant](#r-items.enum.discriminant)

## Discriminants

[items.enum.discriminant.intro](#r-items.enum.discriminant.intro)

Each enum instance has a *discriminant*: an integer logically associated to it that is used to determine which variant it holds.

[items.enum.discriminant.type](#r-items.enum.discriminant.type)

Enums without a [primitive representation](../type-layout.md#primitive-representations) have discriminants of type `isize`. However, the compiler may use a smaller type or another means of distinguishing variants in the actual memory layout.

``` playground
#![allow(unused)]
fn main() {
use core::mem::size_of;
enum E {
    V1 = 0isize, // OK: `isize` is the discriminant type.
    V2,
}

assert!(size_of::<E>() <= size_of::<isize>());
}
```

``` playground
#![allow(unused)]
fn main() {
enum E {
    V = 0u8, // ERROR: Expected `isize`, found `u8`.
}
}
```

[items.enum.discriminant.type-primitive](#r-items.enum.discriminant.type-primitive)

Enums with a [primitive representation](../type-layout.md#primitive-representations) have discriminants of the type named by the representation. This also applies to enums that combine the `C` representation with a primitive representation (see [layout.repr.primitive-c](../type-layout.md#r-layout.repr.primitive-c)).

``` playground
#![allow(unused)]
fn main() {
#[repr(u8)]
enum E {
    V = 0u8, // OK: `u8` is the discriminant type.
}
}
```

``` playground
#![allow(unused)]
fn main() {
#[repr(u8)]
enum E {
    V = 0isize, // ERROR: Expected `u8`, found `isize`.
}
}
```

### Assigning discriminant values

[items.enum.discriminant.explicit](#r-items.enum.discriminant.explicit)

#### Explicit discriminants

[items.enum.discriminant.explicit.intro](#r-items.enum.discriminant.explicit.intro)

In two circumstances, the discriminant of a variant may be explicitly set by following the variant name with `=` and a [constant expression](../const_eval.md#constant-expressions):

[items.enum.discriminant.explicit.unit-only](#r-items.enum.discriminant.explicit.unit-only)

1.  if the enumeration is “[unit-only](#unit-only-enum)”.

[items.enum.discriminant.explicit.primitive-repr](#r-items.enum.discriminant.explicit.primitive-repr)

2.  if a [primitive representation](../type-layout.md#primitive-representations) is used. For example:

    ``` playground
    #![allow(unused)]
    fn main() {
    #[repr(u8)]
    enum Enum {
        Unit = 3,
        Tuple(u16),
        Struct {
            a: u8,
            b: u16,
        } = 1,
    }
    }
    ```

[items.enum.discriminant.implicit](#r-items.enum.discriminant.implicit)

#### Implicit discriminants

If a discriminant for a variant is not specified, then it is set to one higher than the discriminant of the previous variant in the declaration. If the discriminant of the first variant in the declaration is unspecified, then it is set to zero.

``` playground
#![allow(unused)]
fn main() {
enum Foo {
    Bar,            // 0
    Baz = 123,      // 123
    Quux,           // 124
}

let baz_discriminant = Foo::Baz as u32;
assert_eq!(baz_discriminant, 123);
}
```

[items.enum.discriminant.restrictions](#r-items.enum.discriminant.restrictions)

#### Restrictions

[items.enum.discriminant.restrictions.same-discriminant](#r-items.enum.discriminant.restrictions.same-discriminant)

It is an error when two variants share the same discriminant.

``` playground
#![allow(unused)]
fn main() {
enum SharedDiscriminantError {
    SharedA = 1,
    SharedB = 1,
}

enum SharedDiscriminantError2 {
    Zero,       // 0
    One,        // 1
    OneToo = 1, // 1 (collision with previous!)
}
}
```

[items.enum.discriminant.restrictions.above-max-discriminant](#r-items.enum.discriminant.restrictions.above-max-discriminant)

It is also an error to have an unspecified discriminant where the previous discriminant is the maximum value for the size of the discriminant.

``` playground
#![allow(unused)]
fn main() {
#[repr(u8)]
enum OverflowingDiscriminantError {
    Max = 255,
    MaxPlusOne, // Would be 256, but that overflows the enum.
}

#[repr(u8)]
enum OverflowingDiscriminantError2 {
    MaxMinusOne = 254, // 254
    Max,               // 255
    MaxPlusOne,        // Would be 256, but that overflows the enum.
}
}
```

[items.enum.discriminant.restrictions.generics](#r-items.enum.discriminant.restrictions.generics)

Explicit enum discriminant initializers may not use generic parameters from the enclosing enum.

``` playground
#![allow(unused)]
fn main() {
#[repr(u32)]
enum E<'a, T, const N: u32> {
    Lifetime(&'a T) = {
        let a: &'a (); // ERROR.
        1
    },
    Type(T) = {
        let x: T; // ERROR.
        2
    },
    Const = N, // ERROR.
}
}
```

### Accessing discriminant

#### Via `mem::discriminant`

[items.enum.discriminant.access-opaque](#r-items.enum.discriminant.access-opaque)

[`std::mem::discriminant`](https://doc.rust-lang.org/1.100.0/core/mem/fn.discriminant.html) returns an opaque reference to the discriminant of an enum value which can be compared. This cannot be used to get the value of the discriminant.

[items.enum.discriminant.coercion](#r-items.enum.discriminant.coercion)

#### Casting

[items.enum.discriminant.coercion.intro](#r-items.enum.discriminant.coercion.intro)

If an enumeration is [unit-only](#unit-only-enum) (with no tuple and struct variants), then its discriminant can be directly accessed with a [numeric cast](../expressions/operator-expr.md#semantics); e.g.:

``` playground
#![allow(unused)]
fn main() {
enum Enum {
    Foo,
    Bar,
    Baz,
}

assert_eq!(0, Enum::Foo as isize);
assert_eq!(1, Enum::Bar as isize);
assert_eq!(2, Enum::Baz as isize);
}
```

[items.enum.discriminant.coercion.fieldless](#r-items.enum.discriminant.coercion.fieldless)

[Field-less enums](#field-less-enum) can be cast if they do not have explicit discriminants, or where only unit variants are explicit.

``` playground
#![allow(unused)]
fn main() {
enum Fieldless {
    Tuple(),
    Struct{},
    Unit,
}

assert_eq!(0, Fieldless::Tuple() as isize);
assert_eq!(1, Fieldless::Struct{} as isize);
assert_eq!(2, Fieldless::Unit as isize);

#[repr(u8)]
enum FieldlessWithDiscriminants {
    First = 10,
    Tuple(),
    Second = 20,
    Struct{},
    Unit,
}

assert_eq!(10, FieldlessWithDiscriminants::First as u8);
assert_eq!(11, FieldlessWithDiscriminants::Tuple() as u8);
assert_eq!(20, FieldlessWithDiscriminants::Second as u8);
assert_eq!(21, FieldlessWithDiscriminants::Struct{} as u8);
assert_eq!(22, FieldlessWithDiscriminants::Unit as u8);
}
```

#### Pointer casting

[items.enum.discriminant.access-memory](#r-items.enum.discriminant.access-memory)

If the enumeration specifies a [primitive representation](../type-layout.md#primitive-representations), then the discriminant may be reliably accessed via unsafe pointer casting:

``` playground
#![allow(unused)]
fn main() {
#[repr(u8)]
enum Enum {
    Unit,
    Tuple(bool),
    Struct{a: bool},
}

impl Enum {
    fn discriminant(&self) -> u8 {
        unsafe { *(self as *const Self as *const u8) }
    }
}

let unit_like = Enum::Unit;
let tuple_like = Enum::Tuple(true);
let struct_like = Enum::Struct{a: false};

assert_eq!(0, unit_like.discriminant());
assert_eq!(1, tuple_like.discriminant());
assert_eq!(2, struct_like.discriminant());
}
```

[items.enum.empty](#r-items.enum.empty)

## Zero-variant enums

[items.enum.empty.intro](#r-items.enum.empty.intro)

Enums with zero variants are known as *zero-variant enums*. As they have no valid values, they cannot be instantiated.

``` playground
#![allow(unused)]
fn main() {
enum ZeroVariants {}
}
```

[items.enum.empty.uninhabited](#r-items.enum.empty.uninhabited)

Zero-variant enums are equivalent to the [never type](../types/never.md), but they cannot be coerced into other types.

``` playground
#![allow(unused)]
fn main() {
enum ZeroVariants {}
let x: ZeroVariants = panic!();
let y: u32 = x; // mismatched type error
}
```

[items.enum.variant-visibility](#r-items.enum.variant-visibility)

## Variant visibility

Enum variants syntactically allow a [Visibility](../visibility-and-privacy.md#grammar-Visibility) annotation, but this is rejected when the enum is validated. This allows items to be parsed with a unified syntax across different contexts where they are used.

``` playground
#![allow(unused)]
fn main() {
macro_rules! mac_variant {
    ($vis:vis $name:ident) => {
        enum $name {
            $vis Unit,

            $vis Tuple(u8, u16),

            $vis Struct { f: u8 },
        }
    }
}

// Empty `vis` is allowed.
mac_variant! { E }

// This is allowed, since it is removed before being validated.
#[cfg(false)]
enum E {
    pub U,
    pub(crate) T(u8),
    pub(super) T { f: String },
}
}
```
