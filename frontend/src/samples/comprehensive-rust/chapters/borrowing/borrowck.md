# 대여 검증

Rust’s *borrow checker* puts constraints on the ways you can borrow values. For a given value, at any time:

- You can have one or more shared references to the value, *or*
- You can have exactly one exclusive reference to the value.

컴파일 오류 예제

``` rust
fn main() {
    let mut a: i32 = 10;
    let b: &i32 = &a;

    {
        let c: &mut i32 = &mut a;
        *c = 20;
    }

    println!("a: {a}");
    println!("b: {b}");
}
```

> **발표자 노트**
>
> This slide should take about 10 minutes.
>
> - 충돌하는 참조가 같은 지점에 \_존재_해서는 안 됩니다. 참조가 역참조되는 위치는 중요하지 않습니다.
> - 위 코드 컴파일 되지 않습니다. 왜냐하면 `c`는 `a`를 가변 변수로 빌렸고, 이와 동시에 `b`는 `a`를 불변 변수로 빌렸기 때문입니다.
> - `b`에 대한 `println!` 구분을 `c`가 있는 스코프 앞으로 이동하면 컴파일이 됩니다.
> - 이렇게 바꾸면, 컴파일러는 `c`가 `a`를 가변 변수로 빌리기 전에만 `b`가 사용된다는 것을 확인할 수 있습니다. 빌림 검사기의 이러한 기능을 “non-lexical lifetime” 이라고 합니다.
> - The exclusive reference constraint is quite strong. Rust uses it to ensure that data races do not occur. Rust also *relies* on this constraint to optimize code. For example, a value behind a shared reference can be safely cached in a register for the lifetime of that reference.
> - 빌림 검사기는 구조체의 여러 필드에 대한 배타적 참조를 동시에 가져오는 등 여러 일반적인 패턴을 수용하도록 설계되었습니다. 하지만 제대로 “인식“하지 못해 “빌림 검사기와의 충돌“이 발생하는 경우도 있습니다.
