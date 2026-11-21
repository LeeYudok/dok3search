# 연산자

연산자 오버로드는 `std::ops`에 있는 다양한 트레잇들을 통해 구현됩니다:

``` rust
#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
    }
}

fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = Point { x: 100, y: 200 };
    println!("{:?} + {:?} = {:?}", p1, p2, p1 + p2);
}
```

> **발표자 노트**
>
> This slide should take about 10 minutes.
>
> 논의점:
>
> - You could implement `Add` for `&Point`. In which situations is that useful?
>   - 답: `Add:add`는 `self`를 소모합니다. 만약 타입 `T`가 `Copy`트레잇을 구현하고 있지 않다면 `&T`에 대해서도 연산자 오버로딩을 고려해야 합니다. 이렇게 하면 호출부에서 불필요한 복사를 피할 수 있습니다.
> - 왜 `Output`이 연관된 타입인가요? 타입 파라메터로 만들 수 있을까요?
>   - Short answer: Function type parameters are controlled by the caller, but associated types (like `Output`) are controlled by the implementer of a trait.
> - `Add`를 이용해서 서로 다른 두 개의 타입을 더할 수도 있습니다. 예를 들어 `impl Add<(i32, i32)> for Point`는 튜플을 `Point`에 더할 수 있게 해 줍니다.
