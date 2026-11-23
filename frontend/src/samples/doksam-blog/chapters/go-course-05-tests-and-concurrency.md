# Go 강의 5편 — 테스트는 라우터에 대고 돌리고, 고루틴은 멈출 방법을 정하고 띄운다

> 서버를 고칠 때마다 무엇으로 안 깨졌다고 확신하고, 고루틴은 언제 띄워도 됩니까?

경우마다 이름을 붙인 표 주도 테스트를 실제 라우터에 대고 돌립니다. 핸들러만 부르면 미들웨어가 빠져 깨진 것을 놓칩니다. 고루틴은 "누가 멈추는가, 결과는 누가 받는가"에 답할 때만 띄우고, 함께 쓰는 값은 뮤텍스로 지킨 뒤 `go test -race` 로 확인합니다. 예제는 4편의 inbox 서버를 테스트하기 좋게 줄인 것입니다.

> [그림] 배달 로봇 세 대가 세 줄의 길에서 계산대 하나로 달려온다. 계산대 위에 Mutex 자물쇠가 있고, 옆의 context 신호등은 빨간불이다. 오른쪽 검수판에는 경우별 기대 상태 코드 표와 PASS 도장이 있다

세 로봇이 고루틴, 계산대의 숫자가 함께 쓰는 값입니다. 자물쇠는 한 번에 한 로봇만 숫자를 고치게 하고, 신호등(context)이 빨간불이면 모두 멈춥니다.

출력은 2026-11-18 에 맥(Apple M4 Pro)과 Go 1.26.2 로 돌린 결과입니다.

## 경우마다 이름을 붙이면 실패한 줄이 이유를 말한다

검증 함수 `Validate` 는 앞뒤 공백을 지우고, 연락처나 내용이 없으면 `ErrMissing`, 너무 길면 `ErrTooLong` 을 돌려줍니다. 경우를 표로 적고 `t.Run` 으로 하나씩 돌립니다. 제 예약 실행기의 날짜 계산 테스트와 이 사이트 서버의 라우트 테스트가 이 모양입니다.

```go
func TestValidate(t *testing.T) {
	cases := []struct {
		name    string
		in      Message
		wantErr error
	}{
		{"정상", Message{Name: " 홍길동 ", ReplyTo: "a@example.com", Body: "안녕하세요"}, nil},
		{"연락처 없음", Message{Body: "x"}, ErrMissing},
		{"공백뿐인 내용", Message{ReplyTo: "x", Body: "   "}, ErrMissing},
		{"긴 이름", Message{ReplyTo: "x", Body: "x", Name: strings.Repeat("가", 101)}, ErrTooLong},
		{"이름 100자는 통과", Message{ReplyTo: "x", Body: "x", Name: strings.Repeat("가", 100)}, nil},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			_, err := Validate(tc.in)
			if !errors.Is(err, tc.wantErr) {
				t.Errorf("err = %v, 원하는 값 %v", err, tc.wantErr)
			}
		})
	}
}
```

경계값(100자 통과, 101자 실패)을 나란히 두면 `>` 와 `>=` 를 헷갈린 실수가 바로 드러납니다. `go test -v` 는 하위 테스트(subtest)를 `TestValidate/연락처_없음` 처럼 공백을 `_` 로 바꾼 이름으로 찍고, `-run 'TestValidate$/연락처'` 로 하나만 고릅니다. `-run` 은 `/` 로 나눈 단계마다 정규식이라, `$` 를 빼면 `TestValidateTrims` 도 함께 돕니다.

## 테스트는 핸들러가 아니라 라우터에 대고 돌린다

서버의 `Routes()` 는 세 경로를 등록한 라우터에 `X-Content-Type-Options: nosniff` 헤더를 붙이는 미들웨어를 씌워 돌려줍니다. 테스트는 여기에 `httptest.NewRecorder`(응답을 메모리에 받는 가짜 `ResponseWriter`)로 요청을 보내고, 정적 파일은 `testing/fstest.MapFS` 로 넣습니다.

```go
h := newServer().Routes()
cases := []struct {
	name, method, target, body string
	code                       int
	contains                   string
}{
	{"헬스", "GET", "/healthz", "", 200, "ok"},
	{"정적 첫 화면", "GET", "/", "", 200, "<h1>inbox</h1>"},
	{"메시지 저장", "POST", "/api/messages", `{"replyTo":"a@example.com","body":"안녕"}`, 201, `"id":1`},
	{"헬스에 POST", "POST", "/healthz", "", 405, ""},
	// … 없는 파일, 깨진 JSON, 내용 없음
}
```

경우마다 상태 코드, 본문, 미들웨어 헤더를 봅니다. 핸들러를 직접 부른 결과와 견줘 봤습니다.

```text
direct_test.go:20: 직접: 201 "", 라우터: 201 "nosniff"
```

직접 부른 쪽은 헤더가 비었습니다. 인가를 미들웨어로 거는 서버라면 "권한 없이도 된다"는 회귀를 못 잡습니다. 405 같은 라우터의 동작도 라우터를 거쳐야 시험됩니다.

반복되는 준비는 도우미 함수로 빼고 첫 줄에 `t.Helper()` 를 부릅니다. 같은 실패를 두 도우미로 내 보니, 없는 쪽은 도우미 안의 줄(`helper_test.go:10`)을, 있는 쪽은 부른 줄(`helper_test.go:27`)을 찍었습니다. 어느 경우가 틀렸는지는 뒤쪽이 알려 줍니다.

## 시계와 파일은 밖에서 넣는다

요청 제한(rate limit)은 이 사이트 서버의 것을 줄였습니다. IP 마다 시각 목록을 맵에 두고 뮤텍스로 지키며, 현재 시각은 `Allow(key string, now time.Time) bool` 처럼 인자로 받습니다.

```go
func TestLimiterWindow(t *testing.T) {
	l := NewLimiter(1, time.Hour)
	t0 := time.Date(2026, 11, 18, 9, 0, 0, 0, time.UTC)
	if !l.Allow("a", t0) {
		t.Fatal("첫 요청은 허용해야 한다")
	}
	// … 1분 뒤는 막고, 다른 키는 따로 센다
	if !l.Allow("a", t0.Add(61*time.Minute)) {
		t.Fatal("한 시간이 지나면 다시 허용해야 한다")
	}
}
```

`Allow` 가 안에서 `time.Now()` 를 불렀다면 한 시간을 기다려야 합니다. 서버도 `Now func() time.Time` 필드로 시계를 받습니다.

실제 TCP 연결까지 보려면 `httptest.NewServer(s.Routes())` 를 띄우고 `t.Cleanup(srv.Close)` 로 닫기를 걸어 둡니다. 파일 쓰기 시험은 `t.TempDir()` 폴더에서 합니다. 테스트가 실패해도 서버는 닫히고 폴더는 지워집니다.

## go test 의 옵션 다섯 개면 충분하다

| 옵션 | 하는 일 | 이 예제에서 본 것 |
|---|---|---|
| `-run 정규식` | 고른 테스트만 | 위의 `TestValidate$/연락처` |
| `-v` | 테스트마다 결과와 `t.Log` 출력 | 하위 테스트 이름 |
| `-count=1` | 결과 캐시를 쓰지 않음 | 두 번째 `go test .` 는 `(cached)` 였다 |
| `-cover` | 실행된 문장 비율 | `coverage: 94.9% of statements` |
| `-race` | 데이터 경쟁 탐지 | 아래 절 |

커버리지 94.9% 는 문장이 한 번은 실행됐다는 뜻이지 경계값을 확인했다는 뜻이 아닙니다.

## 고루틴은 멈출 사람과 받을 사람을 정하고 띄운다

고루틴(goroutine)은 `go` 키워드 하나로 띄우는 가벼운 실행 흐름입니다. 띄우기 쉬운 만큼 누가 멈추는가, 결과는 누가 받는가에 먼저 답합니다. 주소 여러 개를 동시에 받는 함수입니다.

```go
// FetchAll 은 urls 를 동시에 받아 같은 순서로 결과를 돌려준다.
// 멈추는 쪽은 ctx 이고, 결과는 FetchAll 이 채널에서 모두 받는다.
func FetchAll(ctx context.Context, client *http.Client, urls []string) []Result {
	type item struct {
		i int
		r Result
	}
	ch := make(chan item)
	var wg sync.WaitGroup
	for i, u := range urls {
		wg.Go(func() { ch <- item{i, fetch(ctx, client, u)} })
	}
	go func() { wg.Wait(); close(ch) }()
	out := make([]Result, len(urls))
	for it := range ch {
		out[it.i] = it.r
	}
	return out
}
```

`wg.Go` 는 Go 1.25 에 들어온 메서드로 `Add(1)` 과 `Done()` 을 대신합니다. 고루틴이 모두 끝나면 채널을 닫고, 그러면 `range` 가 끝납니다. 요청은 `http.NewRequestWithContext(ctx, …)` 로 만들어 `ctx` 가 끝나면 함께 끊깁니다.

테스트는 바로 답하는 서버와 끝까지 답하지 않는 서버를 `httptest` 로 띄우고, 100ms 시한을 겁니다. `t.Context()` 는 Go 1.24 에 들어온 메서드로, `t.Cleanup` 에 건 함수가 돌기 직전에 취소되는 context 를 줍니다.

```go
ctx, cancel := context.WithTimeout(t.Context(), 100*time.Millisecond)
defer cancel()
got := FetchAll(ctx, http.DefaultClient, []string{fast.URL, slow.URL, fast.URL})
```

빠른 두 칸은 200 이고, 느린 칸의 에러는 `errors.Is(err, context.DeadlineExceeded)` 로 확인합니다. 테스트는 시한인 100ms 언저리에서 끝납니다.

50ms 걸리는 주소 10개를 순서대로 받으면 513ms, `FetchAll` 로는 51ms 였습니다. 다섯 번 잰 가운데값이고, 세 번 반복해도 512~513ms 와 51~52ms 였습니다.

## 공유하는 값은 뮤텍스로 지키고 -race 로 확인한다

고루틴 둘이 같은 변수를 한 번씩 더하는 테스트입니다.

```go
n := 0
var wg sync.WaitGroup
wg.Go(func() { n++ })
wg.Go(func() { n++ })
wg.Wait()
if n != 2 {
	t.Errorf("n = %d, 원하는 값 2", n)
}
```

`-race` 없이 1,000번씩 세 번 돌려도 값은 늘 2 였고 실패는 0번이었습니다. `-race` 를 붙이면 세 번 모두 잡혔습니다.

```text
$ RACE_DEMO=1 go test -race -run TestRacyTwo .
==================
WARNING: DATA RACE
Read at 0x00c000114208 by goroutine 9:
  example.com/race.TestRacyTwo.func2()
      …/two_test.go:17 +0x2c

Previous write at 0x00c000114208 by goroutine 8:
  example.com/race.TestRacyTwo.func1()
      …/two_test.go:16 +0x3c
…
--- FAIL: TestRacyTwo (0.00s)
    testing.go:1712: race detected during execution of test
```

데이터 경쟁(data race)은 결과가 우연히 맞아도 버그입니다. 고루틴을 100개로 늘리면 `-race` 없이도 100번 중 99~100번 실패했고, `n` 은 67~99 사이였습니다. 고치는 방법은 둘입니다. 고치는 동안 뮤텍스(`mu.Lock()`·`mu.Unlock()`)로 잠그거나, 더하기 하나가 쪼개지지 않는 원자적(atomic) 정수를 씁니다.

```go
var n atomic.Int64
var wg sync.WaitGroup
for range 100 {
	wg.Go(func() { n.Add(1) })
}
```

`atomic.Int64` 타입은 Go 1.19 에 들어왔습니다. 두 방식 모두 고루틴 100개로 `-race` 를 통과했습니다. 무엇을 쓸지는 이렇게 고릅니다.

| 상황 | 고르는 것 | 이 편의 예 |
|---|---|---|
| 결과나 일감을 다른 고루틴에 넘긴다 | 채널 | `FetchAll` 의 `ch` |
| 여럿이 한 자료구조를 읽고 고친다 | 뮤텍스 | `Limiter` 의 맵, `Store` 의 슬라이스 |
| 숫자 하나를 세거나 올린다 | `sync/atomic` | 카운터 |

한 값을 채널로 넘기면서 뮤텍스로도 지키면 그 값의 주인이 흐려지니 섞지 않습니다.

`-race` 는 실행된 코드의 경쟁만 잡습니다. 위 경쟁 테스트는 `RACE_DEMO` 환경변수가 없으면 건너뛰는데, 그대로 `go test -race` 를 돌리면 `ok` 가 나왔습니다.

## 1.22 부터 루프 변수는 반복마다 새로 생긴다

`FetchAll` 의 고루틴은 루프 변수 `i`, `u` 를 그대로 씁니다. Go 1.22 부터 안전한 코드입니다. 루프에서 `func() { fmt.Print(i, " ") }` 세 개를 모아 두었다가 부르는 프로그램을 `go.mod` 의 `go` 줄만 바꿔 돌렸습니다.

| `go.mod` 의 `go` 줄 | 출력 |
|---|---|
| `go 1.21` | `3 3 3` |
| `go 1.22` | `0 1 2` |

1.21 까지는 루프 전체가 변수 하나를 같이 써서 함수들이 마지막 값을 봤습니다. 기준은 툴체인이 아니라 `go` 줄이라서 옛 모듈을 손볼 때는 이 줄부터 봅니다. 테스트에 쓴 `for i := range 5` 도 1.22 부터입니다.

## 한 단계 더 — 새는 고루틴은 숫자로 잡는다

결과를 기다리다 시한이 지나면 포기하는 함수입니다.

```go
func firstResult(ctx context.Context, buffered bool) (int, error) {
	ch := make(chan int)
	if buffered {
		ch = make(chan int, 1) // 받는 쪽이 떠나도 보내는 쪽이 한 번은 넣고 끝난다
	}
	go func() { ch <- slowWork() }() // 20ms 걸린다
	select {
	case v := <-ch:
		return v, nil
	case <-ctx.Done():
		return 0, ctx.Err()
	}
}
```

1ms 시한으로 100번 부르고 100ms 기다린 뒤 `runtime.NumGoroutine()` 을 셌습니다.

```text
buffered=false: 고루틴 1개 → 101개
buffered=true: 고루틴 1개 → 1개
```

버퍼 없는 채널에서는 받는 쪽이 떠난 뒤 `ch <-` 가 영원히 막힙니다. 호출마다 고루틴이 하나씩 남았고, 세 번 돌려 매번 101개였습니다. 버퍼를 하나 두면 보내기가 막히지 않아 모두 끝납니다. 서버라면 요청마다 메모리가 새는 셈입니다.

시간이 걸리는 테스트는 Go 1.25 에 들어온 `testing/synctest` 로 바로 끝냅니다. `synctest.Test` 가 만든 거품(bubble) 안은 가짜 시계로 돌고, 안의 고루틴이 모두 막혀야 시간이 흐릅니다(`go doc testing/synctest`).

```go
func TestLimiterWithFakeClock(t *testing.T) {
	synctest.Test(t, func(t *testing.T) {
		l := NewLimiter(1, time.Hour)
		if !l.Allow("a", time.Now()) {
			t.Fatal("첫 요청은 허용해야 한다")
		}
		time.Sleep(61 * time.Minute)
		if !l.Allow("a", time.Now()) {
			t.Fatal("61분 뒤에는 허용해야 한다")
		}
	})
}
```

```text
--- PASS: TestLimiterWithFakeClock (0.00s)
```

61분을 잤는데 0.00초에 끝났습니다. 시계를 인자로 넣기 어려운 코드에 씁니다.

## 면접에서 이렇게 묻는다

**Q1. 표 주도 테스트의 장점은 무엇입니까?**
경계값을 나란히 놓고 새 경우를 한 줄로 더합니다. `t.Run` 이름 덕에 실패한 경우와 받은 값·원하는 값이 한 줄에 나옵니다.

**Q2. 핸들러 함수를 직접 부르는 테스트의 문제는 무엇입니까?**
라우터와 미들웨어를 건너뜁니다. 인가처럼 미들웨어로 거는 규칙은 라우터를 거친 `httptest` 테스트로 고정합니다.

**Q3. 고루틴 누수는 어떻게 생기고 어떻게 찾습니까?**
받을 쪽이 떠난 채널 보내기처럼 끝나지 않는 대기에서 생깁니다. `runtime.NumGoroutine` 을 전후로 세서 찾고, 버퍼나 `ctx.Done()` 으로 빠져나갈 길을 만듭니다.

**Q4. 채널과 뮤텍스는 어떻게 고릅니까?**
값이나 일감의 소유권을 넘기면 채널, 여럿이 한 자료구조를 고치면 뮤텍스, 숫자 하나면 atomic 입니다. 둘을 섞으면 값의 주인이 흐려진다고 덧붙입니다.

**Q5. `-race` 가 통과하면 경쟁이 없다고 봐도 됩니까?**
아닙니다. 실행된 경로의 경쟁만 잡기 때문입니다. 대신 값이 맞는 경쟁도 잡아서, 1,000번 모두 값이 맞은 고루틴 둘의 덧셈을 `-race` 는 매번 찾아냈습니다.

## 연습

### 연습 1 — 동시에 최대 n 개만 요청하기

`FetchAll` 은 주소 수만큼 동시에 요청합니다. 상대 서버를 배려해 동시에 최대 `n` 개만 돌게 고치고, 그것을 확인하는 테스트를 씁니다.

<details>
<summary>풀이</summary>

버퍼가 `n` 인 채널을 빈 자리 표로 씁니다. 자리를 얻어야 요청하고, 끝나면 돌려줍니다. 자리를 기다리는 동안에도 `ctx` 가 끝나면 빠져나옵니다.

```go
sem := make(chan struct{}, n) // 빈 자리 n 개짜리 표
var wg sync.WaitGroup
for i, u := range urls {
	wg.Go(func() {
		select {
		case sem <- struct{}{}: // 자리를 얻는다
		case <-ctx.Done():
			out[i] = Result{URL: u, Err: ctx.Err()}
			return
		}
		defer func() { <-sem }() // 자리를 돌려준다
		out[i] = fetch(ctx, client, u)
	})
}
wg.Wait()
```

고루틴마다 `out` 의 다른 칸에 쓰므로 경쟁이 아닙니다. 테스트 서버는 처리 중인 요청 수를 `atomic.Int64` 로 세어 최댓값을 남깁니다. 12개를 `n=3` 으로 보내면 세 번 모두 최대 3 이었고 `-race` 도 통과했습니다. `n=12` 로 바꾸면 `동시 요청 최대 = 12, 원하는 값 3 이하` 로 실패해 테스트가 제대로 잰다는 것도 확인했습니다.

</details>

### 연습 2 — 오래된 키를 치우는지 시험하기

`Limiter` 는 1,000번째 호출마다 창 밖의 키를 지웁니다. 이 청소가 실제로 도는지 시각을 주입해 시험합니다.

<details>
<summary>풀이</summary>

키 999개를 한 시각에 넣고 1,000번째 호출을 두 시간 뒤에 합니다. 앞의 키는 모두 창 밖이라 지워져야 합니다. 키 수는 `Keys()` 메서드로 셉니다.

```go
for i := range sweepEvery - 1 {
	l.Allow(fmt.Sprintf("ip-%d", i), t0)
}
// …
l.Allow("late", t0.Add(2*time.Hour))
if got := l.Keys(); got != 1 {
	t.Errorf("청소 뒤 키 수 = %d, 원하는 값 1", got)
}
```

시계를 인자로 받으니 두 시간을 기다리지 않습니다.

</details>

## 정리

서버가 안 깨졌다는 확신은 이름 붙은 경우를 실제 라우터에 대고 돌리는 테스트에서 나옵니다. 고루틴은 멈추는 쪽과 받는 쪽을 정한 뒤 띄우고, 공유 값은 뮤텍스나 atomic 으로 지킨 다음 `-race` 로 확인합니다.

6편에서는 이 서버를 셸 없는 컨테이너에 담고 `/healthz` 와 종료 신호 처리를 붙여 배포합니다.
