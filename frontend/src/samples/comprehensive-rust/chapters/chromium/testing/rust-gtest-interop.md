# `rust_gtest_interop` Library

The [`rust_gtest_interop`](https://chromium.googlesource.com/chromium/src/+/main/testing/rust_gtest_interop/README.md) library provides a way to:

- Use a Rust function as a `gtest` testcase (using the `#[gtest(...)]` attribute)
- Use `expect_eq!` and similar macros (similar to `assert_eq!` but not panicking and not terminating the test when the assertion fails).

Example:

``` rust
use rust_gtest_interop::prelude::*;

#[gtest(MyRustTestSuite, MyAdditionTest)]
fn test_addition() {
    expect_eq!(2 + 2, 4);
}
```
