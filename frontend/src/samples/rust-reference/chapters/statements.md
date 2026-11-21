# Statements

[statement](#r-statement)

# Statements

[statement.syntax](#r-statement.syntax)

**Syntax**\
[Statement](#railroad-Statement) →\
      ;\
    \| [Item](items.md#grammar-Item)\
    \| [LetStatement](#grammar-LetStatement)\
    \| [ExpressionStatement](#grammar-ExpressionStatement)\
    \| [OuterAttribute](attributes.md#grammar-OuterAttribute)\* [MacroInvocationSemi](macros.md#grammar-MacroInvocationSemi)

Show Railroad

![](../assets/inline/statements-1.svg) [statement.intro](#r-statement.intro)

A *statement* is a component of a [block](expressions/block-expr.md), which is in turn a component of an outer [expression](expressions.md) or [function](items/functions.md).

[statement.kind](#r-statement.kind)

Rust has two kinds of statement: [declaration statements](#declaration-statements) and [expression statements](#expression-statements).

[statement.decl](#r-statement.decl)

## Declaration statements

A *declaration statement* is one that introduces one or more *names* into the enclosing statement block. The declared names may denote new variables or new [items](items.md).

The two kinds of declaration statements are item declarations and `let` statements.

[statement.item](#r-statement.item)

### Item declarations

[statement.item.intro](#r-statement.item.intro)

An *item declaration statement* has a syntactic form identical to an [item declaration](items.md) within a [module](items/modules.md).

[statement.item.scope](#r-statement.item.scope)

Declaring an item within a statement block restricts its [scope](names/scopes.md) to the block containing the statement. The item is not given a [canonical path](paths.md#canonical-paths) nor are any sub-items it may declare.

[statement.item.associated-scope](#r-statement.item.associated-scope)

The exception to this is that associated items defined by [implementations](items/implementations.md) are still accessible in outer scopes as long as the item and, if applicable, trait are accessible. It is otherwise identical in meaning to declaring the item inside a module.

[statement.item.outer-generics](#r-statement.item.outer-generics)

There is no implicit capture of the containing function’s generic parameters, parameters, and local variables. For example, `inner` may not access `outer_var`.

``` playground
#![allow(unused)]
fn main() {
fn outer() {
  let outer_var = true;

  fn inner() { /* outer_var is not in scope here */ }

  inner();
}
}
```

[statement.let](#r-statement.let)

### `let` statements

[statement.let.syntax](#r-statement.let.syntax)\
Tests Tests with this rule:

- [tests/ui/tuple/one-tuple.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/tuple/one-tuple.rs)

**Syntax**\
[LetStatement](#railroad-LetStatement) →\
    [OuterAttribute](attributes.md#grammar-OuterAttribute)\* let [PatternNoTopAlt](patterns.md#grammar-PatternNoTopAlt) ( : [Type](types.md#grammar-Type) )?\
    (\
          = [Expression](expressions.md#grammar-Expression)\
        \| = [Expression](expressions.md#grammar-Expression)except [LazyBooleanExpression](expressions/operator-expr.md#grammar-LazyBooleanExpression) or end with a `}`\
              else [BlockExpressionNoInnerAttributes](expressions/block-expr.md#grammar-BlockExpressionNoInnerAttributes)\
    )? ;

Show Railroad

![](../assets/inline/statements-2.svg) [statement.let.intro](#r-statement.let.intro)

A *`let` statement* introduces a new set of [variables](variables.md), given by a [pattern](patterns.md). The pattern is followed optionally by a type annotation and then either ends, or is followed by an initializer expression plus an optional `else` block.

[statement.let.inference](#r-statement.let.inference)

When no type annotation is given, the compiler will infer the type, or signal an error if insufficient type information is available for definite inference.

[statement.let.scope](#r-statement.let.scope)

Any variables introduced by a variable declaration are visible from the point of declaration until the end of the enclosing block scope, except when they are shadowed by another variable declaration.

[statement.let.constraint](#r-statement.let.constraint)

If an `else` block is not present, the pattern must be irrefutable. If an `else` block is present, the pattern may be refutable.

[statement.let.behavior](#r-statement.let.behavior)

If the pattern does not match (this requires it to be refutable), the `else` block is executed. The `else` block must always diverge (evaluate to the [never type](types/never.md)).

``` playground
#![allow(unused)]
fn main() {
let (mut v, w) = (vec![1, 2, 3], 42); // The bindings may be mut or const
let Some(t) = v.pop() else { // Refutable patterns require an else block
    panic!(); // The else block must diverge
};
let [u, v] = [v[0], v[1]] else { // This pattern is irrefutable, so the compiler
                                 // will lint as the else block is redundant.
    panic!();
};
}
```

[statement.expr](#r-statement.expr)

## Expression statements

[statement.expr.syntax](#r-statement.expr.syntax)

**Syntax**\
[ExpressionStatement](#railroad-ExpressionStatement) →\
      [ExpressionWithoutBlock](expressions.md#grammar-ExpressionWithoutBlock) ;\
    \| [ExpressionWithBlock](expressions.md#grammar-ExpressionWithBlock) ;?

Show Railroad

![](../assets/inline/statements-3.svg) [statement.expr.intro](#r-statement.expr.intro)

An *expression statement* is one that evaluates an [expression](expressions.md) and ignores its result. As a rule, an expression statement’s purpose is to trigger the effects of evaluating its expression.

[statement.expr.restriction-semicolon](#r-statement.expr.restriction-semicolon)

An expression that consists of only a [block expression](expressions/block-expr.md) or control flow expression, if used in a context where a statement is permitted, can omit the trailing semicolon. This can cause an ambiguity between it being parsed as a standalone statement and as a part of another expression; in this case, it is parsed as a statement.

[statement.expr.constraint-block](#r-statement.expr.constraint-block)

The type of [ExpressionWithBlock](expressions.md#grammar-ExpressionWithBlock) expressions when used as statements must be the unit type.

``` playground
#![allow(unused)]
fn main() {
let mut v = vec![1, 2, 3];
v.pop();          // Ignore the element returned from pop
if v.is_empty() {
    v.push(5);
} else {
    v.remove(0);
}                 // Semicolon can be omitted.
[1];              // Separate expression statement, not an indexing expression.
}
```

When the trailing semicolon is omitted, the result must be type `()`.

``` playground
#![allow(unused)]
fn main() {
// bad: the block's type is i32, not ()
// Error: expected `()` because of default return type
// if true {
//   1
// }

// good: the block's type is i32
if true {
  1
} else {
  2
};
}
```

[statement.attribute](#r-statement.attribute)

## Attributes on statements

Statements accept [outer attributes](attributes.md). The attributes that have meaning on a statement are [`cfg`](conditional-compilation.md), and [the lint check attributes](attributes/diagnostics.md#lint-check-attributes).
