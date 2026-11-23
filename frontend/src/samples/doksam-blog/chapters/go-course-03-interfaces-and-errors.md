# Go 강의 3편 — 인터페이스는 쓰는 쪽이 정하고, 에러는 감싸서 올린다

> 예외(try/catch)가 없는 Go 에서 실패는 어떻게 전달하고, 받는 쪽은 어떻게 가려냅니까?

실패는 반환값 `error` 로 돌려줍니다. `error` 는 메서드 하나짜리 인터페이스라 어떤 타입이든 실패를 담습니다. 각 함수는 받은 에러에 맥락을 붙여 `%w` 로 감싸 올리고, 받는 쪽은 `errors.Is`(이 값인가)와 `errors.As`(이 타입인가)로 사슬 안을 찾습니다. 이 편에서는 인터페이스부터 보고, 2편의 문의 접수함(inbox)에 저장소와 에러를 붙입니다.

> [그림] 배달 로봇이 상자(값)와 그 위의 빨간 봉투(err)를 함께 건넨다. 열린 봉투 안에 설정 읽기, 파일 열기, open config.json 순으로 감싼 사연이 들어 있고, errors.Is 돋보기가 가장 안쪽 카드를 본다

Go 함수는 상자와 봉투를 함께 건넵니다. 받는 쪽은 봉투부터 열고, 필요하면 감싼 봉투를 하나씩 열어 가장 안쪽 원인을 확인합니다.

## 인터페이스는 선언 없이 메서드만 맞으면 만족한다

1편의 `run` 은 출력을 `io.Writer` 로 받았고, `main` 은 `*os.File` 을, 테스트는 `*bytes.Buffer` 를 넘겼습니다. 두 타입은 `io.Writer` 를 구현한다고 적지 않았지만 `Write([]byte) (int, error)` 메서드가 있어 만족합니다.

| | Java | Go |
|---|---|---|
| 구현 표시 | `class A implements I` 를 적는다 | 적지 않는다 |
| 구현이 인터페이스를 알아야 하나 | 안다(import 한다) | 몰라도 된다 |
| 만족 여부 확인 | 컴파일 | 컴파일(대입하는 곳에서) |

메서드가 모자라면 대입하는 곳에서 `halfStore does not implement Store (missing method Get)` 처럼 컴파일이 멈춥니다.

## 인터페이스는 쓰는 쪽에 필요한 만큼만 정의한다

2편의 접수함을 둘로 나눕니다. `inbox` 는 메시지를 검사하고 저장은 `memstore` 에 맡깁니다. 저장소 인터페이스는 저장소를 **쓰는** `inbox` 가 정의합니다.

```go
// Store 는 Service 가 저장소에 바라는 동작이다. 쓰는 쪽인 이 패키지가 정의한다.
type Store interface {
	Save(ctx context.Context, m Message) (string, error)
	Get(ctx context.Context, id string) (Message, error)
}

// Service 는 메시지를 검사해 저장소에 맡긴다.
type Service struct {
	store Store
	now   func() time.Time // 시험에서 시각을 고정하려고 함수로 받는다
}

func NewService(store Store, now func() time.Time) *Service {
	return &Service{store: store, now: now}
}
```

`memstore.Store` 는 `Save` 와 `Get` 만 가졌을 뿐 `inbox.Store` 를 언급하지 않습니다. 만족 여부는 테스트 파일 한 줄로 컴파일 때 확인합니다.

```go
var _ inbox.Store = (*memstore.Store)(nil)
```

DB 저장소로 바꿔도 `inbox` 는 고치지 않고, 시험에서는 실패만 내는 가짜를 넣습니다(연습 2). `NewService` 는 인터페이스를 받고 구체 타입 `*Service` 를 돌려줍니다.

시계는 함수 필드로 받았습니다. 이 사이트의 Go 서버도 문의 처리기에 저장을 `insert func(ctx context.Context, message string) error`, 시계를 `now func() time.Time` 으로 넣습니다.

| 의존 | 동작 수 | 고른 것 | 시험에서 넣는 것 |
|---|---:|---|---|
| 접수함의 저장소 | 2 | 인터페이스 `Store` | 메모리 저장소, 실패하는 가짜 |
| 시계 | 1 | 함수 필드 `now` | 고정 시각을 돌려주는 함수 |
| 사이트 서버의 저장 | 1 | 함수 필드 `insert` | 받은 글을 모으는 함수 |

동작이 하나면 함수, 둘 이상이 함께 다니면 인터페이스가 덜 번거롭습니다.

## any 에서 값을 꺼낼 때는 쉼표 ok 로 확인한다

`any` 는 `interface{}` 의 별칭입니다. `go.mod` 를 `go 1.17` 로 내리면 `predeclared any requires go1.18 or later` 로 거절되니 1.18 부터입니다. JSON 을 `map[string]any` 로 받으면 값마다 타입이 다릅니다. 이 사이트의 Go 서버는 `s, _ := m[key].(string)` 한 줄로 문자열이 아닌 값을 빈 문자열로 바꿉니다(`str` 함수). 타입별로 갈라야 하면 타입 스위치를 씁니다.

```go
func describe(v any) string {
	switch x := v.(type) {
	case string:
		return "문자열 " + x
	case float64:
		return fmt.Sprintf("숫자 %g", x)
	case nil:
		return "null"
	default:
		return fmt.Sprintf("모르는 타입 %T", x)
	}
}
```

`{"name":"홍길동","replyTo":1,"tags":["a"],"memo":null}` 을 넣은 결과입니다. 첫 줄은 `str` 의 결과입니다.

```text
name="홍길동" replyTo=""
name → 문자열 홍길동
replyTo → 숫자 1
tags → 모르는 타입 []interface {}
memo → null
panic: interface conversion: interface {} is float64, not string
```

JSON 숫자는 `float64` 로 들어옵니다. 마지막 줄은 쉼표 ok 없이 `m["replyTo"].(string)` 으로 단언한 결과라, 외부 입력에는 `v, ok := x.(T)` 를 씁니다.

## 에러는 메서드 하나짜리 인터페이스다

표준 라이브러리 소스(`builtin/builtin.go`)에서 `error` 는 `Error() string` 하나만 요구하는 인터페이스입니다. 호출부가 갈라야 하는 실패는 두 가지로 노출합니다. 정해진 값 하나로 충분하면 센티널(sentinel, 표식 값) 에러를, 필드가 필요하면 에러 타입을 만듭니다.

```go
// ErrNotFound 는 찾는 메시지가 없을 때 돌려준다. 호출부는 errors.Is 로 가른다.
var ErrNotFound = errors.New("메시지 없음")

// ValidationError 는 입력 필드 하나가 규칙을 어긴 것이다.
type ValidationError struct {
	Field  string
	Reason string
}

func (e *ValidationError) Error() string { return e.Field + ": " + e.Reason }

// Validate 는 어긴 규칙을 모두 모아 돌려준다. 어긴 것이 없으면 nil 이다.
func (m Message) Validate() error {
	var errs []error
	if strings.TrimSpace(m.ReplyTo) == "" {
		errs = append(errs, &ValidationError{Field: "replyTo", Reason: "비어 있다"})
	}
	// … body 의 빈 값·길이 검사도 같은 모양이다
	return errors.Join(errs...)
}
```

`errors.Join`(Go 1.20)은 여러 에러를 묶고, 넘긴 것이 없으면 nil 을 돌려줍니다. 연락처와 내용을 비우고 `Post` 를 부르면 틀린 칸이 한 번에 나옵니다.

```text
메시지 검사: replyTo: 비어 있다
body: 비어 있다
```

메시지는 소문자로 시작하고 마침표 없이 씁니다. 감쌀 때마다 "실패"를 붙이면 `설정 읽기 실패: 파일 열기 실패: …` 처럼 겹치니 맥락만 적습니다.

## %w 로 감싸야 사슬이 이어진다

없는 설정 파일을 두 겹의 함수로 열고, 안쪽에서만 `%w` 와 `%v` 를 바꿔 가며 감쌌습니다.

```go
func openConfig(path string, wrap bool) error {
	_, err := os.Open(path)
	if err == nil {
		return nil
	}
	if wrap {
		return fmt.Errorf("파일 열기: %w", err)
	}
	return fmt.Errorf("파일 열기: %v", err)
}

func loadConfig(path string, wrap bool) error {
	if err := openConfig(path, wrap); err != nil {
		return fmt.Errorf("설정 읽기: %w", err)
	}
	return nil
}
```

```text
wrap=true  설정 읽기: 파일 열기: open config.json: no such file or directory
  Is(fs.ErrNotExist)=true As(*fs.PathError)=true
  Op=open Path=config.json
wrap=false 설정 읽기: 파일 열기: open config.json: no such file or directory
  Is(fs.ErrNotExist)=false As(*fs.PathError)=false
```

출력 문자열은 같은데, `%v` 로 한 번 감싸면 원인이 글자로만 남아 `errors.Is` 도 `errors.As` 도 찾지 못합니다. 로그로는 이 차이를 알 수 없습니다. `go doc fmt.Errorf` 는 `%w` 가 있으면 돌려주는 에러가 `Unwrap` 메서드를 갖고, `%w` 가 둘 이상이면 `Unwrap() []error` 를 갖는다고 적습니다.

## Is 는 값을, As 는 타입을 찾는다

| 함수 | 하는 일 | 도입 |
|---|---|---|
| `errors.Is(err, target)` | 사슬에서 같은 값(센티널)을 찾는다 | 1.13 |
| `errors.As(err, &target)` | 그 타입의 첫 에러를 변수에 담는다 | 1.13 |
| `errors.AsType[E](err)` | `As` 와 같고 변수 선언이 필요 없다 | 1.26 |
| `errors.Join(errs...)` | 여러 에러를 묶는다 | 1.20 |

도입 버전은 `$(go env GOROOT)/api/go1.N.txt` 로 확인했습니다. `Service.Get` 은 `ErrNotFound` 를 감싸 올려서 `==` 로는 같지 않습니다.

```text
메시지 m9 읽기: 메시지 없음
Is ErrNotFound: true
== ErrNotFound: false
```

이 사이트의 Go 서버도 종료할 때 `errors.Is(err, http.ErrServerClosed)` 로 정상 종료를 가립니다.

`go.mod` 가 `go 1.25` 인 모듈에서 `errors.AsType` 을 쓰면 빌드와 `go test` 는 통과하고 `go vet` 만 멈춥니다.

```text
$ go vet .
main.go:12:24: errors.AsType requires go1.26 or later (module is go1.25)
```

`go test` 가 먼저 돌리는 vet 검사 목록에 이 검사가 없어서, 완료 조건에 `go vet` 을 따로 둡니다.

## 기록은 맨 위에서 한 번만, panic 은 프로그래머 실수에만 쓴다

로그도 남기고 에러도 돌려주면 같은 실패가 계층마다 기록됩니다. `Service` 는 감싸서 올리기만 합니다. 이 사이트의 Go 서버는 저장 에러를 문의 처리기에서 한 번만 `slog.Error` 로 기록하고 503 으로 답합니다.

`panic` 은 입력이 아니라 코드가 틀렸을 때 씁니다. 제 한국어 글 검사기는 정규식을 패키지 변수에서 `regexp.MustCompile` 로 만들어, 패턴이 틀리면 뜨자마자 패닉으로 알게 합니다. `go doc net/http.Handler` 는 처리기 패닉을 서버가 복구하고 스택을 기록한다고 적습니다. 복구 경계는 9편에서 다룹니다.

## 한 단계 더 — 인터페이스 값은 두 칸이고 nil 도 두 가지다

**인터페이스 값은 두 단어 크기입니다.** 64비트 맥에서 `unsafe.Sizeof` 로 재면 `error` 와 `any` 변수는 16바이트, 포인터는 8바이트였습니다. 런타임 소스 `runtime/runtime2.go` 의 정의도 두 칸입니다. 메서드가 있는 인터페이스는 `iface{tab *itab; data unsafe.Pointer}`, `any` 는 `eface{_type *_type; data unsafe.Pointer}` 로, 앞 칸에 타입 정보를 두고 뒤 칸이 데이터를 가리킵니다.

**그래서 nil 이 두 가지입니다.** 실패가 없을 때 `*ValidationError` 형의 nil 을 `error` 로 돌려주는 함수를 돌렸습니다.

```go
func check(ok bool) error {
	var ve *ValidationError
	if !ok {
		ve = &ValidationError{Field: "body"}
	}
	return ve // 실패가 없어도 타입 칸이 찬다
}
```

```text
err == nil: false
%T=*main.ValidationError, %v=<nil>
```

값 칸은 nil 이지만 타입 칸이 차서 `err == nil` 이 거짓입니다. `%v` 의 `<nil>` 은 `Error()` 가 nil 포인터에서 낸 패닉을 fmt 가 복구하고 찍은 것입니다(`fmt/print.go` 의 `catchPanic`). `go vet` 도 잡지 않았으니, 실패가 없으면 리터럴 `nil` 을 돌려줍니다. 내부 표현은 14편에서 봅니다.

**인터페이스 호출 자체는 싸고, 비용은 다른 데서 납니다.** 구체 타입이 바로 보이면 컴파일러가 직접 호출로 바꿉니다(`-gcflags=-m`).

```text
./dispatch.go:19:7: devirtualizing i.Inc to *Counter
```

구체 타입 호출과 패키지 변수에 든 인터페이스 호출을 `b.Loop`(Go 1.24)로 5번씩 쟀습니다. 앞쪽만 인라인됐는데도 중앙값은 1.58ns 와 1.68ns 였고, 크게 갈린 것은 할당입니다.

| 벤치마크 | 중앙값 | 할당 |
|---|---:|---:|
| 24바이트 구조체 값을 `any` 에 넣기 | 8.32 ns/op | 24 B/op, 1 allocs/op |
| 같은 구조체의 포인터를 `any` 에 넣기 | 1.70 ns/op | 0 B/op, 0 allocs/op |

측정은 2026-11-18, Apple M4 Pro, Go 1.26.2 이고 다른 빌드가 함께 돌던 때라 비율만 봅니다. 값을 인터페이스에 넣으면 데이터 칸에 담을 사본을 힙에 만듭니다(`-m` 출력 `p escapes to heap`). 15편에서 이스케이프 분석으로 다룹니다.

**`errors.Is` 는 사슬을 깊이 우선으로 훑습니다.** `errors/wrap.go` 의 `is` 는 반복문 하나입니다. 현재 에러가 target 과 같거나 `Is(error) bool` 이 참이면 끝내고, 아니면 `Unwrap() error` 로 내려가거나 `Unwrap() []error` 의 자식마다 재귀합니다. `Join` 으로 묶은 검사 에러를 `As` 가 찾는 것도 이 순회 덕분입니다.

## 면접에서 이렇게 묻는다

**Q1. 인터페이스는 어디에 정의합니까?**
쓰는 쪽에 필요한 메서드만 정의합니다. 선언 없이 만족되므로 구현 패키지는 인터페이스를 몰라도 되고, 구현을 갈아 끼우거나 가짜를 만들기 쉽습니다. 동작이 하나면 함수 타입으로 받아도 됩니다.

**Q2. `%w` 와 `%v` 로 감쌀 때 무엇이 다릅니까?**
문자열은 같지만 `%w` 만 `Unwrap` 으로 원인을 남깁니다. `%v` 로 감싸면 `errors.Is`·`errors.As` 가 원인을 찾지 못합니다. 거꾸로 `%w` 로 감싼 원인은 호출부가 기댈 수 있는 API 가 되므로, 내부 구현의 에러를 숨기고 싶으면 `%v` 로 끊습니다.

**Q3. `errors.Is` 와 `errors.As` 는 어떻게 다릅니까?**
`Is` 는 사슬에서 같은 값(센티널)을 찾고, `As` 는 지정한 타입의 첫 에러를 찾아 변수에 담습니다. 필드가 필요하면 `As`, 종류만 알면 되면 `Is` 입니다. Go 1.26 부터는 `errors.AsType` 으로 변수 선언 없이 꺼냅니다.

**Q4. nil 을 돌려줬는데 `err != nil` 이 참인 경우를 설명하십시오.**
인터페이스 값은 타입과 데이터 두 칸입니다. 구체 타입 포인터 변수(nil)를 `error` 로 돌려주면 타입 칸이 차서 nil 이 아닙니다. 실패가 없으면 리터럴 `nil` 을 돌려주고, 반환 타입을 구체 포인터가 아닌 `error` 로 둡니다.

## 연습

### 연습 1 — 에러를 HTTP 상태 코드로 가르기

`Of(err error) int` 를 작성합니다. nil 은 200, 사슬 안에 `ErrNotFound` 가 있으면 404, `*ValidationError` 가 있으면 400, 나머지는 500 입니다. `%v` 로 끊긴 사슬도 표에 넣어 시험합니다.

<details>
<summary>풀이</summary>

```go
func Of(err error) int {
	switch {
	case err == nil:
		return http.StatusOK
	case errors.Is(err, inbox.ErrNotFound):
		return http.StatusNotFound
	}
	if _, ok := errors.AsType[*inbox.ValidationError](err); ok {
		return http.StatusBadRequest
	}
	return http.StatusInternalServerError
}
```

표 주도 테스트에 `%w` 로 감싼 `ErrNotFound`(404), `errors.Join` 안의 검사 에러(400), 같은 원인을 `%v` 로 감싼 것(500)을 넣었고 `go test ./...` 가 통과합니다. `%v` 하나가 404 를 500 으로 바꾼다는 것을 테스트가 고정합니다.

</details>

### 연습 2 — 실패하는 저장소로 Post 시험하기

`Save` 가 항상 `errFull` 을 돌려주는 가짜 저장소를 만들어 `Service.Post` 가 그 에러를 감싸 올리는지 시험합니다.

<details>
<summary>풀이</summary>

```go
var errFull = errors.New("저장소 가득 참")

// fullStore 는 inbox.Store 를 만족한다. 선언 어디에도 그렇다고 적지 않는다.
type fullStore struct{}

func (fullStore) Save(context.Context, inbox.Message) (string, error) { return "", errFull }
func (fullStore) Get(context.Context, string) (inbox.Message, error) {
	return inbox.Message{}, inbox.ErrNotFound
}
```

`inbox.NewService(fullStore{}, time.Now)` 로 `Post` 를 부르면 `errors.Is(err, errFull)` 이 참이고, `err.Error()` 는 `메시지 저장: 저장소 가득 참` 이었습니다. 인터페이스를 쓰는 쪽에 두었기 때문에 테스트 파일 안의 타입만으로 가짜를 만듭니다.

</details>

## 정리

Go 는 실패를 `error` 값으로 돌려줍니다. 각 층은 맥락을 붙여 `%w` 로 감싸고, 받는 쪽은 `errors.Is` 와 `errors.As` 로 사슬 안의 원인을 가려냅니다. 인터페이스는 쓰는 쪽에 작게 정의하고, 실패가 없을 때는 리터럴 `nil` 을 돌려줍니다.

4편에서는 이 접수함을 표준 라이브러리만으로 HTTP 서버로 엽니다. 라우팅, JSON, 설정, 정적 파일을 다룹니다. 연습 1의 상태 코드 함수를 층이 나뉜 서비스의 맨 위에 두는 설계는 9편에서 다시 봅니다.
