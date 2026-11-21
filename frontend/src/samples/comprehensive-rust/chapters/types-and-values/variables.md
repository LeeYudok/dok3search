# 변수

Rust provides type safety via static typing. Variable bindings are made with `let`:

``` rust
fn main() {
    let x: i32 = 10;
    println!("x: {x}");
    // x = 20;
    // println!("x: {x}");
}
```

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> - `x = 20`의 주석 처리를 삭제하여 변수가 기본적으로 불변임을 보여줍니다. `mut` 키워드를 추가하여 변경을 허용합니다.
>
> - 여기서 ’i32’는 변수의 타입입니다. 이는 컴파일 시간에 알려져야 하지만, 타입 추론(나중에 설명)을 사용하면 프로그래머가 이를 생략할 수 있는 경우가 많습니다.
