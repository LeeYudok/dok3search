# 흐름 제어

Rust에는 다른 언어와는 다른 몇 가지 제어 흐름 구조가 있으며 패턴 일치에 사용됩니다.

- `if let` 표현식
- `while let` expressions
- `match` 표현식

## `if let` 표현식

[`if let` 표현식](https://doc.rust-lang.org/reference/expressions/if-expr.html#if-let-expressions)을 사용하면 값이 패턴과 일치하는지에 따라 다른 코드를 실행할 수 있습니다:

``` rust
fn sleep_for(secs: f32) {
    let dur = if let Ok(dur) = std::time::Duration::try_from_secs_f32(secs) {
        dur
    } else {
        std::time::Duration::from_millis(500)
    };
    std::thread::sleep(dur);
    println!("{:?} 동안 잠들었습니다.", dur);
}

fn main() {
    sleep_for(-10.0);
    sleep_for(0.8);
}
```

## `let else` expressions

패턴을 일치시키고 함수에서 반환하는 일반적인 경우에는 [`let else`](https://doc.rust-lang.org/rust-by-example/flow_control/let_else.html)를 사용합니다. ‘else’ 사례는 해당 코드를 벗어나야 합니다 (`return`, `break` 또는 패닉 - 블록의 다음 위치로 이동하는 것만 아니면 됩니다).

``` rust
fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
    let s = if let Some(s) = maybe_string {
        s
    } else {
        return Err(String::from("None을 가져옴"));
    };

    let first_byte_char = if let Some(first_byte_char) = s.chars().next() {
        first_byte_char
    } else {
        return Err(String::from("got empty string"));
    };

    if let Some(digit) = first_byte_char.to_digit(16) {
        Ok(digit)
    } else {
        Err(String::from("16진수가 아님"))
    }
}

fn main() {
    println!("결과: {:?}", hex_or_die_trying(Some(String::from("foo"))));
}
```

마지막으로, 무한 루프를 만드는 [`loop` 키워드](https://doc.rust-lang.org/reference/expressions/loop-expr.html#infinite-loops)가 있습니다:

``` rust
fn main() {
    let mut name = String::from("Comprehensive Rust 🦀");
    while let Some(c) = name.pop() {
        println!("character: {c}");
    }
    // (There are more efficient ways to reverse a string!)
}
```

Here [`String::pop`](https://doc.rust-lang.org/stable/std/string/struct.String.html#method.pop) returns `Some(c)` until the string is empty, after which it will return `None`. The `while let` lets us keep iterating through all items.

> **발표자 노트**
>
> This slide should take about 10 minutes.
>
> ## if-let
>
> - `if let`이 `match`보다 더 간결할 수 있습니다(예: 한가지 브랜치만 흥미로운 경우). 이와 달리 `match`에서는 모든 브랜치가 처리되어야 합니다.
> - 일반적 사용법은 `Option`을 사용할 때 `Some` 값을 처리하는 것입니다.
> - `match`와 달리 `if let`은 패턴 일치를 위한 보호 절을 지원하지 않습니다.
>
> ## let-else
>
> 위에서 본 것처럼 `if-let`은 중첩할 수 있습니다. `let-else` 구조는 이 중첩된 코드의 평면화를 지원합니다. 코드가 어떻게 변화하는지 학생들이 볼 수 있도록 어색한 버전을 다시 작성하세요.
>
> 다시 작성된 버전은 다음과 같습니다.
>
> ``` rust
> #![allow(unused)]
> fn main() {
> fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
>     let Some(s) = maybe_string else {
>         return Err(String::from("None을 가져옴"));
>     };
>
>     let Some(first_byte_char) = s.chars().next() else {
>         return Err(String::from("got empty string"));
>     };
>
>     let Some(digit) = first_byte_char.to_digit(16) else {
>         return Err(String::from("16진수가 아님"));
>     };
>
>     return Ok(digit);
> }
> }
> ```
>
> ## while-let
>
> - `while let`은 값이 패턴에 매치되는 동안 계속됩니다.
> - You could rewrite the `while let` loop as an infinite loop with an if statement that breaks when there is no value to unwrap for `name.pop()`. The `while let` provides syntactic sugar for the above scenario.
