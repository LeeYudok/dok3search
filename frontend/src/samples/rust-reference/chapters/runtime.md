# The Rust runtime

[runtime](#r-runtime)

# The Rust runtime

[runtime.intro](#r-runtime.intro)

This section documents features that define some aspects of the Rust runtime.

[runtime.global_allocator](#r-runtime.global_allocator)

## The `global_allocator` attribute

[runtime.global_allocator.intro](#r-runtime.global_allocator.intro)

The *`global_allocator` [attribute](attributes.md#r-attributes)* selects a [memory allocator](https://doc.rust-lang.org/1.100.0/std/alloc/index.html).

> ![](../assets/inline/runtime-1.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> use core::alloc::{GlobalAlloc, Layout};
> use std::alloc::System;
>
> struct MyAllocator;
>
> unsafe impl GlobalAlloc for MyAllocator {
>     unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
>         unsafe { System.alloc(layout) }
>     }
>     unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
>         unsafe { System.dealloc(ptr, layout) }
>     }
> }
>
> #[global_allocator]
> static GLOBAL: MyAllocator = MyAllocator;
> }
> ```

[runtime.global_allocator.syntax](#r-runtime.global_allocator.syntax)

The `global_allocator` attribute uses the [MetaWord](attributes.md#grammar-MetaWord) syntax.

[runtime.global_allocator.allowed-positions](#r-runtime.global_allocator.allowed-positions)

The `global_allocator` attribute may only be applied to a [static item](items/static-items.md) whose type implements the [`GlobalAlloc`](https://doc.rust-lang.org/1.100.0/core/alloc/global/trait.GlobalAlloc.html) trait.

[runtime.global_allocator.duplicates](#r-runtime.global_allocator.duplicates)

The `global_allocator` attribute may only be used once on an item.

[runtime.global_allocator.single](#r-runtime.global_allocator.single)

The `global_allocator` attribute may only be used once in the crate graph.

[runtime.global_allocator.stdlib](#r-runtime.global_allocator.stdlib)

The `global_allocator` attribute is exported from the [standard library prelude](https://doc.rust-lang.org/1.100.0/core/prelude/v1/index.html).

[runtime.windows_subsystem](#r-runtime.windows_subsystem)

## The `windows_subsystem` attribute

[runtime.windows_subsystem.intro](#r-runtime.windows_subsystem.intro)

The *`windows_subsystem` [attribute](attributes.md#r-attributes)* sets the [subsystem](https://msdn.microsoft.com/en-us/library/fcc1zstk.aspx) when linking on a Windows target.

> ![](../assets/inline/runtime-2.svg)Example
>
> ``` playground
> #![allow(unused)]
> #![windows_subsystem = "windows"]
> fn main() {
> }
> ```

[runtime.windows_subsystem.syntax](#r-runtime.windows_subsystem.syntax)

The `windows_subsystem` attribute uses the [MetaNameValueStr](attributes.md#grammar-MetaNameValueStr) syntax. Accepted values are `"console"` and `"windows"`.

[runtime.windows_subsystem.allowed-positions](#r-runtime.windows_subsystem.allowed-positions)

The `windows_subsystem` attribute may only be applied to the crate root.

[runtime.windows_subsystem.duplicates](#r-runtime.windows_subsystem.duplicates)

Only the first use of `windows_subsystem` has effect.

> ![](../assets/inline/runtime-3.svg)Note
>
> `rustc` lints against any use following the first. This may become an error in the future.

[runtime.windows_subsystem.ignored](#r-runtime.windows_subsystem.ignored)

The `windows_subsystem` attribute is ignored on non-Windows targets and non-`bin` [crate types](linkage.md).

[runtime.windows_subsystem.console](#r-runtime.windows_subsystem.console)

The `"console"` subsystem is the default. If a console process is run from an existing console then it will be attached to that console; otherwise a new console window will be created.

[runtime.windows_subsystem.windows](#r-runtime.windows_subsystem.windows)

The `"windows"` subsystem will run detached from any existing console.

> ![](../assets/inline/runtime-4.svg)Note
>
> The `"windows"` subsystem is commonly used by GUI applications that do not want to display a console window on startup.
