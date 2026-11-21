# Modules

[items.mod](#r-items.mod)

# Modules

[items.mod.syntax](#r-items.mod.syntax)

**Syntax**\
[Module](#railroad-Module) →\
      unsafe? mod [IDENTIFIER](../identifiers.md#grammar-IDENTIFIER) ;\
    \| unsafe? mod [IDENTIFIER](../identifiers.md#grammar-IDENTIFIER) {\
        [InnerAttribute](../attributes.md#grammar-InnerAttribute)\*\
        [Item](../items.md#grammar-Item)\*\
      }

Show Railroad

![](../../assets/inline/modules-1.svg) [items.mod.intro](#r-items.mod.intro)

A module is a container for zero or more [items](../items.md).

[items.mod.def](#r-items.mod.def)

A *module item* is a module, surrounded in braces, named, and prefixed with the keyword `mod`. A module item introduces a new, named module into the tree of modules making up a crate.

[items.mod.nesting](#r-items.mod.nesting)

Modules can nest arbitrarily.

An example of a module:

``` playground
#![allow(unused)]
fn main() {
mod math {
    type Complex = (f64, f64);
    fn sin(f: f64) -> f64 {
        /* ... */
      unimplemented!();
    }
    fn cos(f: f64) -> f64 {
        /* ... */
      unimplemented!();
    }
    fn tan(f: f64) -> f64 {
        /* ... */
      unimplemented!();
    }
}
}
```

[items.mod.namespace](#r-items.mod.namespace)

Modules are defined in the [type namespace](../names/namespaces.md) of the module or block where they are located.

[items.mod.multiple-items](#r-items.mod.multiple-items)

It is an error to define multiple items with the same name in the same namespace within a module. See the [scopes chapter](../names/scopes.md) for more details on restrictions and shadowing behavior.

[items.mod.unsafe](#r-items.mod.unsafe)

The `unsafe` keyword is syntactically allowed to appear before the `mod` keyword, but it is rejected at a semantic level. This allows macros to consume the syntax and make use of the `unsafe` keyword, before removing it from the token stream.

[items.mod.outlined](#r-items.mod.outlined)

## Module source filenames

[items.mod.outlined.intro](#r-items.mod.outlined.intro)

A module without a body is loaded from an external file. When the module does not have a `path` attribute, the path to the file mirrors the logical [module path](../paths.md).

[items.mod.outlined.search](#r-items.mod.outlined.search)

Ancestor module path components are directories, and the module’s contents are in a file with the name of the module plus the `.rs` extension. For example, the following module structure can have this corresponding filesystem structure:

| Module Path           | Filesystem Path  | File Contents |
|-----------------------|------------------|---------------|
| `crate`               | `lib.rs`         | `mod util;`   |
| `crate::util`         | `util.rs`        | `mod config;` |
| `crate::util::config` | `util/config.rs` |               |

[items.mod.outlined.search-mod](#r-items.mod.outlined.search-mod)

Module filenames may also be the name of the module as a directory with the contents in a file named `mod.rs` within that directory. The above example can alternately be expressed with `crate::util`’s contents in a file named `util/mod.rs`. It is not allowed to have both `util.rs` and `util/mod.rs`.

> ![](../../assets/inline/modules-2.svg)Note
>
> Prior to `rustc` 1.30, using `mod.rs` files was the way to load a module with nested children. It is encouraged to use the new naming convention as it is more consistent, and avoids having many files named `mod.rs` within a project.

[items.mod.outlined.path](#r-items.mod.outlined.path)

### The `path` attribute

[items.mod.outlined.path.intro](#r-items.mod.outlined.path.intro)

The directories and files used for loading external file modules can be influenced with the `path` attribute.

[items.mod.outlined.path.search](#r-items.mod.outlined.path.search)

For `path` attributes on modules not inside inline module blocks, the file path is relative to the directory the source file is located. For example, the following code snippet would use the paths shown based on where it is located:

``` rust
#[path = "foo.rs"]
mod c;
```

| Source File    | `c`’s File Location | `c`’s Module Path |
|----------------|---------------------|-------------------|
| `src/a/b.rs`   | `src/a/foo.rs`      | `crate::a::b::c`  |
| `src/a/mod.rs` | `src/a/foo.rs`      | `crate::a::c`     |

[items.mod.outlined.path.search-nested](#r-items.mod.outlined.path.search-nested)

For `path` attributes inside inline module blocks, the relative location of the file path depends on the kind of source file the `path` attribute is located in. “mod-rs” source files are root modules (such as `lib.rs` or `main.rs`) and modules with files named `mod.rs`. “non-mod-rs” source files are all other module files. Paths for `path` attributes inside inline module blocks in a mod-rs file are relative to the directory of the mod-rs file including the inline module components as directories. For non-mod-rs files, it is the same except the path starts with a directory with the name of the non-mod-rs module. For example, the following code snippet would use the paths shown based on where it is located:

``` rust
mod inline {
    #[path = "other.rs"]
    mod inner;
}
```

| Source File    | `inner`’s File Location   | `inner`’s Module Path        |
|----------------|---------------------------|------------------------------|
| `src/a/b.rs`   | `src/a/b/inline/other.rs` | `crate::a::b::inline::inner` |
| `src/a/mod.rs` | `src/a/inline/other.rs`   | `crate::a::inline::inner`    |

An example of combining the above rules of `path` attributes on inline modules and nested modules within (applies to both mod-rs and non-mod-rs files):

``` rust
#[path = "thread_files"]
mod thread {
    // Load the `local_data` module from `thread_files/tls.rs` relative to
    // this source file's directory.
    #[path = "tls.rs"]
    mod local_data;
}
```

[items.mod.attributes](#r-items.mod.attributes)

## Attributes on modules

[items.mod.attributes.intro](#r-items.mod.attributes.intro)

Modules, like all items, accept outer attributes. They also accept inner attributes: either after `{` for a module with a body, or at the beginning of the source file, after the optional BOM and shebang.

[items.mod.attributes.supported](#r-items.mod.attributes.supported)

The built-in attributes that have meaning on a module are [`cfg`](../conditional-compilation.md), [`deprecated`](../attributes/diagnostics.md#the-deprecated-attribute), [`doc`](../../rustdoc/the-doc-attribute.html), [the lint check attributes](../attributes/diagnostics.md#lint-check-attributes), [`path`](#the-path-attribute), and [`no_implicit_prelude`](../names/preludes.md#the-no_implicit_prelude-attribute). Modules also accept macro attributes.
