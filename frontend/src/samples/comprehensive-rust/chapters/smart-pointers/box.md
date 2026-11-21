# `Box<T>`

[`Box`](https://doc.rust-lang.org/std/boxed/struct.Box.html)는 힙 데이터에 대한 소유 포인터입니다:

``` rust
fn main() {
    let five = Box::new(5);
    println!("five: {}", *five);
}
```

![5 · Stack · Heap · five](../../assets/smart-pointers/box-diagram-1.svg)

`Box<T>`은 [`Deref<Target = T>`](https://doc.rust-lang.org/std/ops/trait.Deref.html#more-on-deref-coercion)를 구현합니다. 이는 [`Box<T>`에서 `T` 메서드를 직접 호출](https://doc.rust-lang.org/std/ops/trait.Deref.html#more-on-deref-coercion) 할 수 있다는 의미입니다.

재귀 데이터나 동적크기의 데이터 타입은 `Box`타입을 사용해야 합니다:

``` rust
#[derive(Debug)]
enum List<T> {
    /// A non-empty list: first element and the rest of the list.
    Element(T, Box<List<T>>),
    /// An empty list.
    Nil,
}

fn main() {
    let list: List<i32> =
        List::Element(1, Box::new(List::Element(2, Box::new(List::Nil))));
    println!("{list:?}");
}
```

![Stack · Heap · list · Element · 1 · 2 · Nil](../../assets/smart-pointers/box-diagram-2.svg)

> **발표자 노트**
>
> This slide should take about 8 minutes.
>
> - `Box` is like `std::unique_ptr` in C++, except that it’s guaranteed to be not null.
>
> - `Box`는 아래의 경우에 유용합니다:
>
>   - 타입 크기를 컴파일 시점에 알 수 없는 경우.
>   - 아주 큰 데이터의 소유권을 전달하고 싶은 경우. 스택에 있는 큰 데이터를 복사하는 대신 `Box`를 이용하여 데이터는 힙에 저장하고 포인터만 이동하면 됩니다.
>
> - If `Box` was not used and we attempted to embed a `List` directly into the `List`, the compiler would not be able to compute a fixed size for the struct in memory (the `List` would be of infinite size).
>
> - `Box`는 일반 포인터와 크기가 같기 때문에 크기를 계산하는 데 문제가 없습니다. 다만 힙에 위치한 `List`의 다음 요소를 가리킬 뿐입니다.
>
> - Remove the `Box` in the List definition and show the compiler error. We get the message “recursive without indirection”, because for data recursion, we have to use indirection, a `Box` or reference of some kind, instead of storing the value directly.
>
> ## 더 살펴보기
>
> ## 니치(틈새) 최적화(Niche Optimization)
>
> ``` rust
> #[derive(Debug)]
> enum List<T> {
>     Element(T, Box<List<T>>),
>     Nil,
> }
>
> fn main() {
>     let list: List<i32> =
>         List::Element(1, Box::new(List::Element(2, Box::new(List::Nil))));
>     println!("{list:?}");
> }
> ```
>
> `Box`는 비어있을 수 없습니다. 따라서 포인터는 항상 유효하며 `null`이 아닙니다. 이는 컴파일러가 메모리 레이아웃을 최적화 할 수 있게 해줍니다:
>
> ![Stack · Heap · list · Element · 1 · 2](../../assets/smart-pointers/box-diagram-3.svg)
