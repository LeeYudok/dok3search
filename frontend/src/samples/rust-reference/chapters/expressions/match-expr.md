# match expressions

[expr.match](#r-expr.match)

# `match` expressions

[expr.match.syntax](#r-expr.match.syntax)

**Syntax**\
[MatchExpression](#railroad-MatchExpression) →\
    match [Scrutinee](#grammar-Scrutinee) {\
        [InnerAttribute](../attributes.md#grammar-InnerAttribute)\*\
        [MatchArms](#grammar-MatchArms)?\
    }

[Scrutinee](#railroad-Scrutinee) → [Expression](../expressions.md#grammar-Expression)except [StructExpression](struct-expr.md#grammar-StructExpression)

[MatchArms](#railroad-MatchArms) →\
    ( [MatchArm](#grammar-MatchArm) =\> ( [ExpressionWithoutBlock](../expressions.md#grammar-ExpressionWithoutBlock) , \| [ExpressionWithBlock](../expressions.md#grammar-ExpressionWithBlock) ,? ) )\*\
    [MatchArm](#grammar-MatchArm) =\> [Expression](../expressions.md#grammar-Expression) ,?

[MatchArm](#railroad-MatchArm) → [OuterAttribute](../attributes.md#grammar-OuterAttribute)\* [Pattern](../patterns.md#grammar-Pattern) [MatchArmGuard](#grammar-MatchArmGuard)?

[MatchArmGuard](#railroad-MatchArmGuard) → if [MatchConditions](#grammar-MatchConditions)

[MatchConditions](#railroad-MatchConditions) →\
     [MatchGuardChain](#grammar-MatchGuardChain)\
   \| [Expression](../expressions.md#grammar-Expression)

[MatchGuardChain](#railroad-MatchGuardChain) → [MatchGuardCondition](#grammar-MatchGuardCondition) ( && [MatchGuardCondition](#grammar-MatchGuardCondition) )\*

[MatchGuardCondition](#railroad-MatchGuardCondition) →\
     [Expression](../expressions.md#grammar-Expression)except [ExcludedMatchConditions](#grammar-ExcludedMatchConditions)\
   \| [OuterAttribute](../attributes.md#grammar-OuterAttribute)\* let [Pattern](../patterns.md#grammar-Pattern) = [MatchGuardScrutinee](#grammar-MatchGuardScrutinee)

[MatchGuardScrutinee](#railroad-MatchGuardScrutinee) → [Expression](../expressions.md#grammar-Expression)except [ExcludedMatchConditions](#grammar-ExcludedMatchConditions)

[ExcludedMatchConditions](#railroad-ExcludedMatchConditions) →\
      [LazyBooleanExpression](operator-expr.md#grammar-LazyBooleanExpression)\
    \| [RangeExpr](range-expr.md#grammar-RangeExpr)\
    \| [RangeFromExpr](range-expr.md#grammar-RangeFromExpr)\
    \| [RangeInclusiveExpr](range-expr.md#grammar-RangeInclusiveExpr)\
    \| [AssignmentExpression](operator-expr.md#grammar-AssignmentExpression)\
    \| [CompoundAssignmentExpression](operator-expr.md#grammar-CompoundAssignmentExpression)

Show Railroad

![](../../assets/inline/match-expr-1.svg) ![](../../assets/inline/match-expr-2.svg) ![](../../assets/inline/match-expr-3.svg) ![](../../assets/inline/match-expr-4.svg) ![](../../assets/inline/match-expr-5.svg) ![](../../assets/inline/match-expr-6.svg) ![](../../assets/inline/match-expr-7.svg) ![](../../assets/inline/match-expr-8.svg) ![](../../assets/inline/match-expr-9.svg) ![](../../assets/inline/match-expr-10.svg) [expr.match.intro](#r-expr.match.intro)

A *`match` expression* branches on a pattern. The exact form of matching that occurs depends on the [pattern](../patterns.md).

[expr.match.scrutinee](#r-expr.match.scrutinee)

A `match` expression has a *[scrutinee](../glossary.md#scrutinee) expression*, which is the value to compare to the patterns.

[expr.match.scrutinee-constraint](#r-expr.match.scrutinee-constraint)

The scrutinee expression and the patterns must have the same type.

[expr.match.scrutinee-behavior](#r-expr.match.scrutinee-behavior)

A `match` behaves differently depending on whether or not the scrutinee expression is a [place expression or value expression](../expressions.md#place-expressions-and-value-expressions).

[expr.match.scrutinee-value](#r-expr.match.scrutinee-value)

If the scrutinee expression is a [value expression](../expressions.md#place-expressions-and-value-expressions), it is first evaluated into a temporary location, and the resulting value is sequentially compared to the patterns in the arms until a match is found. The first arm with a matching pattern is chosen as the branch target of the `match`, any variables bound by the pattern are assigned to local variables in the arm’s block, and control enters the block.

[expr.match.scrutinee-place](#r-expr.match.scrutinee-place)

When the scrutinee expression is a [place expression](../expressions.md#place-expressions-and-value-expressions), the match does not allocate a temporary location; however, a by-value binding may copy or move from the memory location. When possible, it is preferable to match on place expressions, as the lifetime of these matches inherits the lifetime of the place expression rather than being restricted to the inside of the match.

An example of a `match` expression:

``` playground
#![allow(unused)]
fn main() {
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    _ => println!("something else"),
}
}
```

[expr.match.pattern-vars](#r-expr.match.pattern-vars)

Variables bound within the pattern are scoped to the match guard and the arm’s expression.

[expr.match.pattern-var-binding](#r-expr.match.pattern-var-binding)

The [binding mode](../patterns.md#binding-modes) (move, copy, or reference) depends on the pattern.

[expr.match.or-pattern](#r-expr.match.or-pattern)

Multiple match patterns may be joined with the `|` operator. Each pattern will be tested in left-to-right sequence until a successful match is found.

``` playground
#![allow(unused)]
fn main() {
let x = 9;
let message = match x {
    0 | 1  => "not many",
    2 ..= 9 => "a few",
    _      => "lots"
};

assert_eq!(message, "a few");

// Demonstration of pattern match order.
struct S(i32, i32);

match S(1, 2) {
    S(z @ 1, _) | S(_, z @ 2) => assert_eq!(z, 1),
    _ => panic!(),
}
}
```

> ![](../../assets/inline/match-expr-11.svg)Note
>
> The `2..=9` is a [Range Pattern](../patterns.md#range-patterns), not a [Range Expression](range-expr.md). Thus, only those types of ranges supported by range patterns can be used in match arms.

[expr.match.or-patterns-restriction](#r-expr.match.or-patterns-restriction)

Every binding in each `|` separated pattern must appear in all of the patterns in the arm.

[expr.match.binding-restriction](#r-expr.match.binding-restriction)

Every binding of the same name must have the same type, and have the same binding mode.

[expr.match.type](#r-expr.match.type)

The type of the overall `match` expression is the [least upper bound](../type-coercions.md#r-coerce.least-upper-bound) of the individual match arms.

[expr.match.empty](#r-expr.match.empty)

If there are no match arms, then the `match` expression is [diverging](../divergence.md#r-divergence) and the type is [`!`](../types/never.md#r-type.never).

> ![](../../assets/inline/match-expr-12.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> enum Empty {}
>
> fn diverging_match_no_arms(e: Empty) -> ! {
>     match e {}
> }
> }
> ```

[expr.match.diverging](#r-expr.match.diverging)

If either the scrutinee expression or all of the match arms diverge, then the entire `match` expression also diverges.

[expr.match.guard](#r-expr.match.guard)

## Match guards

[expr.match.guard.intro](#r-expr.match.guard.intro)

Match arms can accept *match guards* to further refine the criteria for matching a case.

[expr.match.guard.condition](#r-expr.match.guard.condition)

Pattern guards appear after the pattern following the `if` keyword and consist of an [Expression](../expressions.md#grammar-Expression) with a [boolean type](../types/boolean.md#r-type.bool) or a conditional `let` match.

[expr.match.guard.behavior](#r-expr.match.guard.behavior)

When the pattern matches successfully, the pattern guard is executed. If all of the guard condition operands evaluate to `true` and all of the `let` patterns successfully match their [scrutinee](../glossary.md#scrutinee)s, the match arm is successfully matched against and the arm body is executed.

[expr.match.guard.next](#r-expr.match.guard.next)

Otherwise, the next pattern, including other matches with the `|` operator in the same arm, is tested.

``` playground
#![allow(unused)]
fn main() {
let maybe_digit = Some(0);
fn process_digit(i: i32) { }
fn process_other(i: i32) { }
let message = match maybe_digit {
    Some(x) if x < 10 => process_digit(x),
    Some(x) => process_other(x),
    None => panic!(),
};
}
```

> ![](../../assets/inline/match-expr-13.svg)Note
>
> Multiple matches using the `|` operator can cause the pattern guard and the side effects it has to execute multiple times. For example:
>
> ``` playground
> #![allow(unused)]
> fn main() {
> use std::cell::Cell;
> let i : Cell = Cell::new(0);
> match 1 {
>     1 | _ if { i.set(i.get() + 1); false } => {}
>     _ => {}
> }
> assert_eq!(i.get(), 2);
> }
> ```

[expr.match.guard.bound-variables](#r-expr.match.guard.bound-variables)

A pattern guard may refer to the variables bound within the pattern they follow.

[expr.match.guard.shared-ref](#r-expr.match.guard.shared-ref)

Before evaluating the guard, a shared reference is taken to the part of the scrutinee the variable matches on. While evaluating the guard, this shared reference is then used when accessing the variable.

[expr.match.guard.value](#r-expr.match.guard.value)

Only when the guard evaluates successfully is the value moved, or copied, from the scrutinee into the variable. This allows shared borrows to be used inside guards without moving out of the scrutinee in case guard fails to match.

[expr.match.guard.no-mutation](#r-expr.match.guard.no-mutation)

Moreover, by holding a shared reference while evaluating the guard, mutation inside guards is also prevented.

[expr.match.guard.let](#r-expr.match.guard.let)

Guards can use `let` patterns to conditionally match a scrutinee and to bind new variables into scope when the pattern matches successfully.

> ![](../../assets/inline/match-expr-14.svg)Example
>
> In this example, the guard condition `let Some(first_char) = name.chars().next()` is evaluated. If the `let` pattern successfully matches (i.e. the string has at least one character), the arm’s body is executed. Otherwise, pattern matching continues to the next arm.
>
> The `let` pattern creates a new binding (`first_char`), which can be used alongside the original pattern bindings (`name`) in the arm’s body.
>
> ``` playground
> #![allow(unused)]
> fn main() {
> enum Command {
>     Run(String),
>     Stop,
> }
> let cmd = Command::Run("example".to_string());
>
> match cmd {
>     Command::Run(name) if let Some(first_char) = name.chars().next() => {
>         // Both `name` and `first_char` are available here
>         println!("Running: {name} (starts with '{first_char}')");
>     }
>     Command::Run(name) => {
>         println!("{name} is empty");
>     }
>     _ => {}
> }
> }
> ```

[expr.match.guard.chains](#r-expr.match.guard.chains)

## Match guard chains

[expr.match.guard.chains.intro](#r-expr.match.guard.chains.intro)

Multiple guard condition operands can be separated with `&&`.

> ![](../../assets/inline/match-expr-15.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> let foo = Some([123]);
> let already_checked = false;
> match foo {
>     Some(xs) if let [single] = xs && !already_checked => { dbg!(single); }
>     _ => {}
> }
> }
> ```

[expr.match.guard.chains.order](#r-expr.match.guard.chains.order)

Similar to a `&&` [LazyBooleanExpression](operator-expr.md#grammar-LazyBooleanExpression), each operand is evaluated from left-to-right until an operand evaluates as `false` or a `let` match fails, in which case the subsequent operands are not evaluated.

[expr.match.guard.chains.bindings](#r-expr.match.guard.chains.bindings)

The bindings of each `let` pattern are put into scope to be available for the next condition operand and the match arm body.

[expr.match.guard.chains.or](#r-expr.match.guard.chains.or)

If any guard condition operand is a `let` pattern, then none of the condition operands can be a `||` [lazy boolean operator expression](operator-expr.md#r-expr.bool-logic) due to ambiguity and precedence with the `let` scrutinee.

> ![](../../assets/inline/match-expr-16.svg)Example
>
> If a `||` expression is needed, then parentheses can be used. For example:
>
> ``` playground
> #![allow(unused)]
> fn main() {
> let foo = Some([123]);
> match foo {
>     Some(xs) if let [x] = xs
>         // Parentheses are required here.
>         && (x < -100 || x > 20) => {}
>     _ => {}
> }
> }
> ```

[expr.match.attributes](#r-expr.match.attributes)

## Attributes on match arms

[expr.match.attributes.outer](#r-expr.match.attributes.outer)

Outer attributes are allowed on match arms. The only attributes that have meaning on match arms are [`cfg`](../conditional-compilation.md) and the [lint check attributes](../attributes/diagnostics.md#lint-check-attributes).

[expr.match.attributes.inner](#r-expr.match.attributes.inner)

[Inner attributes](../attributes.md) are allowed directly after the opening brace of the match expression in the same expression contexts as [attributes on block expressions](block-expr.md#attributes-on-block-expressions).
