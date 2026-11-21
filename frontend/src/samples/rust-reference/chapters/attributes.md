# Attributes

[attributes](#r-attributes)

# Attributes

[attributes.syntax](#r-attributes.syntax)

**Syntax**\
[InnerAttribute](#railroad-InnerAttribute) → \# ! \[ [Attr](#grammar-Attr) \]

[OuterAttribute](#railroad-OuterAttribute) → \# \[ [Attr](#grammar-Attr) \]

[Attr](#railroad-Attr) →\
      [SimplePath](paths.md#grammar-SimplePath) [AttrInput](#grammar-AttrInput)?\
    \| unsafe ( [SimplePath](paths.md#grammar-SimplePath) [AttrInput](#grammar-AttrInput)? )

[AttrInput](#railroad-AttrInput) →\
      [DelimTokenTree](macros.md#grammar-DelimTokenTree)\
    \| = [Expression](expressions.md#grammar-Expression)

Show Railroad

![](../assets/inline/attributes-1.svg) ![](../assets/inline/attributes-2.svg) ![](../assets/inline/attributes-3.svg) ![](../assets/inline/attributes-4.svg) [attributes.intro](#r-attributes.intro)

An *attribute* is a general, free-form metadatum that is interpreted according to name, convention, language, and compiler version. Attributes are modeled on Attributes in [ECMA-335](https://www.ecma-international.org/publications-and-standards/standards/ecma-335/), with the syntax coming from [ECMA-334](https://www.ecma-international.org/publications-and-standards/standards/ecma-334/) (C#).

[attributes.inner](#r-attributes.inner)

*Inner attributes* apply to the form that the attribute is declared within.

> ![](../assets/inline/attributes-5.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> // General metadata applied to the enclosing module or crate.
> #![crate_type = "lib"]
>
> // Inner attribute applies to the entire function.
> fn some_unused_variables() {
>   #![allow(unused_variables)]
>
>   let x = ();
>   let y = ();
>   let z = ();
> }
> }
> ```

[attributes.outer](#r-attributes.outer)

*Outer attributes* apply to the form that follows the attribute.

> ![](../assets/inline/attributes-6.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> // A function marked as a unit test
> #[test]
> fn test_foo() {
>     /* ... */
> }
>
> // A conditionally-compiled module
> #[cfg(target_os = "linux")]
> mod bar {
>     /* ... */
> }
>
> // A lint attribute used to suppress a warning/error
> #[allow(non_camel_case_types)]
> type int8_t = i8;
> }
> ```

[attributes.input](#r-attributes.input)

The attribute consists of a path to the attribute, followed by an optional delimited token tree whose interpretation is defined by the attribute. Attributes other than macro attributes also allow the input to be an equals sign (`=`) followed by an expression. See the [meta item syntax](#meta-item-attribute-syntax) below for more details.

[attributes.safety](#r-attributes.safety)

An attribute may be unsafe to apply. To avoid undefined behavior when using these attributes, certain obligations that cannot be checked by the compiler must be met. To assert these have been, the attribute is wrapped in `unsafe(..)`, e.g. `#[unsafe(no_mangle)]`.

The following attributes are unsafe:

- [`export_name`](abi.md#the-export_name-attribute)
- [`link_section`](abi.md#the-link_section-attribute)
- [`naked`](attributes/codegen.md#the-naked-attribute)
- [`no_mangle`](abi.md#the-no_mangle-attribute)

[attributes.kind](#r-attributes.kind)

Attributes can be classified into the following kinds:

- [Built-in attributes](#built-in-attributes-index)
- [Proc macro attributes](procedural-macros.md#the-proc_macro_attribute-attribute)
- [Derive macro helper attributes](procedural-macros.md#derive-macro-helper-attributes)
- [Tool attributes](#tool-attributes)

[attributes.allowed-position](#r-attributes.allowed-position)

Attributes may be applied to many forms in the language:

- All [item declarations](items.md) accept outer attributes while [external blocks](items/external-blocks.md), [functions](items/functions.md), [implementations](items/implementations.md), and [modules](items/modules.md) accept inner attributes.
- Most [statements](statements.md) accept outer attributes (see [Expression Attributes](expressions.md#expression-attributes) for limitations on expression statements).
- [Block expressions](expressions/block-expr.md) accept outer and inner attributes, but only when they are the outer expression of an [expression statement](statements.md#expression-statements) or the final expression of another block expression.
- [Enum](items/enumerations.md) variants and [struct](items/structs.md) and [union](items/unions.md) fields accept outer attributes.
- [Match expression arms](expressions/match-expr.md) accept outer attributes.
- [Generic lifetime or type parameter](items/generics.md) accept outer attributes.
- Expressions accept outer attributes in limited situations, see [Expression Attributes](expressions.md#expression-attributes) for details.
- [Function](items/functions.md), [closure](expressions/closure-expr.md) and [function pointer](types/function-pointer.md) parameters accept outer attributes. This includes attributes on variadic parameters denoted with `...` in function pointers and [external blocks](items/external-blocks.md#variadic-functions).
- [Inline assembly](inline-assembly.md) template strings and operands accept outer attributes. Only certain attributes are accepted semantically; for details, see [asm.attributes.supported-attributes](inline-assembly.md#r-asm.attributes.supported-attributes).

[attributes.meta](#r-attributes.meta)

## Meta item attribute syntax

[attributes.meta.intro](#r-attributes.meta.intro)

A “meta item” is the syntax used for the [Attr](#grammar-Attr) rule by most [built-in attributes](#built-in-attributes-index). It has the following grammar:

[attributes.meta.syntax](#r-attributes.meta.syntax)

**Syntax**\
[MetaItem](#railroad-MetaItem) →\
      [SimplePath](paths.md#grammar-SimplePath)\
    \| [SimplePath](paths.md#grammar-SimplePath) = [Expression](expressions.md#grammar-Expression)\
    \| [SimplePath](paths.md#grammar-SimplePath) ( [MetaSeq](#grammar-MetaSeq)? )

[MetaSeq](#railroad-MetaSeq) →\
    [MetaItemInner](#grammar-MetaItemInner) ( , [MetaItemInner](#grammar-MetaItemInner) )\* ,?

[MetaItemInner](#railroad-MetaItemInner) →\
      [MetaItem](#grammar-MetaItem)\
    \| [Expression](expressions.md#grammar-Expression)

Show Railroad

![](../assets/inline/attributes-7.svg) ![](../assets/inline/attributes-8.svg) ![](../assets/inline/attributes-9.svg) [attributes.meta.literal-expr](#r-attributes.meta.literal-expr)

Expressions in meta items must macro-expand to literal expressions, which must not include integer or float type suffixes. Expressions which are not literal expressions will be syntactically accepted (and can be passed to proc-macros), but will be rejected after parsing.

[attributes.meta.order](#r-attributes.meta.order)

Note that if the attribute appears within another macro, it will be expanded after that outer macro. For example, the following code will expand the `Serialize` proc-macro first, which must preserve the `include_str!` call in order for it to be expanded:

``` rust
#[derive(Serialize)]
struct Foo {
    #[doc = include_str!("x.md")]
    x: u32
}
```

[attributes.meta.order-macro](#r-attributes.meta.order-macro)

Additionally, macros in attributes will be expanded only after all other attributes applied to the item:

``` rust
#[macro_attr1] // expanded first
#[doc = mac!()] // `mac!` is expanded fourth.
#[macro_attr2] // expanded second
#[derive(MacroDerive1, MacroDerive2)] // expanded third
fn foo() {}
```

[attributes.meta.builtin](#r-attributes.meta.builtin)

Various built-in attributes use different subsets of the meta item syntax to specify their inputs. The following grammar rules show some commonly used forms:

[attributes.meta.builtin-syntax](#r-attributes.meta.builtin-syntax)

**Syntax**\
[MetaWord](#railroad-MetaWord) →\
    [IDENTIFIER](identifiers.md#grammar-IDENTIFIER)

[MetaNameValueStr](#railroad-MetaNameValueStr) →\
    [IDENTIFIER](identifiers.md#grammar-IDENTIFIER) = ( [STRING_LITERAL](tokens.md#grammar-STRING_LITERAL) \| [RAW_STRING_LITERAL](tokens.md#grammar-RAW_STRING_LITERAL) )

[MetaListPaths](#railroad-MetaListPaths) →\
    [IDENTIFIER](identifiers.md#grammar-IDENTIFIER) ( ( [SimplePath](paths.md#grammar-SimplePath) ( , [SimplePath](paths.md#grammar-SimplePath) )\* ,? )? )

[MetaListIdents](#railroad-MetaListIdents) →\
    [IDENTIFIER](identifiers.md#grammar-IDENTIFIER) ( ( [IDENTIFIER](identifiers.md#grammar-IDENTIFIER) ( , [IDENTIFIER](identifiers.md#grammar-IDENTIFIER) )\* ,? )? )

[MetaListNameValueStr](#railroad-MetaListNameValueStr) →\
    [IDENTIFIER](identifiers.md#grammar-IDENTIFIER) ( ( [MetaNameValueStr](#grammar-MetaNameValueStr) ( , [MetaNameValueStr](#grammar-MetaNameValueStr) )\* ,? )? )

Show Railroad

![](../assets/inline/attributes-10.svg) ![](../assets/inline/attributes-11.svg) ![](../assets/inline/attributes-12.svg) ![](../assets/inline/attributes-13.svg) ![](../assets/inline/attributes-14.svg)

Some examples of meta items are:

| Style | Example |
|----|----|
| [MetaWord](#grammar-MetaWord) | `no_std` |
| [MetaNameValueStr](#grammar-MetaNameValueStr) | `doc = "example"` |
| [MetaListPaths](#grammar-MetaListPaths) | `allow(unused, clippy::inline_always)` |
| [MetaListIdents](#grammar-MetaListIdents) | `macro_use(foo, bar)` |
| [MetaListNameValueStr](#grammar-MetaListNameValueStr) | `link(name = "CoreFoundation", kind = "framework")` |

[attributes.activity](#r-attributes.activity)

## Active and inert attributes

[attributes.activity.intro](#r-attributes.activity.intro)

An attribute is either active or inert. During attribute processing, *active attributes* remove themselves from the form they are on while *inert attributes* stay on.

The [`cfg`](conditional-compilation.md#the-cfg-attribute) and [`cfg_attr`](conditional-compilation.md#the-cfg_attr-attribute) attributes are active. [Attribute macros](procedural-macros.md#the-proc_macro_attribute-attribute) are active. All other attributes are inert.

[attributes.tool](#r-attributes.tool)

## Tool attributes

[attributes.tool.intro](#r-attributes.tool.intro)

The compiler may allow attributes for external tools where each tool resides in its own module in the [tool prelude](names/preludes.md#tool-prelude). The first segment of the attribute path is the name of the tool, with one or more additional segments whose interpretation is up to the tool.

[attributes.tool.ignored](#r-attributes.tool.ignored)

When a tool is not in use, the tool’s attributes are accepted without a warning. When the tool is in use, the tool is responsible for processing and interpretation of its attributes.

[attributes.tool.prelude](#r-attributes.tool.prelude)

Tool attributes are not available if the [`no_implicit_prelude`](names/preludes.md#the-no_implicit_prelude-attribute) attribute is used.

``` playground
#![allow(unused)]
fn main() {
// Tells the rustfmt tool to not format the following element.
#[rustfmt::skip]
struct S {
}

// Controls the "cyclomatic complexity" threshold for the clippy tool.
#[clippy::cyclomatic_complexity = "100"]
pub fn f() {}
}
```

> ![](../assets/inline/attributes-15.svg)Note
>
> `rustc` currently recognizes the tools “clippy”, “rustfmt”, “diagnostic”, “miri”, and “rust_analyzer”.

[attributes.builtin](#r-attributes.builtin)

## Built-in attributes index

The following is an index of all built-in attributes.

- Conditional compilation

  - [`cfg`](conditional-compilation.md#the-cfg-attribute) — Controls conditional compilation.
  - [`cfg_attr`](conditional-compilation.md#the-cfg_attr-attribute) — Conditionally includes attributes.

- Testing

  - [`test`](attributes/testing.md#the-test-attribute) — Marks a function as a test.
  - [`ignore`](attributes/testing.md#the-ignore-attribute) — Disables a test function.
  - [`should_panic`](attributes/testing.md#the-should_panic-attribute) — Indicates a test should generate a panic.

- Derive

  - [`derive`](attributes/derive.md) — Automatic trait implementations.
  - [`automatically_derived`](attributes/derive.md#the-automatically_derived-attribute) — Marker for implementations created by `derive`.

- Macros

  - [`macro_export`](macros-by-example.md#the-macro_export-attribute) — Exports a `macro_rules` macro for cross-crate use.
  - [`macro_use`](macros-by-example.md#the-macro_use-attribute) — Expands macro visibility, or imports macros from other crates.
  - [`proc_macro`](procedural-macros.md#the-proc_macro-attribute) — Defines a function-like macro.
  - [`proc_macro_derive`](procedural-macros.md#r-macro.proc.derive) — Defines a derive macro.
  - [`proc_macro_attribute`](procedural-macros.md#the-proc_macro_attribute-attribute) — Defines an attribute macro.

- Diagnostics

  - [`allow`](attributes/diagnostics.md#lint-check-attributes), [`expect`](attributes/diagnostics.md#lint-check-attributes), [`warn`](attributes/diagnostics.md#lint-check-attributes), [`deny`](attributes/diagnostics.md#lint-check-attributes), [`forbid`](attributes/diagnostics.md#lint-check-attributes) — Alters the default lint level.
  - [`deprecated`](attributes/diagnostics.md#the-deprecated-attribute) — Generates deprecation notices.
  - [`must_use`](attributes/diagnostics.md#the-must_use-attribute) — Generates a lint for unused values.
  - [`diagnostic::on_unimplemented`](attributes/diagnostics.md#the-diagnosticon_unimplemented-attribute) — Hints the compiler to emit a certain error message if a trait is not implemented.
  - [`diagnostic::do_not_recommend`](attributes/diagnostics.md#the-diagnosticdo_not_recommend-attribute) — Hints the compiler to not show a certain trait impl in error messages.

- ABI, linking, symbols, and FFI

  - [`link`](items/external-blocks.md#the-link-attribute) — Specifies a native library to link with an `extern` block.
  - [`link_name`](items/external-blocks.md#the-link_name-attribute) — Specifies the name of the symbol for functions or statics in an `extern` block.
  - [`link_ordinal`](items/external-blocks.md#the-link_ordinal-attribute) — Specifies the ordinal of the symbol for functions or statics in an `extern` block.
  - [`no_link`](items/extern-crates.md#the-no_link-attribute) — Prevents linking an extern crate.
  - [`repr`](type-layout.md#representations) — Controls type layout.
  - [`crate_type`](linkage.md) — Specifies the type of crate (library, executable, etc.).
  - [`no_main`](crates-and-source-files.md#the-no_main-attribute) — Disables emitting the `main` symbol.
  - [`export_name`](abi.md#the-export_name-attribute) — Specifies the exported symbol name for a function or static.
  - [`link_section`](abi.md#the-link_section-attribute) — Specifies the section of an object file to use for a function or static.
  - [`no_mangle`](abi.md#the-no_mangle-attribute) — Disables symbol name encoding.
  - [`used`](abi.md#the-used-attribute) — Forces the compiler to keep a static item in the output object file.
  - [`crate_name`](crates-and-source-files.md#the-crate_name-attribute) — Specifies the crate name.

- Code generation

  - [`inline`](attributes/codegen.md#the-inline-attribute) — Hint to inline code.
  - [`cold`](attributes/codegen.md#the-cold-attribute) — Hint that a function is unlikely to be called.
  - [`naked`](attributes/codegen.md#the-naked-attribute) — Prevent the compiler from emitting a function prologue and epilogue.
  - [`no_builtins`](attributes/codegen.md#the-no_builtins-attribute) — Disables use of certain built-in functions.
  - [`target_feature`](attributes/codegen.md#the-target_feature-attribute) — Configure platform-specific code generation.
  - [`track_caller`](attributes/codegen.md#the-track_caller-attribute) — Pass the parent call location to `std::panic::Location::caller()`.
  - [`instruction_set`](attributes/codegen.md#the-instruction_set-attribute) — Specify the instruction set used to generate a function’s code.

- Documentation

  - `doc` — Specifies documentation. See [The Rustdoc Book](https://doc.rust-lang.org/1.100.0/rustdoc/the-doc-attribute.html) for more information. [Doc comments](comments.md#doc-comments) are transformed into `doc` attributes.

- Preludes

  - [`no_std`](names/preludes.md#the-no_std-attribute) — Removes std from the prelude.
  - [`no_implicit_prelude`](names/preludes.md#the-no_implicit_prelude-attribute) — Disables prelude lookups within a module.

- Modules

  - [`path`](items/modules.md#the-path-attribute) — Specifies the filename for a module.

- Limits

  - [`recursion_limit`](attributes/limits.md#the-recursion_limit-attribute) — Sets the maximum recursion limit for certain compile-time operations.
  - [`type_length_limit`](attributes/limits.md#the-type_length_limit-attribute) — Sets the maximum size of a polymorphic type.

- Runtime

  - [`panic_handler`](panic.md#the-panic_handler-attribute) — Sets the function to handle panics.
  - [`global_allocator`](runtime.md#the-global_allocator-attribute) — Sets the global memory allocator.
  - [`windows_subsystem`](runtime.md#the-windows_subsystem-attribute) — Specifies the windows subsystem to link with.

- Features

  - `feature` — Used to enable unstable or experimental compiler features. See [The Unstable Book](https://doc.rust-lang.org/1.100.0/unstable-book/index.html) for features implemented in `rustc`.

- Type System

  - [`non_exhaustive`](attributes/type_system.md#the-non_exhaustive-attribute) — Indicate that a type will have more fields/variants added in future.

- Debugger

  - [`debugger_visualizer`](attributes/debugger.md#the-debugger_visualizer-attribute) — Embeds a file that specifies debugger output for a type.
  - [`collapse_debuginfo`](attributes/debugger.md#the-collapse_debuginfo-attribute) — Controls how macro invocations are encoded in debuginfo.
