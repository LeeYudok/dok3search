# 비동기 채널

여러 크레이트에서 비동기 채널을 지원합니다. 예를 들어 `tokio`에서는 아래와 같이합니다.

컴파일 오류 예제

``` rust
use tokio::sync::mpsc::{self, Receiver};

async fn ping_handler(mut input: Receiver<()>) {
    let mut count: usize = 0;

    while let Some(_) = input.recv().await {
        count += 1;
        println!("지금까지 핑 {count}개를 받았습니다.");
    }

    println!("ping_handler 완료");
}

#[tokio::main]
async fn main() {
    let (sender, receiver) = mpsc::channel(32);
    let ping_handler_task = tokio::spawn(ping_handler(receiver));
    for i in 0..10 {
        sender.send(()).await.expect("핑을 보내지 못했습니다.");
        println!("지금까지 핑 {}개를 전송했습니다.", i + 1);
    }

    drop(sender);
    ping_handler_task.await.expect("핑 핸들러 작업에 문제가 발생했습니다.");
}
```

> **발표자 노트**
>
> - 채널 크기를 `3`으로 변경하고 동작이 어떻게 바뀌는지 확인하세요.
>
> - 비동기 채널을 사용하기 위한 인터페이스는 [오전 과정](../concurrency/channels.md)에서 배운 `sync` 채널과 비슷합니다.
>
> - `std::mem::drop` 호출하는 줄을 삭제해 보세요. 어떤 결과가 나타나나요? 이유가 무엇인가요?
>
> - [Flume](https://docs.rs/flume/latest/flume/) 크레이트에는 `sync`와 `async`, `send`와 `recv`를 모두 구현하는 채널이 있습니다. 이것은 IO와 CPU 처리 작업이 많은 복잡한 애플리케이션을 구현할 때 매우 유용합니다.
>
> - `async` 채널을 사용하는 것이 더 좋은 이유는 이를 다른 `future`와 결합하여 복잡한 제어 흐름을 만들 수 있기 때문입니다.
