# Go 강의 18편 — 채널은 받을 자리가 없으면 막히고, 취소는 부모에서 자식으로만 내려간다

> 채널은 언제 막히고 언제 패닉을 내며, 취소 신호는 고루틴 나무를 어떻게 타고 내려갑니까?

채널은 값을 놓을 자리가 없으면 막힙니다. 무버퍼 채널은 받는 쪽이 올 때까지, 버퍼 채널은 버퍼가 찰 때까지 막히지 않습니다. 패닉은 세 경우뿐입니다. 닫힌 채널에 보내기, 닫힌 채널 닫기, nil 채널 닫기입니다. context 의 취소는 부모에서 자식으로만 퍼지고, 자식을 취소해도 부모와 형제는 살아 있습니다. 이 편은 Go 1.26.2 런타임 소스와 직접 돌린 실험으로 이 규칙을 확인합니다.

> [그림] 버퍼 네 칸이 가득 찬 컨베이어 앞에서 배달 로봇과 뒤의 작은 로봇이 줄을 서 있고, 받는 자리는 비어 있다. 위에는 빨간 취소 끈이 부모 등에서 자식 등 셋으로 이어져 있다

그림은 버퍼가 가득 찬 채널입니다. 버퍼에 값이 남아 있으면 받는 줄은 비어 있다는 것이 런타임의 불변식이라 받는 자리를 비워 두었습니다.

팬아웃, 버퍼로 막은 고루틴 누수, `wg.Go` 는 [5편](https://blog.doksam.com/blog/go-course-05-tests-and-concurrency/)에서 다뤘습니다. 고루틴이 스레드 위에서 도는 방식은 [로드맵의 17편](https://blog.doksam.com/blog/go-course-00-roadmap/)이 맡습니다. 출력은 2026-11-18 에 맥(Apple M4 Pro)과 Go 1.26.2 로 돌린 결과입니다.

## 채널은 잠금 하나와 고리 버퍼, 대기 줄 둘이다

`make(chan T, n)` 이 만드는 것은 런타임의 `hchan` 구조체입니다. Go 1.26.2 소스 `runtime/chan.go` 에서 필드 일부를 옮깁니다.

```go
type hchan struct {
	qcount   uint           // total data in the queue
	dataqsiz uint           // size of the circular queue
	buf      unsafe.Pointer // points to an array of dataqsiz elements
	closed   uint32
	sendx    uint   // send index
	recvx    uint   // receive index
	recvq    waitq  // list of recv waiters
	sendq    waitq  // list of send waiters
	lock mutex
}
```

`buf` 는 `dataqsiz` 칸짜리 고리 버퍼이고 `sendx`·`recvx` 가 쓰고 읽을 칸을 가리킵니다. 자리가 없어 막힌 고루틴은 `sendq` 나 `recvq` 에 줄을 섭니다. 모든 필드는 `lock` 하나가 지킵니다. 파일 머리 주석은 두 불변식을 적습니다. 버퍼에 값이 있으면(`qcount > 0`) `recvq` 는 비어 있고, 버퍼에 빈 칸이 있으면 `sendq` 가 비어 있습니다.

무버퍼 채널은 버퍼를 거치지 않습니다. 보내는 함수 `chansend` 의 주석은 기다리는 수신자가 있으면 값을 버퍼를 건너뛰고 바로 넘긴다고 적습니다. 값은 수신자의 스택에 직접 복사됩니다. 메모리 모델 문서(`doc/go_mem.html`)는 "무버퍼 채널의 받기는 대응하는 보내기가 끝나기 전에 일어난다"고 정합니다. 무버퍼 채널은 두 고루틴이 만나는 동기화 지점입니다.

## 막히는 경우와 패닉 나는 경우는 표 하나로 정리된다

100ms 안에 끝나지 않으면 막혔다고 보는 도우미 `blocks` 로 경우마다 돌렸습니다.

```text
nil 채널 보내기 막힘: true
nil 채널 받기 막힘: true
무버퍼 채널, 받는 쪽 없이 보내기 막힘: true
버퍼 채널 len, cap: 1 2
가득 찬 버퍼에 보내기 막힘: true
닫힌 뒤 남은 값: 10 true
닫힌 뒤 남은 값: 20 true
다 비운 닫힌 채널: 0 false
```

패닉 경우는 따로 만든 모듈에서 돌렸고, 메시지는 `runtime/chan.go` 의 `panic(plainError(...))` 문자열과 같습니다.

| 연산 | nil 채널 | 열린 채널 | 닫힌 채널 |
|---|---|---|---|
| 보내기 | 영원히 막힘 | 자리가 없으면 막힘 | `panic: send on closed channel` |
| 받기 | 영원히 막힘 | 값이 없으면 막힘 | 남은 값을 받고, 다 비우면 제로 값과 `false` |
| `close` | `panic: close of nil channel` | 성공 | `panic: close of closed channel` |

함정이 하나 더 있습니다. 처음 실험에서 보내다 막힌 고루틴을 둔 채 채널을 닫았더니 `send on closed channel` 로 죽었습니다. `closechan` 은 막혀 있던 송신자를 모두 깨웁니다. 소스 주석대로 깨어난 송신자는 패닉을 냅니다("release all writers (they will panic)"). 그래서 채널은 보내는 쪽이 닫고, 보내는 쪽이 여럿이면 모두 끝난 뒤 한 곳에서 닫습니다. 모든 고루틴이 채널에서 막히면 런타임이 `fatal error: all goroutines are asleep - deadlock!` 으로 멈춥니다.

## select 는 준비된 case 가운데 무작위로 고른다

두 채널에 값을 넣어 둘 다 준비시키고 `select` 를 10만 번 돌렸습니다.

```go
for range 100_000 {
	a <- 1
	b <- 1
	select {
	case <-a:
		countA++
		<-b
	case <-b:
		countB++
		<-a
	}
}
```

```text
a: 50007 b: 49993
a: 49784 b: 50216
a: 49791 b: 50209
```

명세(`doc/go_spec.html`)는 진행할 수 있는 통신이 여럿이면 균등한 의사 난수로 하나를 고른다고 정합니다. 런타임 `select.go` 는 case 순서를 `cheaprandn` 으로 섞어 이 규칙을 지킵니다. 코드 순서로 우선순위를 줄 수 없다는 뜻이고, 우선순위가 필요하면 연습 2처럼 두 단계로 나눕니다.

`default` 가 있으면 `select` 는 막히지 않습니다. 버퍼 3칸짜리 채널에 10개를 막히지 않게 보내면 `버퍼에 남은 수: 3 버린 수: 7` 이 나왔습니다. 지표처럼 늦게 보낼 바에는 버려도 되는 값에 씁니다.

## 닫힌 채널은 nil 로 바꿔 select 에서 뺀다

두 채널을 하나로 합칠 때, 닫힌 채널을 그대로 두면 `select` 는 그 채널에서 제로 값을 계속 받습니다. 하나는 바로 닫히고 하나는 20ms 간격으로 5개를 보내게 두고 두 방식을 비교했습니다.

```go
case v, ok := <-a:
	if !ok {
		a = nil // 이 case 를 끈다
		continue
	}
```

```text
표시만 할 때: 받은 값 6 닫힘 확인 3969391
nil 로 끌 때: 받은 값 6 닫힘 확인 2
```

표시만 해 두면 `b` 가 5개를 보내는 약 0.1초 동안 닫힌 `a` 를 397만 번 확인하며 CPU 를 태웠습니다. 명세는 nil 채널이 통신할 준비가 되지 않는다고 정하고, `select.go` 는 채널이 nil 인 case 를 고를 목록에서 뺍니다. 같은 3번 실행에서 헛돈 횟수는 380만~397만 번이었습니다.

## 타이머 채널은 Go 1.23 부터 버퍼가 없다

`go doc time.After` 는 Go 1.23 부터 멈추지 않은 타이머도 참조가 없으면 GC 가 회수하므로 `NewTimer` 를 따로 고를 이유가 없다고 적습니다. 채널 모양도 바뀌었습니다. 같은 코드를 `go.mod` 의 `go` 줄만 바꿔 돌렸습니다.

| `go.mod` 의 `go` 줄 | 출력 |
|---|---|
| `go 1.22` | `timer 채널 cap: 1` |
| `go 1.23` | `timer 채널 cap: 0` |
| `go 1.23`, `GODEBUG=asynctimerchan=1` | `timer 채널 cap: 1` |

`go doc time.NewTimer` 대로 1.23 부터 타이머 채널은 무버퍼이고, `Stop` 뒤에 낡은 시각을 받는 일이 없습니다. 이 동작은 툴체인 버전이 아니라 `go.mod` 의 `go` 줄을 따릅니다. 옛 모듈을 올릴 때 함께 확인할 지점입니다.

## 취소는 부모에서 자식으로만 내려간다

`root` 아래에 `A`·`B` 를 두고 `A` 아래에 `A1` 을 둔 뒤 `A` 만 취소했습니다.

```text
A 를 취소한 뒤
  root: 살아 있음
  A   : 취소됨(context canceled)
  A1  : 취소됨(context canceled)
  B   : 살아 있음
root 를 취소한 뒤
  B   : 취소됨(context canceled)
```

`ctx.Err()` 는 `context.Canceled` 나 `context.DeadlineExceeded` 둘뿐이라 왜 취소됐는지는 담지 못합니다. 그 빈자리를 원인(cause) API 가 채웁니다.

```text
Err  : context canceled
Cause: 요청 한도를 넘었다
Err  : context deadline exceeded
Cause: 외부 API 가 느리다
Is DeadlineExceeded: true
AfterFunc: 부모가 취소되어 정리를 돌린다
stop() 결과(이미 돌았으면 false): false
WithoutCancel 의 Err: <nil>
```

| API | 하는 일 | 도입 |
|---|---|---|
| `WithCancelCause`, `Cause` | 취소할 때 원인 에러를 남기고 꺼낸다 | 1.20 |
| `WithTimeoutCause`, `WithDeadlineCause` | 시간 초과의 원인을 정한다. `Err()` 는 그대로 `DeadlineExceeded` | 1.21 |
| `AfterFunc` | 취소되면 함수를 돌린다. 고루틴을 따로 세워 기다릴 필요가 없다 | 1.21 |
| `WithoutCancel` | 값은 물려받고 취소는 끊는다. 응답 뒤에도 돌아야 하는 기록에 쓴다 | 1.21 |

도입 버전은 `$(go env GOROOT)/api/go1.20.txt`·`go1.21.txt` 로 확인했습니다. 예약 실행기는 외부 명령을 `exec.CommandContext` 와 시간 제한으로 돌립니다. 끝난 뒤 `errors.Is(ctx.Err(), context.DeadlineExceeded)` 로 시간 초과와 일반 실패를 갈라 알림 문구를 바꿉니다.

`go doc context` 는 규칙도 적습니다. Context 를 구조체에 넣지 말고 첫 매개변수 `ctx` 로 넘깁니다. 값은 프로세스와 API 를 건너는 요청 범위 자료에만 씁니다. 키 타입이 겹치면 값이 덮입니다.

```text
문자열 키 id: 요청-9f3a
authKey: 사용자-42 traceKey: 요청-9f3a
```

두 패키지가 문자열 `"id"` 를 키로 쓰자 뒤의 값이 앞의 값을 가렸습니다. 패키지마다 내보내지 않는 키 타입(`type authKey struct{}`)을 두면 겹치지 않습니다. `go vet` 은 문자열 키를 잡지 않았습니다.

## 파이프라인의 단계마다 ctx.Done 도 함께 기다린다

생성 → 제곱 단계에서 보내기를 `select` 로 감싸 취소를 함께 기다립니다.

```go
func Square(ctx context.Context, in <-chan int) <-chan int {
	out := make(chan int)
	go func() {
		defer close(out)
		for v := range in {
			select {
			case out <- v * v:
			case <-ctx.Done():
				return
			}
		}
	}()
	return out
}
```

값 하나만 받고 떠난 뒤 고루틴 수를 셌습니다. `LeakySquare` 는 `select` 없이 `out <- v * v` 만 하는 판입니다.

```text
시작: 1
ctx 를 보는 파이프라인, 값 하나 받은 뒤: 3
cancel 뒤: 1
ctx 를 보지 않는 단계, cancel 뒤: 2
```

ctx 를 보지 않는 단계는 받는 쪽이 사라진 보내기에서 영원히 멈춰 한 개가 남았습니다. 같은 확인을 `runtime.NumGoroutine` 이 원래 수로 돌아오는지 기다리는 테스트로 고정했고, `go test -race` 로 통과했습니다.

## 한 단계 더 — 채널은 뮤텍스보다 비싸고, close 는 방송이다

같은 정수 하나를 넘기는 비용을 `b.Loop` 벤치마크로 쟀습니다. 5회 중앙값이고, 다른 작업과 함께 돈 상태라 비율로 읽습니다.

| 방식 | ns/op | allocs/op |
|---|---:|---:|
| 뮤텍스로 지킨 덧셈(경합 없음) | 2.025 | 0 |
| 버퍼 128칸 채널에 보내기 | 22.86 | 0 |
| 무버퍼 채널에 보내기 | 114.8 | 0 |
| 뮤텍스 덧셈, 고루틴 14개 경합 | 79.92 | 0 |

무버퍼 채널은 매번 상대 고루틴과 만나야 해서 경합 없는 뮤텍스보다 약 57배 느렸습니다. 숫자 하나를 지키는 데 채널을 쓸 이유는 없습니다. 채널의 값은 소유권을 넘기고 고루틴의 순서를 맞추는 데 있습니다.

보내기는 한 명을 깨우고 `close` 는 기다리는 모두를 깨웁니다. 다섯 고루틴을 한 채널에서 기다리게 한 실험입니다.

```text
값 하나 보낸 뒤 깨어난 수: 1
close 뒤 깨어난 수: 5
```

`closechan` 은 `recvq` 를 끝까지 비우며 모든 수신자를 깨웁니다. 메모리 모델도 "채널을 닫는 것은 닫혀서 제로 값을 받는 받기보다 먼저 일어난다"고 정합니다. `ctx.Done()` 이 닫히는 채널인 이유입니다. 취소는 기다리는 모든 고루틴에게 한 번에 알려야 합니다.

| 상황 | 고르는 것 |
|---|---|
| 일감·결과의 소유권을 넘긴다, 순서를 맞춘다 | 채널 |
| 여럿이 한 자료구조를 고친다, 숫자 하나를 센다 | 뮤텍스·atomic |
| 그만하라는 신호와 마감 시각을 호출 나무 전체에 퍼뜨린다 | context |

## 면접에서 이렇게 묻는다

**Q1. 무버퍼 채널과 버퍼 채널은 무엇이 다릅니까?**
무버퍼 채널은 보내는 쪽과 받는 쪽이 만나야 끝나서 동기화 지점이 되고, 런타임은 값을 상대 스택에 바로 복사합니다. 버퍼 채널은 빈 칸이 있으면 기다리지 않고, 가득 차면 `sendq` 에 줄을 섭니다.

**Q2. 닫힌 채널과 nil 채널에서 무슨 일이 생기고, 채널은 누가 닫습니까?**
닫힌 채널에 보내거나 다시 닫으면 패닉, 받으면 남은 값 뒤에 제로 값과 `false` 입니다. nil 채널은 보내기와 받기가 영원히 막히고 닫으면 패닉입니다. 막힌 송신자가 있는 채널을 닫아도 그 송신자가 패닉하므로 보내는 쪽이 닫습니다.

**Q3. select 의 case 가 여럿 준비되면 무엇이 실행됩니까?**
균등한 무작위입니다. 10만 번 돌려 약 5:5 로 나뉘는 것을 확인했습니다. 우선순위가 필요하면 급한 채널만 `default` 와 함께 먼저 확인한 뒤 둘을 기다립니다.

**Q4. context 취소는 어떻게 퍼지고, 원인은 어떻게 남깁니까?**
부모가 취소되면 자식이 모두 취소되고 반대 방향은 없습니다. `Err()` 는 두 값뿐이라 원인은 `WithCancelCause`(1.20)나 `WithTimeoutCause`(1.21)로 남기고 `context.Cause` 로 꺼냅니다. 구조체에 넣지 않고 첫 매개변수로 넘기는 것이 문서의 규칙입니다.

**Q5. 파이프라인이 고루틴을 새지 않는다는 것을 어떻게 보장합니까?**
단계마다 보내기를 `select` 로 감싸 `ctx.Done()` 을 함께 기다리고, 끝나면 출력 채널을 닫습니다. 확인은 `runtime.NumGoroutine` 이 취소 뒤 원래 수로 돌아오는지 기다리는 테스트로 합니다.

## 연습

### 연습 1 — 가장 빠른 복제본의 답만 쓰기

같은 요청을 복제본 셋에 동시에 보내고 처음 온 답을 돌려주는 `First(ctx, delays)` 를 만듭니다. 늦은 복제본은 멈춰야 하고, 반환 뒤 고루틴이 남지 않아야 합니다.

<details>
<summary>풀이</summary>

```go
func First(ctx context.Context, delays map[string]time.Duration) (string, error) {
	ctx, cancel := context.WithCancel(ctx)
	defer cancel() // 반환하면 늦은 복제본을 멈춘다

	type result struct {
		name string
		err  error
	}
	results := make(chan result, len(delays)) // 늦은 쪽도 막히지 않고 보내고 끝난다
	for name, d := range delays {
		go func() {
			n, err := Query(ctx, name, d)
			results <- result{n, err}
		}()
	}
	r := <-results
	return r.name, r.err
}
```

`Query` 는 `delay` 와 `ctx.Done()` 을 `select` 로 함께 기다립니다. 결과 채널의 버퍼를 복제본 수만큼 잡아 두면, 아무도 받지 않는 늦은 결과도 보내기에서 막히지 않습니다. 10ms·1초·1초짜리 복제본으로 돌린 테스트는 `서울` 을 받았고, 고루틴 수가 원래대로 돌아와 `-race` 와 함께 통과했습니다.

</details>

### 연습 2 — 급한 채널을 먼저 비우기

`high` 와 `low` 에 값이 셋씩 들어 있고 둘 다 닫혀 있습니다. 받는 순서가 언제나 `H1 H2 H3 L1 L2 L3` 이 되게 `Drain` 을 씁니다.

<details>
<summary>풀이</summary>

`high` 만 막히지 않게 먼저 보고, 비어 있을 때만 둘을 함께 기다립니다.

```go
select { // 1단계: high 만 본다
case v, ok := <-high:
	if !ok {
		high = nil
		continue
	}
	out = append(out, v)
	continue
default:
}
select { // 2단계: high 가 비었을 때만 둘을 함께 기다린다
case v, ok := <-high:
	// 1단계와 같다
case v, ok := <-low:
	// low 를 받는다
}
```

`select` 하나로 두 채널을 함께 받는 판은 1,000번 가운데 854번 순서가 섞였습니다.

```text
priority_test.go:36: select 하나: 1000번 가운데 854번 low 가 끼어들었다
```

두 단계 판은 매번 `H1 H2 H3 L1 L2 L3` 이었습니다.

</details>

## 정리

채널은 값을 놓을 자리가 없으면 막히고, 닫힌 채널에 보내거나 닫힌·nil 채널을 닫을 때만 패닉을 냅니다. `select` 는 준비된 case 를 무작위로 고르고, nil 채널 case 는 고르지 않습니다. 취소는 `Done` 채널을 닫는 방송이라 부모에서 모든 자식으로 한 번에 내려갑니다.

다음은 [로드맵의 19편](https://blog.doksam.com/blog/go-course-00-roadmap/) sync·atomic 과 메모리 모델입니다. 채널을 쓰지 않을 때 무엇이 먼저 일어났다고 말할 수 있는지를 happens-before 로 다룹니다.
