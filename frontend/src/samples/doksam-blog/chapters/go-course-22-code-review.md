# Go 강의 22편 — 기계적 실수는 도구가 잡고, 실패와 수명은 사람이 본다

> Go 코드 리뷰에서 리뷰어는 무엇을 짚고, 도구가 잡는 것과 사람이 봐야 하는 것은 어떻게 나뉩니까?

리뷰어가 끝까지 묻는 것은 "이 코드는 실패할 때와 끝날 때 어떻게 됩니까?" 하나입니다. 형식, 형식 문자열, 락 복사, 버린 취소 함수처럼 기계로 판정되는 실수는 Go 배포판의 `gofmt`·`go vet`·`go fix` 가 먼저 잡습니다. 일부러 냄새나게 쓴 124줄 코드에서 도구 셋이 11가지를 잡았습니다. 에러 처리·이름·고루틴 수명·응답 모양 11가지는 사람이 짚어야 했습니다. 사람이 두 번 짚은 규칙은 테스트로 옮겨 다음 리뷰에서 빠지게 합니다.

> [그림] 배달 로봇이 빨간 펜과 점검표를 든 리뷰 로봇에게 상자를 건넨다. 위에는 gofmt 와 vet 표시등이 초록으로 켜져 있고, 상자에는 사람이 볼 것이라고 적힌 쪽지가 붙어 있다

초록 표시등이 도구가 통과시킨 부분이고, 상자에 붙은 쪽지가 도구가 판정하지 못해 사람이 읽을 부분입니다.

이 편은 1~21편의 내용을 리뷰 관점으로 다시 묶습니다. 아직 올라가지 않은 편은 [로드맵](https://blog.doksam.com/blog/go-course-00-roadmap/)에서 찾습니다. 출력은 2026-11-18 에 Apple M4 Pro, macOS, Go 1.26.2 로 돌렸습니다. Go 배포판에 든 도구만 썼고, 외부 린터는 돌리지 않았습니다.

## 리뷰 전 코드는 컴파일되고 돌아간다

문의 접수함의 핸들러와 주기 작업을 한 파일에 몰아 쓴 코드입니다. 빌드는 됩니다. 아래는 그 일부입니다.

```go
var ErrNotFound = errors.New("Message not found.")

type InboxStore struct {
	mu sync.Mutex
	msgs map[string]string
	ctx context.Context
}

func (this InboxStore) GetMessageById(id string) (string, error) {
	this.mu.Lock()
	defer this.mu.Unlock()
	m, ok := this.msgs[id]
	if !ok {
		return "", ErrNotFound
	} else {
		return m, nil
	}
}

func handleGet(store *InboxStore) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		ctx, _ := context.WithTimeout(r.Context(), time.Second)
		_ = ctx
		m, err := store.GetMessageById(r.PathValue("id"))
		if err == ErrNotFound {
			http.Error(w, err.Error(), 404)
			return
		}
		fmt.Fprintf(w, "%d\n", m)
	}
}
```

전체 파일에는 `interface{}`, 3단 `for`, `+=` 문자열 잇기, `wg.Add(1)` 고루틴, 멈추지 않는 작업 고루틴, 루프 안 `defer` 도 들어 있습니다.

## 배포판 도구 셋이 기계적 실수 11가지를 잡았다

```text
$ gofmt -l .
inbox.go
$ go vet ./...
inbox.go:23:12: GetMessageById passes lock by value: example.com/before.InboxStore contains sync.Mutex
inbox.go:65:8: the cancel function returned by context.WithTimeout should be called, not discarded, to avoid a context leak
inbox.go:72:19: fmt.Fprintf format %d has arg m of wrong type string
inbox.go:88:4: result of fmt.Sprintf call not used
```

`go tool fix help` 를 보면 Go 1.26.2 의 `go fix` 에는 분석기가 22개 있습니다. 대부분 새 문법·API 로 바꿔 주는 현대화 분석기(modernizer)입니다. `-diff` 는 고치지 않고 바뀔 모양만 보여 줍니다. 아래는 출력의 가운데 부분입니다. import·정렬 덩어리와 맥락 줄 일부는 `…` 로 줄였습니다.

```diff
 func contains(xs []string, x string) bool {
-	for _, v := range xs {
-		if v == x {
-			return true
-		}
-	}
-	return false
-}
-
-func joinIDs(ids []interface{}, limit int) string {
-	n := len(ids)
-	if limit < n {
-		n = limit
-	}
-	s := ""
-	for i := 0; i < n; i++ {
-		s += fmt.Sprintf("%v,", ids[i])
+	return slices.Contains(xs, x)
+}
+
+func joinIDs(ids []any, limit int) string {
+	n := min(limit, len(ids))
+	var s strings.Builder
+	for i := 0; i < n; i++ {
+		s.WriteString(fmt.Sprintf("%v,", ids[i]))
 	}
-	return s
+	return s.String()
…
-		wg.Add(1)
-		go func() {
-			defer wg.Done()
+		wg.Go(func() {
 			fmt.Sprintf("notify %s", id)
-		}()
+		})
```

3단 `for` 는 그대로 남았습니다. 고친 파일에 `go fix` 를 한 번 더 돌리자 `for i := range n` 으로 바뀌었습니다. 수정을 합치는 코드의 주석(`cmd/vendor/golang.org/x/tools/internal/analysis/driverutil/fix.go`, Go 1.26.2 소스 기준)에 따르면 깔끔하게 합쳐지지 않는 수정은 버려지고, 다시 돌리라는 안내가 나갑니다. `-diff` 가 빌 때까지 돌리는 이유입니다.

| 도구 | 잡은 것 | 고친 모양 |
|---|---|---|
| `gofmt` | 정렬이 어긋난 구조체 필드 | `gofmt -w` |
| `go vet` copylocks | 뮤텍스를 든 구조체의 값 리시버 | `func (s *Store)` |
| `go vet` lostcancel | 버린 `cancel` | `defer cancel()` |
| `go vet` printf | 문자열에 `%d` | `fmt.Fprintln(w, m)` |
| `go vet` unusedresult | 결과를 버린 `fmt.Sprintf` | 결과를 쓰거나 지운다 |
| `go fix` any | `interface{}` | `any` |
| `go fix` minmax | `if limit < n { n = limit }` | `min(limit, len(ids))` |
| `go fix` slicescontains | 포함 검사 루프 | `slices.Contains` |
| `go fix` stringsbuilder | 루프 안 `+=` | `strings.Builder` |
| `go fix` waitgroup | `Add(1)`·`go`·`Done` | `wg.Go` (Go 1.25) |
| `go fix` rangeint | `for i := 0; i < n; i++` | `for i := range n` (Go 1.22) |

copylocks 는 형식 문제가 아니라 버그입니다. 값 리시버는 구조체를 복사하므로 `Lock` 이 복사본의 뮤텍스를 잠그고, 원본은 보호받지 못합니다. 도구가 고친 결과도 다시 읽습니다. `s.WriteString(fmt.Sprintf(…))` 는 `fmt.Fprintf(&s, …)` 로 한 단계 더 줄일 자리입니다. 버전 조건은 `go` 줄을 바꿔 확인했습니다. `go 1.20` 에서 `min` 은 `built-in min requires go1.21 or later`, `go 1.21` 에서 정수 `range` 는 `requires go1.22 or later` 로 거절됐습니다.

## 도구가 지나친 11가지는 사람이 짚는다

빌드도 되고 vet 도 조용한 실수입니다. 고친 모양은 예제의 `inbox` 패키지에 있고 테스트를 통과합니다.

| 지적 | 나쁜 예 | 고친 예 |
|---|---|---|
| 에러 문자열 | `"Message not found."` | `"message not found"` |
| 에러 비교 | `err == ErrNotFound` | `errors.Is(err, ErrNotFound)` |
| 감싸기 | `fmt.Errorf("open failed: %v", err)` | `fmt.Errorf("open %s: %w", path, err)` |
| 기록과 반환 | `log.Printf(…)` 뒤 `return err` | 반환만, 기록은 맨 위 한 번 |
| 처리 안 된 에러 | NotFound 밖의 에러가 200 으로 나감 | `if err != nil` 로 500 |
| return 뒤 else | `return …` 다음 `} else {` | 일찍 반환하고 들여쓰기 없이 |
| 루프 안 `defer` | 반복마다 `defer f.Close()` | 한 번을 함수로 빼서 `defer` |
| 구조체 안 `context` | `ctx context.Context` 필드 | 함수의 첫 인자 |
| 멈출 길 없는 고루틴 | `for { time.Sleep(time.Minute) }` | `select` 로 `ctx.Done()` 확인 |
| 이름 | `GetMessageById`, `this`, `InboxStore`, `Ids` | `Message`, `s`, `inbox.Store`, `IDs` |
| 빈 목록 | `var ids []string` 그대로 JSON | `make([]string, 0, n)` |

이름 규칙은 짧게 정리합니다. 게터에 `Get` 을 붙이지 않고, `ID`·`URL` 같은 약어는 대소문자를 통째로 맞추며, 리시버는 한두 글자로 모든 메서드에서 같게 씁니다. 패키지 이름을 타입 이름에 되풀이하지 않습니다(`inbox.InboxStore` 대신 `inbox.Store`).

## 에러는 사슬로 읽히니 문자열 규칙이 생긴다

에러는 감싸여 한 줄로 이어 붙습니다. 대문자와 마침표가 문장 한가운데 끼고, `==` 와 `%v` 는 사슬을 놓칩니다.

```text
handle GET /m/7: load 7: Message not found.
err == ErrNotFound: false
errors.Is: true
%v 로 감싼 뒤 errors.Is: false
```

표준 라이브러리도 소문자로 씁니다. Go 1.26.2 소스(테스트·testdata·vendor 제외)에서 `errors.New("` 다음 첫 글자를 세니 소문자가 1,749건, 대문자가 77건이었습니다. 대문자로 시작한 것은 `Time.MarshalJSON` 같은 식별자나 `ECDSA`·`URL` 같은 약어가 많았습니다.

## 고루틴과 context 는 수명을 묻는다

멈출 방법이 없는 고루틴은 테스트도 끝낼 수 없습니다. 고친 주기 작업은 `ctx` 를 첫 인자로 받고, 끝나는 조건을 `select` 에 둡니다.

```go
func Report(ctx context.Context, s *Store, every time.Duration, report func(n int)) {
	t := time.NewTicker(every)
	defer t.Stop()
	for {
		select {
		case <-ctx.Done():
			return
		case <-t.C:
			report(len(s.IDs()))
		}
	}
}
```

테스트는 `ctx` 를 취소하고 1초 안에 `Report` 가 끝나는지 봅니다. `go test -race` 로 통과했습니다. `context` 를 구조체 필드에 두면 그 구조체를 만든 시점의 수명이 모든 호출에 묶입니다. `go doc context` 도 Context 를 구조체에 저장하지 말고 첫 인자로 넘기라고 적습니다.

## 빈 목록은 null 이 아니라 [] 로 보낸다

```text
{"ids":null} true
{"ids":[]} false
{"ids":null} true
```

`var` 로 선언만 한 슬라이스, `make([]string, 0)`, 빈 맵에 `slices.Sorted(maps.Keys(m))`(Go 1.23)을 차례로 JSON 으로 바꾼 결과이고, 뒤의 값은 `ids == nil` 입니다. 세 번째가 함정입니다. 표준 함수로 모은 결과도 비어 있으면 nil 이고, 클라이언트는 `ids.length` 에서 깨집니다. 고친 `IDs` 는 `make([]string, 0, len(s.msgs))` 로 시작하고, 테스트가 `{"ids":[]}` 를 고정합니다.

## 한 단계 더 — 받아들여지는 리뷰 의견과 테스트로 옮긴 규칙

받아들여지는 리뷰 의견은 세 부분으로 씁니다. 규칙 한 줄, 증거, 고친 모양입니다. "ctx 를 구조체에 두지 않습니다(`go doc context`). 지금 구조라면 첫 요청의 타임아웃이 이후 호출에도 남습니다. `Report(ctx, …)` 처럼 첫 인자로 받으면 됩니다." 증거는 실패하는 테스트, vet 출력, 벤치마크처럼 다시 돌릴 수 있는 것이어야 합니다. 취향은 의견으로 표시하고 막지 않습니다.

같은 지적을 두 번 했다면 테스트로 옮깁니다. 표준 라이브러리의 `go/parser`·`go/ast` 로 구조체 필드의 `context.Context` 를 찾는 검사를 40줄짜리 파일 하나로 만들었습니다. 핵심은 이 부분입니다.

```go
ast.Inspect(f, func(n ast.Node) bool {
	st, ok := n.(*ast.StructType)
	if !ok {
		return true
	}
	for _, field := range st.Fields.List {
		sel, ok := field.Type.(*ast.SelectorExpr)
		if !ok || sel.Sel.Name != "Context" {
			continue
		}
		if pkg, ok := sel.X.(*ast.Ident); ok && pkg.Name == "context" {
			p := fset.Position(field.Pos())
			found = append(found, fmt.Sprintf("%s:%d", filepath.Base(p.Filename), p.Line))
		}
	}
	return true
})
```

함수 매개변수도 `*ast.Field` 라서 `*ast.StructType` 안에서만 셉니다. 리뷰 전 파일에 돌리면 `inbox.go.txt:20: context.Context 를 구조체 필드에 담았다` 가 나오고 종료 코드는 1 입니다. 고친 패키지는 이 검사를 `TestNoContextInStructs` 로 `go test` 마다 돌립니다.

마지막으로 배포 전 빌드 단계에 도구를 고정합니다. 이 사이트의 Go 서버는 컨테이너 빌드에서 `test -z "$(gofmt -l .)" && go vet ./... && go test ./...` 를 통과해야 이미지를 만듭니다. 사람은 그 뒤에 남은 질문만 읽습니다.

## 면접에서 이렇게 묻는다

**Q1. 코드 리뷰에서 무엇부터 봅니까?**
실패 경로와 수명입니다. 에러가 어디서 감싸이고 어디서 한 번 기록되는지, 고루틴과 `context` 가 어떻게 끝나는지, 응답이 어떤 모양으로 나가는지 봅니다. 형식과 형식 문자열은 `gofmt`·`go vet` 이 CI 에서 먼저 막게 합니다.

**Q2. go vet 이 잡는 것과 못 잡는 것을 예로 들어 보세요.**
뮤텍스 복사(copylocks)와 버린 `cancel`(lostcancel)을 잡습니다. `Printf` 형식 불일치(printf)와 버린 `Sprintf` 결과(unusedresult)도 잡습니다. `err == ErrNotFound`, `%v` 로 끊긴 사슬, 구조체 안 `context`, 루프 안 `defer`, 멈추지 않는 고루틴은 이번 실행에서 잡지 못했습니다.

**Q3. 값 리시버와 포인터 리시버를 리뷰에서 어떻게 봅니까?**
뮤텍스를 담았거나 상태를 바꾸면 포인터 리시버여야 합니다. 값 리시버는 복사본을 잠가 동기화가 사라집니다. 한 타입의 리시버 이름과 종류를 섞지 않습니다.

**Q4. 같은 리뷰 지적이 반복되면 어떻게 합니까?**
규칙을 기계 검사로 옮깁니다. 배포판 분석기로 되면 CI 에 넣고, 아니면 `go/ast` 로 작은 테스트를 만듭니다. 사람의 기억에 맡긴 규칙은 다음 리팩터링에서 빠집니다.

## 연습

### 연습 1 — 루프 안 defer 가 닫는 순서 보기

파일 세 개를 열고 닫는 순서를 기록하는 가짜 `open` 을 만들어, 루프 안 `defer` 판과 한 번을 함수로 뺀 판의 순서를 비교합니다.

<details>
<summary>풀이</summary>

`Close` 가 호출 기록을 남기는 타입을 두고, 두 판을 같은 입력으로 돌립니다.

```go
func LoadBefore(paths []string, open func(string) (io.Closer, error)) error {
	for _, p := range paths {
		f, err := open(p)
		if err != nil {
			return err
		}
		defer f.Close()
	}
	return nil
}
```

```text
before: open a, open b, open c, close c, close b, close a
after:  open a, close a, open b, close b, open c, close c
```

루프 안 `defer` 는 함수가 끝날 때 거꾸로 닫습니다. 파일이 수백 개면 그만큼 열린 채 쌓입니다. `after` 는 반복 한 번을 `loadOne` 으로 빼고 그 안에서 `defer` 해서 바로 닫습니다. 두 순서는 테스트로 고정했습니다.

</details>

### 연습 2 — 에러 문자열 규칙을 검사로 만들기

`errors.New`·`fmt.Errorf` 의 첫 인자가 대문자로 시작하거나 마침표로 끝나면 위치를 찍는 검사를 `go/ast` 로 만들고, 리뷰 전 파일에 돌립니다.

<details>
<summary>풀이</summary>

호출식(`*ast.CallExpr`)의 함수가 `errors.New` 나 `fmt.Errorf` 이고 첫 인자가 문자열 리터럴이면 `strconv.Unquote` 로 풀어 첫 글자와 끝을 봅니다.

```go
first, _ := utf8.DecodeRuneInString(s)
if unicode.IsUpper(first) || strings.HasSuffix(s, ".") {
	p := fset.Position(lit.Pos())
	found = append(found, fmt.Sprintf("%s:%d: %q", filepath.Base(p.Filename), p.Line, s))
}
```

```text
$ errstring before/inbox.go.txt
inbox.go.txt:15: "Message not found."
$ errstring inbox/inbox.go
```

고친 파일에서는 아무것도 나오지 않습니다. `ECDSA` 처럼 약어로 시작하는 정당한 문자열도 걸리니, 실제로 쓰려면 허용 목록을 둡니다.

</details>

## 정리

리뷰어는 코드가 실패할 때와 끝날 때를 봅니다. 기계로 판정되는 11가지는 `gofmt`·`go vet`·`go fix` 에 맡겼고, 에러 사슬·이름·고루틴 수명·응답 모양은 사람이 짚었습니다. 두 번 짚은 규칙은 테스트로 옮겼습니다.

[로드맵의 23편](https://blog.doksam.com/blog/go-course-00-roadmap/)에서는 이 기준을 들고 시스템 설계 과제를 풉니다. 알림 발송 서비스를 요구사항에서 장애 대응까지 Go 로 설계합니다.
