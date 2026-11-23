# Rust 강의 15편 — 내부 가변성: Cell, RefCell, Mutex, Atomic

> `&self` 로 받는 메서드 안에서 캐시를 채워야 한다면 어떻게 할까? 여러 스레드가 같은 값을 고치면?

답부터 적습니다. 내부 가변성(interior mutability) 타입을 씁니다. 이 타입들은 "고치는 빌림은 하나"라는 규칙을 버리지 않습니다. 검사를 컴파일 시점에서 실행 시점으로 옮깁니다. 한 스레드에서는 `Cell`(값 통째 바꾸기)과 `RefCell`(실행 중에 빌림을 세고 어기면 패닉)을 씁니다. 여러 스레드에서는 `Mutex`·`RwLock`(잠금)과 원자 정수를 씁니다. 실측에서 느린 것은 도구가 아니라 다툼이었습니다. 8 스레드가 한 값을 두드리면 1초 안팎이 걸렸고, 각자 세고 합치면 0.005초였습니다.

> [그림] 왼쪽 유리 진열장 &FontTable 안에 cache 서랍과 hits 2 계수기, borrow_mut 1 표시가 있다. 두 번째 고치기 시도에 already borrowed 경고가 번쩍이고 검수 로봇이 계수기를 본다. 오른쪽에는 8 스레드가 8천만 번 더한 시간이 Mutex 0.95~1.03초, AtomicU64 1.0~2.9초, 각자 세고 합치기 0.005초로 적혀 있다

유리 진열장은 `&` 로 빌려 준 값입니다. 밖에서는 보기만 하지만, 안의 장치가 빌림을 세면서 고치기를 허락합니다.

Rust 강의 15편입니다. [14편](https://blog.doksam.com/blog/rust-course-14-smart-pointers/)에서 순환을 만들려고 `RefCell` 을 설명 없이 썼습니다. 이번 편에서 풉니다. 코드는 2026-11-18, Apple M4 Pro 14코어, rustc 1.100.0 으로 돌려 확인했습니다.

## & 로 받고도 고치는 타입이 따로 있습니다

문서 변환기에서 글꼴 번호를 이름으로 바꾸는 표를 생각해 봅니다. 조회는 `&self` 인데, 처음 보는 번호는 계산해서 캐시에 넣고 싶습니다.

```rust
struct FontTable {
    hits: Cell<u32>,
    cache: RefCell<HashMap<u16, String>>,
}

impl FontTable {
    fn name(&self, id: u16) -> String {
        if let Some(n) = self.cache.borrow().get(&id) {
            self.hits.set(self.hits.get() + 1);
            return n.clone();
        }
        let n = format!("글꼴-{id}"); // 비싼 계산이라고 치자
        self.cache.borrow_mut().insert(id, n.clone());
        n
    }
}
```

번호 3, 7, 3, 3 을 조회하자 캐시 2개, 적중 2번이 됐습니다. 호출하는 쪽은 `FontTable` 을 `&` 로만 들고 있습니다.

| 타입 | 고치는 방법 | 검사 | 스레드 |
|---|---|---|---|
| `Cell<T>` | 값을 통째로 넣고 뺀다(`get`·`set`) | 없음. 참조를 주지 않으니 겹칠 일이 없다 | 한 스레드 |
| `RefCell<T>` | `borrow()`·`borrow_mut()` 로 빌린다 | 실행 중에 센다. 어기면 패닉 | 한 스레드 |
| `Mutex<T>` | `lock()` 으로 잠근다 | 잠금. 한 번에 하나 | 여러 스레드 |
| `RwLock<T>` | `read()`·`write()` | 읽기 여럿 또는 쓰기 하나 | 여러 스레드 |
| `AtomicU64` 등 | `fetch_add` 등 원자 연산 | CPU 원자 명령 | 여러 스레드 |
| `OnceLock<T>` | 처음 한 번만 채운다 | 한 번만 쓰기 | 여러 스레드 |

크기도 쟀습니다. `u64`·`Cell<u64>`·`AtomicU64` 는 8바이트, `RefCell<u64>` 는 빌림 수를 더해 16바이트, `Mutex<u64>`·`RwLock<u64>` 는 24바이트였습니다.

## RefCell 은 겹친 고치기를 실행 중에 잡습니다

컴파일러가 막던 코드가 `RefCell` 안에서는 컴파일됩니다.

```rust
    let log = RefCell::new(vec![String::from("시작")]);
    let first = log.borrow_mut();
    let mut second = log.borrow_mut(); // 컴파일은 된다
    second.push(String::from("끝"));
```

```console
thread 'main' (30808704) panicked at src/bin/double_borrow.rs:6:26:
RefCell already borrowed
```

3편의 E0499 가 실행 중 패닉으로 바뀌었습니다. 규칙은 같고 검사 시점만 다릅니다. 패닉이 싫으면 `try_borrow_mut()` 으로 `Result` 를 받습니다. 첫 빌림이 살아 있는 동안은 `Err(BorrowMutError)`, 놓은 뒤에는 `Ok` 였습니다.

해석하면 `RefCell` 은 컴파일러가 증명하지 못하는 구조를 위한 탈출구입니다. 그만큼 틀리면 실행 중에 터집니다. 14편처럼 그래프 구조를 만들 때나 `&self` 캐시처럼 쓸 곳이 분명할 때만 씁니다. 스레드 사이에서는 쓸 수 없습니다.

```console
error[E0277]: `RefCell<Vec<String>>` cannot be shared between threads safely
...
  = help: the trait `Sync` is not implemented for `RefCell<Vec<String>>`
  = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` instead
```

컴파일러가 `RwLock` 을 대신 권합니다. 빌림 수를 세는 정수가 원자 연산이 아니라서 두 스레드가 동시에 세면 수가 틀어지기 때문입니다.

## Mutex 는 잠금을 쥔 채 패닉하면 중독됩니다

`Mutex::lock()` 이 `Result` 를 돌려주는 이유가 있습니다. 잠금을 쥔 스레드가 패닉하면 안의 값이 고치다 만 상태일 수 있습니다. 그래서 그 뒤의 `lock()` 은 `Err` 를 돌려 알립니다. 이것을 중독(poisoning)이라 합니다.

```rust
    let shared = Arc::new(Mutex::new(vec![1, 2]));
    let s2 = Arc::clone(&shared);
    let r = thread::spawn(move || {
        let mut g = s2.lock().unwrap();
        g.push(3);
        panic!("잠금을 쥔 채 패닉");
    })
    .join();
    match shared.lock() {
        Ok(v) => println!("  잠금 성공 {v:?}"),
        Err(poisoned) => println!("  중독됨, 안의 값: {:?}", poisoned.into_inner()),
    }
```

```console
  스레드 결과 is_err = true
  중독됨, 안의 값: [1, 2, 3]
```

`into_inner()` 로 안의 값을 꺼낼 수는 있습니다. 그 값이 믿을 만한지는 프로그램이 판단해야 합니다. 많은 코드가 `lock().unwrap()` 을 쓰는 것은 "중독되면 같이 죽겠다"는 선택입니다.

## 느린 것은 도구가 아니라 다툼입니다

한 스레드에서 1억 번 더했습니다. 그릇을 `black_box` 로 가려 모두 메모리를 오가게 맞췄습니다(세 번 실행).

| 그릇 | 시간 |
|---|---:|
| `u64` | 129~135ms |
| `Cell<u64>` | 129~130ms |
| `RefCell<u64>` | 119~121ms |
| `AtomicU64` (Relaxed) | 172ms |
| `Mutex<u64>` (다툼 없음) | 421~423ms |

`RefCell` 의 실행 중 검사는 이 반복에서 측정 오차 안이었습니다. 다툼 없는 `Mutex` 는 잠그고 푸는 비용 때문에 3배 남짓 느렸습니다.

8 스레드가 같은 값에 1천만 번씩, 모두 8천만 번 더했습니다.

| 방식 | 시간 (세 번) |
|---|---:|
| `Arc<Mutex<u64>>` | 946~1,031ms |
| `Arc<AtomicU64>` | 1,007~2,946ms |
| 스레드마다 따로 세고 끝에 합치기 | 4~5ms |

원자 정수가 잠금보다 느리거나 비슷했고, 실행마다 세 배까지 흔들렸습니다. 해석은 이렇습니다. 한 메모리 칸을 여러 코어가 번갈아 고치면 그 칸이 코어 사이를 오가는 비용이 큽니다. `Mutex` 는 기다리는 스레드를 재워 한 스레드가 여러 번 연달아 더하게 되니 오히려 덜 오갈 수 있습니다. 이 설명은 측정에서 끌어낸 추론이고 하드웨어 카운터로 확인하지는 않았습니다. 확실한 결론은 마지막 줄입니다. 공유하지 않으면 약 200배 빠릅니다. 5편의 rayon 처럼 결과를 `map` 으로 각자 만들고 `collect` 로 모으는 설계가 이 줄입니다.

문서 변환 엔진은 `RefCell`·`Mutex` 를 한 군데도 쓰지 않습니다. 내부 가변성은 실패 수를 세는 `AtomicU32` 와, 무거운 OCR 엔진이나 설정 값을 처음 한 번만 만드는 `OnceLock` 뿐입니다. 공유를 줄이는 설계가 먼저라는 판단과 맞습니다.

## 면접에서 이렇게 묻는다

**Q1. 내부 가변성이란 무엇인가요?**
`&` 로 공유된 값을 고치게 해 주는 성질입니다. 빌림 규칙을 없애지 않고 검사 시점을 실행 중으로 옮깁니다. `Cell`, `RefCell`, `Mutex`, `RwLock`, 원자 타입, `OnceLock` 이 있습니다.

**Q2. `RefCell` 은 언제 패닉하나요?**
고치는 빌림이 살아 있는데 다시 빌리거나, 읽는 빌림이 살아 있는데 고치려 할 때입니다. 메시지는 `RefCell already borrowed` 입니다. `try_borrow_mut` 으로 패닉 대신 `Result` 를 받을 수 있습니다.

**Q3. `Rc<RefCell<T>>` 와 `Arc<Mutex<T>>` 는 언제 쓰나요?**
한 스레드에서 공동 소유와 고치기가 필요하면 앞쪽, 여러 스레드면 뒤쪽입니다. `RefCell` 은 `Sync` 가 아니라 스레드를 넘으면 E0277 입니다.

**Q4. Mutex 중독(poisoning)은 무엇인가요?**
잠금을 쥔 스레드가 패닉하면 이후 `lock()` 이 `Err(PoisonError)` 를 돌려주는 동작입니다. 고치다 만 값을 알리려는 장치입니다. `into_inner()` 로 값을 꺼낼 수 있지만 믿을 만한지는 프로그램이 판단합니다.

**Q5. 카운터를 여러 스레드에서 올릴 때 Mutex 와 원자 정수 중 무엇이 빠른가요?**
다툼이 없으면 원자 정수가 빠릅니다. 다툼이 심하면 둘 다 느리고 원자 정수가 더 흔들릴 수 있습니다(실측 1~2.9초). 가장 빠른 것은 공유하지 않고 각자 센 뒤 합치는 설계입니다(4~5ms).

## 연습

**연습 1.** `FontTable` 에서 `hits` 를 `RefCell<u32>` 로 바꾸지 않고 `Cell<u32>` 로 둔 이유는?

<details>
<summary>풀이 보기</summary>

`u32` 는 `Copy` 라 값을 통째로 꺼내고 넣으면 충분합니다. `Cell` 은 참조를 내주지 않으므로 겹칠 일이 없어 검사도, 빌림 수 칸도 없습니다(`Cell<u64>` 8바이트 대 `RefCell<u64>` 16바이트). 참조가 필요한 `HashMap` 은 `RefCell` 이어야 합니다.

</details>

**연습 2.** `RefCell already borrowed` 로 죽는 `double_borrow` 를 고치세요.

<details>
<summary>풀이 보기</summary>

```rust
    let log = RefCell::new(vec![String::from("시작")]);
    {
        let first = log.borrow_mut();
        println!("첫 빌림: {}개", first.len());
    } // 여기서 first 가 놓인다
    log.borrow_mut().push(String::from("끝"));
```

빌림 가드는 값이라 범위가 끝나면 놓입니다. 블록으로 첫 빌림을 일찍 끝내면 두 번째 빌림이 성공해 `["시작", "끝"]` 이 찍힙니다.

</details>

**연습 3.** 8 스레드가 파일마다 레코드 수를 세어 합계를 낸다면 어떤 구조로 짜겠습니까?

<details>
<summary>풀이 보기</summary>

공유 카운터 없이 스레드마다 합을 돌려받아 더합니다. 본문 실험의 세 번째 방식이고, `thread::scope` 로 스레드를 띄워 `join()` 결과를 `sum()` 했습니다. rayon 이라면 `files.par_iter().map(count).sum()` 한 줄입니다. 실패 수처럼 진행 중에 바깥에서 봐야 하는 값만 원자 정수로 둡니다.

</details>

## 한 단계 더 — 같은 if let 이 에디션에 따라 패닉합니다

`FontTable::name` 을 `else` 로 쓰면 이렇습니다.

```rust
    if let Some(n) = cache.borrow().get(&3) {
        println!("적중 {n}");
    } else {
        cache.borrow_mut().insert(3, "글꼴-3".into());
        println!("채움");
    }
```

이 파일을 `rustc --edition 2024` 로 빌드하면 `채움` 을 찍고 정상으로 끝납니다. `--edition 2021` 로 빌드하면 `RefCell already borrowed` 로 패닉합니다. 2021 에디션까지는 `if let` 조건에서 만든 임시 값(`borrow()` 가드)이 `else` 갈래가 끝날 때까지 살았습니다. 2024 에디션은 `else` 에 들어가기 전에 버리도록 바꿨습니다. Rust Edition Guide 의 "if let temporary scope" 절이 이 변경을 설명합니다.

같은 코드가 에디션에 따라 실행 결과가 다를 수 있다는 뜻입니다. 1편에서 문서 변환 엔진이 아직 2021 에디션이라고 했습니다. 이런 엔진에 2024 에디션 예제를 옮겨 붙이면 컴파일은 되고 실행에서 터질 수 있습니다. 에디션을 올릴 때는 `cargo fix --edition` 의 안내를 따르고, `RefCell` 처럼 실행 중 검사가 있는 코드를 테스트로 덮어 둡니다.

## 정리

`&` 로 받고도 고쳐야 하면 내부 가변성 타입을 고릅니다. 한 스레드는 `Cell`·`RefCell`, 여러 스레드는 `Mutex`·`RwLock`·원자 정수입니다. 검사가 실행 중으로 옮겨 간 만큼 틀리면 패닉과 중독으로 드러납니다. 성능이 문제라면 도구를 바꾸기 전에 공유 자체를 줄일 수 있는지 봅니다.

16편에서는 트레이트를 깊게 봅니다. 제네릭의 모노모피제이션과 `dyn` 의 vtable 을 바이너리 크기와 호출 비용으로 재고, 연관 타입과 객체 안전성을 다룹니다.
