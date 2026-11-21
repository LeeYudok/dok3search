# 문자열

Rust has two types to represent strings, both of which will be covered in more depth later. Both *always* store UTF-8 encoded strings.

- `String` - a modifiable, owned string.
- `&str` - 읽기 전용 문자열입니다. 문자열 리터럴은 이 타입을 가집니다.

``` rust
fn main() {
    let greeting: &str = "  인사말";
    let planet: &str = "🪐";
    let mut sentence = String::new();
    sentence.push_str(greeting);
    sentence.push_str(", ");
    sentence.push_str(planet);
    println!("마지막 문장: {}", sentence);
    println!("{:?}", &sentence[0..5]);
    //println!("{:?}", &sentence[12..13]);
}
```

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> 이 슬라이드는 문자열을 소개합니다. 여기에 있는 모든 내용은 나중에 자세히 다루겠지만 후속 슬라이드와 연습문제에서 문자열을 사용하는 데는 이것으로 충분합니다.
>
> - 문자열 내에 잘못된 UTF-8 인코딩이 있는 것은 정의되지 않은 동작이며, 이는 안전한 Rust에서는 허용되지 않습니다.
>
> - `String`은 생성자(`::new()`) 및 `s.push_str(..)`과 같은 메서드가 포함된 사용자 정의 타입입니다.
>
> - `&str`의 `&`는 참조임을 나타냅니다. 참조는 나중에 다루므로 지금은 `&str`을 ’읽기 전용 문자열’을 의미하는 단위로 생각하세요.
>
> - 주석 처리된 줄은 바이트 위치별로 문자열의 색인을 생성합니다. `12..13`은 문자 경계에서 끝나지 않으므로 프로그램이 패닉 상태가 됩니다. 오류 메시지에 따라 문자 경계에서 끝나는 범위로 조정합니다.
>
> - Raw strings allow you to create a `&str` value with escapes disabled: `r"\n" == "\\n"`. You can embed double-quotes by using an equal amount of `#` on either side of the quotes:
>
>   ``` rust
>   fn main() {
>       println!(r#"<a href="link.html">link</a>"#);
>       println!("<a href=\"link.html\">link</a>");
>   }
>   ```
>
> - Using `{:?}` is a convenient way to print array/vector/struct of values for debugging purposes, and it’s commonly used in code.
