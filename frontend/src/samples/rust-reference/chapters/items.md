# Items

[items](#r-items)

# Items

[items.syntax](#r-items.syntax)

**Syntax**\
[Item](#railroad-Item) →\
    [OuterAttribute](attributes.md#grammar-OuterAttribute)\* ( [VisItem](#grammar-VisItem) \| [MacroItem](#grammar-MacroItem) )

[VisItem](#railroad-VisItem) →\
    [Visibility](visibility-and-privacy.md#grammar-Visibility)?\
    (\
        [Module](items/modules.md#grammar-Module)\
      \| [ExternCrate](items/extern-crates.md#grammar-ExternCrate)\
      \| [UseDeclaration](items/use-declarations.md#grammar-UseDeclaration)\
      \| [Function](items/functions.md#grammar-Function)\
      \| [TypeAlias](items/type-aliases.md#grammar-TypeAlias)\
      \| [Struct](items/structs.md#grammar-Struct)\
      \| [Enumeration](items/enumerations.md#grammar-Enumeration)\
      \| [Union](items/unions.md#grammar-Union)\
      \| [ConstantItem](items/constant-items.md#grammar-ConstantItem)\
      \| [StaticItem](items/static-items.md#grammar-StaticItem)\
      \| [Trait](items/traits.md#grammar-Trait)\
      \| [Implementation](items/implementations.md#grammar-Implementation)\
      \| [ExternBlock](items/external-blocks.md#grammar-ExternBlock)\
    )

[MacroItem](#railroad-MacroItem) →\
      [MacroInvocationSemi](macros.md#grammar-MacroInvocationSemi)\
    \| [MacroRulesDefinition](macros-by-example.md#grammar-MacroRulesDefinition)

Show Railroad

![](../assets/inline/items-1.svg) ![](../assets/inline/items-2.svg) ![](../assets/inline/items-3.svg) [items.intro](#r-items.intro)

An *item* is a component of a crate. Items are organized within a crate by a nested set of [modules](items/modules.md). Every crate has a single “outermost” anonymous module; all further items within the crate have [paths](paths.md) within the module tree of the crate.

[items.static-def](#r-items.static-def)

Items are entirely determined at compile-time, generally remain fixed during execution, and may reside in read-only memory.

[items.kinds](#r-items.kinds)

There are several kinds of items:

- [modules](items/modules.md)
- [`extern crate` declarations](items/extern-crates.md)
- [`use` declarations](items/use-declarations.md)
- [function definitions](items/functions.md)
- [type alias definitions](items/type-aliases.md)
- [struct definitions](items/structs.md)
- [enumeration definitions](items/enumerations.md)
- [union definitions](items/unions.md)
- [constant items](items/constant-items.md)
- [static items](items/static-items.md)
- [trait definitions](items/traits.md)
- [implementations](items/implementations.md)
- [`extern` blocks](items/external-blocks.md)

[items.locations](#r-items.locations)

Items may be declared in the [root of the crate](crates-and-source-files.md), a [module](items/modules.md), or a [block expression](expressions/block-expr.md).

[items.associated-locations](#r-items.associated-locations)

A subset of items, called [associated items](items/associated-items.md), may be declared in [traits](items/traits.md) and [implementations](items/implementations.md).

[items.extern-locations](#r-items.extern-locations)

A subset of items, called external items, may be declared in [`extern` blocks](items/external-blocks.md).

[items.decl-order](#r-items.decl-order)

Items may be defined in any order, with the exception of [`macro_rules`](macros-by-example.md) which has its own scoping behavior.

[items.name-resolution](#r-items.name-resolution)

[Name resolution](names/name-resolution.md) of item names allows items to be defined before or after where the item is referred to in the module or block.

See [item scopes](names/scopes.md#item-scopes) for information on the scoping rules of items.
