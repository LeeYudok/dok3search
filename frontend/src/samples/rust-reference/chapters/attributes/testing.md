# Testing attributes

[attributes.testing](#r-attributes.testing)

# Testing attributes

[attributes.testing.intro](#r-attributes.testing.intro)

The following [attributes](../attributes.md) are used for specifying functions for performing tests. Compiling a crate in “test” mode enables building the test functions along with a test harness for executing the tests. Enabling the test mode also enables the [`test` conditional compilation option](../conditional-compilation.md#test).

[attributes.testing.test](#r-attributes.testing.test)

## The `test` attribute

[attributes.testing.test.intro](#r-attributes.testing.test.intro)

The *`test` [attribute](../attributes.md)* marks a function to be executed as a test.

> ![](../../assets/inline/testing-1.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> pub fn add(left: u64, right: u64) -> u64 { left + right }
> #[test]
> fn it_works() {
>     let result = add(2, 2);
>     assert_eq!(result, 4);
> }
> }
> ```

[attributes.testing.test.syntax](#r-attributes.testing.test.syntax)

The `test` attribute uses the [MetaWord](../attributes.md#grammar-MetaWord) syntax.

[attributes.testing.test.allowed-positions](#r-attributes.testing.test.allowed-positions)\
Tests Tests with this rule:

- [tests/ui/test-attrs/issue-109816.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/test-attrs/issue-109816.rs)
- [tests/ui/test-attrs/test-attr-non-associated-functions.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/test-attrs/test-attr-non-associated-functions.rs)
- [tests/ui/test-attrs/test-function-signature.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/test-attrs/test-function-signature.rs)
- [tests/ui/test-attrs/test-on-not-fn.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/test-attrs/test-on-not-fn.rs)

The `test` attribute may only be applied to [free functions](../glossary.md#free-item) that are monomorphic, that take no arguments, and where the return type implements the [`Termination`](../../std/process/trait.Termination.html) trait.

> ![](../../assets/inline/testing-2.svg)Note
>
> Some of types that implement the [`Termination`](../../std/process/trait.Termination.html) trait include:
>
> - `()`
> - `Result<T, E> where T: Termination, E: Debug`

[attributes.testing.test.duplicates](#r-attributes.testing.test.duplicates)

Only the first use of `test` on a function has effect.

> ![](../../assets/inline/testing-3.svg)Note
>
> `rustc` lints against any use following the first. This may become an error in the future.

[attributes.testing.test.stdlib](#r-attributes.testing.test.stdlib)

The `test` attribute is exported from the standard library prelude as [`std::prelude::v1::test`](../../core/macros/builtin/attr.test.html).

[attributes.testing.test.enabled](#r-attributes.testing.test.enabled)

These functions are only compiled when in test mode.

> ![](../../assets/inline/testing-4.svg)Note
>
> The test mode is enabled by passing the `--test` argument to `rustc` or using `cargo test`.

[attributes.testing.test.success](#r-attributes.testing.test.success)\
Tests Tests with this rule:

- [tests/ui/test-attrs/test-passed.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/test-attrs/test-passed.rs)

The test harness calls the returned value’s [`report`](../../std/process/trait.Termination.html#tymethod.report) method, and classifies the test as passed or failed depending on whether the resulting [`ExitCode`](../../std/process/struct.ExitCode.html) represents successful termination. In particular:

- Tests that return `()` pass as long as they terminate and do not panic.
- Tests that return a `Result<(), E>` pass as long as they return `Ok(())`.
- Tests that return `ExitCode::SUCCESS` pass, and tests that return `ExitCode::FAILURE` fail.
- Tests that do not terminate neither pass nor fail.

> ![](../../assets/inline/testing-5.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> use std::io;
> fn setup_the_thing() -> io::Result { Ok(1) }
> fn do_the_thing(s: &i32) -> io::Result<()> { Ok(()) }
> #[test]
> fn test_the_thing() -> io::Result<()> {
>     let state = setup_the_thing()?; // expected to succeed
>     do_the_thing(&state)?;          // expected to succeed
>     Ok(())
> }
> }
> ```

[attributes.testing.ignore](#r-attributes.testing.ignore)

## The `ignore` attribute

[attributes.testing.ignore.intro](#r-attributes.testing.ignore.intro)

The *`ignore` [attribute](../attributes.md)* can be used with the [`test` attribute](#r-attributes.testing.test) to tell the test harness to not execute that function as a test.

> ![](../../assets/inline/testing-6.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> #[test]
> #[ignore]
> fn check_thing() {
>     // …
> }
> }
> ```

> ![](../../assets/inline/testing-7.svg)Note
>
> The `rustc` test harness supports the `--include-ignored` flag to force ignored tests to be run.

[attributes.testing.ignore.syntax](#r-attributes.testing.ignore.syntax)

The `ignore` attribute uses the [MetaWord](../attributes.md#grammar-MetaWord) and [MetaNameValueStr](../attributes.md#grammar-MetaNameValueStr) syntaxes.

[attributes.testing.ignore.reason](#r-attributes.testing.ignore.reason)

The [MetaNameValueStr](../attributes.md#grammar-MetaNameValueStr) form of the `ignore` attribute provides a way to specify a reason why the test is ignored.

> ![](../../assets/inline/testing-8.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> #[test]
> #[ignore = "not yet implemented"]
> fn mytest() {
>     // …
> }
> }
> ```

[attributes.testing.ignore.allowed-positions](#r-attributes.testing.ignore.allowed-positions)

The `ignore` attribute may only be applied to functions annotated with the `test` attribute.

> ![](../../assets/inline/testing-9.svg)Note
>
> `rustc` ignores use in other positions but lints against it. This may become an error in the future.

[attributes.testing.ignore.duplicates](#r-attributes.testing.ignore.duplicates)

Only the first use of `ignore` on a function has effect.

> ![](../../assets/inline/testing-10.svg)Note
>
> `rustc` lints against any use following the first. This may become an error in the future.

[attributes.testing.ignore.behavior](#r-attributes.testing.ignore.behavior)

Ignored tests are still compiled when in test mode, but they are not executed.

[attributes.testing.should_panic](#r-attributes.testing.should_panic)

## The `should_panic` attribute

[attributes.testing.should_panic.intro](#r-attributes.testing.should_panic.intro)

The *`should_panic` [attribute](../attributes.md)* causes a test to pass only if the [test function](#r-attributes.testing.test) to which the attribute is applied panics.

> ![](../../assets/inline/testing-11.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> #[test]
> #[should_panic(expected = "values don't match")]
> fn mytest() {
>     assert_eq!(1, 2, "values don't match");
> }
> }
> ```

[attributes.testing.should_panic.syntax](#r-attributes.testing.should_panic.syntax)\
Tests Tests with this rule:

- [tests/ui/test-attrs/test-should-panic-attr.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/test-attrs/test-should-panic-attr.rs)

The `should_panic` attribute has these forms:

- [MetaWord](../attributes.md#grammar-MetaWord)

  > ![](../../assets/inline/testing-12.svg)Example
  >
  > ``` playground
  > #![allow(unused)]
  > fn main() {
  > #[test]
  > #[should_panic]
  > fn mytest() { panic!("error: some message, and more"); }
  > }
  > ```

- [MetaNameValueStr](../attributes.md#grammar-MetaNameValueStr) — The given string must appear within the panic message for the test to pass.

  > ![](../../assets/inline/testing-13.svg)Example
  >
  > ``` playground
  > #![allow(unused)]
  > fn main() {
  > #[test]
  > #[should_panic = "some message"]
  > fn mytest() { panic!("error: some message, and more"); }
  > }
  > ```

- [MetaListNameValueStr](../attributes.md#grammar-MetaListNameValueStr) — As with the [MetaNameValueStr](../attributes.md#grammar-MetaNameValueStr) syntax, the given string must appear within the panic message.

  > ![](../../assets/inline/testing-14.svg)Example
  >
  > ``` playground
  > #![allow(unused)]
  > fn main() {
  > #[test]
  > #[should_panic(expected = "some message")]
  > fn mytest() { panic!("error: some message, and more"); }
  > }
  > ```

[attributes.testing.should_panic.allowed-positions](#r-attributes.testing.should_panic.allowed-positions)

The `should_panic` attribute may only be applied to functions annotated with the `test` attribute.

> ![](../../assets/inline/testing-15.svg)Note
>
> `rustc` ignores use in other positions but lints against it. This may become an error in the future.

[attributes.testing.should_panic.duplicates](#r-attributes.testing.should_panic.duplicates)

Only the first use of `should_panic` on a function has effect.

> ![](../../assets/inline/testing-16.svg)Note
>
> `rustc` lints against any use following the first with a future-compatibility warning. This may become an error in the future.

[attributes.testing.should_panic.expected](#r-attributes.testing.should_panic.expected)\
Tests Tests with this rule:

- [tests/ui/test-attrs/test-should-panic-failed-show-span.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/test-attrs/test-should-panic-failed-show-span.rs)

When the [MetaNameValueStr](../attributes.md#grammar-MetaNameValueStr) form or the [MetaListNameValueStr](../attributes.md#grammar-MetaListNameValueStr) form with the `expected` key is used, the given string must appear somewhere within the panic message for the test to pass.

[attributes.testing.should_panic.return](#r-attributes.testing.should_panic.return)

The return type of the test function must be `()`.
