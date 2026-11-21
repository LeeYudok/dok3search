# Grouped expressions

[expr.paren](#r-expr.paren)

# Grouped expressions

[expr.paren.syntax](#r-expr.paren.syntax)

**Syntax**\
[GroupedExpression](#railroad-GroupedExpression) → ( [Expression](../expressions.md#grammar-Expression) )

Show Railroad

![](../../assets/inline/grouped-expr-1.svg) [expr.paren.intro](#r-expr.paren.intro)

A *parenthesized expression* wraps a single expression, evaluating to that expression. The syntax for a parenthesized expression is a `(`, then an expression, called the *enclosed operand*, and then a `)`.

[expr.paren.evaluation](#r-expr.paren.evaluation)

Parenthesized expressions evaluate to the value of the enclosed operand.

[expr.paren.place-or-value](#r-expr.paren.place-or-value)

A parenthesized expression is a [place expression](../expressions.md#place-expressions-and-value-expressions) if the enclosed operand is a place expression, and is a value expression if the enclosed operand is a value expression.

[expr.paren.override-precedence](#r-expr.paren.override-precedence)

Parentheses can be used to explicitly modify the precedence order of subexpressions within an expression.

An example of a parenthesized expression:

``` playground
#![allow(unused)]
fn main() {
let x: i32 = 2 + 3 * 4; // not parenthesized
let y: i32 = (2 + 3) * 4; // parenthesized
assert_eq!(x, 14);
assert_eq!(y, 20);
}
```

An example of a necessary use of parentheses is when calling a function pointer that is a member of a struct:

``` playground
#![allow(unused)]
fn main() {
struct A {
   f: fn() -> &'static str
}
impl A {
   fn f(&self) -> &'static str {
       "The method f"
   }
}
let a = A{f: || "The field f"};

assert_eq!( a.f (), "The method f");
assert_eq!((a.f)(), "The field f");
}
```
