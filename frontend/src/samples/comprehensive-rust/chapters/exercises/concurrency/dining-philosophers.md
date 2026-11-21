# 식사하는 철학자들

식사하는 철학자 문제는 동시성에 있어서 고전적인 문제입니다:

> 5명의 철학자가 원탁에서 식사를 하고 있습니다. 철학자는 원탁에서 자신의 자리에 앉아있습니다. 포크는 각 접시 사이에 있습니다. 제공되는 요리를 먹기 위해서는 두 개의 포크를 모두 사용해야합니다. 철학자는 생각을 하다가 배가 고프면 자신의 좌,우의 포크를 들어 요리를 먹습니다. 철학자는 요리를 먹은 후에는 포크를 다시 자리에 내려놓습니다. 철학자는 자신의 좌,우에 포크가 있을때만 요리를 먹을 수 있습니다. 따라서 두 개의 포크는 오직 자신의 좌,우 철학자가 생각을 할 때만 사용할 수 있습니다.

You will need a local [Cargo installation](../../cargo/running-locally.md) for this exercise. Copy the code below to a file called `src/main.rs`, fill out the blanks, and test that `cargo run` does not deadlock:

컴파일 오류 예제

``` rust
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;

struct Philosopher {
    name: String,
    // left_fork: ...
    // right_fork: ...
    // thoughts: ...
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("유레카! {}에 새로운 아이디어가 있습니다.", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        // 포크를 드세요...
        println!("{}님이 먹고 있습니다...", &self.name);
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] =
    &["Socrates", "히파티아", "플라톤", "아리스토텔레스", "피타고라스"];

fn main() {
    // 포크 만들기

    // 철학자 만들기

    // 각각 100번 생각하고 먹도록 합니다.

    // 생각을 출력합니다.
}
```

다음과 같은 `Cargo.toml`을 사용할 수 있습니다.

``` toml
[package]
name = "dining-philosophers"
version = "0.1.0"
edition = "2021"
```
