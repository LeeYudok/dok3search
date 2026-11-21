# Array types

[type.array](#r-type.array)

# Array types

[type.array.syntax](#r-type.array.syntax)

**Syntax**\
[ArrayType](#railroad-ArrayType) → \[ [Type](../types.md#grammar-Type) ; [Expression](../expressions.md#grammar-Expression) \]

Show Railroad

![](../../assets/inline/array-1.svg) [type.array.intro](#r-type.array.intro)

An array is a fixed-size sequence of `N` elements of type `T`. The array type is written as `[T; N]`.

[type.array.constraint](#r-type.array.constraint)

The size is a [constant expression](../const_eval.md#constant-expressions) that evaluates to a [`usize`](numeric.md#machine-dependent-integer-types).

Examples:

``` playground
#![allow(unused)]
fn main() {
// A stack-allocated array
let array: [i32; 3] = [1, 2, 3];

// A heap-allocated array, coerced to a slice
let boxed_array: Box<[i32]> = Box::new([1, 2, 3]);
}
```

[type.array.index](#r-type.array.index)

All elements of arrays are always initialized, and access to an array is always bounds-checked in safe methods and operators.

> ![](../../assets/inline/array-2.svg)Note
>
> The [`Vec<T>`](../../alloc/vec/struct.Vec.html) standard library type provides a heap-allocated resizable array type.
