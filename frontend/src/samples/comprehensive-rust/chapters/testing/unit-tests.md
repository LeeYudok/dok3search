# 단위 테스트

러스트와 카고(cargo)는 간단한 단위 테스트 프레임워크와 함께 제공됩니다:

- 단위 테스트는 코드 전반에서 지원됩니다.

- 통합 테스트는 `tests/` 디렉터리를 통해 지원됩니다.

Tests are marked with `#[test]`. Unit tests are often put in a nested `tests` module, using `#[cfg(test)]` to conditionally compile them only when building tests.

``` rust
fn first_word(text: &str) -> &str {
    match text.find(' ') {
        Some(idx) => &text[..idx],
        None => &text,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn test_single_word() {
        assert_eq!(first_word("안녕하세요"), "안녕하세요");
    }

    #[test]
    fn test_multiple_words() {
        assert_eq!(first_word("Hello World"), "안녕하세요");
    }
}
```

- 이렇게 서브 모듈로 테스트를 만들면 private한 헬퍼 함수에 대한 단위 테스트도 가능합니다.
- `#[cfg(test)]` 어트리뷰트가 추가된 항목은 `cargo test`를 수행했을 경우에만 동작합니다.

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> 플레이그라운드에서 테스트를 실행하여 결과를 표시합니다.
