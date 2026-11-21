# Code generation attributes

[attributes.codegen](#r-attributes.codegen)

# Code generation attributes

[attributes.codegen.intro](#r-attributes.codegen.intro)

The following [attributes](../attributes.md) are used for controlling code generation.

[attributes.codegen.inline](#r-attributes.codegen.inline)

### The `inline` attribute

[attributes.codegen.inline.intro](#r-attributes.codegen.inline.intro)

The *`inline` [attribute](../attributes.md)* suggests whether a copy of the attributed function’s code should be placed in the caller rather than generating a call to the function.

> ![](../../assets/inline/codegen-1.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> #[inline]
> pub fn example1() {}
>
> #[inline(always)]
> pub fn example2() {}
>
> #[inline(never)]
> pub fn example3() {}
> }
> ```

> ![](../../assets/inline/codegen-2.svg)Note
>
> `rustc` automatically inlines functions when doing so seems worthwhile. Use this attribute carefully as poor decisions about what to inline can slow down programs.

[attributes.codegen.inline.syntax](#r-attributes.codegen.inline.syntax)

The syntax for the `inline` attribute is:

**Syntax**\
[InlineAttribute](#railroad-InlineAttribute) →\
      inline ( always )\
    \| inline ( never )\
    \| inline

Show Railroad

![](../../assets/inline/codegen-3.svg) [attributes.codegen.inline.allowed-positions](#r-attributes.codegen.inline.allowed-positions)

The `inline` attribute may only be applied to functions with [bodies](../items/functions.md#r-items.fn.body) — [closures](../expressions/closure-expr.md#r-expr.closure), [async blocks](../expressions/block-expr.md#r-expr.block.async), [free functions](../items/functions.md#r-items.fn), [associated functions](../items/associated-items.md#r-items.associated.fn) in an [inherent impl](../items/implementations.md#r-items.impl.inherent) or [trait impl](../items/implementations.md#r-items.impl.trait), and associated functions in a [trait definition](../items/traits.md#r-items.traits) when those functions have a [default definition](../items/traits.md#r-items.traits.associated-item-decls) .

> ![](../../assets/inline/codegen-4.svg)Note
>
> `rustc` ignores use in other positions but lints against it. This may become an error in the future.

> ![](../../assets/inline/codegen-5.svg)Note
>
> Though the attribute can be applied to [closures](../expressions/closure-expr.md#r-expr.closure) and [async blocks](../expressions/block-expr.md#r-expr.block.async), the usefulness of this is limited as we do not yet support attributes on expressions.
>
> ``` playground
> #![allow(unused)]
> fn main() {
> // We allow attributes on statements.
> #[inline] || (); // OK
> #[inline] async {}; // OK
> }
> ```
>
> ``` playground
> #![allow(unused)]
> fn main() {
> // We don't yet allow attributes on expressions.
> let f = #[inline] || (); // ERROR
> }
> ```

[attributes.codegen.inline.duplicates](#r-attributes.codegen.inline.duplicates)

Only the first use of `inline` on a function has effect.

> ![](../../assets/inline/codegen-6.svg)Note
>
> `rustc` lints against any use following the first. This may become an error in the future.

[attributes.codegen.inline.modes](#r-attributes.codegen.inline.modes)

The `inline` attribute supports these modes:

- `#[inline]` *suggests* performing inline expansion.
- `#[inline(always)]` *suggests* that inline expansion should always be performed.
- `#[inline(never)]` *suggests* that inline expansion should never be performed.

> ![](../../assets/inline/codegen-7.svg)Note
>
> In every form the attribute is a hint. The compiler may ignore it.

[attributes.codegen.inline.trait](#r-attributes.codegen.inline.trait)

When `inline` is applied to a function in a [trait](../items/traits.md#r-items.traits), it applies only to the code of the [default definition](../items/traits.md#r-items.traits.associated-item-decls).

[attributes.codegen.inline.async](#r-attributes.codegen.inline.async)

When `inline` is applied to an [async function](../items/functions.md#r-items.fn.async) or [async closure](../expressions/closure-expr.md#r-expr.closure.async), it applies only to the code of the generated `poll` function.

> ![](../../assets/inline/codegen-8.svg)Note
>
> For more details, see [Rust issue \#129347](https://github.com/rust-lang/rust/issues/129347).

[attributes.codegen.inline.externally-exported](#r-attributes.codegen.inline.externally-exported)

The `inline` attribute is ignored if the function is externally exported with [`no_mangle`](../abi.md#r-abi.no_mangle) or [`export_name`](../abi.md#r-abi.export_name).

[attributes.codegen.cold](#r-attributes.codegen.cold)

### The `cold` attribute

[attributes.codegen.cold.intro](#r-attributes.codegen.cold.intro)\
Tests Tests with this rule:

- [tests/codegen-llvm/cold-attribute.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/codegen-llvm/cold-attribute.rs)

The *`cold` [attribute](../attributes.md)* suggests that the attributed function is unlikely to be called which may help the compiler produce better code.

> ![](../../assets/inline/codegen-9.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> #[cold]
> pub fn example() {}
> }
> ```

[attributes.codegen.cold.syntax](#r-attributes.codegen.cold.syntax)

The `cold` attribute uses the [MetaWord](../attributes.md#grammar-MetaWord) syntax.

[attributes.codegen.cold.allowed-positions](#r-attributes.codegen.cold.allowed-positions)

The `cold` attribute may only be applied to functions with [bodies](../items/functions.md#r-items.fn.body) — [closures](../expressions/closure-expr.md#r-expr.closure), [async blocks](../expressions/block-expr.md#r-expr.block.async), [free functions](../items/functions.md#r-items.fn), [associated functions](../items/associated-items.md#r-items.associated.fn) in an [inherent impl](../items/implementations.md#r-items.impl.inherent) or [trait impl](../items/implementations.md#r-items.impl.trait), and associated functions in a [trait definition](../items/traits.md#r-items.traits) when those functions have a [default definition](../items/traits.md#r-items.traits.associated-item-decls) .

> ![](../../assets/inline/codegen-10.svg)Note
>
> `rustc` ignores use in other positions but lints against it. This may become an error in the future.

> ![](../../assets/inline/codegen-11.svg)Note
>
> Though the attribute can be applied to [closures](../expressions/closure-expr.md#r-expr.closure) and [async blocks](../expressions/block-expr.md#r-expr.block.async), the usefulness of this is limited as we do not yet support attributes on expressions.

[attributes.codegen.cold.extern-custom](#r-attributes.codegen.cold.extern-custom)

The `cold` attribute may not be applied to an [`extern "custom"` function](../items/functions.md#r-items.fn.extern.custom).

``` playground
#![allow(unused)]
fn main() {
#[cold] // ERROR: Not allowed.
#[unsafe(naked)]
unsafe extern "custom" fn f() {
    core::arch::naked_asm!("ret")
}
}
```

[attributes.codegen.cold.duplicates](#r-attributes.codegen.cold.duplicates)

Only the first use of `cold` on a function has effect.

> ![](../../assets/inline/codegen-12.svg)Note
>
> `rustc` lints against any use following the first. This may become an error in the future.

[attributes.codegen.cold.trait](#r-attributes.codegen.cold.trait)\
Tests Tests with this rule:

- [tests/codegen-llvm/cold-attribute.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/codegen-llvm/cold-attribute.rs)

When `cold` is applied to a function in a [trait](../items/traits.md#r-items.traits), it applies only to the code of the [default definition](../items/traits.md#r-items.traits.associated-item-decls).

[attributes.codegen.naked](#r-attributes.codegen.naked)

## The `naked` attribute

[attributes.codegen.naked.intro](#r-attributes.codegen.naked.intro)

The *`naked` [attribute](../attributes.md)* prevents the compiler from emitting a function prologue and epilogue for the attributed function — a *naked function*.

> ![](../../assets/inline/codegen-13.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> #[cfg(target_arch = "x86_64")] {
> /// Adds 3 to the given number.
> // SAFETY: The body respects the "sysv64" calling convention,
> // upholds the signature, and does not fall through.
> #[unsafe(naked)]
> pub extern "sysv64" fn add_n(number: u64) -> u64 {
>     core::arch::naked_asm!(
>         "add rdi, {}",
>         "mov rax, rdi",
>         "ret",
>         const 3,
>     )
> }
> }
> }
> ```

> ![](../../assets/inline/codegen-14.svg)Note
>
> The assembly code of a naked function often does not follow the calling convention of any ABI known to the compiler. Such a function should be declared as an [`extern "custom"` function](../items/functions.md#r-items.fn.extern.custom).

[attributes.codegen.naked.syntax](#r-attributes.codegen.naked.syntax)

The `naked` attribute uses the [MetaWord](../attributes.md#grammar-MetaWord) syntax.

[attributes.codegen.naked.allowed-positions](#r-attributes.codegen.naked.allowed-positions)

The `naked` attribute may only be applied to [free functions](../items/functions.md#r-items.fn), [associated functions](../items/associated-items.md#r-items.associated.fn) in an [inherent impl](../items/implementations.md#r-items.impl.inherent) or [trait impl](../items/implementations.md#r-items.impl.trait), and associated functions in a [trait definition](../items/traits.md#r-items.traits) when those functions have a [default definition](../items/traits.md#r-items.traits.associated-item-decls).

[attributes.codegen.naked.duplicates](#r-attributes.codegen.naked.duplicates)

Only the first use of `naked` on a function has effect.

> ![](../../assets/inline/codegen-15.svg)Note
>
> `rustc` lints against any use following the first.

[attributes.codegen.naked.unsafe](#r-attributes.codegen.naked.unsafe)

The `naked` attribute must be marked with [`unsafe`](../attributes.md#r-attributes.safety) because the body must respect the function’s calling convention, uphold its signature, and either return or diverge (i.e., not fall through past the end of the assembly code).

[attributes.codegen.naked.body](#r-attributes.codegen.naked.body)\
Tests Tests with this rule:

- [tests/ui/asm/naked-functions/naked-functions.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/asm/naked-functions/naked-functions.rs)

The [function body](../items/functions.md#r-items.fn.body) must consist of exactly one [`naked_asm!`](../inline-assembly.md#r-asm) macro invocation.

[attributes.codegen.naked.prologue-epilogue](#r-attributes.codegen.naked.prologue-epilogue)

The compiler emits no prologue or epilogue for a naked function: the assembly code in the [`naked_asm!`](../inline-assembly.md#r-asm) invocation constitutes its entire body.

[attributes.codegen.naked.call-stack](#r-attributes.codegen.naked.call-stack)

On entry the assembly code may assume that the call stack and register state are valid per the function’s signature and calling convention.

[attributes.codegen.naked.no-duplication](#r-attributes.codegen.naked.no-duplication)

The compiler may not duplicate the assembly code except when monomorphizing a polymorphic function.

> ![](../../assets/inline/codegen-16.svg)Note
>
> This guarantee matters for naked functions that define symbols.

[attributes.codegen.naked.unused-variables](#r-attributes.codegen.naked.unused-variables)

The [`unused_variables` lint](../../rustc/lints/listing/warn-by-default.html#unused-variables) is suppressed in naked functions.

[attributes.codegen.naked.inline](#r-attributes.codegen.naked.inline)\
Tests Tests with this rule:

- [tests/ui/asm/naked-functions/inline.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/asm/naked-functions/inline.rs)

The [`inline` attribute](#r-attributes.codegen.inline) cannot be applied to a naked function.

[attributes.codegen.naked.track_caller](#r-attributes.codegen.naked.track_caller)

The [`track_caller` attribute](#r-attributes.codegen.track_caller) cannot be applied to a naked function.

[attributes.codegen.naked.testing](#r-attributes.codegen.naked.testing)\
Tests Tests with this rule:

- [tests/ui/asm/naked-functions/testattrs.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/asm/naked-functions/testattrs.rs)

The [testing attributes](testing.md#r-attributes.testing) cannot be applied to a naked function.

[attributes.codegen.naked.target_feature](#r-attributes.codegen.naked.target_feature)

The [`target_feature` attribute](#r-attributes.codegen.target_feature) cannot be applied to a naked function.

[attributes.codegen.naked.abi](#r-attributes.codegen.naked.abi)

A naked function cannot use the [“Rust” ABI](../items/external-blocks.md#r-items.extern.abi.rust).

[attributes.codegen.no_builtins](#r-attributes.codegen.no_builtins)

## The `no_builtins` attribute

[attributes.codegen.no_builtins.intro](#r-attributes.codegen.no_builtins.intro)

The *`no_builtins` [attribute](../attributes.md)* disables optimization of certain code patterns related to calls to library functions that are assumed to exist.

> ![](../../assets/inline/codegen-17.svg)Example
>
> ``` playground
> #![allow(unused)]
> #![no_builtins]
> fn main() {
> }
> ```

[attributes.codegen.no_builtins.syntax](#r-attributes.codegen.no_builtins.syntax)

The `no_builtins` attribute uses the [MetaWord](../attributes.md#grammar-MetaWord) syntax.

[attributes.codegen.no_builtins.allowed-positions](#r-attributes.codegen.no_builtins.allowed-positions)

The `no_builtins` attribute can only be applied to the crate root.

[attributes.codegen.no_builtins.duplicates](#r-attributes.codegen.no_builtins.duplicates)

Only the first use of the `no_builtins` attribute has effect.

> ![](../../assets/inline/codegen-18.svg)Note
>
> `rustc` lints against any use following the first.

[attributes.codegen.target_feature](#r-attributes.codegen.target_feature)

## The `target_feature` attribute

[attributes.codegen.target_feature.intro](#r-attributes.codegen.target_feature.intro)

The *`target_feature` [attribute](../attributes.md)* may be applied to a function to enable code generation of that function for specific platform architecture features. It uses the [MetaListNameValueStr](../attributes.md#grammar-MetaListNameValueStr) syntax with a single key of `enable` whose value is a string of comma-separated feature names to enable.

``` playground
#![allow(unused)]
fn main() {
#[cfg(target_feature = "avx2")]
#[target_feature(enable = "avx2")]
fn foo_avx2() {}
}
```

[attributes.codegen.target_feature.arch](#r-attributes.codegen.target_feature.arch)

Each [target architecture](../conditional-compilation.md#target_arch) has a set of features that may be enabled. It is an error to specify a feature for a target architecture that the crate is not being compiled for.

[attributes.codegen.target_feature.closures](#r-attributes.codegen.target_feature.closures)

Closures defined within a `target_feature`-annotated function inherit the attribute from the enclosing function.

[attributes.codegen.target_feature.target-ub](#r-attributes.codegen.target_feature.target-ub)

It is [undefined behavior](../behavior-considered-undefined.md) to call a function that is compiled with a feature that is not supported on the current platform the code is running on, *except* if the platform explicitly documents this to be safe.

[attributes.codegen.target_feature.safety-restrictions](#r-attributes.codegen.target_feature.safety-restrictions)

The following restrictions apply unless otherwise specified by the platform rules below:

- Safe `#[target_feature]` functions (and closures that inherit the attribute) can only be safely called within a caller that enables all the `target_feature`s that the callee enables. This restriction does not apply in an `unsafe` context.
- Safe `#[target_feature]` functions (and closures that inherit the attribute) can only be coerced to *safe* function pointers in contexts that enable all the `target_feature`s that the coercee enables. This restriction does not apply to `unsafe` function pointers.

Implicitly enabled features are included in this rule. For example an `sse2` function can call ones marked with `sse`.

``` playground
#![allow(unused)]
fn main() {
#[cfg(target_feature = "sse2")] {
#[target_feature(enable = "sse")]
fn foo_sse() {}

fn bar() {
    // Calling `foo_sse` here is unsafe, as we must ensure that SSE is
    // available first, even if `sse` is enabled by default on the target
    // platform or manually enabled as compiler flags.
    unsafe {
        foo_sse();
    }
}

#[target_feature(enable = "sse")]
fn bar_sse() {
    // Calling `foo_sse` here is safe.
    foo_sse();
    || foo_sse();
}

#[target_feature(enable = "sse2")]
fn bar_sse2() {
    // Calling `foo_sse` here is safe because `sse2` implies `sse`.
    foo_sse();
}
}
}
```

[attributes.codegen.target_feature.fn-traits](#r-attributes.codegen.target_feature.fn-traits)

A function with a `#[target_feature]` attribute *never* implements the `Fn` family of traits, although closures inheriting features from the enclosing function do.

[attributes.codegen.target_feature.allowed-positions](#r-attributes.codegen.target_feature.allowed-positions)

The `#[target_feature]` attribute is not allowed on the following places:

- [the `main` function](../crates-and-source-files.md#r-crate.main)
- a [`panic_handler` function](../panic.md#r-panic.panic_handler)
- safe trait methods
- safe default functions in traits

[attributes.codegen.target_feature.inline](#r-attributes.codegen.target_feature.inline)

Functions marked with `target_feature` are not inlined into a context that does not support the given features. The `#[inline(always)]` attribute may not be used with a `target_feature` attribute.

[attributes.codegen.target_feature.availability](#r-attributes.codegen.target_feature.availability)

### Available features

The following is a list of the available feature names.

[attributes.codegen.target_feature.x86](#r-attributes.codegen.target_feature.x86)

#### `x86` or `x86_64`

Executing code with unsupported features is undefined behavior on this platform. Hence on this platform use of `#[target_feature]` functions follows the [above restrictions](#r-attributes.codegen.target_feature.safety-restrictions).

| Feature | Implicitly Enables | Description |
|----|----|----|
| `adx` |  | [ADX](https://en.wikipedia.org/wiki/Intel_ADX) — Multi-Precision Add-Carry Instruction Extensions |
| `aes` | `sse2` | [AES](https://en.wikipedia.org/wiki/AES_instruction_set) — Advanced Encryption Standard |
| `avx` | `sse4.2` | [AVX](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions) — Advanced Vector Extensions |
| `avx2` | `avx` | [AVX2](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#AVX2) — Advanced Vector Extensions 2 |
| `avx512bf16` | `avx512bw` | [AVX512-BF16](https://en.wikipedia.org/wiki/AVX-512#BF16) — Advanced Vector Extensions 512-bit - Bfloat16 Extensions |
| `avx512bitalg` | `avx512bw` | [AVX512-BITALG](https://en.wikipedia.org/wiki/AVX-512#VPOPCNTDQ_and_BITALG) — Advanced Vector Extensions 512-bit - Bit Algorithms |
| `avx512bw` | `avx512f` | [AVX512-BW](https://en.wikipedia.org/wiki/AVX-512#BW,_DQ_and_VBMI) — Advanced Vector Extensions 512-bit - Byte and Word Instructions |
| `avx512cd` | `avx512f` | [AVX512-CD](https://en.wikipedia.org/wiki/AVX-512#Conflict_detection) — Advanced Vector Extensions 512-bit - Conflict Detection Instructions |
| `avx512dq` | `avx512f` | [AVX512-DQ](https://en.wikipedia.org/wiki/AVX-512#BW,_DQ_and_VBMI) — Advanced Vector Extensions 512-bit - Doubleword and Quadword Instructions |
| `avx512f` | `avx2`, `fma`, `f16c` | [AVX512-F](https://en.wikipedia.org/wiki/AVX-512) — Advanced Vector Extensions 512-bit - Foundation |
| `avx512fp16` | `avx512bw` | [AVX512-FP16](https://en.wikipedia.org/wiki/AVX-512#FP16) — Advanced Vector Extensions 512-bit - Float16 Extensions |
| `avx512ifma` | `avx512f` | [AVX512-IFMA](https://en.wikipedia.org/wiki/AVX-512#IFMA) — Advanced Vector Extensions 512-bit - Integer Fused Multiply Add |
| `avx512vbmi` | `avx512bw` | [AVX512-VBMI](https://en.wikipedia.org/wiki/AVX-512#BW,_DQ_and_VBMI) — Advanced Vector Extensions 512-bit - Vector Byte Manipulation Instructions |
| `avx512vbmi2` | `avx512bw` | [AVX512-VBMI2](https://en.wikipedia.org/wiki/AVX-512#VBMI2) — Advanced Vector Extensions 512-bit - Vector Byte Manipulation Instructions 2 |
| `avx512vl` | `avx512f` | [AVX512-VL](https://en.wikipedia.org/wiki/AVX-512) — Advanced Vector Extensions 512-bit - Vector Length Extensions |
| `avx512vnni` | `avx512f` | [AVX512-VNNI](https://en.wikipedia.org/wiki/AVX-512#VNNI) — Advanced Vector Extensions 512-bit - Vector Neural Network Instructions |
| `avx512vp2intersect` | `avx512f` | [AVX512-VP2INTERSECT](https://en.wikipedia.org/wiki/AVX-512#VP2INTERSECT) — Advanced Vector Extensions 512-bit - Vector Pair Intersection to a Pair of Mask Registers |
| `avx512vpopcntdq` | `avx512f` | [AVX512-VPOPCNTDQ](https://en.wikipedia.org/wiki/AVX-512#VPOPCNTDQ_and_BITALG) — Advanced Vector Extensions 512-bit - Vector Population Count Instruction |
| `avxifma` | `avx2` | [AVX-IFMA](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#AVX-VNNI,_AVX-IFMA) — Advanced Vector Extensions - Integer Fused Multiply Add |
| `avxneconvert` | `avx2` | [AVX-NE-CONVERT](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#AVX-VNNI,_AVX-IFMA) — Advanced Vector Extensions - No-Exception Floating-Point conversion Instructions |
| `avxvnni` | `avx2` | [AVX-VNNI](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#AVX-VNNI,_AVX-IFMA) — Advanced Vector Extensions - Vector Neural Network Instructions |
| `avxvnniint16` | `avx2` | [AVX-VNNI-INT16](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#AVX-VNNI,_AVX-IFMA) — Advanced Vector Extensions - Vector Neural Network Instructions with 16-bit Integers |
| `avxvnniint8` | `avx2` | [AVX-VNNI-INT8](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#AVX-VNNI,_AVX-IFMA) — Advanced Vector Extensions - Vector Neural Network Instructions with 8-bit Integers |
| `bmi1` |  | [BMI1](https://en.wikipedia.org/wiki/Bit_Manipulation_Instruction_Sets) — Bit Manipulation Instruction Sets |
| `bmi2` |  | [BMI2](https://en.wikipedia.org/wiki/Bit_Manipulation_Instruction_Sets#BMI2) — Bit Manipulation Instruction Sets 2 |
| `cmpxchg16b` |  | [`cmpxchg16b`](https://www.felixcloutier.com/x86/cmpxchg8b:cmpxchg16b) — Compares and exchange 16 bytes (128 bits) of data atomically |
| `f16c` | `avx` | [F16C](https://en.wikipedia.org/wiki/F16C) — 16-bit floating point conversion instructions |
| `fma` | `avx` | [FMA3](https://en.wikipedia.org/wiki/FMA_instruction_set) — Three-operand fused multiply-add |
| `fxsr` |  | [`fxsave`](https://www.felixcloutier.com/x86/fxsave) and [`fxrstor`](https://www.felixcloutier.com/x86/fxrstor) — Save and restore x87 FPU, MMX Technology, and SSE State |
| `gfni` | `sse2` | [GFNI](https://en.wikipedia.org/wiki/AVX-512#GFNI) — Galois Field New Instructions |
| `kl` | `sse2` | [KEYLOCKER](https://en.wikipedia.org/wiki/List_of_x86_cryptographic_instructions#Intel_Key_Locker_instructions) — Intel Key Locker Instructions |
| `lzcnt` |  | [`lzcnt`](https://www.felixcloutier.com/x86/lzcnt) — Leading zeros count |
| `movbe` |  | [`movbe`](https://www.felixcloutier.com/x86/movbe) — Move data after swapping bytes |
| `pclmulqdq` | `sse2` | [`pclmulqdq`](https://www.felixcloutier.com/x86/pclmulqdq) — Packed carry-less multiplication quadword |
| `popcnt` |  | [`popcnt`](https://www.felixcloutier.com/x86/popcnt) — Count of bits set to 1 |
| `rdrand` |  | [`rdrand`](https://en.wikipedia.org/wiki/RdRand) — Read random number |
| `rdseed` |  | [`rdseed`](https://en.wikipedia.org/wiki/RdRand) — Read random seed |
| `sha` | `sse2` | [SHA](https://en.wikipedia.org/wiki/Intel_SHA_extensions) — Secure Hash Algorithm |
| `sha512` | `avx2` | [SHA512](https://en.wikipedia.org/wiki/Intel_SHA_extensions) — Secure Hash Algorithm with 512-bit digest |
| `sm3` | `avx` | [SM3](https://en.wikipedia.org/wiki/List_of_x86_cryptographic_instructions#Intel_SHA_and_SM3_instructions) — ShangMi 3 Hash Algorithm |
| `sm4` | `avx2` | [SM4](https://en.wikipedia.org/wiki/List_of_x86_cryptographic_instructions#Intel_SHA_and_SM3_instructions) — ShangMi 4 Cipher Algorithm |
| `sse` |  | [SSE](https://en.wikipedia.org/wiki/Streaming_SIMD_Extensions) — Streaming SIMD Extensions |
| `sse2` | `sse` | [SSE2](https://en.wikipedia.org/wiki/SSE2) — Streaming SIMD Extensions 2 |
| `sse3` | `sse2` | [SSE3](https://en.wikipedia.org/wiki/SSE3) — Streaming SIMD Extensions 3 |
| `sse4.1` | `ssse3` | [SSE4.1](https://en.wikipedia.org/wiki/SSE4#SSE4.1) — Streaming SIMD Extensions 4.1 |
| `sse4.2` | `sse4.1` | [SSE4.2](https://en.wikipedia.org/wiki/SSE4#SSE4.2) — Streaming SIMD Extensions 4.2 |
| `sse4a` | `sse3` | [SSE4a](https://en.wikipedia.org/wiki/SSE4#SSE4a) — Streaming SIMD Extensions 4a |
| `ssse3` | `sse3` | [SSSE3](https://en.wikipedia.org/wiki/SSSE3) — Supplemental Streaming SIMD Extensions 3 |
| `tbm` |  | [TBM](https://en.wikipedia.org/wiki/X86_Bit_manipulation_instruction_set#TBM_(Trailing_Bit_Manipulation)) — Trailing Bit Manipulation |
| `vaes` | `avx2`, `aes` | [VAES](https://en.wikipedia.org/wiki/AVX-512#VAES) — Vector AES Instructions |
| `vpclmulqdq` | `avx`, `pclmulqdq` | [VPCLMULQDQ](https://en.wikipedia.org/wiki/AVX-512#VPCLMULQDQ) — Vector Carry-less multiplication of Quadwords |
| `widekl` | `kl` | [KEYLOCKER_WIDE](https://en.wikipedia.org/wiki/List_of_x86_cryptographic_instructions#Intel_Key_Locker_instructions) — Intel Wide Keylocker Instructions |
| `xsave` |  | [`xsave`](https://www.felixcloutier.com/x86/xsave) — Save processor extended states |
| `xsavec` |  | [`xsavec`](https://www.felixcloutier.com/x86/xsavec) — Save processor extended states with compaction |
| `xsaveopt` |  | [`xsaveopt`](https://www.felixcloutier.com/x86/xsaveopt) — Save processor extended states optimized |
| `xsaves` |  | [`xsaves`](https://www.felixcloutier.com/x86/xsaves) — Save processor extended states supervisor |

[attributes.codegen.target_feature.aarch64](#r-attributes.codegen.target_feature.aarch64)

#### `aarch64`

On this platform the use of `#[target_feature]` functions follows the [above restrictions](#r-attributes.codegen.target_feature.safety-restrictions).

Further documentation on these features can be found in the [ARM Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest), or elsewhere on [developer.arm.com](https://developer.arm.com).

> ![](../../assets/inline/codegen-19.svg)Note
>
> The following pairs of features should both be marked as enabled or disabled together if used:
>
> - `paca` and `pacg`, which LLVM currently implements as one feature.

| Feature | Implicitly Enables | Feature Name |
|----|----|----|
| `aes` | `neon` | FEAT_AES & FEAT_PMULL — Advanced SIMD AES & PMULL instructions |
| `bf16` |  | FEAT_BF16 — BFloat16 instructions |
| `bti` |  | FEAT_BTI — Branch Target Identification |
| `crc` |  | FEAT_CRC — CRC32 checksum instructions |
| `dit` |  | FEAT_DIT — Data Independent Timing instructions |
| `dotprod` | `neon` | FEAT_DotProd — Advanced SIMD Int8 dot product instructions |
| `dpb` |  | FEAT_DPB — Data cache clean to point of persistence |
| `dpb2` | `dpb` | FEAT_DPB2 — Data cache clean to point of deep persistence |
| `f32mm` | `sve` | FEAT_F32MM — SVE single-precision FP matrix multiply instruction |
| `f64mm` | `sve` | FEAT_F64MM — SVE double-precision FP matrix multiply instruction |
| `fcma` | `neon` | FEAT_FCMA — Floating point complex number support |
| `fhm` | `fp16` | FEAT_FHM — Half-precision FP FMLAL instructions |
| `flagm` |  | FEAT_FLAGM — Conditional flag manipulation |
| `fp16` | `neon` | FEAT_FP16 — Half-precision FP data processing |
| `frintts` |  | FEAT_FRINTTS — Floating-point to int helper instructions |
| `i8mm` |  | FEAT_I8MM — Int8 Matrix Multiplication |
| `jsconv` | `neon` | FEAT_JSCVT — JavaScript conversion instruction |
| `lor` |  | FEAT_LOR — Limited Ordering Regions extension |
| `lse` |  | FEAT_LSE — Large System Extensions |
| `mte` |  | FEAT_MTE & FEAT_MTE2 — Memory Tagging Extension |
| `neon` |  | FEAT_AdvSimd & FEAT_FP — Floating Point and Advanced SIMD extension |
| `paca` |  | FEAT_PAUTH — Pointer Authentication (address authentication) |
| `pacg` |  | FEAT_PAUTH — Pointer Authentication (generic authentication) |
| `pan` |  | FEAT_PAN — Privileged Access-Never extension |
| `pmuv3` |  | FEAT_PMUv3 — Performance Monitors extension (v3) |
| `rand` |  | FEAT_RNG — Random Number Generator |
| `ras` |  | FEAT_RAS & FEAT_RASv1p1 — Reliability, Availability and Serviceability extension |
| `rcpc` |  | FEAT_LRCPC — Release consistent Processor Consistent |
| `rcpc2` | `rcpc` | FEAT_LRCPC2 — RcPc with immediate offsets |
| `rdm` | `neon` | FEAT_RDM — Rounding Double Multiply accumulate |
| `sb` |  | FEAT_SB — Speculation Barrier |
| `sha2` | `neon` | FEAT_SHA1 & FEAT_SHA256 — Advanced SIMD SHA instructions |
| `sha3` | `sha2` | FEAT_SHA512 & FEAT_SHA3 — Advanced SIMD SHA instructions |
| `sm4` | `neon` | FEAT_SM3 & FEAT_SM4 — Advanced SIMD SM3/4 instructions |
| `spe` |  | FEAT_SPE — Statistical Profiling Extension |
| `ssbs` |  | FEAT_SSBS & FEAT_SSBS2 — Speculative Store Bypass Safe |
| `sve` | `neon` | FEAT_SVE — Scalable Vector Extension |
| `sve2` | `sve` | FEAT_SVE2 — Scalable Vector Extension 2 |
| `sve2-aes` | `sve2`, `aes` | FEAT_SVE_AES & FEAT_SVE_PMULL128 — SVE AES instructions |
| `sve2-bitperm` | `sve2` | FEAT_SVE2_BitPerm — SVE Bit Permute |
| `sve2-sha3` | `sve2`, `sha3` | FEAT_SVE2_SHA3 — SVE SHA3 instructions |
| `sve2-sm4` | `sve2`, `sm4` | FEAT_SVE2_SM4 — SVE SM4 instructions |
| `tme` |  | FEAT_TME — Transactional Memory Extension |
| `vh` |  | FEAT_VHE — Virtualization Host Extensions |

[attributes.codegen.target_feature.loongarch](#r-attributes.codegen.target_feature.loongarch)

#### `loongarch`

On this platform the use of `#[target_feature]` functions follows the [above restrictions](#r-attributes.codegen.target_feature.safety-restrictions).

| Feature | Implicitly Enables | Description |
|----|----|----|
| `f` |  | [F](https://loongson.github.io/LoongArch-Documentation/LoongArch-Vol1-EN.html#cpucfg-fp_sp) — Single-precision float-point instructions |
| `d` | `f` | [D](https://loongson.github.io/LoongArch-Documentation/LoongArch-Vol1-EN.html#cpucfg-fp_dp) — Double-precision float-point instructions |
| `frecipe` |  | [FRECIPE](https://loongson.github.io/LoongArch-Documentation/LoongArch-Vol1-EN.html#cpucfg-frecipe) — Reciprocal approximation instructions |
| `lasx` | `lsx` | [LASX](https://loongson.github.io/LoongArch-Documentation/LoongArch-Vol1-EN.html#cpucfg-lasx) — 256-bit vector instructions |
| `lbt` |  | [LBT](https://loongson.github.io/LoongArch-Documentation/LoongArch-Vol1-EN.html#cpucfg-lbt_x86) — Binary translation instructions |
| `lsx` | `d` | [LSX](https://loongson.github.io/LoongArch-Documentation/LoongArch-Vol1-EN.html#cpucfg-lsx) — 128-bit vector instructions |
| `lvz` |  | [LVZ](https://loongson.github.io/LoongArch-Documentation/LoongArch-Vol1-EN.html#cpucfg-lvz) — Virtualization instructions |
| `div32` |  | [DIV32](https://loongson.github.io/LoongArch-Documentation/LoongArch-Vol1-EN.html#cpucfg-div32) — Division instructions accepting non-sign-extended 32-bit operands |
| `lam-bh` |  | [LAM-BH](https://loongson.github.io/LoongArch-Documentation/LoongArch-Vol1-EN.html#cpucfg-lam_bh) — Atomic swap and add instructions for byte and halfword |
| `lamcas` |  | [LAMCAS](https://loongson.github.io/LoongArch-Documentation/LoongArch-Vol1-EN.html#cpucfg-lamcas) — Atomic compare-and-swap instructions for byte, halfword, word, and doubleword |
| `ld-seq-sa` |  | [LD-SEQ-SA](https://loongson.github.io/LoongArch-Documentation/LoongArch-Vol1-EN.html#cpucfg-ld_seq_sa) — Sequential ordering of load operations to the same address |
| `scq` |  | [SCQ](https://loongson.github.io/LoongArch-Documentation/LoongArch-Vol1-EN.html#cpucfg-scq) — Store-conditional quadword instructions |

[attributes.codegen.target_feature.riscv](#r-attributes.codegen.target_feature.riscv)

#### `riscv32` or `riscv64`

On this platform the use of `#[target_feature]` functions follows the [above restrictions](#r-attributes.codegen.target_feature.safety-restrictions).

Further documentation on these features can be found in their respective specification. Many specifications are described in the [RISC-V ISA Manual](https://github.com/riscv/riscv-isa-manual), [version 20250508](https://github.com/riscv/riscv-isa-manual/tree/20250508), or in another manual hosted on the [RISC-V GitHub Account](https://github.com/riscv).

| Feature | Implicitly Enables | Description |
|----|----|----|
| `a` | `zaamo`, `zalrsc` | [A](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/a-st-ext.adoc) — Atomic instructions |
| `b` | `zba`, `zbc`, `zbs` | [B](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/b-st-ext.adoc) — Bit Manipulation instructions |
| `c` | `zca` | [C](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/c-st-ext.adoc) — Compressed instructions |
| `m` |  | [M](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/m-st-ext.adoc) — Integer Multiplication and Division instructions |
| `za64rs` | `za128rs` | [Za64rs](https://github.com/riscv/riscv-profiles/blob/rva23-rvb23-ratified/src/rva23-profile.adoc) — Platform Behavior: Naturally aligned Reservation sets with ≦ 64 Bytes |
| `za128rs` |  | [Za128rs](https://github.com/riscv/riscv-profiles/blob/v1.0/profiles.adoc) — Platform Behavior: Naturally aligned Reservation sets with ≦ 128 Bytes |
| `zaamo` |  | [Zaamo](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/a-st-ext.adoc) — Atomic Memory Operation instructions |
| `zabha` | `zaamo` | [Zabha](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/zabha.adoc) — Byte and Halfword Atomic Memory Operation instructions |
| `zacas` | `zaamo` | [Zacas](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/zacas.adoc) — Atomic Compare-and-Swap (CAS) instructions |
| `zalrsc` |  | [Zalrsc](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/a-st-ext.adoc) — Load-Reserved/Store-Conditional instructions |
| `zama16b` |  | [Zama16b](https://github.com/riscv/riscv-profiles/blob/rva23-rvb23-ratified/src/rva23-profile.adoc) — Platform Behavior: Misaligned loads, stores, and AMOs to main memory regions that do not cross a naturally aligned 16-byte boundary are atomic |
| `zawrs` |  | [Zawrs](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/zawrs.adoc) — Wait-on-Reservation-Set instructions |
| `zba` |  | [Zba](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/b-st-ext.adoc) — Address Generation instructions |
| `zbb` |  | [Zbb](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/b-st-ext.adoc) — Basic bit-manipulation |
| `zbc` | `zbkc` | [Zbc](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/b-st-ext.adoc) — Carry-less multiplication |
| `zbkb` |  | [Zbkb](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/b-st-ext.adoc) — Bit Manipulation Instructions for Cryptography |
| `zbkc` |  | [Zbkc](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/b-st-ext.adoc) — Carry-less multiplication for Cryptography |
| `zbkx` |  | [Zbkx](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/b-st-ext.adoc) — Crossbar permutations |
| `zbs` |  | [Zbs](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/b-st-ext.adoc) — Single-bit instructions |
| `zca` |  | [Zca](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/zc.adoc) — Compressed instructions: integer part subset |
| `zcb` | `zca` | [Zcb](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/zc.adoc) — Simple Code-size Saving Compressed instructions |
| `zcmop` | `zca` | [Zcmop](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/zimop.adoc) — Compressed May-Be-Operations |
| `zic64b` |  | [Zic64b](https://github.com/riscv/riscv-profiles/blob/v1.0/profiles.adoc) — Platform Behavior: Naturally aligned 64 byte Cache blocks |
| `zicbom` |  | [Zicbom](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/cmo.adoc) — Cache-Block Management instructions |
| `zicbop` |  | [Zicbop](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/cmo.adoc) — Cache-Block Prefetch Hint instructions |
| `zicboz` |  | [Zicboz](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/cmo.adoc) — Cache-Block Zero instruction |
| `ziccamoa` |  | [Ziccamoa](https://github.com/riscv/riscv-profiles/blob/v1.0/profiles.adoc) — Platform Behavior: Cacheable and Coherent Main memory supports all basic atomic operations |
| `ziccif` |  | [Ziccif](https://github.com/riscv/riscv-profiles/blob/v1.0/profiles.adoc) — Platform Behavior: Cacheable and Coherent Main memory supports instruction fetch and fetches of naturally aligned power-of-2 sizes up to `min(ILEN,XLEN)` are atomic |
| `zicclsm` |  | [Zicclsm](https://github.com/riscv/riscv-profiles/blob/v1.0/profiles.adoc) — Platform Behavior: Cacheable and Coherent Main memory supports misaligned load/store accesses |
| `ziccrse` |  | [Ziccrse](https://github.com/riscv/riscv-profiles/blob/v1.0/profiles.adoc) — Platform Behavior: Cacheable and Coherent Main memory guarantees eventual success on LR/SC sequences |
| `zicntr` | `zicsr` | [Zicntr](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/counters.adoc) — Base Counters and Timers |
| `zicond` |  | [Zicond](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/zicond.adoc) — Integer Conditional Operation instructions |
| `zicsr` |  | [Zicsr](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/zicsr.adoc) — Control and Status Register (CSR) instructions |
| `zifencei` |  | [Zifencei](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/zifencei.adoc) — Instruction-Fetch Fence instruction |
| `zihintntl` |  | [Zihintntl](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/zihintntl.adoc) — Non-Temporal Locality Hint instructions |
| `zihintpause` |  | [Zihintpause](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/zihintpause.adoc) — Pause Hint instruction |
| `zihpm` | `zicsr` | [Zihpm](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/counters.adoc) — Hardware Performance Counters |
| `zimop` |  | [Zimop](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/zimop.adoc) — May-Be-Operations |
| `zk` | `zkn`, `zkr`, `zks`, `zkt`, `zbkb`, `zbkc`, `zkbx` | [Zk](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc) — Scalar Cryptography |
| `zkn` | `zknd`, `zkne`, `zknh`, `zbkb`, `zbkc`, `zkbx` | [Zkn](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc) — NIST Algorithm suite extension |
| `zknd` |  | [Zknd](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc) — NIST Suite: AES Decryption |
| `zkne` |  | [Zkne](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc) — NIST Suite: AES Encryption |
| `zknh` |  | [Zknh](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc) — NIST Suite: Hash Function Instructions |
| `zkr` |  | [Zkr](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc) — Entropy Source Extension |
| `zks` | `zksed`, `zksh`, `zbkb`, `zbkc`, `zkbx` | [Zks](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc) — ShangMi Algorithm Suite |
| `zksed` |  | [Zksed](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc) — ShangMi Suite: SM4 Block Cipher Instructions |
| `zksh` |  | [Zksh](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc) — ShangMi Suite: SM3 Hash Function Instructions |
| `zkt` |  | [Zkt](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc) — Data Independent Execution Latency Subset |
| `ztso` |  | [Ztso](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/ztso-st-ext.adoc) — Total Store Ordering |

[attributes.codegen.target_feature.wasm](#r-attributes.codegen.target_feature.wasm)

#### `wasm32` or `wasm64`

Safe `#[target_feature]` functions may always be used in safe contexts on Wasm platforms. It is impossible to cause undefined behavior via the `#[target_feature]` attribute because attempting to use instructions unsupported by the Wasm engine will fail at load time without the risk of being interpreted in a way different from what the compiler expected.

| Feature | Implicitly Enables | Description |
|----|----|----|
| `bulk-memory` |  | [WebAssembly bulk memory operations proposal](https://github.com/WebAssembly/bulk-memory-operations) |
| `extended-const` |  | [WebAssembly extended const expressions proposal](https://github.com/WebAssembly/extended-const) |
| `mutable-globals` |  | [WebAssembly mutable global proposal](https://github.com/WebAssembly/mutable-global) |
| `nontrapping-fptoint` |  | [WebAssembly non-trapping float-to-int conversion proposal](https://github.com/WebAssembly/nontrapping-float-to-int-conversions) |
| `relaxed-simd` | `simd128` | [WebAssembly relaxed simd proposal](https://github.com/WebAssembly/relaxed-simd) |
| `sign-ext` |  | [WebAssembly sign extension operators Proposal](https://github.com/WebAssembly/sign-extension-ops) |
| `simd128` |  | [WebAssembly simd proposal](https://github.com/webassembly/simd) |
| `multivalue` |  | [WebAssembly multivalue proposal](https://github.com/webassembly/multi-value) |
| `reference-types` |  | [WebAssembly reference-types proposal](https://github.com/webassembly/reference-types) |
| `tail-call` |  | [WebAssembly tail-call proposal](https://github.com/webassembly/tail-call) |

[attributes.codegen.target_feature.s390x](#r-attributes.codegen.target_feature.s390x)

#### `s390x`

On `s390x` targets, use of functions with the `#[target_feature]` attribute follows the [above restrictions](#r-attributes.codegen.target_feature.safety-restrictions).

Further documentation on these features can be found in the “Additions to z/Architecture” section of Chapter 1 of the *[z/Architecture Principles of Operation](https://publibfp.dhe.ibm.com/epubs/pdf/a227832d.pdf)*.

| Feature | Implicitly Enables | Description |
|----|----|----|
| `vector` |  | 128-bit vector instructions |
| `vector-enhancements-1` | `vector` | vector enhancements 1 |
| `vector-enhancements-2` | `vector-enhancements-1` | vector enhancements 2 |
| `vector-enhancements-3` | `vector-enhancements-2` | vector enhancements 3 |
| `vector-packed-decimal` | `vector` | vector packed-decimal |
| `vector-packed-decimal-enhancement` | `vector-packed-decimal` | vector packed-decimal enhancement |
| `vector-packed-decimal-enhancement-2` | `vector-packed-decimal-enhancement-2` | vector packed-decimal enhancement 2 |
| `vector-packed-decimal-enhancement-3` | `vector-packed-decimal-enhancement-3` | vector packed-decimal enhancement 3 |
| `nnp-assist` | `vector` | nnp assist |
| `miscellaneous-extensions-2` |  | miscellaneous extensions 2 |
| `miscellaneous-extensions-3` |  | miscellaneous extensions 3 |
| `miscellaneous-extensions-4` |  | miscellaneous extensions 4 |

[attributes.codegen.target_feature.info](#r-attributes.codegen.target_feature.info)

### Additional information

[attributes.codegen.target_feature.remark-cfg](#r-attributes.codegen.target_feature.remark-cfg)

See the [`target_feature` conditional compilation option](../conditional-compilation.md#target_feature) for selectively enabling or disabling compilation of code based on compile-time settings. Note that this option is not affected by the `target_feature` attribute, and is only driven by the features enabled for the entire crate.

[attributes.codegen.target_feature.remark-rt](#r-attributes.codegen.target_feature.remark-rt)

Whether a feature is enabled can be checked at runtime using a platform-specific macro from the standard library, for instance [`is_x86_feature_detected`](../../std/arch/macro.is_x86_feature_detected.html) or [`is_aarch64_feature_detected`](../../std/arch/macro.is_aarch64_feature_detected.html).

> ![](../../assets/inline/codegen-20.svg)Note
>
> `rustc` has a default set of features enabled for each target and CPU. The CPU may be chosen with the [`-C target-cpu`](../../rustc/codegen-options/index.html#target-cpu) flag. Individual features may be enabled or disabled for an entire crate with the [`-C target-feature`](../../rustc/codegen-options/index.html#target-feature) flag.

[attributes.codegen.track_caller](#r-attributes.codegen.track_caller)

## The `track_caller` attribute

[attributes.codegen.track_caller.allowed-positions](#r-attributes.codegen.track_caller.allowed-positions)

The `track_caller` attribute may be applied to any function with [`"Rust"` ABI](../items/external-blocks.md#abi) with the exception of the entry point `fn main`.

[attributes.codegen.track_caller.traits](#r-attributes.codegen.track_caller.traits)

When applied to functions and methods in trait declarations, the attribute applies to all implementations. If the trait provides a default implementation with the attribute, then the attribute also applies to override implementations.

[attributes.codegen.track_caller.extern](#r-attributes.codegen.track_caller.extern)

When applied to a function in an `extern` block the attribute must also be applied to any linked implementations, otherwise undefined behavior results. When applied to a function which is made available to an `extern` block, the declaration in the `extern` block must also have the attribute, otherwise undefined behavior results.

[attributes.codegen.track_caller.behavior](#r-attributes.codegen.track_caller.behavior)

### Behavior

Applying the attribute to a function `f` allows code within `f` to get a hint of the [`Location`](../../core/panic/location/struct.Location.html) of the “topmost” tracked call that led to `f`’s invocation. At the point of observation, an implementation behaves as if it walks up the stack from `f`’s frame to find the nearest frame of an *unattributed* function `outer`, and it returns the [`Location`](../../core/panic/location/struct.Location.html) of the tracked call in `outer`.

``` playground
#![allow(unused)]
fn main() {
#[track_caller]
fn f() {
    println!("{}", std::panic::Location::caller());
}
}
```

> ![](../../assets/inline/codegen-21.svg)Note
>
> `core` provides [`core::panic::Location::caller`](../../core/panic/location/struct.Location.html#method.caller) for observing caller locations. It wraps the [`core::intrinsics::caller_location`](../../core/intrinsics/fn.caller_location.html) intrinsic implemented by `rustc`.

> ![](../../assets/inline/codegen-22.svg)Note
>
> Because the resulting `Location` is a hint, an implementation may halt its walk up the stack early. See [Limitations](#limitations) for important caveats.

#### Examples

When `f` is called directly by `calls_f`, code in `f` observes its callsite within `calls_f`:

``` playground
#![allow(unused)]
fn main() {
#[track_caller]
fn f() {
    println!("{}", std::panic::Location::caller());
}
fn calls_f() {
    f(); // <-- f() prints this location
}
}
```

When `f` is called by another attributed function `g` which is in turn called by `calls_g`, code in both `f` and `g` observes `g`’s callsite within `calls_g`:

``` playground
#![allow(unused)]
fn main() {
#[track_caller]
fn f() {
    println!("{}", std::panic::Location::caller());
}
#[track_caller]
fn g() {
    println!("{}", std::panic::Location::caller());
    f();
}

fn calls_g() {
    g(); // <-- g() prints this location twice, once itself and once from f()
}
}
```

When `g` is called by another attributed function `h` which is in turn called by `calls_h`, all code in `f`, `g`, and `h` observes `h`’s callsite within `calls_h`:

``` playground
#![allow(unused)]
fn main() {
#[track_caller]
fn f() {
    println!("{}", std::panic::Location::caller());
}
#[track_caller]
fn g() {
    println!("{}", std::panic::Location::caller());
    f();
}
#[track_caller]
fn h() {
    println!("{}", std::panic::Location::caller());
    g();
}

fn calls_h() {
    h(); // <-- prints this location three times, once itself, once from g(), once from f()
}
}
```

And so on.

[attributes.codegen.track_caller.limits](#r-attributes.codegen.track_caller.limits)

### Limitations

[attributes.codegen.track_caller.hint](#r-attributes.codegen.track_caller.hint)

This information is a hint and implementations are not required to preserve it.

[attributes.codegen.track_caller.decay](#r-attributes.codegen.track_caller.decay)

In particular, coercing a function with `#[track_caller]` to a function pointer creates a shim which appears to observers to have been called at the attributed function’s definition site, losing actual caller information across virtual calls. A common example of this coercion is the creation of a trait object whose methods are attributed.

> ![](../../assets/inline/codegen-23.svg)Note
>
> The aforementioned shim for function pointers is necessary because `rustc` implements `track_caller` in a codegen context by appending an implicit parameter to the function ABI, but this would be unsound for an indirect call because the parameter is not a part of the function’s type and a given function pointer type may or may not refer to a function with the attribute. The creation of a shim hides the implicit parameter from callers of the function pointer, preserving soundness.

[attributes.codegen.instruction_set](#r-attributes.codegen.instruction_set)

## The `instruction_set` attribute

[attributes.codegen.instruction_set.intro](#r-attributes.codegen.instruction_set.intro)

The *`instruction_set` [attribute](../attributes.md)* specifies the instruction set that a function will use during code generation. This allows mixing more than one instruction set in a single program.

> ![](../../assets/inline/codegen-24.svg)Example
>
> ``` rust
> #[instruction_set(arm::a32)]
> fn arm_code() {}
>
> #[instruction_set(arm::t32)]
> fn thumb_code() {}
> ```

[attributes.codegen.instruction_set.syntax](#r-attributes.codegen.instruction_set.syntax)

The `instruction_set` attribute uses the [MetaListPaths](../attributes.md#grammar-MetaListPaths) syntax to specify a single path consisting of the architecture family name and instruction set name.

[attributes.codegen.instruction_set.allowed-positions](#r-attributes.codegen.instruction_set.allowed-positions)

The `instruction_set` attribute may only be applied to functions with [bodies](../items/functions.md#r-items.fn.body) — [closures](../expressions/closure-expr.md#r-expr.closure), [async blocks](../expressions/block-expr.md#r-expr.block.async), [free functions](../items/functions.md#r-items.fn), [associated functions](../items/associated-items.md#r-items.associated.fn) in an [inherent impl](../items/implementations.md#r-items.impl.inherent) or [trait impl](../items/implementations.md#r-items.impl.trait), and associated functions in a [trait definition](../items/traits.md#r-items.traits) when those functions have a [default definition](../items/traits.md#r-items.traits.associated-item-decls) .

> ![](../../assets/inline/codegen-25.svg)Note
>
> `rustc` ignores use in other positions but lints against it. This may become an error in the future.

> ![](../../assets/inline/codegen-26.svg)Note
>
> Though the attribute can be applied to [closures](../expressions/closure-expr.md#r-expr.closure) and [async blocks](../expressions/block-expr.md#r-expr.block.async), the usefulness of this is limited as we do not yet support attributes on expressions.

[attributes.codegen.instruction_set.duplicates](#r-attributes.codegen.instruction_set.duplicates)

The `instruction_set` attribute may be used only once on a function.

[attributes.codegen.instruction_set.target-limits](#r-attributes.codegen.instruction_set.target-limits)

The `instruction_set` attribute may only be used with a target that supports the given value.

[attributes.codegen.instruction_set.inline-asm](#r-attributes.codegen.instruction_set.inline-asm)

When the `instruction_set` attribute is used, any inline assembly in the function must use the specified instruction set instead of the target default.

[attributes.codegen.instruction_set.arm](#r-attributes.codegen.instruction_set.arm)

### `instruction_set` on ARM

When targeting the `ARMv4T` and `ARMv5te` architectures, the supported values for `instruction_set` are:

- `arm::a32` — Generate the function as A32 “ARM” code.
- `arm::t32` — Generate the function as T32 “Thumb” code.

If the address of the function is taken as a function pointer, the low bit of the address will depend on the selected instruction set:

- For `arm::a32` (“ARM”), it will be 0.
- For `arm::t32` (“Thumb”), it will be 1.
