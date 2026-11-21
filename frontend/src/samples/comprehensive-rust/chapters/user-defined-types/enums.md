# 열거형

`enum` 키워드는 몇가지 변형(variant)으로 표현되는 열거형 타입을 생성합니다:

``` rust
#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum PlayerMove {
    Pass,                        // 단순 변형
    Run(Direction),              // 튜플 변형
    Teleport { x: u32, y: u32 }, // 구조체 변형
}

fn main() {
    let m: PlayerMove = PlayerMove::Run(Direction::Left);
    println!("이번 차례: {:?}", m);
}
```

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> 키 포인트:
>
> - Enumerations allow you to collect a set of values under one type.
> - `Direction`은 변형을 가지는 열거형 타입입니다. 여기에는 `Direction::Left`와 `Direction::Right`의 두 값이 포함됩니다.
> - `PlayerMove` is a type with three variants. In addition to the payloads, Rust will store a discriminant so that it knows at runtime which variant is in a `PlayerMove` value.
> - This might be a good time to compare structs and enums:
>   - In both, you can have a simple version without fields (unit struct) or one with different types of fields (variant payloads).
>   - You could even implement the different variants of an enum with separate structs but then they wouldn’t be the same type as they would if they were all defined in an enum.
> - Rust는 판별식을 저장하는 데 최소한의 공간을 사용합니다.
>   - 필요한 경우 필요한 가장 작은 크기의 정수를 저장합니다.
>
>   - 허용된 변형 값이 모든 비트 패턴을 포함하지 않는 경우 잘못된 비트 패턴을 사용하여 판별식을 인코딩합니다(‘틈새 최적화’). 예를 들어 `Option<&u8>`은 정수를 가리키는 포인터나 `None` 변형의 경우 `NULL`을 저장합니다.
>
>   - C와의 연동을 위해 식별자 값을 직접 지정할 수도 있습니다:
>
>     ``` rust
>     #[repr(u32)]
>     enum Bar {
>         A, // 0
>         B = 10000,
>         C, // 10001
>     }
>
>     fn main() {
>         println!("A: {}", Bar::A as u32);
>         println!("B: {}", Bar::B as u32);
>         println!("C: {}", Bar::C as u32);
>     }
>     ```
>
>     `repr` 속성이 없다면 10001이 2 바이트로 표현가능하기 때문에 식별자의 타입 크기는 2 바이트가 됩니다.
>
> ## 더 살펴보기
>
> Rust에는 enum이 더 적은 공간을 차지하도록 하는 데 사용할 수 있는 여러 최적화가 있습니다.
>
> - 널포인터 최적화: [어떤 타입들](https://doc.rust-lang.org/std/option/#representation)에 대해서 러스트는 `size_of::<T>()`가 `size_of::<Option<T>>()`와 같은 것을 보장합니다.
>
>   실제로 널포인터 최적화가 적용된 것을 확인하고 싶다면 아래의 예제코드를 사용하세요. 주의할 점은, 여기에서 보여주는 비트 패턴이 컴파일러가 보장해 주는 것은 아니라는 점입니다. 여기에 의존하는 것은 완전히 unsafe합니다.
>
>   ``` rust
>   use std::mem::transmute;
>
>   macro_rules! dbg_bits {
>       ($e:expr, $bit_type:ty) => {
>           println!("- {}: {:#x}", stringify!($e), transmute::<_, $bit_type>($e));
>       };
>   }
>
>   fn main() {
>       unsafe {
>           println!("bool:");
>           dbg_bits!(false, u8);
>           dbg_bits!(true, u8);
>
>           println!("Option<bool>:");
>           dbg_bits!(None::<bool>, u8);
>           dbg_bits!(Some(false), u8);
>           dbg_bits!(Some(true), u8);
>
>           println!("Option<Option<bool>>:");
>           dbg_bits!(Some(Some(false)), u8);
>           dbg_bits!(Some(Some(true)), u8);
>           dbg_bits!(Some(None::<bool>), u8);
>           dbg_bits!(None::<Option<bool>>, u8);
>
>           println!("Option<&i32>:");
>           dbg_bits!(None::<&i32>, usize);
>           dbg_bits!(Some(&0i32), usize);
>       }
>   }
>   ```
