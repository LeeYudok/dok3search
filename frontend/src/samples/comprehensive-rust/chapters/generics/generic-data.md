# 제네릭 데이터 타입

제네릭을 사용하여 필드의 타입을 추상화 할 수 있습니다:

``` rust
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn coords(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }

    // fn set_x(&mut self, x: T)
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("{integer:?} 및 {float:?}");
    println!("좌표: {:?}", integer.coords());
}
```

> **발표자 노트**
>
> This slide should take about 10 minutes.
>
> - *질문:* `impl<T> Point<T> {}`에서 `T`가 왜 두 번 사용됩니까?
>
>   - 제네릭 타입에 대한 제네릭 구현 이기 때문입니다. 이 두 제네릭은 서로 독립적입니다.
>   - 이는 임의의 모든 `T`에 대해서 이 메소드들이 정의된다는 것을 의미합니다.
>   - It is possible to write `impl Point<u32> { .. }`.
>     - `Point`는 여전히 제네릭이며 `Point<f64>`를 사용할 수도 있지만 이 블록의 메서드는 `Point<u32>`만 쓸 수 있습니다.
>
> - 새 변수 `let p = Point { x: 5, y: 10.0 };`를 선언해 보세요. 다음과 같은 두 개의 타입 변수를 사용하여 다른 타입의 요소를 가진 포인트를 허용하도록 코드를 업데이트합니다. `T`와 `U`
