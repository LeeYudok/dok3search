# 배열

``` rust
fn main() {
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {a:?}");
}
```

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> - A value of the array type `[T; N]` holds `N` (a compile-time constant) elements of the same type `T`. Note that the length of the array is *part of its type*, which means that `[u8; 3]` and `[u8; 4]` are considered two different types. Slices, which have a size determined at runtime, are covered later.
>
> - 범위를 벗어난 배열 요소에 액세스해 보세요. 배열 액세스는 런타임에 확인됩니다. Rust는 일반적으로 이러한 확인을 최적화할 수 있으며, 안전하지 않은 Rust를 사용하면 범위 확인을 하지 않을 수도 있습니다.
>
> - 리터럴을 사용하여 배열에 값을 할당할 수 있습니다.
>
> - The `println!` macro asks for the debug implementation with the `?` format parameter: `{}` gives the default output, `{:?}` gives the debug output. Types such as integers and strings implement the default output, but arrays only implement the debug output. This means that we must use debug output here.
>
> - `#`을 추가하면(`{a:#?}`) 좀 더 읽기 쉬운 “이쁜” 형태로 출력이 됩니다.
