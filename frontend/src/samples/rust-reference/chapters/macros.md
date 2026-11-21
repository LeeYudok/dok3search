# Macros

[macro](#r-macro)

# Macros

[macro.intro](#r-macro.intro)

The functionality and syntax of Rust can be extended with custom definitions called macros. They are given names, and invoked through a consistent syntax: `some_extension!(...)`.

There are two ways to define new macros:

- [Macros by example](macros-by-example.md) define new syntax in a higher-level, declarative way.
- [Procedural macros](procedural-macros.md) define function-like macros, custom derives, and custom attributes using functions that operate on input tokens.

[macro.invocation](#r-macro.invocation)

## Macro invocation

[macro.invocation.syntax](#r-macro.invocation.syntax)

**Syntax**\
[MacroInvocation](#railroad-MacroInvocation) →\
    [SimplePath](paths.md#grammar-SimplePath) ! [DelimTokenTree](#grammar-DelimTokenTree)

[DelimTokenTree](#railroad-DelimTokenTree) →\
      ( [TokenTree](#grammar-TokenTree)\* )\
    \| \[ [TokenTree](#grammar-TokenTree)\* \]\
    \| { [TokenTree](#grammar-TokenTree)\* }

[TokenTree](#railroad-TokenTree) →\
    [Token](tokens.md#grammar-Token)except [delimiters](tokens.md#r-lex.token.delim) \| [DelimTokenTree](#grammar-DelimTokenTree)

[MacroInvocationSemi](#railroad-MacroInvocationSemi) →\
      [SimplePath](paths.md#grammar-SimplePath) ! ( [TokenTree](#grammar-TokenTree)\* ) ;\
    \| [SimplePath](paths.md#grammar-SimplePath) ! \[ [TokenTree](#grammar-TokenTree)\* \] ;\
    \| [SimplePath](paths.md#grammar-SimplePath) ! { [TokenTree](#grammar-TokenTree)\* }

Show Railroad

![](../assets/inline/macros-1.svg) ![](../assets/inline/macros-2.svg) ![](../assets/inline/macros-3.svg) ![](../assets/inline/macros-4.svg) [macro.invocation.intro](#r-macro.invocation.intro)

A macro invocation expands a macro at compile time and replaces the invocation with the result of the macro. Macros may be invoked in the following situations:

[macro.invocation.expr](#r-macro.invocation.expr)

- [Expressions](expressions.md) and [statements](statements.md)

[macro.invocation.pattern](#r-macro.invocation.pattern)

- [Patterns](patterns.md)

[macro.invocation.type](#r-macro.invocation.type)

- [Types](types.md)

[macro.invocation.item](#r-macro.invocation.item)

- [Items](items.md) including [associated items](items/associated-items.md)

[macro.invocation.nested](#r-macro.invocation.nested)

- [`macro_rules`](macros-by-example.md) transcribers

[macro.invocation.extern](#r-macro.invocation.extern)

- [External blocks](items/external-blocks.md)

[macro.invocation.item-statement](#r-macro.invocation.item-statement)

When used as an item or a statement, the [MacroInvocationSemi](#grammar-MacroInvocationSemi) form is used where a semicolon is required at the end when not using curly braces. [Visibility qualifiers](visibility-and-privacy.md) are never allowed before a macro invocation or [`macro_rules`](macros-by-example.md) definition.

``` playground
#![allow(unused)]
fn main() {
// Used as an expression.
let x = vec![1,2,3];

// Used as a statement.
println!("Hello!");

// Used in a pattern.
macro_rules! pat {
    ($i:ident) => (Some($i))
}

if let pat!(x) = Some(1) {
    assert_eq!(x, 1);
}

// Used in a type.
macro_rules! Tuple {
    { $A:ty, $B:ty } => { ($A, $B) };
}

type N2 = Tuple!(i32, i32);

// Used as an item.
use std::cell::RefCell;
thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

// Used as an associated item.
macro_rules! const_maker {
    ($t:ty, $v:tt) => { const CONST: $t = $v; };
}
trait T {
    const_maker!{i32, 7}
}

// Macro calls within macros.
macro_rules! example {
    () => { println!("Macro call in a macro!") };
}
// Outer macro `example` is expanded, then inner macro `println` is expanded.
example!();
}
```

[macro.invocation.name-resolution](#r-macro.invocation.name-resolution)

Macros invocations can be resolved via two kinds of scopes:

- Textual Scope
  - [Textual scope `macro_rules`](macros-by-example.md#r-macro.decl.scope.textual)
- Path-based scope
  - [Path-based scope `macro_rules`](macros-by-example.md#r-macro.decl.scope.path-based)
  - [Procedural macros](procedural-macros.md)
