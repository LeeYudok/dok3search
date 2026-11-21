# 프로그램 메모리 검토

프로그램은 두 가지 방법으로 메모리를 할당합니다.

- 스택: 로컬 변수를 위한 연속적인 메모리 영역.

  - 여기 저장되는 값은 컴파일 시 결정되는 고정 크기를 갖습니다.
  - 매우 빠름: 메모리 할당/반환이 단지 스택 포인터의 이동만으로 구현됩니다.
  - 관리가 쉬움: 함수가 호출되면 할당되고, 리턴하면 반환됩니다.
  - 스택에 있는 값들은 매우 높은 메모리 인접성을 가집니다.

- 힙: 함수 호출/리턴과 상관 없이 유지되는 값이 저장되는 곳.

  - 여기 저장되는 값은 프로그램 수행시 그 크기가 결정됩니다.
  - 스택 보다는 느림: 메모리 할당/반환시 해야 할 일이 좀 더 있습니다.
  - 메모리 인접성을 보장하지 않습니다.

## 예제

`String`을 하나 만들게 되면, 스택에는 고정된 크기의 메타 데이터가 생성되고, 힙에는 가변 크기의 데이터, 즉, 실제 문자열, 이 생성됩니다:

``` rust
fn main() {
    let s1 = String::from("안녕하세요");
}
```

![Stack · Heap · s1 · capacity · 5 · ptr · H · e · l · o · len](../../assets/memory-management/review-diagram-1.svg)

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> - 문자열(`String`)은 실제로는 `Vec`입니다. 크기(capacity)와 현재 길이(length) 정보를 가지며, 더 큰 크기가 필요할 경우 힙에서 재 할당을 합니다.
>
> - 힙은 기본적으로 [System Allocator](https://doc.rust-lang.org/std/alloc/struct.System.html)를 통해 할당됩니다. 그리고 [Allocator API](https://doc.rust-lang.org/std/alloc/index.html)를 이용해서 커스텀 메모리 할당자를 만들 수도 있습니다.
>
> ## 더 살펴보기
>
> We can inspect the memory layout with `unsafe` Rust. However, you should point out that this is rightfully unsafe!
>
> ``` rust
> fn main() {
>     let mut s1 = String::from("안녕하세요");
>     s1.push(' ');
>     s1.push_str("world");
>     // 집에서는 하지 마세요. 교육 목적으로만 사용할 수 있습니다.
>     // 문자열은 레이아웃을 보장하지 않으므로
>     // 정의되지 않은 동작이 발생할 수 있습니다.
>     unsafe {
>         let (capacity, ptr, len): (usize, usize, usize) = std::mem::transmute(s1);
>         println!("capacity = {capacity}, ptr = {ptr:#x}, len = {len}");
>     }
> }
> ```
