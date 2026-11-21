# 정적변수(static)와 상수(const)

Static and constant variables are two different ways to create globally-scoped values that cannot be moved or reallocated during the execution of the program.

## 상수(`const`)

상수는 컴파일 할 때 그 값이 정해집니다. 그리고 그 값은 그 상수가 사용되는 모든 부분에서 인라인 됩니다:

``` rust
const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

fn main() {
    let digest = compute_digest("Hello");
    println!("digest: {digest:?}");
}
```

[Rust RFC Book](https://rust-lang.github.io/rfcs/0246-const-vs-static.html)에 따르면 상수는, 그 상수가 사용되는 곳에 인라인 됩니다.

`const` 값을 생성할 때에는 `const`로 마킹된 함수만이 호출 가능하며, 이 함수들은 컴파일 시에 호출이 됩니다. 물론 `const`함수들을 런타임에 호출하는 것도 가능합니다.

## 정적변수(`static`)

정적 변수는 프로그램이 수행되는 동안 유지가 됩니다. 그러므로 다른 변수로 이동(move)되지 않습니다:

``` rust
static BANNER: &str = "Welcome to RustOS 3.14";

fn main() {
    println!("{BANNER}");
}
```

As noted in the [Rust RFC Book](https://rust-lang.github.io/rfcs/0246-const-vs-static.html), these are not inlined upon use and have an actual associated memory location. This is useful for unsafe and embedded code, and the variable lives through the entirety of the program execution. When a globally-scoped value does not have a reason to need object identity, `const` is generally preferred.

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> - 러스트의 `const`는 C++의 `constexpr`과 매우 비슷합니다.
> - 반면에 러스트의 `static`은 C++의 `const`나 가변 정적 변수(mutable global variable)와 훨씬 더 유사합니다.
> - `static`은 객체에 정체성을 부여합니다. 정체정이란 메모리 상에서의 주소, 그리고 내부 상태를 의미합니다.
> - 프로그램 수행시 그 값이 정해지는 상수가 필요한 경우는 드뭅니다. 그러나 그렇다고 해도, 정적 변수를 사용하는 것 보다는 더 유용하고 안전합니다.
>
> ### 속성 비교 테이블:
>
> | 속성 | 정적(static) 변수 | 상수(constant) |
> |----|----|----|
> | 메모리 상에 주소가 있는가 | 예 | 아니오(인라인 됨) |
> | 프로그램이 수행되는 동안 계속 살아 있는가 | 예 | 아니오 |
> | 변경 가능한가 | 예 (그러나 안전하지 않음) | 아니오 |
> | 컴파일시 그 값이 결정되는가 | 예 (컴파일시 초기화 됨) | 예 |
> | 사용되는 곳에 인라인 되는가 | 아니오 | 예 |
>
> ## 더 살펴보기
>
> Because `static` variables are accessible from any thread, they must be `Sync`. Interior mutability is possible through a [`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html), atomic or similar.
>
> Thread-local data can be created with the macro `std::thread_local`.
