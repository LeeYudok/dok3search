# 튜플

``` rust
fn main() {
    let t: (i8, bool) = (7, true);
    println!("t.0: {}", t.0);
    println!("t.1: {}", t.1);
}
```

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> - 배열과 마찬가지로 튜플은 고정 길이를 갖습니다.
>
> - 튜플은 서로 다른 타입의 값들을 하나의 복합 타입으로 묶습니다.
>
> - 튜플에 속한 값은 `t.0`, `t.1`과 같이 인덱스로 접근할 수 있습니다.
>
> - The empty tuple `()` is referred to as the “unit type” and signifies absence of a return value, akin to `void` in other languages.
