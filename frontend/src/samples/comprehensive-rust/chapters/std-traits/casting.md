# 캐스팅

Rust에는 *암시적* 타입 변환이 없지만 `as`를 사용한 명시적 변환은 지원됩니다. 이는 일반적으로 C 의미론을 따라 정의됩니다.

``` rust
fn main() {
    let value: i64 = 1000;
    println!("as u16: {}", value as u16);
    println!("as i16: {}", value as i16);
    println!("as u8: {}", value as u8);
}
```

`as`의 결과는 Rust에서 *항상* 정의되며 여러 플랫폼에서 일관됩니다. 이는 기호를 변경하거나 더 작은 타입으로 변환할 때의 직관과 일치하지 않을 수 있습니다. 문서를 확인하고 명확하게 설명해 주세요.

Casting with `as` is a relatively sharp tool that is easy to use incorrectly, and can be a source of subtle bugs as future maintenance work changes the types that are used or the ranges of values in types. Casts are best used only when the intent is to indicate unconditional truncation (e.g. selecting the bottom 32 bits of a `u64` with `as u32`, regardless of what was in the high bits).

For infallible casts (e.g. `u32` to `u64`), prefer using `From` or `Into` over `as` to confirm that the cast is in fact infallible. For fallible casts, `TryFrom` and `TryInto` are available when you want to handle casts that fit differently from those that don’t.

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> 이 슬라이드가 끝난 후 잠시 쉬어가는 것이 좋습니다.
>
> `as` is similar to a C++ static cast. Use of `as` in cases where data might be lost is generally discouraged, or at least deserves an explanatory comment.
>
> 이는 정수를 `usize`로 변환하여 색인으로 사용할 때 일반적입니다.
