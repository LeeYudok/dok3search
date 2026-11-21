# API 수정

API를 확장하여 더 많은 기능을 제공해 봅시다. 클라이언트가 생일 카드에 담길 내용을 지정할 수 있도록 하겠습니다:

``` java
package com.example.birthdayservice;

/** 생일 서비스 인터페이스입니다. */
interface IBirthdayService {
    /** 생일 축하 메시지를 생성합니다. */
    String wishHappyBirthday(String name, int years, in String[] text);
}
```

This results in an updated trait definition for `IBirthdayService`:

``` rust
trait IBirthdayService {
    fn wishHappyBirthday(
        &self,
        name: &str,
        years: i32,
        text: &[String],
    ) -> binder::Result<String>;
}
```

> **발표자 노트**
>
> - Note how the `String[]` in the AIDL definition is translated as a `&[String]` in Rust, i.e. that idiomatic Rust types are used in the generated bindings wherever possible:
>   - `in` array arguments are translated to slices.
>   - `out` and `inout` args are translated to `&mut Vec<T>`.
>   - Return values are translated to returning a `Vec<T>`.
