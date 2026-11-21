# Crates and source files

[crate](#r-crate)

# Crates and source files

[crate.syntax](#r-crate.syntax)

**Syntax**\
[Crate](#railroad-Crate) →\
    [InnerAttribute](attributes.md#grammar-InnerAttribute)\*\
    [Item](items.md#grammar-Item)\*

Show Railroad

![](../assets/inline/crates-and-source-files-1.svg)

> ![](../assets/inline/crates-and-source-files-2.svg)Note
>
> Although Rust, like any other language, can be implemented by an interpreter as well as a compiler, the only existing implementation is a compiler, and the language has always been designed to be compiled. For these reasons, this section assumes a compiler.

[crate.compile-time](#r-crate.compile-time)

Rust’s semantics obey a *phase distinction* between compile-time and run-time.[1](#footnote-phase-distinction) Semantic rules that have a *static interpretation* govern the success or failure of compilation, while semantic rules that have a *dynamic interpretation* govern the behavior of the program at run-time.

[crate.unit](#r-crate.unit)

The compilation model centers on artifacts called *crates*. Each compilation processes a single crate in source form, and if successful, produces a single crate in binary form: either an executable or some sort of library.[2](#footnote-cratesourcefile)

[crate.module](#r-crate.module)

A *crate* is a unit of compilation and linking, as well as versioning, distribution, and runtime loading. A crate contains a *tree* of nested [module](items/modules.md) scopes. The top level of this tree is a module that is anonymous (from the point of view of paths within the module) and any item within a crate has a canonical [module path](paths.md) denoting its location within the crate’s module tree.

[crate.input-source](#r-crate.input-source)

The Rust compiler is always invoked with a single source file as input, and always produces a single output crate. The processing of that source file may result in other source files being loaded as modules. Source files have the extension `.rs`.

[crate.module-def](#r-crate.module-def)

A Rust source file describes a module, the name and location of which — in the module tree of the current crate — are defined from outside the source file: either by an explicit [Module](items/modules.md#grammar-Module) item in a referencing source file, or by the name of the crate itself.

[crate.inline-module](#r-crate.inline-module)

Every source file is a module, but not every module needs its own source file: [module definitions](items/modules.md) can be nested within one file.

[crate.items](#r-crate.items)

Each source file contains a sequence of zero or more [Item](items.md#grammar-Item) definitions, and may optionally begin with any number of [attributes](attributes.md) that apply to the containing module, most of which influence the behavior of the compiler.

[crate.attributes](#r-crate.attributes)

The anonymous crate module can have additional attributes that apply to the crate as a whole.

> ![](../assets/inline/crates-and-source-files-3.svg)Note
>
> The file’s contents may be preceded by a [shebang](shebang.md).

``` playground
#![allow(unused)]
fn main() {
// Specify the crate name.
#![crate_name = "projx"]

// Specify the type of output artifact.
#![crate_type = "lib"]

// Turn on a warning.
// This can be done in any module, not just the anonymous crate module.
#![warn(non_camel_case_types)]
}
```

[crate.main](#r-crate.main)

## Main functions

[crate.main.executable](#r-crate.main.executable)

A crate that contains a `main` [function](items/functions.md) can be compiled to an executable.

[crate.main.restriction](#r-crate.main.restriction)

If a `main` function is present, it must take no arguments, must not declare any [trait or lifetime bounds](trait-bounds.md), must not have any [where clauses](items/generics.md#where-clauses), and its return type must implement the [`Termination`](https://doc.rust-lang.org/1.100.0/std/process/trait.Termination.html) trait.

``` playground
fn main() {}
```

``` playground
fn main() -> ! {
    std::process::exit(0);
}
```

``` playground
fn main() -> impl std::process::Termination {
    std::process::ExitCode::SUCCESS
}
```

[crate.main.import](#r-crate.main.import)

The `main` function may be an import, e.g. from an external crate or from the current one.

``` playground
#![allow(unused)]
fn main() {
mod foo {
    pub fn bar() {
        println!("Hello, world!");
    }
}
use foo::bar as main;
}
```

> ![](../assets/inline/crates-and-source-files-4.svg)Note
>
> Types with implementations of [`Termination`](https://doc.rust-lang.org/1.100.0/std/process/trait.Termination.html) in the standard library include:
>
> - `()`
> - [`!`](types/never.md)
> - [`Infallible`](https://doc.rust-lang.org/1.100.0/core/convert/type.Infallible.html)
> - [`ExitCode`](https://doc.rust-lang.org/1.100.0/std/process/struct.ExitCode.html)
> - `Result<T, E> where T: Termination, E: Debug`

[crate.uncaught-foreign-unwinding](#r-crate.uncaught-foreign-unwinding)

### Uncaught foreign unwinding

When a “foreign” unwind (e.g. an exception thrown from C++ code, or a `panic!` in Rust code using a different panic handler) propagates beyond the `main` function, the process will be safely terminated. This may take the form of an abort, in which case it is not guaranteed that any `Drop` calls will be executed, and the error output may be less informative than if the runtime had been terminated by a “native” Rust `panic`.

For more information, see the [panic documentation](panic.md#unwinding-across-ffi-boundaries).

[crate.no_main](#r-crate.no_main)

### The `no_main` attribute

The *`no_main` [attribute](attributes.md)* may be applied at the crate level to disable emitting the `main` symbol for an executable binary. This is useful when some other object being linked to defines `main`.

[crate.crate_name](#r-crate.crate_name)

## The `crate_name` attribute

[crate.crate_name.general](#r-crate.crate_name.general)

The *`crate_name` [attribute](attributes.md)* may be applied at the crate level to specify the name of the crate with the [MetaNameValueStr](attributes.md#grammar-MetaNameValueStr) syntax.

``` playground
#![allow(unused)]
#![crate_name = "mycrate"]
fn main() {
}
```

[crate.crate_name.restriction](#r-crate.crate_name.restriction)

The crate name must not be empty, and must only contain [Unicode alphanumeric](https://doc.rust-lang.org/1.100.0/std/primitive.char.html#method.is_alphanumeric) or `_` (U+005F) characters.

------------------------------------------------------------------------

1.  

    This distinction would also exist in an interpreter. Static checks like syntactic analysis, type checking, and lints should happen before the program is executed regardless of when it is executed. [↩](#fr-phase-distinction-1)

    

2.  

    A crate is somewhat analogous to an *assembly* in the ECMA-335 CLI model, a *library* in the SML/NJ Compilation Manager, a *unit* in the Owens and Flatt module system, or a *configuration* in Mesa. [↩](#fr-cratesourcefile-1)
