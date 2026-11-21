# Names

[names](#r-names)

# Names

[names.intro](#r-names.intro)

An *entity* is a language construct that can be referred to in some way within the source program, usually via a [path](paths.md). Entities include [types](types.md), [items](items.md), [generic parameters](items/generics.md), [variable bindings](patterns.md), [loop labels](expressions/loop-expr.md#loop-labels), [lifetimes](tokens.md#lifetimes-and-loop-labels), [fields](expressions/field-expr.md), [attributes](attributes.md), and [lints](attributes/diagnostics.md#lint-check-attributes).

A *declaration* is a syntactical construct that can introduce a *name* to refer to an entity. Entity names are valid within a [*scope*](names/scopes.md) — a region of source text where that name may be referenced.

Some entities are [explicitly declared](#explicitly-declared-entities) in the source code, and some are [implicitly declared](#implicitly-declared-entities) as part of the language or compiler extensions.

[*Paths*](paths.md) are used to refer to an entity, possibly in another module or type.

Lifetimes and loop labels use a [dedicated syntax](tokens.md#lifetimes-and-loop-labels) using a leading quote.

Names are segregated into different [*namespaces*](names/namespaces.md), allowing entities in different namespaces to share the same name without conflict.

[*Name resolution*](names/name-resolution.md) is the compile-time process of tying paths, identifiers, and labels to entity declarations.

Access to certain names may be restricted based on their [*visibility*](visibility-and-privacy.md).

[names.explicit](#r-names.explicit)

## Explicitly declared entities

[names.explicit.list](#r-names.explicit.list)

Entities that explicitly introduce a name in the source code are:

[names.explicit.item-decl](#r-names.explicit.item-decl)

- [Items](items.md):
  - [Module declarations](items/modules.md)
  - [External crate declarations](items/extern-crates.md)
  - [Use declarations](items/use-declarations.md)
  - [Function declarations](items/functions.md) and [function parameters](items/functions.md#function-parameters)
  - [Type aliases](items/type-aliases.md)
  - [struct](items/structs.md), [union](items/unions.md), [enum](items/enumerations.md), enum variant declarations, and their named fields
  - [Constant item declarations](items/constant-items.md)
  - [Static item declarations](items/static-items.md)
  - [Trait item declarations](items/traits.md) and their [associated items](items/associated-items.md)
  - [External block items](items/external-blocks.md)
  - [`macro_rules` declarations](macros-by-example.md) and [matcher metavariables](macros-by-example.md#metavariables)
  - [Implementation](items/implementations.md) associated items

[names.explicit.expr](#r-names.explicit.expr)

- [Expressions](expressions.md):
  - [Closure](expressions/closure-expr.md) parameters
  - [`while let`](expressions/loop-expr.md#while-let-patterns) pattern bindings
  - [`for`](expressions/loop-expr.md#iterator-loops) pattern bindings
  - [`if let`](expressions/if-expr.md#if-let-patterns) pattern bindings
  - [`match`](expressions/match-expr.md) pattern bindings
  - [Loop labels](expressions/loop-expr.md#loop-labels)

[names.explicit.generics](#r-names.explicit.generics)

- [Generic parameters](items/generics.md)

[names.explicit.higher-ranked-bounds](#r-names.explicit.higher-ranked-bounds)

- [Higher ranked trait bounds](trait-bounds.md#higher-ranked-trait-bounds)

[names.explicit.binding](#r-names.explicit.binding)

- [`let` statement](statements.md#let-statements) pattern bindings

[names.explicit.macro_use](#r-names.explicit.macro_use)

- The [`macro_use` attribute](macros-by-example.md#the-macro_use-attribute) can introduce macro names from another crate

[names.explicit.macro_export](#r-names.explicit.macro_export)

- The [`macro_export` attribute](macros-by-example.md#the-macro_export-attribute) can introduce an alias for the macro into the crate root

[names.explicit.macro-invocation](#r-names.explicit.macro-invocation)

Additionally, [macro invocations](macros.md#macro-invocation) and [attributes](attributes.md) can introduce names by expanding to one of the above items.

[names.implicit](#r-names.implicit)

## Implicitly declared entities

[names.implicit.list](#r-names.implicit.list)

The following entities are implicitly defined by the language, or are introduced by compiler options and extensions:

[names.implicit.primitive-types](#r-names.implicit.primitive-types)

- [Language prelude](names/preludes.md#language-prelude):
  - [Boolean type](types/boolean.md) — `bool`
  - Textual types — [`char`](types/char.md) and [`str`](types/str.md)
  - [Integer types](types/numeric.md#integer-types) — `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`
  - [Machine-dependent integer types](types/numeric.md#machine-dependent-integer-types) — `usize` and `isize`
  - [floating-point types](types/numeric.md#floating-point-types) — `f32` and `f64`

[names.implicit.builtin-attributes](#r-names.implicit.builtin-attributes)

- [Built-in attributes](attributes.md#built-in-attributes-index)

[names.implicit.prelude](#r-names.implicit.prelude)

- [Standard library prelude](names/preludes.md#standard-library-prelude) items, attributes, and macros

[names.implicit.stdlib](#r-names.implicit.stdlib)

- [Standard library](names/preludes.md#extern-prelude) crates in the root module

[names.implicit.extern-prelude](#r-names.implicit.extern-prelude)

- [External crates](names/preludes.md#extern-prelude) linked by the compiler

[names.implicit.tool-attributes](#r-names.implicit.tool-attributes)

- [Tool attributes](attributes.md#tool-attributes)

[names.implicit.lints](#r-names.implicit.lints)

- [Lints](attributes/diagnostics.md#lint-check-attributes) and [tool lint attributes](attributes/diagnostics.md#tool-lint-attributes)

[names.implicit.derive-helpers](#r-names.implicit.derive-helpers)

- [Derive helper attributes](procedural-macros.md#derive-macro-helper-attributes) are valid within an item without being explicitly imported

[names.implicit.lifetime-static](#r-names.implicit.lifetime-static)

- The [`'static`](keywords.md#weak-keywords) lifetime

[names.implicit.root](#r-names.implicit.root)

Additionally, the crate root module does not have a name, but can be referred to with certain [path qualifiers](paths.md#path-qualifiers) or aliases.
