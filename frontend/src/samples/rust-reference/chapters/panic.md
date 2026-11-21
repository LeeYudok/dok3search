# Panic

[panic](#r-panic)

# Panic

[panic.intro](#r-panic.intro)

Rust provides a mechanism to prevent a function from returning normally, and instead “panic,” which is a response to an error condition that is typically not expected to be recoverable within the context in which the error is encountered.

[panic.lang-ops](#r-panic.lang-ops)

Some language constructs, such as out-of-bounds [array indexing](expressions/array-expr.md#array-and-slice-indexing-expressions), panic automatically.

[panic.control](#r-panic.control)

There are also language features that provide a level of control over panic behavior:

- A [*panic handler*](#the-panic_handler-attribute) defines the behavior of a panic.
- [FFI ABIs](items/functions.md#unwinding) may alter how panics behave.

> ![](../assets/inline/panic-1.svg)Note
>
> The standard library provides the capability to explicitly panic via the [`panic!` macro](https://doc.rust-lang.org/1.100.0/std/macro.panic.html).

[panic.panic_handler](#r-panic.panic_handler)

## The `panic_handler` attribute

[panic.panic_handler.intro](#r-panic.panic_handler.intro)

The *`panic_handler` attribute* can be applied to a function to define the behavior of panics.

[panic.panic_handler.allowed-positions](#r-panic.panic_handler.allowed-positions)

The `panic_handler` attribute can only be applied to a function with signature `fn(&PanicInfo) -> !`.

> ![](../assets/inline/panic-2.svg)Note
>
> The [`PanicInfo`](https://doc.rust-lang.org/1.100.0/core/panic/panic_info/struct.PanicInfo.html) struct contains information about the location of the panic.

[panic.panic_handler.unique](#r-panic.panic_handler.unique)

There must be a single `panic_handler` function in the dependency graph.

Below is shown a `panic_handler` function that logs the panic message and then halts the thread.

``` rust
#![no_std]

use core::fmt::{self, Write};
use core::panic::PanicInfo;

struct Sink {
    // ..
   _0: (),
}

impl Sink {
    fn new() -> Sink { Sink { _0: () }}
}

impl fmt::Write for Sink {
    fn write_str(&mut self, _: &str) -> fmt::Result { Ok(()) }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut sink = Sink::new();

    // logs "panicked at '$reason', src/main.rs:27:4" to some `sink`
    let _ = writeln!(sink, "{}", info);

    loop {}
}
```

[panic.panic_handler.std](#r-panic.panic_handler.std)

### Standard behavior

[panic.panic_handler.std.kinds](#r-panic.panic_handler.std.kinds)

`std` provides two different panic handlers:

- `unwind` — unwinds the stack and is potentially recoverable.
- `abort` –– aborts the process and is non-recoverable.

Not all targets may provide the `unwind` handler.

> ![](../assets/inline/panic-3.svg)Note
>
> The panic handler used when linking with `std` can be set with the [`-C panic`](https://doc.rust-lang.org/1.100.0/rustc/codegen-options/index.html#panic) CLI flag. The default for most targets is `unwind`.
>
> The standard library’s panic behavior can be modified at runtime with the [`std::panic::set_hook`](https://doc.rust-lang.org/1.100.0/std/panicking/fn.set_hook.html) function.

[panic.panic_handler.std.no_std](#r-panic.panic_handler.std.no_std)

Linking a [`no_std`](names/preludes.md#the-no_std-attribute) binary, dylib, cdylib, or staticlib will require specifying your own panic handler.

[panic.strategy](#r-panic.strategy)

## Panic strategy

[panic.strategy.intro](#r-panic.strategy.intro)

The *panic strategy* defines the kind of panic behavior that a crate is built to support.

> ![](../assets/inline/panic-4.svg)Note
>
> The panic strategy can be chosen in `rustc` with the [`-C panic`](https://doc.rust-lang.org/1.100.0/rustc/codegen-options/index.html#panic) CLI flag.
>
> When generating a binary, dylib, cdylib, or staticlib and linking with `std`, the `-C panic` CLI flag also influences which [panic handler](#the-panic_handler-attribute) is used.

> ![](../assets/inline/panic-5.svg)Note
>
> When compiling code with the `abort` panic strategy, the optimizer may assume that unwinding across Rust frames is impossible, which can result in both code-size and runtime speed improvements.

> ![](../assets/inline/panic-6.svg)Note
>
> See [link.unwinding](linkage.md#r-link.unwinding) for restrictions on linking crates with different panic strategies. An implication is that crates built with the `unwind` strategy can use the `abort` panic handler, but the `abort` strategy cannot use the `unwind` panic handler.

[panic.unwind](#r-panic.unwind)

## Unwinding

[panic.unwind.intro](#r-panic.unwind.intro)

Panicking may either be recoverable or non-recoverable, though it can be configured (by choosing a non-unwinding panic handler) to always be non-recoverable. (The converse is not true: the `unwind` handler does not guarantee that all panics are recoverable, only that panicking via the `panic!` macro and similar standard library mechanisms is recoverable.)

[panic.unwind.destruction](#r-panic.unwind.destruction)

When a panic occurs, the `unwind` handler “unwinds” Rust frames, just as C++’s `throw` unwinds C++ frames, until the panic reaches the point of recovery (for instance at a thread boundary). This means that as the panic traverses Rust frames, live objects in those frames that [implement `Drop`](destructors.md) will have their `drop` methods called. Thus, when normal execution resumes, no-longer-accessible objects will have been “cleaned up” just as if they had gone out of scope normally.

> ![](../assets/inline/panic-7.svg)Note
>
> As long as this guarantee of resource-cleanup is preserved, “unwinding” may be implemented without actually using the mechanism used by C++ for the target platform.

> ![](../assets/inline/panic-8.svg)Note
>
> The standard library provides two mechanisms for recovering from a panic, [`std::panic::catch_unwind`](https://doc.rust-lang.org/1.100.0/std/panic/fn.catch_unwind.html) (which enables recovery within the panicking thread) and [`std::thread::spawn`](https://doc.rust-lang.org/1.100.0/std/thread/functions/fn.spawn.html) (which automatically sets up panic recovery for the spawned thread so that other threads may continue running).

[panic.unwind.ffi](#r-panic.unwind.ffi)

### Unwinding across FFI boundaries

[panic.unwind.ffi.intro](#r-panic.unwind.ffi.intro)

It is possible to unwind across FFI boundaries using an [appropriate ABI declaration](items/functions.md#unwinding). While useful in certain cases, this creates unique opportunities for undefined behavior, especially when multiple language runtimes are involved.

[panic.unwind.ffi.undefined](#r-panic.unwind.ffi.undefined)

Unwinding with the wrong ABI is undefined behavior:

- Causing an unwind into Rust code from a foreign function that was called via a function declaration or pointer declared with a non-unwinding ABI, such as `"C"`, `"system"`, etc. (For example, this case occurs when such a function written in C++ throws an exception that is uncaught and propagates to Rust.)
- Calling a Rust `extern` function that unwinds (with `extern "C-unwind"` or another ABI that permits unwinding) from code that does not support unwinding, such as code compiled with GCC or Clang using `-fno-exceptions`

[panic.unwind.ffi.catch-foreign](#r-panic.unwind.ffi.catch-foreign)

Catching a foreign unwinding operation (such as a C++ exception) using [`std::panic::catch_unwind`](https://doc.rust-lang.org/1.100.0/std/panic/fn.catch_unwind.html), [`std::thread::JoinHandle::join`](https://doc.rust-lang.org/1.100.0/std/thread/join_handle/struct.JoinHandle.html#method.join), or by letting it propagate beyond the Rust `main()` function or thread root will have one of two behaviors, and it is unspecified which will occur:

- The process aborts.
- The function returns a [`Result::Err`](https://doc.rust-lang.org/1.100.0/core/result/enum.Result.html#variant.Err) containing an opaque type.

> ![](../assets/inline/panic-9.svg)Note
>
> Rust code compiled or linked with a different instance of the Rust standard library counts as a “foreign exception” for the purpose of this guarantee. Thus, a library that uses `panic!` and is linked against one version of the Rust standard library, invoked from an application that uses a different version of the standard library, may cause the entire application to abort even if the library is only used within a child thread.

[panic.unwind.ffi.dispose-panic](#r-panic.unwind.ffi.dispose-panic)

There are currently no guarantees about the behavior that occurs when a foreign runtime attempts to dispose of, or rethrow, a Rust `panic` payload. In other words, an unwind originated from a Rust runtime must either lead to termination of the process or be caught by the same runtime.
