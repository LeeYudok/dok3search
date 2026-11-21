# Struct expressions

[expr.struct](#r-expr.struct)

# Struct expressions

[expr.struct.syntax](#r-expr.struct.syntax)

**Syntax**\
[StructExpression](#railroad-StructExpression) →\
    [PathInExpression](../paths.md#grammar-PathInExpression) { ( [StructExprFields](#grammar-StructExprFields) \| [StructBase](#grammar-StructBase) )? }

[StructExprFields](#railroad-StructExprFields) →\
    [StructExprField](#grammar-StructExprField) ( , [StructExprField](#grammar-StructExprField) )\* ( , [StructBase](#grammar-StructBase) \| ,? )

[StructExprField](#railroad-StructExprField) →\
    [OuterAttribute](../attributes.md#grammar-OuterAttribute)\*\
    (\
        [IDENTIFIER](../identifiers.md#grammar-IDENTIFIER)\
      \| ( [IDENTIFIER](../identifiers.md#grammar-IDENTIFIER) \| [TUPLE_INDEX](../tokens.md#grammar-TUPLE_INDEX) ) : [Expression](../expressions.md#grammar-Expression)\
    )

[StructBase](#railroad-StructBase) → .. [Expression](../expressions.md#grammar-Expression)

Show Railroad

![](../../assets/inline/struct-expr-1.svg) ![](../../assets/inline/struct-expr-2.svg) ![](../../assets/inline/struct-expr-3.svg) ![](../../assets/inline/struct-expr-4.svg) [expr.struct.intro](#r-expr.struct.intro)

A *struct expression* creates a struct, enum, or union value. It consists of a path to a [struct](../items/structs.md), [enum variant](../items/enumerations.md), or [union](../items/unions.md) item followed by the values for the fields of the item.

The following are examples of struct expressions:

``` playground
#![allow(unused)]
fn main() {
struct Point { x: f64, y: f64 }
struct NothingInMe { }
mod game { pub struct User<'a> { pub name: &'a str, pub age: u32, pub score: usize } }
enum Enum { Variant {} }
Point {x: 10.0, y: 20.0};
NothingInMe {};
let u = game::User {name: "Joe", age: 35, score: 100_000};
Enum::Variant {};
}
```

> ![](../../assets/inline/struct-expr-5.svg)Note
>
> Tuple structs and tuple enum variants are typically instantiated using a [call expression](call-expr.md#r-expr.call) referring to the [constructor in the value namespace](../items/structs.md#r-items.struct.tuple). These are distinct from a struct expression using curly braces referring to the constructor in the type namespace.
>
> ``` playground
> #![allow(unused)]
> fn main() {
> struct Position(i32, i32, i32);
> Position(0, 0, 0);  // Typical way of creating a tuple struct.
> let c = Position;  // `c` is a function that takes 3 arguments.
> let pos = c(8, 6, 7);  // Creates a `Position` value.
>
> enum Version { Triple(i32, i32, i32) };
> Version::Triple(0, 0, 0);
> let f = Version::Triple;
> let ver = f(8, 6, 7);
> }
> ```
>
> The last segment of the call path cannot refer to a type alias:
>
> ``` playground
> #![allow(unused)]
> fn main() {
> trait Tr { type T; }
> impl Tr for T { type T = T; }
>
> struct Tuple();
> enum Enum { Tuple() }
>
> // ::T(); // causes an error -- `::T` is a type, not a value
> ::T::Tuple(); // OK
> }
> ```
>
> ------------------------------------------------------------------------
>
> Unit structs and unit enum variants are typically instantiated using a [path expression](path-expr.md#r-expr.path) referring to the [constant in the value namespace](../items/structs.md#r-items.struct.unit).
>
> ``` playground
> #![allow(unused)]
> fn main() {
> struct Gamma;
> // Gamma unit value, referring to the const in the value namespace.
> let a = Gamma;
> // Exact same value as `a`, but constructed using a struct expression
> // referring to the type namespace.
> let b = Gamma {};
>
> enum ColorSpace { Oklch }
> let c = ColorSpace::Oklch;
> let d = ColorSpace::Oklch {};
> }
> ```

[expr.struct.field](#r-expr.struct.field)

## Field struct expression

[expr.struct.field.intro](#r-expr.struct.field.intro)

A struct expression with fields enclosed in curly braces allows you to specify the value for each individual field in any order. The field name is separated from its value with a colon.

[expr.struct.field.union-constraint](#r-expr.struct.field.union-constraint)\
Tests Tests with this rule:

- [tests/ui/union/union-fields-2.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/union-fields-2.rs)

A value of a [union](../items/unions.md) type can only be created using this syntax, and it must specify exactly one field.

[expr.struct.update](#r-expr.struct.update)

## Functional update syntax

[expr.struct.update.intro](#r-expr.struct.update.intro)

A struct expression that constructs a value of a struct type can terminate with the syntax `..` followed by an expression to denote a functional update.

[expr.struct.update.base-same-type](#r-expr.struct.update.base-same-type)

The expression following `..` (the base) must have the same struct type as the new struct type being formed.

[expr.struct.update.fields](#r-expr.struct.update.fields)

The entire expression uses the given values for the fields that were specified and moves or copies the remaining fields from the base expression.

[expr.struct.update.visibility-constraint](#r-expr.struct.update.visibility-constraint)

As with all struct expressions, all of the fields of the struct must be [visible](../visibility-and-privacy.md), even those not explicitly named.

``` playground
#![allow(unused)]
fn main() {
struct Point3d { x: i32, y: i32, z: i32 }
let mut base = Point3d {x: 1, y: 2, z: 3};
let y_ref = &mut base.y;
Point3d {y: 0, z: 10, .. base}; // OK, only base.x is accessed
drop(y_ref);
}
```

[expr.struct.brace-restricted-positions](#r-expr.struct.brace-restricted-positions)

Struct expressions can’t be used directly in a [loop](loop-expr.md) or [if](if-expr.md#if-expressions) expression’s head, or in the [scrutinee](../glossary.md#scrutinee) of an [if let](if-expr.md#if-let-patterns) or [match](match-expr.md) expression. However, struct expressions can be used in these situations if they are within another expression, for example inside [parentheses](grouped-expr.md).

[expr.struct.tuple-field](#r-expr.struct.tuple-field)

The field names can be decimal integer values to specify indices for constructing tuple structs. This can be used with base structs to fill out the remaining indices not specified:

``` playground
#![allow(unused)]
fn main() {
struct Color(u8, u8, u8);
let c1 = Color(0, 0, 0);  // Typical way of creating a tuple struct.
let c2 = Color{0: 255, 1: 127, 2: 0};  // Specifying fields by index.
let c3 = Color{1: 0, ..c2};  // Fill out all other fields using a base struct.
}
```

[expr.struct.field.named](#r-expr.struct.field.named)

### Struct field init shorthand

When initializing a data structure (struct, enum, union) with named (but not numbered) fields, it is allowed to write `fieldname` as a shorthand for `fieldname: fieldname`. This allows a compact syntax with less duplication. For example:

``` playground
#![allow(unused)]
fn main() {
struct Point3d { x: i32, y: i32, z: i32 }
let x = 0;
let y_value = 0;
let z = 0;
Point3d { x: x, y: y_value, z: z };
Point3d { x, y: y_value, z };
}
```
