# `Vec`

[`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html) 는 힙에 할당된 표준 가변 크기 버퍼입니다:

``` rust
fn main() {
    let mut v1 = Vec::new();
    v1.push(42);
    println!("v1: len = {}, 용량 = {}", v1.len(), v1.capacity());

    let mut v2 = Vec::with_capacity(v1.len() + 1);
    v2.extend(v1.iter());
    v2.push(9999);
    println!("v2: len = {}, 용량 = {}", v2.len(), v2.capacity());

    // 요소가 있는 벡터를 초기화하는 표준 매크로입니다.
    let mut v3 = vec![0, 0, 1, 2, 3, 4];

    // 짝수 요소만 유지합니다.
    v3.retain(|x| x % 2 == 0);
    println!("{v3:?}");

    // 연속 중복 삭제
    v3.dedup();
    println!("{v3:?}");
}
```

`Vec`은 [`Deref<Target = [T]>`](https://doc.rust-lang.org/std/vec/struct.Vec.html#deref-methods-%5BT%5D)를 구현합니다. 이는 `Vec`에서 슬라이스 메서드를 호출 할 수 있다는 의미입니다.

> **발표자 노트**
>
> This slide should take about 10 minutes.
>
> - `Vec` is a type of collection, along with `String` and `HashMap`. The data it contains is stored on the heap. This means the amount of data doesn’t need to be known at compile time. It can grow or shrink at runtime.
> - `Vec<T>`는 제네릭 타입이기도 합니다. 하지만 `T`를 꼭 지정해줄 필요는 없습니다. 이 경우, 러스트 타입 추론이 벡터에 처음 `push`하는 데이터로 `T`를 알 수 있었습니다.
> - `vec![...]`는 `Vec::new()` 대신 쓸 수 있는 표준 매크로로서, 초기 데이터를 추가한 벡터를 생성할 수 있습니다.
> - 벡터는 `[` `]`를 사용하여 인덱스로 접근할 수 있습니다. 하지만 범위를 벗어나면 패닉이 발생합니다. 대신 `get`을 사용하면 `Option`을 반환합니다. `pop` 함수는 마지막 요소를 제거합니다.
> - Slices are covered on day 3. For now, students only need to know that a value of type `Vec` gives access to all of the documented slice methods, too.
