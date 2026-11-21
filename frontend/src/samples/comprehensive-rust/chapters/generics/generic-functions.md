# 제네릭 함수

Rust supports generics, which lets you abstract algorithms or data structures (such as sorting or a binary tree) over the types used or stored.

``` rust
/// `n` 값에 따라 `even` 또는 `odd`를 선택합니다.
fn pick<T>(n: i32, even: T, odd: T) -> T {
    if n % 2 == 0 {
        even
    } else {
        odd
    }
}

fn main() {
    println!("선택한 숫자: {:?}", pick(97, 222, 333));
    println!("선택한 튜플: {:?}", pick(28, ("개", 1), ("고양이", 2)));
}
```

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> - Rust는 인수 및 반환 값의 타입을 기반으로 T의 타입을 추론합니다.
>
> - 이는 C++ 템플릿과 비슷하지만, Rust는 제네릭 함수를 즉시 부분적으로 컴파일하므로 제약 조건과 일치하는 모든 타입에 함수가 유효해야 합니다. 예를 들어 `n == 0`인 경우 `even + odd`를 반환하도록 `pick`을 수정하세요. 정수가 포함된 `pick` 인스턴스화만 사용되더라도 Rust는 이를 유효하지 않은 것으로 간주합니다. C++에서는 이를 허용합니다.
>
> - Generic code is turned into non-generic code based on the call sites. This is a zero-cost abstraction: you get exactly the same result as if you had hand-coded the data structures without the abstraction.
