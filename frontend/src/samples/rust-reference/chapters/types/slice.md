# Slice types

[type.slice](#r-type.slice)

# Slice types

[type.slice.syntax](#r-type.slice.syntax)

**Syntax**\
[SliceType](#railroad-SliceType) → \[ [Type](../types.md#grammar-Type) \]

Show Railroad

![](../../assets/inline/slice-1.svg) [type.slice.intro](#r-type.slice.intro)

A slice is a [dynamically sized type](../dynamically-sized-types.md) representing a ‘view’ into a sequence of elements of type `T`. The slice type is written as `[T]`.

[type.slice.unsized](#r-type.slice.unsized)

Slice types are generally used through pointer types. For example:

- `&[T]`: a ‘shared slice’, often just called a ‘slice’. It doesn’t own the data it points to; it borrows it.
- `&mut [T]`: a ‘mutable slice’. It mutably borrows the data it points to.
- `Box<[T]>`: a ‘boxed slice’

Examples:

``` playground
#![allow(unused)]
fn main() {
// A heap-allocated array, coerced to a slice
let boxed_array: Box<[i32]> = Box::new([1, 2, 3]);

// A (shared) slice into an array
let slice: &[i32] = &boxed_array[..];
}
```

[type.slice.safe](#r-type.slice.safe)

All elements of slices are always initialized, and access to a slice is always bounds-checked in safe methods and operators.
