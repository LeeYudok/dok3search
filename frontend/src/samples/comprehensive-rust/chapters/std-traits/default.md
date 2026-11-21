# `Default` 트레잇

[`Default`](https://doc.rust-lang.org/std/default/trait.Default.html) 트레잇은 어떤 타입에 대한 기본값을 제공합니다.

``` rust
#[derive(Debug, Default)]
struct Derived {
    x: u32,
    y: String,
    z: Implemented,
}

#[derive(Debug)]
struct Implemented(String);

impl Default for Implemented {
    fn default() -> Self {
        Self("존 스미스".into())
    }
}

fn main() {
    let default_struct = Derived::default();
    println!("{default_struct:#?}");

    let almost_default_struct =
        Derived { y: "Y 설정됨".into(), ..Derived::default() };
    println!("{almost_default_struct:#?}");

    let nothing: Option<Derived> = None;
    println!("{:#?}", nothing.unwrap_or_default());
}
```

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> - 트레잇을 직접 구현하거나 `#[derive(Default)]`를 붙여서 컴파일러에게 구현을 맡길 수 있습니다.
> - 컴파일러가 제공하는 자동 구현의 경우 모든 필드에 대해 기본 값을 설정한 새 인스턴스를 반환합니다.
>   - 이는 구조체의 각 필드 타입들이 모두 `Default` 트레잇을 구현해야 함을 의미합니다.
> - 러스트 표준 타입들은 대부분 `Default`를 구현하고 있으며, 기본 값은 `0`이나 `""`처럼 예상 가능한 값들입니다.
> - The partial struct initialization works nicely with default.
> - The Rust standard library is aware that types can implement `Default` and provides convenience methods that use it.
> - The `..` syntax is called [struct update syntax](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax).
