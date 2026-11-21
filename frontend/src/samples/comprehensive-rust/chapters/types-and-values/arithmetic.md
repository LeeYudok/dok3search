# 연산

``` rust
fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c * a;
}

fn main() {
    println!("결과: {}", interproduct(120, 100, 248));
}
```

> **발표자 노트**
>
> This slide should take about 3 minutes.
>
> `main` 이외의 함수는 이번이 처음이지만 의미는 명확합니다. 세 개의 정수를 사용하고 정수를 반환합니다. 함수는 나중에 더 자세히 다루겠습니다.
>
> 산술연산은 다른 언어와 매우 유사하며 우선순위가 비슷합니다.
>
> 정수 오버플로는 어떻게 되나요? C 및 C++에서 *부호있는* 정수의 오버플로는 실제로 정의되지 않으며, 다른 플랫폼이나 컴파일러에서 다른 작업을 실행할 수 있습니다. Rust에서는 정수 오버플로 시의 동작이 정의되어 있습니다.
>
> Change the `i32`’s to `i16` to see an integer overflow, which panics (checked) in a debug build and wraps in a release build. There are other options, such as overflowing, saturating, and carrying. These are accessed with method syntax, e.g., `(a * b).saturating_add(b * c).saturating_add(c * a)`.
>
> 사실 컴파일러는 상수 표현식의 오버플로를 감지하므로 이 예에서는 별도의 함수가 필요합니다.
