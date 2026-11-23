# Go 강의 4편 — 표준 라이브러리만으로 API 서버가 된다

> 프레임워크 없이 Go 표준 라이브러리만으로 API 서버를 만들 수 있습니까?

만들 수 있습니다. Go 1.22 부터 `net/http` 의 라우터 `ServeMux` 가 메서드와 경로 변수를 받습니다. JSON 은 `encoding/json` 이, 정적 파일은 실행 파일에 파일을 넣는 `go:embed` 가 맡고, 설정은 환경변수면 됩니다. 이 편에서는 이 사이트의 Go 서버를 줄인 문의 접수 서버 `inbox` 를 외부 모듈 없이 만듭니다.

> [그림] 우체국 벽의 칸마다 GET /healthz 같은 패턴이 적혀 있고, 배달 로봇이 상자를 맞는 칸으로 나른다. 메서드가 다른 편지에는 405 도장이 찍혔다

라우터는 우체국의 칸막이 벽입니다. 메서드와 경로가 맞는 칸에 요청이 들어가고, 메서드만 다르면 405 로 되돌아갑니다.

## 라우팅 표 하나가 서버의 지도다

```go
func newMux(st *store, static fs.FS) *http.ServeMux {
	mux := http.NewServeMux()
	mux.HandleFunc("GET /healthz", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "text/plain; charset=utf-8")
		fmt.Fprintln(w, "ok")
	})
	mux.HandleFunc("POST /api/messages", st.create)
	mux.HandleFunc("GET /api/messages/{id}", st.show)
	mux.HandleFunc("GET /api/", func(w http.ResponseWriter, r *http.Request) {
		writeError(w, http.StatusNotFound, "없는 주소입니다")
	})
	mux.Handle("GET /", http.FileServerFS(static))
	return mux
}
```

패턴은 `[메서드 ][호스트]/경로` 모양입니다. `{id}` 는 경로 한 칸을 받는 와일드카드(wildcard, 자리표시)이고 `r.PathValue("id")` 로 꺼냅니다. `newMux` 가 mux 를 돌려주니 테스트도 같은 라우팅을 씁니다. 서버를 띄우고 `curl` 로 본 결과입니다.

| 요청 | 받은 패턴 | 응답 |
|---|---|---|
| `GET /healthz` | `GET /healthz` | 200 `ok` |
| `POST /api/messages` | `POST /api/messages` | 201 `{"id":"1",…}` |
| `GET /api/messages/1` | `GET /api/messages/{id}` | 200 |
| `GET /api/messages` | `GET /api/` | 404 `{"error":"없는 주소입니다"}` |
| `POST /healthz` | 없음 | 405, `Allow: GET, HEAD` |
| `GET /` | `GET /` | 200 `index.html` |

`GET` 패턴은 `HEAD` 도 받아서(`go doc net/http ServeMux`) `Allow` 에 둘이 함께 나옵니다. `PathValue` 와 `http.FileServerFS` 는 Go 1.22 에 들어왔습니다(`api/go1.22.txt`).

## 메서드가 틀리면 405 지만, 폴백이 있으면 폴백이 받는다

경로는 맞는데 메서드가 맞는 패턴이 없으면 `ServeMux` 가 405 와 `Allow` 헤더를 직접 만듭니다. 그런데 표의 `GET /api/messages` 는 404 입니다. 폴백 `GET /api/` 때문입니다. `POST /api/messages` 만 등록한 라우터로 따로 실험했습니다.

```text
GET /api/messages → 405 Allow="POST" "Method Not Allowed\n"
-- 메서드 없는 폴백을 더하면
GET /api/messages → 200 Allow="" "fallback"
```

폴백이 맞는 요청은 폴백이 받습니다. API 전체에 JSON 404 를 주는 대신 정확한 405 를 잃습니다.

경로 변수는 디코딩한 값입니다. `GET /files/{name}` 에 `/files/a%2Fb` 를 보내면 `name` 은 `a/b` 가 됩니다. 파일 이름으로 쓰기 전에 검증합니다.

## JSON 은 구조체로 받고, 본문 크기부터 자른다

```go
const maxBody = 64 << 10 // 64KiB

func (st *store) create(w http.ResponseWriter, r *http.Request) {
	var in newMessage
	if err := json.NewDecoder(http.MaxBytesReader(w, r.Body, maxBody)).Decode(&in); err != nil {
		if _, ok := errors.AsType[*http.MaxBytesError](err); ok {
			writeError(w, http.StatusRequestEntityTooLarge, "본문이 너무 큽니다")
			return
		}
		writeError(w, http.StatusBadRequest, "JSON 형식이 아닙니다")
		return
	}
	in.ReplyTo, in.Body = strings.TrimSpace(in.ReplyTo), strings.TrimSpace(in.Body)
	if in.ReplyTo == "" || in.Body == "" {
		writeError(w, http.StatusBadRequest, "연락처와 내용은 필수입니다")
		return
	}
	m := st.add(Message{Name: in.Name, ReplyTo: in.ReplyTo, Body: in.Body})
	writeJSON(w, http.StatusCreated, m)
}
```

`newMessage` 는 `json:"replyTo"` 같은 태그로 필드 이름을 정한 구조체입니다. `http.MaxBytesReader` 는 이 사이트 서버와 같은 64KiB 에서 읽기를 끊고 `*http.MaxBytesError`(Go 1.19)를 냅니다. 70,000자 본문을 보낸 응답입니다.

```text
HTTP/1.1 413 Request Entity Too Large
Cache-Control: no-store
Connection: close
…
```

`MaxBytesReader` 소스 주석대로 한도를 넘은 연결은 닫힙니다(`Connection: close`). `errors.AsType` 은 Go 1.26 기능이라 `go.mod` 가 낮으면 `errors.As` 를 씁니다.

디코딩 에러는 그대로 돌려주지 않습니다. `{"replyTo":1}` 의 에러 `json: cannot unmarshal number into Go struct field newMessage.replyTo of type string` 에는 내부 타입 이름이 들어 있습니다.

이 사이트 서버는 구조체 대신 `map[string]any` 로 받습니다. 둘은 이렇게 다릅니다.

| 기준 | 구조체 + 태그 | `map[string]any` |
|---|---|---|
| 타입이 틀린 값 | `Decode` 가 에러를 낸다 | 꺼낼 때 직접 검사한다 |
| 코드 양 | 적다 | 값을 꺼내는 함수가 필요하다 |
| 모르는 필드 | 조용히 버린다 | 그대로 들어온다 |

모르는 필드는 `dec.DisallowUnknownFields()` 로 막습니다. `website` 가 섞인 요청은 `json: unknown field "website"` 로 거절됐습니다. 이 사이트 서버는 봇 함정으로 그 필드를 일부러 받으니 엄격함에도 비용이 따릅니다.

## 응답 헤더는 WriteHeader 전에 쓴다

```go
func writeJSON(w http.ResponseWriter, status int, v any) {
	w.Header().Set("Content-Type", "application/json; charset=utf-8")
	w.Header().Set("Cache-Control", "no-store")
	w.WriteHeader(status)
	if err := json.NewEncoder(w).Encode(v); err != nil {
		slog.Warn("응답 쓰기 실패", "err", err) // 상태 코드는 이미 나갔다
	}
}
```

`WriteHeader` 뒤에 넣은 `Content-Type` 은 응답에 남지 않았고, `ResponseWriter.Header` 문서도 그렇게 적습니다. 200 을 쓴 뒤 `math.Inf(1)` 을 인코딩하면 `json: unsupported value: +Inf` 가 나고 본문은 비었습니다. 상태 코드를 되돌릴 수 없으니 기록만 남깁니다. `no-store` 는 중간 캐시가 문의 내용을 들고 있지 않게 합니다.

## 설정은 환경변수로 받고 기본값을 둔다

```go
func env(key, fallback string) string {
	if v := os.Getenv(key); v != "" {
		return v
	}
	return fallback
}

// main 안에서
	srv := &http.Server{
		Addr:              ":" + env("PORT", "3000"),
		Handler:           newMux(newStore(), staticFS()),
		ReadHeaderTimeout: 10 * time.Second,
	}
```

`env` 는 이 사이트 서버의 함수 그대로입니다. 같은 실행 파일을 설정만 바꿔 띄우고, 비밀 값을 저장소 밖에 둡니다. 이 사이트 서버는 DB 주소를 못 읽어도 "해석하지 못했다" 만 남기고 값은 로그에 쓰지 않습니다.

`ReadHeaderTimeout` 은 요청 헤더를 다 받을 때까지 기다리는 상한입니다. 빈 줄 없이 헤더 한 줄만 보내고 기다리는 클라이언트로 쟀습니다.

```text
ReadHeaderTimeout 1초: 1.0초 뒤 "", err=<nil>
타임아웃 없음: 3.0초 뒤 "", err=read tcp …: i/o timeout
```

설정이 없으면 클라이언트가 3초 뒤 포기할 때까지 연결이 열려 있었습니다. 나머지 타임아웃은 10편에서 다룹니다.

## 정적 파일은 go:embed 로 실행 파일에 넣는다

```go
//go:embed static
var embedded embed.FS

func staticFS() fs.FS {
	if dir := os.Getenv("STATIC_DIR"); dir != "" {
		return os.DirFS(dir)
	}
	sub, err := fs.Sub(embedded, "static")
	if err != nil {
		panic(err) // 패턴이 컴파일 때 확인되므로 여기 오면 프로그래머 오류다
	}
	return sub
}
```

`go:embed` 와 `io/fs` 는 Go 1.16 에 들어왔습니다(`api/go1.16.txt`). `fs.Sub` 는 `static/` 을 뿌리로 바꿉니다. `embed.FS` 와 `os.DirFS` 가 둘 다 `fs.FS` 라서 출처만 바꿔 끼웁니다.

`go:embed` 의 규칙 셋을 빌드로 확인했습니다.

```text
$ go build ./_fails/up        # //go:embed ../README.md
_fails/up/main.go:5:12: pattern ../README.md: invalid pattern syntax
$ go build ./_fails/nomatch   # //go:embed dist (폴더 없음)
_fails/nomatch/main.go:5:12: pattern dist: no matching files found
$ go run ./app                # static 에 index.html, .env, _draft.html
static: static/index.html
all:static: static/.env static/_draft.html static/index.html
```

두 출처를 견주면 이렇습니다.

| 기준 | `go:embed` | `os.DirFS` |
|---|---|---|
| 배포물 | 실행 파일 하나 | 실행 파일 + 폴더 |
| 파일을 바꾸려면 | 다시 빌드 | 파일만 바꾼다 |
| `Last-Modified`·304 | 없음(항상 200) | 있음(304) |
| 폴더의 `.env` | 들어가지 않아 404 | 그대로 200 으로 나간다 |

디스크 폴더를 내보낼 때는 그 폴더의 내용이 곧 공개 범위입니다.

## 한 단계 더 — 충돌은 등록할 때 터지고, 요청마다 고루틴이 돈다

**가장 구체적인 패턴이 이깁니다.** 문서는 P1 이 받는 요청이 P2 의 진부분집합이면 P1 이 더 구체적이라고 정의합니다. 실험에서 `/api/messages/latest` 는 `{id}` 대신 글자 그대로의 `latest` 패턴이 받았습니다. 우열이 없으면 등록할 때 패닉이 납니다.

```text
panic: pattern "/api/" (registered at …/main.go:26) conflicts with pattern "GET /" (registered at …/main.go:26):
/api/ matches more methods than GET /, but has a more specific path pattern
```

자연스럽게 쓰는 조합이라 더 위험합니다. 그래서 `inbox` 는 폴백도 `GET /api/` 로 등록했습니다. 이 사이트 서버는 정적 파일을 메서드 없는 `/` 에 두고 메서드를 핸들러에서 검사합니다.

**`go.mod` 의 `go` 줄이 라우팅을 바꿉니다.** `GET /hello/{name}` 하나를 등록한 프로그램을 돌렸습니다.

| 조건 | `GET /hello/go` |
|---|---|
| `go 1.22` | 200 `hello go` |
| `go 1.21` | 404 |
| `go 1.22` + `GODEBUG=httpmuxgo121=1` | 404 |

`go 1.21` 모듈에서는 새 문법이 옛 규칙으로 해석돼 조용히 404 가 됩니다. 문서가 말하는 `httpmuxgo121=1` 의 기본값을 `go` 줄이 정하는 것입니다. 빌드는 통과하지만 `go vet` 은 `http.PathValue requires go1.22 or later (module is go1.21)` 로 잡았습니다.

**연결마다 고루틴이 하나씩 돕니다.** `net/http` 의 `Serve` 주석은 연결마다 새 고루틴을 만든다고 적습니다(`server.go` 의 `go c.serve(connCtx)`). 200ms 걸리는 핸들러에 요청 다섯 개를 동시에 보냈습니다.

```text
요청 5개(각 200ms): 0.20초, 고루틴 2 → 최대 27
```

세 번 모두 0.20초였습니다(2026-11-18, Apple M4 Pro, Go 1.26.2). 고루틴 수에는 클라이언트 쪽 것도 섞여 있습니다. 핸들러가 동시에 돌아서 `store` 의 맵은 뮤텍스로 지켰고, 안 지키면 어떻게 되는지는 5편에서 `-race` 로 봅니다.

## 면접에서 이렇게 묻는다

**Q1. 두 패턴이 한 요청에 함께 맞으면 어느 쪽이 받습니까?**
받는 요청이 더 좁은 패턴, 즉 더 구체적인 패턴이 받습니다. 우열이 없으면 등록할 때 패닉이 나고, `GET /` 와 `/api/` 가 그 예입니다.

**Q2. 405 는 언제 나오고, 언제 안 나옵니까?**
경로는 맞고 메서드가 맞는 패턴이 없을 때 `Allow` 헤더와 함께 나옵니다. 그 요청에 맞는 폴백이 있으면 폴백이 받아 405 가 나오지 않습니다.

**Q3. 큰 본문과 느린 클라이언트는 어떻게 막습니까?**
본문은 `http.MaxBytesReader` 로 자르고 `*http.MaxBytesError` 를 413 으로 바꿉니다. 헤더를 느리게 보내는 연결은 `ReadHeaderTimeout` 으로 끊습니다. 설정이 없으면 클라이언트가 끊을 때까지 열려 있습니다.

**Q4. 설정을 환경변수로 받는 이유와 주의점은 무엇입니까?**
같은 실행 파일을 다시 빌드하지 않고 환경마다 띄우고, 비밀 값을 저장소 밖에 둡니다. 비밀 값에는 기본값을 두지 않고 로그에도 남기지 않습니다.

**Q5. `go:embed` 의 제약은 무엇입니까?**
패키지 폴더와 그 아래만 가리키고, 맞는 파일이 없으면 컴파일 에러이며, `.`·`_` 파일은 `all:` 이 있어야 들어갑니다. 넣은 파일은 수정 시각이 0이라(`embed` 소스) `Last-Modified` 가 붙지 않습니다.

## 연습

### 연습 1 — 목록 엔드포인트 더하기

`GET /api/messages` 로 저장한 문의를 순서대로 돌려줍니다. 하나도 없으면 `null` 이 아니라 `[]` 여야 합니다.

<details>
<summary>풀이</summary>

```go
// list 는 저장 순서대로 돌려준다. 없으면 nil 이 아니라 빈 슬라이스다.
func (s *store) list() []Message {
	s.mu.Lock()
	defer s.mu.Unlock()
	out := make([]Message, 0, len(s.msgs))
	for i := 1; i <= s.last; i++ {
		out = append(out, s.msgs[strconv.Itoa(i)])
	}
	return out
}
```

`newMux` 에 `GET /api/messages` 를 더해 `writeJSON(w, http.StatusOK, st.list())` 로 답합니다. `GET /api/` 보다 구체적이라 폴백 대신 받습니다.

```bash
$ curl --silent 127.0.0.1:18407/api/messages
[]
$ curl --silent -X POST 127.0.0.1:18407/api/messages -d '{"replyTo":"a@example.com","body":"x"}'
{"id":"1","replyTo":"a@example.com","body":"x"}
$ curl --silent 127.0.0.1:18407/api/messages
[{"id":"1","replyTo":"a@example.com","body":"x"}]
```

`var out []Message` 로 시작하면 빈 결과가 `null` 로 나갑니다. 테스트에서 nil 슬라이스는 `null`, 빈 슬라이스는 `[]` 였습니다.

</details>

### 연습 2 — JSON 뒤에 붙은 값 거절하기

지금 서버는 `{"replyTo":"a@example.com","body":"x"} garbage` 를 201 로 받습니다. `Decode` 가 첫 값만 읽기 때문입니다. 값이 하나가 아니면 400 으로 거절합니다.

<details>
<summary>풀이</summary>

같은 디코더로 한 번 더 읽어 `io.EOF` 가 나오는지 봅니다.

```go
dec := json.NewDecoder(http.MaxBytesReader(w, r.Body, maxBody))
if err := dec.Decode(&in); err != nil {
	// 앞과 같다
}
if err := dec.Decode(&struct{}{}); !errors.Is(err, io.EOF) {
	writeError(w, http.StatusBadRequest, "JSON 값은 하나만 보냅니다")
	return
}
```

```bash
$ curl --silent -X POST 127.0.0.1:18407/api/messages -d '{"replyTo":"a@example.com","body":"x"} garbage' -w ' %{http_code}\n'
{"error":"JSON 값은 하나만 보냅니다"}
 400
```

두 번째 값이 `{}` 처럼 온전한 JSON 이어도 400 입니다(테스트로 확인).

</details>

## 정리

표준 라이브러리만으로 API 서버가 됩니다. 라우팅은 `ServeMux` 패턴이, JSON 은 구조체와 `MaxBytesReader` 가, 정적 파일은 `go:embed` 가 맡습니다. 패턴 충돌, 폴백과 405, `go` 줄의 영향은 알고 씁니다.

5편에서는 이 서버에 표 주도 테스트와 `httptest` 를 붙이고, 고루틴과 `-race` 로 `store` 를 왜 지켜야 하는지 확인합니다.
