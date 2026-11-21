# Structs

[items.struct](#r-items.struct)

# Structs

[items.struct.syntax](#r-items.struct.syntax)

**Syntax**\
[Struct](#railroad-Struct) →\
      [StructStruct](#grammar-StructStruct)\
    \| [TupleStruct](#grammar-TupleStruct)

[StructStruct](#railroad-StructStruct) →\
    struct [IDENTIFIER](../identifiers.md#grammar-IDENTIFIER) [GenericParams](generics.md#grammar-GenericParams)? [WhereClause](generics.md#grammar-WhereClause)? ( { [StructFields](#grammar-StructFields)? } \| ; )

[TupleStruct](#railroad-TupleStruct) →\
    struct [IDENTIFIER](../identifiers.md#grammar-IDENTIFIER) [GenericParams](generics.md#grammar-GenericParams)? ( [TupleFields](#grammar-TupleFields)? ) [WhereClause](generics.md#grammar-WhereClause)? ;

[StructFields](#railroad-StructFields) → [StructField](#grammar-StructField) ( , [StructField](#grammar-StructField) )\* ,?

[StructField](#railroad-StructField) → [OuterAttribute](../attributes.md#grammar-OuterAttribute)\* [Visibility](../visibility-and-privacy.md#grammar-Visibility)? [IDENTIFIER](../identifiers.md#grammar-IDENTIFIER) : [Type](../types.md#grammar-Type)

[TupleFields](#railroad-TupleFields) → [TupleField](#grammar-TupleField) ( , [TupleField](#grammar-TupleField) )\* ,?

[TupleField](#railroad-TupleField) → [OuterAttribute](../attributes.md#grammar-OuterAttribute)\* [Visibility](../visibility-and-privacy.md#grammar-Visibility)? [Type](../types.md#grammar-Type)

Show Railroad

![](../../assets/inline/structs-1.svg) ![](../../assets/inline/structs-2.svg) ![](../../assets/inline/structs-3.svg) ![](../../assets/inline/structs-4.svg) ![](../../assets/inline/structs-5.svg) ![](../../assets/inline/structs-6.svg) ![](../../assets/inline/structs-7.svg) [items.struct.intro](#r-items.struct.intro)

A *struct* is a nominal [struct type](../types/struct.md) defined with the keyword `struct`.

[items.struct.namespace](#r-items.struct.namespace)

A struct declaration defines the given name in the [type namespace](../names/namespaces.md) of the module or block where it is located.

An example of a `struct` item and its use:

``` playground
#![allow(unused)]
fn main() {
struct Point {x: i32, y: i32}
let p = Point {x: 10, y: 11};
let px: i32 = p.x;
}
```

[items.struct.tuple](#r-items.struct.tuple)

A *tuple struct* is a nominal [tuple type](../types/tuple.md), and is also defined with the keyword `struct`. In addition to defining a type, it also defines a constructor of the same name in the [value namespace](../names/namespaces.md). The constructor is a function which can be called to create a new instance of the struct. For example:

``` playground
#![allow(unused)]
fn main() {
struct Point(i32, i32);
let p = Point(10, 11);
let px: i32 = match p { Point(x, _) => x };
}
```

[items.struct.unit](#r-items.struct.unit)

A *unit-like struct* is a struct without any fields, defined by leaving off the list of fields entirely. Such a struct implicitly defines a [constant](constant-items.md) of its type with the same name. For example:

``` playground
#![allow(unused)]
fn main() {
struct Cookie;
let c = [Cookie, Cookie {}, Cookie, Cookie {}];
}
```

is equivalent to

``` playground
#![allow(unused)]
fn main() {
struct Cookie {}
const Cookie: Cookie = Cookie {};
let c = [Cookie, Cookie {}, Cookie, Cookie {}];
}
```

[items.struct.layout](#r-items.struct.layout)

The precise memory layout of a struct is not specified. One can specify a particular layout using the [`repr` attribute](../type-layout.md#representations).
