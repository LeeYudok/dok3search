# Boolean type

[type.bool](#r-type.bool)

# Boolean type

[type.bool.intro](#r-type.bool.intro)

``` playground
#![allow(unused)]
fn main() {
let b: bool = true;
}
```

The *boolean type* or *bool* is a primitive data type that can take on one of two values, called *true* and *false*.

[type.bool.literal](#r-type.bool.literal)

Values of this type may be created using a [literal expression](../expressions/literal-expr.md) using the keywords `true` and `false` corresponding to the value of the same name.

[type.bool.namespace](#r-type.bool.namespace)

This type is a part of the [language prelude](../names/preludes.md#language-prelude) with the [name](../names.md) `bool`.

[type.bool.layout](#r-type.bool.layout)

An object with the boolean type has a [size and alignment](../type-layout.md#size-and-alignment) of 1 each.

[type.bool.repr](#r-type.bool.repr)

The value false has the bit pattern `0x00` and the value true has the bit pattern `0x01`. It is [undefined behavior](../behavior-considered-undefined.md) for an object with the boolean type to have any other bit pattern.

[type.bool.use](#r-type.bool.use)

The boolean type is the type of many operands in various [expressions](../expressions.md):

[type.bool.use-in-condition](#r-type.bool.use-in-condition)

- The condition operand in [if expressions](../expressions/if-expr.md#if-expressions) and [while expressions](../expressions/loop-expr.md#predicate-loops)

[type.bool.use-in-lazy-operator](#r-type.bool.use-in-lazy-operator)

- The operands in [lazy boolean operator expressions](../expressions/operator-expr.md#lazy-boolean-operators)

> ![](../../assets/inline/boolean-1.svg)Note
>
> The boolean type acts similarly to but is not an [enumerated type](enum.md). In practice, this mostly means that constructors are not associated to the type (e.g. `bool::true`).

[type.bool.traits](#r-type.bool.traits)

Like all primitives, the boolean type [implements](../items/implementations.md) the [traits](../items/traits.md) [`Clone`](../special-types-and-traits.md#clone), [`Copy`](../special-types-and-traits.md#copy), [`Sized`](../special-types-and-traits.md#sized), [`Send`](../special-types-and-traits.md#send), and [`Sync`](../special-types-and-traits.md#sync).

> ![](../../assets/inline/boolean-2.svg)Note
>
> See the [standard library docs](https://doc.rust-lang.org/1.100.0/std/primitive.bool.html) for library operations.

[type.bool.expr](#r-type.bool.expr)

## Operations on boolean values

[type.bool.expr.intro](#r-type.bool.expr.intro)

When using certain operator expressions with a boolean type for its operands, they evaluate using the rules of [boolean logic](https://en.wikipedia.org/wiki/Boolean_algebra).

[type.bool.expr.not](#r-type.bool.expr.not)

### Logical not

| `b`     | [`!b`](../expressions/operator-expr.md#negation-operators) |
|---------|------------------------------------------------------------|
| `true`  | `false`                                                    |
| `false` | `true`                                                     |

[type.bool.expr.or](#r-type.bool.expr.or)

### Logical or

| `a` | `b` | [`a | b`](../expressions/operator-expr.md#arithmetic-and-logical-binary-operators) |
|----|----|----|
| `true` | `true` | `true` |
| `true` | `false` | `true` |
| `false` | `true` | `true` |
| `false` | `false` | `false` |

[type.bool.expr.and](#r-type.bool.expr.and)

### Logical and

| `a` | `b` | [`a & b`](../expressions/operator-expr.md#arithmetic-and-logical-binary-operators) |
|----|----|----|
| `true` | `true` | `true` |
| `true` | `false` | `false` |
| `false` | `true` | `false` |
| `false` | `false` | `false` |

[type.bool.expr.xor](#r-type.bool.expr.xor)

### Logical xor

| `a` | `b` | [`a ^ b`](../expressions/operator-expr.md#arithmetic-and-logical-binary-operators) |
|----|----|----|
| `true` | `true` | `false` |
| `true` | `false` | `true` |
| `false` | `true` | `true` |
| `false` | `false` | `false` |

[type.bool.expr.cmp](#r-type.bool.expr.cmp)

### Comparisons

[type.bool.expr.cmp.eq](#r-type.bool.expr.cmp.eq)

| `a` | `b` | [`a == b`](../expressions/operator-expr.md#comparison-operators) |
|----|----|----|
| `true` | `true` | `true` |
| `true` | `false` | `false` |
| `false` | `true` | `false` |
| `false` | `false` | `true` |

[type.bool.expr.cmp.greater](#r-type.bool.expr.cmp.greater)

| `a` | `b` | [`a > b`](../expressions/operator-expr.md#comparison-operators) |
|----|----|----|
| `true` | `true` | `false` |
| `true` | `false` | `true` |
| `false` | `true` | `false` |
| `false` | `false` | `false` |

[type.bool.expr.cmp.not-eq](#r-type.bool.expr.cmp.not-eq)

- `a != b` is the same as `!(a == b)`

[type.bool.expr.cmp.greater-eq](#r-type.bool.expr.cmp.greater-eq)

- `a >= b` is the same as `a == b | a > b`

[type.bool.expr.cmp.less](#r-type.bool.expr.cmp.less)

- `a < b` is the same as `!(a >= b)`

[type.bool.expr.cmp.less-eq](#r-type.bool.expr.cmp.less-eq)

- `a <= b` is the same as `a == b | a < b`

[type.bool.validity](#r-type.bool.validity)

## Bit validity

The single byte of a `bool` is guaranteed to be initialized (in other words, `transmute::<bool, u8>(...)` is always sound – but since some bit patterns are invalid `bool`s, the inverse is not always sound).
