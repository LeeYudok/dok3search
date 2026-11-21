# Path expressions

[expr.path](#r-expr.path)

# Path expressions

[expr.path.syntax](#r-expr.path.syntax)

**Syntax**\
[PathExpression](#railroad-PathExpression) →\
      [PathInExpression](../paths.md#grammar-PathInExpression)\
    \| [QualifiedPathInExpression](../paths.md#grammar-QualifiedPathInExpression)

Show Railroad

![](../../assets/inline/path-expr-1.svg) [expr.path.intro](#r-expr.path.intro)

A [path](../paths.md) used as an expression context denotes either a local variable or an item.

[expr.path.place](#r-expr.path.place)

Path expressions that resolve to local or static variables are [place expressions](../expressions.md#place-expressions-and-value-expressions); other paths are [value expressions](../expressions.md#place-expressions-and-value-expressions).

[expr.path.safety](#r-expr.path.safety)

Using a [`static mut`](../items/static-items.md#mutable-statics) variable requires an [`unsafe` block](block-expr.md#unsafe-blocks).

``` playground
#![allow(unused)]
fn main() {
mod globals {
    pub static STATIC_VAR: i32 = 5;
    pub static mut STATIC_MUT_VAR: i32 = 7;
}
let local_var = 3;
local_var;
globals::STATIC_VAR;
unsafe { globals::STATIC_MUT_VAR };
let some_constructor = Some::<i32>;
let push_integer = Vec::<i32>::push;
let slice_reverse = <[i32]>::reverse;
}
```

[expr.path.const](#r-expr.path.const)

Evaluation of associated constants is handled the same way as [`const` blocks](block-expr.md#const-blocks).
