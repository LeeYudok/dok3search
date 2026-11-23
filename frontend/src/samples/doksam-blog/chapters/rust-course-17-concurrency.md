# Rust 강의 17편 — 동시성: Send, Sync, 스레드와 채널

> 여러 스레드가 데이터를 나눠 쓸 때 Rust 는 무엇을 막아 주고, 무엇은 막지 못할까?

답부터 적습니다. 막아 주는 것은 데이터 경쟁입니다. 타입마다 두 표시가 붙습니다. `Send` 는 "다른 스레드로 넘겨도 된다", `Sync` 는 "여러 스레드가 `&` 로 함께 봐도 된다"입니다. 컴파일러가 이 표시를 자동으로 따지고, 어기면 E0277 로 막습니다. 막지 못하는 것은 교착(deadlock)입니다. 두 스레드가 잠금을 엇갈린 순서로 잡으면 컴파일은 되고 실행이 멈춥니다. 그래서 데이터는 범위 스레드로 빌려 주거나 채널로 소유권째 넘기고, 공유가 꼭 필요하면 잠금 순서를 정합니다.

> [그림] 왼쪽에서 스레드 셋이 결과 상자를 관을 따라 rx 상자로 보내고 검수 로봇이 rx 를 본다. drop(tx) 를 해야 받기가 끝난다. 오른쪽에서 두 로봇이 a 와 b 자물쇠를 하나씩 쥔 채 상대 것을 기다려 1초 Timeout 도장이 찍혀 있다. 늘 a 다음 b 로 잠그면 둘 다 끝난다

두 장면 모두 이 글의 실행 결과입니다. 오른쪽 교착은 1초를 기다려도 끝나지 않았습니다.

Rust 강의 17편입니다. 이 연재에서 `Send`·`Sync` 는 여러 번 오류로 나왔습니다. [5편](https://blog.doksam.com/blog/rust-course-05-parser-cli/)의 rayon, [14편](https://blog.doksam.com/blog/rust-course-14-smart-pointers/)의 `Rc`, [15편](https://blog.doksam.com/blog/rust-course-15-interior-mutability/)의 `RefCell` 입니다. 이번 편에서 한데 모읍니다. 코드는 2026-11-18 에 rustc 1.100.0 으로 돌려 확인했습니다.

## Send 와 Sync 는 컴파일러가 자동으로 붙이는 표시입니다

두 트레이트에는 메서드가 없습니다. 타입의 성질을 표시만 합니다. 구조체는 필드가 모두 `Send` 면 자동으로 `Send` 가 됩니다. `Sync` 도 같습니다. 타입 조건만 검사하는 빈 함수로 확인했습니다.

```rust
fn is_send<T: Send>() -> bool {
    true
}
fn is_sync<T: Sync>() -> bool {
    true
}
```

| 타입 | `Send` | `Sync` | 이유 |
|---|---|---|---|
| `String`, `Vec<u8>` | 예 | 예 | 소유한 값, 내부 가변성 없음 |
| `Arc<String>` | 예 | 예 | 원자 연산으로 센다 |
| `Mutex<Vec<u8>>` | 예 | 예 | 잠금이 동시 접근을 막는다 |
| `RefCell<u32>`, `Cell<u32>` | 예 | 아니오 | 넘기는 것은 되지만, 함께 보며 고치면 수가 틀어진다 |
| `Rc<u8>` | 아니오 | 아니오 | 원자 연산이 아닌 수를 복제본이 함께 쓴다 |

"예"인 칸은 위 함수가 컴파일되고 `true` 를 찍은 결과입니다. "아니오" 칸은 컴파일 오류로 확인했습니다.

```console
error[E0277]: `Cell<u32>` cannot be shared between threads safely
...
  = help: the trait `Sync` is not implemented for `Cell<u32>`
  = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` or `std::sync::atomic::AtomicU32` instead
```

`RefCell` 이 `Send` 인데 `Sync` 가 아닌 것을 보면 두 표시의 차이가 보입니다. 통째로 다른 스레드에 넘기면 한 스레드만 쓰니 안전합니다. 여러 스레드가 동시에 `&` 로 보면서 `borrow_mut` 을 부르면 빌림 수가 틀어집니다. The Rustonomicon 의 "Send and Sync" 절은 이를 "`&T` 가 `Send` 면 `T` 는 `Sync`"로 정리합니다.

## 범위 스레드는 지역 데이터를 빌려 줍니다

`thread::spawn` 은 `'static` 을 요구해 지역 변수를 빌려 줄 수 없었습니다(9·13편). `thread::scope` 는 범위가 끝나기 전에 모든 스레드를 기다리므로 빌려 줄 수 있습니다.

```rust
    let lines: Vec<String> = (0..1000).map(|i| format!("줄 {i} 표 문단")).collect();
    let chunks: Vec<&[String]> = lines.chunks(250).collect();
    let counts: Vec<usize> = thread::scope(|s| {
        let handles: Vec<_> = chunks
            .iter()
            .map(|c| s.spawn(move || c.iter().filter(|l| l.contains('표')).count()))
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });
```

조각 넷이 `[250, 250, 250, 250]`, 합 1000 이었습니다. `Arc` 도 `clone` 도 없습니다. 클로저가 받는 `c` 는 `&[String]` 이고, 이것이 다른 스레드로 넘어가려면 `String` 이 `Sync` 여야 합니다. 그 조건을 컴파일러가 확인했습니다. 15편에서 "각자 세고 합치기"가 공유 카운터보다 약 200배 빨랐던 구조가 바로 이것입니다.

## 채널은 결과의 소유권을 넘깁니다

공유하지 않고 주고받는 방법이 채널입니다. 보내는 쪽이 값을 `send` 하면 소유권이 받는 쪽으로 넘어갑니다.

```rust
    let (tx, rx) = mpsc::channel::<(usize, usize)>();
    for id in 0..4 {
        let tx = tx.clone();
        thread::spawn(move || {
            let n = (id + 1) * 100;
            tx.send((id, n)).unwrap();
        });
    }
    drop(tx); // 원본 송신자를 버려야 수신 반복이 끝난다
    let mut got: Vec<(usize, usize)> = rx.iter().collect();
```

받은 결과는 `[(0, 100), (1, 200), (2, 300), (3, 400)]` 이었습니다(도착 순서는 실행마다 달라 정렬했습니다). 빠뜨리기 쉬운 줄이 `drop(tx)` 입니다. `rx.iter()` 는 모든 송신자가 사라져야 끝납니다. 원본 `tx` 를 쥐고 있으면 일꾼이 다 끝나도 받는 쪽은 기다립니다. 원본을 버리지 않은 채널로 시험했더니 첫 값은 `Ok(1)` 이었고, 다음 값은 1초 뒤 `Err(Timeout)` 이었습니다. `recv()` 였다면 영원히 기다렸을 것입니다.

## 교착은 컴파일러가 막지 못합니다

두 스레드가 잠금 `a`, `b` 를 서로 반대 순서로 잡습니다.

```rust
        thread::spawn(move || {
            let (first, second) = if flip { (&b, &a) } else { (&a, &b) };
            let _g1 = first.lock().unwrap();
            thread::sleep(Duration::from_millis(50));
            let _g2 = second.lock().unwrap();
            done.send(flip).unwrap();
        });
```

```console
  1초 안에 끝난 스레드: Err(Timeout)
```

컴파일은 경고 하나 없이 통과했습니다. 첫 스레드는 `a` 를 쥐고 `b` 를, 둘째는 `b` 를 쥐고 `a` 를 기다립니다. 메모리 안전과는 상관없는 문제라 빌림 검사기의 범위 밖입니다. 해결은 규칙입니다. 여러 잠금을 잡을 때는 언제나 같은 순서로 잡습니다. 두 스레드 모두 `a` 다음 `b` 로 바꾸자 둘 다 끝났고 `a = 2, b = 2` 였습니다. 더 근본적인 해결은 잠금을 둘 이상 동시에 쥐는 설계를 피하는 것입니다.

## 면접에서 이렇게 묻는다

**Q1. `Send` 와 `Sync` 는 무엇인가요?**
`Send` 는 값을 다른 스레드로 옮겨도 되는 타입, `Sync` 는 `&T` 를 여러 스레드가 함께 가져도 되는 타입입니다. 필드가 모두 그러면 컴파일러가 자동으로 붙입니다. 어기면 E0277 입니다.

**Q2. `Rc` 는 왜 `Send` 가 아닌가요?**
복제본들이 원자 연산이 아닌 참조 수를 함께 씁니다. 두 스레드가 동시에 복제하거나 버리면 수가 틀어져 이중 해제나 누수가 납니다. 스레드 사이에서는 `Arc` 를 씁니다.

**Q3. `thread::scope` 와 `thread::spawn` 은 어떻게 다른가요?**
`spawn` 은 스레드가 함수보다 오래 살 수 있어 `'static` 을 요구합니다. `scope` 는 범위가 끝나기 전에 모든 스레드를 기다리므로 지역 데이터를 빌려 줄 수 있습니다.

**Q4. 채널과 공유 메모리는 언제 고르나요?**
결과를 한 곳에 모으거나 단계별로 넘기면 채널이 단순합니다. 소유권이 넘어가 공유 자체가 없습니다. 여러 스레드가 같은 상태를 계속 고쳐야 하면 `Arc<Mutex<T>>` 입니다. 이때는 다툼 비용(15편)과 교착을 함께 따집니다.

**Q5. Rust 가 교착도 막아 주나요?**
아닙니다. 실험에서 엇갈린 잠금 순서는 컴파일되고 1초 뒤에도 끝나지 않았습니다. 잠금 순서를 정하거나, 잠금을 둘 이상 쥐지 않게 설계합니다.

## 연습

**연습 1.** 범위 스레드 예제에서 `move` 를 빼면 어떻게 될까요? 에디션 2018 과 2021 에서 결과가 같을까요?

<details>
<summary>풀이 보기</summary>

에디션 2021 이후(이 연재의 2024 포함)에서는 `move` 를 빼도 컴파일되고 같은 결과가 나옵니다. 같은 파일을 `rustc --edition 2018` 로 빌드하면 E0373(`closure may outlive the current function, but it borrows c`)입니다. `c` 는 `map` 클로저의 지역 변수라 반복 한 번이 끝나면 사라집니다. 2018 에서는 클로저가 변수 `c` 자체를 빌려서 막혔습니다. 2021 부터는 클로저가 실제로 쓰는 경로만 잡습니다(Rust Edition Guide 2021, "Disjoint capture in closures"). 그래서 `c` 를 거쳐 `chunks` 의 원소를 빌리고, 그 원소는 범위 밖에 있어 안전합니다. 에디션에 기대지 않으려면 `move` 를 적어 두는 편이 읽기 쉽습니다.

</details>

**연습 2.** 채널로 결과를 모을 때 일꾼 수를 모르고도 끝을 알 수 있는 이유는?

<details>
<summary>풀이 보기</summary>

모든 `Sender` 가 버려지면 `rx.iter()` 가 끝납니다. 일꾼마다 `tx.clone()` 을 하나씩 들고 가서 일이 끝나면 버립니다. 원본 `tx` 는 직접 `drop` 합니다. 본문 실험처럼 원본을 쥐고 있으면 받는 쪽이 끝나지 않습니다.

</details>

**연습 3.** 교착 예제를 고쳐 두 스레드가 모두 끝나게 하세요.

<details>
<summary>풀이 보기</summary>

```rust
        thread::spawn(move || {
            let mut ga = a.lock().unwrap();
            thread::sleep(Duration::from_millis(50));
            let mut gb = b.lock().unwrap();
            *ga += 1;
            *gb += 1;
            done.send(id).unwrap();
        });
```

두 스레드가 모두 `a` 다음 `b` 로 잠급니다. 실행하면 `끝난 스레드 2 개, a = 2, b = 2` 입니다.

</details>

## 한 단계 더 — Send 와 Sync 를 직접 구현할 때

`Send`·`Sync` 는 `unsafe impl` 로 직접 붙일 수 있습니다. 컴파일러가 자동으로 붙이지 않은 타입(예: 원시 포인터 `*const T` 를 품은 구조체)에 "그래도 안전하다"고 사람이 보증하는 것입니다. 보증이 틀리면 데이터 경쟁이 그대로 생기고 컴파일러는 막지 못합니다. 그래서 `unsafe` 가 붙습니다. 19편에서 `unsafe` 의 범위를 다루며 다시 봅니다.

반대로 자동으로 붙는 것을 막고 싶을 때도 있습니다. 필드에 `PhantomData<*const ()>` 같은 표시용 값을 넣으면 그 타입은 `Send`·`Sync` 가 아니게 됩니다. 특정 스레드에서만 써야 하는 핸들(그래픽 문맥, 일부 C 라이브러리 객체)을 감쌀 때 씁니다. 이 두 방법의 규칙은 The Rustonomicon "Send and Sync" 절에 근거했고, 이번 편에서 직접 돌리지는 않았습니다.

## 정리

Rust 는 `Send`·`Sync` 표시로 데이터 경쟁을 컴파일할 때 막고, 교착은 막지 못합니다. 데이터는 범위 스레드로 빌려 주거나 채널로 넘기고, 공유는 마지막 수단으로 둡니다. 잠금이 둘 이상이면 순서를 정해 문서에 적습니다.

18편에서는 스레드 대신 async 를 씁니다. tokio 를 실제로 받아 `Future` 가 어떻게 돌고, 블로킹 호출이 왜 문제인지 잽니다.
