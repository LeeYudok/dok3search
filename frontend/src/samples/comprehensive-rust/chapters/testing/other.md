# 다른 테스트 프로젝트 타입

## 통합 테스트

라이브러리를 사용자 입장에서 테스트 하려면, 통합 테스트를 해야 합니다.

`test/`디렉터리 아래에 `.rs`파일을 하나 만드세요:

``` rust
// tests/my_library.rs
use my_library::init;

#[test]
fn test_init() {
    assert!(init().is_ok());
}
```

이 테스트는 크레이트의 공개 API에만 접근할 수 있습니다.

## 문서화주석 테스트

러스트는 문서화주석에 대한 테스트를 내장하여 제공합니다:

``` rust
#![allow(unused)]
fn main() {
/// Shortens a string to the given length.
///
/// ```
/// # use playground::shorten_string;
/// assert_eq!(shorten_string("Hello World", 5), "Hello");
/// assert_eq!(shorten_string("Hello World", 20), "Hello World");
/// ```
pub fn shorten_string(s: &str, length: usize) -> &str {
    &s[..std::cmp::min(length, s.len())]
}
}
```

- `///` 주석안의 코드 블록은 자동으로 러스트 코드로 인식됩니다.
- 이 코드 블록은 `cargo test` 호출하면 자동으로 컴파일되고 실행됩니다.
- Adding `#` in the code will hide it from the docs, but will still compile/run it.
- 위 코드를 [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=3ce2ad13ea1302f6572cb15cd96becf0)에서 테스트 해 보시기 바랍니다.
