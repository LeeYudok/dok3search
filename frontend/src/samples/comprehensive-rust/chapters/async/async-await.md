# `async`/`await`

겉에서 보았을 때, 비동기 Rust 코드는 일반적인 절차적 코드와 매우 유사합니다.

컴파일 오류 예제

``` rust
use futures::executor::block_on;

async fn count_to(count: i32) {
    for i in 1..=count {
        println!("수: {i}개!");
    }
}

async fn async_main(count: i32) {
    count_to(count).await;
}

fn main() {
    block_on(async_main(10));
}
```

> **발표자 노트**
>
> 키 포인트:
>
> - Rust 비동기 문법을 보여주는 간단한 예시입니다. 여기에는 오래 실행되는 작업이나, 실제로 동시에 수행되는 것들은 없습니다.
>
> - `async`함수의 리턴 타입은 무엇인가요?
>
>   - `main`에서 \`let future: () = async_main(10);을 사용하여 타입을 확인하세요.
>
> - The “async” keyword is syntactic sugar. The compiler replaces the return type with a future.
>
> - `main`을 비동기 함수로 만들수는 없습니다. 만약 그렇게 할 경우 컴파일러는 리턴 타입인 future를 어떻게 사용할 지 모르기 때문입니다.
>
> - You need an executor to run async code. `block_on` blocks the current thread until the provided future has run to completion.
>
> - `.await`는 다른 작업이 완료될 때까지 비동기적으로 대기합니다. `block_on`과 달리 `.await`는 현재 스레드를 블록하지 않습니다.
>
> - `.await` can only be used inside an `async` function (or block; these are introduced later).
