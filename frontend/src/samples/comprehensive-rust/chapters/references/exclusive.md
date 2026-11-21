# 배타적(Exclusive) 참조

변경 가능한 참조라고도 하는 배타적 참조를 사용하면 참조되는 값을 변경할 수 있습니다. 타입은 `&mut T`입니다.

``` rust
fn main() {
    let mut point = (1, 2);
    let x_coord = &mut point.0;
    *x_coord = 20;
    println!("point: {point:?}");
}
```

> **발표자 노트**
>
> This slide should take about 10 minutes.
>
> 키 포인트:
>
> - ’배타적’이란 값에 액세스하는 데 이 참조만 사용할 수 있음을 의미합니다. 동시에 다른 참조(공유 또는 배타적)가 존재할 수 없으며, 배타적 참조가 존재하는 동안에는 참조된 값에 액세스할 수 없습니다. `x_coord`가 활성화되어 있는 동안 `&point.0`을 만들거나 `point.0`을 변경해 보세요.
>
> - Be sure to note the difference between `let mut x_coord: &i32` and `let x_coord: &mut i32`. The first one represents a shared reference which can be bound to different values, while the second represents an exclusive reference to a mutable value.
