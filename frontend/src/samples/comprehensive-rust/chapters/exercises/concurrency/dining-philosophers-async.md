# Dining Philosophers — Async

문제에 관한 설명은 [식사하는 철학자](dining-philosophers.md)를 참고하세요.

As before, you will need a local [Cargo installation](../../cargo/running-locally.md) for this exercise. Copy the code below to a file called `src/main.rs`, fill out the blanks, and test that `cargo run` does not deadlock:

컴파일 오류 예제

``` rust
use std::sync::Arc;
use tokio::sync::mpsc::{self, Sender};
use tokio::sync::Mutex;
use tokio::time;

struct Fork;

struct Philosopher {
    name: String,
    // left_fork: ...
    // right_fork: ...
    // thoughts: ...
}

impl Philosopher {
    async fn think(&self) {
        self.thoughts
            .send(format!("유레카! {}에 새로운 아이디어가 있습니다.", &self.name))
            .await
            .unwrap();
    }

    async fn eat(&self) {
        // Keep trying until we have both forks
        println!("{}님이 먹고 있습니다...", &self.name);
        time::sleep(time::Duration::from_millis(5)).await;
    }
}

static PHILOSOPHERS: &[&str] =
    &["Socrates", "히파티아", "플라톤", "아리스토텔레스", "피타고라스"];

#[tokio::main]
async fn main() {
    // 포크 만들기

    // 철학자 만들기

    // 생각하고 먹게 합니다.

    // 생각을 출력합니다.
}
```

이번에는 Async Rust를 사용하므로 `tokio` 종속 항목이 필요합니다. 다음 `Cargo.toml`을 사용할 수 있습니다.

``` toml
[package]
name = "dining-philosophers-async-dine"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.26.0", features = ["sync", "time", "macros", "rt-multi-thread"] }
```

또한 이번에는 `tokio` 크레이트의 `Mutex`와 `mpsc` 모듈을 사용해야 합니다.

> **발표자 노트**
>
> - Can you make your implementation single-threaded?
