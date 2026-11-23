# Rust 강의 23편 — 시스템 과제를 Rust 로 풀기

> 면접에서 "레이트 리미터를 만들어 보세요"를 Rust 로 받으면 무엇부터 할까?

답부터 적습니다. 코드보다 질문이 먼저입니다. 몇 개를, 얼마나 자주, 여러 스레드에서 쓰는지 묻습니다. 그다음 자료 구조를 고르고 이유를 말합니다. 그리고 시간처럼 바깥에서 들어오는 것은 인자로 받아 테스트가 마음대로 돌릴 수 있게 만듭니다. 이 편의 세 과제는 모두 이 순서로 풀었고, 단위 테스트 3개와 실행 측정을 거쳤습니다. 로그 100만 줄을 26ms 에 셌고, 8 스레드가 함께 쓴 레이트 리미터는 0.5초 동안 계산대로 약 600개(실측 599개)를 통과시켰습니다.

> [그림] 칠판 세 칸에 토큰 버킷(초당 1,000, 0.5초에 599개 통과), LRU 캐시(가장 오래 안 쓴 b 가 나간다, HashMap + BTreeMap), 로그 집계기(각자 세고 채널로 합침, 100만 줄 26ms)가 그려져 있다. 아래에 요구부터 묻는다, 시간은 인자로 받는다, 공유는 마지막에 라는 세 줄이 있고 검수 로봇이 칠판을 본다

칠판 아래 세 줄이 이 편의 순서입니다. 숫자는 모두 이 글의 실행 결과입니다.

Rust 강의 23편입니다. 지금까지의 도구를 과제 셋에 씁니다. [8편](https://blog.doksam.com/blog/rust-course-08-collections-iterators/)의 컬렉션, [15편](https://blog.doksam.com/blog/rust-course-15-interior-mutability/)의 내부 가변성, [17편](https://blog.doksam.com/blog/rust-course-17-concurrency/)의 범위 스레드와 채널입니다. 코드는 2026-11-18 에 rustc 1.100.0 으로 돌려 확인했습니다.

## 요구부터 묻습니다

세 과제 모두 처음 1~2분은 질문입니다. 면접관은 답만큼 질문을 봅니다.

| 과제 | 먼저 물을 것 | 답에 따라 바뀌는 것 |
|---|---|---|
| 레이트 리미터 | 사용자별인가 전체인가, 순간 몰림을 얼마나 허용하나, 여러 서버인가 | 키별 맵 여부, 버킷 용량, 공유 저장소 필요 여부 |
| LRU 캐시 | 용량, 읽기와 쓰기 비율, 여러 스레드인가 | 자료 구조, 잠금 방식 |
| 로그 집계기 | 양, 실시간인가 일괄인가, 무엇을 세나 | 병렬 방식, 메모리 한도 |

아래 풀이는 "한 프로세스, 여러 스레드, 메모리 안" 조건입니다. 여러 서버라면 레이트 리미터의 상태를 공유 저장소로 옮겨야 해서 설계가 달라집니다.

## 레이트 리미터: 시간을 인자로 받는 토큰 버킷

토큰 버킷은 초당 일정 개수씩 토큰이 차는 통입니다. 요청마다 하나씩 쓰고, 없으면 거절합니다. 통의 용량만큼 순간 몰림을 허용합니다.

```rust
pub struct TokenBucket {
    capacity: f64,
    tokens: f64,
    per_sec: f64,
    last: Duration,
}

impl TokenBucket {
    /// now 는 시작 시점부터 지난 시간. 토큰이 있으면 하나 쓰고 true.
    pub fn try_acquire(&mut self, now: Duration) -> bool {
        let elapsed = now.saturating_sub(self.last).as_secs_f64();
        self.tokens = (self.tokens + elapsed * self.per_sec).min(self.capacity);
        self.last = now;
        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }
}
```

핵심은 `now` 를 인자로 받은 것입니다. 안에서 `Instant::now()` 를 부르면 테스트가 실제로 1초를 기다려야 하고 결과가 흔들립니다. 바깥에서 받으면 테스트가 시계를 돌립니다.

```rust
    #[test]
    fn bucket_refills_over_time() {
        let mut b = TokenBucket::new(2, 1);
        let t = Duration::from_millis;
        assert!(b.try_acquire(t(0)));
        assert!(b.try_acquire(t(0)));
        assert!(!b.try_acquire(t(0)), "두 개를 다 썼다");
        assert!(!b.try_acquire(t(500)), "0.5초에는 반 개");
        assert!(b.try_acquire(t(1000)), "1초에는 한 개");
        assert!(b.try_acquire(t(10_000)) && b.try_acquire(t(10_000)));
        assert!(!b.try_acquire(t(10_000)), "용량 2를 넘게 쌓이지 않는다");
    }
```

이 테스트는 0초 만에 10초 뒤를 시험합니다. `saturating_sub` 는 시계가 거꾸로 들어와도 음수가 되지 않게 합니다(1편의 넘침 처리).

여러 스레드가 함께 쓸 때는 `Arc<Mutex<TokenBucket>>` 로 감쌌습니다. 용량 100, 초당 1,000 인 버킷을 8 스레드가 0.5초 동안 두드리자 599개가 통과했습니다. 계산값은 처음 용량 100 에 0.5초 × 1,000 = 500 을 더한 600 입니다. 잠금 안의 일이 덧셈 몇 번이라 15편에서 본 다툼 비용도 작습니다. 키(사용자)별 버킷이 필요하면 `HashMap<UserId, TokenBucket>` 을 잠금 하나로 감싸거나, 다툼이 크면 키를 나눠 잠금을 여러 개 둡니다.

## LRU 캐시: 빌림 검사기와 싸우지 않는 구조

LRU 캐시는 용량을 넘으면 가장 오래 쓰지 않은 항목을 내보냅니다. 다른 언어의 교과서 답은 해시맵과 이중 연결 리스트입니다. Rust 에서 이중 연결 리스트는 노드가 서로를 가리키므로 `Rc<RefCell>` 과 `Weak`(14·15편)이나 `unsafe` 가 필요합니다. 면접 시간 안에 안전하게 풀려면 구조를 바꿉니다.

```rust
pub struct Lru<K, V> {
    cap: usize,
    tick: u64,
    map: HashMap<K, (V, u64)>,
    order: BTreeMap<u64, K>,
}
```

값마다 마지막으로 쓴 번호(`tick`)를 붙이고, `BTreeMap` 이 번호 순서로 키를 들고 있습니다. 가장 오래된 것은 `order.pop_first()` 입니다.

```rust
    pub fn put(&mut self, k: K, v: V) -> Option<K> {
        self.tick += 1;
        if let Some((_, used)) = self.map.insert(k.clone(), (v, self.tick)) {
            self.order.remove(&used);
        }
        self.order.insert(self.tick, k);
        if self.map.len() > self.cap {
            let (_, oldest) = self.order.pop_first()?;
            self.map.remove(&oldest);
            return Some(oldest);
        }
        None
    }
```

테스트에서 용량 2에 `a`, `b` 를 넣고 `a` 를 다시 읽은 뒤 `c` 를 넣자 `b` 가 나갔습니다. 같은 키를 다시 넣으면 내보내지 않고 값만 바뀌었습니다.

| 연산 | 연결 리스트 판 | 이 판 |
|---|---|---|
| `get` | O(1) | O(log n) |
| `put` | O(1) | O(log n) |
| 안전성 | `Rc<RefCell>` 이나 `unsafe` | 안전한 표준 컬렉션만 |

면접에서는 이 교환을 말로 설명합니다. "O(1) 이 꼭 필요하면 인덱스로 노드를 가리키는 연결 리스트(노드를 `Vec` 에 두고 이웃을 번호로 적기)를 쓰겠다"까지 말하면 충분합니다. 그 판은 이번에 구현하지 않았습니다.

## 로그 집계기: 공유하지 말고 각자 세서 합칩니다

로그 줄에서 `[ERROR]` 같은 수준을 뽑아 셉니다. 100만 줄을 4조각으로 나눠, 스레드마다 자기 `BTreeMap` 에 세고, 결과 맵만 채널로 보내 합쳤습니다.

```rust
        for shard in &shards {
            let tx = tx.clone();
            s.spawn(move || {
                let mut local = BTreeMap::new(); // 공유하지 않고 각자 센다
                for line in shard {
                    if let Some(lvl) = level_of(line) {
                        *local.entry(lvl.to_string()).or_insert(0) += 1;
                    }
                }
                tx.send(local).unwrap();
            });
        }
```

```console
100만 줄 집계 26ms: {"ERROR": 200000, "INFO": 600000, "WARN": 200000}
```

공유 `Mutex<BTreeMap>` 에 줄마다 잠금을 걸었다면 15편처럼 다툼이 시간을 먹었을 것입니다. 각자 세면 잠금이 없고, 합치는 일은 조각 수만큼만 합니다. `BTreeMap` 을 쓴 것은 출력 순서가 늘 같아 테스트하기 쉽기 때문입니다(8편).

## 면접에서 이렇게 묻는다

**Q1. 레이트 리미터를 어떻게 테스트하나요?**
시간을 인자로 받습니다. 테스트가 0초, 0.5초, 10초를 직접 넣어 실제로 기다리지 않고 채움과 상한을 확인합니다. 트레이트로 시계를 추상화하는 방법도 있지만, 인자 하나가 가장 단순합니다.

**Q2. Rust 로 LRU 를 만들 때 어려운 점은?**
이중 연결 리스트가 서로를 가리켜 소유권 모델과 맞지 않습니다. `Rc<RefCell>` 과 `Weak`, `unsafe`, 또는 인덱스 연결 리스트가 필요합니다. 이 편은 `HashMap` 과 `BTreeMap` 으로 O(log n) 을 받아들이고 안전성을 택했습니다.

**Q3. 여러 스레드가 카운터를 함께 고쳐야 하면?**
먼저 공유하지 않을 수 있는지 봅니다. 각자 세고 합치면 잠금이 없습니다. 실시간으로 함께 봐야 하면 원자 정수나 `Mutex` 를 쓰고 다툼을 잽니다(15편).

**Q4. 설계 문제에서 Rust 를 고른 이유를 물으면?**
타입과 빌림 검사기가 데이터 경쟁을 컴파일할 때 막고, `Send`·`Sync` 가 스레드 사이의 약속을 드러냅니다. 대신 연결 구조처럼 소유권과 안 맞는 구조는 설계를 바꿔야 합니다. 장단을 함께 말합니다.

**Q5. 이 레이트 리미터를 여러 서버로 늘리려면?**
버킷 상태를 공유 저장소로 옮기고 원자적으로 고쳐야 합니다. 이번 풀이 범위 밖이고, 질문 단계에서 조건을 확인한 이유가 이것입니다.

## 연습

**연습 1.** 사용자별 레이트 리미터 `fn try_acquire(&mut self, user: &str, now: Duration) -> bool` 의 구조를 설계하세요.

<details>
<summary>풀이 보기</summary>

`HashMap<String, TokenBucket>` 을 두고 `entry(user.to_string()).or_insert_with(|| TokenBucket::new(cap, rate))` 로 버킷을 꺼내 `try_acquire(now)` 를 부릅니다. 사용자가 계속 늘면 맵이 커지니, 오래 안 쓴 버킷을 지우는 정리 단계가 필요합니다. 이 편의 LRU 를 그 정리에 쓸 수 있습니다. 이 풀이는 설계이고 구현해 돌리지는 않았습니다.

</details>

**연습 2.** `Lru::new(0)` 을 허용하지 않은 이유는?

<details>
<summary>풀이 보기</summary>

용량 0 이면 `put` 할 때마다 넣은 것을 바로 내보내야 해서 의미가 없고, `pop_first` 와의 상호 작용도 헷갈립니다. 생성 시점에 `assert!(cap > 0)` 으로 막아 불변식을 세웠습니다(7편의 "코드가 보장한 불변식에는 패닉"). 라이브러리로 내보낸다면 `Result` 나 `NonZeroUsize` 를 받는 쪽이 낫습니다(19편).

</details>

## 한 단계 더 — 면접관이 보는 것

시스템 과제에서 정답은 여러 개입니다. 면접관이 보는 것은 판단의 근거입니다. 이 편에서 판단마다 근거를 붙인 곳은 셋입니다. 시간을 인자로 받은 것은 테스트 가능성 때문이었습니다. LRU 에서 O(log n) 을 받아들인 것은 안전성과 구현 시간 때문이었습니다. 로그를 각자 센 것은 15편의 다툼 측정 때문이었습니다. 근거가 측정이나 이전 경험이면 더 설득력이 있습니다. 반대로 "Rust 라서 빠르다"처럼 측정 없는 근거는 되묻기 좋은 빈틈입니다. 이 해석은 이 연재의 측정에서 나온 것이고, 특정 회사의 면접 기준을 근거로 한 것은 아닙니다.

## 정리

시스템 과제는 질문, 구조 선택, 시험 가능한 설계 순서로 풉니다. Rust 에서는 소유권과 맞는 구조를 고르고, 맞지 않으면 교환을 설명합니다. 공유는 마지막 수단입니다.

24편은 연재의 마지막 편입니다. 지금까지의 면접 질문을 주제별 지도로 묶고, 각 질문이 어느 편의 어떤 실험과 이어지는지 정리합니다.
