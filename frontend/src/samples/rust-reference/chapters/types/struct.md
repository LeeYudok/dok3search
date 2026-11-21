# Struct types

[type.struct](#r-type.struct)

# Struct types

[type.struct.intro](#r-type.struct.intro)

A `struct` *type* is a heterogeneous product of other types, called the *fields* of the type.[1](#footnote-structtype)

[type.struct.constructor](#r-type.struct.constructor)

New instances of a `struct` can be constructed with a [struct expression](../expressions/struct-expr.md).

[type.struct.layout](#r-type.struct.layout)

The memory layout of a `struct` is undefined by default to allow for compiler optimizations like field reordering, but it can be fixed with the [`repr` attribute](../type-layout.md#representations). In either case, fields may be given in any order in a corresponding struct *expression*; the resulting `struct` value will always have the same memory layout.

[type.struct.field-visibility](#r-type.struct.field-visibility)

The fields of a `struct` may be qualified by [visibility modifiers](../visibility-and-privacy.md), to allow access to data in a struct outside a module.

[type.struct.tuple](#r-type.struct.tuple)\
Tests Tests with this rule:

- [tests/ui/tuple/tuple-index-out-of-bounds.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/tuple/tuple-index-out-of-bounds.rs)
- [tests/ui/tuple/tuple-index.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/tuple/tuple-index.rs)

A *tuple struct* type is just like a struct type, except that the fields are anonymous.

[type.struct.unit](#r-type.struct.unit)

A *unit-like struct* type is like a struct type, except that it has no fields. The one value constructed by the associated [struct expression](../expressions/struct-expr.md) is the only value that inhabits such a type.

------------------------------------------------------------------------

1.  

    `struct` types are analogous to `struct` types in C, the *record* types of the ML family, or the *struct* types of the Lisp family. [↩](#fr-structtype-1)
