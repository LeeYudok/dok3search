# Loops and other breakable expressions

[expr.loop](#r-expr.loop)

# Loops and other breakable expressions

[expr.loop.syntax](#r-expr.loop.syntax)

**Syntax**\
[LoopExpression](#railroad-LoopExpression) →\
    [LoopLabel](#grammar-LoopLabel)? (\
        [InfiniteLoopExpression](#grammar-InfiniteLoopExpression)\
      \| [PredicateLoopExpression](#grammar-PredicateLoopExpression)\
      \| [IteratorLoopExpression](#grammar-IteratorLoopExpression)\
      \| [LabelBlockExpression](#grammar-LabelBlockExpression)\
    )

Show Railroad

![](../../assets/inline/loop-expr-1.svg) [expr.loop.intro](#r-expr.loop.intro)

Rust supports four loop expressions:

- A [`loop` expression](#infinite-loops) denotes an infinite loop.
- A [`while` expression](#predicate-loops) loops until a predicate is false.
- A [`for` expression](#iterator-loops) extracts values from an iterator, looping until the iterator is empty.
- A [labeled block expression](#r-expr.loop.block-labels) runs a loop exactly once, but allows exiting the loop early with `break`.

[expr.loop.break-label](#r-expr.loop.break-label)

All four types of loop support [`break` expressions](#break-expressions), and [labels](#loop-labels).

[expr.loop.continue-label](#r-expr.loop.continue-label)

All except labeled block expressions support [`continue` expressions](#continue-expressions).

[expr.loop.explicit-result](#r-expr.loop.explicit-result)

Only `loop` and labeled block expressions support [evaluation to non-trivial values](#break-and-loop-values).

[expr.loop.infinite](#r-expr.loop.infinite)

## Infinite loops

[expr.loop.infinite.syntax](#r-expr.loop.infinite.syntax)

**Syntax**\
[InfiniteLoopExpression](#railroad-InfiniteLoopExpression) → loop [BlockExpression](block-expr.md#grammar-BlockExpression)

Show Railroad

![](../../assets/inline/loop-expr-2.svg) [expr.loop.infinite.intro](#r-expr.loop.infinite.intro)

A `loop` expression repeats execution of its body continuously: `loop { println!("I live."); }`.

[expr.loop.infinite.diverging](#r-expr.loop.infinite.diverging)

A `loop` expression without an associated `break` expression is [diverging](../divergence.md#r-divergence) and has type [`!`](../types/never.md#r-type.never).

[expr.loop.infinite.break](#r-expr.loop.infinite.break)

A `loop` expression containing associated [`break` expression(s)](#break-expressions) may terminate, and must have type compatible with the value of the `break` expression(s).

[expr.loop.while](#r-expr.loop.while)

## Predicate loops

[expr.loop.while.syntax](#r-expr.loop.while.syntax)

**Syntax**\
[PredicateLoopExpression](#railroad-PredicateLoopExpression) → while [Conditions](if-expr.md#grammar-Conditions) [BlockExpression](block-expr.md#grammar-BlockExpression)

Show Railroad

![](../../assets/inline/loop-expr-3.svg) [expr.loop.while.intro](#r-expr.loop.while.intro)

A `while` loop expression allows repeating the evaluation of a block while a set of conditions remain true.

[expr.loop.while.condition](#r-expr.loop.while.condition)

Condition operands must be either an [Expression](../expressions.md#grammar-Expression) with a [boolean type](../types/boolean.md) or a conditional `let` match. If all of the condition operands evaluate to `true` and all of the `let` patterns successfully match their [scrutinee](../glossary.md#scrutinee)s, then the loop body block executes.

[expr.loop.while.repeat](#r-expr.loop.while.repeat)

After the loop body successfully executes, the condition operands are re-evaluated to determine if the body should be executed again.

[expr.loop.while.exit](#r-expr.loop.while.exit)

If any condition operand evaluates to `false` or any `let` pattern does not match its scrutinee, the body is not executed and execution continues after the `while` expression.

[expr.loop.while.eval](#r-expr.loop.while.eval)

A `while` expression evaluates to `()`.

An example:

``` playground
#![allow(unused)]
fn main() {
let mut i = 0;

while i < 10 {
    println!("hello");
    i = i + 1;
}
}
```

[expr.loop.while.let](#r-expr.loop.while.let)

### `while let` patterns

[expr.loop.while.let.intro](#r-expr.loop.while.let.intro)

`let` patterns in a `while` condition allow binding new variables into scope when the pattern matches successfully. The following examples illustrate bindings using `let` patterns:

``` playground
#![allow(unused)]
fn main() {
let mut x = vec![1, 2, 3];

while let Some(y) = x.pop() {
    println!("y = {}", y);
}

while let _ = 5 {
    println!("Irrefutable patterns are always true");
    break;
}
}
```

[expr.loop.while.let.desugar](#r-expr.loop.while.let.desugar)

A `while let` loop is equivalent to a `loop` expression containing a [`match` expression](match-expr.md) as follows.

``` rust
'label: while let PATS = EXPR {
    /* loop body */
}
```

is equivalent to

``` rust
'label: loop {
    match EXPR {
        PATS => { /* loop body */ },
        _ => break,
    }
}
```

[expr.loop.while.let.or-pattern](#r-expr.loop.while.let.or-pattern)

Multiple patterns may be specified with the `|` operator. This has the same semantics as with `|` in `match` expressions:

``` playground
#![allow(unused)]
fn main() {
let mut vals = vec![2, 3, 1, 2, 2];
while let Some(v @ 1) | Some(v @ 2) = vals.pop() {
    // Prints 2, 2, then 1
    println!("{}", v);
}
}
```

[expr.loop.while.chains](#r-expr.loop.while.chains)

### `while` condition chains

[expr.loop.while.chains.intro](#r-expr.loop.while.chains.intro)

Multiple condition operands can be separated with `&&`. These have the same semantics and restrictions as [`if` condition chains](if-expr.md#chains-of-conditions).

The following is an example of chaining multiple expressions, mixing `let` bindings and boolean expressions, and with expressions able to reference pattern bindings from previous expressions:

``` playground
fn main() {
    let outer_opt = Some(Some(1i32));

    while let Some(inner_opt) = outer_opt
        && let Some(number) = inner_opt
        && number == 1
    {
        println!("Peek a boo");
        break;
    }
}
```

[expr.loop.for](#r-expr.loop.for)

## Iterator loops

[expr.loop.for.syntax](#r-expr.loop.for.syntax)

**Syntax**\
[IteratorLoopExpression](#railroad-IteratorLoopExpression) →\
    for [Pattern](../patterns.md#grammar-Pattern) in [Expression](../expressions.md#grammar-Expression)except [StructExpression](struct-expr.md#grammar-StructExpression) [BlockExpression](block-expr.md#grammar-BlockExpression)

Show Railroad

![](../../assets/inline/loop-expr-4.svg) [expr.loop.for.intro](#r-expr.loop.for.intro)

A `for` expression is a syntactic construct for looping over elements provided by an implementation of `std::iter::IntoIterator`.

[expr.loop.for.condition](#r-expr.loop.for.condition)

If the iterator yields a value, that value is matched against the irrefutable pattern, the body of the loop is executed, and then control returns to the head of the `for` loop. If the iterator is empty, the `for` expression completes.

An example of a `for` loop over the contents of an array:

``` playground
#![allow(unused)]
fn main() {
let v = &["apples", "cake", "coffee"];

for text in v {
    println!("I like {}.", text);
}
}
```

An example of a for loop over a series of integers:

``` playground
#![allow(unused)]
fn main() {
let mut sum = 0;
for n in 1..11 {
    sum += n;
}
assert_eq!(sum, 55);
}
```

[expr.loop.for.desugar](#r-expr.loop.for.desugar)

A `for` loop is equivalent to a `loop` expression containing a [`match` expression](match-expr.md) as follows:

``` rust
'label: for PATTERN in iter_expr {
    /* loop body */
}
```

is equivalent to

``` rust
{
    let result = match IntoIterator::into_iter(iter_expr) {
        mut iter => 'label: loop {
            let mut next;
            match Iterator::next(&mut iter) {
                Option::Some(val) => next = val,
                Option::None => break,
            };
            let PATTERN = next;
            let () = { /* loop body */ };
        },
    };
    result
}
```

[expr.loop.for.lang-items](#r-expr.loop.for.lang-items)

`IntoIterator`, `Iterator`, and `Option` are always the standard library items here, not whatever those names resolve to in the current scope.

The variable names `next`, `iter`, and `val` are for exposition only, they do not actually have names the user can type.

> ![](../../assets/inline/loop-expr-5.svg)Note
>
> The outer `match` is used to ensure that any [temporary values](../expressions.md#temporaries) in `iter_expr` don’t get dropped before the loop is finished. `next` is declared before being assigned because it results in types being inferred correctly more often.

[expr.loop.label](#r-expr.loop.label)

## Loop labels

[expr.loop.label.syntax](#r-expr.loop.label.syntax)

**Syntax**\
[LoopLabel](#railroad-LoopLabel) → [LIFETIME_OR_LABEL](../tokens.md#grammar-LIFETIME_OR_LABEL) :

Show Railroad

![](../../assets/inline/loop-expr-6.svg) [expr.loop.label.intro](#r-expr.loop.label.intro)

A loop expression may optionally have a *label*. The label is written as a lifetime preceding the loop expression, as in `'foo: loop { break 'foo; }`, `'bar: while false {}`, `'humbug: for _ in 0..0 {}`.

[expr.loop.label.control-flow](#r-expr.loop.label.control-flow)

If a label is present, then labeled `break` and `continue` expressions nested within this loop may exit out of this loop or return control to its head. See [break expressions](#break-expressions) and [continue expressions](#continue-expressions).

[expr.loop.label.ref](#r-expr.loop.label.ref)

Labels follow the hygiene and shadowing rules of local variables. For example, this code will print “outer loop”:

``` playground
#![allow(unused)]
fn main() {
'a: loop {
    'a: loop {
        break 'a;
    }
    print!("outer loop");
    break 'a;
}
}
```

`'_` is not a valid loop label.

[expr.loop.break](#r-expr.loop.break)

## `break` expressions

[expr.loop.break.syntax](#r-expr.loop.break.syntax)

**Syntax**\
[BreakExpression](#railroad-BreakExpression) → break [LIFETIME_OR_LABEL](../tokens.md#grammar-LIFETIME_OR_LABEL)? [Expression](../expressions.md#grammar-Expression)?

Show Railroad

![](../../assets/inline/loop-expr-7.svg) [expr.loop.break.intro](#r-expr.loop.break.intro)

When `break` is encountered, execution of the associated loop body is immediately terminated, for example:

``` playground
#![allow(unused)]
fn main() {
let mut last = 0;
for x in 1..100 {
    if x > 12 {
        break;
    }
    last = x;
}
assert_eq!(last, 12);
}
```

[expr.loop.break.diverging](#r-expr.loop.break.diverging)

A `break` expression is [diverging](../divergence.md#r-divergence) and has a type of [`!`](../types/never.md#r-type.never).

[expr.loop.break.label](#r-expr.loop.break.label)

A `break` expression is normally associated with the innermost `loop`, `for` or `while` loop enclosing the `break` expression, but a [label](#loop-labels) can be used to specify which enclosing loop is affected. Example:

``` playground
#![allow(unused)]
fn main() {
'outer: loop {
    while true {
        break 'outer;
    }
}
}
```

[expr.loop.break.value](#r-expr.loop.break.value)

A `break` expression is only permitted in the body of a loop, and has one of the forms `break`, `break 'label` or ([see below](#break-and-loop-values)) `break EXPR` or `break 'label EXPR`.

[expr.loop.break-value.implicit-value](#r-expr.loop.break-value.implicit-value)

In a [`loop` with break expressions](#r-expr.loop.break-value) or a [labeled block expression](#r-expr.loop.block-labels), a `break` without an expression is equivalent to `break ()`.

[expr.loop.block-labels](#r-expr.loop.block-labels)

## Labeled block expressions

[expr.loop.block-labels.syntax](#r-expr.loop.block-labels.syntax)

**Syntax**\
[LabelBlockExpression](#railroad-LabelBlockExpression) → [BlockExpression](block-expr.md#grammar-BlockExpression)

Show Railroad

![](../../assets/inline/loop-expr-8.svg) [expr.loop.block-labels.intro](#r-expr.loop.block-labels.intro)

Labeled block expressions are exactly like block expressions, except that they allow using `break` expressions within the block.

[expr.loop.block-labels.break](#r-expr.loop.block-labels.break)

Unlike loops, `break` expressions within a labeled block expression *must* have a label (i.e. the label is not optional).

[expr.loop.block-labels.label-required](#r-expr.loop.block-labels.label-required)

Similarly, labeled block expressions *must* begin with a label.

``` playground
#![allow(unused)]
fn main() {
fn do_thing() {}
fn condition_not_met() -> bool { true }
fn do_next_thing() {}
fn do_last_thing() {}
let result = 'block: {
    do_thing();
    if condition_not_met() {
        break 'block 1;
    }
    do_next_thing();
    if condition_not_met() {
        break 'block 2;
    }
    do_last_thing();
    3
};
}
```

[expr.loop.block-labels.type](#r-expr.loop.block-labels.type)

The type of a labeled block expression is the [least upper bound](../type-coercions.md#r-coerce.least-upper-bound) of all of the break operands and the final operand. If the final operand is omitted, the type of the final operand defaults to the [unit type](../types/tuple.md#r-type.tuple.unit), unless the block [diverges](block-expr.md#r-expr.block.diverging), in which case it is the [never type](../types/never.md#r-type.never).

> ![](../../assets/inline/loop-expr-9.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> fn example(condition: bool) {
>     let s = String::from("owned");
>
>     let _: &str = 'block: {
>         if condition {
>             break 'block &s;  // &String coerced to &str via Deref
>         }
>         break 'block "literal";  // &'static str coerced to &str
>     };
> }
> }
> ```

[expr.loop.continue](#r-expr.loop.continue)

## `continue` expressions

[expr.loop.continue.syntax](#r-expr.loop.continue.syntax)

**Syntax**\
[ContinueExpression](#railroad-ContinueExpression) → continue [LIFETIME_OR_LABEL](../tokens.md#grammar-LIFETIME_OR_LABEL)?

Show Railroad

![](../../assets/inline/loop-expr-10.svg) [expr.loop.continue.intro](#r-expr.loop.continue.intro)

When `continue` is encountered, the current iteration of the associated loop body is immediately terminated, returning control to the loop *head*.

[expr.loop.continue.diverging](#r-expr.loop.continue.diverging)

A `continue` expression is [diverging](../divergence.md#r-divergence) and has a type of [`!`](../types/never.md#r-type.never).

[expr.loop.continue.while](#r-expr.loop.continue.while)

In the case of a `while` loop, the head is the conditional operands controlling the loop.

[expr.loop.continue.for](#r-expr.loop.continue.for)

In the case of a `for` loop, the head is the call-expression controlling the loop.

[expr.loop.continue.label](#r-expr.loop.continue.label)

Like `break`, `continue` is normally associated with the innermost enclosing loop, but `continue 'label` may be used to specify the loop affected.

[expr.loop.continue.in-loop-only](#r-expr.loop.continue.in-loop-only)

A `continue` expression is only permitted in the body of a loop.

[expr.loop.break-value](#r-expr.loop.break-value)

## `break` and loop values

[expr.loop.break-value.intro](#r-expr.loop.break-value.intro)

When associated with a `loop`, a break expression may be used to return a value from that loop, via one of the forms `break EXPR` or `break 'label EXPR`, where `EXPR` is an expression whose result is returned from the `loop`. For example:

``` playground
#![allow(unused)]
fn main() {
let (mut a, mut b) = (1, 1);
let result = loop {
    if b > 10 {
        break b;
    }
    let c = a + b;
    a = b;
    b = c;
};
// first number in Fibonacci sequence over 10:
assert_eq!(result, 13);
}
```

[expr.loop.break-value.type](#r-expr.loop.break-value.type)

The type of a `loop` with associated `break` expressions is the [least upper bound](../type-coercions.md#r-coerce.least-upper-bound) of all of the break operands.

> ![](../../assets/inline/loop-expr-11.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> fn example(condition: bool) {
>     let s = String::from("owned");
>
>     let _: &str = loop {
>         if condition {
>             break &s; // &String coerced to &str via Deref
>         }
>         break "literal"; // &'static str coerced to &str
>     };
> }
> }
> ```

[expr.loop.break-value.diverging](#r-expr.loop.break-value.diverging)

A `loop` with associated `break` expressions does not [diverge](../divergence.md#r-divergence) if any of the break operands do not diverge. If all of the `break` operands diverge, then the `loop` expression also diverges.

> ![](../../assets/inline/loop-expr-12.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> fn diverging_loop_with_break(condition: bool) -> ! {
>     // This loop is diverging because all `break` operands are diverging.
>     loop {
>         if condition {
>             break loop {};
>         } else {
>             break panic!();
>         }
>     }
> }
> }
> ```
>
> ``` playground
> #![allow(unused)]
> fn main() {
> fn loop_with_non_diverging_break(condition: bool) -> ! {
>     // The type of this loop is i32 even though one of the breaks is
>     // diverging.
>     loop {
>         if condition {
>             break loop {};
>         } else {
>             break 123i32;
>         }
>     } // ERROR: expected `!`, found `i32`
> }
> }
> ```
