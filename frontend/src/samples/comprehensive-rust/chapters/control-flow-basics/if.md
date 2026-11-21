# `if` 표현식

다른 언어의 `if` 문과 똑같이 [`if` 표현식](https://doc.rust-lang.org/reference/expressions/if-expr.html#if-expressions)을 사용합니다:

``` rust
fn main() {
    let x = 10;
    if x == 0 {
        println!("zero!");
    } else if x < 100 {
        println!("큰");
    } else {
        println!("거대한");
    }
}
```

게다가 `if`는 표현식으로 사용할 수도 있습니다. 아래 코드는 위와 동일합니다:

``` rust
fn main() {
    let x = 10;
    let size = if x < 20 { "작은" } else { "대형" };
    println!("숫자 크기: {}", size);
}
```

> **발표자 노트**
>
> This slide should take about 4 minutes.
>
> Because `if` is an expression and must have a particular type, both of its branch blocks must have the same type. Show what happens if you add `;` after `"small"` in the second example.
>
> 표현식에 ’if’가 사용된 경우 다음 문과 구분하기 위해 표현식에 `;`이 있어야 합니다. 컴파일러 오류를 보려면 `println!` 앞의 `;`을 삭제하세요.
