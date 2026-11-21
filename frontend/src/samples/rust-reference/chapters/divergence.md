# Divergence

[divergence](#r-divergence)

# Divergence

[divergence.intro](#r-divergence.intro)

A *diverging expression* is an expression that never completes normal execution.

``` playground
#![allow(unused)]
fn main() {
fn diverges() -> ! {
    panic!("This function never returns!");
}

fn example() {
    let x: i32 = diverges(); // This line never completes.
    println!("This is never printed: {x}");
}
}
```

See the following rules for specific expression divergence behavior:

- [expr.block.diverging](expressions/block-expr.md#r-expr.block.diverging) — Block expressions.
- [expr.if.diverging](expressions/if-expr.md#r-expr.if.diverging) — `if` expressions.
- [expr.loop.block-labels.type](expressions/loop-expr.md#r-expr.loop.block-labels.type) — Labeled block expressions with `break`.
- [expr.loop.break-value.diverging](expressions/loop-expr.md#r-expr.loop.break-value.diverging) — `loop` expressions with `break`.
- [expr.loop.break.diverging](expressions/loop-expr.md#r-expr.loop.break.diverging) — `break` expressions.
- [expr.loop.continue.diverging](expressions/loop-expr.md#r-expr.loop.continue.diverging) — `continue` expressions.
- [expr.loop.infinite.diverging](expressions/loop-expr.md#r-expr.loop.infinite.diverging) — Infinite `loop` expressions.
- [expr.match.diverging](expressions/match-expr.md#r-expr.match.diverging) — `match` expressions.
- [expr.match.empty](expressions/match-expr.md#r-expr.match.empty) — Empty `match` expressions.
- [expr.return.diverging](expressions/return-expr.md#r-expr.return.diverging) — `return` expressions.

> ![](../assets/inline/divergence-1.svg)Note
>
> The [`panic!`](https://doc.rust-lang.org/1.100.0/std/macro.panic.html) macro and related panic-generating macros like [`unreachable!`](https://doc.rust-lang.org/1.100.0/core/macro.unreachable.html) also have the type [`!`](types/never.md#r-type.never) and are diverging.

[divergence.never](#r-divergence.never)

Any expression of type [`!`](types/never.md#r-type.never) is a diverging expression. However, diverging expressions are not limited to type [`!`](types/never.md#r-type.never); expressions of other types may also diverge (e.g., `Some(loop {})` has type `Option<!>`).

> ![](../assets/inline/divergence-2.svg)Note
>
> Though `!` is considered an uninhabited type, a type being uninhabited is not sufficient for it to diverge.
>
> ``` playground
> #![allow(unused)]
> fn main() {
> enum Empty {}
> fn make_never() -> ! {loop{}}
> fn make_empty() -> Empty {loop{}}
>
> fn diverging() -> ! {
>     // This has a type of `!`.
>     // So, the entire function is considered diverging.
>     make_never();
>     // OK: The type of the body is `!` which matches the return type.
> }
> fn not_diverging() -> ! {
>     // This type is uninhabited.
>     // However, the entire function is not considered diverging.
>     make_empty();
>     // ERROR: The type of the body is `()` but expected type `!`.
> }
> }
> ```

> ![](../assets/inline/divergence-3.svg)Note
>
> Divergence can propagate to the surrounding block. See [expr.block.diverging](expressions/block-expr.md#r-expr.block.diverging).

[divergence.fallback](#r-divergence.fallback)

## Fallback

If a type to be inferred is only unified with diverging expressions, then that type will be inferred to be [`!`](types/never.md#r-type.never).

> ![](../assets/inline/divergence-4.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> fn foo() -> i32 { 22 }
> match foo() {
>     // ERROR: The trait bound `!: Default` is not satisfied.
>     4 => Default::default(),
>     _ => return,
> };
> }
> ```

> ![](../assets/inline/divergence-5.svg)Note
>
> Importantly, type unification may happen *structurally*, so the fallback `!` may be part of a larger type. The following compiles:
>
> ``` playground
> #![allow(unused)]
> fn main() {
> fn foo() -> i32 { 22 }
> // This has the type `Option<!>`, not `!`
> match foo() {
>     4 => Default::default(),
>     _ => Some(return),
> };
> }
> ```
