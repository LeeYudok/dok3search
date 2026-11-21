# Procedural macros

[macro.proc](#r-macro.proc)

# Procedural macros

[macro.proc.intro](#r-macro.proc.intro)

*Procedural macros* allow creating syntax extensions as execution of a function. Procedural macros come in one of three flavors:

- [Function-like macros](#the-proc_macro-attribute) - `custom!(...)`
- [Derive macros](#r-macro.proc.derive) - `#[derive(CustomDerive)]`
- [Attribute macros](#the-proc_macro_attribute-attribute) - `#[CustomAttribute]`

Procedural macros allow you to run code at compile time that operates over Rust syntax, both consuming and producing Rust syntax. You can sort of think of procedural macros as functions from an AST to another AST.

[macro.proc.def](#r-macro.proc.def)

Procedural macros must be defined in the root of a crate with the [crate type](linkage.md) of `proc-macro`. The macros may not be used from the crate where they are defined, and can only be used when imported in another crate.

> ![](../assets/inline/procedural-macros-1.svg)Note
>
> When using Cargo, Procedural macro crates are defined with the `proc-macro` key in your manifest:
>
> ``` toml
> [lib]
> proc-macro = true
> ```

[macro.proc.result](#r-macro.proc.result)

As functions, they must either return syntax, panic, or loop endlessly. Returned syntax either replaces or adds the syntax depending on the kind of procedural macro. Panics are caught by the compiler and are turned into a compiler error. Endless loops are not caught by the compiler which hangs the compiler.

Procedural macros run during compilation, and thus have the same resources that the compiler has. For example, standard input, error, and output are the same that the compiler has access to. Similarly, file access is the same. Because of this, procedural macros have the same security concerns that [Cargo’s build scripts](https://doc.rust-lang.org/1.100.0/cargo/reference/build-scripts.html) have.

[macro.proc.error](#r-macro.proc.error)

Procedural macros have two ways of reporting errors. The first is to panic. The second is to emit a [`compile_error`](https://doc.rust-lang.org/1.100.0/core/macro.compile_error.html) macro invocation.

[macro.proc.proc_macro-crate](#r-macro.proc.proc_macro-crate)

## The `proc_macro` crate

[macro.proc.proc_macro-crate.intro](#r-macro.proc.proc_macro-crate.intro)

Procedural macro crates almost always will link to the compiler-provided [`proc_macro` crate](https://doc.rust-lang.org/1.100.0/proc_macro/index.html). The `proc_macro` crate provides types required for writing procedural macros and facilities to make it easier.

[macro.proc.proc_macro-crate.token-stream](#r-macro.proc.proc_macro-crate.token-stream)

This crate primarily contains a [`TokenStream`](https://doc.rust-lang.org/1.100.0/proc_macro/struct.TokenStream.html) type. Procedural macros operate over *token streams* instead of AST nodes, which is a far more stable interface over time for both the compiler and for procedural macros to target. A *token stream* is roughly equivalent to `Vec<TokenTree>` where a `TokenTree` can roughly be thought of as lexical token. For example `foo` is an `Ident` token, `.` is a `Punct` token, and `1.2` is a `Literal` token. The `TokenStream` type, unlike `Vec<TokenTree>`, is cheap to clone.

[macro.proc.proc_macro-crate.span](#r-macro.proc.proc_macro-crate.span)

All tokens have an associated `Span`. A `Span` is an opaque value that cannot be modified but can be manufactured. `Span`s represent an extent of source code within a program and are primarily used for error reporting. While you cannot modify a `Span` itself, you can always change the `Span` *associated* with any token, such as through getting a `Span` from another token.

[macro.proc.hygiene](#r-macro.proc.hygiene)

## Procedural macro hygiene

Procedural macros are *unhygienic*. This means they behave as if the output token stream was simply written inline to the code it’s next to. This means that it’s affected by external items and also affects external imports.

Macro authors need to be careful to ensure their macros work in as many contexts as possible given this limitation. This often includes using absolute paths to items in libraries (for example, `::std::option::Option` instead of `Option`) or by ensuring that generated functions have names that are unlikely to clash with other functions (like `__internal_foo` instead of `foo`).

[macro.proc.proc_macro](#r-macro.proc.proc_macro)

## The `proc_macro` attribute

[macro.proc.proc_macro.intro](#r-macro.proc.proc_macro.intro)

The *`proc_macro` [attribute](attributes.md)* defines a [function-like](macros.md#r-macro.invocation) procedural macro.

> ![](../assets/inline/procedural-macros-2.svg)Example
>
> This macro definition ignores its input and emits a function `answer` into its scope.
>
> ``` rust
> #![crate_type = "proc-macro"]
> extern crate proc_macro;
> use proc_macro::TokenStream;
>
> #[proc_macro]
> pub fn make_answer(_item: TokenStream) -> TokenStream {
>     "fn answer() -> u32 { 42 }".parse().unwrap()
> }
> ```
>
> We can use it in a binary crate to print “42” to standard output.
>
> ``` rust
> extern crate proc_macro_examples;
> use proc_macro_examples::make_answer;
>
> make_answer!();
>
> fn main() {
>     println!("{}", answer());
> }
> ```

[macro.proc.proc_macro.syntax](#r-macro.proc.proc_macro.syntax)

The `proc_macro` attribute uses the [MetaWord](attributes.md#grammar-MetaWord) syntax.

[macro.proc.proc_macro.allowed-positions](#r-macro.proc.proc_macro.allowed-positions)

The `proc_macro` attribute may only be applied to a `pub` function of type `fn(TokenStream) -> TokenStream` where [`TokenStream`](https://doc.rust-lang.org/1.100.0/proc_macro/struct.TokenStream.html) comes from the [`proc_macro` crate](https://doc.rust-lang.org/1.100.0/proc_macro/index.html). It must have the [“Rust” ABI](items/functions.md#r-items.fn.extern). No other function qualifiers are allowed. It must be located in the root of the crate.

[macro.proc.proc_macro.duplicates](#r-macro.proc.proc_macro.duplicates)

The `proc_macro` attribute may only be specified once on a function.

[macro.proc.proc_macro.namespace](#r-macro.proc.proc_macro.namespace)

The `proc_macro` attribute publicly defines the macro in the [macro namespace](names/namespaces.md) in the root of the crate with the same name as the function.

[macro.proc.proc_macro.behavior](#r-macro.proc.proc_macro.behavior)

A function-like macro invocation of a function-like procedural macro will pass what is inside the delimiters of the macro invocation as the input [`TokenStream`](https://doc.rust-lang.org/1.100.0/proc_macro/struct.TokenStream.html) argument and replace the entire macro invocation with the output [`TokenStream`](https://doc.rust-lang.org/1.100.0/proc_macro/struct.TokenStream.html) of the function.

[macro.proc.proc_macro.invocation](#r-macro.proc.proc_macro.invocation)

Function-like procedural macros may be invoked in any macro invocation position, which includes:

- [Statements](statements.md)
- [Expressions](expressions.md)
- [Patterns](patterns.md)
- [Type expressions](types.md#type-expressions)
- [Item](items.md) positions, including items in [`extern` blocks](items/external-blocks.md)
- Inherent and trait [implementations](items/implementations.md)
- [Trait definitions](items/traits.md)

[macro.proc.derive](#r-macro.proc.derive)

## The `proc_macro_derive` attribute

[macro.proc.derive.intro](#r-macro.proc.derive.intro)

Applying the *`proc_macro_derive` [attribute](attributes.md)* to a function defines a *derive macro* that can be invoked by the [`derive` attribute](attributes/derive.md). These macros are given the token stream of a [struct](items/structs.md), [enum](items/enumerations.md), or [union](items/unions.md) definition and can emit new [items](items.md) after it. They can also declare and use [derive macro helper attributes](#derive-macro-helper-attributes).

> ![](../assets/inline/procedural-macros-3.svg)Example
>
> This derive macro ignores its input and appends tokens that define a function.
>
> ``` rust
> #![crate_type = "proc-macro"]
> extern crate proc_macro;
> use proc_macro::TokenStream;
>
> #[proc_macro_derive(AnswerFn)]
> pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
>     "fn answer() -> u32 { 42 }".parse().unwrap()
> }
> ```
>
> To use it, we might write:
>
> ``` rust
> extern crate proc_macro_examples;
> use proc_macro_examples::AnswerFn;
>
> #[derive(AnswerFn)]
> struct Struct;
>
> fn main() {
>     assert_eq!(42, answer());
> }
> ```

[macro.proc.derive.syntax](#r-macro.proc.derive.syntax)

The syntax for the `proc_macro_derive` attribute is:

**Syntax**\
[ProcMacroDeriveAttribute](#railroad-ProcMacroDeriveAttribute) →\
    proc_macro_derive ( [DeriveMacroName](#grammar-DeriveMacroName) ( , [DeriveMacroAttributes](#grammar-DeriveMacroAttributes) )? ,? )

[DeriveMacroName](#railroad-DeriveMacroName) → [IDENTIFIER](identifiers.md#grammar-IDENTIFIER)

[DeriveMacroAttributes](#railroad-DeriveMacroAttributes) →\
    attributes ( ( [IDENTIFIER](identifiers.md#grammar-IDENTIFIER) ( , [IDENTIFIER](identifiers.md#grammar-IDENTIFIER) )\* ,? )? )

Show Railroad

![](../assets/inline/procedural-macros-4.svg) ![](../assets/inline/procedural-macros-5.svg) ![](../assets/inline/procedural-macros-6.svg)

The name of the derive macro is given by [DeriveMacroName](#grammar-DeriveMacroName). The optional `attributes` argument is described in [macro.proc.derive.attributes](#r-macro.proc.derive.attributes).

[macro.proc.derive.allowed-positions](#r-macro.proc.derive.allowed-positions)

The `proc_macro_derive` attribute may only be applied to a `pub` function with the [Rust ABI](items/functions.md#r-items.fn.extern) defined in the root of the crate with a type of `fn(TokenStream) -> TokenStream` where [`TokenStream`](https://doc.rust-lang.org/1.100.0/proc_macro/struct.TokenStream.html) comes from the [`proc_macro` crate](https://doc.rust-lang.org/1.100.0/proc_macro/index.html). The function may be `const` and may use `extern` to explicitly specify the Rust ABI, but it may not use any other [qualifiers](items/functions.md#grammar-FunctionQualifiers) (e.g. it may not be `async` or `unsafe`).

[macro.proc.derive.duplicates](#r-macro.proc.derive.duplicates)

The `proc_macro_derive` attribute may be used only once on a function.

[macro.proc.derive.namespace](#r-macro.proc.derive.namespace)

The `proc_macro_derive` attribute publicly defines the derive macro in the [macro namespace](names/namespaces.md) in the root of the crate.

[macro.proc.derive.output](#r-macro.proc.derive.output)

The input [`TokenStream`](https://doc.rust-lang.org/1.100.0/proc_macro/struct.TokenStream.html) is the token stream of the item to which the `derive` attribute is applied. The output [`TokenStream`](https://doc.rust-lang.org/1.100.0/proc_macro/struct.TokenStream.html) must be a (possibly empty) set of items. These items are appended following the input item within the same [module](items/modules.md) or [block](expressions/block-expr.md).

[macro.proc.derive.attributes](#r-macro.proc.derive.attributes)

### Derive macro helper attributes

[macro.proc.derive.attributes.intro](#r-macro.proc.derive.attributes.intro)

Derive macros can declare *derive macro helper attributes* to be used within the scope of the [item](items.md) to which the derive macro is applied. These [attributes](attributes.md) are [inert](attributes.md#active-and-inert-attributes). While their purpose is to be used by the macro that declared them, they can be seen by any macro.

[macro.proc.derive.attributes.decl](#r-macro.proc.derive.attributes.decl)

A helper attribute for a derive macro is declared by adding its identifier to the `attributes` list in the `proc_macro_derive` attribute.

> ![](../assets/inline/procedural-macros-7.svg)Example
>
> This declares a helper attribute and then ignores it.
>
> ``` rust
> #![crate_type="proc-macro"]
> extern crate proc_macro;
> use proc_macro::TokenStream;
>
> #[proc_macro_derive(WithHelperAttr, attributes(helper))]
> pub fn derive_with_helper_attr(_item: TokenStream) -> TokenStream {
>     TokenStream::new()
> }
> ```
>
> To use it, we might write:
>
> ``` rust
> #[derive(WithHelperAttr)]
> struct Struct {
>     #[helper] field: (),
> }
> ```

[macro.proc.derive.attributes.scope](#r-macro.proc.derive.attributes.scope)

When a derive macro invocation is applied to an item, the helper attributes introduced by that derive macro become in scope 1) for attributes that are applied to that item and are applied lexically after the derive macro invocation and 2) for attributes that are applied to fields and variants inside of the item.

> ![](../assets/inline/procedural-macros-8.svg)Note
>
> rustc currently allows derive helpers to be used before the macro that introduces them. Such derive helpers used out of order may not shadow other attribute macros. This behavior is deprecated and slated for removal.
>
> ``` rust
> #[helper] // Deprecated, hard error in the future.
> #[derive(WithHelperAttr)]
> struct Struct {
>     field: (),
> }
> ```
>
> For more details, see [Rust issue \#79202](https://github.com/rust-lang/rust/issues/79202).

[macro.proc.attribute](#r-macro.proc.attribute)

## The `proc_macro_attribute` attribute

[macro.proc.attribute.intro](#r-macro.proc.attribute.intro)

The *`proc_macro_attribute` [attribute](attributes.md)* defines an *attribute macro* which can be used as an [outer attribute](attributes.md).

> ![](../assets/inline/procedural-macros-9.svg)Example
>
> This attribute macro takes the input stream and emits it as-is, effectively being a no-op attribute.
>
> ``` rust
> #![crate_type = "proc-macro"]
> extern crate proc_macro;
> use proc_macro::TokenStream;
>
> #[proc_macro_attribute]
> pub fn return_as_is(_attr: TokenStream, item: TokenStream) -> TokenStream {
>     item
> }
> ```

> ![](../assets/inline/procedural-macros-10.svg)Example
>
> This shows, in the output of the compiler, the stringified [`TokenStream`s](https://doc.rust-lang.org/1.100.0/proc_macro/struct.TokenStream.html) that attribute macros see.
>
> ``` rust
> // my-macro/src/lib.rs
> extern crate proc_macro;
> use proc_macro::TokenStream;
> #[proc_macro_attribute]
> pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
>     println!("attr: \"{attr}\"");
>     println!("item: \"{item}\"");
>     item
> }
> ```
>
> ``` rust
> // src/lib.rs
> extern crate my_macro;
>
> use my_macro::show_streams;
>
> // Example: Basic function.
> #[show_streams]
> fn invoke1() {}
> // out: attr: ""
> // out: item: "fn invoke1() {}"
>
> // Example: Attribute with input.
> #[show_streams(bar)]
> fn invoke2() {}
> // out: attr: "bar"
> // out: item: "fn invoke2() {}"
>
> // Example: Multiple tokens in the input.
> #[show_streams(multiple => tokens)]
> fn invoke3() {}
> // out: attr: "multiple => tokens"
> // out: item: "fn invoke3() {}"
>
> // Example: Delimiters in the input.
> #[show_streams { delimiters }]
> fn invoke4() {}
> // out: attr: "delimiters"
> // out: item: "fn invoke4() {}"
> ```

[macro.proc.attribute.syntax](#r-macro.proc.attribute.syntax)

The `proc_macro_attribute` attribute uses the [MetaWord](attributes.md#grammar-MetaWord) syntax.

[macro.proc.attribute.allowed-positions](#r-macro.proc.attribute.allowed-positions)

The `proc_macro_attribute` attribute may only be applied to a `pub` function of type `fn(TokenStream, TokenStream) -> TokenStream` where [`TokenStream`](https://doc.rust-lang.org/1.100.0/proc_macro/struct.TokenStream.html) comes from the [`proc_macro` crate](https://doc.rust-lang.org/1.100.0/proc_macro/index.html). It must have the [“Rust” ABI](items/functions.md#r-items.fn.extern). No other function qualifiers are allowed. It must be located in the root of the crate.

[macro.proc.attribute.duplicates](#r-macro.proc.attribute.duplicates)

The `proc_macro_attribute` attribute may only be specified once on a function.

[macro.proc.attribute.namespace](#r-macro.proc.attribute.namespace)

The `proc_macro_attribute` attribute defines the attribute in the [macro namespace](names/namespaces.md) in the root of the crate with the same name as the function.

[macro.proc.attribute.use-positions](#r-macro.proc.attribute.use-positions)

Attribute macros can only be used on:

- [Items](items.md)
- Items in [`extern` blocks](items/external-blocks.md)
- Inherent and trait [implementations](items/implementations.md)
- [Trait definitions](items/traits.md)

[macro.proc.attribute.inner](#r-macro.proc.attribute.inner)

Attribute macros cannot be used as an [inner attribute](attributes.md#r-attributes.inner).

[macro.proc.attribute.outline-mod](#r-macro.proc.attribute.outline-mod)

For any [outline modules](items/modules.md#r-items.mod.outlined) present in the macro’s input, only the tokens of the module declaration are passed; the file contents of the module are not loaded or included in the input.

[macro.proc.attribute.behavior](#r-macro.proc.attribute.behavior)

The first [`TokenStream`](https://doc.rust-lang.org/1.100.0/proc_macro/struct.TokenStream.html) parameter is the delimited token tree following the attribute’s name but not including the outer delimiters. If the applied attribute contains only the attribute name or the attribute name followed by empty delimiters, the [`TokenStream`](https://doc.rust-lang.org/1.100.0/proc_macro/struct.TokenStream.html) is empty.

The second [`TokenStream`](https://doc.rust-lang.org/1.100.0/proc_macro/struct.TokenStream.html) is the rest of the [item](items.md), including other [attributes](attributes.md) on the [item](items.md).

The item to which the attribute is applied is replaced by the zero or more items in the returned [`TokenStream`](https://doc.rust-lang.org/1.100.0/proc_macro/struct.TokenStream.html).

[macro.proc.token](#r-macro.proc.token)

## Declarative macro tokens and procedural macro tokens

[macro.proc.token.intro](#r-macro.proc.token.intro)

Declarative `macro_rules` macros and procedural macros use similar, but different definitions for tokens (or rather [`TokenTree`s](https://doc.rust-lang.org/1.100.0/proc_macro/enum.TokenTree.html).)

[macro.proc.token.macro_rules](#r-macro.proc.token.macro_rules)

Token trees in `macro_rules` (corresponding to `tt` matchers) are defined as

- Delimited groups (`(...)`, `{...}`, etc)
- All operators supported by the language, both single-character and multi-character ones (`+`, `+=`).
  - Note that this set doesn’t include the single quote `'`.
- Literals (`"string"`, `1`, etc)
  - Note that negation (e.g. `-1`) is never a part of such literal tokens, but a separate operator token.
- Identifiers, including keywords (`ident`, `r#ident`, `fn`)
- Lifetimes (`'ident`)
- Metavariable substitutions in `macro_rules` (e.g. `$my_expr` in `macro_rules! mac { ($my_expr: expr) => { $my_expr } }` after the `mac`’s expansion, which will be considered a single token tree regardless of the passed expression)

[macro.proc.token.tree](#r-macro.proc.token.tree)

Token trees in procedural macros are defined as

- Delimited groups (`(...)`, `{...}`, etc)
- All punctuation characters used in operators supported by the language (`+`, but not `+=`), and also the single quote `'` character (typically used in lifetimes, see below for lifetime splitting and joining behavior)
- Literals (`"string"`, `1`, etc)
  - Negation (e.g. `-1`) is supported as a part of integer and floating point literals.
- Identifiers, including keywords (`ident`, `r#ident`, `fn`)

[macro.proc.token.conversion.intro](#r-macro.proc.token.conversion.intro)

Mismatches between these two definitions are accounted for when token streams are passed to and from procedural macros. Note that the conversions below may happen lazily, so they might not happen if the tokens are not actually inspected.

[macro.proc.token.conversion.to-proc_macro](#r-macro.proc.token.conversion.to-proc_macro)

When passed to a proc-macro

- All multi-character operators are broken into single characters.
- Lifetimes are broken into a `'` character and an identifier.
- The keyword metavariable [`$crate`](macros-by-example.md#r-macro.decl.hygiene.crate) is passed as a single identifier.
- All other metavariable substitutions are represented as their underlying token streams.
  - Such token streams may be wrapped into delimited groups ([`Group`](https://doc.rust-lang.org/1.100.0/proc_macro/struct.Group.html)) with implicit delimiters ([`Delimiter::None`](https://doc.rust-lang.org/1.100.0/proc_macro/enum.Delimiter.html#variant.None)) when it’s necessary for preserving parsing priorities.
  - `tt` and `ident` substitutions are never wrapped into such groups and always represented as their underlying token trees.

[macro.proc.token.conversion.from-proc_macro](#r-macro.proc.token.conversion.from-proc_macro)

When emitted from a proc macro

- Punctuation characters are glued into multi-character operators when applicable.
- Single quotes `'` joined with identifiers are glued into lifetimes.
- Negative literals are converted into two tokens (the `-` and the literal) possibly wrapped into a delimited group ([`Group`](https://doc.rust-lang.org/1.100.0/proc_macro/struct.Group.html)) with implicit delimiters ([`Delimiter::None`](https://doc.rust-lang.org/1.100.0/proc_macro/enum.Delimiter.html#variant.None)) when it’s necessary for preserving parsing priorities.

[macro.proc.token.doc-comment](#r-macro.proc.token.doc-comment)

Note that neither declarative nor procedural macros support doc comment tokens (e.g. `/// Doc`), so they are always converted to token streams representing their equivalent `#[doc = r"str"]` attributes when passed to macros.
