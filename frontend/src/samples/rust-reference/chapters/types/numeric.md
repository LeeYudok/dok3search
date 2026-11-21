# Numeric types

[type.numeric](#r-type.numeric)

# Numeric types

[type.numeric.int](#r-type.numeric.int)

## Integer types

[type.numeric.int.unsigned](#r-type.numeric.int.unsigned)

The unsigned integer types consist of:

| Type   | Minimum | Maximum           |
|--------|---------|-------------------|
| `u8`   | 0       | 28-1   |
| `u16`  | 0       | 216-1  |
| `u32`  | 0       | 232-1  |
| `u64`  | 0       | 264-1  |
| `u128` | 0       | 2128-1 |

[type.numeric.int.signed](#r-type.numeric.int.signed)

The signed two’s complement integer types consist of:

| Type   | Minimum            | Maximum           |
|--------|--------------------|-------------------|
| `i8`   | -(27)   | 27-1   |
| `i16`  | -(215)  | 215-1  |
| `i32`  | -(231)  | 231-1  |
| `i64`  | -(263)  | 263-1  |
| `i128` | -(2127) | 2127-1 |

[type.numeric.float](#r-type.numeric.float)

## Floating-point types

The IEEE 754-2008 “binary32” and “binary64” floating-point types are `f32` and `f64`, respectively.

[type.numeric.int.size](#r-type.numeric.int.size)

## Machine-dependent integer types

[type.numeric.int.size.usize](#r-type.numeric.int.size.usize)

The `usize` type is an unsigned integer type with the same number of bits as the platform’s pointer type. It can represent every memory address in the process.

> ![](../../assets/inline/numeric-1.svg)Note
>
> While a `usize` can represent every *address*, converting a *pointer* to a `usize` is not necessarily a reversible operation. For more information, see the documentation for [type cast expressions](../expressions/operator-expr.md#type-cast-expressions), [`std::ptr`](https://doc.rust-lang.org/1.100.0/core/ptr/index.html), and [provenance](https://doc.rust-lang.org/1.100.0/core/ptr/index.html#provenance) in particular.

[type.numeric.int.size.isize](#r-type.numeric.int.size.isize)

The `isize` type is a signed two’s complement integer type with the same number of bits as the platform’s pointer type. The theoretical upper bound on object and array size is the maximum `isize` value. This ensures that `isize` can be used to calculate differences between pointers into an object or array and can address every byte within an object along with one byte past the end.

[type.numeric.int.size.minimum](#r-type.numeric.int.size.minimum)

`usize` and `isize` are at least 16-bits wide.

> ![](../../assets/inline/numeric-2.svg)Note
>
> Many pieces of Rust code may assume that pointers, `usize`, and `isize` are either 32-bit or 64-bit. As a consequence, 16-bit pointer support is limited and may require explicit care and acknowledgment from a library to support.

[type.numeric.validity](#r-type.numeric.validity)

## Bit validity

For every numeric type, `T`, the bit validity of `T` is equivalent to the bit validity of `[u8; size_of::<T>()]`. An uninitialized byte is not a valid `u8`.
