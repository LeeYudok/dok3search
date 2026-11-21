# Macros by example

[macro.decl](#r-macro.decl)

# Macros by example

[macro.decl.syntax](#r-macro.decl.syntax)\
Tests Tests with this rule:

- [tests/ui/macros/macro-error.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/macro-error.rs)

**Syntax**\
[MacroRulesDefinition](#railroad-MacroRulesDefinition) →\
    macro_rules ! [IDENTIFIER](identifiers.md#grammar-IDENTIFIER) [MacroRulesDef](#grammar-MacroRulesDef)

[MacroRulesDef](#railroad-MacroRulesDef) →\
      ( [MacroRules](#grammar-MacroRules) ) ;\
    \| \[ [MacroRules](#grammar-MacroRules) \] ;\
    \| { [MacroRules](#grammar-MacroRules) }

[MacroRules](#railroad-MacroRules) →\
    [MacroRule](#grammar-MacroRule) ( ; [MacroRule](#grammar-MacroRule) )\* ;?

[MacroRule](#railroad-MacroRule) →\
    [MacroMatcher](#grammar-MacroMatcher) =\> [MacroTranscriber](#grammar-MacroTranscriber)

[MacroMatcher](#railroad-MacroMatcher) →\
      ( [MacroMatch](#grammar-MacroMatch)\* )\
    \| \[ [MacroMatch](#grammar-MacroMatch)\* \]\
    \| { [MacroMatch](#grammar-MacroMatch)\* }

[MacroMatch](#railroad-MacroMatch) →\
      [Token](tokens.md#grammar-Token)except `$` and [delimiters](tokens.md#r-lex.token.delim)\
    \| [MacroMatcher](#grammar-MacroMatcher)\
    \| \$ ( [IDENTIFIER_OR_KEYWORD](identifiers.md#grammar-IDENTIFIER_OR_KEYWORD)except `crate` \| [RAW_IDENTIFIER](identifiers.md#grammar-RAW_IDENTIFIER) ) : [MacroFragSpec](#grammar-MacroFragSpec)\
    \| \$ ( [MacroMatch](#grammar-MacroMatch)+ ) [MacroRepSep](#grammar-MacroRepSep)? [MacroRepOp](#grammar-MacroRepOp)

[MacroFragSpec](#railroad-MacroFragSpec) →\
      block \| expr \| expr_2021 \| ident \| item \| lifetime \| literal\
    \| meta \| pat \| pat_param \| path \| stmt \| tt \| ty \| vis

[MacroRepSep](#railroad-MacroRepSep) → [Token](tokens.md#grammar-Token)except [delimiters](tokens.md#r-lex.token.delim) and [MacroRepOp](#grammar-MacroRepOp)

[MacroRepOp](#railroad-MacroRepOp) → \* \| + \| ?

[MacroTranscriber](#railroad-MacroTranscriber) → [DelimTokenTree](macros.md#grammar-DelimTokenTree)

Show Railroad

![](../assets/inline/macros-by-example-1.svg) ![](../assets/inline/macros-by-example-2.svg) ![](../assets/inline/macros-by-example-3.svg) ![](../assets/inline/macros-by-example-4.svg) ![](../assets/inline/macros-by-example-5.svg) ![](../assets/inline/macros-by-example-6.svg) ![](../assets/inline/macros-by-example-7.svg) ![](../assets/inline/macros-by-example-8.svg) ![](../assets/inline/macros-by-example-9.svg) ![](../assets/inline/macros-by-example-10.svg) [macro.decl.intro](#r-macro.decl.intro)

`macro_rules` allows users to define syntax extension in a declarative way. We call such extensions “macros by example” or simply “macros”.

Each macro by example has a name, and one or more *rules*. Each rule has two parts: a *matcher*, describing the syntax that it matches, and a *transcriber*, describing the syntax that will replace a successfully matched invocation. Both the matcher and the transcriber must be surrounded by delimiters. Macros can expand to expressions, statements, items (including traits, impls, and foreign items), types, or patterns.

[macro.decl.transcription](#r-macro.decl.transcription)

## Transcribing

[macro.decl.transcription.intro](#r-macro.decl.transcription.intro)

When a macro is invoked, the macro expander looks up macro invocations by name, and tries each macro rule in turn. It transcribes the first successful match; if this results in an error, then future matches are not tried.

[macro.decl.transcription.lookahead](#r-macro.decl.transcription.lookahead)\
Tests Tests with this rule:

- [tests/ui/macros/local-ambiguity-multiple-parsing-options.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/local-ambiguity-multiple-parsing-options.rs)

When matching, no lookahead is performed; if the compiler cannot unambiguously determine how to parse the macro invocation one token at a time, then it is an error. In the following example, the compiler does not look ahead past the identifier to see if the following token is a `)`, even though that would allow it to parse the invocation unambiguously:

``` playground
#![allow(unused)]
fn main() {
macro_rules! ambiguity {
    ($($i:ident)* $j:ident) => { };
}

ambiguity!(error); // Error: local ambiguity
}
```

[macro.decl.transcription.syntax](#r-macro.decl.transcription.syntax)

In both the matcher and the transcriber, the `$` token is used to invoke special behaviours from the macro engine (described below in [Metavariables](#metavariables) and [Repetitions](#repetitions)). Tokens that aren’t part of such an invocation are matched and transcribed literally, with one exception. The exception is that the outer delimiters for the matcher will match any pair of delimiters. Thus, for instance, the matcher `(())` will match `{()}` but not `{{}}`. The character `$` cannot be matched or transcribed literally.

[macro.decl.transcription.fragment](#r-macro.decl.transcription.fragment)

### Forwarding a matched fragment

When forwarding a matched fragment to another macro-by-example, matchers in the second macro will see an opaque AST of the fragment type. The second macro can’t use literal tokens to match the fragments in the matcher, only a fragment specifier of the same type. The `ident`, `lifetime`, and `tt` fragment types are an exception, and *can* be matched by literal tokens. The following illustrates this restriction:

``` playground
#![allow(unused)]
fn main() {
macro_rules! foo {
    ($l:expr) => { bar!($l); }
// ERROR:               ^^ no rules expected this token in macro call
}

macro_rules! bar {
    (3) => {}
}

foo!(3);
}
```

The following illustrates how tokens can be directly matched after matching a `tt` fragment:

``` playground
#![allow(unused)]
fn main() {
// compiles OK
macro_rules! foo {
    ($l:tt) => { bar!($l); }
}

macro_rules! bar {
    (3) => {}
}

foo!(3);
}
```

[macro.decl.meta](#r-macro.decl.meta)

## Metavariables

[macro.decl.meta.intro](#r-macro.decl.meta.intro)

In the matcher, `$` *name* `:` *fragment-specifier* matches a Rust syntax fragment of the kind specified and binds it to the metavariable `$`*name*.

[macro.decl.meta.specifier](#r-macro.decl.meta.specifier)

Valid fragment specifiers are:

- `block`: a [BlockExpressionNoInnerAttributes](expressions/block-expr.md#grammar-BlockExpressionNoInnerAttributes)
- `expr`: an [Expression](expressions.md#grammar-Expression)
- `expr_2021`: an [Expression](expressions.md#grammar-Expression) except [UnderscoreExpression](expressions/underscore-expr.md#grammar-UnderscoreExpression) and [ConstBlockExpression](expressions/block-expr.md#grammar-ConstBlockExpression) (see [macro.decl.meta.edition2024](#r-macro.decl.meta.edition2024))
- `ident`: an [IDENTIFIER_OR_KEYWORD](identifiers.md#grammar-IDENTIFIER_OR_KEYWORD) except `_`, [RAW_IDENTIFIER](identifiers.md#grammar-RAW_IDENTIFIER), or [`$crate`](#r-macro.decl.hygiene.crate)
- `item`: an [Item](items.md#grammar-Item)
- `lifetime`: a [LIFETIME_TOKEN](tokens.md#grammar-LIFETIME_TOKEN)
- `literal`: matches `-`?[LiteralExpression](expressions/literal-expr.md#grammar-LiteralExpression)
- `meta`: an [Attr](attributes.md#grammar-Attr), the contents of an attribute
- `pat`: a [Pattern](patterns.md#grammar-Pattern) (see [macro.decl.meta.edition2021](#r-macro.decl.meta.edition2021))
- `pat_param`: a [PatternNoTopAlt](patterns.md#grammar-PatternNoTopAlt)
- `path`: a [TypePath](paths.md#grammar-TypePath)
- `stmt`: a [Statement](statements.md#grammar-Statement) without the trailing semicolon (except for item statements that require semicolons)
- `tt`: a [TokenTree](macros.md#grammar-TokenTree) (a single [token](tokens.md) or tokens in matching delimiters `()`, `[]`, or `{}`)
- `ty`: a [Type](types.md#grammar-Type)
- `vis`: a possibly empty [Visibility](visibility-and-privacy.md#grammar-Visibility) qualifier

[macro.decl.meta.transcription](#r-macro.decl.meta.transcription)

In the transcriber, metavariables are referred to simply by `$`*name*, since the fragment kind is specified in the matcher. Metavariables are replaced with the syntax element that matched them. Metavariables can be transcribed more than once or not at all.

[macro.decl.meta.dollar-crate](#r-macro.decl.meta.dollar-crate)

The keyword metavariable [`$crate`](#r-macro.decl.hygiene.crate) can be used to refer to the current crate.

[macro.decl.meta.edition2021](#r-macro.decl.meta.edition2021)

> 2021 Edition differences
>
> Starting with the 2021 edition, `pat` fragment-specifiers match top-level or-patterns (that is, they accept [Pattern](patterns.md#grammar-Pattern)).
>
> Before the 2021 edition, they match exactly the same fragments as `pat_param` (that is, they accept [PatternNoTopAlt](patterns.md#grammar-PatternNoTopAlt)).
>
> The relevant edition is the one in effect for the `macro_rules!` definition.

[macro.decl.meta.edition2024](#r-macro.decl.meta.edition2024)\
Tests Tests with this rule:

- [tests/ui/macros/expr_2021.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/expr_2021.rs)
- [tests/ui/macros/expr_2021_inline_const.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/expr_2021_inline_const.rs)
- [tests/ui/macros/expr_2024_underscore_expr.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/expr_2024_underscore_expr.rs)

> 2024 Edition differences
>
> Before the 2024 edition, `expr` fragment specifiers do not match [UnderscoreExpression](expressions/underscore-expr.md#grammar-UnderscoreExpression) or [ConstBlockExpression](expressions/block-expr.md#grammar-ConstBlockExpression) at the top level. They are allowed within subexpressions.
>
> The `expr_2021` fragment specifier exists to maintain backwards compatibility with editions before 2024.

[macro.decl.repetition](#r-macro.decl.repetition)

## Repetitions

[macro.decl.repetition.intro](#r-macro.decl.repetition.intro)

In both the matcher and transcriber, repetitions are indicated by placing the tokens to be repeated inside `$(`…`)`, followed by a repetition operator, optionally with a separator token between.

[macro.decl.repetition.separator](#r-macro.decl.repetition.separator)

The separator token can be any token other than a delimiter or one of the repetition operators, but `;` and `,` are the most common. For instance, `$( $i:ident ),*` represents any number of identifiers separated by commas. Nested repetitions are permitted.

[macro.decl.repetition.operators](#r-macro.decl.repetition.operators)

The repetition operators are:

- `*` — indicates any number of repetitions.
- `+` — indicates any number but at least one.
- `?` — indicates an optional fragment with zero or one occurrence.

[macro.decl.repetition.optional-restriction](#r-macro.decl.repetition.optional-restriction)

Since `?` represents at most one occurrence, it cannot be used with a separator.

[macro.decl.repetition.fragment](#r-macro.decl.repetition.fragment)

The repeated fragment both matches and transcribes to the specified number of the fragment, separated by the separator token. Metavariables are matched to every repetition of their corresponding fragment. For instance, the `$( $i:ident ),*` example above matches `$i` to all of the identifiers in the list.

During transcription, additional restrictions apply to repetitions so that the compiler knows how to expand them properly:

1.  A metavariable must appear in exactly the same number, kind, and nesting order of repetitions in the transcriber as it did in the matcher. So for the matcher `$( $i:ident ),*`, the transcribers `=> { $i }`, `=> { $( $( $i )* )* }`, and `=> { $( $i )+ }` are all illegal, but `=> { $( $i );* }` is correct and replaces a comma-separated list of identifiers with a semicolon-separated list.
2.  Each repetition in the transcriber must contain at least one metavariable to decide how many times to expand it. If multiple metavariables appear in the same repetition, they must be bound to the same number of fragments. For instance, `( $( $i:ident ),* ; $( $j:ident ),* ) => (( $( ($i,$j) ),* ))` must bind the same number of `$i` fragments as `$j` fragments. This means that invoking the macro with `(a, b, c; d, e, f)` is legal and expands to `((a,d), (b,e), (c,f))`, but `(a, b, c; d, e)` is illegal because it does not have the same number. This requirement applies to every layer of nested repetitions.

[macro.decl.scope](#r-macro.decl.scope)

## Scoping, exporting, and importing

[macro.decl.scope.intro](#r-macro.decl.scope.intro)

For historical reasons, the scoping of macros by example does not work entirely like items. Macros have two forms of scope: textual scope, and path-based scope. Textual scope is based on the order that things appear in source files, or even across multiple files, and is the default scoping. It is explained further below. Path-based scope works exactly the same way that item scoping does. The scoping, exporting, and importing of macros is controlled largely by attributes.

[macro.decl.scope.unqualified](#r-macro.decl.scope.unqualified)

When a macro is invoked by an unqualified identifier (not part of a multi-part path), it is first looked up in textual scoping. If this does not yield any results, then it is looked up in path-based scoping. If the macro’s name is qualified with a path, then it is only looked up in path-based scoping.

``` rust
use lazy_static::lazy_static; // Path-based import.

macro_rules! lazy_static { // Textual definition.
    (lazy) => {};
}

lazy_static!{lazy} // Textual lookup finds our macro first.
self::lazy_static!{} // Path-based lookup ignores our macro, finds imported one.
```

[macro.decl.scope.textual](#r-macro.decl.scope.textual)

### Textual scope

[macro.decl.scope.textual.intro](#r-macro.decl.scope.textual.intro)

Textual scope is based largely on the order that things appear in source files, and works similarly to the scope of local variables declared with `let` except it also applies at the module level. When `macro_rules!` is used to define a macro, the macro enters the scope after the definition (note that it can still be used recursively, since names are looked up from the invocation site), up until its surrounding scope, typically a module, is closed. This can enter child modules and even span across multiple files:

``` rust
//// src/lib.rs
mod has_macro {
    // m!{} // Error: m is not in scope.

    macro_rules! m {
        () => {};
    }
    m!{} // OK: appears after declaration of m.

    mod uses_macro;
}

// m!{} // Error: m is not in scope.

//// src/has_macro/uses_macro.rs

m!{} // OK: appears after declaration of m in src/lib.rs
```

[macro.decl.scope.textual.shadow](#r-macro.decl.scope.textual.shadow)\
Tests Tests with this rule:

- [tests/ui/macros/macro-shadowing.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/macro-shadowing.rs)

It is not an error to define a macro multiple times; the most recent declaration will shadow the previous one unless it has gone out of scope.

``` playground
#![allow(unused)]
fn main() {
macro_rules! m {
    (1) => {};
}

m!(1);

mod inner {
    m!(1);

    macro_rules! m {
        (2) => {};
    }
    // m!(1); // Error: no rule matches '1'
    m!(2);

    macro_rules! m {
        (3) => {};
    }
    m!(3);
}

m!(1);
}
```

[macro.decl.scope.textual.function-local](#r-macro.decl.scope.textual.function-local)

Macros can be declared and used locally inside functions as well, and work similarly:

``` playground
#![allow(unused)]
fn main() {
fn foo() {
    // m!(); // Error: m is not in scope.
    macro_rules! m {
        () => {};
    }
    m!();
}

// m!(); // Error: m is not in scope.
}
```

[macro.decl.scope.textual.shadow-path-based](#r-macro.decl.scope.textual.shadow-path-based)

Textual scope name bindings for macros shadow path-based scope bindings to macros.

``` playground
#![allow(unused)]
fn main() {
macro_rules! m2 {
    () => {
        println!("m2");
    };
}

// Resolves to path-based candidate from use declaration below.
m!(); // prints "m2\n"

// Introduce second candidate for `m` with textual scope.
//
// This shadows path-based candidate from below for the rest of this
// example.
macro_rules! m {
    () => {
        println!("m");
    };
}

// Introduce `m2` macro as path-based candidate.
//
// This item is in scope for this entire example, not just below the
// use declaration.
use m2 as m;

// Resolves to the textual macro candidate from above the use
// declaration.
m!(); // prints "m\n"
}
```

> ![](../assets/inline/macros-by-example-11.svg)Note
>
> For areas where shadowing is not allowed, see [name resolution ambiguities](names/name-resolution.md#r-names.resolution.expansion.imports.ambiguity).

[macro.decl.scope.path-based](#r-macro.decl.scope.path-based)

### Path-based scope

[macro.decl.scope.path-based.intro](#r-macro.decl.scope.path-based.intro)

By default, a macro has no path-based scope. Macros can gain path-based scope in two ways:

- [Use declaration re-export](items/use-declarations.md#use-visibility)
- [`macro_export`](#the-macro_export-attribute)

[macro.decl.scope.path.reexport](#r-macro.decl.scope.path.reexport)

Macros can be re-exported to give them path-based scope from a module other than the crate root.

``` playground
#![allow(unused)]
fn main() {
mac::m!(); // OK: Path-based lookup finds `m` in the mac module.

mod mac {
    // Introduce macro `m` with textual scope.
    macro_rules! m {
        () => {};
    }

    // Reexport with path-based scope from within `m`'s textual scope.
    pub(crate) use m;
}
}
```

[macro.decl.scope.path-based.visibility](#r-macro.decl.scope.path-based.visibility)

Macros have an implicit visibility of `pub(crate)`. `#[macro_export]` changes the implicit visibility to `pub`.

``` playground
#![allow(unused)]
fn main() {
// Implicit visibility is `pub(crate)`.
macro_rules! private_m {
    () => {};
}

// Implicit visibility is `pub`.
#[macro_export]
macro_rules! pub_m {
    () => {};
}

pub(crate) use private_m as private_macro; // OK.
pub use pub_m as pub_macro; // OK.
}
```

``` playground
#![allow(unused)]
fn main() {
// Implicit visibility is `pub(crate)`.
macro_rules! private_m {
    () => {};
}

// Implicit visibility is `pub`.
#[macro_export]
macro_rules! pub_m {
    () => {};
}

pub(crate) use private_m as private_macro; // OK.
pub use pub_m as pub_macro; // OK.

pub use private_m; // ERROR: `private_m` is only public within
                   // the crate and cannot be re-exported outside.
}
```

[macro.decl.scope.macro_use](#r-macro.decl.scope.macro_use)

### The `macro_use` attribute

[macro.decl.scope.macro_use.intro](#r-macro.decl.scope.macro_use.intro)

The *`macro_use` [attribute](attributes.md#r-attributes)* has two purposes: it may be used on modules to extend the scope of macros defined within them, and it may be used on [`extern crate`](items/extern-crates.md#r-items.extern-crate) to import macros from another crate into the [`macro_use` prelude](names/preludes.md#macro_use-prelude).

> ![](../assets/inline/macros-by-example-12.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> #[macro_use]
> mod inner {
>     macro_rules! m {
>         () => {};
>     }
> }
> m!();
> }
> ```
>
> ``` rust
> #[macro_use]
> extern crate log;
> ```

[macro.decl.scope.macro_use.syntax](#r-macro.decl.scope.macro_use.syntax)\
Tests Tests with this rule:

- [tests/ui/macros/macro-use-bad-args-1.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/macro-use-bad-args-1.rs)
- [tests/ui/macros/macro-use-bad-args-2.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/macro-use-bad-args-2.rs)
- [tests/ui/macros/macro-use-both.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/macro-use-both.rs)
- [tests/ui/macros/module-macro_use-arguments.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/module-macro_use-arguments.rs)

When used on modules, the `macro_use` attribute uses the [MetaWord](attributes.md#grammar-MetaWord) syntax.

When used on `extern crate`, it uses the [MetaWord](attributes.md#grammar-MetaWord) and [MetaListIdents](attributes.md#grammar-MetaListIdents) syntaxes. For more on how these syntaxes may be used, see [macro.decl.scope.macro_use.prelude](#r-macro.decl.scope.macro_use.prelude).

[macro.decl.scope.macro_use.allowed-positions](#r-macro.decl.scope.macro_use.allowed-positions)

The `macro_use` attribute may be applied to modules or `extern crate`.

> ![](../assets/inline/macros-by-example-13.svg)Note
>
> `rustc` ignores use in other positions but lints against it. This may become an error in the future.

[macro.decl.scope.macro_use.extern-crate-self](#r-macro.decl.scope.macro_use.extern-crate-self)

The `macro_use` attribute may not be used on [`extern crate self`](items/extern-crates.md#r-items.extern-crate.self).

[macro.decl.scope.macro_use.duplicates](#r-macro.decl.scope.macro_use.duplicates)\
Tests Tests with this rule:

- [tests/ui/macros/macro-use-all-and-none.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/macro-use-all-and-none.rs)

The `macro_use` attribute may be used any number of times on a form.

Multiple instances of `macro_use` in the [MetaListIdents](attributes.md#grammar-MetaListIdents) syntax may be specified. The union of all specified macros will be imported.

> ![](../assets/inline/macros-by-example-14.svg)Note
>
> On modules, `rustc` lints against any [MetaWord](attributes.md#grammar-MetaWord) `macro_use` attributes following the first.
>
> On `extern crate`, `rustc` lints against any `macro_use` attributes that have no effect due to not importing any macros not already imported by another `macro_use` attribute. If two or more [MetaListIdents](attributes.md#grammar-MetaListIdents) `macro_use` attributes import the same macro, the first is linted against. If any [MetaWord](attributes.md#grammar-MetaWord) `macro_use` attributes are present, all [MetaListIdents](attributes.md#grammar-MetaListIdents) `macro_use` attributes are linted against. If two or more [MetaWord](attributes.md#grammar-MetaWord) `macro_use` attributes are present, the ones following the first are linted against.

[macro.decl.scope.macro_use.mod-decl](#r-macro.decl.scope.macro_use.mod-decl)

When `macro_use` is used on a module, the module’s macro scope extends beyond the module’s lexical scope.

> ![](../assets/inline/macros-by-example-15.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> #[macro_use]
> mod inner {
>     macro_rules! m {
>         () => {};
>     }
> }
> m!(); // OK
> }
> ```

[macro.decl.scope.macro_use.prelude](#r-macro.decl.scope.macro_use.prelude)\
Tests Tests with this rule:

- [tests/ui/macros/macro-use-both.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/macro-use-both.rs)

Specifying `macro_use` on an `extern crate` declaration in the crate root imports exported macros from that crate.

Macros imported this way are imported into the [`macro_use` prelude](names/preludes.md#macro_use-prelude), not textually, which means that they can be shadowed by any other name. Macros imported by `macro_use` can be used before the import statement.

> ![](../assets/inline/macros-by-example-16.svg)Note
>
> `rustc` currently prefers the last macro imported in case of conflict. Don’t rely on this. This behavior is unusual, as imports in Rust are generally order-independent. This behavior of `macro_use` may change in the future.
>
> For details, see [Rust issue \#148025](https://github.com/rust-lang/rust/issues/148025).

When using the [MetaWord](attributes.md#grammar-MetaWord) syntax, all exported macros are imported. When using the [MetaListIdents](attributes.md#grammar-MetaListIdents) syntax, only the specified macros are imported.

> ![](../assets/inline/macros-by-example-17.svg)Example
>
> ``` rust
> #[macro_use(lazy_static)] // Or `#[macro_use]` to import all macros.
> extern crate lazy_static;
>
> lazy_static!{}
> // self::lazy_static!{} // ERROR: lazy_static is not defined in `self`.
> ```

[macro.decl.scope.macro_use.export](#r-macro.decl.scope.macro_use.export)

Macros to be imported with `macro_use` must be exported with [`macro_export`](#r-macro.decl.scope.macro_export).

[macro.decl.scope.macro_export](#r-macro.decl.scope.macro_export)

### The `macro_export` attribute

[macro.decl.scope.macro_export.intro](#r-macro.decl.scope.macro_export.intro)

The *`macro_export` [attribute](attributes.md#r-attributes)* exports the macro from the crate and makes it available in the root of the crate for path-based resolution.

> ![](../assets/inline/macros-by-example-18.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> self::m!();
> //  ^^^^ OK: Path-based lookup finds `m` in the current module.
> m!(); // As above.
>
> mod inner {
>     super::m!();
>     crate::m!();
> }
>
> mod mac {
>     #[macro_export]
>     macro_rules! m {
>         () => {};
>     }
> }
> }
> ```

[macro.decl.scope.macro_export.syntax](#r-macro.decl.scope.macro_export.syntax)

The `macro_export` attribute uses the [MetaWord](attributes.md#grammar-MetaWord) and [MetaListIdents](attributes.md#grammar-MetaListIdents) syntaxes. With the [MetaListIdents](attributes.md#grammar-MetaListIdents) syntax, it accepts a single [`local_inner_macros`](#r-macro.decl.scope.macro_export.local_inner_macros) value.

[macro.decl.scope.macro_export.allowed-positions](#r-macro.decl.scope.macro_export.allowed-positions)

The `macro_export` attribute may be applied to `macro_rules` definitions.

> ![](../assets/inline/macros-by-example-19.svg)Note
>
> `rustc` ignores use in other positions but lints against it. This may become an error in the future.

[macro.decl.scope.macro_export.duplicates](#r-macro.decl.scope.macro_export.duplicates)

Only the first use of `macro_export` on a macro has effect.

> ![](../assets/inline/macros-by-example-20.svg)Note
>
> `rustc` lints against any use following the first.

[macro.decl.scope.macro_export.path-based](#r-macro.decl.scope.macro_export.path-based)

By default, macros only have [textual scope](#r-macro.decl.scope.textual) and cannot be resolved by path. When the `macro_export` attribute is used, the macro is made available in the crate root and can be referred to by its path.

> ![](../assets/inline/macros-by-example-21.svg)Example
>
> Without `macro_export`, macros only have textual scope, so path-based resolution of the macro fails.
>
> ``` playground
> macro_rules! m {
>     () => {};
> }
> self::m!(); // ERROR
> crate::m!(); // ERROR
> fn main() {}
> ```
>
> With `macro_export`, path-based resolution works.
>
> ``` playground
> #[macro_export]
> macro_rules! m {
>     () => {};
> }
> self::m!(); // OK
> crate::m!(); // OK
> fn main() {}
> ```

[macro.decl.scope.macro_export.export](#r-macro.decl.scope.macro_export.export)

The `macro_export` attribute causes a macro to be exported from the crate root so that it can be referred to in other crates by path.

> ![](../assets/inline/macros-by-example-22.svg)Example
>
> Given the following in a `log` crate:
>
> ``` playground
> #![allow(unused)]
> fn main() {
> #[macro_export]
> macro_rules! warn {
>     ($message:expr) => { eprintln!("WARN: {}", $message) };
> }
> }
> ```
>
> From another crate, you can refer to the macro by path:
>
> ``` rust
> fn main() {
>     log::warn!("example warning");
> }
> ```

[macro.decl.scope.macro_export.macro_use](#r-macro.decl.scope.macro_export.macro_use)

`macro_export` allows the use of [`macro_use`](#r-macro.decl.scope.macro_use) on an `extern crate` to import the macro into the [`macro_use` prelude](names/preludes.md#macro_use-prelude).

> ![](../assets/inline/macros-by-example-23.svg)Example
>
> Given the following in a `log` crate:
>
> ``` playground
> #![allow(unused)]
> fn main() {
> #[macro_export]
> macro_rules! warn {
>     ($message:expr) => { eprintln!("WARN: {}", $message) };
> }
> }
> ```
>
> Using `macro_use` in a dependent crate allows you to use the macro from the prelude:
>
> ``` rust
> #[macro_use]
> extern crate log;
>
> pub mod util {
>     pub fn do_thing() {
>         // Resolved via macro prelude.
>         warn!("example warning");
>     }
> }
> ```

[macro.decl.scope.macro_export.local_inner_macros](#r-macro.decl.scope.macro_export.local_inner_macros)

Adding `local_inner_macros` to the `macro_export` attribute causes all single-segment macro invocations in the macro definition to have an implicit `$crate::` prefix.

> ![](../assets/inline/macros-by-example-24.svg)Note
>
> This is intended primarily as a tool to migrate code written before [`$crate`](#r-macro.decl.hygiene.crate) was added to the language to work with Rust 2018’s path-based imports of macros. Its use is discouraged in new code.

> ![](../assets/inline/macros-by-example-25.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> #[macro_export(local_inner_macros)]
> macro_rules! helped {
>     () => { helper!() } // Automatically converted to $crate::helper!().
> }
>
> #[macro_export]
> macro_rules! helper {
>     () => { () }
> }
> }
> ```

[macro.decl.hygiene](#r-macro.decl.hygiene)

## Hygiene

[macro.decl.hygiene.intro](#r-macro.decl.hygiene.intro)\
Tests Tests with this rule:

- [tests/ui/macros/macro-hygiene-help-issue-148580.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/macro-hygiene-help-issue-148580.rs)
- [tests/ui/macros/macro-hygiene-help-issue-149604.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/macro-hygiene-help-issue-149604.rs)
- [tests/ui/macros/macro-hygiene-scope-15167.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/macro-hygiene-scope-15167.rs)

Macros by example have *mixed-site hygiene*. This means that [loop labels](expressions/loop-expr.md#loop-labels), [block labels](expressions/loop-expr.md#r-expr.loop.block-labels), and local variables are looked up at the macro definition site while other symbols are looked up at the macro invocation site. For example:

``` playground
#![allow(unused)]
fn main() {
let x = 1;
fn func() {
    unreachable!("this is never called")
}

macro_rules! check {
    () => {
        assert_eq!(x, 1); // Uses `x` from the definition site.
        func();           // Uses `func` from the invocation site.
    };
}

{
    let x = 2;
    fn func() { /* does not panic */ }
    check!();
}
}
```

Labels and local variables defined in macro expansion are not shared between invocations, so this code doesn’t compile:

``` playground
#![allow(unused)]
fn main() {
macro_rules! m {
    (define) => {
        let x = 1;
    };
    (refer) => {
        dbg!(x);
    };
}

m!(define);
m!(refer);
}
```

[macro.decl.hygiene.crate](#r-macro.decl.hygiene.crate)

A special case is the `$crate` metavariable. It refers to the crate defining the macro, and can be used at the start of the path to look up items or macros which are not in scope at the invocation site.

``` rust
//// Definitions in the `helper_macro` crate.
#[macro_export]
macro_rules! helped {
    // () => { helper!() } // This might lead to an error due to 'helper' not being in scope.
    () => { $crate::helper!() }
}

#[macro_export]
macro_rules! helper {
    () => { () }
}

//// Usage in another crate.
// Note that `helper_macro::helper` is not imported!
use helper_macro::helped;

fn unit() {
    helped!();
}
```

Note that, because `$crate` refers to the current crate, it must be used with a fully qualified module path when referring to non-macro items:

``` playground
#![allow(unused)]
fn main() {
pub mod inner {
    #[macro_export]
    macro_rules! call_foo {
        () => { $crate::inner::foo() };
    }

    pub fn foo() {}
}
}
```

[macro.decl.hygiene.vis](#r-macro.decl.hygiene.vis)

Additionally, even though `$crate` allows a macro to refer to items within its own crate when expanding, its use has no effect on visibility. An item or macro referred to must still be visible from the invocation site. In the following example, any attempt to invoke `call_foo!()` from outside its crate will fail because `foo()` is not public.

``` playground
#![allow(unused)]
fn main() {
#[macro_export]
macro_rules! call_foo {
    () => { $crate::foo() };
}

fn foo() {}
}
```

> ![](../assets/inline/macros-by-example-26.svg)Note
>
> Prior to Rust 1.30, `$crate` and [`local_inner_macros`](#r-macro.decl.scope.macro_export.local_inner_macros) were unsupported. They were added alongside [path-based imports of macros](#r-macro.decl.scope.macro_export), to ensure that helper macros did not need to be manually imported by users of a macro-exporting crate. Crates written for earlier versions of Rust that use helper macros need to be modified to use `$crate` or `local_inner_macros` to work well with path-based imports.

[macro.decl.follow-set](#r-macro.decl.follow-set)

## Follow-set ambiguity restrictions

[macro.decl.follow-set.intro](#r-macro.decl.follow-set.intro)

The parser used by the macro system is reasonably powerful, but it is limited in order to prevent ambiguity in current or future versions of the language.

[macro.decl.follow-set.token-restriction](#r-macro.decl.follow-set.token-restriction)

In particular, in addition to the rule about ambiguous expansions, a nonterminal matched by a metavariable must be followed by a token which has been decided can be safely used after that kind of match.

As an example, a macro matcher like `$i:expr [ , ]` could in theory be accepted in Rust today, since `[,]` cannot be part of a legal expression and therefore the parse would always be unambiguous. However, because `[` can start trailing expressions, `[` is not a character which can safely be ruled out as coming after an expression. If `[,]` were accepted in a later version of Rust, this matcher would become ambiguous or would misparse, breaking working code. Matchers like `$i:expr,` or `$i:expr;` would be legal, however, because `,` and `;` are legal expression separators. The specific rules are:

[macro.decl.follow-set.token-expr-stmt](#r-macro.decl.follow-set.token-expr-stmt)\
Tests Tests with this rule:

- [tests/ui/macros/macro-follow-rpass.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/macro-follow-rpass.rs)
- [tests/ui/macros/macro-follow.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/macro-follow.rs)

- `expr` and `stmt` may only be followed by one of: `=>`, `,`, or `;`.

[macro.decl.follow-set.token-pat_param](#r-macro.decl.follow-set.token-pat_param)

- `pat_param` may only be followed by one of: `=>`, `,`, `=`, `|`, `if`, or `in`.

[macro.decl.follow-set.token-pat](#r-macro.decl.follow-set.token-pat)\
Tests Tests with this rule:

- [tests/ui/macros/macro-follow-rpass.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/macro-follow-rpass.rs)
- [tests/ui/macros/macro-follow.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/macro-follow.rs)
- [tests/ui/macros/macro-pat-follow.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/macro-pat-follow.rs)
- [tests/ui/macros/macro-pat-pattern-followed-by-or-in-2021.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/macro-pat-pattern-followed-by-or-in-2021.rs)
- [tests/ui/macros/macro-pat-pattern-followed-by-or.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/macro-pat-pattern-followed-by-or.rs)
- [tests/ui/macros/macro-pat2021-pattern-followed-by-or.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/macro-pat2021-pattern-followed-by-or.rs)

- `pat` may only be followed by one of: `=>`, `,`, `=`, `if`, or `in`.

[macro.decl.follow-set.token-path-ty](#r-macro.decl.follow-set.token-path-ty)\
Tests Tests with this rule:

- [tests/ui/macros/macro-follow-rpass.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/macro-follow-rpass.rs)
- [tests/ui/macros/macro-follow.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/macro-follow.rs)

- `path` and `ty` may only be followed by one of: `=>`, `,`, `=`, `|`, `;`, `:`, `>`, `>>`, `[`, `{`, `as`, `where`, or a macro variable of `block` fragment specifier.

[macro.decl.follow-set.token-vis](#r-macro.decl.follow-set.token-vis)

- `vis` may only be followed by one of: `,`, an identifier other than a non-raw `priv`, any token that can begin a type, or a metavariable with a `ident`, `ty`, or `path` fragment specifier.

[macro.decl.follow-set.token-other](#r-macro.decl.follow-set.token-other)\
Tests Tests with this rule:

- [tests/ui/macros/macro-follow-rpass.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/macro-follow-rpass.rs)

- All other fragment specifiers have no restrictions.

[macro.decl.follow-set.edition2021](#r-macro.decl.follow-set.edition2021)\
Tests Tests with this rule:

- [tests/ui/macros/macro-follow-rpass.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/macro-follow-rpass.rs)
- [tests/ui/macros/macro-or-patterns-back-compat.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/macro-or-patterns-back-compat.rs)
- [tests/ui/macros/macro-pat-follow-2018.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/macro-pat-follow-2018.rs)
- [tests/ui/macros/macro-pat-pattern-followed-by-or.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/macro-pat-pattern-followed-by-or.rs)
- [tests/ui/macros/macro-pat2021-pattern-followed-by-or.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/macro-pat2021-pattern-followed-by-or.rs)

> 2021 Edition differences
>
> Before the 2021 edition, `pat` may also be followed by `|`.

[macro.decl.follow-set.repetition](#r-macro.decl.follow-set.repetition)\
Tests Tests with this rule:

- [tests/ui/macros/macro-followed-by-seq-bad.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/macro-followed-by-seq-bad.rs)
- [tests/ui/macros/macro-followed-by-seq.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/macro-followed-by-seq.rs)

When repetitions are involved, then the rules apply to every possible number of expansions, taking separators into account. This means:

- If the repetition includes a separator, that separator must be able to follow the contents of the repetition.
- If the repetition can repeat multiple times (`*` or `+`), then the contents must be able to follow themselves.
- The contents of the repetition must be able to follow whatever comes before, and whatever comes after must be able to follow the contents of the repetition.
- If the repetition can match zero times (`*` or `?`), then whatever comes after must be able to follow whatever comes before.

For more detail, see the [formal specification](macro-ambiguity.md).
