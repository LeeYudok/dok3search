# Go 강의 17편 — 고루틴은 P 개수만큼 동시에 돌고, 막히면 자리를 내준다

> 고루틴 10만 개는 스레드 몇 개 위에서 돌고, 하나가 CPU 를 붙잡거나 시스템 콜에 막히면 나머지는 어떻게 됩니까?

Go 코드를 동시에 실행하는 자리(P)는 `GOMAXPROCS` 개이고, 스레드(M)가 이 자리를 잡아야 고루틴(G)이 돕니다. 이 맥에서 고루틴 1만 개를 돌렸을 때 스레드는 15개였습니다. CPU 를 붙잡은 고루틴은 런타임이 신호로 끊어 자리를 넘깁니다. 시스템 콜에 막힌 고루틴은 스레드를 붙든 채 자리만 내놓고, 런타임이 그 자리에 새 스레드를 붙입니다. 그래서 어디서 막히느냐에 따라 스레드 수가 달라집니다.

> [그림] 전역 큐 통에 G 상자가 쌓여 있고 배달 로봇이 상자를 들고 간다. 작업대 P1·P2·P3 위에 천장 집게 M1·M2·새 M 이 G 를 하나씩 집고, 시스템 콜 부스에는 M3 와 G 가 갇혀 있다

그림의 작업대가 P, 천장 집게가 M, 상자가 G 입니다. 시스템 콜 부스에 들어간 M3 가 두고 간 P3 에는 새 M 이 내려와 일을 잇습니다.

[5편](https://blog.doksam.com/blog/go-course-05-tests-and-concurrency/)에서 고루틴을 띄우는 법을 봤고, 이 편은 그 안을 봅니다. 출력은 2026-11-18 에 맥(Apple M4 Pro 14코어)과 Go 1.26.2 로 돌렸습니다. 다른 작업과 함께 돈 상태입니다.

## 고루틴은 P 를 잡은 M 위에서만 돈다

런타임 소스 `runtime/proc.go` 첫머리 주석(Go 1.26.2 소스 기준)이 세 글자를 정의합니다.

```text
// G - goroutine.
// M - worker thread, or machine.
// P - processor, a resource that is required to execute Go code.
//     M must have an associated P to execute Go code, however it can be
//     blocked or in a syscall w/o an associated P.
```

| 이름 | 무엇 | 몇 개 |
|---|---|---|
| G | 고루틴. 스택과 실행 상태 | 만든 만큼 |
| M | OS 스레드 | 필요할 때 늘고, 기본 상한은 10,000 (`go doc runtime/debug.SetMaxThreads`) |
| P | Go 코드를 실행할 자리와 로컬 실행 큐(256칸, `runtime/runtime2.go`) | `GOMAXPROCS` 개 |

```text
$ go run ./cmd/procs
NumCPU     = 14
GOMAXPROCS = 14
```

`GOMAXPROCS` 기본값은 논리 CPU 수입니다. 리눅스 컨테이너에서 cgroup CPU 상한도 따지는 동작은 Go 1.25 에 들어왔습니다(`doc/godebug.md`). 조건은 면접 Q2 에 적었습니다.

## schedtrace 로 줄 선 고루틴을 센다

고루틴 1만 개를 모두 만든 뒤 한꺼번에 풀어 2초씩 CPU 를 쓰게 했습니다. `GODEBUG=schedtrace=500` 은 0.5초마다 스케줄러 상태를 한 줄씩 찍습니다.

```go
start := make(chan struct{})
var wg sync.WaitGroup
for range 10_000 {
	wg.Go(func() {
		<-start
		spin(time.Now().Add(2 * time.Second)) // 2초 동안 시각만 읽는 루프
	})
}
close(start)
wg.Wait()
```

```text
$ GOMAXPROCS=4 GODEBUG=schedtrace=500 ./schedtrace
SCHED 0ms: gomaxprocs=4 idleprocs=1 threads=5 spinningthreads=1 needspinning=0 idlethreads=1 runqueue=0 [ 0 0 0 0 ] schedticks=[ 1 0 0 2 ]
SCHED 513ms: gomaxprocs=4 idleprocs=0 threads=5 spinningthreads=0 needspinning=1 idlethreads=0 runqueue=9491 [ 110 110 111 174 ] schedticks=[ 1211 2126 3966 2793 ]
…
SCHED 3062ms: gomaxprocs=4 idleprocs=0 threads=5 spinningthreads=0 needspinning=1 idlethreads=0 runqueue=9832 [ 25 25 25 89 ] schedticks=[ 1298 2213 4053 2880 ]
```

`runqueue` 는 전역 큐, 대괄호 안은 P 마다의 로컬 큐 길이입니다. 513ms 줄을 더하면 9,491 + 505(로컬 넷) + 4(실행 중) = 10,000 입니다. 고루틴 1만 개가 스레드 5개 위에 있습니다. `GOMAXPROCS` 를 14로 두면 `threads=15` 였습니다.

시간이 갈수록 전역 큐는 늘고 로컬 큐는 줄었습니다. 선점된 고루틴은 전역 큐로 가기 때문입니다(`proc.go` 의 `goschedImpl` 이 `globrunqput` 을 부름). P 는 로컬 큐를 먼저 쓰고, 61번에 한 번은 전역 큐를 봅니다. 주석은 이유를 "fairness"(공정성)라고 적습니다.

## 고루틴 하나는 약 2KiB 스택으로 시작한다

채널에서 기다리는 고루틴 10만 개를 만들고 `runtime.MemStats` 를 비교했습니다. 세 번 돌린 결과입니다.

```text
$ ./stackmem
고루틴 100001개, 만드는 데 84ms
StackInuse  +200256 KiB → 개당 2050 바이트
Sys         +271456 KiB → 개당 2779 바이트
```

스택은 개당 2,050바이트, OS 메모리는 개당 약 2.8KB(2,779·2,819·2,784바이트) 늘었습니다. 만드는 데 72~84ms 가 걸렸습니다. `runtime/stack.go` 의 `stackMin = 2048` 이 이 값입니다. 같은 파일 주석에 따르면 시작 크기는 GC 마다 살펴본 스택 평균으로 조정됩니다(`GODEBUG` 의 `adaptivestackstart`, 기본 켜짐). 스택이 모자라면 런타임이 더 큰 스택으로 복사해 옮깁니다(`stack.go` 의 `copystack`).

## CPU 를 붙잡는 고루틴은 신호로 끊긴다

P 를 하나만 두고 함수 호출이 없는 무한 루프를 띄웠습니다. main 이 100ms 자고 깨는지 봅니다.

```go
runtime.GOMAXPROCS(1)
go func() {
	x := 0
	for {
		x++ // 함수 호출이 없는 루프
	}
}()
t0 := time.Now()
time.Sleep(100 * time.Millisecond)
fmt.Printf("main 이 다시 돌았다: %v 뒤\n", time.Since(t0).Round(time.Millisecond))
```

```text
$ ./preempt          # 세 번
main 이 다시 돌았다: 103ms 뒤
main 이 다시 돌았다: 107ms 뒤
main 이 다시 돌았다: 107ms 뒤
$ ./run-asyncoff.sh  # GODEBUG=asyncpreemptoff=1, 3초 뒤 강제 종료
3초가 지나도 main 이 돌지 못했다
```

`runtime/preempt.go` 주석은 선점 지점을 두 가지로 나눕니다. 동기 선점은 함수 첫머리의 스택 검사를 이용하므로 함수를 부르지 않는 루프에서는 일어나지 않습니다. 비동기 선점은 OS 신호로 스레드를 세워 어느 명령에서든 끊습니다. `asyncpreemptoff=1` 로 이것을 끄자 main 은 3초 동안 P 를 얻지 못했습니다. 선점 기준 시간은 10ms(`proc.go` 의 `forcePreemptNS`)입니다.

## 시스템 콜에 막히면 스레드가 늘고, 채널·타이머·네트워크는 그렇지 않다

고루틴을 세 가지 방식으로 막아 두고, 런타임이 만든 스레드 수를 `pprof.Lookup("threadcreate").Count()` 로 셌습니다.

```go
case "sleep": // 타이머에서 기다린다
	go time.Sleep(time.Hour)
case "pipe": // os.File 읽기: 런타임의 네트워크 폴러가 기다린다
	r, _, err := os.Pipe()
	if err != nil {
		panic(err)
	}
	go r.Read(make([]byte, 1))
case "syscall": // 시스템 콜 read 에서 스레드째 막힌다
	var fds [2]int
	if err := syscall.Pipe(fds[:]); err != nil {
		panic(err)
	}
	go syscall.Read(fds[0], make([]byte, 1))
```

| 막힌 곳 | 고루틴 | 만든 스레드(전 → 후) |
|---|---:|---|
| `time.Sleep` | 100 | 5 → 6~7 |
| `os.File` 파이프 읽기 | 100 | 5 → 5~7 |
| `syscall.Read` | 100 | 5 → 103 (세 번 같음) |
| `syscall.Read`, `GOMAXPROCS=2` | 200 | 4 → 203 |
| `time.Sleep`, `GOMAXPROCS=2` | 10,000 | 4 → 4 |

시스템 콜에 막힌 고루틴은 하나당 스레드 하나를 늘렸고, `GOMAXPROCS` 와 상관이 없었습니다. `os.File` 파이프는 OS 이벤트 알림(맥은 kqueue)에 맡겨서 고루틴만 잠들고 스레드는 다른 일을 합니다. `go doc runtime/debug.SetMaxThreads` 도 새 스레드는 기존 스레드가 모두 시스템 콜, cgo 호출, `LockOSThread` 로 묶였을 때만 만든다고 적습니다.

`runtime.Gosched` 는 고루틴을 멈추지 않고 자리만 양보합니다. 비동기 선점이 있어 부를 일은 드뭅니다. `runtime.LockOSThread` 는 고루틴을 지금 스레드에 묶어, 스레드별 상태에 기대는 OS 기능이나 C 라이브러리를 부를 때 씁니다. 풀지 않고 끝나면 그 스레드도 종료됩니다(두 함수의 `go doc`).

## 고루틴 누수는 개수와 프로파일로 찾는다

제한 시간이 지나면 포기하는 함수입니다. 결과를 보내려던 고루틴은 받을 쪽이 사라져 영원히 막힙니다.

```go
func Leaky(ctx context.Context, d time.Duration) (int, error) {
	ch := make(chan int)
	go func() { ch <- slow(d) }()
	select {
	case v := <-ch:
		return v, nil
	case <-ctx.Done():
		return 0, ctx.Err()
	}
}
```

50ms 걸리는 호출을 10ms 제한으로 1,000번 불렀습니다(`-trimpath` 빌드, 주소 열은 줄임).

```text
$ ./leak goroutine 1
시작: 1
200ms 뒤: 1001
goroutine profile: total 1001
1000 @ 0x104d8953c …
#	0x104ddc987	example.com/sched/internal/fetch.Leaky.func1+0x37	example.com/sched/internal/fetch/fetch.go:19
$ ./leak goroutine 2 | grep -A2 -m1 "chan send"
goroutine 17 [chan send]:
example.com/sched/internal/fetch.Leaky.func1()
	example.com/sched/internal/fetch/fetch.go:19 +0x38
```

같은 줄(`fetch.go:19`)에 고루틴 1,000개가 `[chan send]` 로 서 있습니다. 운영 서버라면 `runtime.NumGoroutine` 을 지표로 내보내고, 계속 오르면 이 프로파일을 봅니다.

Go 1.26.2 에는 누수만 골라 주는 `goroutineleak` 프로파일이 실험 기능(GOEXPERIMENT)으로 들어 있습니다. 기본값 목록(`internal/buildcfg/exp.go`)에는 없고, 켜고 빌드해야 생깁니다.

```text
$ GOEXPERIMENT=goroutineleakprofile go build -trimpath -o leak-exp ./cmd/leak
$ ./leak-exp goroutineleak 1
…
goroutineleak profile: total 1000
1000 @ 0x100c25cbc …
#	0x100c79357	example.com/sched/internal/fetch.Leaky.func1+0x37	example.com/sched/internal/fetch/fetch.go:19
```

이번에는 main 을 빼고 막힌 1,000개만 나왔습니다. 기본 빌드에서는 `pprof.Lookup("goroutineleak")` 이 nil 이라 그대로 부르면 패닉이 납니다. `runtime/mgcmark.go` 주석에 따르면 이 검사용 GC 는 실행할 수 있는 고루틴만 뿌리로 삼아 표시합니다. 주석으로 보면, 그 뿌리에서 닿지 않는 채널에 막힌 고루틴을 누수로 고르는 방식입니다.

## 한 단계 더 — 남의 큐에서 훔치고, 막힌 스레드에서 자리를 되찾는다

일이 없는 P 는 다른 P 의 큐에서 훔칩니다. `findRunnable` 주석은 이 함수가 남의 P 에서 훔치고, 로컬·전역 큐를 보고, 네트워크를 폴링한다고 적습니다. `runqsteal` 은 상대 로컬 큐의 절반을 가져옵니다. 일을 찾아 도는 스레드를 "spinning"이라 부릅니다. `proc.go` 첫머리 주석은 spinning 스레드가 하나라도 있으면 새 스레드를 깨우지 않는다고 적습니다. schedtrace 의 `spinningthreads` 가 그 수입니다.

시스템 콜에서 자리를 되찾는 쪽은 P 없이 도는 감시 스레드 `sysmon` 입니다. 할 일이 남았는데 시스템 콜에 20µs 넘게 머문 P 는 떼어 다른 M 에 넘기고, 10ms 넘게 도는 고루틴에는 선점을 요청합니다(`proc.go` 의 `retake`). 앞 실험에서 `syscall.Read` 마다 스레드가 늘어난 까닭입니다.

| 막히는 곳 | 스레드가 느는가 | 대처 |
|---|---|---|
| 채널·뮤텍스·`select` | 아니다. G 만 잠든다 | 누수만 조심한다 |
| 타이머·`time.Sleep` | 아니다(실측 +1~2) | 없음 |
| 네트워크·파이프(폴러) | 아니다(실측 0~2) | 없음 |
| 블로킹 시스템 콜·cgo | 막힌 G 하나당 하나(실측 100 → +98) | 동시 호출 수를 세마포어로 제한한다 |
| CPU 를 오래 쓰는 루프 | 아니다. 10ms 마다 선점 | `GOMAXPROCS` 이상 병렬로 돌지 않는다 |
| `LockOSThread` | 묶인 G 마다 하나 | 꼭 필요한 곳에만 쓴다 |

## 면접에서 이렇게 묻는다

**Q1. G·M·P 는 무엇이고, P 는 왜 따로 있습니까?**
G 는 고루틴, M 은 OS 스레드, P 는 Go 코드를 실행할 자리와 로컬 실행 큐입니다. M 이 시스템 콜로 막히면 P 만 떼어 다른 M 에 붙일 수 있어서, 고루틴 수와 스레드 수가 따로 놉니다. 실측으로 고루틴 1만 개가 `GOMAXPROCS=4` 에서 스레드 5개 위에 있었다고 덧붙입니다.

**Q2. 컨테이너에서 `GOMAXPROCS` 는 어떻게 정해집니까?**
Go 1.25 부터 리눅스에서는 CPU 수, CPU 친화도, cgroup 상한(`cpu.max`) 중 가장 작은 값을 씁니다. CPU 가 둘 이상이면 2 아래로 내리지 않습니다(`go doc runtime.GOMAXPROCS`). `go.mod` 언어 버전이 1.24 이하면 `containermaxprocs=0` 이 기본이라 CPU 수를 씁니다. 환경변수로 고정하면 자동 갱신이 꺼집니다.

**Q3. `for {}` 로 도는 고루틴이 다른 고루틴을 굶깁니까?**
지금 Go 에서는 아닙니다. 런타임이 10ms 넘게 도는 고루틴을 신호로 끊는 비동기 선점이 있습니다. `GOMAXPROCS=1` 실험에서 main 은 103~107ms 에 깨어났고, `asyncpreemptoff=1` 로 끄면 3초 동안 깨지 못했습니다.

**Q4. 고루틴 10만 개의 비용은 얼마입니까?**
스택은 2KiB 로 시작합니다(`stackMin = 2048`). 실측으로 10만 개에 스택이 개당 2,050바이트, OS 메모리가 개당 약 2.8KB 늘었습니다. 다만 블로킹 시스템 콜에 막히면 고루틴마다 스레드가 붙으니, 개수보다 막히는 곳이 중요하다고 답합니다.

**Q5. 고루틴 누수는 어떻게 찾습니까?**
`runtime.NumGoroutine` 이 계속 오르는지 보고, goroutine 프로파일에서 같은 줄에 몰린 스택과 `[chan send]` 같은 상태를 찾습니다. 테스트에서는 호출 전후 고루틴 수를 비교합니다. Go 1.26 에는 실험 기능으로 `goroutineleak` 프로파일도 있습니다.

## 연습

### 연습 1 — 누수를 고치고 테스트로 막기

`Leaky` 를 고쳐 제한 시간이 지나도 고루틴이 남지 않게 하고, 호출 전후 고루틴 수를 비교하는 테스트를 씁니다.

<details>
<summary>풀이</summary>

결과 칸을 하나 둡니다. 받는 쪽이 떠나도 보내는 고루틴은 칸에 값을 넣고 끝납니다.

```go
ch := make(chan int, 1) // Leaky 에서 이 줄만 바꾼다
```

테스트는 100번 부른 뒤 고루틴 수가 돌아올 때까지 최대 1초 기다립니다.

```go
func settle(t *testing.T, base int) {
	t.Helper()
	deadline := time.Now().Add(time.Second)
	for runtime.NumGoroutine() > base && time.Now().Before(deadline) {
		time.Sleep(10 * time.Millisecond)
	}
	if got := runtime.NumGoroutine(); got > base {
		t.Errorf("고루틴 = %d, 원하는 값 %d 이하", got, base)
	}
}
```

```text
$ go test -race -count=1 ./internal/fetch/
ok  	example.com/sched/internal/fetch	1.469s
$ LEAK_DEMO=1 go test -count=1 -run Leaky ./internal/fetch/
--- FAIL: TestLeakyLeaks (1.11s)
    fetch_test.go:32: 고루틴 = 102, 원하는 값 2 이하
```

고친 함수는 통과하고, 고치기 전 함수는 100개가 남아 실패합니다.

</details>

### 연습 2 — 스레드 상한에 걸리면 어떻게 되는가

`debug.SetMaxThreads(50)` 을 두고 `syscall.Read` 에 막힌 고루틴 100개를 만들면 어떻게 되는지 예상하고 돌려 봅니다.

<details>
<summary>풀이</summary>

```text
$ ./threads -mode syscall -n 100 -max 50
runtime: program exceeds 50-thread limit
fatal error: thread exhaustion
…
$ echo $?
2
```

에러가 아니라 프로그램 전체가 죽습니다(`go doc runtime/debug.SetMaxThreads`). 기본 상한 10,000 에서도 같습니다. 블로킹 시스템 콜이나 cgo 를 많이 부르면 버퍼 있는 채널(세마포어)로 동시 호출 수를 막아 둡니다.

</details>

## 정리

고루틴은 `GOMAXPROCS` 개의 자리(P)를 돌아가며 쓰고, 스레드(M)는 자리를 잡아야 고루틴을 돌립니다. CPU 를 붙잡은 고루틴은 10ms 단위로 선점됩니다. 블로킹 시스템 콜에 막힌 고루틴은 자리를 내놓는 대신 스레드를 하나씩 붙듭니다. 그래서 고루틴 수보다 어디서 막히는지를 먼저 봅니다.

다음 편은 [로드맵](https://blog.doksam.com/blog/go-course-00-roadmap/)의 18편, 채널 내부와 `select`, `context` 전파입니다. 이 편의 누수를 만든 채널이 안에서 어떻게 막히고 깨어나는지 봅니다.
