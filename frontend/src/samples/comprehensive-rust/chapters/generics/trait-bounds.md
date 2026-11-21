# 제네릭 타입 제한(트레잇 경계)

제네릭을 이용하다 보면 타입이 어떤 트레잇을 구현하고 있어야 하는 경우가 있습니다. 그래야 그 트레잇의 메서드를 호출할 수 있기 때문입니다.

`T: Trait` 혹은 `impl Trait`를 사용하면 됩니다:

``` rust
fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}

// struct NotClonable;

fn main() {
    let foo = String::from("foo");
    let pair = duplicate(foo);
    println!("{pair:?}");
}
```

> **발표자 노트**
>
> This slide should take about 8 minutes.
>
> - `NonClonable`을 만들어 `duplicate`에 전달해 보세요.
>
> - 여러 트레잇이 필요한 경우 `+`를 사용하여 트레잇을 결합합니다.
>
> - `where` 문법을 사용할 수도 있습니다. 수강생들도 코드를 읽다가 그 문법을 마주할 수 있습니다.
>
>   ``` rust
>   fn duplicate<T>(a: T) -> (T, T)
>   where
>       T: Clone,
>   {
>       (a.clone(), a.clone())
>   }
>   ```
>
>   - 이를 이용하면 타입 파라메터가 많은 경우 함수 시그니처를 간결하게 정리하는 데 도움이 됩니다.
>   - 좀 더 강력한 추가 기능도 제공합니다.
>     - `:` 왼쪽에 임의의 타입(예를 들어 `Option<T>`)을 사용할 수 있습니다.
>
> - Rust는 아직 특수화를 지원하지 않습니다. 예를 들어 원본 `duplicate`가 있는 경우 특수 `duplicate(a: u32)`를 추가하는 것은 유효하지 않습니다.
