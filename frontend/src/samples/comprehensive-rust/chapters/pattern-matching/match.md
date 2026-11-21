# Matching Values

The `match` keyword lets you match a value against one or more *patterns*. The comparisons are done from top to bottom and the first match wins.

C/C++의 `switch`와 비슷하게 값을 패턴으로 사용할 수도 있습니다:

``` rust
#[rustfmt::skip]
fn main() {
    let input = 'x';
    match input {
        'q'                       => println!("Quitting"),
        'a' | 's' | 'w' | 'd'     => println!("이리저리 이동"),
        '0'..='9'                 => println!("숫자 입력"),
        key if key.is_lowercase() => println!("소문자: {key}"),
        _                         => println!("기타"),
    }
}
```

The `_` pattern is a wildcard pattern which matches any value. The expressions *must* be exhaustive, meaning that it covers every possibility, so `_` is often used as the final catch-all case.

Match can be used as an expression. Just like `if`, each match arm must have the same type. The type is the last expression of the block, if any. In the example above, the type is `()`.

패턴의 변수(이 예에서는 `key`)는 일치 부문 내에서 사용할 수 있는 바인딩을 만듭니다.

일치 가드는 조건이 참인 경우에만 부분이 일치하도록 합니다.

> **발표자 노트**
>
> This slide should take about 10 minutes.
>
> 키 포인트:
>
> - 패턴에서 사용되는 특수 문자들을 알려주세요
>
>   - `|`: or 기호입니다
>   - `..`: 필요한 만큼 확장합니다
>   - `1..=5`: 끝 값(여기서는 5)을 포함하는 범위를 나타냅니다
>   - `_`: 와일드카드입니다
>
> - 매치 가드는 별도의 문법 요소로서 패턴 자체만으로 표현하기 어려운 복잡한 경우를 간결하게 표현하고자 할 때 유용합니다.
>
> - 매치의 각 팔(혹은 가지) 안에 따로 `if`를 사용한 것과 다릅니다. 매치 가지의 `=>` 뒤에 사용된 `if` 표현식은 해당 가지가 선택된 다음에 실행됩니다. 따라서 여기서 `if` 조건이 실패하더라도 원래 `match`의 다른 가지는 고려되지 않습니다.
>
> - 가드에 정의된 조건은 `|` 를 포함하는 패턴의 모든 표현식에 적용됩니다.
