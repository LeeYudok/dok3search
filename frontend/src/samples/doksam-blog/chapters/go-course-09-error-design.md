# Go 강의 9편 — 에러는 층마다 감싸고, 기록과 상태 코드는 맨 위에서 한 번 정한다

> 에러가 저장소 → 서비스 → 핸들러로 올라올 때 누가 감싸고, 누가 기록하고, 누가 상태 코드를 정합니까?

아래 층은 자기만 아는 맥락을 한 번 붙여 감싸고 반환만 합니다. 기록과 상태 코드는 맨 위 핸들러의 실패 처리 함수 하나가 정합니다. 클라이언트에게는 상태 코드와 정해 둔 문구만 보내고, 자세한 원인은 로그 한 줄에 남깁니다. 에러를 값으로 다루는 기초(센티널, 타입, `%w`, `errors.Is`·`As`)는 [3편](https://blog.doksam.com/blog/go-course-03-interfaces-and-errors/)에서 다뤘습니다. 이 편은 그 도구를 층이 나뉜 서비스에 배치합니다.

> [그림] 3층 건물 단면. 1층 저장소에서 디스크 쓰기가 실패해 봉투에 스티커가 붙고, 2층 서비스에서 스티커가 하나 더 붙는다. 3층 핸들러의 책상에서만 로그를 적고 500 도장을 찍으며, 사용자 화면에는 일반 문구만 간다

그림의 봉투가 에러입니다. 한 층을 오를 때마다 맥락 스티커가 하나 붙고, 로그와 상태 코드 도장은 맨 위 책상에만 있습니다.

출력은 2026-11-18 에 맥(Apple M4 Pro)과 Go 1.26.2 로 돌린 결과입니다.

## 층마다 자기만 아는 맥락을 한 번 붙인다

저장소는 무엇을 하다 실패했는지를, 서비스는 어떤 업무였는지를 붙입니다.

```go
// 저장소
if err := os.WriteFile(filepath.Join(s.Dir, m.ID+".json"), b, 0o600); err != nil {
	return fmt.Errorf("디스크 쓰기: %w", err)
}
// 서비스
if err := s.Store.Save(ctx, m); err != nil {
	return Message{}, fmt.Errorf("메시지 저장: %w", err)
}
```

저장 폴더가 없을 때 맨 위에 도착한 에러는 이렇습니다.

```text
메시지 저장: 디스크 쓰기: open /tmp/inbox-demo-없는폴더/m1.json: no such file or directory
```

왼쪽부터 읽으면 업무 → 동작 → 원인 순서입니다. `os` 에러가 이미 `open <경로>` 를 담고 있어서 저장소는 경로를 다시 적지 않습니다. 모든 에러는 실패이니 "실패"도 붙이지 않습니다. 둘 다 어기면 어떻게 되는지는 아래 기록 실험에 나옵니다.

## 저장소 에러는 도메인 센티널로 번역한다

서비스가 보기에 파일 없음은 "메시지 없음"이고, 번역은 저장소가 맡습니다.

```go
b, err := os.ReadFile(filepath.Join(s.Dir, id+".json"))
if errors.Is(err, fs.ErrNotExist) {
	return Message{}, fmt.Errorf("%w: %v", ErrNotFound, err)
}
```

```text
[조회 실패] 메시지 m9 조회: 메시지 없음: open /tmp/inbox-demo-없는폴더/m9.json: no such file or directory
  Is(ErrNotFound) = true  Is(fs.ErrNotExist) = false
```

`ErrNotFound` 는 `%w` 로, 파일 시스템 에러는 `%v` 로 붙였습니다. 원인은 글자로 로그에 남고, 호출부가 분기할 수 있는 값은 도메인 센티널 하나뿐입니다. 3편에서 사슬을 끊는 실수로 본 `%v` 를 여기서는 일부러 씁니다. 저장소를 파일에서 DB 로 바꿔도 `fs.ErrNotExist` 에 기대는 호출부가 없으니 위층은 그대로입니다.

## 상태 코드는 함수 하나가 정한다

```go
func errorStatus(err error) int {
	if _, ok := errors.AsType[*ValidationError](err); ok {
		return http.StatusBadRequest
	}
	switch {
	case errors.Is(err, ErrNotFound):
		return http.StatusNotFound
	case errors.Is(err, context.DeadlineExceeded):
		return http.StatusServiceUnavailable
	default:
		return http.StatusInternalServerError
	}
}
```

`errors.AsType` 은 Go 1.26 에 들어왔습니다(`api/go1.26.txt`). `Is`·`As` 는 `Unwrap() []error` 로 묶인 트리도 깊이 우선으로 훑어서(`errors` 문서) Join 으로 묶은 검증 실패도 400 이 됩니다. 이 매핑은 표 주도 테스트로 고정합니다.

| 에러 | 상태 | ERROR 기록 |
|---|---|---|
| `*ValidationError` (Join 포함) | 400 | 안 함 |
| `ErrNotFound` | 404 | 안 함 |
| `context.DeadlineExceeded` | 503 | 함 |
| 그 밖 | 500 | 함 |

4xx 는 클라이언트의 실수입니다. ERROR 로 남기면 잘못된 요청 몇 개로 경보가 울립니다.

## 기록은 맨 위에서 한 번, 응답에는 정해 둔 문구만 보낸다

```go
func (h *Handler) fail(w http.ResponseWriter, r *http.Request, err error) {
	status := errorStatus(err)
	body := map[string]any{"ok": false}
	switch {
	case status == http.StatusBadRequest:
		body["error"] = "입력을 확인해 주세요."
		body["fields"] = invalidFields(err)
	case status == http.StatusNotFound:
		body["error"] = "없는 메시지입니다."
	default:
		h.Log.ErrorContext(r.Context(), "요청 실패",
			"method", r.Method, "path", r.URL.Path, "status", status, "err", err)
		body["error"] = "일시적인 오류입니다. 잠시 후 다시 시도해 주세요."
	}
	writeJSON(w, status, body)
}
```

요청 세 개를 보낸 결과입니다. 로그의 시각 칸은 지면 때문에 뺐습니다.

```text
POST /api/messages → 400 {"error":"입력을 확인해 주세요.","fields":["replyTo","body"],"ok":false}
GET /api/messages/m9 → 404 {"error":"없는 메시지입니다.","ok":false}
level=ERROR msg="요청 실패" method=POST path=/api/messages status=500 err="메시지 저장: 디스크 쓰기: open /tmp/inbox-demo-없는폴더/m1.json: no such file or directory"
POST /api/messages → 500 {"error":"일시적인 오류입니다. 잠시 후 다시 시도해 주세요.","ok":false}
```

응답에는 경로도 `os` 에러도 없습니다. 내부 구조는 공격자의 지도가 됩니다. 이 사이트의 Go 서버도 문의 저장이 실패하면 한 번 기록하고 일반 문구로 503 을 돌려줍니다. DB 접속 주소는 해석에 실패해도 로그에 쓰지 않습니다. `log/slog` 는 Go 1.21 에 들어왔습니다(`api/go1.21.txt`).

층마다 기록하고 반환하면 실패 하나가 세 줄이 됩니다.

```text
level=ERROR msg="파일 열기 실패" path=/tmp/없는파일.json err="open /tmp/없는파일.json: no such file or directory"
level=ERROR msg="저장 실패" err="파일 /tmp/없는파일.json 열기 실패: open /tmp/없는파일.json: no such file or directory"
level=ERROR msg="요청 실패" status=500 err="저장 실패: 파일 /tmp/없는파일.json 열기 실패: open …"
```

마지막 줄은 줄였습니다. 같은 경로와 "실패"가 줄마다 되풀이되고, 장애 하나가 경보 세 건이 됩니다. 세 줄을 같은 요청으로 묶을 단서도 없습니다.

## 실패의 종류는 센티널·타입·동작 셋 중 하나로 묻는다

시간 초과는 출처마다 모양이 다릅니다. 세 출처의 시간 초과를 세 방식으로 물었습니다.

```text
http.Client.Timeout
  Get "http://127.0.0.1:…": context deadline exceeded (Client.Timeout exceeded while awaiting headers)
  Timeout()=true  Is(context.DeadlineExceeded)=true  Is(os.ErrDeadlineExceeded)=false
요청 context 기한
  Get "http://127.0.0.1:…": context deadline exceeded
  Timeout()=true  Is(context.DeadlineExceeded)=true  Is(os.ErrDeadlineExceeded)=false
net.Conn 읽기 기한
  read tcp 127.0.0.1:…->127.0.0.1:…: i/o timeout
  Timeout()=true  Is(context.DeadlineExceeded)=false  Is(os.ErrDeadlineExceeded)=true
```

센티널 하나로는 셋을 다 잡지 못했고, `Timeout() bool` 메서드가 있는지 묻는 동작 확인만 셋 모두 참이었습니다. 출처가 여러 패키지면 메서드로 묻는 편이 결합이 적습니다(연습 1).

## 여러 실패는 errors.Join 으로 모으고 Close 에러도 버리지 않는다

검증은 첫 실패에서 멈추지 않고 어긴 칸을 모두 모읍니다. `errors.Join` 은 Go 1.20 에 들어왔고, 문서대로 모두 nil 이면 nil 을 돌려줍니다.

```text
replyTo: 비어 있음
body: 비어 있음
name: 100자 초과
```

쓰기 쪽의 `Close` 는 실패할 수 있습니다. `go doc compress/gzip.Writer.Close` 대로 Close 가 남은 데이터와 꼬리말을 쓰기 때문입니다. 쓰기는 되고 Close 만 실패하는 가짜 파일로 견줘 봅니다.

```go
func save(f io.WriteCloser, b []byte) (err error) {
	defer func() { err = errors.Join(err, f.Close()) }()
	_, err = f.Write(b)
	return err
}
```

```text
Close 를 버림: <nil>
Close 를 합침: 디스크로 비우기: no space left on device
```

`defer f.Close()` 만 쓰면 데이터가 디스크에 닿지 않았는데도 성공이라고 답합니다. 이름 붙인 반환값을 `defer` 에서 고쳐 Close 에러를 합칩니다.

## 취소에는 원인을 붙여 둔다

`ctx.Err()` 는 `context canceled` 와 `context deadline exceeded` 둘뿐이라 왜 취소됐는지 모릅니다. Go 1.20 의 `WithCancelCause`·`Cause`, Go 1.21 의 `WithTimeoutCause` 가 원인을 붙입니다.

```text
Err:   context canceled
Cause: 클라이언트가 업로드를 취소함
Err:   context deadline exceeded
Cause: 저장소 응답이 2초를 넘김
Is(Err, DeadlineExceeded) = true  Is(Cause, errSlowStore) = true
```

`Err` 는 그대로라 기존 분기는 깨지지 않고, 로그에는 `context.Cause(ctx)` 를 남깁니다. 제 예약 실행기는 `errors.Is(ctx.Err(), context.DeadlineExceeded)` 로 시간 초과와 실행 실패를 갈라 알림을 다르게 씁니다.

## 패닉은 요청 경계에서 한 번 잡는다

패닉은 nil 맵에 쓰기 같은 프로그래머 오류에만 씁니다. 핸들러가 패닉을 내면 `net/http` 가 잡습니다.

```text
2026/11/18 01:32:25 http: panic serving 127.0.0.1:…: assignment to entry in nil map
클라이언트: Get "http://127.0.0.1:…/boom": EOF
```

`go doc net/http.Handler` 대로 서버는 패닉을 복구하고 스택을 남긴 뒤 연결을 닫습니다. 클라이언트는 상태 코드 없이 `EOF` 를 받고, 로그는 `Server.ErrorLog`(비우면 표준 `log`)로 가서 요청 필드가 붙지 않습니다. 그래서 경계에 미들웨어를 둡니다.

```go
defer func() {
	v := recover()
	if v == nil {
		return
	}
	if v == http.ErrAbortHandler {
		panic(v)
	}
	h.Log.ErrorContext(r.Context(), "패닉", "path", r.URL.Path, "panic", v, "stack", string(debug.Stack()))
	writeJSON(w, http.StatusInternalServerError, map[string]any{"ok": false, "error": "일시적인 오류입니다. 잠시 후 다시 시도해 주세요."})
}()
```

`http.ErrAbortHandler` 는 응답을 끊되 스택을 남기지 않는 약속된 패닉 값이라(`go doc net/http.ErrAbortHandler`) 다시 던집니다. 실험에서도 `/abort` 는 로그 없이 `EOF` 로 끝났습니다. Go 에러에는 스택이 없습니다. 위치는 층마다 붙인 맥락이 말하고, 스택은 패닉 경계에서만 남깁니다.

## 한 단계 더 — 에러의 비용과 설계 선택

에러를 만드는 비용을 쟀습니다. `-benchmem -count=5` 의 중앙값입니다. 2026-11-18 에 다른 작업이 함께 돌던 맥(Apple M4 Pro, Go 1.26.2)에서 쟀습니다.

| 방식 | ns/op | B/op | allocs/op |
|---|---:|---:|---:|
| 미리 만든 센티널 반환 | 1.862 | 0 | 0 |
| 호출마다 `errors.New` | 10.23 | 16 | 1 |
| `fmt.Errorf("…%s…: %w", id, err)` | 94.39 | 96 | 3 |
| `errors.Is`, 사슬 깊이 1 | 5.343 | 0 | 0 |
| `errors.Is`, 사슬 깊이 10 | 26.32 | 0 | 0 |
| `errors.Is`, 사슬 깊이 100 | 238.6 | 0 | 0 |

감싸기 한 번은 약 94ns, 할당 3번이라 디스크나 네트워크를 기다리는 요청에서는 보이지 않습니다. 읽을 때마다 나오는 끝 신호는 다릅니다. `go doc io.EOF` 는 호출부가 `==` 로 비교하니 `io.EOF` 를 감싸지 말라고 적습니다. `errors.Is` 는 `errors/wrap.go` 의 반복문이 `Unwrap` 을 하나씩 따라가서 깊이에 비례해 느려졌습니다. 층마다 한 번만 감싸면 깊이는 층 수에 머뭅니다.

| 방식 | 예 | 맞는 곳 | 호출부에 약속하는 것 |
|---|---|---|---|
| 센티널 | `ErrNotFound`, `io.EOF` | 분기만 하면 될 때 | 값 하나 |
| 타입 | `*ValidationError`, `*fs.PathError` | 칸 이름 같은 정보가 더 필요할 때 | 타입과 필드 |
| 동작 | `interface{ Timeout() bool }` | 출처가 여러 패키지일 때 | 메서드 하나 |
| 에러 코드 | 외부 API 의 `"code"` 값 | 프로세스 경계를 넘을 때 | 코드 표 문서 |

## 면접에서 이렇게 묻는다

**Q1. 에러는 어디서 기록합니까?**
맨 위 한 곳입니다. 아래 층은 맥락을 붙여 반환만 합니다. 둘 다 하면 실패 하나가 세 줄이 된 실험을 예로 듭니다.

**Q2. 도메인 에러를 HTTP 상태 코드로 어떻게 바꿉니까?**
함수 하나가 `Is`·`AsType` 으로 매핑하고 표 주도 테스트로 고정합니다. 내부 문구는 로그에만, 응답에는 정해 둔 문구만 보냅니다. 시간 초과의 원인은 `context.Cause` 로 로그에 남깁니다.

**Q3. 센티널, 타입, 동작 확인은 각각 언제 씁니까?**
분기만 하면 센티널, 정보가 더 필요하면 타입, 출처가 여러 패키지면 동작입니다. 시간 초과도 출처마다 센티널이 갈리고 `Timeout()` 만 셋 모두 참이었습니다.

**Q4. `defer f.Close()` 의 에러는 어떻게 다룹니까?**
쓰기 쪽은 Close 가 남은 데이터를 내보내므로 버리면 실패를 성공으로 보고합니다. 이름 붙인 반환값과 `errors.Join` 으로 합칩니다.

**Q5. 패닉은 어디서 recover 합니까?**
요청 하나의 경계에서 한 번입니다. `net/http` 도 잡지만 클라이언트는 `EOF` 를 받으니 미들웨어가 500 과 한 줄 기록으로 바꿉니다. `http.ErrAbortHandler` 는 다시 던집니다. 이 줄을 지우면 일부러 끊은 응답이 500 과 ERROR 로그로 바뀌는 것을 테스트가 잡았습니다.

## 연습

### 연습 1 — 시간 초과를 동작으로 묻기

`net.Conn` 의 읽기 기한 에러(`os.ErrDeadlineExceeded`)를 감싼 에러가 지금 어떤 상태 코드가 되는지 테스트로 먼저 보이고, 503 이 되게 고칩니다.

<details>
<summary>풀이</summary>

`os.ErrDeadlineExceeded` 를 감싼 에러가 503 인지 보는 테스트는 먼저 실패합니다.

```text
    status_test.go:12: got = 500, 원하는 값 503
FAIL
```

`switch` 의 시간 초과 줄에 동작 확인을 더합니다.

```go
var te interface{ Timeout() bool }
switch {
case errors.Is(err, ErrNotFound):
	return http.StatusNotFound
case errors.Is(err, context.DeadlineExceeded), errors.As(err, &te) && te.Timeout():
	return http.StatusServiceUnavailable
```

고친 뒤 이 테스트와 기존 테스트가 모두 통과했습니다(`ok example.com/inbox`).

</details>

### 연습 2 — 두 겹의 Close

`gzip.Writer` 로 압축해 `io.WriteCloser` 에 쓰는 `WriteGzip(w, data) error` 를 만듭니다. 두 Close 의 에러를 모두 돌려주고, 바깥 Close 가 실패해도 압축 데이터는 온전해야 합니다.

<details>
<summary>풀이</summary>

```go
func WriteGzip(w io.WriteCloser, data []byte) (err error) {
	defer func() { err = errors.Join(err, w.Close()) }()
	zw := gzip.NewWriter(w)
	defer func() { err = errors.Join(err, zw.Close()) }()
	_, err = zw.Write(data)
	return err
}
```

`defer` 는 나중에 건 것부터 돌아서 `zw.Close()` 가 꼬리말을 쓴 뒤 `w.Close()` 가 돕니다. 바깥 Close 가 실패하는 경우도 압축을 풀면 원문이 나왔습니다(`--- PASS: TestWriteGzip`).

</details>

## 정리

에러는 층마다 자기 맥락을 한 번 붙여 올려 보내고, 맨 위 한 곳이 상태 코드와 기록을 정합니다. 응답에는 정해 둔 문구만 나갑니다. Close 에러와 취소 원인은 버리지 않고, 패닉은 경계 한 곳에서 잡습니다.

10편에서는 이 핸들러를 감싸는 서버 쪽을 다룹니다. 타임아웃 네 가지, 미들웨어 순서, 그레이스풀 종료를 운영 수준으로 맞춥니다.
