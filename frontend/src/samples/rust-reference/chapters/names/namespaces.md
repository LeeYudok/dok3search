# Namespaces

[names.namespaces](#r-names.namespaces)

# Namespaces

[names.namespaces.intro](#r-names.namespaces.intro)

A *namespace* is a logical grouping of declared [names](../names.md). Names are segregated into separate namespaces based on the kind of entity the name refers to. Namespaces allow the occurrence of a name in one namespace to not conflict with the same name in another namespace.

There are several different namespaces that each contain different kinds of entities. The use of a name will look for the declaration of that name in different namespaces, based on the context, as described in the [name resolution](name-resolution.md) chapter.

[names.namespaces.kinds](#r-names.namespaces.kinds)

The following is a list of namespaces, with their corresponding entities:

- Type Namespace
  - [Module declarations](../items/modules.md)
  - [External crate declarations](../items/extern-crates.md)
  - [External crate prelude](preludes.md#extern-prelude) items
  - [Struct](../items/structs.md), [union](../items/unions.md), [enum](../items/enumerations.md), enum variant declarations
  - [Trait item declarations](../items/traits.md)
  - [Type aliases](../items/type-aliases.md)
  - [Associated type declarations](../items/associated-items.md#associated-types)
  - Built-in types: [boolean](../types/boolean.md), [numeric](../types/numeric.md), [`char`](../types/char.md), and [`str`](../types/str.md)
  - [Generic type parameters](../items/generics.md)
  - [`Self` type](../paths.md#self-1)
  - [Tool attribute modules](../attributes.md#tool-attributes)
- Value Namespace
  - [Function declarations](../items/functions.md)
  - [Constant item declarations](../items/constant-items.md)
  - [Static item declarations](../items/static-items.md)
  - [Struct constructors](../items/structs.md)
  - [Enum variant constructors](../items/enumerations.md)
  - [`Self` constructors](../paths.md#self-1)
  - [Generic const parameters](../items/generics.md#const-generics)
  - [Associated const declarations](../items/associated-items.md#associated-constants)
  - [Associated function declarations](../items/associated-items.md#associated-functions-and-methods)
  - Local bindings — [`let`](../statements.md#let-statements), [`if let`](../expressions/if-expr.md#if-let-patterns), [`while let`](../expressions/loop-expr.md#while-let-patterns), [`for`](../expressions/loop-expr.md#iterator-loops), [`match`](../expressions/match-expr.md) arms, [function parameters](../items/functions.md#function-parameters), [closure parameters](../expressions/closure-expr.md)
  - Captured [closure](../expressions/closure-expr.md) variables
- Macro Namespace
  - [`macro_rules` declarations](../macros-by-example.md)
  - [Built-in attributes](../attributes.md#built-in-attributes-index)
  - [Tool attributes](../attributes.md#tool-attributes)
  - [Function-like procedural macros](../procedural-macros.md#the-proc_macro-attribute)
  - [Derive macros](../procedural-macros.md#r-macro.proc.derive)
  - [Derive macro helpers](../procedural-macros.md#derive-macro-helper-attributes)
  - [Attribute macros](../procedural-macros.md#the-proc_macro_attribute-attribute)
- Lifetime Namespace
  - [Generic lifetime parameters](../items/generics.md)
- Label Namespace
  - [Loop labels](../expressions/loop-expr.md#loop-labels)
  - [Block labels](../expressions/loop-expr.md#r-expr.loop.block-labels)

An example of how overlapping names in different namespaces can be used unambiguously:

``` playground
#![allow(unused)]
fn main() {
// Foo introduces a type in the type namespace and a constructor in the value
// namespace.
struct Foo(u32);

// The `Foo` macro is declared in the macro namespace.
macro_rules! Foo {
    () => {};
}

// `Foo` in the `f` parameter type refers to `Foo` in the type namespace.
// `'Foo` introduces a new lifetime in the lifetime namespace.
fn example<'Foo>(f: Foo) {
    // `Foo` refers to the `Foo` constructor in the value namespace.
    let ctor = Foo;
    // `Foo` refers to the `Foo` macro in the macro namespace.
    Foo!{}
    // `'Foo` introduces a label in the label namespace.
    'Foo: loop {
        // `'Foo` refers to the `'Foo` lifetime parameter, and `Foo`
        // refers to the type namespace.
        let x: &'Foo Foo;
        // `'Foo` refers to the label.
        break 'Foo;
    }
}
}
```

[names.namespaces.without](#r-names.namespaces.without)

## Named entities without a namespace

[names.namespaces.without.intro](#r-names.namespaces.without.intro)

The following entities have explicit names, but the names are not a part of any specific namespace.

### Fields

[names.namespaces.without.fields](#r-names.namespaces.without.fields)

Even though struct, enum, and union fields are named, the named fields do not live in an explicit namespace. They can only be accessed via a [field expression](../expressions/field-expr.md), which only inspects the field names of the specific type being accessed.

### Use declarations

[names.namespaces.without.use](#r-names.namespaces.without.use)

A [use declaration](../items/use-declarations.md) has named aliases that it imports into scope, but the `use` item itself does not belong to a specific namespace. Instead, it can introduce aliases into multiple namespaces, depending on the item kind being imported.

[names.namespaces.sub-namespaces](#r-names.namespaces.sub-namespaces)

## Sub-namespaces

[names.namespaces.sub-namespaces.intro](#r-names.namespaces.sub-namespaces.intro)\
Tests Tests with this rule:

- [tests/ui/macros/macro-rules-as-derive-or-attr-issue-132928.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/macro-rules-as-derive-or-attr-issue-132928.rs)

The macro namespace is split into two sub-namespaces: one for [bang-style macros](../macros.md) and one for [attributes](../attributes.md). When an attribute is resolved, any bang-style macros in scope will be ignored. And conversely resolving a bang-style macro will ignore attribute macros in scope. This prevents one style from shadowing another.

For example, the [`cfg` attribute](../conditional-compilation.md#the-cfg-attribute) and the [`cfg` macro](../conditional-compilation.md#the-cfg-macro) are two different entities with the same name in the macro namespace, but they can still be used in their respective context.

> ![](../../assets/inline/namespaces-1.svg)Note
>
> `use` imports still cannot create duplicate bindings of the same name in a module or block, regardless of sub-namespace.
>
> ``` rust
> #[macro_export]
> macro_rules! mymac {
>     () => {};
> }
>
> use myattr::mymac; // error[E0252]: the name `mymac` is defined multiple times.
> ```
