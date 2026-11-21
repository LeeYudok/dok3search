# if expressions

[expr.if](#r-expr.if)

# `if` expressions

[expr.if.syntax](#r-expr.if.syntax)

**Syntax**\
[IfExpression](#railroad-IfExpression) →\
    if [Conditions](#grammar-Conditions) [BlockExpressionNoInnerAttributes](block-expr.md#grammar-BlockExpressionNoInnerAttributes)\
    ( else ( [BlockExpressionNoInnerAttributes](block-expr.md#grammar-BlockExpressionNoInnerAttributes) \| [IfExpression](#grammar-IfExpression) ) )?

[Conditions](#railroad-Conditions) →\
      [Expression](../expressions.md#grammar-Expression)except [StructExpression](struct-expr.md#grammar-StructExpression)\
    \| [LetChain](#grammar-LetChain)

[LetChain](#railroad-LetChain) → [LetChainCondition](#grammar-LetChainCondition) ( && [LetChainCondition](#grammar-LetChainCondition) )\*

[LetChainCondition](#railroad-LetChainCondition) →\
      [Expression](../expressions.md#grammar-Expression)except [ExcludedConditions](#grammar-ExcludedConditions)\
    \| [OuterAttribute](../attributes.md#grammar-OuterAttribute)\* let [Pattern](../patterns.md#grammar-Pattern) = [Scrutinee](match-expr.md#grammar-Scrutinee)except [ExcludedConditions](#grammar-ExcludedConditions)

[ExcludedConditions](#railroad-ExcludedConditions) →\
      [StructExpression](struct-expr.md#grammar-StructExpression)\
    \| [LazyBooleanExpression](operator-expr.md#grammar-LazyBooleanExpression)\
    \| [RangeExpr](range-expr.md#grammar-RangeExpr)\
    \| [RangeFromExpr](range-expr.md#grammar-RangeFromExpr)\
    \| [RangeInclusiveExpr](range-expr.md#grammar-RangeInclusiveExpr)\
    \| [AssignmentExpression](operator-expr.md#grammar-AssignmentExpression)\
    \| [CompoundAssignmentExpression](operator-expr.md#grammar-CompoundAssignmentExpression)

Show Railroad

![](../../assets/inline/if-expr-1.svg) ![](../../assets/inline/if-expr-2.svg) ![](../../assets/inline/if-expr-3.svg) ![](../../assets/inline/if-expr-4.svg) ![](../../assets/inline/if-expr-5.svg) [expr.if.intro](#r-expr.if.intro)

The syntax of an `if` expression is a sequence of one or more condition operands separated by `&&`, followed by a consequent block, any number of `else if` conditions and blocks, and an optional trailing `else` block.

[expr.if.condition](#r-expr.if.condition)

Condition operands must be either an [Expression](../expressions.md#grammar-Expression) with a [boolean type](../types/boolean.md) or a conditional `let` match.

[expr.if.condition-true](#r-expr.if.condition-true)

If all of the condition operands evaluate to `true` and all of the `let` patterns successfully match their [scrutinee](../glossary.md#scrutinee)s, the consequent block is executed and any subsequent `else if` or `else` block is skipped.

[expr.if.else-if](#r-expr.if.else-if)

If any condition operand evaluates to `false` or any `let` pattern does not match its scrutinee, the consequent block is skipped and any subsequent `else if` condition is evaluated.

[expr.if.else](#r-expr.if.else)

If all `if` and `else if` conditions evaluate to `false` then any `else` block is executed.

[expr.if.result](#r-expr.if.result)

An `if` expression evaluates to the same value as the executed block, or `()` if no block is evaluated.

[expr.if.type](#r-expr.if.type)

An `if` expression must have the same type in all situations.

``` playground
#![allow(unused)]
fn main() {
let x = 3;
if x == 4 {
    println!("x is four");
} else if x == 3 {
    println!("x is three");
} else {
    println!("x is something else");
}

// `if` can be used as an expression.
let y = if 12 * 15 > 150 {
    "Bigger"
} else {
    "Smaller"
};
assert_eq!(y, "Bigger");
}
```

[expr.if.diverging](#r-expr.if.diverging)

An `if` expression [diverges](../divergence.md#r-divergence) if either the condition expression diverges or if all arms diverge.

``` playground
#![allow(unused)]
fn main() {
fn diverging_condition() -> ! {
    // Diverges because the condition expression diverges
    if loop {} {
        ()
    } else {
        ()
    };
    // The semicolon above is important: The type of the `if` expression is
    // `()`, despite being diverging. When the final body expression is
    // elided, the type of the body is inferred to ! because the function body
    // diverges. Without the semicolon, the `if` would be the tail expression
    // with type `()`, which would fail to match the return type `!`.
}

fn diverging_arms() -> ! {
    // Diverges because all arms diverge
    if true {
        loop {}
    } else {
        loop {}
    }
}
}
```

[expr.if.let](#r-expr.if.let)

## `if let` patterns

[expr.if.let.intro](#r-expr.if.let.intro)

`let` patterns in an `if` condition allow binding new variables into scope when the pattern matches successfully.

The following examples illustrate bindings using `let` patterns:

``` playground
#![allow(unused)]
fn main() {
let dish = ("Ham", "Eggs");

// This body will be skipped because the pattern is refuted.
if let ("Bacon", b) = dish {
    println!("Bacon is served with {}", b);
} else {
    // This block is evaluated instead.
    println!("No bacon will be served");
}

// This body will execute.
if let ("Ham", b) = dish {
    println!("Ham is served with {}", b);
}

if let _ = 5 {
    println!("Irrefutable patterns are always true");
}
}
```

[expr.if.let.or-pattern](#r-expr.if.let.or-pattern)

Multiple patterns may be specified with the `|` operator. This has the same semantics as with `|` in [`match` expressions](match-expr.md):

``` playground
#![allow(unused)]
fn main() {
enum E {
    X(u8),
    Y(u8),
    Z(u8),
}
let v = E::Y(12);
if let E::X(n) | E::Y(n) = v {
    assert_eq!(n, 12);
}
}
```

[expr.if.chains](#r-expr.if.chains)

## Chains of conditions

[expr.if.chains.intro](#r-expr.if.chains.intro)

Multiple condition operands can be separated with `&&`.

[expr.if.chains.order](#r-expr.if.chains.order)

Similar to a `&&` [LazyBooleanExpression](operator-expr.md#grammar-LazyBooleanExpression), each operand is evaluated from left-to-right until an operand evaluates as `false` or a `let` match fails, in which case the subsequent operands are not evaluated.

[expr.if.chains.bindings](#r-expr.if.chains.bindings)

The bindings of each pattern are put into scope to be available for the next condition operand and the consequent block.

The following is an example of chaining multiple expressions, mixing `let` bindings and boolean expressions, and with expressions able to reference pattern bindings from previous expressions:

``` playground
#![allow(unused)]
fn main() {
fn single() {
    let outer_opt = Some(Some(1i32));

    if let Some(inner_opt) = outer_opt
        && let Some(number) = inner_opt
        && number == 1
    {
        println!("Peek a boo");
    }
}
}
```

The above is equivalent to the following without using chains of conditions:

``` playground
#![allow(unused)]
fn main() {
fn nested() {
    let outer_opt = Some(Some(1i32));

    if let Some(inner_opt) = outer_opt {
        if let Some(number) = inner_opt {
            if number == 1 {
                println!("Peek a boo");
            }
        }
    }
}
}
```

[expr.if.chains.or](#r-expr.if.chains.or)

If any condition operand is a `let` pattern, then none of the condition operands can be a `||` [lazy boolean operator expression](operator-expr.md#r-expr.bool-logic) due to ambiguity and precedence with the `let` scrutinee.

> ![](../../assets/inline/if-expr-6.svg)Example
>
> If a `||` expression is needed, then parentheses can be used. For example:
>
> ``` playground
> #![allow(unused)]
> fn main() {
> let foo = Some(123);
> let condition1 = true;
> let condition2 = false;
> if let Some(x) = foo
>     // Parentheses are required here.
>     && (condition1 || condition2)
> {}
> }
> ```

[expr.if.edition2024](#r-expr.if.edition2024)

> 2024 Edition differences
>
> Before the 2024 edition, let chains are not supported. That is, the [LetChain](#grammar-LetChain) grammar is not allowed in an `if` expression.
