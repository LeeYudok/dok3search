# Debugger attributes

[attributes.debugger](#r-attributes.debugger)

# Debugger attributes

[attributes.debugger.intro](#r-attributes.debugger.intro)

The following [attributes](../attributes.md) are used for enhancing the debugging experience when using third-party debuggers like GDB or WinDbg.

[attributes.debugger.debugger_visualizer](#r-attributes.debugger.debugger_visualizer)

## The `debugger_visualizer` attribute

[attributes.debugger.debugger_visualizer.intro](#r-attributes.debugger.debugger_visualizer.intro)

The *`debugger_visualizer` [attribute](../attributes.md)* can be used to embed a debugger visualizer file into the debug information. This improves the debugger experience when displaying values.

> ![](../../assets/inline/debugger-1.svg)Example
>
> ``` rust
> #![debugger_visualizer(natvis_file = "Example.natvis")]
> #![debugger_visualizer(gdb_script_file = "example.py")]
> ```

[attributes.debugger.debugger_visualizer.syntax](#r-attributes.debugger.debugger_visualizer.syntax)

The `debugger_visualizer` attribute uses the [MetaListNameValueStr](../attributes.md#grammar-MetaListNameValueStr) syntax to specify its inputs. One of the following keys must be specified:

- [`natvis_file`](#r-attributes.debugger.debugger_visualizer.natvis)
- [`gdb_script_file`](#r-attributes.debugger.debugger_visualizer.gdb)

[attributes.debugger.debugger_visualizer.allowed-positions](#r-attributes.debugger.debugger_visualizer.allowed-positions)

The `debugger_visualizer` attribute may only be applied to a [module](../items/modules.md) or to the crate root.

[attributes.debugger.debugger_visualizer.duplicates](#r-attributes.debugger.debugger_visualizer.duplicates)

The `debugger_visualizer` attribute may be used any number of times on a form. All specified visualizer files will be loaded.

[attributes.debugger.debugger_visualizer.natvis](#r-attributes.debugger.debugger_visualizer.natvis)

### Using `debugger_visualizer` with Natvis

[attributes.debugger.debugger_visualizer.natvis.intro](#r-attributes.debugger.debugger_visualizer.natvis.intro)

Natvis is an XML-based framework for Microsoft debuggers (such as Visual Studio and WinDbg) that uses declarative rules to customize the display of types. For detailed information on the Natvis format, refer to Microsoft’s [Natvis documentation](https://docs.microsoft.com/en-us/visualstudio/debugger/create-custom-views-of-native-objects).

[attributes.debugger.debugger_visualizer.natvis.msvc](#r-attributes.debugger.debugger_visualizer.natvis.msvc)

This attribute only supports embedding Natvis files on `-windows-msvc` targets.

[attributes.debugger.debugger_visualizer.natvis.path](#r-attributes.debugger.debugger_visualizer.natvis.path)

The path to the Natvis file is specified with the `natvis_file` key, which is a path relative to the source file.

> ![](../../assets/inline/debugger-2.svg)Example
>
> ``` rust
> #![debugger_visualizer(natvis_file = "Rectangle.natvis")]
>
> struct FancyRect {
>     x: f32,
>     y: f32,
>     dx: f32,
>     dy: f32,
> }
>
> fn main() {
>     let fancy_rect = FancyRect { x: 10.0, y: 10.0, dx: 5.0, dy: 5.0 };
>     println!("set breakpoint here");
> }
> ```
>
> `Rectangle.natvis` contains:
>
> ``` xml
> <?xml version="1.0" encoding="utf-8"?>
> 
>     
>       ({x},{y}) + ({dx}, {dy})
>       
>         
>           ({x}, {y})
>         
>         
>           ({x}, {y + dy})
>         
>         
>           ({x + dx}, {y + dy})
>         
>         
>           ({x + dx}, {y})
>         
>       
>     
> 
> ```
>
> When viewed under WinDbg, the `fancy_rect` variable would be shown as follows:
>
> ``` text
> > Variables:
>   > fancy_rect: (10.0, 10.0) + (5.0, 5.0)
>     > LowerLeft: (10.0, 10.0)
>     > UpperLeft: (10.0, 15.0)
>     > UpperRight: (15.0, 15.0)
>     > LowerRight: (15.0, 10.0)
> ```

[attributes.debugger.debugger_visualizer.gdb](#r-attributes.debugger.debugger_visualizer.gdb)

### Using `debugger_visualizer` with GDB

[attributes.debugger.debugger_visualizer.gdb.pretty](#r-attributes.debugger.debugger_visualizer.gdb.pretty)

GDB supports the use of a structured Python script, called a *pretty printer*, that describes how a type should be visualized in the debugger view. For detailed information on pretty printers, refer to GDB’s [pretty printing documentation](https://sourceware.org/gdb/onlinedocs/gdb/Pretty-Printing.html).

> ![](../../assets/inline/debugger-3.svg)Note
>
> Embedded pretty printers are not automatically loaded when debugging a binary under GDB.
>
> There are two ways to enable auto-loading embedded pretty printers:
>
> 1.  Launch GDB with extra arguments to explicitly add a directory or binary to the auto-load safe path: `gdb -iex "add-auto-load-safe-path safe-path path/to/binary" path/to/binary` For more information, see GDB’s [auto-loading documentation](https://sourceware.org/gdb/onlinedocs/gdb/Auto_002dloading-safe-path.html).
> 2.  Create a file named `gdbinit` under `$HOME/.config/gdb` (you may need to create the directory if it doesn’t already exist). Add the following line to that file: `add-auto-load-safe-path path/to/binary`.

[attributes.debugger.debugger_visualizer.gdb.path](#r-attributes.debugger.debugger_visualizer.gdb.path)

These scripts are embedded using the `gdb_script_file` key, which is a path relative to the source file.

> ![](../../assets/inline/debugger-4.svg)Example
>
> ``` rust
> #![debugger_visualizer(gdb_script_file = "printer.py")]
>
> struct Person {
>     name: String,
>     age: i32,
> }
>
> fn main() {
>     let bob = Person { name: String::from("Bob"), age: 10 };
>     println!("set breakpoint here");
> }
> ```
>
> `printer.py` contains:
>
> ``` python
> import gdb
>
> class PersonPrinter:
>     "Print a Person"
>
>     def __init__(self, val):
>         self.val = val
>         self.name = val["name"]
>         self.age = int(val["age"])
>
>     def to_string(self):
>         return "{} is {} years old.".format(self.name, self.age)
>
> def lookup(val):
>     lookup_tag = val.type.tag
>     if lookup_tag is None:
>         return None
>     if "foo::Person" == lookup_tag:
>         return PersonPrinter(val)
>
>     return None
>
> gdb.current_objfile().pretty_printers.append(lookup)
> ```
>
> When the crate’s debug executable is passed into GDB[1](#footnote-rust-gdb), `print bob` will display:
>
> ``` text
> "Bob" is 10 years old.
> ```

[attributes.debugger.collapse_debuginfo](#r-attributes.debugger.collapse_debuginfo)

## The `collapse_debuginfo` attribute

[attributes.debugger.collapse_debuginfo.intro](#r-attributes.debugger.collapse_debuginfo.intro)

The *`collapse_debuginfo` [attribute](../attributes.md)* controls whether code locations from a macro definition are collapsed into a single location associated with the macro’s call site when generating debuginfo for code calling this macro.

> ![](../../assets/inline/debugger-5.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> #[collapse_debuginfo(yes)]
> macro_rules! example {
>     () => {
>         println!("hello!");
>     };
> }
> }
> ```
>
> When using a debugger, invoking the `example` macro may appear as though it is calling a function. That is, when you step to the invocation site, it may show the macro invocation rather than the expanded code.

[attributes.debugger.collapse_debuginfo.syntax](#r-attributes.debugger.collapse_debuginfo.syntax)

The syntax for the `collapse_debuginfo` attribute is:

**Syntax**\
[CollapseDebuginfoAttribute](#railroad-CollapseDebuginfoAttribute) → collapse_debuginfo ( [CollapseDebuginfoOption](#grammar-CollapseDebuginfoOption) )

[CollapseDebuginfoOption](#railroad-CollapseDebuginfoOption) →\
      yes\
    \| no\
    \| external

Show Railroad

![](../../assets/inline/debugger-6.svg) ![](../../assets/inline/debugger-7.svg) [attributes.debugger.collapse_debuginfo.allowed-positions](#r-attributes.debugger.collapse_debuginfo.allowed-positions)

The `collapse_debuginfo` attribute may only be applied to a [`macro_rules` definition](../macros-by-example.md).

[attributes.debugger.collapse_debuginfo.duplicates](#r-attributes.debugger.collapse_debuginfo.duplicates)

The `collapse_debuginfo` attribute may used only once on a macro.

[attributes.debugger.collapse_debuginfo.options](#r-attributes.debugger.collapse_debuginfo.options)

The `collapse_debuginfo` attribute accepts these options:

- `#[collapse_debuginfo(yes)]` — Code locations in debuginfo are collapsed.
- `#[collapse_debuginfo(no)]` — Code locations in debuginfo are not collapsed.
- `#[collapse_debuginfo(external)]` — Code locations in debuginfo are collapsed only if the macro comes from a different crate.

[attributes.debugger.collapse_debuginfo.default](#r-attributes.debugger.collapse_debuginfo.default)

The `external` behavior is the default for macros that don’t have this attribute unless they are built-in macros. For built-in macros the default is `yes`.

> ![](../../assets/inline/debugger-8.svg)Note
>
> `rustc` has a [`-C collapse-macro-debuginfo`](../../rustc/codegen-options/index.html#collapse-macro-debuginfo) CLI option to override both the default behavior and the values of any `#[collapse_debuginfo]` attributes.

------------------------------------------------------------------------

1.  

    Note: This assumes you are using the `rust-gdb` script which configures pretty-printers for standard library types like `String`. [↩](#fr-rust-gdb-1)
