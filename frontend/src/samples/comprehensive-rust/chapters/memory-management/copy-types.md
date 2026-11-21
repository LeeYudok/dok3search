# 복사 타입

이동이 기본 설정이지만, 특정 타입은 복사됩니다:

``` rust
fn main() {
    let x = 42;
    let y = x;
    println!("x: {x}"); // would not be accessible if not Copy
    println!("y: {y}");
}
```

이러한 타입들은 `Copy` 트레잇을 구현합니다.

직접 만든 타입들도 `Copy`트레잇을 구현하여 복사를 할 수 있습니다:

``` rust
#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);

fn main() {
    let p1 = Point(3, 4);
    let p2 = p1;
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}
```

- 할당 후, `p1`와 `p2`는 자신의 데이터를 소유합니다.
- 명시적으로 `p1.clone()`를 사용하여 데이터를 복사할 수 있습니다.

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> 복사(copy)와 복제(clone)는 같지 않습니다:
>
> - 복사는 메모리의 내용을 그대로 한 벌 더 만드는 것을 의미하며, 아무 객체에서나 다 지원하지는 않습니다.
> - 복사는 커스터마이즈 할 수 없습니다. (C++에서 복사 생성자를 통해 복사 동작을 임의로 구현할 수 있는 것과 비교가 됩니다.)
> - 복제는 보다 일반적인 작업이며, `Clone`트레잇을 구현하여 복제시 동작을 커스터마이즈 할 수 있습니다.
> - `Drop` 트레잇을 구현한 타입은 복사되지 않습니다.
>
> 위의 예시에서 다음을 시도해 보시기 바랍니다:
>
> - `Point`구조체에 `String`필드를 추가하세요. 컴파일 되지 않을 것입니다. 왜냐하면 `String`은 `Copy`트레잇을 구현하고 있지 않기 때문입니다.
> - Remove `Copy` from the `derive` attribute. The compiler error is now in the `println!` for `p1`.
> - `p1`을 복제하면 잘 동작함을 확인해 보세요.
