# GoogleTest

The [GoogleTest](https://docs.rs/googletest/) crate allows for flexible test assertions using *matchers*:

``` rust
use googletest::prelude::*;

#[googletest::test]
fn test_elements_are() {
    let value = vec!["foo", "bar", "baz"];
    expect_that!(value, elements_are!(eq("foo"), lt("xyz"), starts_with("b")));
}
```

마지막 요소를 `'!'`로 변경하면 테스트가 실패하고 오류를 정확히 가리키는 구조화된 오류 메시지가 표시됩니다.

``` text
---- test_elements_are stdout ----
Value of: value
Expected: has elements:
  0. is equal to "foo"
  1. is less than "xyz"
  2. starts with prefix "!"
Actual: ["foo", "bar", "baz"],
  where element #2 is "baz", which does not start with "!"
  at src/testing/googletest.rs:6:5
Error: See failure output above
```

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> - GoogleTest는 Rust 플레이그라운드의 일부가 아니므로 로컬 환경에서 이 예를 실행해야 합니다. `cargo add googletest`를 사용하여 기존 Cargo 프로젝트에 빠르게 추가하세요.
>
> - `use googletest::prelude::*;` 줄은 [일반적으로 사용되는 매크로 및 타입](https://docs.rs/googletest/latest/googletest/prelude/index.html)을 여러 개 가져옵니다.
>
> - 이는 일부일 뿐이며 내장된 매처가 많이 있습니다.
>
> - A particularly nice feature is that mismatches in multi-line strings are shown as a diff:
>
> ``` rust
> #[test]
> fn test_multiline_string_diff() {
>     let haiku = "Memory safete found,\n\
>                  Rust's strong typing guides the way,\n\
>                  Secure code you'll write.";
>     assert_that!(
>         haiku,
>         eq("Memory safety found,\n\
>             Rust's silly humor guides the way,\n\
>             Secure code you'll write.")
>     );
> }
> ```
>
> 색상으로 구분된 diff를 표시합니다(여기에서는 색상이 표시되지 않습니다).
>
> ``` text
>     Value of: haiku
> Expected: is equal to "Memory safety found,\nRust's silly humor guides the way,\nSecure code you'll write."
> Actual: "Memory safety found,\nRust's strong typing guides the way,\nSecure code you'll write.",
>   which isn't equal to "Memory safety found,\nRust's silly humor guides the way,\nSecure code you'll write."
> Difference(-actual / +expected):
>  Memory safety found,
> -Rust's strong typing guides the way,
> +Rust's silly humor guides the way,
>  Secure code you'll write.
>   at src/testing/googletest.rs:17:5
> ```
>
> - 크레이트는 [C++용 GoogleTest](https://google.github.io/googletest/)의 Rust 포트입니다.
