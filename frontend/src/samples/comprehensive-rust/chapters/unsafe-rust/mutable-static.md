# 정적 가변 변수

불변 정적변수를 읽는 것은 안전합니다:

``` rust
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("HELLO_WORLD: {HELLO_WORLD}");
}
```

하지만, 데이터 레이스가 발생할 수 있으므로 정적 가변변수를 읽고 쓰는 것은 안전하지 않습니다:

``` rust
static mut COUNTER: u32 = 0;

fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_counter(42);

    unsafe {
        println!("COUNTER: {COUNTER}");
    }
}
```

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> - 이 프로그램은 단일 스레드이므로 안전합니다. 그러나 Rust 컴파일러는 보수적이며 최악의 상황을 가정합니다. `unsafe`를 삭제해 보고 컴파일러가 여러 스레드에서 static을 변경하는 것이 정의되지 않은 동작이라고 어떻게 설명하는지 확인하세요.
>
> - 일반적으로 이야기 해서, 정적 가변 변수를 쓰는 것은 좋은 아이디어가 아닙니다. 그러나 `no_std`와 같은 저수준 코딩을 할 경우에는 필요하기도 합니다. 예를 들어 힙 할당기를 구현하거나, C API를 사용하는 게 그런 경우입니다.
