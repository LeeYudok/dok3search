# 비동기 트레잇

Async methods in traits are were stabilized only recently, in the 1.75 release. This required support for using return-position `impl Trait` (RPIT) in traits, as the desugaring for `async fn` includes `-> impl Future<Output = ...>`.

However, even with the native support today there are some pitfalls around `async fn` and RPIT in traits:

- Return-position impl Trait captures all in-scope lifetimes (so some patterns of borrowing cannot be expressed)

- Traits whose methods use return-position `impl trait` or `async` are not `dyn` compatible.

If we do need `dyn` support, the crate [async_trait](https://docs.rs/async-trait/latest/async_trait/) provides a workaround through a macro, with some caveats:

컴파일 오류 예제

``` rust
use async_trait::async_trait;
use std::time::Instant;
use tokio::time::{sleep, Duration};

#[async_trait]
trait Sleeper {
    async fn sleep(&self);
}

struct FixedSleeper {
    sleep_ms: u64,
}

#[async_trait]
impl Sleeper for FixedSleeper {
    async fn sleep(&self) {
        sleep(Duration::from_millis(self.sleep_ms)).await;
    }
}

async fn run_all_sleepers_multiple_times(
    sleepers: Vec<Box<dyn Sleeper>>,
    n_times: usize,
) {
    for _ in 0..n_times {
        println!("모든 수면자를 실행");
        for sleeper in &sleepers {
            let start = Instant::now();
            sleeper.sleep().await;
            println!("{}밀리초 동안 절전 모드", start.elapsed().as_millis());
        }
    }
}

#[tokio::main]
async fn main() {
    let sleepers: Vec<Box<dyn Sleeper>> = vec![
        Box::new(FixedSleeper { sleep_ms: 50 }),
        Box::new(FixedSleeper { sleep_ms: 100 }),
    ];
    run_all_sleepers_multiple_times(sleepers, 5).await;
}
```

> **발표자 노트**
>
> - `async_trait`은 사용하기 쉽지만 이를 위해 힙에 메모리를 할당한다는 점에 유의하세요. 이 힙 할당에는 성능 오버헤드가 있습니다.
>
> - `async trait` 를 언어 차원에서 지원하는 것과 관련된 문제는 매우 전문적인 토픽이며 따라서 이 강의에서 다룰 내용은 아닙니다. [이 게시물](https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/)에 이에 관한 니코 마사키스의 좋은 설명이 있으므로 관심이 있다면 참고하세요.
>
> - 임의의 시간 동안 sleep 하는 새로운 sleeper 구조체를 만들어 Vec에 추가해 보세요.
