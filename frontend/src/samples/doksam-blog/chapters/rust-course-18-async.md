# Rust 강의 18편 — async/await: Future, tokio, Pin

> async 함수는 스레드와 무엇이 다르고, 언제 써야 할까?

답부터 적습니다. `async fn` 은 호출해도 바로 돌지 않고 `Future` 라는 상태 기계를 돌려줍니다. 실행기(tokio)가 그 `Future` 를 `poll` 하고, "아직(Pending)"이면 다른 일로 넘어갔다가 깨우면 다시 `poll` 합니다. 기다리는 동안 스레드를 놓으니 기다림이 많은 일(네트워크, 타이머)을 적은 스레드로 많이 돌립니다. 대신 `Future` 안에서 스레드를 붙잡는 호출(블로킹)을 하면 그 실행기의 다른 일이 모두 멈춥니다. 실측에서 200ms 기다림 셋이 `join!` 으로 202ms, 블로킹 호출이 섞이자 614ms 였습니다.

> [그림] Future 상자 넷 가운데 셋은 Pending·Ready·Pending 이고 넷째는 thread::sleep 닻에 묶여 있다. 실행기 화살표가 넷째에서 멈추고 검수 로봇이 그 상자를 본다. 아래 띠에 차례로 await 604ms, join 202ms, 블로킹 3개 614ms, spawn_blocking 207ms 가 적혀 있다

닻에 묶인 상자가 블로킹 호출입니다. 실행기는 그 상자에서 다음으로 넘어가지 못합니다.

Rust 강의 18편입니다. [17편](https://blog.doksam.com/blog/rust-course-17-concurrency/)은 스레드로 일을 나눴습니다. 이번에는 tokio 1.54.1(`features = ["full"]`)을 받아 async 를 돌립니다. 측정은 2026-11-18, Apple M4 Pro, rustc 1.100.0 릴리스 빌드에서 한 번씩 했습니다.

## Future 는 poll 될 때만 한 걸음 나아갑니다

`Future` 트레이트의 핵심은 메서드 하나입니다. 직접 구현해 몇 번 불리는지 셌습니다.

```rust
struct Countdown {
    left: u32,
    polls: u32,
}

impl Future for Countdown {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32> {
        self.polls += 1;
        if self.left == 0 {
            return Poll::Ready(self.polls);
        }
        self.left -= 1;
        cx.waker().wake_by_ref(); // 곧 다시 poll 해 달라
        Poll::Pending
    }
}
```

`Countdown { left: 2, polls: 0 }.await` 는 `poll 횟수 3` 을 돌려줬습니다. 두 번 `Pending` 을 내고 세 번째에 `Ready` 였습니다. `Pending` 을 낼 때는 반드시 깨울 방법(`Waker`)을 남겨야 합니다. 깨우지 않으면 실행기는 다시 `poll` 하지 않고 그 `Future` 는 영영 멈춥니다. 실제 타이머나 소켓은 "준비되면 깨워 달라"고 운영체제에 등록하고 `Pending` 을 냅니다.

같은 이유로 `async fn` 을 부르기만 하고 `.await` 하지 않으면 아무 일도 일어나지 않습니다. 컴파일러가 경고합니다.

```console
warning: unused implementer of `Future` that must be used
 --> src/main.rs:7:5
  |
7 |     save_report();
  |     ^^^^^^^^^^^^^
  |
  = note: futures do nothing unless you `.await` or poll them
```

실행하면 `저장했다` 는 찍히지 않고 `끝` 만 찍혔습니다. 8편의 게으른 이터레이터와 같은 구조입니다.

## 기다림을 겹치려면 join 합니다

200ms 를 기다리는 `fetch` 세 번입니다.

```rust
    let a = fetch(200).await;
    let b = fetch(200).await;
    let c = fetch(200).await;
    // 차례로: 604ms

    let (a, b, c) = tokio::join!(fetch(200), fetch(200), fetch(200));
    // join!: 202ms
```

`.await` 는 "끝날 때까지 이 자리에서 기다린다"는 뜻이라 차례로 쓰면 시간이 더해집니다. `join!` 은 셋을 한꺼번에 `poll` 해 기다림을 겹칩니다. 스레드는 하나(`current_thread` 런타임)였습니다.

## 블로킹 호출은 실행기 전체를 멈춥니다

이번에는 작업 안에서 `std::thread::sleep` 을 불렀습니다. 운영체제 스레드를 재우는 블로킹 호출입니다.

```rust
            tokio::spawn(async {
                std::thread::sleep(Duration::from_millis(200)); // 실행기 스레드를 붙잡는다
            })
```

| 작업 3개 | `current_thread` 런타임 |
|---|---:|
| `std::thread::sleep(200ms)` | 614ms |
| `spawn_blocking` 으로 옮김 | 207ms |

블로킹 호출은 `Pending` 을 내지 않고 스레드를 쥐고 있습니다. 실행기는 그동안 다른 작업을 `poll` 할 수 없어 시간이 더해졌습니다. `spawn_blocking` 은 블로킹 일을 별도 스레드 묶음으로 보내고, 그 완료를 `Future` 로 돌려줍니다. 파일 읽기, 압축 풀기, 문서 변환처럼 CPU 를 오래 쓰는 일도 같은 이유로 async 작업 안에서 그대로 부르면 안 됩니다.

문서 변환 엔진의 HTTP 서버는 tokio 위에서 돌지만 변환을 같은 프로세스에서 부르지 않습니다. 명령줄 도구를 `tokio::process::Command` 로 자식 프로세스로 띄우고, 시간 제한을 넘기면 `kill_on_drop(true)` 로 프로세스째 끝냅니다. 소스 주석이 이유를 적어 둡니다. tokio 의 시간 제한은 `Future` 를 버릴 뿐이라, 망가진 문서에서 블로킹 반복에 빠진 변환을 멈추지 못하기 때문입니다. `Pending` 을 내지 않는 코드는 실행기가 끊을 수 없다는 것을 운영에서 확인한 셈입니다.

## 기다림이 많으면 작업이 스레드보다 가볍습니다

| 무엇을 | 걸린 시간 |
|---|---:|
| tokio 작업 10,000개가 각자 100ms 기다림 | 111ms |
| OS 스레드 1,000개가 각자 100ms 기다림 | 125ms |

작업 1만 개가 스레드 하나에서 111ms 에 끝났습니다. 스레드는 1천 개만 만들었는데도 125ms 였습니다. 해석은 이렇습니다. 작업 하나는 상태 기계 하나라 만들고 바꾸는 비용이 작고, 스레드는 스택과 운영체제 일정 관리 비용이 듭니다. 다만 이것은 "기다리기만 하는" 일의 비교입니다. CPU 를 계속 쓰는 일이라면 코어 수만큼의 스레드(rayon, 5편)가 맞습니다.

## Send 가 아닌 값을 await 너머로 들고 가면 막힙니다

`tokio::spawn` 은 작업을 여러 스레드로 옮길 수 있어 `Future` 가 `Send` 이기를 요구합니다. `Rc` 를 쥔 채 `.await` 하면 이렇습니다.

```console
error: future cannot be sent between threads safely
   --> src/main.rs:12:26
    |
 12 |     let h = tokio::spawn(count_later(name));
    |                          ^^^^^^^^^^^^^^^^^ future returned by `count_later` is not `Send`
...
note: future is not `Send` as this value is used across an await
   --> src/main.rs:5:51
    |
  4 | async fn count_later(name: Rc<String>) -> usize {
    |                      ---- has type `Rc<String>` which is not `Send`
  5 |     tokio::time::sleep(Duration::from_millis(10)).await; // Rc 를 쥔 채 기다린다
    |                                                   ^^^^^ await occurs here, with `name` maybe used later
```

`.await` 를 건너 살아 있는 값은 상태 기계의 필드가 됩니다. 그 필드에 `Rc` 가 있으니 `Future` 전체가 `Send` 가 아닙니다. `Arc` 로 바꾸거나 `.await` 전에 값을 버립니다. 15편의 `std::sync::Mutex` 가드를 `.await` 너머로 들고 가도 같은 오류가 납니다. 그때는 tokio 의 `Mutex` 를 쓰거나 가드를 `.await` 전에 놓습니다.

## 면접에서 이렇게 묻는다

**Q1. `Future` 는 어떻게 실행되나요?**
실행기가 `poll` 을 부르고, `Pending` 이면 `Waker` 로 깨워질 때까지 다른 일을 합니다. `Ready` 면 결과를 받습니다. 직접 만든 `Future` 는 `Pending` 두 번 뒤 세 번째 `poll` 에서 끝났습니다.

**Q2. async 함수 안에서 블로킹 호출을 하면?**
그 실행기 스레드를 붙잡아 같은 스레드의 다른 작업이 멈춥니다. 실측에서 200ms 블로킹 셋이 614ms 였고 `spawn_blocking` 으로 옮기자 207ms 였습니다.

**Q3. async 와 스레드는 언제 고르나요?**
기다림이 많은 입출력은 async, CPU 계산은 스레드(rayon)입니다. 작업 1만 개가 스레드 하나로 111ms 에 끝났습니다. CPU 일은 async 로 해도 빨라지지 않습니다.

**Q4. `Pin` 은 왜 필요한가요?**
async 블록은 `.await` 를 건너 살아 있는 지역 변수와 그 참조를 함께 품는 상태 기계입니다. 이런 값이 메모리에서 옮겨지면 안쪽 참조가 틀어집니다. `Pin` 은 "이제 옮기지 않는다"는 약속을 타입으로 적습니다. 그래서 `poll` 이 `Pin<&mut Self>` 를 받습니다.

**Q5. `tokio::spawn` 에서 "future cannot be sent between threads safely"가 나면?**
`.await` 너머로 `Send` 가 아닌 값(`Rc`, `std` `Mutex` 가드 등)을 들고 있는 것입니다. `Arc` 로 바꾸거나, 그 값을 `.await` 전에 버리거나, 작업을 한 스레드에 묶는 `spawn_local` 을 씁니다.

## 연습

**연습 1.** `Countdown` 에서 `wake_by_ref()` 줄을 지우면 어떻게 될지 예상해 보세요.

<details>
<summary>풀이 보기</summary>

첫 `poll` 에서 `Pending` 을 낸 뒤 아무도 깨우지 않으니 실행기가 다시 `poll` 하지 않습니다. 프로그램이 끝나지 않고 멈춥니다. 직접 돌리면 터미널이 멈추므로 이 연재에서는 실행하지 않았습니다. `Pending` 을 낼 때 깨울 길을 남기는 것이 `Future` 구현자의 의무입니다(std 문서 `Future::poll`).

</details>

**연습 2.** `count_later` 를 `tokio::spawn` 할 수 있게 고치세요.

<details>
<summary>풀이 보기</summary>

`Rc<String>` 을 `Arc<String>` 으로 바꿉니다. 또는 `let n = name.len();` 으로 길이를 먼저 구하고 `drop(name)` 한 뒤 `.await` 해서, `.await` 너머에 `Rc` 가 남지 않게 합니다. 어느 쪽이든 상태 기계의 필드에 `Send` 가 아닌 값이 없어집니다.

</details>

**연습 3.** 파일 32개를 읽어 레코드를 세는 일을 async 로 짜야 할까요?

<details>
<summary>풀이 보기</summary>

대부분은 아닙니다. 읽기는 짧고 세기는 CPU 일이라 5편처럼 rayon 이 맞습니다. 파일이 네트워크 저장소에 있어 기다림이 길거나, 서버가 요청마다 이 일을 해야 한다면 async 로 받고 세는 부분은 `spawn_blocking` 으로 보냅니다.

</details>

## 한 단계 더 — async 블록은 크기가 정해진 상태 기계입니다

async 블록의 크기를 쟀습니다.

```rust
    let small = async {
        fetch(1).await;
    };
    let big = async {
        let buf = [7u8; 4096]; // await 를 건너 살아 있는 값
        fetch(1).await;
        buf[0]
    };
```

`small` 은 144 바이트, `big` 은 4,240 바이트였습니다. `.await` 를 건너 살아 있는 4KB 배열이 상태 기계의 필드가 된 것입니다. `Box::pin(big)` 으로 힙에 두자 손에 쥔 값은 8 바이트가 됐고, 결과 `7` 을 그대로 돌려줬습니다. 큰 `Future` 를 여러 단계 넘기면 매번 복사되므로 힙에 고정해 두는 편이 낫습니다.

이 상태 기계가 자기 필드를 가리키는 참조를 품을 수 있어 `Pin` 이 필요합니다. async 블록은 기본으로 `Unpin` 이 아닙니다. `Unpin` 을 요구하는 함수에 넘기면 막힙니다.

```console
error[E0277]: `{async block@src/main.rs:6:15: 6:20}` cannot be unpinned
...
   = note: consider using the `pin!` macro
           consider using `Box::pin` if you need to access the pinned value outside of the current scope
```

컴파일러가 권하는 `pin!` 이나 `Box::pin` 으로 고정하면 됩니다. 자기 참조와 `Pin` 의 규칙은 std 문서 `std::pin` 모듈에 근거했고, 자기 참조가 실제로 틀어지는 모습은 안전한 코드로는 만들 수 없어 보이지 않았습니다.

## 정리

async 는 기다림을 겹치는 도구입니다. `Future` 는 `poll` 될 때만 나아가고, `Pending` 동안 스레드를 놓습니다. 그래서 블로킹 호출은 `spawn_blocking` 으로 빼고, `.await` 너머로 들고 가는 값은 `Send` 인지 봅니다.

19편에서는 메모리 배치와 `unsafe` 를 다룹니다. `size_of` 와 `repr` 로 배치를 재고, Miri 로 정의되지 않은 동작을 잡아 봅니다.
