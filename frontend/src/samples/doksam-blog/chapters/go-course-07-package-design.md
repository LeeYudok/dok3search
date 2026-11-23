# Go 강의 7편 — 패키지는 import 가 아래로만 흐르게 나눈다

> 파일이 늘어나면 패키지를 어떻게 나누고, 무엇이 무엇을 import 해도 됩니까?

import 가 한 방향으로만 흐르게 나눕니다. 맨 아래에 HTTP 도 저장소도 모르는 도메인 패키지를, 그 위에 저장소와 HTTP 패키지를, 맨 위 `cmd` 에 이들을 잇는 `main` 을 둡니다. 저장소가 필요한 쪽은 구현 대신 작은 인터페이스를 자기 패키지에 둡니다. Go 는 순환 import 를 컴파일 에러로 막으니 이 방향은 빌드 조건입니다. 1단계의 문의 서버(inbox)를 네 패키지로 나눠 봅니다.

> [그림] 창고 선반 세 칸에 cmd/inbox, httpapi 와 store, inbox 상자가 놓여 있고 화살표는 아래로만 내려간다. 아래에서 위로 올라가는 빨간 점선에는 X 표와 순환 금지가 붙어 있다

선반의 위 칸이 아래 칸을 import 합니다. 새 패키지를 들고 온 로봇은 그 패키지가 무엇을 import 할지 보고 칸을 고릅니다.

## 한 패키지로 버틸 수 있는 동안은 파일로만 나눈다

나누면 공개 이름과 경계의 인터페이스를 설계해야 합니다. 그래서 제 코드도 대부분 한 패키지입니다. 이 사이트의 Go 서버는 테스트를 빼고 481줄인데 `main` 패키지 하나에 파일 셋(시작과 헬스, 문의 접수, 정적 파일)입니다. 한국어 글 검사기는 1,476줄이 `main` 패키지 하나이고, 파일을 검사 엔진·규칙 표·표기 프로필·본문 고르기·문단 구조·원문 대조로 나눴습니다.

| 기준 | 한 패키지, 파일로 나눔 | 여러 패키지로 나눔 |
|---|---|---|
| 경계 | 없다. 소문자 이름도 모두 보인다 | 컴파일러가 강제한다(대문자 공개, `internal`, 순환 금지) |
| 이름 | `New`, `validate` 그대로 | `inbox.New` 처럼 패키지 이름이 붙는다 |
| 테스트 | 한 곳에서 다 본다 | 경계마다 가짜 구현을 끼운다 |

나누는 신호는 셋입니다. 실행 파일 둘 이상이 같은 코드를 쓸 때, 바꿔 끼울 경계(저장소, 외부 API)가 생길 때, 한쪽이 다른 쪽을 몰라야 할 때입니다. 제 데이터 수집 도구는 명령줄 처리(`cmd/market`)와 수집·계산(`internal/market`)을 나눴습니다. 계산을 테스트할 때 플래그 처리가 끼지 않습니다.

## 네 패키지로 나누고 import 는 위에서 아래로만 흐르게 한다

```text
inbox/
├── cmd/inbox/main.go     # 부품을 잇기만 한다
└── internal/
    ├── inbox/            # 도메인: 우리 패키지를 import 하지 않는다
    ├── store/            # 저장소 구현: inbox 를 import 한다
    └── httpapi/          # HTTP: 저장소는 인터페이스로 받는다
```

말로 정한 방향이 지켜지는지는 `go list` 가 보여 줍니다.

```text
$ go list -f '{{.ImportPath}}: {{join .Imports " "}}' ./...
example.com/inbox/cmd/inbox: example.com/inbox/internal/httpapi example.com/inbox/internal/store log/slog net/http os time
example.com/inbox/internal/httpapi: context encoding/json errors example.com/inbox/internal/inbox net/http time
example.com/inbox/internal/inbox: context errors fmt strings time unicode/utf8
example.com/inbox/internal/store: context example.com/inbox/internal/inbox strconv sync
```

`internal/inbox` 줄에는 표준 라이브러리만 있습니다. 도메인은 `Message`, `New`, 에러 값만 압니다. `main` 은 `httpapi.Handler(store.NewMemory(), time.Now)` 한 줄로 부품을 잇고 서버를 띄웁니다. 메모리 저장소를 다른 저장소로 바꾸는 곳도 이 한 줄입니다.

## 저장소 인터페이스는 쓰는 쪽에 둔다

`httpapi` 는 `store` 를 import 하지 않습니다. 저장소에 바라는 두 메서드를 자기 패키지에 적습니다.

```go
// Store 는 이 패키지가 저장소에 바라는 것만 적는다. 구현은 store 패키지에 있다.
type Store interface {
	Save(ctx context.Context, m inbox.Message) (string, error)
	Get(ctx context.Context, id string) (inbox.Message, error)
}
```

Java 는 구현 옆에 인터페이스를 두고 `implements` 로 선언합니다. Go 는 메서드만 맞으면 만족하므로(암묵 구현) 쓰는 쪽이 필요한 만큼만 정합니다. 만족 여부는 값을 넘기는 곳에서 컴파일러가 봅니다. `Memory` 에서 `Get` 을 지우면 `main` 에서 멈춥니다.

```text
cmd/inbox/main.go:21:38: cannot use store.NewMemory() (value of type *store.Memory) as httpapi.Store value in argument to httpapi.Handler: *store.Memory does not implement httpapi.Store (missing method Get)
```

테스트는 두 메서드만 맞춘 가짜 저장소(`fakeStore`)를 끼웁니다. 저장소가 실패할 때 503 을 주는지 DB 없이 확인합니다.

## 순환 import 는 공유할 것을 내리거나 인터페이스로 끊는다

흔한 실수를 일부러 만들었습니다. `ErrNotFound` 를 `store` 에 두고, 도메인에 `*store.Memory` 를 받는 `Lookup` 을 넣었습니다. `store` 는 이미 `inbox.Message` 때문에 `inbox` 를 import 합니다.

```text
$ go build ./internal/inbox
package example.com/inbox/internal/inbox
	imports example.com/inbox/internal/store from lookup.go
	imports example.com/inbox/internal/inbox from memory.go: import cycle not allowed
```

언어 명세는 패키지가 자기 자신을 직접이든 간접이든 import 하는 것을 금지합니다. 두 가지로 풉니다.

1. **공유하는 값을 아래로 내립니다.** `ErrNotFound` 를 `inbox` 로 옮기고, `store` 는 `inbox.ErrNotFound` 를 돌려줍니다.
2. **구체 타입 대신 쓰는 쪽 인터페이스를 받습니다.** `Lookup` 은 `Get` 하나만 있으면 됩니다.

```go
// Getter 는 Lookup 이 저장소에 바라는 것만 적는다.
type Getter interface {
	Get(ctx context.Context, id string) (Message, error)
}

// Lookup 은 id 의 메시지를 찾는다. 실패하면 id 를 붙여 감싼다.
func Lookup(ctx context.Context, g Getter, id string) (Message, error) {
	m, err := g.Get(ctx, id)
	if err != nil {
		return Message{}, fmt.Errorf("문의 %s 조회: %w", id, err)
	}
	return m, nil
}
```

`%w` 로 감쌌으니 호출부는 `errors.Is(err, inbox.ErrNotFound)` 로 분기합니다.

## internal 은 겹쳐서 범위를 더 좁힌다

`go help gopath` 에 따르면 `internal` 아래 코드는 `internal` 의 부모를 뿌리로 하는 트리 안에서만 import 됩니다. 그래서 겹쳐 두면 범위가 더 좁아집니다. `store` 만 쓰는 ID 생성기를 `internal/store/internal/idgen` 에 두니 `store` 에서는 빌드되고 `httpapi` 에서는 거절됐습니다.

```text
$ go build ./internal/httpapi
package example.com/inbox/internal/httpapi
	internal/httpapi/httpapi.go:12:2: use of internal package example.com/inbox/internal/store/internal/idgen not allowed
```

## 패키지 이름은 짧은 명사 하나로 짓고 겹쳐 쓰지 않는다

호출부에서는 패키지 이름이 늘 앞에 붙습니다. `util`·`common` 은 무엇이든 들어가서 결국 모든 패키지가 import 하는 덩어리가 됩니다. 이런 이름을 넣고 `go vet ./...` 을 돌려도 종료 코드 0 이었습니다. 도구가 아니라 리뷰에서 잡는 규칙입니다.

| 피할 이름 | 바꾼 이름 | 이유 |
|---|---|---|
| `inbox.InboxService` | `inbox.Service` | 패키지 이름을 되풀이한다 |
| `util.Trim` | 쓰는 패키지 안의 소문자 함수 | 이름이 내용을 말하지 않는다 |

문서 주석은 이름으로 시작합니다. `go doc` 이 그 첫 줄을 그대로 보여 줍니다(가운데 변수 줄은 줄였습니다).

```text
$ go doc ./internal/inbox
package inbox // import "example.com/inbox/internal/inbox"

Package inbox 는 문의 메시지와 그 검증 규칙을 담는다. HTTP 와 저장소를 모른다.

const MaxBody = 5000
…
    func Lookup(ctx context.Context, g Getter, id string) (Message, error)
    func New(replyTo, body string, now time.Time) (Message, error)
```

## 생성자는 설정 구조체로 시작하고 옵션 함수는 필요할 때 쓴다

이 사이트의 Go 서버가 쓰는 `http.Server{Addr: …, ReadHeaderTimeout: …}` 가 설정 구조체 방식입니다. 같은 레이트 리미터(호출 횟수 제한기)를 두 방식으로 만들었습니다.

```go
// 설정 구조체: 제로 값 필드는 기본값으로
l1 := limiter.NewFromConfig(limiter.Config{Limit: 10})

// 옵션 함수(functional options): 기본값 위에 옵션을 차례로
l2 := limiter.New(limiter.WithLimit(10), limiter.WithWindow(time.Minute))
```

차이는 0 에서 드러납니다. 설정 구조체는 `Limit: 0` 을 "안 적음"과 구별하지 못해 기본값 5 로 바꿉니다. 옵션 함수의 `WithLimit(0)` 은 0 을 그대로 씁니다. 표 주도 테스트로 두 경우를 고정했습니다.

| 기준 | 설정 구조체 | 옵션 함수 |
|---|---|---|
| 기본값 | 제로 값을 기본값으로 읽는다. 0 을 일부러 줄 수 없다 | 생성자가 기본값을 먼저 채운다. 0 도 줄 수 있다 |
| 설정 하나 추가 | 필드 하나 | 함수 하나와 필드 하나 |
| 읽기 | 리터럴 하나에 다 보인다 | 호출부에 `With…` 가 이어진다 |
| 잘못된 값 | 생성자 반환값에 `error` 를 더해 검사한다 | 옵션 타입을 `func(*Limiter) error` 로 바꿔 거절한다 |

설정 구조체로 시작합니다. 0 이 뜻 있는 값이거나, 공개 API 에 선택 설정이 계속 늘 때 옵션 함수로 옮깁니다.

## 밖으로 내보내는 값은 따로 만든 응답 타입으로 줄인다

도메인의 `Message` 에는 연락처가 있습니다. 응답에는 ID·본문·접수 시각만 담은 `messageResponse` 를 따로 두어, 도메인에 필드가 늘어도 새지 않게 합니다.

서버를 띄워 저장한 뒤 조회하면 `{"id":"1","body":"견적 문의드립니다","received":"2026-11-18 01:31:03"}` 가 나옵니다. 연락처가 없다는 것은 테스트 `TestSaveThenGetHidesReplyTo` 가 고정합니다.

## 테스트 패키지는 안쪽과 바깥쪽 두 가지다

같은 폴더의 테스트는 `package inbox`(안쪽, 소문자 이름까지 본다)나 `package inbox_test`(바깥쪽, 공개 API 만 본다)를 고릅니다. 바깥쪽은 순환도 피합니다. `Lookup` 을 진짜 메모리 저장소로 시험하려면 `store` 를 import 해야 하는데, `store` 는 `inbox` 를 import 합니다. 안쪽 테스트로 쓰면 멈춥니다.

```text
$ go test ./internal/inbox
# example.com/inbox/internal/inbox
package example.com/inbox/internal/inbox
	imports example.com/inbox/internal/store from lookup_internal_test.go
	imports example.com/inbox/internal/inbox from memory.go: import cycle not allowed in test
FAIL	example.com/inbox/internal/inbox [setup failed]
```

`package inbox_test` 로 바꾸면 통과합니다. 바깥쪽 테스트가 내부 함수 하나를 꼭 불러야 하면 `export_test.go` 를 씁니다.

```go
package inbox

// TooLong 은 외부 테스트 패키지(inbox_test)가 내부 함수 tooLong 을 부르게 열어 둔다.
// _test.go 파일이라 테스트 빌드에만 들어간다.
var TooLong = tooLong
```

`go doc ./internal/inbox TooLong` 은 `no symbol TooLong` 으로 답합니다. 일반 빌드에는 이 이름이 없습니다.

## 한 단계 더 — 아래 패키지를 바꾸면 위가 다시 컴파일된다

패키지 경계는 빌드 단위이기도 합니다. 빈 캐시로 한 번 빌드한 뒤 한 곳씩 고쳤습니다. 그때마다 `go build -x` 에서 컴파일된 패키지를 셌습니다(2026-11-18, Apple M4 Pro, Go 1.26.2). 바꾼 것이 없으면 링크만 돕니다.

| 바꾼 곳 | 다시 컴파일된 패키지 |
|---|---|
| `store` 에 메서드 추가 | store, main |
| `inbox` 에 메서드 추가 | inbox, httpapi, store, main |
| `inbox` 의 `Lookup` 에러 문구만 | inbox, httpapi, store |

아래에 있을수록 다시 컴파일되는 범위가 넓습니다. 마지막 줄에서 `main` 이 빠진 까닭은 `go` 명령 소스(`cmd/go/internal/work/buildid.go`) 주석에 있습니다. 작업 식별값(action ID)에는 입력 파일 전부의 내용 해시(content ID)가 들어갑니다. `inbox` 결과물이 바뀌어 바로 위 둘은 다시 컴파일됐지만, 그 둘의 결과물이 같아 `main` 의 입력은 그대로였습니다.

도메인이 가벼우면 테스트 빌드도 작습니다. `go list -deps` 로 센 패키지 수(표준 라이브러리 포함)는 `internal/inbox` 62개, `internal/httpapi` 187개였습니다. `net/http` 혼자 183개에 기댑니다.

## 면접에서 이렇게 묻는다

**Q1. 순환 import 는 어떻게 풉니까?**
명세가 금지하고 빌드가 `import cycle not allowed` 로 멈춥니다. 양쪽이 공유하는 타입·에러를 아래 패키지로 내리거나, 구체 타입 대신 쓰는 쪽 패키지에 정의한 작은 인터페이스를 받게 바꿉니다.

**Q2. 인터페이스는 어디에 정의합니까?**
쓰는 쪽입니다. 암묵 구현이라 구현 패키지는 인터페이스를 몰라도 됩니다. 쓰는 쪽은 필요한 메서드만 적어 가짜 구현이 쉽고, 만족 여부는 값을 넘기는 곳에서 컴파일러가 봅니다.

**Q3. 함수형 옵션 패턴과 설정 구조체 중 무엇을 씁니까?**
기본은 설정 구조체이고 `http.Server` 도 이 방식입니다. 제로 값을 기본값으로 읽어 0 을 일부러 줄 수 없습니다. 그 구별이 필요하거나 선택 설정이 계속 늘 때 옵션 함수를 씁니다.

**Q4. `package foo` 와 `package foo_test` 테스트는 무엇이 다릅니까?**
안쪽은 소문자 이름까지 보는 화이트박스, 바깥쪽은 공개 API 만 보는 블랙박스 테스트입니다. 바깥쪽에서는 `foo` 를 import 하는 패키지를 써도 순환이 없습니다. 내부 함수는 `export_test.go` 로 테스트 빌드에만 엽니다.

## 연습

### 연습 1 — 최근 메시지 n 개를 도메인에 더하기

`inbox` 에 최근 메시지 n 개를 돌려주는 `Recent` 를 더합니다. `Recent(ctx, s *store.Memory, n)` 처럼 쓰면 무엇이 일어나는지 보고, 순환 없이 고칩니다.

<details>
<summary>풀이</summary>

구체 타입을 받으면 본문과 같은 `import cycle not allowed` 가 납니다(`recent.go` → `store` → `inbox`). 쓰는 쪽(`inbox`)에 `List` 하나만 적은 인터페이스를 두고, `store.Memory` 에 `List` 를 더합니다.

```go
// Lister 는 Recent 가 저장소에 바라는 것만 적는다.
type Lister interface {
	List(ctx context.Context) ([]Message, error)
}

// Recent 는 최근 메시지 n 개를 오래된 순으로 돌려준다.
func Recent(ctx context.Context, l Lister, n int) ([]Message, error)
```

본문은 `List` 결과의 뒤쪽 n 개를 자르고, 에러는 `%w` 로 감쌉니다. 시험은 `package inbox_test` 에 두고 진짜 `store.NewMemory()` 에 세 건을 넣어 마지막 두 건이 나오는지 봅니다. `go test ./...` 가 통과했습니다.

</details>

### 연습 2 — 의존 방향을 테스트로 고정하기

도메인 패키지가 `net/http` 나 우리 다른 패키지를 import 하면 실패하는 테스트를 씁니다.

<details>
<summary>풀이</summary>

테스트 안에서 `go list -deps` 를 돌립니다. `go help test` 에 따르면 테스트는 그 패키지의 소스 폴더에서 돌기 때문에 `.` 이 도메인 패키지입니다.

```go
func TestDomainStaysAtTheBottom(t *testing.T) {
	out, err := exec.Command("go", "list", "-deps", ".").Output()
	if err != nil {
		t.Skipf("go list 를 돌리지 못했다: %v", err)
	}
	for _, p := range strings.Fields(string(out)) {
		if p == "net/http" || (strings.HasPrefix(p, "example.com/inbox/") && p != "example.com/inbox/internal/inbox") {
			t.Errorf("도메인 패키지가 %s 에 기댄다", p)
		}
	}
}
```

도메인에 HTTP 상태 코드를 고르는 함수를 일부러 넣자 이렇게 실패했습니다.

```text
--- FAIL: TestDomainStaysAtTheBottom (0.03s)
    deps_test.go:17: 도메인 패키지가 net/http 에 기댄다
```

말로 지키던 규칙이 `go test` 에서 걸리니, 에이전트가 고친 코드도 같은 검사를 지납니다.

</details>

## 정리

패키지는 import 가 아래로만 흐르게 나눕니다. 도메인을 맨 아래에 두고, 저장소가 필요하면 쓰는 쪽에 작은 인터페이스를 둡니다. 순환은 공유하는 것을 내리거나 인터페이스로 끊습니다. 그 전에 한 패키지에서 파일로 버틸 수 있는지부터 봅니다.

8편에서는 모듈과 버전을 다룹니다. 남이 만든 모듈을 들일 때 `go.mod`·`go.sum` 이 무엇을 고정하고, 최소 버전 선택이 어떤 버전을 고르는지 실험합니다.
