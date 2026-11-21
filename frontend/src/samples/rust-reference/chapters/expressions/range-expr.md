# Range expressions

[expr.range](#r-expr.range)

# Range expressions

[expr.range.syntax](#r-expr.range.syntax)

**Syntax**\
[RangeExpression](#railroad-RangeExpression) →\
      [RangeExpr](#grammar-RangeExpr)\
    \| [RangeFromExpr](#grammar-RangeFromExpr)\
    \| [RangeToExpr](#grammar-RangeToExpr)\
    \| [RangeFullExpr](#grammar-RangeFullExpr)\
    \| [RangeInclusiveExpr](#grammar-RangeInclusiveExpr)\
    \| [RangeToInclusiveExpr](#grammar-RangeToInclusiveExpr)

[RangeExpr](#railroad-RangeExpr) → [Expression](../expressions.md#grammar-Expression) .. [Expression](../expressions.md#grammar-Expression)

[RangeFromExpr](#railroad-RangeFromExpr) → [Expression](../expressions.md#grammar-Expression) ..

[RangeToExpr](#railroad-RangeToExpr) → .. [Expression](../expressions.md#grammar-Expression)

[RangeFullExpr](#railroad-RangeFullExpr) → ..

[RangeInclusiveExpr](#railroad-RangeInclusiveExpr) → [Expression](../expressions.md#grammar-Expression) ..= [Expression](../expressions.md#grammar-Expression)

[RangeToInclusiveExpr](#railroad-RangeToInclusiveExpr) → ..= [Expression](../expressions.md#grammar-Expression)

Show Railroad

![](../../assets/inline/range-expr-1.svg) ![](../../assets/inline/range-expr-2.svg) ![](../../assets/inline/range-expr-3.svg) ![](../../assets/inline/range-expr-4.svg) ![](../../assets/inline/range-expr-5.svg) ![](../../assets/inline/range-expr-6.svg) ![](../../assets/inline/range-expr-7.svg) [expr.range.behavior](#r-expr.range.behavior)

The `..` and `..=` operators will construct an object of one of the `std::ops::Range` (or `core::ops::Range`) variants, according to the following table:

| Production | Syntax | Type | Range |
|----|----|----|----|
| [RangeExpr](#grammar-RangeExpr) | start`..`end | [std::ops::Range](../../core/ops/range/struct.Range.html) | start ≤ x \< end |
| [RangeFromExpr](#grammar-RangeFromExpr) | start`..` | [std::ops::RangeFrom](../../core/ops/range/struct.RangeFrom.html) | start ≤ x |
| [RangeToExpr](#grammar-RangeToExpr) | `..`end | [std::ops::RangeTo](../../core/ops/range/struct.RangeTo.html) | x \< end |
| [RangeFullExpr](#grammar-RangeFullExpr) | `..` | [std::ops::RangeFull](../../core/ops/range/struct.RangeFull.html) | \- |
| [RangeInclusiveExpr](#grammar-RangeInclusiveExpr) | start`..=`end | [std::ops::RangeInclusive](../../core/ops/range/struct.RangeInclusive.html) | start ≤ x ≤ end |
| [RangeToInclusiveExpr](#grammar-RangeToInclusiveExpr) | `..=`end | [std::ops::RangeToInclusive](../../core/ops/range/struct.RangeToInclusive.html) | x ≤ end |

Examples:

``` playground
#![allow(unused)]
fn main() {
1..2;   // std::ops::Range
3..;    // std::ops::RangeFrom
..4;    // std::ops::RangeTo
..;     // std::ops::RangeFull
5..=6;  // std::ops::RangeInclusive
..=7;   // std::ops::RangeToInclusive
}
```

[expr.range.equivalence](#r-expr.range.equivalence)

The following expressions are equivalent.

``` playground
#![allow(unused)]
fn main() {
let x = std::ops::Range {start: 0, end: 10};
let y = 0..10;

assert_eq!(x, y);
}
```

[expr.range.for](#r-expr.range.for)

Ranges can be used in `for` loops:

``` playground
#![allow(unused)]
fn main() {
for i in 1..11 {
    println!("{}", i);
}
}
```
