# Go 강의 10편 — 운영 서버는 기본값 "무한정 기다림"을 걷어 내야 버틴다

> 로컬에서 잘 돌던 Go 서버가 운영에서 멈추거나 새는 이유는 무엇이고, 무엇을 설정해야 합니까?

`net/http` 의 기본값이 "무한정 기다림"이기 때문입니다. `http.Server` 의 타임아웃은 0 이면 없고, `http.DefaultClient` 에도 타임아웃이 없습니다. 헤더를 천천히 보내는 클라이언트, 중간에 끊은 클라이언트, 끝나지 않는 종료가 모두 이 기본값에서 생깁니다. 이 편은 그 기본값을 걷어 내는 설정을 하나씩 실험으로 확인합니다.

> [그림] 계산대 뒤의 배달 로봇. 계산대에 헤더·읽기·쓰기·대기 타이머가 있고 헤더 타이머가 울린다. 앞에는 ID·로그·복구 회전문이 있고, GET / HT 까지만 말한 손님이 나가는 문으로 간다

계산대의 타이머 네 개가 서버 타임아웃이고, 회전문 세 개가 미들웨어입니다. 헤더를 끝내지 못한 손님은 타이머가 울리면 나갑니다.

라우팅과 JSON 은 [4편](https://blog.doksam.com/blog/go-course-04-http-server/), 헬스 체크와 종료 기초는 [6편](https://blog.doksam.com/blog/go-course-06-deploy-and-operate/)에 있습니다. 실험은 127.0.0.1 에 서버와 클라이언트를 한 프로세스로 띄워 돌렸습니다. 환경은 2026-11-18, Apple M4 Pro, Go 1.26.2 입니다. 다른 작업과 함께 돌린 값이라 시간은 0.1초 단위로만 읽고, 출력의 포트 번호는 `PORT` 로 바꿨습니다.

## 서버 타임아웃 네 개는 기본값이 모두 "없음"이다

`net/http` 소스의 필드 주석을 표로 옮겼습니다.

| 필드 | 재는 구간 | 0 이면 |
|---|---|---|
| `ReadHeaderTimeout` | 요청 헤더를 다 읽을 때까지 | `ReadTimeout` 값. 그것도 0 이면 없음 |
| `ReadTimeout` | 헤더와 본문 전체 | 없음 |
| `WriteTimeout` | 헤더를 읽은 뒤부터 응답을 다 쓸 때까지 | 없음 |
| `IdleTimeout` | keep-alive 연결이 다음 요청을 기다리는 시간 | `ReadTimeout` 값. 그것도 0 이면 없음 |

이 사이트의 Go 서버는 차례로 10·30·60·120초를 줍니다.

**`ReadHeaderTimeout` 은 느린 헤더 공격(slowloris)을 막습니다.** 날 TCP 연결(`net.Dial`)로 헤더를 200ms 에 한 바이트씩 보냈습니다. 서버가 연결을 끊을 때까지 걸린 시간입니다.

```text
ReadHeaderTimeout=1s: 서버가 1.0초 만에 연결을 끊었다
ReadHeaderTimeout=0s: 5.0초가 지나도 연결이 열려 있다
```

타임아웃이 있으면 서버는 `HTTP/1.1 400 Bad Request` 를 보내고 연결을 닫았습니다. 없으면 클라이언트가 포기할 때까지 연결과 고루틴 하나가 묶입니다.

**`WriteTimeout` 을 넘긴 핸들러는 실패를 모릅니다.** `WriteTimeout` 1초인 서버에서 1.5초 걸리는 핸들러를 불렀습니다.

```text
  핸들러: 쓰기 err=<nil>
/slow: 1.5초 뒤 클라이언트 에러: Get "http://127.0.0.1:PORT/slow": EOF
/export: 1.5초 뒤 200 "내보내기 완료\n"
```

핸들러의 `Fprintln` 은 에러 없이 끝났지만 클라이언트는 상태 코드 없이 `EOF` 를 받았습니다. 쓰기는 버퍼에만 들어가 에러가 없었고, 연결은 마감이 지나 닫혔습니다. 그래서 `WriteTimeout` 은 핸들러가 지킬 시간 제한이 아니라 연결을 정리하는 안전망입니다. 내보내기처럼 오래 걸리는 경로만 늘리려면 Go 1.20 의 `http.ResponseController` 를 씁니다. `/export` 는 이렇게 마감을 늘려 200 을 받았습니다.

```go
rc := http.NewResponseController(w)
if err := rc.SetWriteDeadline(time.Now().Add(5 * time.Second)); err != nil {
	http.Error(w, "deadline", http.StatusInternalServerError)
	return
}
```

## 요청 context 를 끝까지 넘기면 끊긴 요청의 일도 멈춘다

클라이언트가 연결을 끊으면 `r.Context()` 가 취소됩니다. 이 context 를 DB·외부 API 호출까지 넘겨야 그 일도 멈춥니다. 이 사이트의 Go 서버는 문의를 저장할 때 `context.WithTimeout(r.Context(), 10*time.Second)` 를 씁니다. 같은 모양으로 `ctx.Done()` 을 기다리는 3초짜리 작업을 부르고, 클라이언트는 0.3초 만에 포기하게 했습니다.

```text
클라이언트: Get "http://127.0.0.1:PORT/report": context deadline exceeded
서버 핸들러: 작업 중단: context canceled (0.3초)
TimeoutHandler: 503 "처리 시간이 길어 중단했습니다\n"
TimeoutHandler 안: 작업 중단: context deadline exceeded
```

서버 작업은 0.3초에 멈췄습니다. 아래 두 줄은 `http.TimeoutHandler(slow, 500*time.Millisecond, "…")` 의 결과입니다. 500ms 뒤 503 과 정한 문구를 보내고 안쪽 핸들러의 context 를 끝냅니다. `WriteTimeout` 과 달리 클라이언트가 이유를 받습니다.

## 미들웨어는 한 번 감싸고, 인가는 등록하는 줄에서 건다

미들웨어는 `func(http.Handler) http.Handler` 입니다. 요청 ID, 접근 로그, 패닉 복구를 순서대로 겹쳐 씁니다.

```go
type Middleware func(http.Handler) http.Handler

// Chain 은 Chain(h, a, b) 를 a(b(h)) 로 만든다. 요청은 a 를 먼저 지난다.
func Chain(h http.Handler, mws ...Middleware) http.Handler {
	for i := len(mws) - 1; i >= 0; i-- {
		h = mws[i](h)
	}
	return h
}
```

**순서가 결과를 바꿉니다.** 패닉을 내는 `/boom` 을 두 순서로 불렀습니다.

```text
== RequestID → AccessLog → Recover
level=ERROR msg=panic id=req-1 value="nil map 에 썼다"
level=INFO msg=access id=req-1 method=GET path=/boom status=500
응답: 500
== RequestID → Recover → AccessLog
level=ERROR msg=panic id=req-1 value="nil map 에 썼다"
응답: 500
```

두 번째 순서에서는 접근 로그가 사라졌습니다. 패닉이 `AccessLog` 를 지나 위로 풀리면서 로그 줄을 건너뛰었기 때문입니다. 복구는 로그보다 안쪽에 둡니다. 상태 코드를 기억하는 `ResponseWriter` 래퍼에는 `Unwrap()` 을 붙입니다. `NewResponseController` 가 그것으로 원래 `ResponseWriter` 를 찾습니다.

인가는 핸들러 안에서 검사하지 않고 라우트를 등록하는 줄에서 감쌉니다. 핸들러마다 검사하면 언젠가 하나가 빠집니다.

```go
mux.Handle("GET /admin/stats", RequireToken(adminToken, http.HandlerFunc(stats)))
```

`RequireToken` 은 `crypto/subtle.ConstantTimeCompare` 로 비교하고, 토큰이 비어 있으면 열지 않고 닫습니다. 시험은 핸들러를 직접 부르지 않고 `httptest.NewServer` 에 실제 mux 와 미들웨어를 올려 돌립니다. 토큰 없음·틀린 토큰은 401, 맞는 토큰은 200, `/boom` 은 500 이 나왔습니다.

## 종료는 준비 상태를 먼저 내리고 Shutdown 을 부른다

`Shutdown` 의 문서 주석은 순서를 이렇게 적습니다. 리스너를 닫고, 쉬는 연결을 닫고, 연결이 쉬는 상태가 될 때까지 기다립니다. 진행 중인 요청의 context 는 취소하지 않습니다. 1초 걸리는 요청이 도는 중에 세 가지로 종료했습니다.

```text
== 기다릴 시간이 충분하다(5초)
/readyz: 503
Shutdown: err=<nil> (1.1초)
새 연결: 거절됨
진행 중이던 요청: status=200 err=<nil>
핸들러: 끝까지 처리
== 기다릴 시간이 모자라다(300ms)
/readyz: 503
Shutdown: err=context deadline exceeded (0.3초)
새 연결: 거절됨
진행 중이던 요청: status=0 err=Get "http://127.0.0.1:PORT/job": EOF
핸들러: 중단: context canceled
== BaseContext 를 함께 취소한다
/readyz: 503
Shutdown: err=<nil> (0.0초)
새 연결: 거절됨
진행 중이던 요청: status=503 err=<nil>
핸들러: 중단: 서버 종료
```

- 시간이 충분하면 요청을 끝까지 처리합니다.
- 시간이 모자라면 `context deadline exceeded` 가 돌아오고 요청은 계속 돕니다. `srv.Close()` 로 강제로 닫으면 클라이언트는 `EOF` 를 받습니다.
- `BaseContext` 로 요청 context 의 뿌리를 만들고 `RegisterOnShutdown` 에서 취소하면 핸들러가 바로 503 을 씁니다.

```go
base, cancel := context.WithCancelCause(context.Background())
srv := &http.Server{BaseContext: func(net.Listener) context.Context { return base } /* … */}
srv.RegisterOnShutdown(func() { cancel(errors.New("서버 종료")) })
```

준비 상태(`/readyz`)는 `Shutdown` 전에 내립니다. `Shutdown` 은 리스너부터 닫아서 그 뒤에는 `/readyz` 요청도 받지 못합니다. 운영 순서는 신호 수신, 준비 상태 503, 로드밸런서가 알아챌 만큼 대기, `Shutdown(10초)` 입니다. 실패하면 `Close` 하고, 마지막에 DB 같은 자원을 닫습니다.

## 클라이언트는 타임아웃을 주고, 하나를 재사용하고, Body 를 끝까지 읽는다

`net/http` 의 `Client.Timeout` 주석은 "A Timeout of zero means no timeout." 입니다. `http.DefaultClient` 와 `http.Get` 이 그 상태입니다. 제 데이터 수집 도구는 `&http.Client{Timeout: 15 * time.Second}` 를 하나 만들어 모든 요청에 씁니다.

연결 재사용은 `net/http/httptrace` 의 `GotConn` 으로 확인합니다. 64KiB 응답을 세 번씩 받았습니다.

```text
Body 를 끝까지 읽음=true
  reused=false
  reused=true
  reused=true
Body 를 끝까지 읽음=false
  reused=false
  reused=false
  reused=false
```

`Close` 만 하고 끝까지 읽지 않으면 매번 새 연결을 맺었습니다. `Client.Do` 문서도 Body 를 EOF 까지 읽고 닫아야 재사용할 수 있다고 적습니다. 연결 풀은 `Transport` 에 있어서, 요청마다 `&http.Transport{}` 를 새로 만들면 다 읽어도 세 번 모두 `reused=false` 입니다.

재시도는 결과가 같은(멱등) 요청에만 합니다. GET 은 되고, 문의 저장 같은 POST 는 두 번 저장될 수 있어 안 됩니다. 아래는 `retry.go` 의 반복문을 줄인 것입니다.

```go
for i := range attempts {
	if i > 0 {
		wait := rand.N(base << (i - 1)) // 0 ~ base·2^(i-1) 사이 무작위(full jitter)
		select {
		case <-time.After(wait):
		case <-ctx.Done():
			return nil, i, fmt.Errorf("재시도 대기 중 중단: %w", ctx.Err())
		}
	}
	// 네트워크 오류와 429·5xx 만 다시 한다. 404 같은 4xx 는 바로 돌려준다.
}
```

`rand.N` 은 Go 1.22 의 `math/rand/v2` 에 있습니다. 시험에서 503 두 번 뒤 성공은 3회, 404 는 1회였습니다. context 가 50ms 에 끝나면 대기를 멈추고 `context.DeadlineExceeded` 를 돌려줍니다. 대기를 무작위로 흩으면 여러 클라이언트가 한순간에 다시 몰리지 않습니다.

## 레이트 리미터는 기억할 양과 버스트로 고른다

이 사이트의 Go 서버는 IP 마다 최근 요청 시각을 기억하고 1시간에 5건까지 받습니다. 토큰 버킷은 토큰 수와 마지막 시각만 기억합니다.

| 기준 | 시각 목록(sliding log) | 토큰 버킷 |
|---|---|---|
| 키마다 기억하는 것 | 창 안의 시각 최대 `limit` 개 | 숫자 두 개 |
| 판정 | 창 안의 개수 < `limit` | 쌓인 토큰 ≥ 1 |
| 버스트 | 창 안에서 `limit` 개를 한꺼번에 허용 | `burst` 개까지 허용, 뒤로는 초당 `rate` |

두 방식 모두 `sync.Mutex` 로 맵을 지키고, 키가 끝없이 늘지 않게 오래된 키를 치워야 합니다. 사이트 서버는 1,000번 호출마다 한 번 치웁니다.

키가 되는 IP 는 앞단 프록시가 넣은 값만 믿습니다. 클라이언트가 `X-Forwarded-For: 203.0.113.9` 를 적어 보내면 프록시가 실제 주소를 오른쪽에 붙여 `203.0.113.9, 198.51.100.7` 이 됩니다. 시험은 맨 오른쪽 값을 쓰는지, 맨 왼쪽의 위조값과 다른지 확인합니다. 서버가 프록시를 거쳐서만 닿을 때만 맞는 규칙입니다.

## 한 단계 더 — 연결 하나에 고루틴 하나

`net/http` 의 `Serve` 주석은 "creating a new service goroutine for each" 라고 적습니다. 연결마다 고루틴이 하나입니다. 고루틴을 쓰지 않는 날 TCP 클라이언트 100개로 요청을 하나씩 보내고 연결을 열어 둔 채 `runtime.NumGoroutine` 을 셌습니다. `ReadHeaderTimeout` 은 두 경우 모두 1초입니다.

```text
== IdleTimeout=1s
시작: 2
쉬는 연결 100개: 102
1.5초 뒤: 2
첫 연결 읽기: EOF
== IdleTimeout=0s
시작: 2
쉬는 연결 100개: 102
1.5초 뒤: 102
첫 연결 읽기: read tcp 127.0.0.1:PORT->127.0.0.1:PORT: i/o timeout
```

세 번 돌려 같은 수가 나왔습니다. 쉬는 연결 100개가 고루틴 100개였고, `IdleTimeout` 이 연결과 고루틴을 함께 거뒀습니다. 두 번째 경우는 `ReadHeaderTimeout` 이 있어도 쉬는 연결이 그대로였고, 표의 "0 이면" 칸 그대로입니다. 고루틴은 싸도 연결 수만큼 늘어나므로, 타임아웃 네 개가 그 수의 상한입니다.

## 면접에서 이렇게 묻는다

**Q1. `http.Server` 의 타임아웃 네 가지와 느린 헤더 공격을 설명하십시오.**
ReadHeader·Read·Write·Idle 이고 기본값은 모두 없음입니다. 헤더를 천천히 보내는 연결은 `ReadHeaderTimeout` 이 없으면 끝없이 고루틴과 소켓을 잡습니다. 본문 속도는 핸들러가 정하게 두고 헤더 타임아웃은 반드시 줍니다.

**Q2. 클라이언트가 요청을 끊으면 서버 핸들러는 어떻게 됩니까?**
`r.Context()` 가 취소됩니다. 그 context 를 DB·외부 호출까지 넘겨야 일이 멈춥니다. 넘기지 않으면 아무도 받지 않을 결과를 끝까지 계산합니다.

**Q3. 미들웨어 순서는 왜 중요합니까?**
요청은 바깥부터, 패닉과 반환은 안쪽부터 지납니다. 복구가 접근 로그 바깥에 있으면 패닉 난 요청이 로그에서 빠집니다. 인가는 등록 지점에서 감쌉니다.

**Q4. Go HTTP 클라이언트에서 연결이 재사용되지 않는 흔한 원인은 무엇입니까?**
Body 를 끝까지 읽지 않고 닫거나, 요청마다 `Transport` 를 새로 만드는 경우입니다. `httptrace` 의 `GotConn.Reused` 로 확인합니다. 타임아웃 없는 `DefaultClient` 를 운영 코드에 쓰지 않는다는 점도 덧붙입니다.

**Q5. 재시도와 레이트 리미터를 설계할 때 무엇을 봅니까?**
재시도는 멱등 요청과 429·5xx·네트워크 오류에만 합니다. 지수 백오프에 무작위 지연을 섞고, context 가 끝나면 멈춥니다. 리미터는 키마다 기억할 양과 버스트로 고릅니다. 키가 될 IP 는 신뢰하는 프록시가 넣은 값만 씁니다.

## 연습

### 연습 1 — 접근 로그에 응답 바이트 수 남기기

접근 로그에 `bytes=` 를 더합니다. 핸들러가 `"hello"` 를 쓰면 `status=200 bytes=5` 가 남아야 합니다.

<details>
<summary>풀이</summary>

기록기에 `Write` 를 덧씌워 쓴 바이트를 셉니다. `Unwrap` 도 그대로 둡니다.

```go
func (r *recorder) Write(b []byte) (int, error) {
	n, err := r.ResponseWriter.Write(b)
	r.bytes += n
	return n, err
}
```

```text
--- PASS: TestAccessLogCountsBytes (0.00s)
```

`WriteHeader` 없이 바로 `Write` 하는 핸들러가 많아 상태 코드 초깃값은 200 입니다.

</details>

### 연습 2 — 토큰 버킷을 미들웨어로 걸기

IP 마다 토큰 버킷(초당 1개, 최대 2개)을 적용하고, 막을 때는 429 와 `Retry-After` 를 줍니다. 같은 IP 의 세 번째 요청은 429, 다른 IP 는 200 이어야 합니다.

<details>
<summary>풀이</summary>

```go
func RateLimit(tb *prodhttp.TokenBucket, retryAfter time.Duration, now func() time.Time) prodhttp.Middleware {
	return func(next http.Handler) http.Handler {
		return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			if !tb.Allow(prodhttp.ClientIP(r), now()) {
				w.Header().Set("Retry-After", strconv.Itoa(int(retryAfter.Seconds())))
				http.Error(w, "too many requests", http.StatusTooManyRequests)
				return
			}
			next.ServeHTTP(w, r)
		})
	}
}
```

```text
--- PASS: TestRateLimitPerIP (0.00s)
```

시각을 `now func() time.Time` 으로 받아 시험에서 시계를 멈춥니다.

</details>

## 정리

Go 서버가 운영에서 멈추거나 새는 것은 `net/http` 가 기본으로 무한정 기다리기 때문입니다. 서버에는 타임아웃 네 개, 요청에는 context, 라우트에는 등록 지점의 미들웨어를 줍니다. 종료는 정한 순서로 하고, 클라이언트는 타임아웃을 주고 재사용합니다. 다음에 서버 코드를 열면 `http.Server{…}` 에 `ReadHeaderTimeout` 이 있는지부터 봅니다.

11편에서는 이 서버 뒤의 데이터베이스를 다룹니다. `database/sql` 의 연결 풀과 트랜잭션, context 로 쿼리를 취소하는 법을 봅니다.
