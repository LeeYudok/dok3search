# Conditional compilation

[cfg](#r-cfg)

# Conditional compilation

[cfg.syntax](#r-cfg.syntax)

**Syntax**\
[ConfigurationPredicate](#railroad-ConfigurationPredicate) →\
      [ConfigurationOption](#grammar-ConfigurationOption)\
    \| [ConfigurationAll](#grammar-ConfigurationAll)\
    \| [ConfigurationAny](#grammar-ConfigurationAny)\
    \| [ConfigurationNot](#grammar-ConfigurationNot)\
    \| true\
    \| false

[ConfigurationOption](#railroad-ConfigurationOption) →\
    [IDENTIFIER](identifiers.md#grammar-IDENTIFIER) ( = ( [STRING_LITERAL](tokens.md#grammar-STRING_LITERAL) \| [RAW_STRING_LITERAL](tokens.md#grammar-RAW_STRING_LITERAL) ) )?

[ConfigurationAll](#railroad-ConfigurationAll) →\
    all ( [ConfigurationPredicateList](#grammar-ConfigurationPredicateList)? )

[ConfigurationAny](#railroad-ConfigurationAny) →\
    any ( [ConfigurationPredicateList](#grammar-ConfigurationPredicateList)? )

[ConfigurationNot](#railroad-ConfigurationNot) →\
    not ( [ConfigurationPredicate](#grammar-ConfigurationPredicate) )

[ConfigurationPredicateList](#railroad-ConfigurationPredicateList) →\
    [ConfigurationPredicate](#grammar-ConfigurationPredicate) ( , [ConfigurationPredicate](#grammar-ConfigurationPredicate) )\* ,?

Show Railroad

![](../assets/inline/conditional-compilation-1.svg) ![](../assets/inline/conditional-compilation-2.svg) ![](../assets/inline/conditional-compilation-3.svg) ![](../assets/inline/conditional-compilation-4.svg) ![](../assets/inline/conditional-compilation-5.svg) ![](../assets/inline/conditional-compilation-6.svg) [cfg.intro](#r-cfg.intro)

*Conditionally compiled source code* is source code that is compiled only under certain conditions.

[cfg.attributes-macro](#r-cfg.attributes-macro)

Source code can be made conditionally compiled using the [`cfg`](#the-cfg-attribute) and [`cfg_attr`](#the-cfg_attr-attribute) [attributes](attributes.md) and the built-in [`cfg!`](#the-cfg-macro) and [`cfg_select!`](#the-cfg_select-macro) [macros](macros.md).

[cfg.conditional](#r-cfg.conditional)

Whether to compile can depend on the target architecture of the compiled crate, arbitrary values passed to the compiler, and other things further described below.

[cfg.predicate](#r-cfg.predicate)

Each form of conditional compilation takes a *configuration predicate* that evaluates to true or false. The predicate is one of the following:

[cfg.predicate-option](#r-cfg.predicate-option)

- A configuration option. The predicate is true if the option is set, and false if it is unset.

[cfg.predicate-all](#r-cfg.predicate-all)

- `all()` with a comma-separated list of configuration predicates. It is true if all of the given predicates are true, or if the list is empty.

[cfg.predicate-any](#r-cfg.predicate-any)

- `any()` with a comma-separated list of configuration predicates. It is true if at least one of the given predicates is true. If there are no predicates, it is false.

[cfg.predicate-not](#r-cfg.predicate-not)

- `not()` with a configuration predicate. It is true if its predicate is false and false if its predicate is true.

[cfg.predicate-literal](#r-cfg.predicate-literal)

- `true` or `false` literals, which are always true or false respectively.

[cfg.option-spec](#r-cfg.option-spec)\
Tests Tests with this rule:

- [tests/ui/cfg/cfg-path-error.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/cfg-path-error.rs)
- [tests/ui/cfg/invalid-cli-cfg-pred.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/invalid-cli-cfg-pred.rs)
- [tests/ui/cfg/invalid-cli-check-cfg-pred.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/invalid-cli-check-cfg-pred.rs)

*Configuration options* are either names or key-value pairs, and are either set or unset.

[cfg.option-name](#r-cfg.option-name)\
Tests Tests with this rule:

- [tests/ui/cfg/invalid-cli-cfg-pred.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/invalid-cli-cfg-pred.rs)
- [tests/ui/cfg/invalid-cli-check-cfg-pred.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/invalid-cli-check-cfg-pred.rs)

Names are written as a single identifier, such as `unix`.

[cfg.option-key-value](#r-cfg.option-key-value)\
Tests Tests with this rule:

- [tests/ui/cfg/invalid-cli-cfg-pred.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/invalid-cli-cfg-pred.rs)
- [tests/ui/cfg/invalid-cli-check-cfg-pred.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/invalid-cli-check-cfg-pred.rs)

Key-value pairs are written as an identifier, `=`, and then a string, such as `target_arch = "x86_64"`.

> ![](../assets/inline/conditional-compilation-7.svg)Note
>
> Whitespace around the `=` is ignored, so `foo="bar"` and `foo = "bar"` are equivalent.

[cfg.option-key-uniqueness](#r-cfg.option-key-uniqueness)

Keys do not need to be unique. For example, both `feature = "std"` and `feature = "serde"` can be set at the same time.

[cfg.options.set](#r-cfg.options.set)

## Set configuration options

[cfg.options.intro](#r-cfg.options.intro)

Which configuration options are set is determined statically during the compilation of the crate.

[cfg.options.target](#r-cfg.options.target)

Some options are *compiler-set* based on data about the compilation.

[cfg.options.other](#r-cfg.options.other)

Other options are *arbitrarily-set* based on input passed to the compiler outside of the code.

[cfg.options.crate](#r-cfg.options.crate)

It is not possible to set a configuration option from within the source code of the crate being compiled.

> ![](../assets/inline/conditional-compilation-8.svg)Note
>
> For `rustc`, arbitrary-set configuration options are set using the [`--cfg`](../rustc/command-line-arguments.html#--cfg-configure-the-compilation-environment) flag. Configuration values for a specified target can be displayed with `rustc --print cfg --target $TARGET`.

> ![](../assets/inline/conditional-compilation-9.svg)Note
>
> Configuration options with the key `feature` are a convention used by [Cargo](../../cargo-book/chapters/reference/features.md) for specifying compile-time options and optional dependencies.

[cfg.target_arch](#r-cfg.target_arch)

### `target_arch`

[cfg.target_arch.def](#r-cfg.target_arch.def)\
Tests Tests with this rule:

- [tests/ui/cfg/conditional-compile-arch.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/conditional-compile-arch.rs)

Key-value option set once with the target’s CPU architecture. The value is similar to the first element of the platform’s target triple, but not identical.

[cfg.target_arch.values](#r-cfg.target_arch.values)\
Tests Tests with this rule:

- [tests/ui/cfg/conditional-compile-arch.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/conditional-compile-arch.rs)

Example values:

- `"x86"`
- `"x86_64"`
- `"mips"`
- `"powerpc"`
- `"powerpc64"`
- `"arm"`
- `"aarch64"`

[cfg.target_feature](#r-cfg.target_feature)

### `target_feature`

[cfg.target_feature.def](#r-cfg.target_feature.def)

Key-value option set for each platform feature available for the current compilation target.

[cfg.target_feature.values](#r-cfg.target_feature.values)

Example values:

- `"avx"`
- `"avx2"`
- `"crt-static"`
- `"rdrand"`
- `"sse"`
- `"sse2"`
- `"sse4.1"`

See the [`target_feature` attribute](attributes/codegen.md#the-target_feature-attribute) for more details on the available features.

[cfg.target_feature.crt_static](#r-cfg.target_feature.crt_static)

An additional feature of `crt-static` is available to the `target_feature` option to indicate that a [static C runtime](linkage.md#static-and-dynamic-c-runtimes) is available.

[cfg.target_os](#r-cfg.target_os)

### `target_os`

[cfg.target_os.def](#r-cfg.target_os.def)

Key-value option set once with the target’s operating system. This value is similar to the second and third element of the platform’s target triple.

[cfg.target_os.values](#r-cfg.target_os.values)

Example values:

- `"windows"`
- `"macos"`
- `"ios"`
- `"linux"`
- `"android"`
- `"freebsd"`
- `"dragonfly"`
- `"openbsd"`
- `"netbsd"`
- `"none"` (typical for embedded targets)

[cfg.target_family](#r-cfg.target_family)

### `target_family`

[cfg.target_family.def](#r-cfg.target_family.def)\
Tests Tests with this rule:

- [tests/ui/cfg/cfg-target-family.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/cfg-target-family.rs)

Key-value option providing a more generic description of a target, such as the family of the operating systems or architectures that the target generally falls into. Any number of `target_family` key-value pairs can be set.

[cfg.target_family.values](#r-cfg.target_family.values)\
Tests Tests with this rule:

- [tests/ui/cfg/cfg-target-family.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/cfg-target-family.rs)

Example values:

- `"unix"`
- `"windows"`
- `"wasm"`
- Both `"unix"` and `"wasm"`

[cfg.target_family.unix](#r-cfg.target_family.unix)\
Tests Tests with this rule:

- [tests/ui/cfg/cfg-family.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/cfg-family.rs)

### `unix` and `windows`

`unix` is set if `target_family = "unix"` is set.

[cfg.target_family.windows](#r-cfg.target_family.windows)\
Tests Tests with this rule:

- [tests/ui/cfg/cfg-family.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/cfg-family.rs)

`windows` is set if `target_family = "windows"` is set.

[cfg.target_env](#r-cfg.target_env)

### `target_env`

[cfg.target_env.def](#r-cfg.target_env.def)

Key-value option set with further disambiguating information about the target platform with information about the ABI or `libc` used. For historical reasons, this value is only defined as not the empty-string when actually needed for disambiguation. Thus, for example, on many GNU platforms, this value will be empty. This value is similar to the fourth element of the platform’s target triple. One difference is that embedded ABIs such as `gnueabihf` will simply define `target_env` as `"gnu"`.

[cfg.target_env.values](#r-cfg.target_env.values)

Example values:

- `""`
- `"gnu"`
- `"msvc"`
- `"musl"`
- `"sgx"`
- `"sim"`
- `"macabi"`

[cfg.target_abi](#r-cfg.target_abi)

### `target_abi`

[cfg.target_abi.def](#r-cfg.target_abi.def)\
Tests Tests with this rule:

- [tests/ui/cfg/cfg-target-abi.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/cfg-target-abi.rs)

Key-value option set to further disambiguate the target with information about the target ABI.

[cfg.target_abi.disambiguation](#r-cfg.target_abi.disambiguation)

For historical reasons, this value is only defined as not the empty-string when actually needed for disambiguation. Thus, for example, on many GNU platforms, this value will be empty.

[cfg.target_abi.values](#r-cfg.target_abi.values)\
Tests Tests with this rule:

- [tests/ui/cfg/cfg-target-abi.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/cfg-target-abi.rs)

Example values:

- `""`
- `"llvm"`
- `"eabihf"`
- `"abi64"`

[cfg.target_endian](#r-cfg.target_endian)

### `target_endian`

Key-value option set once with either a value of “little” or “big” depending on the endianness of the target’s CPU.

[cfg.target_pointer_width](#r-cfg.target_pointer_width)

### `target_pointer_width`

[cfg.target_pointer_width.def](#r-cfg.target_pointer_width.def)

Key-value option set once with the target’s pointer width in bits.

[cfg.target_pointer_width.values](#r-cfg.target_pointer_width.values)

Example values:

- `"16"`
- `"32"`
- `"64"`

[cfg.target_vendor](#r-cfg.target_vendor)

### `target_vendor`

[cfg.target_vendor.def](#r-cfg.target_vendor.def)\
Tests Tests with this rule:

- [tests/ui/cfg/cfg-target-vendor.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/cfg-target-vendor.rs)

Key-value option set once with the vendor of the target.

[cfg.target_vendor.values](#r-cfg.target_vendor.values)\
Tests Tests with this rule:

- [tests/ui/cfg/cfg-target-vendor.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/cfg-target-vendor.rs)

Example values:

- `"apple"`
- `"fortanix"`
- `"pc"`
- `"unknown"`

[cfg.target_has_atomic](#r-cfg.target_has_atomic)

### `target_has_atomic`

[cfg.target_has_atomic.def](#r-cfg.target_has_atomic.def)

Key-value option set for each bit width that the target supports atomic loads, stores, and compare-and-swap operations.

[cfg.target_has_atomic.stdlib](#r-cfg.target_has_atomic.stdlib)

When this cfg is present, all of the stable [`core::sync::atomic`](../core/sync/atomic/index.html) APIs are available for the relevant atomic width.

[cfg.target_has_atomic.values](#r-cfg.target_has_atomic.values)

Possible values:

- `"8"`
- `"16"`
- `"32"`
- `"64"`
- `"128"`
- `"ptr"`

[cfg.target_has_atomic_primitive_alignment](#r-cfg.target_has_atomic_primitive_alignment)

### `target_has_atomic_primitive_alignment`

[cfg.target_has_atomic_primitive_alignment.def](#r-cfg.target_has_atomic_primitive_alignment.def)

Key-value option set for each bit width where the [atomic](../core/sync/atomic/index.html) type has the same alignment as the corresponding integer type.

> ![](../assets/inline/conditional-compilation-10.svg)Note
>
> The alignment is usually the same for a given bit width. However, on some targets such as 32-bit x86, 64-bit atomic types such as [`AtomicI64`](../core/sync/atomic/type.AtomicI64.html) have an alignment of 8 bytes while `i64` is only aligned to 4 bytes. In this situation, `target_has_atomic_primitive_alignment = "64"` is not set.

[cfg.target_has_atomic_primitive_alignment.values](#r-cfg.target_has_atomic_primitive_alignment.values)

Possible values:

- `"8"`
- `"16"`
- `"32"`
- `"64"`
- `"128"`
- `"ptr"`

[cfg.test](#r-cfg.test)\
Tests Tests with this rule:

- [tests/ui/test-attrs/test-vs-cfg-test.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/test-attrs/test-vs-cfg-test.rs)

### `test`

Enabled when compiling the test harness. Done with `rustc` by using the [`--test`](../rustc/command-line-arguments.html#--test-build-a-test-harness) flag. See [Testing](attributes/testing.md) for more on testing support.

[cfg.debug_assertions](#r-cfg.debug_assertions)

### `debug_assertions`

Enabled by default when compiling without optimizations. This can be used to enable extra debugging code in development but not in production. For example, it controls the behavior of the standard library’s [`debug_assert!`](../core/macro.debug_assert.html) macro.

[cfg.proc_macro](#r-cfg.proc_macro)

### `proc_macro`

Set when the crate being compiled is being compiled with the `proc_macro` [crate type](linkage.md).

[cfg.panic](#r-cfg.panic)

### `panic`

[cfg.panic.def](#r-cfg.panic.def)\
Tests Tests with this rule:

- [tests/ui/cfg/cfg-panic-abort.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/cfg-panic-abort.rs)
- [tests/ui/cfg/cfg-panic.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/cfg-panic.rs)

Key-value option set depending on the [panic strategy](panic.md#panic-strategy). Note that more values may be added in the future.

[cfg.panic.values](#r-cfg.panic.values)\
Tests Tests with this rule:

- [tests/ui/cfg/cfg-panic-abort.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/cfg-panic-abort.rs)
- [tests/ui/cfg/cfg-panic.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/cfg-panic.rs)

Example values:

- `"abort"`
- `"unwind"`

## Forms of conditional compilation

[cfg.attr](#r-cfg.attr)

### The `cfg` attribute

[cfg.attr.intro](#r-cfg.attr.intro)

The *`cfg` [attribute](attributes.md)* conditionally includes the form to which it is attached based on a configuration predicate.

> ![](../assets/inline/conditional-compilation-11.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> // The function is only included in the build when compiling for macOS
> #[cfg(target_os = "macos")]
> fn macos_only() {
>   // ...
> }
>
> // This function is only included when either foo or bar is defined
> #[cfg(any(foo, bar))]
> fn needs_foo_or_bar() {
>   // ...
> }
>
> // This function is only included when compiling for a unixish OS with a 32-bit
> // architecture
> #[cfg(all(unix, target_pointer_width = "32"))]
> fn on_32bit_unix() {
>   // ...
> }
>
> // This function is only included when foo is not defined
> #[cfg(not(foo))]
> fn needs_not_foo() {
>   // ...
> }
>
> // This function is only included when the panic strategy is set to unwind
> #[cfg(panic = "unwind")]
> fn when_unwinding() {
>   // ...
> }
> }
> ```

[cfg.attr.syntax](#r-cfg.attr.syntax)\
Tests Tests with this rule:

- [tests/ui/cfg/path-kw-as-cfg-pred.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/path-kw-as-cfg-pred.rs)

The syntax for the `cfg` attribute is:

**Syntax**\
[CfgAttribute](#railroad-CfgAttribute) → cfg ( [ConfigurationPredicate](#grammar-ConfigurationPredicate) )

Show Railroad

![](../assets/inline/conditional-compilation-12.svg) [cfg.attr.allowed-positions](#r-cfg.attr.allowed-positions)\
Tests Tests with this rule:

- [tests/ui/cfg/cfg_stmt_expr.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/cfg_stmt_expr.rs)
- [tests/ui/cfg/conditional-compile.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/conditional-compile.rs)

The `cfg` attribute may be used anywhere attributes are allowed.

[cfg.attr.duplicates](#r-cfg.attr.duplicates)\
Tests Tests with this rule:

- [tests/ui/cfg/both-true-false.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/both-true-false.rs)

The `cfg` attribute may be used any number of times on a form. The form to which the attributes are attached will not be included if any of the `cfg` predicates are false except as described in [cfg.attr.crate-level-attrs](#r-cfg.attr.crate-level-attrs).

[cfg.attr.effect](#r-cfg.attr.effect)\
Tests Tests with this rule:

- [tests/ui/cfg/cfg-false-use-item.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/cfg-false-use-item.rs)

If the predicates are true, the form is rewritten to not have the `cfg` attributes on it. If any predicate is false, the form is removed from the source code.

[cfg.attr.crate-level-attrs](#r-cfg.attr.crate-level-attrs)\
Tests Tests with this rule:

- [tests/ui/cfg/cfg_false_no_std-1.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/cfg_false_no_std-1.rs)
- [tests/ui/cfg/cfg_false_no_std-2.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/cfg_false_no_std-2.rs)
- [tests/ui/cfg/cfg_false_no_std.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/cfg_false_no_std.rs)

When a crate-level `cfg` has a false predicate, the crate itself still exists. Any crate attributes preceding the `cfg` are kept, and any crate attributes following the `cfg` are removed as well as removing all of the following crate contents.

> ![](../assets/inline/conditional-compilation-13.svg)Example
>
> The behavior of not removing the preceding attributes allows you to do things such as include `#![no_std]` to avoid linking `std` even if a `#![cfg(...)]` has otherwise removed the contents of the crate. For example:
>
> ``` rust
> // This `no_std` attribute is kept even though the crate-level `cfg`
> // attribute is false.
> #![no_std]
> #![cfg(false)]
>
> // This function is not included.
> pub fn example() {}
> ```

[cfg.cfg_attr](#r-cfg.cfg_attr)

### The `cfg_attr` attribute

[cfg.cfg_attr.intro](#r-cfg.cfg_attr.intro)\
Tests Tests with this rule:

- [tests/ui/cfg/cfg_attr.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/cfg_attr.rs)

The *`cfg_attr` [attribute](attributes.md)* conditionally includes attributes based on a configuration predicate.

> ![](../assets/inline/conditional-compilation-14.svg)Example
>
> The following module will either be found at `linux.rs` or `windows.rs` based on the target.
>
> ``` rust
> #[cfg_attr(target_os = "linux", path = "linux.rs")]
> #[cfg_attr(windows, path = "windows.rs")]
> mod os;
> ```

[cfg.cfg_attr.syntax](#r-cfg.cfg_attr.syntax)\
Tests Tests with this rule:

- [tests/ui/cfg/cfg_attr.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/cfg_attr.rs)
- [tests/ui/cfg/path-kw-as-cfg-pred.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/path-kw-as-cfg-pred.rs)

The syntax for the `cfg_attr` attribute is:

**Syntax**\
[CfgAttrAttribute](#railroad-CfgAttrAttribute) → cfg_attr ( [ConfigurationPredicate](#grammar-ConfigurationPredicate) , [CfgAttrs](#grammar-CfgAttrs)? )

[CfgAttrs](#railroad-CfgAttrs) → [Attr](attributes.md#grammar-Attr) ( , [Attr](attributes.md#grammar-Attr) )\* ,?

Show Railroad

![](../assets/inline/conditional-compilation-15.svg) ![](../assets/inline/conditional-compilation-16.svg) [cfg.cfg_attr.allowed-positions](#r-cfg.cfg_attr.allowed-positions)

The `cfg_attr` attribute may be used anywhere attributes are allowed.

[cfg.cfg_attr.duplicates](#r-cfg.cfg_attr.duplicates)

The `cfg_attr` attribute may be used any number of times on a form.

[cfg.cfg_attr.attr-restriction](#r-cfg.cfg_attr.attr-restriction)\
Tests Tests with this rule:

- [tests/ui/cfg/crate-attributes-using-cfg_attr.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/cfg/crate-attributes-using-cfg_attr.rs)

The [`crate_type`](linkage.md) and [`crate_name`](crates-and-source-files.md#the-crate_name-attribute) attributes cannot be used with `cfg_attr`.

[cfg.cfg_attr.behavior](#r-cfg.cfg_attr.behavior)

When the configuration predicate is true, `cfg_attr` expands out to the attributes listed after the predicate.

[cfg.cfg_attr.attribute-list](#r-cfg.cfg_attr.attribute-list)

Zero, one, or more attributes may be listed. Multiple attributes will each be expanded into separate attributes.

> ![](../assets/inline/conditional-compilation-17.svg)Example
>
> ``` rust
> #[cfg_attr(feature = "magic", sparkles, crackles)]
> fn bewitched() {}
>
> // When the `magic` feature flag is enabled, the above will expand to:
> #[sparkles]
> #[crackles]
> fn bewitched() {}
> ```

> ![](../assets/inline/conditional-compilation-18.svg)Note
>
> The `cfg_attr` can expand to another `cfg_attr`. For example, `#[cfg_attr(target_os = "linux", cfg_attr(feature = "multithreaded", some_other_attribute))]` is valid. This example would be equivalent to `#[cfg_attr(all(target_os = "linux", feature = "multithreaded"), some_other_attribute)]`.

[cfg.macro](#r-cfg.macro)

### The `cfg` macro

The built-in `cfg` macro takes in a single configuration predicate and evaluates to the `true` literal when the predicate is true and the `false` literal when it is false.

For example:

``` playground
#![allow(unused)]
fn main() {
let machine_kind = if cfg!(unix) {
  "unix"
} else if cfg!(windows) {
  "windows"
} else {
  "unknown"
};

println!("I'm running on a {} machine!", machine_kind);
}
```

[cfg.cfg_select](#r-cfg.cfg_select)

### The `cfg_select` macro

[cfg.cfg_select.intro](#r-cfg.cfg_select.intro)

The built-in [`cfg_select!`](../core/macros/macro.cfg_select.html) macro can be used to select code at compile-time based on multiple configuration predicates.

> ![](../assets/inline/conditional-compilation-19.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> cfg_select! {
>     unix => {
>         fn foo() { /* unix specific functionality */ }
>     }
>     target_pointer_width = "32" => {
>         fn foo() { /* non-unix, 32-bit functionality */ }
>     }
>     _ => {
>         fn foo() { /* fallback implementation */ }
>     }
> }
>
> let is_unix_str = cfg_select! {
>     unix => "unix",
>     _ => "not unix",
> };
> }
> ```

[cfg.cfg_select.syntax](#r-cfg.cfg_select.syntax)

**Syntax**\
[CfgSelect](#railroad-CfgSelect) → [CfgSelectArms](#grammar-CfgSelectArms)?

[CfgSelectArms](#railroad-CfgSelectArms) →\
    [CfgSelectConfigurationPredicate](#grammar-CfgSelectConfigurationPredicate) =\>\
    (\
        { ^ [TokenTree](macros.md#grammar-TokenTree) } ,? [CfgSelectArms](#grammar-CfgSelectArms)?\
      \| [ExpressionWithBlockNoAttrs](expressions.md#grammar-ExpressionWithBlockNoAttrs) ,? [CfgSelectArms](#grammar-CfgSelectArms)?\
      \| [ExpressionWithoutBlockNoAttrs](expressions.md#grammar-ExpressionWithoutBlockNoAttrs) ( , [CfgSelectArms](#grammar-CfgSelectArms)? )?\
    )

[CfgSelectConfigurationPredicate](#railroad-CfgSelectConfigurationPredicate) →\
    [ConfigurationPredicate](#grammar-ConfigurationPredicate) \| \_

Show Railroad

![](../assets/inline/conditional-compilation-20.svg) ![](../assets/inline/conditional-compilation-21.svg) ![](../assets/inline/conditional-compilation-22.svg) [cfg.cfg_select.first-arm](#r-cfg.cfg_select.first-arm)

`cfg_select` expands to the payload of the first arm whose configuration predicate evaluates to true.

[cfg.cfg_select.braces](#r-cfg.cfg_select.braces)

If the entire payload is wrapped in curly braces, the braces are removed during expansion.

[cfg.cfg_select.wildcard](#r-cfg.cfg_select.wildcard)

The configuration predicate `_` always evaluates to true.

[cfg.cfg_select.fallthrough](#r-cfg.cfg_select.fallthrough)

It is a compile error if none of the predicates evaluate to true.

[cfg.cfg_select.well-formed](#r-cfg.cfg_select.well-formed)

Each right-hand side must be a syntactically valid expansion for the position in which the macro is invoked.
