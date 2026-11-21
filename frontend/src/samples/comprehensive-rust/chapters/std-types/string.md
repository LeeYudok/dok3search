# String

[`String`](https://doc.rust-lang.org/std/string/struct.String.html)은 힙에 할당되고 가변 길이의 표준 UTF-8 문자열 버퍼입니다:

``` rust
fn main() {
    let mut s1 = String::new();
    s1.push_str("안녕하세요");
    println!("s1: len = {}, 용량 = {}", s1.len(), s1.capacity());

    let mut s2 = String::with_capacity(s1.len() + 1);
    s2.push_str(&s1);
    s2.push('!');
    println!("s2: len = {}, 용량 = {}", s2.len(), s2.capacity());

    let s3 = String::from("🇨🇭");
    println!("s3: len = {}, 문자 수 = {}", s3.len(), s3.chars().count());
}
```

`String`은 [`Deref<Target = str>`](https://doc.rust-lang.org/std/string/struct.String.html#deref-methods-str)을 구현합니다. 이는 , `String` 값에 대해서도 `str`의 모든 메서드를 호출 할 수 있다는 의미 입니다.

> **발표자 노트**
>
> This slide should take about 10 minutes.
>
> - `String::new`는 새로운 빈 문자열을 반환합니다. `String::with_capacity`는 새로 만들 문자열 버퍼에 넣을 데이터 크기를 알고 있는 경우에 사용할 수 있습니다.
> - `String::len`은 `String`의 바이트 크기를 반환합니다. (실제 문자 개수와는 다를 수 있습니다.)
> - `String::chars`는 실제 문자(character)들에 대한 이터레이터를 반환합니다. `char`로 표현되는 문자는 우리가 실제로 인식하고 사용하는 문자와는 다를 수 있습니다. 자소 결합으로 문자를 표현하는 경우가 있기 때문입니다. 이에 대해서는 [Grapheme Cluster](https://docs.rs/unicode-segmentation/latest/unicode_segmentation/struct.Graphemes.html)를 참고하세요.
> - 사람들이 문자열이라고 말할 때에는 `&str`이거나 `String`일 수 있습니다.
> - 어떤 타입이 `Deref<Target = T>`를 구현하고 있으면, 컴파일러는 여러분이 `T`의 메소드들을 호출할 수 있게 도와줍니다.
>   - `Deref` 트레잇에 관해서 아직 다루지 않았으므로 이 시점에서는 이것으로 문서의 사이드바 구조가 대부분 설명됩니다.
>   - `String`은 `Deref<Target = str>`을 구현하고 있기 때문에 `String`에 대해서도 `str` 메소드들을 호출할 수 있습니다.
>   - Write and compare `let s3 = s1.deref();` and `let s3 = &*s1;`.
> - `String`은 바이트 벡터의 래퍼로 구현되어 있습니다. 벡터가 지원하는 여러가지 연산들을 `String`도 지원합니다. 다만 `String`은 몇가지 보장 내용이 더 있습니다.
> - `String`을 인덱스로 접근하는 방법들을 비교해보세요:
>   - `s3.chars().nth(i).unwrap()`를 이용하여 한 문자를 선택하는 경우, `i`값이 범위를 벗어날 때, 벗어나지 않을 때 동작을 설명하세요.
>   - `s3[0..4]`를 이용해서 문자열의 일부를 선택하는데, 슬라이스가 유니코드 문자열 경계에 딱 맞지 않을 경우 어떻게 되는지 설명하세요.
> - Many types can be converted to a string with the [`to_string`](https://doc.rust-lang.org/std/string/trait.ToString.html#tymethod.to_string) method. This trait is automatically implemented for all types that implement `Display`, so anything that can be formatted can also be converted to a string.
