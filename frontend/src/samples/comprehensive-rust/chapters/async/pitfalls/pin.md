# `Pin`

Async blocks and functions return types implementing the `Future` trait. The type returned is the result of a compiler transformation which turns local variables into data stored inside the future.

Some of those variables can hold pointers to other local variables. Because of that, the future should never be moved to a different memory location, as it would invalidate those pointers.

To prevent moving the future type in memory, it can only be polled through a pinned pointer. `Pin` is a wrapper around a reference that disallows all operations that would move the instance it points to into a different memory location.

컴파일 오류 예제

``` rust
use tokio::sync::{mpsc, oneshot};
use tokio::task::spawn;
use tokio::time::{sleep, Duration};

// 작업 항목. 이 경우 지정된 시간 동안 절전 모드이고
// `respond_on` 채널의 메시지로 응답합니다.
#[derive(Debug)]
struct Work {
    input: u32,
    respond_on: oneshot::Sender<u32>,
}

// 큐에서 작업을 수신 대기하고 실행하는 worker입니다.
async fn worker(mut work_queue: mpsc::Receiver<Work>) {
    let mut iterations = 0;
    loop {
        tokio::select! {
            Some(work) = work_queue.recv() => {
                sleep(Duration::from_millis(10)).await; // 작업하는 척합니다.
                work.respond_on
                    .send(work.input * 1000)
                    .expect("응답을 보내지 못했습니다.");
                iterations += 1;
            }
            // TODO: 100밀리초마다 반복 횟수를 보고합니다.
        }
    }
}

// 작업을 요청하고 작업이 완료되기를 기다리는 요청자입니다.
async fn do_work(work_queue: &mpsc::Sender<Work>, input: u32) -> u32 {
    let (tx, rx) = oneshot::channel();
    work_queue
        .send(Work { input, respond_on: tx })
        .await
        .expect("작업 큐에서 전송하지 못했습니다.");
    rx.await.expect("응답 대기 실패")
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(10);
    spawn(worker(rx));
    for i in 0..100 {
        let resp = do_work(&tx, i).await;
        println!("반복 작업 결과 {i}: {resp}");
    }
}
```

> **발표자 노트**
>
> - 위에서 소개한 것은 액터(actor) 패턴의 한 예라고 봐도 무방합니다. 액터는 일반적으로 루프 안에서 `select!`를 호출합니다.
>
> - 이전 강의 몇 개의 내용을 요약한 것이기 때문에 천천히 살펴보세요.
>
>   - `_ = sleep(Duration::from_millis(100)) => { println!(..) }`을 `select!`에 추가해 보세요. 이 작업은 실행되지 않습니다. 왜 그럴까요?
>
>   - 대신, 해당 future가 포함된 `timeout_fut`를 `loop` 외부에 추가해 보세요.
>
>     컴파일 오류 예제
>
>     ``` rust
>     #![allow(unused)]
>     fn main() {
>     let mut timeout_fut = sleep(Duration::from_millis(100));
>     loop {
>         select! {
>             ..,
>             _ = timeout_fut => { println!(..); },
>         }
>     }
>     }
>     ```
>
>   - 여전히 작동하지 않습니다. 컴파일러 오류를 따라 `select!`의 `timeout_fut`에 `&mut`를 추가하여 Move 시멘틱 관련한 문제를 해결하고 `Box::pin`을 사용하세요.
>
>     컴파일 오류 예제
>
>     ``` rust
>     #![allow(unused)]
>     fn main() {
>     let mut timeout_fut = Box::pin(sleep(Duration::from_millis(100)));
>     loop {
>         select! {
>             ..,
>             _ = &mut timeout_fut => { println!(..); },
>         }
>     }
>     }
>     ```
>
>   - 이는 컴파일은 되지만 타임 아웃이 되면 매번 반복할 때 마다 `Poll::Ready`가 됩니다(융합된 future가 도움이 될 수 있음). 타임 아웃 될 때마다 `timeout_fut`를 리셋하도록 수정하세요.
>
> - Box는 힙에 할당합니다. 경우에 따라 `std::pin::pin!`(최근에야 안정화되었으며 이전 코드는 `tokio::pin!`을 사용하는 경우가 많음)도 사용할 수 있지만 이는 재할당된 future에 사용하기가 어렵습니다.
>
> - 또 다른 방법은 `pin`을 아예 사용하지 않고 100ms마다 `oneshot` 채널에 전송할 다른 작업을 생성하는 것입니다.
>
> - Data that contains pointers to itself is called self-referential. Normally, the Rust borrow checker would prevent self-referential data from being moved, as the references cannot outlive the data they point to. However, the code transformation for async blocks and functions is not verified by the borrow checker.
>
> - `Pin` is a wrapper around a reference. An object cannot be moved from its place using a pinned pointer. However, it can still be moved through an unpinned pointer.
>
> - The `poll` method of the `Future` trait uses `Pin<&mut Self>` instead of `&mut Self` to refer to the instance. That’s why it can only be called on a pinned pointer.
