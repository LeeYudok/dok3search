# Array and array index expressions

[expr.array](#r-expr.array)

# Array and array index expressions

## Array expressions

[expr.array.syntax](#r-expr.array.syntax)

**Syntax**\
[ArrayExpression](#railroad-ArrayExpression) → \[ [ArrayElements](#grammar-ArrayElements)? \]

[ArrayElements](#railroad-ArrayElements) →\
      [Expression](../expressions.md#grammar-Expression) ( , [Expression](../expressions.md#grammar-Expression) )\* ,?\
    \| [Expression](../expressions.md#grammar-Expression) ; [Expression](../expressions.md#grammar-Expression)

Show Railroad

![](../../assets/inline/array-expr-1.svg) ![](../../assets/inline/array-expr-2.svg) [expr.array.constructor](#r-expr.array.constructor)

*Array expressions* construct [arrays](../types/array.md). Array expressions come in two forms.

[expr.array.array](#r-expr.array.array)

The first form lists out every value in the array.

[expr.array.array-syntax](#r-expr.array.array-syntax)

The syntax for this form is a comma-separated list of expressions of uniform type enclosed in square brackets.

[expr.array.array-behavior](#r-expr.array.array-behavior)

This produces an array containing each of these values in the order they are written.

[expr.array.repeat](#r-expr.array.repeat)

The syntax for the second form is two expressions separated by a semicolon (`;`) enclosed in square brackets.

[expr.array.repeat-operand](#r-expr.array.repeat-operand)

The expression before the `;` is called the *repeat operand*.

[expr.array.length-operand](#r-expr.array.length-operand)

The expression after the `;` is called the *length operand*.

[expr.array.length-restriction](#r-expr.array.length-restriction)

The length operand must either be an [inferred const](../items/generics.md#r-items.generics.const.inferred) or be a [constant expression](../const_eval.md#constant-expressions) of type `usize` (e.g. a [literal](../tokens.md#literals) or a [constant item](../items/constant-items.md)).

``` playground
#![allow(unused)]
fn main() {
const C: usize = 1;
let _: [u8; C] = [0; 1]; // Literal.
let _: [u8; C] = [0; C]; // Constant item.
let _: [u8; C] = [0; _]; // Inferred const.
let _: [u8; C] = [0; (((_)))]; // Inferred const.
}
```

> ![](../../assets/inline/array-expr-3.svg)Note
>
> In an array expression, an [inferred const](../items/generics.md#r-items.generics.const.inferred) is parsed as an [expression](../expressions.md#grammar-Expression) but then semantically treated as a separate kind of [const generic argument](../items/generics.md#r-items.generics.const.argument).

[expr.array.repeat-behavior](#r-expr.array.repeat-behavior)

An array expression of this form creates an array with the length of the value of the length operand with each element being a copy of the repeat operand. That is, `[a; b]` creates an array containing `b` copies of the value of `a`.

[expr.array.repeat-copy](#r-expr.array.repeat-copy)

If the length operand has a value greater than 1 then this requires the repeat operand to have a type that implements [`Copy`](../special-types-and-traits.md#copy), to be a [const block expression](block-expr.md#r-expr.block.const), or to be a [path](path-expr.md) to a constant item.

[expr.array.repeat-const-item](#r-expr.array.repeat-const-item)

When the repeat operand is a const block or a path to a constant item, it is evaluated the number of times specified in the length operand.

[expr.array.repeat-evaluation-zero](#r-expr.array.repeat-evaluation-zero)

If that value is `0`, then the const block or constant item is not evaluated at all.

[expr.array.repeat-non-const](#r-expr.array.repeat-non-const)

For expressions that are neither a const block nor a path to a constant item, it is evaluated exactly once, and then the result is copied the length operand’s value times.

``` playground
#![allow(unused)]
fn main() {
[1, 2, 3, 4];
["a", "b", "c", "d"];
[0; 128];              // array with 128 zeros
[0u8, 0u8, 0u8, 0u8,];
[[1, 0, 0], [0, 1, 0], [0, 0, 1]]; // 2D array
const EMPTY: Vec<i32> = Vec::new();
[EMPTY; 2];
}
```

[expr.array.index](#r-expr.array.index)

## Array and slice indexing expressions

[expr.array.index.syntax](#r-expr.array.index.syntax)

**Syntax**\
[IndexExpression](#railroad-IndexExpression) → [Expression](../expressions.md#grammar-Expression) \[ [Expression](../expressions.md#grammar-Expression) \]

Show Railroad

![](../../assets/inline/array-expr-4.svg) [expr.array.index.array](#r-expr.array.index.array)

[Array](../types/array.md) and [slice](../types/slice.md)-typed values can be indexed by writing a square-bracket-enclosed expression of type `usize` (the index) after them. When the array is mutable, the resulting [memory location](../expressions.md#place-expressions-and-value-expressions) can be assigned to.

[expr.array.index.trait](#r-expr.array.index.trait)\
Tests Tests with this rule:

- [tests/ui/binop/operator-overloading.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/binop/operator-overloading.rs)

For other types an index expression `a[b]` is equivalent to `*std::ops::Index::index(&a, b)`, or `*std::ops::IndexMut::index_mut(&mut a, b)` in a mutable place expression context, except that when the index expression undergoes [temporary lifetime extension](../destructors.md#r-destructors.scope.lifetime-extension), the indexed expression `a` also has its [temporary scope](../destructors.md#r-destructors.scope.temporary) extended. Just as with methods, Rust will also insert dereference operations on `a` repeatedly to find an implementation.

``` playground
#![allow(unused)]
fn main() {
// The temporary holding the result of `vec![()]` is extended to
// live to the end of the block, so `x` may be used in subsequent
// statements.
let x = &vec![()][0];
x;
}
```

``` playground
#![allow(unused)]
fn main() {
// The temporary holding the result of `vec![()]` is dropped at the
// end of the statement, so it's an error to use `y` after.
let y = &*std::ops::Index::index(&vec![()], 0); // ERROR
y;
}
```

[expr.array.index.zero-index](#r-expr.array.index.zero-index)

Indices are zero-based for arrays and slices.

[expr.array.index.const](#r-expr.array.index.const)

Array access is a [constant expression](../const_eval.md#constant-expressions), so bounds can be checked at compile-time with a constant index value. Otherwise a check will be performed at run-time that will put the thread in a [*panicked state*](../panic.md) if it fails.

``` playground
#![allow(unused)]
fn main() {
// lint is deny by default.
#![warn(unconditional_panic)]

([1, 2, 3, 4])[2];        // Evaluates to 3

let b = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
b[1][2];                  // multidimensional array indexing

let x = (["a", "b"])[10]; // warning: index out of bounds

let n = 10;
let y = (["a", "b"])[n];  // panics

let arr = ["a", "b"];
arr[10];                  // warning: index out of bounds
}
```

[expr.array.index.trait-impl](#r-expr.array.index.trait-impl)

The array index expression can be implemented for types other than arrays and slices by implementing the [Index](https://doc.rust-lang.org/1.100.0/core/ops/index/trait.Index.html) and [IndexMut](https://doc.rust-lang.org/1.100.0/core/ops/index/trait.IndexMut.html) traits.
