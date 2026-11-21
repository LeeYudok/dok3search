# Tuple types

[type.tuple](#r-type.tuple)

# Tuple types

[type.tuple.syntax](#r-type.tuple.syntax)\
Tests Tests with this rule:

- [tests/ui/tuple/add-tuple-within-arguments.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/tuple/add-tuple-within-arguments.rs)

**Syntax**\
[TupleType](#railroad-TupleType) →\
      ( )\
    \| ( ( [Type](../types.md#grammar-Type) , )+ [Type](../types.md#grammar-Type)? )

Show Railroad

![](../../assets/inline/tuple-1.svg) [type.tuple.intro](#r-type.tuple.intro)

*Tuple types* are a family of structural types[^1] for heterogeneous lists of other types.

The syntax for a tuple type is a parenthesized, comma-separated list of types.

[type.tuple.restriction](#r-type.tuple.restriction)\
Tests Tests with this rule:

- [tests/ui/tuple/add-tuple-within-arguments.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/tuple/add-tuple-within-arguments.rs)
- [tests/ui/tuple/one-tuple.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/tuple/one-tuple.rs)

1-ary tuples require a comma after their element type to be disambiguated with a [parenthesized type](../types.md#parenthesized-types).

[type.tuple.field-number](#r-type.tuple.field-number)\
Tests Tests with this rule:

- [tests/ui/tuple/tuple-arity-mismatch.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/tuple/tuple-arity-mismatch.rs)
- [tests/ui/tuple/tuple-index-out-of-bounds.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/tuple/tuple-index-out-of-bounds.rs)

A tuple type has a number of fields equal to the length of the list of types. This number of fields determines the *arity* of the tuple. A tuple with `n` fields is called an *n-ary tuple*. For example, a tuple with 2 fields is a 2-ary tuple.

[type.tuple.field-name](#r-type.tuple.field-name)\
Tests Tests with this rule:

- [tests/ui/tuple/index-invalid.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/tuple/index-invalid.rs)
- [tests/ui/tuple/tuple-index-out-of-bounds.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/tuple/tuple-index-out-of-bounds.rs)
- [tests/ui/tuple/tuple-index.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/tuple/tuple-index.rs)

Fields of tuples are named using increasing numeric names matching their position in the list of types. The first field is `0`. The second field is `1`. And so on. The type of each field is the type of the same position in the tuple’s list of types.

[type.tuple.unit](#r-type.tuple.unit)

For convenience and historical reasons, the tuple type with no fields (`()`) is often called *unit* or *the unit type*. Its one value is also called *unit* or *the unit value*.

Some examples of tuple types:

- `()` (unit)
- `(i32,)` (1-ary tuple)
- `(f64, f64)`
- `(String, i32)`
- `(i32, String)` (different type from the previous example)
- `(i32, f64, Vec<String>, Option<bool>)`

[type.tuple.constructor](#r-type.tuple.constructor)\
Tests Tests with this rule:

- [tests/ui/tuple/tup.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/tuple/tup.rs)

Values of this type are constructed using a [tuple expression](../expressions/tuple-expr.md#tuple-expressions). Furthermore, various expressions will produce the unit value if there is no other meaningful value for it to evaluate to.

[type.tuple.access](#r-type.tuple.access)\
Tests Tests with this rule:

- [tests/ui/tuple/tuple-index.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/tuple/tuple-index.rs)

Tuple fields can be accessed by either a [tuple index expression](../expressions/tuple-expr.md#tuple-indexing-expressions) or [pattern matching](../patterns.md#tuple-patterns).

------------------------------------------------------------------------

[^1]: 

    Structural types are always equivalent if their internal types are equivalent. For a nominal version of tuples, see [tuple structs](struct.md). [↩](#fr-1-1)
