# Go 강의 19편 — 다른 고루틴이 쓴 값은 동기화를 거쳐야 보인다

> 고루틴 A 가 쓴 값을 고루틴 B 가 언제 반드시 본다고 말할 수 있고, 그 보장을 무엇으로 만듭니까?

A 의 쓰기가 B 의 읽기보다 "먼저 일어남(happens before)"일 때만 반드시 봅니다. 이 관계는 채널 주고받기, 뮤텍스의 Unlock→Lock, `Once`, atomic 연산 같은 동기화 사건으로만 생깁니다. 일반 변수로 깃발을 세워 알리는 코드는 오늘 돌아가도 보장이 없습니다. 이 맥(Apple M4 Pro)에서는 실제로 쓴 순서와 다르게 보였습니다.

> [그림] 왼쪽 배달 로봇이 x = 1 상자를 들고, 가운데 문 위에 atomic ready 등이 켜져 있고 문에 Mutex 자물쇠가 걸려 있다. 문을 지나는 초록 화살표에 먼저 일어남이라고 적혀 있고, 오른쪽 로봇은 x = 1 이 보인다고 말한다

쓴 값은 그림의 문(동기화)을 지나야 건너편에 보입니다. 문을 거치지 않고 읽으면 `-race` 가 잡습니다.

[5편](https://blog.doksam.com/blog/go-course-05-tests-and-concurrency/)에서 경쟁하는 카운터를 `-race` 로 잡고 고쳤습니다. 이 편은 그 이유를 메모리 모델로 설명하고, 도구마다 비용을 잽니다. 출력은 2026-11-18 에 Apple M4 Pro(14코어), macOS, Go 1.26.2 에서 얻었고, 벤치마크는 다른 작업과 함께 돈 상태에서 5번 잰 가운데값입니다.

## 보장은 "먼저 일어남" 하나로 정해진다

Go 1.26.2 배포판에 든 공식 메모리 모델 문서(`doc/go_mem.html`)는 "먼저 일어남"을 같은 고루틴 안의 실행 순서와 동기화 사건의 순서를 이어 붙인 관계로 정의합니다. 일반 읽기 r 이 쓰기 w 를 반드시 보려면 w 가 r 보다 먼저 일어나야 합니다. 그 사이에 다른 쓰기가 끼면 안 됩니다. 동기화 사건은 문서에 이렇게 적혀 있습니다.

| 사건 | 문서의 규칙 |
|---|---|
| 채널 보내기 | 보내기가 짝이 되는 받기의 완료보다 먼저 |
| 채널 닫기 | 닫기가 "닫혀서 제로 값"을 받는 받기보다 먼저 |
| `Mutex` | n번째 `Unlock` 이 m번째(n < m) `Lock` 의 반환보다 먼저 |
| `Once` | `f` 의 완료가 모든 `once.Do(f)` 의 반환보다 먼저 |
| atomic | B 가 A 의 효과를 보면 A 가 B 보다 먼저. 모든 atomic 연산은 하나의 순차 순서로 보인다 |

문서에 따르면 Go 의 atomic 은 C++ 순차 일관(sequentially consistent) atomic, Java `volatile` 과 같은 뜻입니다. 첫머리 조언은 한 줄입니다. "Don't be clever."

## 일반 변수 깃발은 순서를 지키지 않았다

메시지 전달 리트머스 시험(litmus test, 순서 보장을 확인하는 짧은 실험)을 돌렸습니다. 쓰는 쪽은 `x` 다음 `y` 를 쓰고, 읽는 쪽은 `y` 가 1 이 될 때까지 돈 뒤 `x` 를 읽습니다. 아래는 핵심 줄만 옮긴 것입니다.

```go
// 쓰는 고루틴: 매 회차 시작 신호를 받으면
p.x = 1
p.y = 1

// 읽는 고루틴
for p.y == 0 {
}
if p.x == 0 {
	stale++
}
```

회차마다 둘을 0 으로 되돌리고 100만 번 돌린 결과입니다. `-atomic` 은 `y` 만 `atomic.Int64` 로 바꾼 판입니다.

| 판 | 실행 횟수 | y=1 인데 x=0 을 본 실행 | 한 실행의 최대 |
|---|---:|---:|---:|
| 일반 변수 | 15 | 7 | 2번 |
| `y` 만 atomic | 15 | 0 | 0번 |

일반 변수 판은 15번 중 7번, 100만 회차에 1~2번씩 `x` 가 0 이었습니다. 이 arm64 기계에서 일반 변수 두 개가 쓴 순서와 다르게 보인 것입니다. 메모리 모델 문서도 경쟁이 있는 프로그램은 순차 일관이 아닌 실행을 보일 수 있다고 적습니다. atomic 판에서는 `x = 1` 이 `Store` 보다, `Store` 가 그 값을 본 `Load` 보다 먼저 일어나므로 `x` 는 반드시 1 입니다. `-race` 로 돌리면 일반 변수 판은 `Found 3 data race(s)` 로, atomic 판은 경고 없이 끝났습니다.

문서에는 `data = "hello"; ready = true` 로 알리고 `for !ready {}` 로 기다리는 예가 나옵니다. 이 루프는 끝난다는 보장조차 없다고 적혀 있습니다. 같은 모양을 테스트로 만들어 `-race` 로 돌렸습니다(경로는 줄였습니다).

```text
$ RACE_DEMO=1 go test -race .
==================
WARNING: DATA RACE
Write at 0x000100cd7fed by goroutine 8:
  example.com/flagrace.TestFlagPublish.func1()
      …/flag_test.go:19 +0x6c

Previous read at 0x000100cd7fed by goroutine 7:
  example.com/flagrace.TestFlagPublish()
      …/flag_test.go:21 +0x84
…
```

19행 `ready = true` 와 21행 `for !ready {` 가 순서 없이 만났다는 보고입니다. `-race` 없이 돌리면 이 테스트는 통과했습니다. 컴파일러가 루프마다 깃발을 다시 읽는 명령(`MOVBU`)을 만들었기 때문인데, 그 동작은 보장이 아니라 오늘의 구현입니다.

## 고치는 방법은 셋이고 모두 -race 를 통과한다

채널로 값을 넘기거나, 뮤텍스로 감싸거나, 깃발만 atomic 으로 바꿉니다. 셋째가 가장 덜 익숙합니다.

```go
type Flag struct {
	data  string
	ready atomic.Bool // Go 1.19
}

func (f *Flag) Put(s string) {
	f.data = s          // 일반 쓰기
	f.ready.Store(true) // 이 저장을 본 쪽은 위의 쓰기도 본다
}

func (f *Flag) Get() (string, bool) {
	if !f.ready.Load() {
		return "", false
	}
	return f.data, true
}
```

`go test -race -count=3 ./publish` 는 채널·뮤텍스·atomic 세 판 모두 통과했습니다. 깃발을 올린 뒤 `data` 를 다시 고치게 하면 `-race` 가 `DATA RACE` 로 잡았습니다. atomic 깃발은 한 번 내보내는 값에만 맞습니다.

뮤텍스를 든 구조체는 값으로 넘기면 잠금이 복사됩니다. `go vet` 이 `show passes lock by value: example.com/copylock.Box contains sync.Mutex` 로 잡습니다([2편](https://blog.doksam.com/blog/go-course-02-values-and-structs/)). 조건을 기다리는 `sync.Cond` 도 있지만, `go doc sync.Cond` 는 단순한 경우 채널이 낫다고 적습니다. `Broadcast` 는 채널 닫기, `Signal` 은 보내기에 해당합니다.

## -race 는 실행된 접근만 보고 값을 크게 치른다

`src/runtime/race/README` 는 경쟁 탐지기가 LLVM 의 ThreadSanitizer 를 바탕으로 한다고 적습니다. 메모리 모델 문서는 데이터 경쟁을 "먼저 일어남으로 순서가 정해지지 않았고 적어도 하나가 쓰기인 두 접근"으로 정의하고, 탐지기는 실행 중에 그런 짝을 찾아 보고합니다. `go help build` 는 지원 플랫폼을 darwin/arm64, linux/amd64 등으로 적습니다. 같은 벤치마크(뮤텍스로 감싼 맵 조회, 1코어, 300만 회 고정)를 두 바이너리로 돌렸습니다.

| 빌드 | ns/op | 최대 상주 메모리 |
|---|---:|---:|
| 보통 | 8.1 | 6.5MB |
| `-race` | 285.8 | 26.0MB |

약 35배 느리고 약 4배 큽니다. 잠금만 반복하는 루프라 불리한 쪽의 값입니다. 탐지기는 실행된 접근만 봅니다. 5편처럼 경쟁 테스트를 환경변수로 건너뛰면 `ok` 가 나옵니다. 그래서 `-race` 는 동시 경로를 실제로 타는 테스트와 함께 돌려야 의미가 있습니다.

## 데이터 경쟁이 없어도 결과는 틀릴 수 있다

데이터 경쟁(data race)은 동기화 없는 동시 접근이고, 경쟁 상태(race condition)는 실행 순서에 따라 결과가 틀리는 설계 버그입니다. 아래 출금은 모든 접근이 atomic 이라 `-race` 가 조용합니다.

```go
if bal.Load() < amt { // 확인
	return false
}
runtime.Gosched() // 확인과 실행 사이에 다른 고루틴이 끼어들 틈
bal.Add(-amt)     // 실행
```

잔액 100 에서 고루틴 100개가 10씩 빼면 `-race` 아래에서 `-90`, `0`, `-10` 이, `-race` 없이 다섯 번 돌리면 `-40 0 -20 0 0` 이 나왔습니다. 확인과 실행을 `CompareAndSwap` 하나로 묶어야 합니다.

```go
for {
	old := bal.Load()
	if old < amt {
		return false
	}
	if bal.CompareAndSwap(old, old-amt) {
		return true
	}
}
```

이 판은 200번 반복해 모두 잔액 0, 성공 10건이었고 `-race` 도 통과했습니다.

## RWMutex 는 읽기 구역이 길 때만 이겼다

1,024키 맵을 `b.RunParallel` 로 읽었습니다. 짧은 읽기는 키 1개, 긴 읽기는 키 64개를 잠근 채 읽습니다(ns/op, 가운데값).

| 경우 | 고루틴 | Mutex | RWMutex |
|---|---:|---:|---:|
| 짧은 읽기만 | 1 | 7.8 | 7.6 |
| 짧은 읽기만 | 14 | 109.2 | 144.9 |
| 긴 읽기만 | 1 | 244.6 | 238.9 |
| 긴 읽기만 | 14 | 313.6 | 176.4 |
| 긴 읽기 + 쓰기 10% | 14 | 310.2 | 289.3 |

경합이 없으면 둘은 같습니다. 짧은 구역에서는 RWMutex 가 더 느렸습니다. `RLock` 은 모든 읽는 쪽이 같은 카운터를 `readerCount.Add(1)` 로 고칩니다(`src/sync/rwmutex.go`, Go 1.26.2 소스 기준). 구역이 짧으면 이 카운터 다툼이 비용의 대부분입니다. 짧은 읽기에 쓰기 10% 를 섞은 경우는 RWMutex 가 4.7배 빨랐는데, 원인을 이 실험으로 가려내지 못해 표에서 뺐습니다. RWMutex 는 재 보고 고릅니다.

## sync.Map 은 두 경우에만 쓴다

`src/sync/map.go` 주석은 대부분의 코드는 일반 맵과 별도 잠금을 쓰라고 하고, 두 경우만 꼽습니다. 한 번 쓰고 여러 번 읽는 커지기만 하는 캐시, 그리고 고루틴마다 겹치지 않는 키를 다루는 경우입니다. 첫째 경우를 14고루틴으로 쟀습니다.

| 방식 | ns/op |
|---|---:|
| RWMutex + map | 130.5 |
| `sync.Map` | 1.11 |

Go 1.26.2 소스의 `sync.Map` 은 `internal/sync` 의 `HashTrieMap` 위에 있고, 그 주석은 잦은 읽기를 중심으로 설계했다고 적습니다. 대신 값이 `any` 라 타입 검사가 빠지고 길이를 바로 알 수 없습니다.

## Once 는 결과뿐 아니라 실패도 기억한다

제 데이터 수집 도구는 실행 파일에 넣은 글꼴을 처음 쓸 때 한 번만 풉니다. `sync.OnceValues`(Go 1.21)로 줄이면 이렇습니다.

```go
func NewLoader(extract func() (string, error)) func() (string, error) {
	return sync.OnceValues(func() (string, error) {
		Calls.Add(1)
		return extract()
	})
}
```

고루틴 100개가 동시에 불러도 `extract` 는 1번 돌았습니다. 첫 호출이 실패하면 두 번째 호출도 같은 에러를 돌려주고 다시 시도하지 않았습니다. `go doc sync.OnceValues` 는 `f` 가 패닉하면 호출마다 같은 값으로 패닉한다고 적습니다. 실패 뒤 다시 시도해야 하면 연습 1처럼 직접 씁니다. 고루틴을 기다릴 때는 Go 1.25 의 `wg.Go(func(){…})` 가 `Add`·`Done` 짝을 대신합니다.

## 한 단계 더 — 캐시 줄과 뮤텍스의 두 모드

**같은 캐시 줄의 다른 변수도 서로 막습니다.** 고루틴마다 자기 카운터만 올리게 하고, 카운터를 붙여 둔 판과 128바이트씩 띄운 판을 비교했습니다. `src/internal/cpu/cpu_arm64.go` 의 `CacheLinePadSize` 가 128 입니다.

| 배치 | ns/op (14고루틴) |
|---|---:|
| 붙여 둠 | 46.15 |
| 128바이트 띄움 | 0.144 |

같은 값을 쓰지 않아도 같은 캐시 줄이면 코어끼리 줄을 주고받습니다(거짓 공유, false sharing). 고루틴별 통계를 배열로 둘 때 확인합니다.

**뮤텍스에는 두 모드가 있습니다.** Go 1.26.2 의 `sync.Mutex` 는 `internal/sync.Mutex` 를 감싸고, 그 소스 주석은 이렇게 설명합니다. 보통 모드에서는 깨어난 대기자가 새로 온 고루틴과 경쟁하고 대개 집니다. 1ms 넘게 못 잡은 대기자가 생기면 굶주림 모드로 바꿔 잠금을 맨 앞 대기자에게 바로 넘깁니다. 처리량은 보통 모드가 좋고, 꼬리 지연은 굶주림 모드가 막습니다.

| 공유 방식 | 도구 | 이 편의 측정 | 주의 |
|---|---|---|---|
| 값을 넘긴다 | 채널 | — | 받은 쪽만 고친다 |
| 여러 필드를 함께 고친다 | `Mutex` | 짧은 구역 109 ns/op | 구역을 짧게, 복사 금지 |
| 긴 읽기 위주 | `RWMutex` | 176 대 314 ns/op | 짧은 구역에서는 더 느렸다 |
| 커지기만 하는 캐시 | `sync.Map` | 1.11 대 130.5 ns/op | `any` 타입 |
| 숫자·깃발 하나 | atomic | — | 여러 값의 일관성은 못 지킨다 |
| 한 번 초기화 | `OnceValue(s)` | 호출 1회 | 실패도 기억한다 |

## 면접에서 이렇게 묻는다

**Q1. happens-before 가 무엇이고 왜 필요합니까?**
한 고루틴의 쓰기를 다른 고루틴이 반드시 보는 조건입니다. 채널, 뮤텍스, `Once`, atomic 같은 동기화 사건이 관계를 만듭니다. 일반 변수 깃발이 arm64 에서 순서를 지키지 않은 실험을 근거로 듭니다.

**Q2. 데이터 경쟁과 경쟁 상태는 어떻게 다릅니까?**
데이터 경쟁은 동기화 없는 동시 접근이라 `-race` 가 잡습니다. 경쟁 상태는 확인과 실행 사이가 끊긴 설계 버그라 atomic 만 써도 생기고 `-race` 가 못 잡습니다. CAS 나 한 번의 잠금으로 묶어 고칩니다.

**Q3. `-race` 는 어떻게 동작하고 한계는 무엇입니까?**
ThreadSanitizer 기반으로 실행된 접근을 기록해 먼저 일어남으로 이어지지 않은 쓰기를 찾습니다. 실행되지 않은 경로는 못 보고, 측정한 루프에서 약 35배 느리고 4배 컸습니다.

**Q4. atomic 연산은 어떤 순서를 보장합니까?**
Go 의 atomic 은 모두 하나의 순차 순서로 보이고, 효과를 본 쪽에는 먼저 일어남이 성립합니다. 그래서 atomic 깃발 하나로 그 앞의 일반 쓰기도 내보낼 수 있고, 리트머스 시험의 atomic 판은 15번 모두 0회였습니다. 다만 값 두 개를 함께 바꾸는 일관성은 atomic 하나로 못 지킵니다.

**Q5. 읽기가 많으면 RWMutex 가 항상 낫습니까?**
아닙니다. 짧은 구역에서는 읽는 쪽 카운터 다툼 때문에 Mutex 보다 느렸고, 키 64개를 읽는 긴 구역에서만 1.8배 빨랐습니다. 재 보고 고릅니다.

## 연습

### 연습 1 — 실패하면 다시 시도하는 한 번 초기화

`OnceValues` 는 첫 실패를 기억합니다. 성공할 때까지만 다시 시도하고, 성공한 뒤에는 그 값만 돌려주는 `Loader` 를 만듭니다.

<details>
<summary>풀이</summary>

```go
type Loader struct {
	mu   sync.Mutex
	val  string
	done bool
	load func() (string, error)
}

func (l *Loader) Get() (string, error) {
	l.mu.Lock()
	defer l.mu.Unlock()
	if l.done {
		return l.val, nil
	}
	v, err := l.load()
	if err != nil {
		return "", err
	}
	l.val, l.done = v, true
	return v, nil
}
```

첫 호출이 실패하고 이어서 고루틴 50개가 부르는 테스트에서 `load` 는 정확히 2번 돌았고 `-race` 도 통과했습니다. 뮤텍스를 쥔 채 `load` 를 부르므로 느린 `load` 는 다른 호출을 기다리게 합니다.

</details>

### 연습 2 — 최댓값 기록기

여러 고루틴이 보고한 값의 최댓값을 `atomic.Int64` 에 남깁니다. `if v > m.Load() { m.Store(v) }` 가 왜 틀리는지 보이고 고칩니다.

<details>
<summary>풀이</summary>

확인과 저장 사이에 다른 고루틴이 더 큰 값을 넣으면 작은 값으로 덮입니다. 0~99 를 고루틴 100개가 보고하는 시험을 200번 돌리면 `-race` 는 조용한데 최댓값 99 를 잃은 횟수가 90, 96, 104번이었습니다.

```go
func Record(m *atomic.Int64, v int64) {
	for {
		old := m.Load()
		if v <= old || m.CompareAndSwap(old, v) {
			return
		}
	}
}
```

이 판은 200번 모두 99 였고 `-race` 를 통과했습니다.

</details>

## 정리

다른 고루틴이 쓴 값은 동기화 사건이 만든 "먼저 일어남"을 거칠 때만 반드시 보입니다. 일반 변수 깃발은 arm64 에서 실제로 순서가 뒤집혔고, 채널·뮤텍스·atomic 으로 고친 판은 `-race` 를 통과했습니다. 도구는 공유 방식으로 고르고, RWMutex·sync.Map 처럼 성능을 기대하는 도구는 재 보고 씁니다.

다음은 [로드맵](https://blog.doksam.com/blog/go-course-00-roadmap/)의 20편 제네릭입니다. 타입 매개변수가 필요한 곳과 아닌 곳을 가르고, 컴파일러가 제네릭 코드를 어떻게 만드는지 봅니다.
