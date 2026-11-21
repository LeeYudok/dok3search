# Keywords

[lex.keywords](#r-lex.keywords)

# Keywords

[lex.keywords.intro](#r-lex.keywords.intro)

Rust divides keywords into three categories:

- [strict](#strict-keywords)
- [reserved](#reserved-keywords)
- [weak](#weak-keywords)

[lex.keywords.strict](#r-lex.keywords.strict)

## Strict keywords

[lex.keywords.strict.intro](#r-lex.keywords.strict.intro)

Strict keywords can only be used in their correct contexts. They cannot be used as the names of:

- [Items](items.md)
- [Variables](variables.md) and function parameters
- Fields and [variants](items/enumerations.md)
- [Type parameters](types/parameters.md)
- Lifetime parameters or [loop labels](expressions/loop-expr.md#loop-labels)
- [Macros](macros.md) or [attributes](attributes.md)
- [Macro placeholders](macros-by-example.md)
- [Crates](crates-and-source-files.md)

[lex.keywords.strict.syntax](#r-lex.keywords.strict.syntax)

**Lexer**\
[STRICT_KEYWORDS](#railroad-STRICT_KEYWORDS) →\
      \_\
    \| as\
    \| async\
    \| await\
    \| break\
    \| const\
    \| continue\
    \| crate\
    \| dyn\
    \| else\
    \| enum\
    \| extern\
    \| false\
    \| fn\
    \| for\
    \| if\
    \| impl\
    \| in\
    \| let\
    \| loop\
    \| match\
    \| mod\
    \| move\
    \| mut\
    \| pub\
    \| ref\
    \| return\
    \| self\
    \| Self\
    \| static\
    \| struct\
    \| super\
    \| trait\
    \| true\
    \| type\
    \| unsafe\
    \| use\
    \| where\
    \| while

Show Railroad

![](../assets/inline/keywords-1.svg) [lex.keywords.strict.edition2018](#r-lex.keywords.strict.edition2018)\
Tests Tests with this rule:

- [tests/ui/rust-2018/async-ident-allowed.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/rust-2018/async-ident-allowed.rs)
- [tests/ui/rust-2018/async-ident.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/rust-2018/async-ident.rs)
- [tests/ui/rust-2018/dyn-keyword.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/rust-2018/dyn-keyword.rs)

> 2018 Edition differences
>
> The following keywords were added in the 2018 edition:
>
> - `async`
> - `await`
> - `dyn`

[lex.keywords.reserved](#r-lex.keywords.reserved)

## Reserved keywords

[lex.keywords.reserved.intro](#r-lex.keywords.reserved.intro)

Reserved keywords aren’t used yet, but they are reserved for future use. They have the same restrictions as strict keywords. The reasoning behind this is to make current programs forward compatible with future versions of Rust by forbidding them to use these keywords.

[lex.keywords.reserved.syntax](#r-lex.keywords.reserved.syntax)

**Lexer**\
[RESERVED_KEYWORDS](#railroad-RESERVED_KEYWORDS) →\
      abstract\
    \| become\
    \| box\
    \| do\
    \| final\
    \| gen\
    \| macro\
    \| override\
    \| priv\
    \| try\
    \| typeof\
    \| unsized\
    \| virtual\
    \| yield

Show Railroad

![](../assets/inline/keywords-2.svg) [lex.keywords.reserved.edition2018](#r-lex.keywords.reserved.edition2018)\
Tests Tests with this rule:

- [tests/ui/rust-2018/try-ident.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/rust-2018/try-ident.rs)
- [tests/ui/rust-2018/try-macro.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/rust-2018/try-macro.rs)

> 2018 Edition differences
>
> The `try` keyword was added as a reserved keyword in the 2018 edition.

[lex.keywords.reserved.edition2024](#r-lex.keywords.reserved.edition2024)

> 2024 Edition differences
>
> The `gen` keyword was added as a reserved keyword in the 2024 edition.

[lex.keywords.weak](#r-lex.keywords.weak)

## Weak keywords

[lex.keywords.weak.intro](#r-lex.keywords.weak.intro)

Weak keywords have special meaning only in certain contexts. For example, it is possible to declare a variable or method with the name `union`.

[lex.keywords.weak.syntax](#r-lex.keywords.weak.syntax)

**Lexer**\
[WEAK_KEYWORDS](#railroad-WEAK_KEYWORDS) →\
      'static\
    \| macro_rules\
    \| raw\
    \| safe\
    \| union

Show Railroad

![](../assets/inline/keywords-3.svg) [lex.keywords.weak.macro_rules](#r-lex.keywords.weak.macro_rules)

- `macro_rules` is used to create custom [macros](macros.md).

[lex.keywords.weak.union](#r-lex.keywords.weak.union)\
Tests Tests with this rule:

- [tests/ui/union/union-backcomp.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/union-backcomp.rs)

- `union` is used to declare a [union](items/unions.md) and is only a keyword when used in a union declaration.

[lex.keywords.weak.lifetime-static](#r-lex.keywords.weak.lifetime-static)

- `'static` is used for the static lifetime and cannot be used as a [generic lifetime parameter](items/generics.md) or [loop label](expressions/loop-expr.md#loop-labels)

  ``` compile_fail
  // error[E0262]: invalid lifetime parameter name: `'static`
  fn invalid_lifetime_parameter<'static>(s: &'static str) -> &'static str { s }
  ```

[lex.keywords.weak.safe](#r-lex.keywords.weak.safe)

- `safe` is used for functions and statics, which has meaning in [external blocks](items/external-blocks.md).

[lex.keywords.weak.raw](#r-lex.keywords.weak.raw)

- `raw` is used for [raw borrow operators](expressions/operator-expr.md#raw-borrow-operators), and is only a keyword when matching a raw borrow operator form (such as `&raw const expr` or `&raw mut expr`).

[lex.keywords.weak.dyn.edition2018](#r-lex.keywords.weak.dyn.edition2018)

> 2018 Edition differences
>
> In the 2015 edition, [`dyn`](types/trait-object.md) is a keyword when used in a type position followed by a path that does not start with `::` or `<`, a lifetime, a question mark, a `for` keyword or an opening parenthesis.
>
> Beginning in the 2018 edition, `dyn` has been promoted to a strict keyword.
