# 문서화

Rust comes with extensive documentation. For example:

- All of the details about [loops](https://doc.rust-lang.org/stable/reference/expressions/loop-expr.html).
- [`u8`](https://doc.rust-lang.org/stable/std/primitive.u8.html)과 같은 기본 타입
- Standard library types like [`Option`](https://doc.rust-lang.org/stable/std/option/enum.Option.html) or [`BinaryHeap`](https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html).

사실 자체 코드를 문서화할 수 있습니다.

``` rust
/// 첫 번째 인수를 두 번째 인수로 나눌 수 있는지 확인합니다.
///
/// 두 번째 인수가 0이면 결과는 false입니다.
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}
```

콘텐츠는 마크다운으로 처리됩니다. 게시된 모든 Rust 라이브러리 크레이트는 [rustdoc](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html) 도구를 사용하여 [`docs.rs`](https://docs.rs)에 자동으로 문서화됩니다. 일반적으로 API의 모든 공개 항목은 이 패턴을 사용하여 문서화됩니다.

항목 내부(예: 모듈 내부)의 항목을 문서화하려면 ’내부 문서 주석’이라고 하는 `//!` 혹은 `/*! .. */`를 사용하세요.

``` rust
//! 이 모듈에는 정수의 분할 가능성과 관련된 기능이 포함되어 있습니다.
```

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> - Show students the generated docs for the `rand` crate at <https://docs.rs/rand>.
