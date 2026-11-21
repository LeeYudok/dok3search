# Types

[type](#r-type)

# Types

[type.intro](#r-type.intro)

Every variable, item, and value in a Rust program has a type. The *type* of a *value* defines the interpretation of the memory holding it and the operations that may be performed on the value.

[type.builtin](#r-type.builtin)

Built-in types are tightly integrated into the language, in nontrivial ways that are not possible to emulate in user-defined types.

[type.user-defined](#r-type.user-defined)

User-defined types have limited capabilities.

[type.kinds](#r-type.kinds)

The list of types is:

- Primitive types:
  - [Boolean](types/boolean.md) — `bool`
  - [Numeric](types/numeric.md) — integer and float
  - [`char`](types/char.md)
  - [`str`](types/str.md)
  - [Never](types/never.md) — `!` — a type with no values
- Sequence types:
  - [Tuple](types/tuple.md)
  - [Array](types/array.md)
  - [Slice](types/slice.md)
- User-defined types:
  - [Struct](types/struct.md)
  - [Enum](types/enum.md)
  - [Union](types/union.md)
- Function types:
  - [Functions](types/function-item.md)
  - [Closures](types/closure.md)
- Pointer types:
  - [References](types/pointer.md#shared-references-)
  - [Raw pointers](types/pointer.md#raw-pointers-const-and-mut)
  - [Function pointers](types/function-pointer.md)
- Trait types:
  - [Trait objects](types/trait-object.md)
  - [Impl trait](types/impl-trait.md)

[type.name](#r-type.name)

## Type expressions

[type.name.syntax](#r-type.name.syntax)

**Syntax**\
[Type](#railroad-Type) →\
      [TypeNoBounds](#grammar-TypeNoBounds)\
    \| [ImplTraitType](types/impl-trait.md#grammar-ImplTraitType)\
    \| [TraitObjectType](types/trait-object.md#grammar-TraitObjectType)

[TypeNoBounds](#railroad-TypeNoBounds) →\
      [ParenthesizedType](#grammar-ParenthesizedType)\
    \| [ImplTraitTypeOneBound](types/impl-trait.md#grammar-ImplTraitTypeOneBound)\
    \| [TraitObjectTypeOneBound](types/trait-object.md#grammar-TraitObjectTypeOneBound)\
    \| [TypePath](paths.md#grammar-TypePath)\
    \| [TupleType](types/tuple.md#grammar-TupleType)\
    \| [NeverType](types/never.md#grammar-NeverType)\
    \| [RawPointerType](types/pointer.md#grammar-RawPointerType)\
    \| [ReferenceType](types/pointer.md#grammar-ReferenceType)\
    \| [ArrayType](types/array.md#grammar-ArrayType)\
    \| [SliceType](types/slice.md#grammar-SliceType)\
    \| [InferredType](types/inferred.md#grammar-InferredType)\
    \| [QualifiedPathInType](paths.md#grammar-QualifiedPathInType)\
    \| [BareFunctionType](types/function-pointer.md#grammar-BareFunctionType)\
    \| [MacroInvocation](macros.md#grammar-MacroInvocation)

Show Railroad

![](../assets/inline/types-1.svg) ![](../assets/inline/types-2.svg) [type.name.intro](#r-type.name.intro)

A *type expression* as defined in the [Type](#r-type) grammar rule above is the syntax for referring to a type. It may refer to:

[type.name.sequence](#r-type.name.sequence)

- Sequence types ([tuple](types/tuple.md), [array](types/array.md), [slice](types/slice.md)).

[type.name.path](#r-type.name.path)

- [Type paths](paths.md#paths-in-types) which can reference:
  - Primitive types ([boolean](types/boolean.md), [numeric](types/numeric.md), [`char`](types/char.md), [`str`](types/str.md)).
  - Paths to an [item](items.md) ([struct](types/struct.md), [enum](types/enum.md), [union](types/union.md), [type alias](items/type-aliases.md), [trait](types/trait-object.md)).
  - [`Self` path](paths.md#self-1) where `Self` is the implementing type.
  - Generic [type parameters](types/parameters.md).

[type.name.pointer](#r-type.name.pointer)

- Pointer types ([reference](types/pointer.md#shared-references-), [raw pointer](types/pointer.md#raw-pointers-const-and-mut), [function pointer](types/function-pointer.md)).

[type.name.inference](#r-type.name.inference)

- The [inferred type](types/inferred.md) which asks the compiler to determine the type.

[type.name.grouped](#r-type.name.grouped)

- [Parentheses](#parenthesized-types) which are used for disambiguation.

[type.name.trait](#r-type.name.trait)

- Trait types: [Trait objects](types/trait-object.md) and [impl trait](types/impl-trait.md).

[type.name.never](#r-type.name.never)

- The [never](types/never.md) type.

[type.name.macro-expansion](#r-type.name.macro-expansion)

- [Macros](macros.md) which expand to a type expression.

[type.name.parenthesized](#r-type.name.parenthesized)

### Parenthesized types

[type.name.parenthesized.syntax](#r-type.name.parenthesized.syntax)

**Syntax**\
[ParenthesizedType](#railroad-ParenthesizedType) → ( [Type](#grammar-Type) )

Show Railroad

![](../assets/inline/types-3.svg) [type.name.parenthesized.intro](#r-type.name.parenthesized.intro)

In some situations the combination of types may be ambiguous. Use parentheses around a type to avoid ambiguity. For example, the `+` operator for [type boundaries](trait-bounds.md) within a [reference type](types/pointer.md#shared-references-) is unclear where the boundary applies, so the use of parentheses is required. Grammar rules that require this disambiguation use the [TypeNoBounds](#grammar-TypeNoBounds) rule instead of [Type](#grammar-Type).

``` playground
#![allow(unused)]
fn main() {
use std::any::Any;
type T<'a> = &'a (dyn Any + Send);
}
```

[type.recursive](#r-type.recursive)

## Recursive types

[type.recursive.intro](#r-type.recursive.intro)

Nominal types — [structs](types/struct.md), [enumerations](types/enum.md), and [unions](types/union.md) — may be recursive. That is, each `enum` variant or `struct` or `union` field may refer, directly or indirectly, to the enclosing `enum` or `struct` type itself.

[type.recursive.constraint](#r-type.recursive.constraint)

Such recursion has restrictions:

- Recursive types must include a nominal type in the recursion (not mere [type aliases](items/type-aliases.md), or other structural types such as [arrays](types/array.md) or [tuples](types/tuple.md)). So `type Rec = &'static [Rec]` is not allowed.
- The size of a recursive type must be finite; in other words the recursive fields of the type must be [pointer types](types/pointer.md).

An example of a *recursive* type and its use:

``` playground
#![allow(unused)]
fn main() {
enum List<T> {
    Nil,
    Cons(T, Box<List<T>>)
}

let a: List<i32> = List::Cons(7, Box::new(List::Cons(13, Box::new(List::Nil))));
}
```
