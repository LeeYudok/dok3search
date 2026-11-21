# 열거형 분해(역구조화)

튜플과 마찬가지로 구조체 및 enum도 다음을 일치시켜 디스트럭처링할 수 있습니다.

## 구조체

``` rust
struct Foo {
    x: (u32, u32),
    y: u32,
}

#[rustfmt::skip]
fn main() {
    let foo = Foo { x: (1, 2), y: 3 };
    match foo {
        Foo { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
        Foo { y: 2, x: i }   => println!("y = 2, x = {i:?}"),
        Foo { y, .. }        => println!("y = {y}, 다른 필드는 무시됨"),
    }
}
```

## 열거형

구조체나 열거형 값의 일부를 패턴 매치를 통해 변수에 바인딩할 수 있습니다. 간단한 `enum` 타입을 먼저 살펴보겠습니다:

``` rust
enum Result {
    Ok(i32),
    Err(String),
}

fn divide_in_two(n: i32) -> Result {
    if n % 2 == 0 {
        Result::Ok(n / 2)
    } else {
        Result::Err(format!("{n}을(를) 두 개의 동일한 부분으로 나눌 수 없음"))
    }
}

fn main() {
    let n = 100;
    match divide_in_two(n) {
        Result::Ok(half) => println!("{n}을(를) 둘로 나눈 값은 {half}입니다."),
        Result::Err(msg) => println!("죄송합니다. 오류가 발생했습니다. {msg}"),
    }
}
```

`match`구문에서 `divide_in_two`함수에서 반환되는 `Result` 값을 두 개의 팔(혹은 가지)로 *분해(destructure)* 하였습니다. 첫번째 팔에서 `half`는 `Ok` variant에 담긴 값으로 바인딩됩니다. 두번째 팔에서 `msg`는 오류 메시지 문자열에 바인딩됩니다.

> **발표자 노트**
>
> This slide should take about 8 minutes.
>
> ## 구조체
>
> - `foo`의 리터럴 값을 다른 패턴과 일치하도록 변경합니다.
> - `Foo`에 새 필드를 추가하고 필요에 따라 패턴을 변경합니다.
> - 캡처와 상수 표현식은 구분하기 어려울 수 있습니다. 두 번째 부문의 `2`를 변수로 변경해 보고 작동하지 않는 것을 확인하세요. `const`로 변경하고 다시 작동하는지 확인합니다.
>
> ## 열거형
>
> 키 포인트:
>
> - `if`/`else` 표현식은 열거형을 반환하고, 이 값은 나중에 `match`로 분해됩니다.
> - 열거형에 세번째 variant를 추가하고 코드를 실행하여 오류를 표시해보세요. 코드 어느 부분에 누락이 있는지, 그리고 컴파일러가 어떤 식으로 힌트를 주는지 같이 살펴보세요.
> - The values in the enum variants can only be accessed after being pattern matched.
> - Demonstrate what happens when the search is inexhaustive. Note the advantage the Rust compiler provides by confirming when all cases are handled.
> - Save the result of `divide_in_two` in the `result` variable and `match` it in a loop. That won’t compile because `msg` is consumed when matched. To fix it, match `&result` instead of `result`. That will make `msg` a reference so it won’t be consumed. This [“match ergonomics”](https://rust-lang.github.io/rfcs/2005-match-ergonomics.html) appeared in Rust 2018. If you want to support older Rust, replace `msg` with `ref msg` in the pattern.
