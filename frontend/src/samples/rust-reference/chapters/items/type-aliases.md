# Type aliases

[items.type](#r-items.type)

# Type aliases

[items.type.syntax](#r-items.type.syntax)

**Syntax**\
[TypeAlias](#railroad-TypeAlias) →\
    type [IDENTIFIER](../identifiers.md#grammar-IDENTIFIER) [GenericParams](generics.md#grammar-GenericParams)? ( : [Bounds](../trait-bounds.md#grammar-Bounds)? )?\
        [WhereClause](generics.md#grammar-WhereClause)?\
        ( = [Type](../types.md#grammar-Type) [WhereClause](generics.md#grammar-WhereClause)? )? ;

Show Railroad

![](../../assets/inline/type-aliases-1.svg) [items.type.intro](#r-items.type.intro)

A *type alias* defines a new name for an existing [type](../types.md) in the [type namespace](../names/namespaces.md) of the module or block where it is located. Type aliases are declared with the keyword `type`. Every value has a single, specific type, but may implement several different traits, and may be compatible with several different type constraints.

For example, the following defines the type `Point` as a synonym for the type `(u8, u8)`, the type of pairs of unsigned 8 bit integers:

``` playground
#![allow(unused)]
fn main() {
type Point = (u8, u8);
let p: Point = (41, 68);
}
```

[items.type.constructor-alias](#r-items.type.constructor-alias)

A type alias to a tuple-struct or unit-struct cannot be used to qualify that type’s constructor:

``` playground
#![allow(unused)]
fn main() {
struct MyStruct(u32);

use MyStruct as UseAlias;
type TypeAlias = MyStruct;

let _ = UseAlias(5); // OK
let _ = TypeAlias(5); // Doesn't work
}
```

[items.type.associated-type](#r-items.type.associated-type)

A type alias, when not used as an [associated type](associated-items.md#associated-types), must include a [Type](../types.md#grammar-Type) and may not include [Bounds](../trait-bounds.md#grammar-Bounds).

[items.type.associated-trait](#r-items.type.associated-trait)

A type alias, when used as an [associated type](associated-items.md#associated-types) in a [trait](traits.md), must not include a [Type](../types.md#grammar-Type) specification but may include [Bounds](../trait-bounds.md#grammar-Bounds).

[items.type.associated-impl](#r-items.type.associated-impl)

A type alias, when used as an [associated type](associated-items.md#associated-types) in a [trait impl](implementations.md#trait-implementations), must include a [Type](../types.md#grammar-Type) specification and may not include [Bounds](../trait-bounds.md#grammar-Bounds).

[items.type.deprecated](#r-items.type.deprecated)

Where clauses before the equals sign on a type alias in a [trait impl](implementations.md#trait-implementations) (like `type TypeAlias<T> where T: Foo = Bar<T>`) are deprecated. Where clauses after the equals sign (like `type TypeAlias<T> = Bar<T> where T: Foo`) are preferred.
