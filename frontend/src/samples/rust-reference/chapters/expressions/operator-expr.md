# Operator expressions

[expr.operator](#r-expr.operator)

# Operator expressions

[expr.operator.syntax](#r-expr.operator.syntax)

**Syntax**\
[OperatorExpression](#railroad-OperatorExpression) →\
      [BorrowExpression](#grammar-BorrowExpression)\
    \| [DereferenceExpression](#grammar-DereferenceExpression)\
    \| [TryPropagationExpression](#grammar-TryPropagationExpression)\
    \| [NegationExpression](#grammar-NegationExpression)\
    \| [ArithmeticOrLogicalExpression](#grammar-ArithmeticOrLogicalExpression)\
    \| [ComparisonExpression](#grammar-ComparisonExpression)\
    \| [LazyBooleanExpression](#grammar-LazyBooleanExpression)\
    \| [TypeCastExpression](#grammar-TypeCastExpression)\
    \| [AssignmentExpression](#grammar-AssignmentExpression)\
    \| [CompoundAssignmentExpression](#grammar-CompoundAssignmentExpression)

Show Railroad

![](../../assets/inline/operator-expr-1.svg) [expr.operator.intro](#r-expr.operator.intro)

Operators are defined for built in types by the Rust language.

[expr.operator.trait](#r-expr.operator.trait)

Many of the following operators can also be overloaded using traits in `std::ops` or `std::cmp`.

[expr.operator.int-overflow](#r-expr.operator.int-overflow)

## Overflow

[expr.operator.int-overflow.intro](#r-expr.operator.int-overflow.intro)

Integer operators will panic when they overflow when compiled in debug mode. The `-C debug-assertions` and `-C overflow-checks` compiler flags can be used to control this more directly. The following things are considered to be overflow:

[expr.operator.int-overflow.binary-arith](#r-expr.operator.int-overflow.binary-arith)

- When `+`, `*` or binary `-` create a value greater than the maximum value, or less than the minimum value that can be stored.

[expr.operator.int-overflow.unary-neg](#r-expr.operator.int-overflow.unary-neg)

- Applying unary `-` to the most negative value of any signed integer type, unless the operand is a [literal expression](literal-expr.md#integer-literal-expressions) (or a literal expression standing alone inside one or more [grouped expressions](grouped-expr.md)).

[expr.operator.int-overflow.div](#r-expr.operator.int-overflow.div)

- Using `/` or `%`, where the left-hand argument is the smallest integer of a signed integer type and the right-hand argument is `-1`. These checks occur even when `-C overflow-checks` is disabled, for legacy reasons.

[expr.operator.int-overflow.shift](#r-expr.operator.int-overflow.shift)

- Using `<<` or `>>` where the right-hand argument is greater than or equal to the number of bits in the type of the left-hand argument, or is negative.

> ![](../../assets/inline/operator-expr-2.svg)Note
>
> The exception for literal expressions behind unary `-` means that forms such as `-128_i8` or `let j: i8 = -(128)` never cause a panic and have the expected value of -128.
>
> In these cases, the literal expression already has the most negative value for its type (for example, `128_i8` has the value -128) because integer literals are truncated to their type per the description in [Integer literal expressions](literal-expr.md#integer-literal-expressions).
>
> Negation of these most negative values leaves the value unchanged due to two’s complement overflow conventions.
>
> In `rustc`, these most negative expressions are also ignored by the `overflowing_literals` lint check.

[expr.operator.borrow](#r-expr.operator.borrow)

## Borrow operators

[expr.operator.borrow.syntax](#r-expr.operator.borrow.syntax)

**Syntax**\
[BorrowExpression](#railroad-BorrowExpression) →\
      ( & \| && ) [Expression](../expressions.md#grammar-Expression)\
    \| ( & \| && ) mut [Expression](../expressions.md#grammar-Expression)\
    \| ( & \| && ) raw const [Expression](../expressions.md#grammar-Expression)\
    \| ( & \| && ) raw mut [Expression](../expressions.md#grammar-Expression)

Show Railroad

![](../../assets/inline/operator-expr-3.svg) [expr.operator.borrow.intro](#r-expr.operator.borrow.intro)

The `&` (shared borrow) and `&mut` (mutable borrow) operators are unary prefix operators.

[expr.operator.borrow.result](#r-expr.operator.borrow.result)

When applied to a [place expression](../expressions.md#place-expressions-and-value-expressions), this expressions produces a reference (pointer) to the location that the value refers to.

[expr.operator.borrow.lifetime](#r-expr.operator.borrow.lifetime)

The memory location is also placed into a borrowed state for the duration of the reference. For a shared borrow (`&`), this implies that the place may not be mutated, but it may be read or shared again. For a mutable borrow (`&mut`), the place may not be accessed in any way until the borrow expires.

[expr.operator.borrow.mut](#r-expr.operator.borrow.mut)

`&mut` evaluates its operand in a mutable place expression context.

[expr.operator.borrow.temporary](#r-expr.operator.borrow.temporary)

If the `&` or `&mut` operators are applied to a [value expression](../expressions.md#place-expressions-and-value-expressions), then a [temporary value](../expressions.md#temporaries) is created.

These operators cannot be overloaded.

``` playground
#![allow(unused)]
fn main() {
{
    // a temporary with value 7 is created that lasts for this scope.
    let shared_reference = &7;
}
let mut array = [-2, 3, 9];
{
    // Mutably borrows `array` for this scope.
    // `array` may only be used through `mutable_reference`.
    let mutable_reference = &mut array;
}
}
```

[expr.borrow.and-and-syntax](#r-expr.borrow.and-and-syntax)

Even though `&&` is a single token ([the lazy ‘and’ operator](#lazy-boolean-operators)), when used in the context of borrow expressions it works as two borrows:

``` playground
#![allow(unused)]
fn main() {
// same meanings:
let a = &&  10;
let a = & & 10;

// same meanings:
let a = &&&&  mut 10;
let a = && && mut 10;
let a = & & & & mut 10;
}
```

[expr.borrow.raw](#r-expr.borrow.raw)

### Raw borrow operators

[expr.borrow.raw.intro](#r-expr.borrow.raw.intro)

`&raw const` and `&raw mut` are the *raw borrow operators*.

[expr.borrow.raw.place](#r-expr.borrow.raw.place)

The operand expression of these operators is evaluated in place expression context.

[expr.borrow.raw.result](#r-expr.borrow.raw.result)

`&raw const expr` then creates a const raw pointer of type `*const T` to the given place, and `&raw mut expr` creates a mutable raw pointer of type `*mut T`.

[expr.borrow.raw.invalid-ref](#r-expr.borrow.raw.invalid-ref)

The raw borrow operators must be used instead of a borrow operator whenever the place expression could evaluate to a place that is not properly aligned or does not store a valid value as determined by its type, or whenever creating a reference would introduce incorrect aliasing assumptions. In those situations, using a borrow operator would cause [undefined behavior](../behavior-considered-undefined.md) by creating an invalid reference, but a raw pointer may still be constructed.

The following is an example of creating a raw pointer to an unaligned place through a `packed` struct:

``` playground
#![allow(unused)]
fn main() {
#[repr(packed)]
struct Packed {
    f1: u8,
    f2: u16,
}

let packed = Packed { f1: 1, f2: 2 };
// `&packed.f2` would create an unaligned reference, and thus be undefined behavior!
let raw_f2 = &raw const packed.f2;
assert_eq!(unsafe { raw_f2.read_unaligned() }, 2);
}
```

The following is an example of creating a raw pointer to a place that does not contain a valid value:

``` playground
#![allow(unused)]
fn main() {
use std::mem::MaybeUninit;

struct Demo {
    field: bool,
}

let mut uninit = MaybeUninit::<Demo>::uninit();
// `&uninit.as_mut().field` would create a reference to an uninitialized `bool`,
// and thus be undefined behavior!
let f1_ptr = unsafe { &raw mut (*uninit.as_mut_ptr()).field };
unsafe { f1_ptr.write(true); }
let init = unsafe { uninit.assume_init() };
}
```

[expr.deref](#r-expr.deref)

## The dereference operator

[expr.deref.syntax](#r-expr.deref.syntax)

**Syntax**\
[DereferenceExpression](#railroad-DereferenceExpression) → \* [Expression](../expressions.md#grammar-Expression)

Show Railroad

![](../../assets/inline/operator-expr-4.svg) [expr.deref.intro](#r-expr.deref.intro)

The `*` (dereference) operator is also a unary prefix operator.

[expr.deref.result](#r-expr.deref.result)

When applied to a [pointer](../types/pointer.md) or [`Box`](../special-types-and-traits.md#boxt), it denotes the pointed-to location.

[expr.deref.mut](#r-expr.deref.mut)

If the expression is of type `&mut T`, `*mut T`, or `Box<T>`, and is either a local variable, a (nested) field of a local variable or is a mutable [place expression](../expressions.md#place-expressions-and-value-expressions), then the resulting memory location can be assigned to.

[expr.deref.box](#r-expr.deref.box)

When applied to a [`Box`](../special-types-and-traits.md#boxt), the resultant place may be [moved from](../expressions.md#r-expr.move.movable-place).

[expr.deref.safety](#r-expr.deref.safety)

Dereferencing a raw pointer requires `unsafe`.

[expr.deref.traits](#r-expr.deref.traits)

On non-pointer types `*x` is equivalent to `*std::ops::Deref::deref(&x)` in an [immutable place expression context](../expressions.md#mutability) and `*std::ops::DerefMut::deref_mut(&mut x)` in a mutable place expression context, except that when `*x` undergoes [temporary lifetime extension](../destructors.md#r-destructors.scope.lifetime-extension), the dereferenced expression `x` also has its [temporary scope](../destructors.md#r-destructors.scope.temporary) extended.

``` playground
#![allow(unused)]
fn main() {
struct NoCopy;
let a = &7;
assert_eq!(*a, 7);
let b = &mut 9;
*b = 11;
assert_eq!(*b, 11);
let c = Box::new(NoCopy);
let d: NoCopy = *c;
}
```

``` playground
#![allow(unused)]
fn main() {
// The temporary holding the result of `String::new()` is extended
// to live to the end of the block, so `x` may be used in subsequent
// statements.
let x = &*String::new();
x;
}
```

``` playground
#![allow(unused)]
fn main() {
// The temporary holding the result of `String::new()` is dropped at
// the end of the statement, so it's an error to use `y` after.
let y = &*std::ops::Deref::deref(&String::new()); // ERROR
y;
}
```

[expr.try](#r-expr.try)

## The try propagation expression

[expr.try.syntax](#r-expr.try.syntax)

**Syntax**\
[TryPropagationExpression](#railroad-TryPropagationExpression) → [Expression](../expressions.md#grammar-Expression) ?

Show Railroad

![](../../assets/inline/operator-expr-5.svg) [expr.try.intro](#r-expr.try.intro)

The try propagation expression uses the value of the inner expression and the [`Try`](../../core/ops/try_trait/trait.Try.html) trait to decide whether to produce a value, and if so, what value to produce, or whether to return a value to the caller, and if so, what value to return.

> ![](../../assets/inline/operator-expr-6.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> use std::num::ParseIntError;
> fn try_to_parse() -> Result {
>     let x: i32 = "123".parse()?; // `x` is `123`.
>     let y: i32 = "24a".parse()?; // Returns an `Err()` immediately.
>     Ok(x + y)                    // Doesn't run.
> }
>
> let res = try_to_parse();
> println!("{res:?}");
> assert!(res.is_err())
> }
> ```
>
> ``` playground
> #![allow(unused)]
> fn main() {
> fn try_option_some() -> Option {
>     let val = Some(1)?;
>     Some(val)
> }
> assert_eq!(try_option_some(), Some(1));
>
> fn try_option_none() -> Option {
>     let val = None?;
>     Some(val)
> }
> assert_eq!(try_option_none(), None);
> }
> ```
>
> ``` playground
> use std::ops::ControlFlow;
>
> pub struct TreeNode {
>     value: T,
>     left: Option>>,
>     right: Option>>,
> }
>
> impl TreeNode {
>     pub fn traverse_inorder(&self, f: &mut impl FnMut(&T) -> ControlFlow) -> ControlFlow {
>         if let Some(left) = &self.left {
>             left.traverse_inorder(f)?;
>         }
>         f(&self.value)?;
>         if let Some(right) = &self.right {
>             right.traverse_inorder(f)?;
>         }
>         ControlFlow::Continue(())
>     }
> }
>
> fn main() {
>     let n = TreeNode {
>         value: 1,
>         left: Some(Box::new(TreeNode{value: 2, left: None, right: None})),
>         right: None,
>     };
>     let v = n.traverse_inorder(&mut |t| {
>         if *t == 2 {
>             ControlFlow::Break("found")
>         } else {
>             ControlFlow::Continue(())
>         }
>     });
>     assert_eq!(v, ControlFlow::Break("found"));
> }
> ```

> ![](../../assets/inline/operator-expr-7.svg)Note
>
> The [`Try`](../../core/ops/try_trait/trait.Try.html) trait is currently unstable, and thus cannot be implemented for user types.
>
> The try propagation expression is currently roughly equivalent to:
>
> ``` playground
> #![allow(unused)]
> fn main() {
> #![ feature(try_trait_v2) ]
> fn example() -> Result<(), ()> {
> let expr = Ok(());
> match core::ops::Try::branch(expr) {
>     core::ops::ControlFlow::Continue(val) => val,
>     core::ops::ControlFlow::Break(residual) =>
>         return core::ops::FromResidual::from_residual(residual),
> }
> Ok(())
> }
> }
> ```

> ![](../../assets/inline/operator-expr-8.svg)Note
>
> The try propagation operator is sometimes called *the question mark operator*, *the `?` operator*, or *the try operator*.

[expr.try.restricted-types](#r-expr.try.restricted-types)

The try propagation operator can be applied to expressions with the type of:

- [`Result<T, E>`](../../core/result/enum.Result.html)
  - `Result::Ok(val)` evaluates to `val`.
  - `Result::Err(e)` returns `Result::Err(From::from(e))`.
- [`Option<T>`](../../core/option/enum.Option.html)
  - `Option::Some(val)` evaluates to `val`.
  - `Option::None` returns `Option::None`.
- [`ControlFlow<B, C>`](../../core/ops/control_flow/enum.ControlFlow.html)
  - `ControlFlow::Continue(c)` evaluates to `c`.
  - `ControlFlow::Break(b)` returns `ControlFlow::Break(b)`.
- [`Poll<Result<T, E>>`](../../core/task/poll/enum.Poll.html)
  - `Poll::Ready(Ok(val))` evaluates to `Poll::Ready(val)`.
  - `Poll::Ready(Err(e))` returns `Poll::Ready(Err(From::from(e)))`.
  - `Poll::Pending` evaluates to `Poll::Pending`.
- [`Poll<Option<Result<T, E>>>`](../../core/task/poll/enum.Poll.html)
  - `Poll::Ready(Some(Ok(val)))` evaluates to `Poll::Ready(Some(val))`.
  - `Poll::Ready(Some(Err(e)))` returns `Poll::Ready(Some(Err(From::from(e))))`.
  - `Poll::Ready(None)` evaluates to `Poll::Ready(None)`.
  - `Poll::Pending` evaluates to `Poll::Pending`.

[expr.negate](#r-expr.negate)

## Negation operators

[expr.negate.syntax](#r-expr.negate.syntax)

**Syntax**\
[NegationExpression](#railroad-NegationExpression) →\
      - [Expression](../expressions.md#grammar-Expression)\
    \| ! [Expression](../expressions.md#grammar-Expression)

Show Railroad

![](../../assets/inline/operator-expr-9.svg) [expr.negate.intro](#r-expr.negate.intro)

These are the last two unary operators.

[expr.negate.results](#r-expr.negate.results)\
Tests Tests with this rule:

- [tests/ui/binop/operator-overloading.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/binop/operator-overloading.rs)

This table summarizes the behavior of them on primitive types and which traits are used to overload these operators for other types. Remember that signed integers are always represented using two’s complement. The operands of all of these operators are evaluated in [value expression context](../expressions.md#place-expressions-and-value-expressions) so are moved or copied.

| Symbol | Integer | `bool` | Floating Point | Overloading Trait |
|----|----|----|----|----|
| `-` | Negation\* |  | Negation | `std::ops::Neg` |
| `!` | Bitwise NOT | [Logical NOT](../types/boolean.md#logical-not) |  | `std::ops::Not` |

\* Only for signed integer types.

Here are some example of these operators

``` playground
#![allow(unused)]
fn main() {
let x = 6;
assert_eq!(-x, -6);
assert_eq!(!x, -7);
assert_eq!(true, !false);
}
```

[expr.arith-logic](#r-expr.arith-logic)

## Arithmetic and logical binary operators

[expr.arith-logic.syntax](#r-expr.arith-logic.syntax)\
Tests Tests with this rule:

- [tests/ui/binop/binary-minus-without-space.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/binop/binary-minus-without-space.rs)

**Syntax**\
[ArithmeticOrLogicalExpression](#railroad-ArithmeticOrLogicalExpression) →\
      [Expression](../expressions.md#grammar-Expression) + [Expression](../expressions.md#grammar-Expression)\
    \| [Expression](../expressions.md#grammar-Expression) - [Expression](../expressions.md#grammar-Expression)\
    \| [Expression](../expressions.md#grammar-Expression) \* [Expression](../expressions.md#grammar-Expression)\
    \| [Expression](../expressions.md#grammar-Expression) / [Expression](../expressions.md#grammar-Expression)\
    \| [Expression](../expressions.md#grammar-Expression) % [Expression](../expressions.md#grammar-Expression)\
    \| [Expression](../expressions.md#grammar-Expression) & [Expression](../expressions.md#grammar-Expression)\
    \| [Expression](../expressions.md#grammar-Expression) \| [Expression](../expressions.md#grammar-Expression)\
    \| [Expression](../expressions.md#grammar-Expression) ^ [Expression](../expressions.md#grammar-Expression)\
    \| [Expression](../expressions.md#grammar-Expression) \<\< [Expression](../expressions.md#grammar-Expression)\
    \| [Expression](../expressions.md#grammar-Expression) \>\> [Expression](../expressions.md#grammar-Expression)

Show Railroad

![](../../assets/inline/operator-expr-10.svg) [expr.arith-logic.intro](#r-expr.arith-logic.intro)

Binary operators expressions are all written with infix notation.

[expr.arith-logic.behavior](#r-expr.arith-logic.behavior)\
Tests Tests with this rule:

- [tests/ui/binop/augmented-assignment.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/binop/augmented-assignment.rs)
- [tests/ui/binop/binop-consume-args.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/binop/binop-consume-args.rs)
- [tests/ui/binop/binop-move-semantics.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/binop/binop-move-semantics.rs)
- [tests/ui/binop/operator-overloading.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/binop/operator-overloading.rs)

This table summarizes the behavior of arithmetic and logical binary operators on primitive types and which traits are used to overload these operators for other types. Remember that signed integers are always represented using two’s complement. The operands of all of these operators are evaluated in [value expression context](../expressions.md#place-expressions-and-value-expressions) so are moved or copied.

| Symbol | Integer | `bool` | Floating Point | Overloading Trait | Overloading Compound Assignment Trait |
|----|----|----|----|----|----|
| `+` | Addition |  | Addition | `std::ops::Add` | `std::ops::AddAssign` |
| `-` | Subtraction |  | Subtraction | `std::ops::Sub` | `std::ops::SubAssign` |
| `*` | Multiplication |  | Multiplication | `std::ops::Mul` | `std::ops::MulAssign` |
| `/` | Division\*† |  | Division | `std::ops::Div` | `std::ops::DivAssign` |
| `%` | Remainder\*\*† |  | Remainder | `std::ops::Rem` | `std::ops::RemAssign` |
| `&` | Bitwise AND | [Logical AND](../types/boolean.md#logical-and) |  | `std::ops::BitAnd` | `std::ops::BitAndAssign` |
| `|` | Bitwise OR | [Logical OR](../types/boolean.md#logical-or) |  | `std::ops::BitOr` | `std::ops::BitOrAssign` |
| `^` | Bitwise XOR | [Logical XOR](../types/boolean.md#logical-xor) |  | `std::ops::BitXor` | `std::ops::BitXorAssign` |
| `<<` | Left Shift |  |  | `std::ops::Shl` | `std::ops::ShlAssign` |
| `>>` | Right Shift\*\*\* |  |  | `std::ops::Shr` | `std::ops::ShrAssign` |

\* Integer division rounds towards zero.

\*\* Rust uses a remainder defined with [truncating division](https://en.wikipedia.org/wiki/Modulo_operation#Variants_of_the_definition). Given `remainder = dividend % divisor`, the remainder will have the same sign as the dividend.

\*\*\* Arithmetic right shift on signed integer types, logical right shift on unsigned integer types.

† For integer types, division by zero panics.

Here are examples of these operators being used.

``` playground
#![allow(unused)]
fn main() {
assert_eq!(3 + 6, 9);
assert_eq!(5.5 - 1.25, 4.25);
assert_eq!(-5 * 14, -70);
assert_eq!(14 / 3, 4);
assert_eq!(100 % 7, 2);
assert_eq!(0b1010 & 0b1100, 0b1000);
assert_eq!(0b1010 | 0b1100, 0b1110);
assert_eq!(0b1010 ^ 0b1100, 0b110);
assert_eq!(13 << 3, 104);
assert_eq!(-10 >> 2, -3);
}
```

[expr.cmp](#r-expr.cmp)

## Comparison operators

[expr.cmp.syntax](#r-expr.cmp.syntax)

**Syntax**\
[ComparisonExpression](#railroad-ComparisonExpression) →\
      [Expression](../expressions.md#grammar-Expression) == [Expression](../expressions.md#grammar-Expression)\
    \| [Expression](../expressions.md#grammar-Expression) != [Expression](../expressions.md#grammar-Expression)\
    \| [Expression](../expressions.md#grammar-Expression) \> [Expression](../expressions.md#grammar-Expression)\
    \| [Expression](../expressions.md#grammar-Expression) \< [Expression](../expressions.md#grammar-Expression)\
    \| [Expression](../expressions.md#grammar-Expression) \>= [Expression](../expressions.md#grammar-Expression)\
    \| [Expression](../expressions.md#grammar-Expression) \<= [Expression](../expressions.md#grammar-Expression)

Show Railroad

![](../../assets/inline/operator-expr-11.svg) [expr.cmp.intro](#r-expr.cmp.intro)

Comparison operators are also defined both for primitive types and many types in the standard library.

[expr.cmp.paren-chaining](#r-expr.cmp.paren-chaining)

Parentheses are required when chaining comparison operators. For example, the expression `a == b == c` is invalid and may be written as `(a == b) == c`.

[expr.cmp.trait](#r-expr.cmp.trait)

Unlike arithmetic and logical operators, the traits for overloading these operators are used more generally to show how a type may be compared and will likely be assumed to define actual comparisons by functions that use these traits as bounds. Many functions and macros in the standard library can then use that assumption (although not to ensure safety).

[expr.cmp.place](#r-expr.cmp.place)

Unlike the arithmetic and logical operators above, these operators implicitly take shared borrows of their operands, evaluating them in [place expression context](../expressions.md#place-expressions-and-value-expressions):

``` playground
#![allow(unused)]
fn main() {
let a = 1;
let b = 1;
a == b;
// is equivalent to
::std::cmp::PartialEq::eq(&a, &b);
}
```

This means that the operands don’t have to be moved out of.

[expr.cmp.behavior](#r-expr.cmp.behavior)

| Symbol | Meaning                  | Overloading method         |
|--------|--------------------------|----------------------------|
| `==`   | Equal                    | `std::cmp::PartialEq::eq`  |
| `!=`   | Not equal                | `std::cmp::PartialEq::ne`  |
| `>`    | Greater than             | `std::cmp::PartialOrd::gt` |
| `<`    | Less than                | `std::cmp::PartialOrd::lt` |
| `>=`   | Greater than or equal to | `std::cmp::PartialOrd::ge` |
| `<=`   | Less than or equal to    | `std::cmp::PartialOrd::le` |

Here are examples of the comparison operators being used.

``` playground
#![allow(unused)]
fn main() {
assert!(123 == 123);
assert!(23 != -12);
assert!(12.5 > 12.2);
assert!([1, 2, 3] < [1, 3, 4]);
assert!('A' <= 'B');
assert!("World" >= "Hello");
}
```

[expr.bool-logic](#r-expr.bool-logic)

## Lazy boolean operators

[expr.bool-logic.syntax](#r-expr.bool-logic.syntax)

**Syntax**\
[LazyBooleanExpression](#railroad-LazyBooleanExpression) →\
      [Expression](../expressions.md#grammar-Expression) \|\| [Expression](../expressions.md#grammar-Expression)\
    \| [Expression](../expressions.md#grammar-Expression) && [Expression](../expressions.md#grammar-Expression)

Show Railroad

![](../../assets/inline/operator-expr-12.svg) [expr.bool-logic.intro](#r-expr.bool-logic.intro)

The operators `||` and `&&` may be applied to operands of boolean type. The `||` operator denotes logical ‘or’, and the `&&` operator denotes logical ‘and’.

[expr.bool-logic.conditional-evaluation](#r-expr.bool-logic.conditional-evaluation)

They differ from `|` and `&` in that the right-hand operand is only evaluated when the left-hand operand does not already determine the result of the expression. That is, `||` only evaluates its right-hand operand when the left-hand operand evaluates to `false`, and `&&` only when it evaluates to `true`.

``` playground
#![allow(unused)]
fn main() {
let x = false || true; // true
let y = false && panic!(); // false, doesn't evaluate `panic!()`
}
```

[expr.as](#r-expr.as)

## Type cast expressions

[expr.as.syntax](#r-expr.as.syntax)

**Syntax**\
[TypeCastExpression](#railroad-TypeCastExpression) → [Expression](../expressions.md#grammar-Expression) as [TypeNoBounds](../types.md#grammar-TypeNoBounds)

Show Railroad

![](../../assets/inline/operator-expr-13.svg) [expr.as.intro](#r-expr.as.intro)

A type cast expression is denoted with the binary operator `as`.

[expr.as.result](#r-expr.as.result)

Executing an `as` expression casts the value on the left-hand side to the type on the right-hand side.

An example of an `as` expression:

``` playground
#![allow(unused)]
fn main() {
fn sum(values: &[f64]) -> f64 { 0.0 }
fn len(values: &[f64]) -> i32 { 0 }
fn average(values: &[f64]) -> f64 {
    let sum: f64 = sum(values);
    let size: f64 = len(values) as f64;
    sum / size
}
}
```

[expr.as.coercions](#r-expr.as.coercions)

`as` can be used to explicitly perform [coercions](../type-coercions.md), as well as the following additional casts. Any cast that does not fit either a coercion rule or an entry in the table is a compiler error. Here `*T` means either `*const T` or `*mut T`. `m` stands for optional `mut` in reference types and `mut` or `const` in pointer types.

| Type of `e` | `U` | Cast performed by `e as U` |
|----|----|----|
| Integer or Float type | Integer or Float type | [Numeric cast](#r-expr.as.numeric) |
| Enumeration | Integer type | [Enum cast](#r-expr.as.enum) |
| `bool` or `char` | Integer type | [Primitive to integer cast](#r-expr.as.bool-char-as-int) |
| `u8` | `char` | [`u8` to `char` cast](#r-expr.as.u8-as-char) |
| `*T` | `*V` (when [compatible](#r-expr.as.pointer)) | [Pointer to pointer cast](#r-expr.as.pointer) |
| `*T` where `T: Sized` | Integer type | [Pointer to address cast](#r-expr.as.pointer-as-int) |
| Integer type | `*V` where `V: Sized` | [Address to pointer cast](#r-expr.as.int-as-pointer) |
| `&m₁ [T; n]` | `*m₂ T` [1](#footnote-lessmut) | Array to pointer cast |
| `*m₁ [T; n]` | `*m₂ T` [1](#footnote-lessmut) | Array to pointer cast |
| [Function item](../types/function-item.md) | [Function pointer](../types/function-pointer.md) | Function item to function pointer cast |
| [Function item](../types/function-item.md) | `*V` where `V: Sized` | Function item to pointer cast |
| [Function item](../types/function-item.md) | Integer | Function item to address cast |
| [Function pointer](../types/function-pointer.md) | `*V` where `V: Sized` | Function pointer to pointer cast |
| [Function pointer](../types/function-pointer.md) | Integer | Function pointer to address cast |
| Closure [2](#footnote-no-capture) | Function pointer | Closure to function pointer cast |

### Semantics

[expr.as.numeric](#r-expr.as.numeric)

#### Numeric cast

[expr.as.numeric.int-same-size](#r-expr.as.numeric.int-same-size)

- Casting between two integers of the same size (e.g. i32 -\> u32) is a no-op (Rust uses 2’s complement for negative values of fixed integers)

  ``` playground
  #![allow(unused)]
  fn main() {
  assert_eq!(42i8 as u8, 42u8);
  assert_eq!(-1i8 as u8, 255u8);
  assert_eq!(255u8 as i8, -1i8);
  assert_eq!(-1i16 as u16, 65535u16);
  }
  ```

[expr.as.numeric.int-truncation](#r-expr.as.numeric.int-truncation)

- Casting from a larger integer to a smaller integer (e.g. u32 -\> u8) will truncate

  ``` playground
  #![allow(unused)]
  fn main() {
  assert_eq!(42u16 as u8, 42u8);
  assert_eq!(1234u16 as u8, 210u8);
  assert_eq!(0xabcdu16 as u8, 0xcdu8);

  assert_eq!(-42i16 as i8, -42i8);
  assert_eq!(1234u16 as i8, -46i8);
  assert_eq!(0xabcdi32 as i8, -51i8);
  }
  ```

[expr.as.numeric.int-extension](#r-expr.as.numeric.int-extension)

- Casting from a smaller integer to a larger integer (e.g. u8 -\> u32) will

  - zero-extend if the source is unsigned
  - sign-extend if the source is signed

  ``` playground
  #![allow(unused)]
  fn main() {
  assert_eq!(42i8 as i16, 42i16);
  assert_eq!(-17i8 as i16, -17i16);
  assert_eq!(0b1000_1010u8 as u16, 0b0000_0000_1000_1010u16, "Zero-extend");
  assert_eq!(0b0000_1010i8 as i16, 0b0000_0000_0000_1010i16, "Sign-extend 0");
  assert_eq!(0b1000_1010u8 as i8 as i16, 0b1111_1111_1000_1010u16 as i16, "Sign-extend 1");
  }
  ```

[expr.as.numeric.float-as-int](#r-expr.as.numeric.float-as-int)

- Casting from a float to an integer will round the float towards zero

  - `NaN` will return `0`
  - Values larger than the maximum integer value, including `INFINITY`, will saturate to the maximum value of the integer type.
  - Values smaller than the minimum integer value, including `NEG_INFINITY`, will saturate to the minimum value of the integer type.

  ``` playground
  #![allow(unused)]
  fn main() {
  assert_eq!(42.9f32 as i32, 42);
  assert_eq!(-42.9f32 as i32, -42);
  assert_eq!(42_000_000f32 as i32, 42_000_000);
  assert_eq!(std::f32::NAN as i32, 0);
  assert_eq!(1_000_000_000_000_000f32 as i32, 0x7fffffffi32);
  assert_eq!(std::f32::NEG_INFINITY as i32, -0x80000000i32);
  }
  ```

[expr.as.numeric.int-as-float](#r-expr.as.numeric.int-as-float)

- Casting from an integer to float will produce the closest possible float \*

  - if necessary, rounding is according to `roundTiesToEven` mode \*\*\*
  - on overflow, infinity (of the same sign as the input) is produced
  - note: with the current set of numeric types, overflow can only happen on `u128 as f32` for values greater or equal to `f32::MAX + (0.5 ULP)`

  ``` playground
  #![allow(unused)]
  fn main() {
  assert_eq!(1337i32 as f32, 1337f32);
  assert_eq!(123_456_789i32 as f32, 123_456_790f32, "Rounded");
  assert_eq!(0xffffffff_ffffffff_ffffffff_ffffffff_u128 as f32, std::f32::INFINITY);
  }
  ```

[expr.as.numeric.float-widening](#r-expr.as.numeric.float-widening)

- Casting from an f32 to an f64 is perfect and lossless

  ``` playground
  #![allow(unused)]
  fn main() {
  assert_eq!(1_234.5f32 as f64, 1_234.5f64);
  assert_eq!(std::f32::INFINITY as f64, std::f64::INFINITY);
  assert!((std::f32::NAN as f64).is_nan());
  }
  ```

[expr.as.numeric.float-narrowing](#r-expr.as.numeric.float-narrowing)

- Casting from an f64 to an f32 will produce the closest possible f32 \*\*

  - if necessary, rounding is according to `roundTiesToEven` mode \*\*\*
  - on overflow, infinity (of the same sign as the input) is produced

  ``` playground
  #![allow(unused)]
  fn main() {
  assert_eq!(1_234.5f64 as f32, 1_234.5f32);
  assert_eq!(1_234_567_891.123f64 as f32, 1_234_567_890f32, "Rounded");
  assert_eq!(std::f64::INFINITY as f32, std::f32::INFINITY);
  assert!((std::f64::NAN as f32).is_nan());
  }
  ```

\* if integer-to-float casts with this rounding mode and overflow behavior are not supported natively by the hardware, these casts will likely be slower than expected.

\*\* if f64-to-f32 casts with this rounding mode and overflow behavior are not supported natively by the hardware, these casts will likely be slower than expected.

\*\*\* as defined in IEEE 754-2008 §4.3.1: pick the nearest floating point number, preferring the one with an even least significant digit if exactly halfway between two floating point numbers.

[expr.as.enum](#r-expr.as.enum)

#### Enum cast

[expr.as.enum.discriminant](#r-expr.as.enum.discriminant)

Casts an enum to its discriminant, then uses a numeric cast if needed. Casting is limited to the following kinds of enumerations:

- [Unit-only enums](../items/enumerations.md#unit-only-enum)
- [Field-less enums](../items/enumerations.md#field-less-enum) without [explicit discriminants](../items/enumerations.md#explicit-discriminants), or where only unit-variants have explicit discriminants

``` playground
#![allow(unused)]
fn main() {
enum Enum { A, B, C }
assert_eq!(Enum::A as i32, 0);
assert_eq!(Enum::B as i32, 1);
assert_eq!(Enum::C as i32, 2);
}
```

[expr.as.enum.no-drop](#r-expr.as.enum.no-drop)

Casting is not allowed if the enum implements [`Drop`](../../core/ops/drop/trait.Drop.html).

[expr.as.bool-char-as-int](#r-expr.as.bool-char-as-int)

#### Primitive to integer cast

- `false` casts to `0`, `true` casts to `1`
- `char` casts to the value of the code point, then uses a numeric cast if needed.

``` playground
#![allow(unused)]
fn main() {
assert_eq!(false as i32, 0);
assert_eq!(true as i32, 1);
assert_eq!('A' as i32, 65);
assert_eq!('Ö' as i32, 214);
}
```

[expr.as.u8-as-char](#r-expr.as.u8-as-char)

#### `u8` to `char` cast

Casts to the `char` with the corresponding code point.

``` playground
#![allow(unused)]
fn main() {
assert_eq!(65u8 as char, 'A');
assert_eq!(214u8 as char, 'Ö');
}
```

[expr.as.pointer-as-int](#r-expr.as.pointer-as-int)

#### Pointer to address cast

Casting from a raw pointer to an integer produces the machine address of the referenced memory. If the integer type is smaller than the pointer type, the address may be truncated; using `usize` avoids this.

[expr.as.int-as-pointer](#r-expr.as.int-as-pointer)

#### Address to pointer cast

Casting from an integer to a raw pointer interprets the integer as a memory address and produces a pointer referencing that memory.

> ![](../../assets/inline/operator-expr-14.svg)Warning
>
> This interacts with the Rust memory model, which is still under development. A pointer obtained from this cast may suffer additional restrictions even if it is bitwise equal to a valid pointer. Dereferencing such a pointer may be [undefined behavior](../behavior-considered-undefined.md) if aliasing rules are not followed.

A trivial example of sound address arithmetic:

``` playground
#![allow(unused)]
fn main() {
let mut values: [i32; 2] = [1, 2];
let p1: *mut i32 = values.as_mut_ptr();
let first_address = p1 as usize;
let second_address = first_address + 4; // 4 == size_of::<i32>()
let p2 = second_address as *mut i32;
unsafe {
    *p2 += 1;
}
assert_eq!(values[1], 3);
}
```

[expr.as.pointer](#r-expr.as.pointer)

#### Pointer-to-pointer cast

[expr.as.pointer.behavior](#r-expr.as.pointer.behavior)

`*const T` / `*mut T` can be cast to `*const U` / `*mut U` with the following behavior:

[expr.as.pointer.sized](#r-expr.as.pointer.sized)

- If `T` and `U` are both sized, the pointer is returned unchanged.

  > ![](../../assets/inline/operator-expr-15.svg)Example
  >
  > ``` playground
  > #![allow(unused)]
  > fn main() {
  > let x: i32 = 42;
  > let p1: *const i32 = &x;
  > let p2: *const u8 = p1 as *const u8;
  > // The pointer address remains the same.
  > assert_eq!(p1 as usize, p2 as usize);
  > }
  > ```

[expr.as.pointer.discard-metadata](#r-expr.as.pointer.discard-metadata)

- If `T` is unsized and `U` is sized, the cast discards all [metadata](../dynamically-sized-types.md#r-dynamic-sized.pointer-types) that completes the wide pointer `T` and produces a thin pointer `U` consisting of the data part of the unsized pointer.

  > ![](../../assets/inline/operator-expr-16.svg)Example
  >
  > ``` playground
  > #![allow(unused)]
  > fn main() {
  > let slice: &[i32] = &[1, 2, 3];
  > let ptr: *const [i32] = slice as *const [i32];
  > // Cast from wide pointer (*const [i32]) to thin pointer (*const i32)
  > // discarding the length metadata.
  > let data_ptr: *const i32 = ptr as *const i32;
  > assert_eq!(unsafe { *data_ptr }, 1);
  > }
  > ```

[expr.as.pointer.unsized.unchanged](#r-expr.as.pointer.unsized.unchanged)

- If `T` and `U` are both unsized, the pointer is also returned unchanged. In particular, the metadata is preserved exactly. The cast can only be performed if the metadata is compatible according to the below rules:

[expr.as.pointer.unsized.slice](#r-expr.as.pointer.unsized.slice)

- When `T` and `U` are unsized with slice metadata, they are always compatible. The metadata of a slice is the number of elements, so casting `*[u16] -> *[u8]` is legal but will result in reducing the number of bytes by half.

  > ![](../../assets/inline/operator-expr-17.svg)Example
  >
  > ``` playground
  > #![allow(unused)]
  > fn main() {
  > let slice: &[u16] = &[1, 2, 3];
  > let ptr: *const [u16] = slice as *const [u16];
  > let byte_ptr: *const [u8] = ptr as *const [u8];
  > assert_eq!(byte_ptr.len(), 3);
  > }
  > ```

[expr.as.pointer.unsized.trait](#r-expr.as.pointer.unsized.trait)

- When `T` and `U` are unsized with trait object metadata, the metadata is compatible only when all of the following holds:
  1.  The principal trait must be the same.

      > ![](../../assets/inline/operator-expr-18.svg)Example
      >
      > ``` playground
      > #![allow(unused)]
      > fn main() {
      > trait Foo {}
      > trait Bar {}
      > impl Foo for i32 {}
      > impl Bar for i32 {}
      >
      > let x: i32 = 42;
      > let ptr_foo: *const dyn Foo = &x as *const dyn Foo;
      > // You can't cast to a different principal trait.
      > let ptr_bar: *const dyn Bar = ptr_foo as *const dyn Bar; // ERROR
      > }
      > ```

  2.  Auto traits may be removed.

      > ![](../../assets/inline/operator-expr-19.svg)Example
      >
      > ``` playground
      > #![allow(unused)]
      > fn main() {
      > trait Foo {}
      > struct S;
      > impl Foo for S {}
      > unsafe impl Send for S {}
      >
      > let s = S;
      > let ptr_send: *const (dyn Foo + Send) = &s;
      > // Removing an auto trait.
      > let ptr_no_send: *const dyn Foo = ptr_send as *const dyn Foo;
      > }
      > ```

  3.  Auto traits may be added only if they are a super trait of the principal trait.

      > ![](../../assets/inline/operator-expr-20.svg)Example
      >
      > ``` playground
      > #![allow(unused)]
      > fn main() {
      > trait Foo: Send {}
      > struct S;
      > impl Foo for S {}
      > unsafe impl Send for S {}
      >
      > let s = S;
      > let ptr_no_send: *const dyn Foo = &s;
      > // Adding an auto trait.
      > let ptr_send: *const (dyn Foo + Send) = ptr_no_send as *const (dyn Foo + Send);
      > }
      > ```
      >
      > ``` playground
      > #![allow(unused)]
      > fn main() {
      > trait Foo {}
      > struct S;
      > impl Foo for S {}
      > unsafe impl Send for S {}
      >
      > let s = S;
      > let ptr_no_send: *const dyn Foo = &s;
      > // Same as above, except trait Foo does not have Send as a super trait.
      > let ptr_send: *const (dyn Foo + Send) = ptr_no_send as *const (dyn Foo + Send); // ERROR
      > }
      > ```

  4.  Trailing lifetimes may only be shortened.

      > ![](../../assets/inline/operator-expr-21.svg)Example
      >
      > ``` playground
      > #![allow(unused)]
      > fn main() {
      > trait Foo {}
      >
      > fn shorten_lifetime<'long: 'short, 'short>(
      >     ptr: *const (dyn Foo + 'long),
      > ) -> *const (dyn Foo + 'short) {
      >     // Shortening the lifetime is allowed.
      >     ptr as *const (dyn Foo + 'short)
      > }
      > }
      > ```
      >
      > ``` playground
      > #![allow(unused)]
      > fn main() {
      > trait Foo {}
      >
      > fn lengthen_lifetime<'long: 'short, 'short>(
      >     ptr: *const (dyn Foo + 'short),
      > ) -> *const (dyn Foo + 'long) {
      >     // It is not allowed to cast to a longer lifetime.
      >     ptr as *const (dyn Foo + 'long) // ERROR
      > }
      > }
      > ```

  5.  Generics (including lifetimes) and associated types must match exactly.

      > ![](../../assets/inline/operator-expr-22.svg)Example
      >
      > ``` playground
      > #![allow(unused)]
      > fn main() {
      > trait Generic {}
      > impl Generic for () {}
      > impl Generic for () {}
      >
      > let x = ();
      > let ptr_i32: *const dyn Generic = &x;
      > // You can't cast to a different generic parameter.
      > let ptr_u32: *const dyn Generic = ptr_i32 as *const dyn Generic; // ERROR
      > }
      > ```
      >
      > ``` playground
      > #![allow(unused)]
      > fn main() {
      > trait HasType {
      >     type Output;
      > }
      >
      > trait Generic<'x, T> {}
      >
      > fn cast_via_associated<'a, 'b, A, B>(
      >     ptr: *const dyn Generic<'a, A::Output>,
      > ) -> *const dyn Generic<'b, B::Output>
      > where
      >     'a: 'b,
      >     'b: 'a,
      >     A: HasType,
      >     B: HasType, // Forces equality
      > {
      >     ptr as *const dyn Generic<'b, B::Output>
      > }
      > }
      > ```

[expr.as.pointer.unsized.compound](#r-expr.as.pointer.unsized.compound)

- When `T` or `U` is a struct or tuple type whose last field is unsized, it has the same metadata and compatibility rules as its last field.

  > ![](../../assets/inline/operator-expr-23.svg)Example
  >
  > ``` playground
  > #![allow(unused)]
  > fn main() {
  > struct Wrapper(u32, [u8]);
  >
  > let slice: &[u8] = &[1, 2, 3];
  > let ptr: *const [u8] = slice;
  >
  > // The metadata (length 3) is preserved when casting to a struct
  > // where the last field is the unsized type `[u8]`.
  > let wrapper_ptr: *const Wrapper = ptr as *const Wrapper;
  >
  > // And preserved when casting back.
  > let ptr_back: *const [u8] = wrapper_ptr as *const [u8];
  > assert_eq!(ptr_back.len(), 3);
  > }
  > ```

[expr.assign](#r-expr.assign)

## Assignment expressions

[expr.assign.syntax](#r-expr.assign.syntax)

**Syntax**\
[AssignmentExpression](#railroad-AssignmentExpression) → [Expression](../expressions.md#grammar-Expression) = [Expression](../expressions.md#grammar-Expression)

Show Railroad

![](../../assets/inline/operator-expr-24.svg) [expr.assign.intro](#r-expr.assign.intro)

An *assignment expression* moves a value into a specified place.

[expr.assign.assignee](#r-expr.assign.assignee)

An assignment expression consists of a [mutable](../expressions.md#mutability) [assignee expression](../expressions.md#place-expressions-and-value-expressions), the *assignee operand*, followed by an equals sign (`=`) and a [value expression](../expressions.md#place-expressions-and-value-expressions), the *assigned value operand*.

[expr.assign.behavior-basic](#r-expr.assign.behavior-basic)

In its most basic form, an assignee expression is a [place expression](../expressions.md#place-expressions-and-value-expressions), and we discuss this case first.

[expr.assign.behavior-destructuring](#r-expr.assign.behavior-destructuring)

The more general case of destructuring assignment is discussed below, but this case always decomposes into sequential assignments to place expressions, which may be considered the more fundamental case.

[expr.assign.basic](#r-expr.assign.basic)

### Basic assignments

[expr.assign.evaluation-order](#r-expr.assign.evaluation-order)

Evaluating assignment expressions begins by evaluating its operands. The assigned value operand is evaluated first, followed by the assignee expression.

[expr.assign.destructuring-order](#r-expr.assign.destructuring-order)

For destructuring assignment, subexpressions of the assignee expression are evaluated left-to-right.

> ![](../../assets/inline/operator-expr-25.svg)Note
>
> This is different than other expressions in that the right operand is evaluated before the left one.

[expr.assign.drop-target](#r-expr.assign.drop-target)

It then has the effect of first [dropping](../destructors.md) the value at the assigned place, unless the place is an uninitialized local variable or an uninitialized field of a local variable.

[expr.assign.behavior](#r-expr.assign.behavior)

Next it either [copies or moves](../expressions.md#moved-and-copied-types) the assigned value to the assigned place.

[expr.assign.result](#r-expr.assign.result)

An assignment expression always produces [the unit value](../types/tuple.md).

Example:

``` playground
#![allow(unused)]
fn main() {
let mut x = 0;
let y = 0;
x = y;
}
```

[expr.assign.destructure](#r-expr.assign.destructure)

### Destructuring assignments

[expr.assign.destructure.intro](#r-expr.assign.destructure.intro)

Destructuring assignment is a counterpart to destructuring pattern matches for variable declaration, permitting assignment to complex values, such as tuples or structs. For instance, we may swap two mutable variables:

``` playground
#![allow(unused)]
fn main() {
let (mut a, mut b) = (0, 1);
// Swap `a` and `b` using destructuring assignment.
(b, a) = (a, b);
}
```

[expr.assign.destructure.assignee](#r-expr.assign.destructure.assignee)

In contrast to destructuring declarations using `let`, patterns may not appear on the left-hand side of an assignment due to syntactic ambiguities. Instead, a group of expressions that correspond to patterns are designated to be [assignee expressions](../expressions.md#place-expressions-and-value-expressions), and permitted on the left-hand side of an assignment. Assignee expressions are then desugared to pattern matches followed by sequential assignment.

[expr.assign.destructure.irrefutable](#r-expr.assign.destructure.irrefutable)

The desugared patterns must be irrefutable: in particular, this means that only slice patterns whose length is known at compile-time, and the trivial slice `[..]`, are permitted for destructuring assignment.

The desugaring method is straightforward, and is illustrated best by example.

``` playground
#![allow(unused)]
fn main() {
struct Struct { x: u32, y: u32 }
let (mut a, mut b) = (0, 0);
(a, b) = (3, 4);

[a, b] = [3, 4];

Struct { x: a, y: b } = Struct { x: 3, y: 4};

// desugars to:

{
    let (_a, _b) = (3, 4);
    a = _a;
    b = _b;
}

{
    let [_a, _b] = [3, 4];
    a = _a;
    b = _b;
}

{
    let Struct { x: _a, y: _b } = Struct { x: 3, y: 4};
    a = _a;
    b = _b;
}
}
```

[expr.assign.destructure.repeat-ident](#r-expr.assign.destructure.repeat-ident)

Identifiers are not forbidden from being used multiple times in a single assignee expression.

[expr.assign.destructure.discard-value](#r-expr.assign.destructure.discard-value)

[Underscore expressions](underscore-expr.md) and empty [range expressions](range-expr.md) may be used to ignore certain values, without binding them.

[expr.assign.destructure.default-binding](#r-expr.assign.destructure.default-binding)

Note that default binding modes do not apply for the desugared expression.

[expr.assign.destructure.tmp-scopes](#r-expr.assign.destructure.tmp-scopes)

> ![](../../assets/inline/operator-expr-26.svg)Note
>
> The desugaring restricts the [temporary scope](../destructors.md#r-destructors.scope.temporary) of the assigned value operand (the RHS) of a destructuring assignment.
>
> In a basic assignment, the [temporary](../expressions.md#r-expr.temporary) is dropped at the end of the enclosing temporary scope. Below, that’s the statement. Therefore, the assignment and use is allowed.
>
> ``` playground
> #![allow(unused)]
> fn main() {
> fn temp() {}
> fn f(x: T) -> T { x }
> let x;
> (x = f(&temp()), x); // OK
> }
> ```
>
> Conversely, in a destructuring assignment, the temporary is dropped at the end of the `let` statement in the desugaring. As that happens before we try to assign to `x`, below, it fails.
>
> ``` playground
> #![allow(unused)]
> fn main() {
> fn temp() {}
> fn f(x: T) -> T { x }
> let x;
> [x] = [f(&temp())]; // ERROR
> }
> ```
>
> This desugars to:
>
> ``` playground
> #![allow(unused)]
> fn main() {
> fn temp() {}
> fn f(x: T) -> T { x }
> let x;
> {
>     let [_x] = [f(&temp())];
>     //                     ^
>     //      The temporary is dropped here.
>     x = _x; // ERROR
> }
> }
> ```

[expr.assign.destructure.tmp-ext](#r-expr.assign.destructure.tmp-ext)

> ![](../../assets/inline/operator-expr-27.svg)Note
>
> Due to the desugaring, the assigned value operand (the RHS) of a destructuring assignment is an [extending expression](../destructors.md#r-destructors.scope.lifetime-extension.exprs) within a newly-introduced block.
>
> Below, because the [temporary scope](../destructors.md#r-destructors.scope.temporary) is extended to the end of this introduced block, the assignment is allowed.
>
> ``` playground
> #![allow(unused)]
> fn main() {
> fn temp() {}
> let x;
> [x] = [&temp()]; // OK
> }
> ```
>
> This desugars to:
>
> ``` playground
> #![allow(unused)]
> fn main() {
> fn temp() {}
> let x;
> { let [_x] = [&temp()]; x = _x; } // OK
> }
> ```
>
> However, if we try to use `x`, even within the same statement, we’ll get an error because the [temporary](../expressions.md#r-expr.temporary) is dropped at the end of this introduced block.
>
> ``` playground
> #![allow(unused)]
> fn main() {
> fn temp() {}
> let x;
> ([x] = [&temp()], x); // ERROR
> }
> ```
>
> This desugars to:
>
> ``` playground
> #![allow(unused)]
> fn main() {
> fn temp() {}
> let x;
> (
>     {
>         let [_x] = [&temp()];
>         x = _x;
>     }, // <-- The temporary is dropped here.
>     x, // ERROR
> );
> }
> ```

[expr.compound-assign](#r-expr.compound-assign)

## Compound assignment expressions

[expr.compound-assign.syntax](#r-expr.compound-assign.syntax)

**Syntax**\
[CompoundAssignmentExpression](#railroad-CompoundAssignmentExpression) →\
      [Expression](../expressions.md#grammar-Expression) += [Expression](../expressions.md#grammar-Expression)\
    \| [Expression](../expressions.md#grammar-Expression) -= [Expression](../expressions.md#grammar-Expression)\
    \| [Expression](../expressions.md#grammar-Expression) \*= [Expression](../expressions.md#grammar-Expression)\
    \| [Expression](../expressions.md#grammar-Expression) /= [Expression](../expressions.md#grammar-Expression)\
    \| [Expression](../expressions.md#grammar-Expression) %= [Expression](../expressions.md#grammar-Expression)\
    \| [Expression](../expressions.md#grammar-Expression) &= [Expression](../expressions.md#grammar-Expression)\
    \| [Expression](../expressions.md#grammar-Expression) \|= [Expression](../expressions.md#grammar-Expression)\
    \| [Expression](../expressions.md#grammar-Expression) ^= [Expression](../expressions.md#grammar-Expression)\
    \| [Expression](../expressions.md#grammar-Expression) \<\<= [Expression](../expressions.md#grammar-Expression)\
    \| [Expression](../expressions.md#grammar-Expression) \>\>= [Expression](../expressions.md#grammar-Expression)

Show Railroad

![](../../assets/inline/operator-expr-28.svg) [expr.compound-assign.intro](#r-expr.compound-assign.intro)

*Compound assignment expressions* combine arithmetic and logical binary operators with assignment expressions.

For example:

``` playground
#![allow(unused)]
fn main() {
let mut x = 5;
x += 1;
assert!(x == 6);
}
```

The syntax of compound assignment is a [mutable](../expressions.md#mutability) [place expression](../expressions.md#place-expressions-and-value-expressions), the *assigned operand*, then one of the operators followed by an `=` as a single token (no whitespace), and then a [value expression](../expressions.md#place-expressions-and-value-expressions), the *modifying operand*.

[expr.compound-assign.place](#r-expr.compound-assign.place)

Unlike other place operands, the assigned place operand must be a place expression.

[expr.compound-assign.no-value](#r-expr.compound-assign.no-value)

Attempting to use a value expression is a compiler error rather than promoting it to a temporary.

[expr.compound-assign.operand-order](#r-expr.compound-assign.operand-order)

Evaluation of compound assignment expressions depends on the types of the operands.

[expr.compound-assign.primitives](#r-expr.compound-assign.primitives)

If the types of both operands are known, prior to monomorphization, to be primitive, the right hand side is evaluated first, the left hand side is evaluated next, and the place given by the evaluation of the left hand side is mutated by applying the operator to the values of both sides.

``` playground
use core::{num::Wrapping, ops::AddAssign};

trait Equate {}
impl<T> Equate for (T, T) {}

fn f1(x: (u8,)) {
    let mut order = vec![];
    // The RHS is evaluated first as both operands are of primitive
    // type.
    { order.push(2); x }.0 += { order.push(1); x }.0;
    assert!(order.is_sorted());
}

fn f2(x: (Wrapping<u8>,)) {
    let mut order = vec![];
    // The LHS is evaluated first as `Wrapping<_>` is not a primitive
    // type.
    { order.push(1); x }.0 += { order.push(2); (0u8,) }.0;
    assert!(order.is_sorted());
}

fn f3<T: AddAssign<u8> + Copy>(x: (T,)) where (T, u8): Equate {
    let mut order = vec![];
    // The LHS is evaluated first as one of the operands is a generic
    // parameter, even though that generic parameter can be unified
    // with a primitive type due to the where clause bound.
    { order.push(1); x }.0 += { order.push(2); (0u8,) }.0;
    assert!(order.is_sorted());
}

fn main() {
    f1((0u8,));
    f2((Wrapping(0u8),));
    // We supply a primitive type as the generic argument, but this
    // does not affect the evaluation order in `f3` when
    // monomorphized.
    f3::<u8>((0u8,));
}
```

> ![](../../assets/inline/operator-expr-29.svg)Note
>
> This is unusual. Elsewhere left to right evaluation is the norm.
>
> See the [eval order test](https://github.com/rust-lang/rust/blob/1.58.0/src/test/ui/expr/compound-assignment/eval-order.rs) for more examples.

[expr.compound-assign.trait](#r-expr.compound-assign.trait)

Otherwise, this expression is syntactic sugar for using the corresponding trait for the operator (see [expr.arith-logic.behavior](#r-expr.arith-logic.behavior)) and calling its method with the left hand side as the [receiver](method-call-expr.md#r-expr.method.intro) and the right hand side as the next argument.

For example, the following two statements are equivalent:

``` playground
#![allow(unused)]
fn main() {
use std::ops::AddAssign;
fn f<T: AddAssign + Copy>(mut x: T, y: T) {
    x += y; // Statement 1.
    x.add_assign(y); // Statement 2.
}
}
```

> ![](../../assets/inline/operator-expr-30.svg)Note
>
> Surprisingly, desugaring this further to a fully qualified method call is not equivalent, as there is special borrow checker behavior when the mutable reference to the first operand is taken via [autoref](method-call-expr.md#r-expr.method.candidate-receivers-refs).
>
> ``` playground
> #![allow(unused)]
> fn main() {
> use std::ops::AddAssign;
> fn f(mut x: T) {
>     // Here we used `x` as both the LHS and the RHS. Because the
>     // mutable borrow of the LHS needed to call the trait method
>     // is taken implicitly by autoref, this is OK.
>     x += x; //~ OK
>     x.add_assign(x); //~ OK
> }
> }
> ```
>
> ``` playground
> #![allow(unused)]
> fn main() {
> use std::ops::AddAssign;
> fn f(mut x: T) {
>     // We can't desugar the above to the below, as once we take the
>     // mutable borrow of `x` to pass the first argument, we can't
>     // pass `x` by value in the second argument because the mutable
>     // reference is still live.
>     ::add_assign(&mut x, x);
>     //~^ ERROR cannot use `x` because it was mutably borrowed
> }
> }
> ```
>
> ``` playground
> #![allow(unused)]
> fn main() {
> use std::ops::AddAssign;
> fn f(mut x: T) {
>     // As above.
>     (&mut x).add_assign(x);
>     //~^ ERROR cannot use `x` because it was mutably borrowed
> }
> }
> ```

[expr.compound-assign.result](#r-expr.compound-assign.result)

As with normal assignment expressions, compound assignment expressions always produce [the unit value](../types/tuple.md).

> ![](../../assets/inline/operator-expr-31.svg)Warning
>
> Avoid writing code that depends on the evaluation order of operands in compound assignments as it can be unusual and surprising.

------------------------------------------------------------------------

1.  

    Only when `m₁` is `mut` or `m₂` is `const`. Casting `mut` reference/pointer to `const` pointer is allowed. [↩](#fr-lessmut-1) [↩2](#fr-lessmut-2)

    

2.  

    Only closures that do not capture (close over) any local variables can be cast to function pointers. [↩](#fr-no-capture-1)
