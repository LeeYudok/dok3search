# `Rc`

[`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html)는 참조 카운팅 공유 포인터입니다. 여러 위치에서 동일한 데이터를 참조해야할 경우 사용합니다:

``` rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(10);
    let b = Rc::clone(&a);

    println!("a: {a}");
    println!("b: {b}");
}
```

- 멀티스레드 환경에서 작업하는 경우 [`Arc`](../concurrency/shared_state/arc.md)와 [`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html)를 참조하세요.
- drop 가능한 순환 구조를 만들기 위해 공유 포인터를 [`Weak`](https://doc.rust-lang.org/std/rc/struct.Weak.html) 포인터로 \_다운그레이드_할 수도 있습니다.

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> - `Rc`는 참조 카운트를 통해 참조가 있는 동안은 `Rc`가 가리키고 있는 값이 메모리에서 해제되지 않음을 보장합니다.
> - C++의 `std::shared_ptr`와 유사합니다.
> - `clone`은 비용이 거의 들지 않습니다. 같은 곳을 가리키는 포인터를 하나 더 만들고, 참조 카운트를 늘립니다. 포인터가 가리키는 값 자체가 복제(깊은 복제)되지는 않으며, 그래서 코드에서 성능 문제가 있는지 검토할 때 일반적으로 `Rc`를 `clone`하는 것은 무시할 수 있습니다.
> - `make_mut`는 실제로 필요한 경우에 내부 값을 복제하고(“clone-on-write”) 가변 참조를 반환합니다.
> - 참조 카운트를 확인하려면 `Rc::strong_count`를 사용하세요.
> - `Rc::downgrade` gives you a *weakly reference-counted* object to create cycles that will be dropped properly (likely in combination with `RefCell`).
