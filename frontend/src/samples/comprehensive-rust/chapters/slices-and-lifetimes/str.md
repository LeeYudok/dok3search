# 문자열 레퍼런스

이제 Rust의 두 가지 문자열 타입을 이해할 수 있습니다. `&str`은 거의 `&[char]`와 비슷하지만 데이터가 가변 길이 인코딩(UTF-8)으로 저장됩니다.

``` rust
fn main() {
    let s1: &str = "World";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");

    let s3: &str = &s2[6..];
    println!("s3: {s3}");
}
```

러스트 용어:

- `&str`은 문자열 슬라이스에 대한 (불변) 참조입니다.
- `String`은 문자열을 담을 수 있는 버퍼입니다.

> **발표자 노트**
>
> This slide should take about 10 minutes.
>
> - `&str`은 문자열 슬라이스입니다. 문자열 슬라이스는 UTF-8로 인코딩된 문자열 데이터를 의미합니다. 문자열 리터럴(`"Hello"`)은 프로그램 바이너리에 저장됩니다.
>
> - 러스트의 `String`타입은 실제로는 문자열을 이루는 바이트에 대한 백터(`Vec<u8>`)입니다. `Vec<T>`가 `T`를 소유하고 있듯이, `String`이 가리키고 있는 문자열은 `String`의 소유입니다.
>
> - As with many other types `String::from()` creates a string from a string literal; `String::new()` creates a new empty string, to which string data can be added using the `push()` and `push_str()` methods.
>
> - The `format!()` macro is a convenient way to generate an owned string from dynamic values. It accepts the same format specification as `println!()`.
>
> - You can borrow `&str` slices from `String` via `&` and optionally range selection. If you select a byte range that is not aligned to character boundaries, the expression will panic. The `chars` iterator iterates over characters and is preferred over trying to get character boundaries right.
>
> - For C++ programmers: think of `&str` as `std::string_view` from C++, but the one that always points to a valid string in memory. Rust `String` is a rough equivalent of `std::string` from C++ (main difference: it can only contain UTF-8 encoded bytes and will never use a small-string optimization).
>
> - Byte strings literals allow you to create a `&[u8]` value directly:
>
>   ``` rust
>   fn main() {
>       println!("{:?}", b"abc");
>       println!("{:?}", &[97, 98, 99]);
>   }
>   ```
