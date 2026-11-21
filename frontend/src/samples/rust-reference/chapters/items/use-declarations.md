# Use declarations

[items.use](#r-items.use)

# Use declarations

[items.use.syntax](#r-items.use.syntax)

**Syntax**\
[UseDeclaration](#railroad-UseDeclaration) → use [UseTree](#grammar-UseTree) ;

[UseTree](#railroad-UseTree) →\
      ( [SimplePath](../paths.md#grammar-SimplePath)? :: )? \*\
    \| ( [SimplePath](../paths.md#grammar-SimplePath)? :: )? { ( [UseTree](#grammar-UseTree) ( , [UseTree](#grammar-UseTree) )\* ,? )? }\
    \| [SimplePath](../paths.md#grammar-SimplePath) ( as ( [IDENTIFIER](../identifiers.md#grammar-IDENTIFIER) \| \_ ) )?

Show Railroad

![](../../assets/inline/use-declarations-1.svg) ![](../../assets/inline/use-declarations-2.svg) [items.use.intro](#r-items.use.intro)

A *use declaration* creates one or more local name bindings synonymous with some other [path](../paths.md). Usually a `use` declaration is used to shorten the path required to refer to a module item. These declarations may appear in [modules](modules.md) and [blocks](../expressions/block-expr.md), usually at the top. A `use` declaration is also sometimes called an *import*, or, if it is public, a *re-export*.

[items.use.forms](#r-items.use.forms)

Use declarations support a number of convenient shortcuts:

[items.use.forms-multiple](#r-items.use.forms-multiple)

- Simultaneously binding a list of paths with a common prefix, using the brace syntax `use a::b::{c, d, e::f, g::h::i};`

[items.use.forms-self](#r-items.use.forms-self)

- Simultaneously binding a list of paths with a common prefix and their common parent module, using the `self` keyword, such as `use a::b::{self, c, d::e};`

[items.use.forms-as](#r-items.use.forms-as)

- Rebinding the target name as a new local name, using the syntax `use p::q::r as x;`. This can also be used with the last two features: `use a::b::{self as ab, c as abc}`.

[items.use.forms-glob](#r-items.use.forms-glob)

- Binding all paths matching a given prefix, using the asterisk wildcard syntax `use a::b::*;`.

[items.use.forms-nesting](#r-items.use.forms-nesting)

- Nesting groups of the previous features multiple times, such as `use a::b::{self as ab, c, d::{*, e::f}};`

An example of `use` declarations:

``` playground
use std::collections::hash_map::{self, HashMap};

fn foo<T>(_: T){}
fn bar(map1: HashMap<String, usize>, map2: hash_map::HashMap<String, usize>){}

fn main() {
    // use declarations can also exist inside of functions
    use std::option::Option::{Some, None};

    // Equivalent to 'foo(vec![std::option::Option::Some(1.0f64),
    // std::option::Option::None]);'
    foo(vec![Some(1.0f64), None]);

    // Both `hash_map` and `HashMap` are in scope.
    let map1 = HashMap::new();
    let map2 = hash_map::HashMap::new();
    bar(map1, map2);
}
```

[items.use.visibility](#r-items.use.visibility)

## `use` Visibility

[items.use.visibility.intro](#r-items.use.visibility.intro)

Like items, `use` declarations are private to the containing module, by default. Also like items, a `use` declaration can be public, if qualified by the `pub` keyword. Such a `use` declaration serves to *re-export* a name. A public `use` declaration can therefore *redirect* some public name to a different target definition: even a definition with a private canonical path, inside a different module.

[items.use.visibility.unambiguous](#r-items.use.visibility.unambiguous)

If a sequence of such redirections form a cycle or cannot be resolved unambiguously, they represent a compile-time error.

An example of re-exporting:

``` playground
mod quux {
    pub use self::foo::{bar, baz};
    pub mod foo {
        pub fn bar() {}
        pub fn baz() {}
    }
}

fn main() {
    quux::bar();
    quux::baz();
}
```

In this example, the module `quux` re-exports two public names defined in `foo`.

[items.use.path](#r-items.use.path)

## `use` Paths

[items.use.path.intro](#r-items.use.path.intro)

The [paths](../paths.md) that are allowed in a `use` item follow the [SimplePath](../paths.md#grammar-SimplePath) grammar and are similar to the paths that may be used in an expression. They may create bindings for:

- Nameable [items](../items.md)
- [Enum variants](enumerations.md)
- [Built-in types](../types.md)
- [Attributes](../attributes.md)
- [Derive macros](../procedural-macros.md#r-macro.proc.derive)
- [`macro_rules`](../macros-by-example.md)

[items.use.path.disallowed](#r-items.use.path.disallowed)

They cannot import [associated items](associated-items.md), [generic parameters](generics.md), [local variables](../variables.md), paths with [`Self`](../paths.md#self), or [tool attributes](../attributes.md#tool-attributes). More restrictions are described below.

[items.use.path.namespace](#r-items.use.path.namespace)

`use` will create bindings for all [namespaces](../names/namespaces.md) from the imported entities, with the exception that a `self` import will only import from the type namespace (as described below). For example, the following illustrates creating bindings for the same name in two namespaces:

``` playground
#![allow(unused)]
fn main() {
mod stuff {
    pub struct Foo(pub i32);
}

// Imports the `Foo` type and the `Foo` constructor.
use stuff::Foo;

fn example() {
    let ctor = Foo; // Uses `Foo` from the value namespace.
    let x: Foo = ctor(123); // Uses `Foo` From the type namespace.
}
}
```

[items.use.path.edition2018](#r-items.use.path.edition2018)

> 2018 Edition differences
>
> In the 2015 edition, `use` paths are relative to the crate root. For example:
>
> ``` playground
> mod foo {
>     pub mod example { pub mod iter {} }
>     pub mod baz { pub fn foobaz() {} }
> }
> mod bar {
>     // Resolves `foo` from the crate root.
>     use foo::example::iter;
>     // The `::` prefix explicitly resolves `foo`
>     // from the crate root.
>     use ::foo::baz::foobaz;
> }
>
> fn main() {}
> ```
>
> The 2015 edition does not allow use declarations to reference the [extern prelude](../names/preludes.md#extern-prelude). Thus, [`extern crate`](extern-crates.md) declarations are still required in 2015 to reference an external crate in a `use` declaration. Beginning with the 2018 edition, `use` declarations can specify an external crate dependency the same way `extern crate` can.

[items.use.as](#r-items.use.as)

## `as` renames

The `as` keyword can be used to change the name of an imported entity. For example:

``` playground
#![allow(unused)]
fn main() {
// Creates a non-public alias `bar` for the function `foo`.
use inner::foo as bar;

mod inner {
    pub fn foo() {}
}
}
```

[items.use.multiple-syntax](#r-items.use.multiple-syntax)

## Brace syntax

[items.use.multiple-syntax.intro](#r-items.use.multiple-syntax.intro)

Braces can be used in the last segment of the path to import multiple entities from the previous segment, or, if there are no previous segments, from the current scope. Braces can be nested, creating a tree of paths, where each grouping of segments is logically combined with its parent to create a full path.

``` playground
#![allow(unused)]
fn main() {
// Creates bindings to:
// - `std::collections::BTreeSet`
// - `std::collections::hash_map`
// - `std::collections::hash_map::HashMap`
use std::collections::{BTreeSet, hash_map::{self, HashMap}};
}
```

[items.use.multiple-syntax.empty](#r-items.use.multiple-syntax.empty)

An empty brace does not import anything, though the leading path is validated that it is accessible.

[items.use.multiple-syntax.edition2018](#r-items.use.multiple-syntax.edition2018)

> 2018 Edition differences
>
> In the 2015 edition, paths are relative to the crate root, so an import such as `use {foo, bar};` will import the names `foo` and `bar` from the crate root, whereas starting in 2018, those names are relative to the current scope.

[items.use.self](#r-items.use.self)

## `self` imports

[items.use.self.intro](#r-items.use.self.intro)

The keyword `self` may be used within [brace syntax](#r-items.use.multiple-syntax) to create a binding of the parent entity under its own name.

``` playground
mod stuff {
    pub fn foo() {}
    pub fn bar() {}
}
mod example {
    // Creates a binding for `stuff` and `foo`.
    use crate::stuff::{self, foo};
    pub fn baz() {
        foo();
        stuff::bar();
    }
}
fn main() {}
```

> ![](../../assets/inline/use-declarations-3.svg)Note
>
> `self` may also be used as the first segment of a path. The use of `self` as the first segment and inside a `use` brace is logically the same; it means the current module of the parent segment, or the current module if there is no parent segment. See [`self`](../paths.md#self) in the paths chapter for more information on the meaning of a leading `self`.

[items.use.self.trailing](#r-items.use.self.trailing)

`self` may appear as the last segment of a `use` path, preceded by `::`. A path of the form `P::self` is equivalent to `P::{self}`, and `P::self as name` is equivalent to `P::{self as name}`.

``` playground
mod m {
    pub enum E { V1, V2 }
}
use m::self as _; // Equivalent to `use m::{self as _};`.
use m::E::self; // Equivalent to `use m::E::{self};`.
fn main() {}
```

> ![](../../assets/inline/use-declarations-4.svg)Note
>
> See [paths.qualifiers.mod-self.trailing](../paths.md#r-paths.qualifiers.mod-self.trailing) for restrictions on the preceding path.

[items.use.self.module](#r-items.use.self.module)

When `self` is used within [brace syntax](#r-items.use.multiple-syntax), the path preceding the brace group must resolve to a [module](modules.md#r-items.mod), [enumeration](enumerations.md#r-items.enum), or [trait](traits.md#r-items.traits).

``` playground
mod m {
    pub enum E { V1, V2 }
    pub trait Tr { fn f(&self); }
}
use m::{self as _}; // OK: Modules can be parents of `self`.
use m::E::{self, V1}; // OK: Enums can be parents of `self`.
use m::Tr::{self}; // OK: Traits can be parents of `self`.
fn main() {}
```

``` playground
struct S {}
use S::{self as _}; // ERROR: Structs cannot be parents of `self`.
fn main() {}
```

[items.use.self.namespace](#r-items.use.self.namespace)

`self` only creates a binding from the [type namespace](../names/namespaces.md) of the parent entity. For example, in the following, only the `foo` mod is imported:

``` playground
mod bar {
    pub mod foo {}
    pub fn foo() {}
}

// This only imports the module `foo`. The function `foo` lives in
// the value namespace and is not imported.
use bar::foo::{self};

fn main() {
    foo(); //~ ERROR `foo` is a module
}
```

[items.use.glob](#r-items.use.glob)

## Glob imports

[items.use.glob.intro](#r-items.use.glob.intro)

The `*` character may be used as the last segment of a `use` path to import all importable entities from the entity of the preceding segment. For example:

``` playground
#![allow(unused)]
fn main() {
// Creates a non-public alias to `bar`.
use foo::*;

mod foo {
    fn i_am_private() {}
    enum Example {
        V1,
        V2,
    }
    pub fn bar() {
        // Creates local aliases to `V1` and `V2`
        // of the `Example` enum.
        use Example::*;
        let x = V1;
    }
}
}
```

[items.use.glob.shadowing](#r-items.use.glob.shadowing)

Items and named imports are allowed to shadow names from glob imports in the same [namespace](../names/namespaces.md). That is, if there is a name already defined by another item in the same namespace, the glob import will be shadowed. For example:

``` playground
#![allow(unused)]
fn main() {
// This creates a binding to the `clashing::Foo` tuple struct
// constructor, but does not import its type because that would
// conflict with the `Foo` struct defined here.
//
// Note that the order of definition here is unimportant.
use clashing::*;
struct Foo {
    field: f32,
}

fn do_stuff() {
    // Uses the constructor from `clashing::Foo`.
    let f1 = Foo(123);
    // The struct expression uses the type from
    // the `Foo` struct defined above.
    let f2 = Foo { field: 1.0 };
    // `Bar` is also in scope due to the glob import.
    let z = Bar {};
}

mod clashing {
    pub struct Foo(pub i32);
    pub struct Bar {}
}
}
```

> ![](../../assets/inline/use-declarations-5.svg)Note
>
> For areas where shadowing is not allowed, see [name resolution ambiguities](../names/name-resolution.md#r-names.resolution.expansion.imports.ambiguity).

[items.use.glob.last-segment-only](#r-items.use.glob.last-segment-only)

`*` cannot be used as the first or intermediate segments.

[items.use.glob.self-import](#r-items.use.glob.self-import)

`*` cannot be used to import a module’s contents into itself (such as `use self::*;`).

[items.use.glob.edition2018](#r-items.use.glob.edition2018)

> 2018 Edition differences
>
> In the 2015 edition, paths are relative to the crate root, so an import such as `use *;` is valid, and it means to import everything from the crate root. This cannot be used in the crate root itself.

[items.use.as-underscore](#r-items.use.as-underscore)

## Underscore imports

[items.use.as-underscore.intro](#r-items.use.as-underscore.intro)

Items can be imported without binding to a name by using an underscore with the form `use path as _`. This is particularly useful to import a trait so that its methods may be used without importing the trait’s symbol, for example if the trait’s symbol may conflict with another symbol. Another example is to link an external crate without importing its name.

``` playground
mod foo {
    pub trait Zoo {
        fn zoo(&self) {}
    }

    impl<T> Zoo for T {}
}

use self::foo::Zoo as _;
struct Zoo;  // Underscore import avoids name conflict with this item.

fn main() {
    let z = Zoo;
    z.zoo();
}
```

[items.use.as-underscore.glob](#r-items.use.as-underscore.glob)

Asterisk glob imports will import items imported with `_` in their unnameable form.

[items.use.as-underscore.macro](#r-items.use.as-underscore.macro)

The unique, unnameable symbols are created after macro expansion so that macros may safely emit multiple references to `_` imports. For example, the following should not produce an error:

``` playground
#![allow(unused)]
fn main() {
macro_rules! m {
    ($item: item) => { $item $item }
}

m!(use std as _;);
// This expands to:
// use std as _;
// use std as _;
}
```

[items.use.restrictions](#r-items.use.restrictions)

## Restrictions

[items.use.restrictions.intro](#r-items.use.restrictions.intro)

The following rules are restrictions for valid `use` declarations.

[items.use.restrictions.crate-alias](#r-items.use.restrictions.crate-alias)

When using `crate` to import the current crate, you must use `as` to define the binding name.

> ![](../../assets/inline/use-declarations-6.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> use crate as root;
> use crate::{self as root2};
>
> // Not allowed:
> // use crate;
> // use crate::{self};
> }
> ```

[items.use.restrictions.macro-crate-alias](#r-items.use.restrictions.macro-crate-alias)

When using [`$crate`](../paths.md#r-paths.qualifiers.macro-crate) in a macro transcriber to import the current crate, you must use `as` to define the binding name.

> ![](../../assets/inline/use-declarations-7.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> macro_rules! import_crate_root {
>     () => {
>         use $crate as my_crate;
>         use $crate::{self as my_crate2};
>     };
> }
> }
> ```

[items.use.restrictions.self-alias](#r-items.use.restrictions.self-alias)

When using `self` to import the current module, you must use `as` to define the binding name.

> ![](../../assets/inline/use-declarations-8.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> use {self as this_module};
> use self as this_module2;
> use self::{self as this_module3};
>
> // Not allowed:
> // use {self};
> // use self;
> // use self::{self};
> }
> ```

[items.use.restrictions.super-alias](#r-items.use.restrictions.super-alias)

When using `super` to import a parent module, you must use `as` to define the binding name.

> ![](../../assets/inline/use-declarations-9.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> mod a {
>     mod b {
>         use super as parent;
>         use super::{self as parent2};
>         use self::super as parent3;
>         use super::super as grandparent;
>         use super::super::{self as grandparent2};
>
>         // Not allowed:
>         // use super;
>         // use super::{self};
>         // use self::super;
>         // use super::super;
>         // use super::super::{self};
>     }
> }
> }
> ```

[items.use.restrictions.extern-prelude](#r-items.use.restrictions.extern-prelude)

`::` as the [extern prelude](../names/preludes.md#extern-prelude) cannot be imported.

> ![](../../assets/inline/use-declarations-10.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> use ::{self as root}; //~ Error
> }
> ```

> 2018 Edition differences
>
> In the 2015 edition, the prefix `::` refers to the crate root, so `use ::{self as root};` is allowed because it is same as `use crate::{self as root};`. Starting with the 2018 edition the `::` prefix refers to the extern prelude, which cannot be directly imported.
>
> ``` playground
> #![allow(unused)]
> fn main() {
> use ::{self as root}; //~ Ok
> }
> ```

[items.use.restrictions.duplicate-name](#r-items.use.restrictions.duplicate-name)

As with any item definition, `use` imports cannot create duplicate bindings of the same name in the same namespace in a module or block.

[items.use.restrictions.variant](#r-items.use.restrictions.variant)

`use` paths cannot refer to enum variants through a [type alias](type-aliases.md).

> ![](../../assets/inline/use-declarations-11.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> enum MyEnum {
>   MyVariant
> }
> type TypeAlias = MyEnum;
>
> use MyEnum::MyVariant; //~ OK
> use TypeAlias::MyVariant; //~ ERROR
> }
> ```
