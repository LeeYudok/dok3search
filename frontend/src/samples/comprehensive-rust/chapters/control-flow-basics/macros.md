# 매크로

매크로는 컴파일 중에 Rust 코드로 확장되며 다양한 수의 인수를 사용할 수 있습니다. 끝에 `!`로 구분됩니다. Rust 표준 라이브러리에는 여러 가지 유용한 매크로가 포함되어 있습니다.

- `println!(format, ..)` prints a line to standard output, applying formatting described in [`std::fmt`](https://doc.rust-lang.org/std/fmt/index.html).
- `format!(format, ..)`은 `println!`처럼 작동하지만 결과를 문자열로 반환합니다.
- `dbg!(expression)`은 표현식의 값을 기록하고 반환합니다.
- `todo!()`는 일부 코드를 아직 구현되지 않은 것으로 표시합니다. 실행하면 패닉이 발생합니다.
- `unreachable!()`은 일부 코드를 연결할 수 없다고 표시합니다. 실행하면 패닉이 발생합니다.

``` rust
fn factorial(n: u32) -> u32 {
    let mut product = 1;
    for i in 1..=n {
        product *= dbg!(i);
    }
    product
}

fn fizzbuzz(n: u32) -> u32 {
    todo!()
}

fn main() {
    let n = 4;
    println!("{n}! = {}", factorial(n));
}
```

> **발표자 노트**
>
> This slide should take about 2 minutes.
>
> 이 섹션에서는 이러한 일반적인 편의 기능이 있으며 이를 사용하는 방법을 기억해야 합니다. 매크로로 정의되는 이유와 확장 대상은 특별히 중요하지 않습니다.
>
> 이 과정에서는 매크로 정의를 다루지 않지만 이후 섹션에서는 파생 매크로의 사용에 관해 설명합니다.
