# Tuple and tuple indexing expressions

[expr.tuple](#r-expr.tuple)

# Tuple and tuple indexing expressions

## Tuple expressions

[expr.tuple.syntax](#r-expr.tuple.syntax)

**Syntax**\
[TupleExpression](#railroad-TupleExpression) → ( [TupleElements](#grammar-TupleElements)? )

[TupleElements](#railroad-TupleElements) → ( [Expression](../expressions.md#grammar-Expression) , )+ [Expression](../expressions.md#grammar-Expression)?

Show Railroad

![](../../assets/inline/tuple-expr-1.svg) ![](../../assets/inline/tuple-expr-2.svg) [expr.tuple.result](#r-expr.tuple.result)

A *tuple expression* constructs [tuple values](../types/tuple.md).

[expr.tuple.intro](#r-expr.tuple.intro)

The syntax for tuple expressions is a parenthesized, comma separated list of expressions, called the *tuple initializer operands*.

[expr.tuple.unary-tuple-restriction](#r-expr.tuple.unary-tuple-restriction)\
Tests Tests with this rule:

- [tests/ui/tuple/add-tuple-within-arguments.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/tuple/add-tuple-within-arguments.rs)
- [tests/ui/tuple/one-tuple.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/tuple/one-tuple.rs)

1-ary tuple expressions require a comma after their tuple initializer operand to be disambiguated with a [parenthetical expression](grouped-expr.md).

[expr.tuple.value](#r-expr.tuple.value)

Tuple expressions are a [value expression](../expressions.md#place-expressions-and-value-expressions) that evaluate into a newly constructed value of a tuple type.

[expr.tuple.type](#r-expr.tuple.type)\
Tests Tests with this rule:

- [tests/ui/tuple/tuple-arity-mismatch.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/tuple/tuple-arity-mismatch.rs)

The number of tuple initializer operands is the arity of the constructed tuple.

[expr.tuple.unit](#r-expr.tuple.unit)

Tuple expressions without any tuple initializer operands produce the unit tuple.

[expr.tuple.fields](#r-expr.tuple.fields)\
Tests Tests with this rule:

- [tests/ui/tuple/tuple-index.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/tuple/tuple-index.rs)

For other tuple expressions, the first written tuple initializer operand initializes the field `0` and subsequent operands initializes the next highest field. For example, in the tuple expression `('a', 'b', 'c')`, `'a'` initializes the value of the field `0`, `'b'` field `1`, and `'c'` field `2`.

Examples of tuple expressions and their types:

| Expression            | Type                          |
|-----------------------|-------------------------------|
| `()`                  | `()` (unit)                   |
| `(0.0, 4.5)`          | `(f64, f64)`                  |
| `("x".to_string(), )` | `(String, )`                  |
| `("a", 4usize, true)` | `(&'static str, usize, bool)` |

[expr.tuple-index](#r-expr.tuple-index)

## Tuple indexing expressions

[expr.tuple-index.syntax](#r-expr.tuple-index.syntax)\
Tests Tests with this rule:

- [tests/ui/tuple/index-float.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/tuple/index-float.rs)

**Syntax**\
[TupleIndexingExpression](#railroad-TupleIndexingExpression) → [Expression](../expressions.md#grammar-Expression) . [TUPLE_INDEX](../tokens.md#grammar-TUPLE_INDEX)

Show Railroad

![](../../assets/inline/tuple-expr-3.svg) [expr.tuple-index.intro](#r-expr.tuple-index.intro)

A *tuple indexing expression* accesses fields of [tuples](../types/tuple.md) and [tuple structs](../types/struct.md).

The syntax for a tuple index expression is an expression, called the *tuple operand*, then a `.`, then finally a tuple index.

[expr.tuple-index.index-syntax](#r-expr.tuple-index.index-syntax)\
Tests Tests with this rule:

- [tests/ui/tuple/index-invalid.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/tuple/index-invalid.rs)

The syntax for the *tuple index* is a [decimal literal](../tokens.md#integer-literals) with no leading zeros, underscores, or suffix. For example `0` and `2` are valid tuple indices but not `01`, `0_`, nor `0i32`.

[expr.tuple-index.required-type](#r-expr.tuple-index.required-type)\
Tests Tests with this rule:

- [tests/ui/tuple/tuple-index-not-tuple.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/tuple/tuple-index-not-tuple.rs)

The type of the tuple operand must be a [tuple type](../types/tuple.md) or a [tuple struct](../types/struct.md).

[expr.tuple-index.index-name-operand](#r-expr.tuple-index.index-name-operand)\
Tests Tests with this rule:

- [tests/ui/tuple/index-invalid.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/tuple/index-invalid.rs)
- [tests/ui/tuple/tuple-index-out-of-bounds.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/tuple/tuple-index-out-of-bounds.rs)

The tuple index must be a name of a field of the type of the tuple operand.

[expr.tuple-index.result](#r-expr.tuple-index.result)\
Tests Tests with this rule:

- [tests/ui/tuple/tuple-index.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/tuple/tuple-index.rs)

Evaluation of tuple index expressions has no side effects beyond evaluation of its tuple operand. As a [place expression](../expressions.md#place-expressions-and-value-expressions), it evaluates to the location of the field of the tuple operand with the same name as the tuple index.

Examples of tuple indexing expressions:

``` playground
#![allow(unused)]
fn main() {
// Indexing a tuple
let pair = ("a string", 2);
assert_eq!(pair.1, 2);

// Indexing a tuple struct
struct Point(f32, f32);
let point = Point(1.0, 0.0);
assert_eq!(point.0, 1.0);
assert_eq!(point.1, 0.0);
}
```

> ![](../../assets/inline/tuple-expr-4.svg)Note
>
> Unlike field access expressions, tuple index expressions can be the function operand of a [call expression](call-expr.md) as it cannot be confused with a method call since method names cannot be numbers.

> ![](../../assets/inline/tuple-expr-5.svg)Note
>
> Although arrays and slices also have elements, you must use an [array or slice indexing expression](array-expr.md#array-and-slice-indexing-expressions) or a [slice pattern](../patterns.md#slice-patterns) to access their elements.
