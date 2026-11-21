# 안전하지 않은 함수 호출

## 안전하지 않은 함수 호출

함수나 메서드가 정의되지 않은 동작으로 빠지지 않게 하기 위해서 만족해야 하는 전제 조건이 있는 경우, 그 함수나 메서드를 `unsafe`로 표시할 수 있습니다:

``` rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    let emojis = "🗻∈🌏";

    // 색인이 올바른 순서이고 문자열 슬라이스의 경계 내에
    // 있으며 UTF-8 시퀀스 경계에 있으므로 안전합니다.
    unsafe {
        println!("이모티콘: {}", emojis.get_unchecked(0..4));
        println!("이모티콘: {}", emojis.get_unchecked(4..7));
        println!("이모티콘: {}", emojis.get_unchecked(7..11));
    }

    println!("문자 수: {}", count_chars(unsafe { emojis.get_unchecked(0..7) }));

    unsafe {
        // Undefined behavior if abs misbehaves.
        println!("C에 따른 절댓값 -3: {}", abs(-3));
    }

    // Not upholding the UTF-8 encoding requirement breaks memory safety!
    // println!("emoji: {}", unsafe { emojis.get_unchecked(0..3) });
    // println!("char count: {}", count_chars(unsafe {
    // emojis.get_unchecked(0..3) }));
}

fn count_chars(s: &str) -> usize {
    s.chars().count()
}
```

## 안전하지 않은 함수 작성하기

여러분이 작성한 함수를 사용할 때 어떤 특별한 조건을 만족해야 한다면, `unsafe`로 마킹할 수 있습니다.

``` rust
/// 지정된 포인터가 가리키는 값을 바꿉니다.
///
/// # Safety
///
/// 포인터는 유효하고 올바르게 정렬되어야 합니다.
unsafe fn swap(a: *mut u8, b: *mut u8) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

fn main() {
    let mut a = 42;
    let mut b = 66;

    // 다음과 같은 이유로 안전합니다.
    unsafe {
        swap(&mut a, &mut b);
    }

    println!("a = {}, b = {}", a, b);
}
```

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> ## 안전하지 않은 함수 호출
>
> `get_unchecked`, like most `_unchecked` functions, is unsafe, because it can create UB if the range is incorrect. `abs` is incorrect for a different reason: it is an external function (FFI). Calling external functions is usually only a problem when those functions do things with pointers which might violate Rust’s memory model, but in general any C function might have undefined behaviour under any arbitrary circumstances.
>
> 위 예제 코드에서 `"C"`는 ABI를 의미합니다. [다른 ABI도 있습니다.](https://doc.rust-lang.org/reference/items/external-blocks.html)
>
> ## 안전하지 않은 함수 작성하기
>
> We wouldn’t actually use pointers for a `swap` function - it can be done safely with references.
>
> Note that unsafe code is allowed within an unsafe function without an `unsafe` block. We can prohibit this with `#[deny(unsafe_op_in_unsafe_fn)]`. Try adding it and see what happens. This will likely change in a future Rust edition.
