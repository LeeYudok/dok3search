# Go 강의 12편 — 회귀는 테스트 개수가 아니라 테스트 종류로 막는다

> 테스트가 많은데도 버그가 새는 이유는 무엇이고, 어떤 테스트를 더해야 회귀가 막힙니까?

버그는 사람이 고른 입력 밖에서 샙니다. 이 편의 한 함수는 두 경우로 커버리지 100% 를 찍었지만 한글을 자르면 깨진 UTF-8 을 냈습니다. 그래서 테스트의 개수가 아니라 종류를 늘립니다. 기준 파일과 비교하는 골든(golden) 테스트, 기계가 입력을 만드는 퍼즈(fuzz) 테스트, 가짜 시계, 순서 섞기입니다.

> [그림] 배달 로봇이 상자를 컨베이어에 올리고, 상자가 골든 파일·퍼즈·가짜 시간 세 검사대를 지난다

그림의 세 검사대가 이 편에서 더하는 종류입니다. 표 주도 테스트와 `httptest` 같은 기본은 [5편](https://blog.doksam.com/blog/go-course-05-tests-and-concurrency/)에 있습니다. 출력은 2026-11-18 에 맥(Apple M4 Pro)과 Go 1.26.2 로 돌린 결과입니다.

## 시험 대상은 내부가 아니라 경계의 동작이다

테스트는 공개된 함수의 입력·출력·에러 갈래를 봅니다. 내부 필드를 보면 리팩터링마다 깨집니다. 보안 요건은 꼭 테스트로 고정합니다. 이 사이트의 Go 서버는 `/../../etc/passwd` 요청이 404 인지, 같은 IP 의 여섯 번째 문의가 429 인지 시험합니다.

## 병렬 서브테스트는 Go 1.22 부터 tc := tc 없이 안전하다

일부러 틀린 기대값(3 은 짝수)을 넣은 표입니다. 서브테스트마다 `t.Parallel()` 을 부릅니다.

```go
for _, tc := range cases { // {"짝수", 2, true}, {"홀수", 3, true}, {"영", 0, true}
	t.Run(tc.name, func(t *testing.T) {
		t.Parallel()
		t.Logf("검사하는 값: %d", tc.n)
		if got := isEven(tc.n); got != tc.want {
			t.Errorf("isEven(%d) = %v, 원하는 값 %v", tc.n, got, tc.want)
		}
	})
}
```

`go.mod` 의 `go` 줄을 1.21 로 두고 `go test -v` 를 돌렸습니다. 로그와 결과 줄만 추렸습니다.

```text
    even_test.go:20: 검사하는 값: 0
    even_test.go:20: 검사하는 값: 0
    even_test.go:20: 검사하는 값: 0
--- PASS: TestIsEven (0.00s)
```

세 서브테스트가 모두 마지막 값 0 만 검사하고 통과했습니다. 병렬 서브테스트는 루프가 끝난 뒤 돌고, 1.21 까지 `tc` 는 루프 전체에 하나였기 때문입니다(예전 코드는 `tc := tc` 로 막았습니다). `go` 줄을 1.22 로 바꾸면 틀린 경우가 드러납니다.

```text
    even_test.go:22: isEven(3) = false, 원하는 값 true
--- FAIL: TestIsEven (0.00s)
```

1.21 모드의 `go vet` 은 `loop variable tc captured by func literal` 을 다섯 번 냈습니다. 하지만 `go test` 가 먼저 돌리는 vet 검사 열 가지에는 이 검사(loopclosure)가 없습니다.

서브테스트 이름의 공백은 밑줄이 됩니다. 하나만 돌릴 때는 `go test -run 'TestParseLine/빈_값'` 처럼 `/` 로 잇습니다.

## 순서에 기대는 테스트는 -shuffle 로 드러난다

패키지 변수 `Prefix` 를 바꾸고 되돌리지 않는 테스트가 있습니다. 기본 순서로는 늘 통과합니다. `-shuffle=on` 으로 여섯 번 돌리자 두 번 실패했습니다.

```text
$ go test -count=1 -shuffle=on .
-test.shuffle 1794933191288594000
--- FAIL: TestHello (0.00s)
    greet_test.go:7: Hello = "반갑습니다, 민수", 원하는 값 "안녕하세요, 민수"
FAIL
```

첫 줄의 씨앗 값을 `-shuffle=1794933191288594000` 으로 주면 같은 순서로 다시 실패합니다. 바꾸기 전에 `old := Prefix` 로 받아 두고 `t.Cleanup(func() { Prefix = old })` 로 되돌리게 고치자 `-shuffle=on` 열 번이 모두 통과했습니다. 더 나은 방법은 전역 상태를 없애고 값을 인자로 받는 것입니다.

`go test` 는 통과한 결과를 캐시해 다시 돌리면 `ok … (cached)` 를 찍고, `-count=1` 을 주면 다시 돕니다. `go help test` 에 따르면 캐시는 테스트 바이너리와 플래그 말고는 테스트가 연 모듈 안 파일과 읽은 환경변수만 봅니다. 모듈 밖의 DB 는 보지 않으니 통합 테스트는 `-count=1` 로 돌립니다.

## 출력 전체는 골든 파일과 비교한다

지수 요약을 표 문자열로 만드는 `Render` 가 있습니다. 필드마다 `if` 를 쓰는 대신 출력 전체를 `testdata/*.golden` 과 비교합니다.

```go
var update = flag.Bool("update", false, "testdata/*.golden 을 지금 출력으로 다시 쓴다")

func TestRenderGolden(t *testing.T) {
	got := Render(loadSummary(t, "testdata/2026-11-13.json"))
	golden := "testdata/2026-11-13.golden"
	if *update {
		writeFile(t, golden, got)
	}
	if want := readFile(t, golden); got != want {
		t.Errorf("%s 와 다르다 (의도한 변경이면 go test -update)\n%s", golden, lineDiff(want, got))
	}
}
```

`loadSummary`·`readFile`·`writeFile` 은 `t.Helper()` 를 부르고 실패하면 `t.Fatal` 하는 작은 함수이고, `lineDiff` 는 다른 줄만 보여 줍니다. `go test ./report -update` 로 처음 파일을 만들고, 등락률 형식을 `%+.2f` 에서 `%+.1f` 로 바꾸자 이렇게 실패했습니다.

```text
    report_test.go:21: testdata/2026-11-13.golden 와 다르다 (의도한 변경이면 go test -update)
        2행
        - ALPHA    5290.53   -52.20 (-0.98%)
        + ALPHA    5290.53   -52.20 (-1.0%)
        …
```

제 데이터 수집 도구도 받아 둔 원자료로 계산한 값을 블로그에 올린 값과 비교합니다. 위험은 `-update` 를 습관처럼 돌리는 것이라, 골든 파일의 diff 도 코드처럼 리뷰합니다.

## 퍼즈 테스트는 사람이 고르지 않은 입력을 만든다

설정 파일의 `key = value` 한 줄을 읽는 `ParseLine` 에 버그를 남겼습니다. 값의 따옴표를 벗길 때 `if value[0] == '"' { value = value[1 : len(value)-1] }` 로 길이를 확인하지 않았습니다.

퍼즈 테스트는 `testing.F`(Go 1.18)로 씁니다. 씨앗 입력 두 개와 "패닉 없이, 키는 비지 않고 앞뒤 공백이 없다" 는 성질만 적었습니다.

```go
func FuzzParseLine(f *testing.F) {
	f.Add("name = inbox")
	f.Add(`title = "새 문의"`)
	f.Fuzz(func(t *testing.T, line string) {
		k, _, err := ParseLine(line)
		if err != nil {
			return
		}
		if k == "" || k != strings.TrimSpace(k) {
			t.Errorf("ParseLine(%q) 키 = %q", line, k)
		}
	})
}
```

```text
$ go test -fuzz=FuzzParseLine -fuzztime=60s .
fuzz: elapsed: 0s, gathering baseline coverage: 0/2 completed
fuzz: elapsed: 0s, gathering baseline coverage: 2/2 completed, now fuzzing with 14 workers
fuzz: elapsed: 0s, execs: 139 (6421/sec), new interesting: 1 (total: 3)
--- FAIL: FuzzParseLine (0.02s)
    --- FAIL: FuzzParseLine (0.00s)
        testing.go:1927: panic: runtime error: index out of range [0] with length 0
…
    Failing input written to testdata/fuzz/FuzzParseLine/f8ad2e20bf1778fa
```

139번 실행, 0.02초 만에 찾았습니다(스택 추적은 줄였습니다). 저장된 입력은 `string("00=")`, 값이 빈 줄입니다. 이 파일은 씨앗 입력이 되어 `-fuzz` 없는 `go test` 도 실패시킵니다. 길이를 확인하게(`len(value) >= 2 && …`) 고치면 통과하고, 퍼저가 찾은 입력이 그대로 회귀 테스트가 됩니다.

## 시간에 기대는 코드는 synctest 의 가짜 시계로 시험한다

한 시간에 다섯 번만 허용하는 레이트 리미터를 시험하려면 한 시간을 기다려야 합니다. 5편처럼 시계를 주입해도 되지만, `time.Now` 를 그대로 쓰는 코드는 `testing/synctest`(Go 1.25)로 감쌉니다.

```go
synctest.Test(t, func(t *testing.T) {
	l := New(5, time.Hour)
	// 다섯 번 허용, 여섯 번째 거절을 확인한다(생략)
	start := time.Now()
	time.Sleep(61 * time.Minute)
	if !l.Allow("1.1.1.1") {
		t.Fatal("한 시간이 지나도 거절한다")
	}
	t.Logf("가짜 시계로 흐른 시간: %v", time.Since(start))
})
```

```text
    limiter_test.go:25: 가짜 시계로 흐른 시간: 1h1m0s
--- PASS: TestAllowWindow (0.00s)
```

61분이 0.00초에 지났습니다. `go doc testing/synctest` 에 따르면 이 거품(bubble) 안의 `time` 은 가짜 시계를 씁니다. 시간은 거품 안의 모든 고루틴이 막혔을 때만 흐릅니다.

## 테스트 더블은 가짜를 먼저 쓰고, 인터페이스는 쓰는 쪽에 둔다

| 종류 | 무엇인가 | 언제 쓰나 |
|---|---|---|
| 가짜(fake) | 진짜처럼 동작하는 단순 구현. 메모리 저장소 | 정상 흐름과 규칙을 시험할 때 |
| 스텁(stub) | 정해 둔 값이나 실패만 돌려준다 | 에러 갈래를 시험할 때 |
| 목(mock) | 어떤 호출이 몇 번 왔는지 기록하고 검증한다 | 호출 자체가 요건일 때만. 구현을 묶어 리팩터링을 막는다 |

인터페이스 `Store` 는 저장소 패키지가 아니라 그것을 쓰는 `inbox` 패키지에 둡니다. 가짜와 진짜가 같은 약속을 지키는지는 계약 테스트 함수 하나로 봅니다.

```go
func testStoreContract(t *testing.T, s Store) {
	t.Helper()
	if err := s.Save(t.Context(), Message{ReplyTo: "a@example.com"}); err != nil {
		t.Fatal(err)
	}
	if n, err := s.CountByReplyTo(t.Context(), "a@example.com"); err != nil || n != 1 {
		t.Errorf("CountByReplyTo = %d, %v, 원하는 값 1, nil", n, err)
	}
}
```

진짜 DB 구현은 환경변수가 있을 때만 같은 함수로 시험하고, 없으면 `--- SKIP: TestPostgresStoreContract` 가 찍힙니다. `t.Context()`(Go 1.24)는 정리 함수 직전에 취소됩니다. `TestMain` 은 `go doc testing` 이 보통은 필요 없다고 적는 저수준 도구입니다. 남의 라이브러리는 목으로 흉내 내지 말고 내 인터페이스 뒤에 감춥니다.

## 커버리지 100% 는 버그가 없다는 뜻이 아니다

`Truncate(s, n)` 은 `len(s) <= n` 이면 `s` 를, 아니면 `s[:n] + "…"` 를 돌려줍니다. "짧은 글"(`"hi"`, 5)과 "긴 글"(`"hello world"`, 5) 두 경우로 시험했습니다.

```text
$ go test -coverprofile=cover.out . && go tool cover -func=cover.out
ok  	example.com/text	0.249s	coverage: 100.0% of statements
example.com/text/text.go:5:	Truncate	100.0%
total:				(statements)	100.0%
```

그런데 `Truncate("안녕하세요", 4)` 는 `"안\xeb…"` 를 돌려줬고, `utf8.ValidString` 은 `false` 였습니다. 한글 한 글자는 3바이트라 4바이트에서 자르면 둘째 글자가 깨집니다. 커버리지는 어느 줄이 실행됐는지만 셉니다. 어떤 값으로 실행됐는지는 세지 않습니다. 이 사이트의 Go 서버가 입력 길이를 `utf8.RuneCountInString` 으로 재는 이유도 같습니다.

## 한 단계 더 — 퍼즈 엔진과 테스트 바이너리

**퍼즈 엔진은 커버리지를 보고 입력을 고릅니다.** `go doc testing` 에 따르면 `-fuzz` 는 테스트를 커버리지 계측과 함께 컴파일하고, 씨앗 입력을 바꿔 가며 새 경로를 연 입력(`new interesting`)을 빌드 캐시에 모읍니다. 실패한 입력만 `testdata/fuzz/<이름>` 에 씁니다. 고친 `ParseLine` 을 10초 돌리자 9,082,680번 실행에 `new interesting: 206` 이었습니다(다른 작업과 함께 돈 값). 실패 입력은 줄이기도 해서, 연습 1에서는 38바이트 입력이 한 글자로 줄었습니다.

**테스트도 실행 파일입니다.** `go test -c` 는 테스트 바이너리(이번에는 5,395,330바이트)를 만들고, 이 파일은 `-test.run`·`-test.v` 처럼 `-test.` 접두사로 옵션을 받습니다. 다른 기기에서 돌리거나 `-exec` 로 지정한 프로그램을 거칠 때 씁니다.

**벤치마크는 `b.Loop`(Go 1.24)로 씁니다.** `for b.Loop()` 안에서 `ParseLine` 을 부르고 `-benchmem` 으로 돌렸습니다(21편에서 자세히).

```text
BenchmarkParseLine-14    	173174409	         6.857 ns/op	       0 B/op	       0 allocs/op
```

세 번의 가운데 값입니다(6.835~6.909 ns/op). 부분 문자열을 잘라 쓰기만 해서 힙 할당이 0번입니다.

## 면접에서 이렇게 묻는다

**Q1. 병렬 서브테스트에서 예전에는 왜 `tc := tc` 를 썼습니까?**
Go 1.21 까지 `range` 변수는 루프 전체에 하나여서, 루프가 끝난 뒤 도는 병렬 서브테스트가 모두 마지막 값을 봤습니다. 실험에서는 틀린 기대값이 있어도 셋 다 통과했습니다. `go` 줄이 1.22 이상이면 반복마다 새 변수라 필요 없다고 답합니다.

**Q2. 골든 파일 테스트의 장점과 위험은 무엇입니까?**
출력 전체를 고정해 형식 변화를 놓치지 않고 기대값을 손으로 쓰지 않습니다. 위험은 `-update` 로 틀린 출력을 정답으로 굳히는 것이라, 골든 diff 도 리뷰한다고 답합니다.

**Q3. 퍼즈 테스트는 표 주도 테스트와 무엇이 다릅니까?**
표는 사람이 고른 입력이고, 퍼즈는 엔진이 커버리지를 보며 입력을 만듭니다. 그래서 기대값 대신 성질(패닉 없음, 출력이 올바른 UTF-8)을 적습니다. 찾은 입력은 `testdata/fuzz` 에 남아 회귀 테스트가 됩니다.

**Q4. 시간에 기대는 코드는 어떻게 시험합니까?**
시계를 함수로 주입하거나 Go 1.25 의 `testing/synctest` 를 씁니다. 재시도 함수의 테스트가 실제 시계로 20초, synctest 안에서 0초였다고 수치로 말합니다.

## 연습

### 연습 1 — Truncate 의 버그를 퍼저로 찾기

위 `Truncate` 에 퍼즈 테스트를 붙여 버그를 찾고 고칩니다. 성질은 "올바른 UTF-8 을 넣으면 올바른 UTF-8 이 나온다" 입니다.

<details>
<summary>풀이</summary>

```go
func FuzzTruncate(f *testing.F) {
	f.Add("hello world", 5)
	f.Fuzz(func(t *testing.T, s string, n int) {
		if !utf8.ValidString(s) || n < 0 {
			return
		}
		if out := Truncate(s, n); !utf8.ValidString(out) {
			t.Errorf("Truncate(%q, %d) = %q, 올바른 UTF-8 이 아니다", s, n, out)
		}
	})
}
```

```text
fuzz: minimizing 38-byte failing input file
--- FAIL: FuzzTruncate (1.06s)
    --- FAIL: FuzzTruncate (0.00s)
        text_test.go:33: Truncate("ɞ", 1) = "\xc9…", 올바른 UTF-8 이 아니다
    Failing input written to testdata/fuzz/FuzzTruncate/1411d4bc6d9bafcb
```

`fuzz: elapsed` 진행 줄은 뺐습니다. 2바이트 글자를 1바이트에서 잘랐습니다. 자를 위치 `cut` 을 `n` 에서 시작해 `utf8.RuneStart(s[cut])` 가 참일 때까지 당긴 뒤 `s[:cut] + "…"` 를 돌려주게 고칩니다.

고친 뒤 저장된 입력은 `FuzzTruncate/1411d4bc6d9bafcb` 서브테스트로 통과했고, 10초 퍼징에도 실패가 없었습니다. `Truncate("안녕하세요", 4)` 는 `"안…"` 입니다.

</details>

### 연습 2 — 재시도 함수를 기다리지 않고 시험하기

`Do(ctx, attempts, wait, f)` 는 `f` 가 실패하면 `wait` 만큼 쉬고 다시 부릅니다. 세 번 모두 실패하면 정확히 20초(10초 × 2)가 걸리는지, context 마감이 15초면 `context.DeadlineExceeded` 로 멈추는지 시험합니다.

<details>
<summary>풀이</summary>

```go
synctest.Test(t, func(t *testing.T) {
	calls := 0
	start := time.Now()
	err := Do(t.Context(), 3, 10*time.Second, func() error { calls++; return errDown })
	if !errors.Is(err, errDown) || calls != 3 {
		t.Fatalf("Do = %v, 호출 %d번, 원하는 값 errDown, 3번", err, calls)
	}
	if got := time.Since(start); got != 20*time.Second {
		t.Errorf("걸린 시간 = %v, 원하는 값 20s", got)
	}
})
```

마감 테스트는 `context.WithTimeout(t.Context(), 15*time.Second)` 를 넘기고 `errors.Is(err, context.DeadlineExceeded)` 를 봅니다. 둘 다 `(0.00s)` 로 통과했고, `synctest.Test` 를 빼면 `(20.00s)` 가 걸렸습니다. 가짜 시계에서는 `time.Since` 가 정확히 `20s` 라 `==` 로 비교합니다.

</details>

## 정리

테스트가 많아도 종류가 같으면 같은 곳에서 샙니다. 출력은 골든 파일로, 입력은 퍼저로, 시간은 가짜 시계로, 순서는 `-shuffle` 로 넓히면 사람이 고르지 않은 경우까지 회귀 테스트가 됩니다. 다음에 테스트를 더할 때는 커버리지 숫자보다 시험한 입력의 종류를 먼저 봅니다.

13편부터는 3단계 심화입니다. 첫 편은 슬라이스·맵·문자열의 내부 구조입니다. 이 편의 `Truncate` 가 왜 바이트 단위로 잘렸는지도 문자열의 내부 표현에서 다시 봅니다.
