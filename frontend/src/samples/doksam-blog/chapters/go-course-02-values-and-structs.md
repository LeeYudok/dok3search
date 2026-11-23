# Go 강의 2편 — 넘기면 복사되고, 함께 쓰려면 포인터를 넘긴다

> Go 는 값을 넘길 때 무엇을 복사하고, 언제 포인터를 씁니까?

Go 는 대입과 함수·메서드 호출에서 언제나 값을 복사합니다. 구조체는 필드 전체가 복사되고, 슬라이스·맵·문자열은 데이터가 아니라 작은 머리(헤더)만 복사됩니다. 고친 내용을 부른 쪽도 봐야 하거나 뮤텍스처럼 복사하면 안 되는 필드가 있으면 포인터를 넘깁니다. 연재 내내 키울 문의 보관함 `inbox` 의 타입을 만들며 확인합니다.

> [그림] 왼쪽에서 배달 로봇이 원본 상자를 들고 있고 복사기에서 사본 상자 두 개가 나와 있다. 오른쪽에는 사물함 한 칸의 상자를 a, b 두 꼬리표가 함께 가리킨다

왼쪽 값 전달에서는 사본을 고쳐도 로봇이 든 원본이 그대로입니다. 오른쪽 포인터 전달에서는 꼬리표가 둘이어도 상자가 하나라서 누가 고치든 같은 상자가 바뀝니다.

## 구조체는 대입하고 넘길 때 통째로 복사된다

```go
type Message struct {
	Name string
	Tags []string
}

func rename(m Message)     { m.Name = "바뀐 이름" }
func renamePtr(m *Message) { m.Name = "바뀐 이름" }

func main() {
	a := Message{Name: "처음", Tags: []string{"문의"}}
	b := a // 필드 전체를 복사한다
	b.Name = "사본"
	rename(a) // 사본을 고친다
	fmt.Println(a.Name, b.Name)
	renamePtr(&a) // 주소를 넘긴다
	fmt.Println(a.Name)
	b.Tags[0] = "스팸" // 슬라이스는 머리만 복사됐다
	fmt.Println(a.Tags[0])
}
```

```text
처음 사본
바뀐 이름
스팸
```

`b := a` 와 `rename(a)` 는 사본을 만들고, `&a` 를 넘겨야 원본이 바뀝니다. 포인터로도 필드는 점으로 접근하며 포인터 연산(`p + 1`)은 컴파일 오류입니다. 마지막 줄이 함정입니다. 안의 슬라이스는 같은 배열을 가리켜서 `b.Tags` 를 고치면 `a.Tags` 도 바뀝니다.

| 넘기는 것 | Python·JavaScript·Java 객체 | Go 구조체 |
|---|---|---|
| 대입·호출 | 같은 객체를 가리킨다 | 필드 전체를 복사한다 |
| 함께 고치려면 | 그대로 넘긴다 | `&` 로 포인터를 넘긴다 |

## 제로 값만으로 쓸 수 있게 만든다

변수는 선언하는 순간 제로 값(0, `""`, `false`, `nil`)을 갖고, 표준 라이브러리에는 그 상태로 바로 쓰는 타입이 많습니다.

```go
var buf bytes.Buffer // 만들지 않고 바로 쓴다
buf.WriteString("제로 값 그대로")
var mu sync.Mutex
mu.Lock()
mu.Unlock()
var names []string // nil 슬라이스에도 append 된다
names = append(names, "a")
var counts map[string]int // nil 맵은 읽기만 된다
fmt.Println(buf.String(), len(names), counts["x"], counts == nil)
```

```text
제로 값 그대로 1 0 true
```

`go doc bytes.Buffer` 와 `go doc sync.Mutex` 는 제로 값이 바로 쓸 수 있는 상태라고 적습니다. 직접 만드는 타입도 이렇게 설계하면 초기화를 잊는 버그가 사라집니다. 아래 `Inbox` 는 맵을 처음 쓸 때 만들어서 `var b Inbox` 로 바로 씁니다.

## 설정값이 필요한 타입은 생성자 함수로 만든다

제로 값으로 정할 수 없는 설정이 있으면 `NewX` 함수를 둡니다. 이 사이트의 Go 서버에 있는 IP 당 요청 제한기입니다.

```go
type Limiter struct {
	mu     sync.Mutex
	hits   map[string][]time.Time
	limit  int
	window time.Duration
}

func NewLimiter(limit int, window time.Duration) *Limiter {
	return &Limiter{hits: map[string][]time.Time{}, limit: limit, window: window}
}
```

모든 요청 처리기가 같은 기록을 봐야 하고 안의 뮤텍스는 복사하면 안 되므로 포인터를 돌려줍니다. Go 에는 생성자 문법이 없어 이 함수가 관례입니다.

## 고치는 메서드는 포인터 리시버로 쓴다

보관함 본체는 상태를 고치므로 리시버(receiver, 메서드를 받는 값)가 `*Inbox` 입니다.

```go
// Inbox 는 메시지를 받은 순서대로 보관한다. 제로 값으로 바로 쓴다.
type Inbox struct {
	mu   sync.Mutex
	msgs []Message
	byID map[string]int
}

func (b *Inbox) Add(m Message) string {
	b.mu.Lock()
	defer b.mu.Unlock()
	if b.byID == nil {
		b.byID = map[string]int{}
	}
	m.ID = fmt.Sprintf("m%d", len(b.msgs)+1)
	b.byID[m.ID] = len(b.msgs)
	b.msgs = append(b.msgs, m)
	return m.ID
}
```

같은 모양의 `Get(id string) (Message, bool)` 은 메시지를 값으로 돌려줍니다. 받은 쪽이 `Body` 를 고쳐도 보관함 안은 그대로이고, 테스트가 이것을 확인합니다. 리시버를 값으로 잘못 쓰면 어떻게 될까요? 뮤텍스가 든 카운터로 실험했습니다.

```go
type Counter struct {
	mu sync.Mutex
	n  int
}

func (c Counter) Inc() { // 값 리시버라서 사본의 n 을 올린다
	c.mu.Lock()
	defer c.mu.Unlock()
	c.n++
}
```

```text
$ go run .
0
$ go vet .
main.go:14:9: Inc passes lock by value: example.com/copylock.Counter contains sync.Mutex
```

`Inc` 를 두 번 불러도 0 이 나옵니다. 컴파일러는 막지 않고, `go vet` 의 copylocks 검사가 잡습니다. 리시버는 이렇게 고릅니다.

| 상황 | 리시버 |
|---|---|
| 필드를 고친다 | 포인터 |
| 뮤텍스처럼 복사하면 안 되는 필드가 있다 | 포인터 |
| 구조체가 크다(아래 "한 단계 더") | 포인터 |
| 작고 바뀌지 않는 값이다 | 값. `time.Time` 이 예다(`func (t Time) Add(d Duration) Time`) |

## 값에는 포인터 리시버 메서드가 없다

```go
type Inbox struct{ n int }

func (b *Inbox) Add(body string) int { b.n++; return b.n }

type Adder interface{ Add(body string) int }

func main() {
	var box Inbox
	box.Add("x") // 변수는 주소를 얻을 수 있어 (&box).Add 로 불린다

	var a Adder = &box // *Inbox 는 Add 를 가진다
	var b Adder = box  // Inbox 값의 메서드 집합에는 Add 가 없다
	_, _ = a, b
}
```

```text
./main.go:14:16: cannot use box (variable of struct type Inbox) as Adder value in variable declaration: Inbox does not implement Adder (method Add has pointer receiver)
```

명세는 `T` 의 메서드 집합을 `T` 리시버 메서드로, `*T` 의 메서드 집합을 둘 다로 정합니다. `box.Add("x")` 는 컴파일러가 주소를 대신 잡아 줘서 되지만, 인터페이스에 넣을 때는 그 편의가 없습니다. 한 타입 안에서 리시버 종류를 섞지 않는 이유입니다.

## 임베딩은 상속이 아니라 필드를 끌어올린다

필드 이름 없이 타입만 적으면 그 필드와 메서드가 바깥 타입으로 올라옵니다(promotion).

```go
type Stamp struct{ At time.Time }

func (s Stamp) Age(now time.Time) time.Duration { return now.Sub(s.At) }

type Message struct {
	Stamp // 이름 없이 넣으면 필드와 메서드가 바깥으로 올라온다
	Body  string
}
```

`m.Age(…)` 와 `m.At` 이 바로 되고, 실행하면 `1h30m0s true` 가 나옵니다. 상속과 달리 `Age` 의 리시버는 안쪽 `Stamp` 이고, `Message` 를 `Stamp` 자리에 넘기면 `cannot use m (variable of struct type Message) as Stamp value` 로 거절됩니다.

올라온 메서드는 공개됩니다. `sync.Mutex` 를 이름 없이 넣은 타입과 `mu` 로 넣은 타입의 메서드를 `reflect` 로 세었습니다.

```text
*main.Exposed [Lock TryLock Unlock]
*main.Hidden []
```

이름 없이 넣으면 `Lock`·`Unlock` 이 밖에 공개되므로 `Inbox` 와 `Limiter` 는 `mu` 로 이름을 붙였습니다.

## 구조체 태그는 JSON 이름을 정한다

필드 뒤 백틱 문자열이 태그이고, `encoding/json` 이 읽습니다.

```go
type Message struct {
	Name    string `json:"name,omitempty"`
	ReplyTo string `json:"replyTo"`
	note    string // 소문자 필드는 JSON 에 나가지 않는다
}
```

`Message{ReplyTo: "a@example.com", note: "내부 메모"}` 를 `json.Marshal` 하면 `{"replyTo":"a@example.com"}` 입니다. 빈 `Name` 은 `omitempty` 로, 소문자 `note` 는 공개되지 않아 빠졌습니다. 자세한 것은 4편에서 다룹니다.

## 슬라이스는 배열을 들여다보는 창이다

```go
s := make([]int, 3, 5)
fmt.Println(len(s), cap(s), s)
t := s[1:3] // 같은 배열의 창
t[0] = 99
fmt.Println(s, len(t), cap(t))
t = append(t, 7) // 용량이 남아 같은 배열에 쓴다
fmt.Println(s[:5])
u := append(s, 1, 2, 3) // 용량을 넘어 새 배열로 옮긴다
u[0] = -1
fmt.Println(s[0], u[0], len(u), cap(u))
```

```text
3 5 [0 0 0]
[0 99 0] 2 4
[0 99 0 7 0]
0 -1 6 10
```

`len` 은 지금 보이는 원소 수, `cap` 은 밑의 배열에 남은 자리입니다. `s[1:3]` 은 복사가 아니라 같은 배열의 창이라서 `t[0]` 을 고치면 `s[1]` 이 바뀝니다. `append` 는 자리가 남으면 같은 배열에 쓰고, 모자라면 새 배열로 옮깁니다. 그래서 `append` 의 결과는 항상 변수에 다시 받습니다. 늘어나는 규칙은 13편에서 봅니다.

## 맵은 순서를 지키지 않는다

같은 맵을 세 번 돌았습니다.

```go
m := map[string]int{"a": 1, "b": 2, "c": 3, "d": 4, "e": 5}
for range 3 {
	var keys []string
	for k := range m {
		keys = append(keys, k)
	}
	fmt.Println(keys)
}
v, ok := m["z"] // 없으면 제로 값과 false
fmt.Println(v, ok)
fmt.Println(slices.Sorted(maps.Keys(m)))
```

```text
[a b c d e]
[a b c d e]
[e a b c d]
0 false
[a b c d e]
```

세 번째 순서가 다릅니다. 명세도 맵 순회 순서는 정해지지 않았다고 적습니다. 순서가 필요하면 키를 정렬합니다. `slices`·`maps` 패키지는 Go 1.21 에 들어왔습니다. 반복자(iterator)를 쓰는 `maps.Keys`·`slices.Sorted` 는 Go 1.23 부터입니다. 정수를 도는 `for range 3` 은 Go 1.22 부터라서 `go.mod` 를 `go 1.21` 로 낮추면 `requires go1.22 or later` 로 거절됩니다. `v, ok := m[k]` 의 `ok` 는 "없음"과 "제로 값이 들어 있음"을 가릅니다.

## 문자열은 바이트의 나열이고 글자는 rune 이다

```go
s := "안녕 Go"
fmt.Println(len(s), utf8.RuneCountInString(s))
for i, r := range s {
	fmt.Printf("%d:%c ", i, r)
}
fmt.Println()
fmt.Println(s[0], s[:3], utf8.ValidString(s[:2]))
```

```text
9 5
0:안 3:녕 6:  7:G 8:o 
236 안 false
```

`len` 은 바이트 수이고, 한글은 UTF-8 로 3바이트라서 다섯 글자가 9바이트입니다. `for range` 는 글자(`rune`) 단위로 돌며 `i` 에 바이트 위치를 줍니다. 인덱스로 한글 중간을 자르면 올바르지 않은 UTF-8 이 됩니다. 이 사이트의 Go 서버는 이름·연락처·본문 길이 제한을 `utf8.RuneCountInString` 으로, 즉 글자 수로 잽니다.

## 한 단계 더 — 헤더 크기와 복사 비용

**복사되는 머리는 몇 바이트일까요?** `unsafe.Sizeof` 로 쟀습니다(arm64).

```text
string 16 slice 24 map 8 pointer 8 any 16
Message 88 Inbox 40
```

문자열은 포인터·길이(16), 슬라이스는 포인터·길이·용량(24), 맵은 포인터(8)입니다. 넘겨도 데이터는 복사되지 않습니다. `Message` 는 문자열 넷과 `time.Time` 을 더한 88바이트입니다. `any` 의 16바이트는 14편에서 풉니다.

**구조체를 값으로 넘기면 얼마나 느릴까요?** 512바이트와 88바이트 구조체를 값과 포인터로 넘기는 벤치마크를 `-benchmem -count 5` 로 돌렸습니다. 함수에는 `//go:noinline` 을, 루프에는 Go 1.24 의 `b.Loop` 를 썼습니다. 표는 다섯 번의 중앙값입니다(2026-11-18, Apple M4 Pro, Go 1.26.2, 다른 작업과 함께 돌림).

| 함수 | ns/op (중앙값) | allocs/op |
|---|---:|---:|
| 512바이트 값 전달 | 7.021 | 0 |
| 512바이트 포인터 전달 | 1.631 | 0 |
| 88바이트 값 전달 | 1.888 | 0 |
| 88바이트 포인터 전달 | 1.945 | 0 |

512바이트는 호출마다 약 5.4ns 를 복사에 썼습니다. 88바이트는 차이가 없었습니다. 보통 크기의 구조체에서 포인터를 고르는 기준은 속도가 아니라 뜻(함께 고치는가, 복사하면 안 되는가)입니다.

**포인터는 값을 힙으로 보낼 수 있습니다.** 지역 변수의 주소를 돌려주는 함수를 `go build -gcflags=-m` 으로 빌드했습니다.

```text
experiments/escape/main.go:9:2: moved to heap: m
experiments/escape/main.go:14:14: m does not escape
```

함수가 끝나도 살아야 하는 `m` 은 힙으로 갔고, 값으로 받아 길이만 쓴 `m` 은 스택에 남았습니다. 힙에 간 값은 가비지 컬렉터가 치웁니다(15·16편).

## 면접에서 이렇게 묻는다

**Q1. 값 리시버와 포인터 리시버는 어떻게 고릅니까?**
필드를 고치거나, 뮤텍스처럼 복사하면 안 되는 필드가 있거나, 구조체가 크면 포인터입니다. 작고 바뀌지 않는 값은 값 리시버이고 `time.Time` 이 예입니다. 한 타입 안에서는 섞지 않는다고 덧붙이고, 근거로 메서드 집합을 듭니다.

**Q2. 포인터 리시버로 메서드를 만들었는데 왜 값이 인터페이스를 만족하지 못합니까?**
명세상 `T` 의 메서드 집합에는 `T` 리시버 메서드만 있고, `*T` 에 둘 다 있습니다. 변수에서 `v.M()` 이 되는 것은 컴파일러가 `(&v).M()` 으로 바꿔 주기 때문이며 인터페이스 대입에는 이 변환이 없습니다.

**Q3. 슬라이스를 함수에 넘기면 복사입니까, 참조입니까?**
24바이트 헤더(포인터·길이·용량)가 복사됩니다. 원소를 고치면 부른 쪽도 봅니다. 함수 안의 `append` 는 용량에 따라 같은 배열에 쓰거나 새 배열로 옮기므로, 늘린 결과는 반환값으로 돌려줍니다.

**Q4. 맵을 순회하는 순서에 기대도 됩니까? nil 맵은 어떻습니까?**
안 됩니다. 명세가 순서를 정하지 않았고 실험에서도 바뀌었으니 필요하면 키를 정렬합니다(`slices.Sorted(maps.Keys(m))`, 1.23+). nil 맵은 읽기는 제로 값, 쓰기는 패닉입니다.

## 연습

### 연습 1 — 기본 태그를 붙였더니 다른 결과가 바뀐다

아래 `withDefault` 를 두 번 부르고 `a[1]` 만 고쳤는데 `b[1]` 이 바뀝니다. 이유를 설명하고 고칩니다.

```go
func withDefault(tags []string) []string { return append(tags, "문의") }

base := make([]string, 1, 4)
base[0] = "일반"
a := withDefault(base)
b := withDefault(base)
a[1] = "스팸"
fmt.Println(b[1], len(base), cap(base))
```

<details>
<summary>풀이</summary>

실행하면 `스팸 1 4` 가 나옵니다. `base` 는 길이 1, 용량 4 라서 두 `append` 모두 같은 배열의 1번 칸에 썼고, `a` 와 `b` 는 같은 배열을 봅니다. 받은 슬라이스를 늘려 돌려줄 때는 먼저 복사합니다(`slices.Clone`, Go 1.21+).

```go
func withDefaultSafe(tags []string) []string { return append(slices.Clone(tags), "문의") }
```

같은 순서로 부르고 `c[1]` 을 고치면 `d[1]` 은 `문의` 그대로입니다.

</details>

### 연습 2 — 한글을 글자 수로 자르기

미리보기에 본문 앞 n 글자만 보여 주려 합니다. `s[:n]` 은 바이트로 자르므로 한글이 깨집니다. 글자 단위로 자르는 `Runes(s string, n int) string` 을 쓰고 표 주도 테스트로 확인합니다.

<details>
<summary>풀이</summary>

`for range` 가 주는 바이트 위치를 씁니다. n 번째 글자가 시작하는 위치에서 자릅니다.

```go
// Runes 는 s 를 앞에서부터 n 글자(rune)까지 남긴다.
func Runes(s string, n int) string {
	count := 0
	for i := range s { // i 는 각 글자가 시작하는 바이트 위치다
		if count == n {
			return s[:i]
		}
		count++
	}
	return s
}
```

표 주도 테스트는 `("안녕하세요", 2) → "안녕"`, `("Go 강의", 4) → "Go 강"`, `("안녕", 5) → "안녕"`, `("안녕", 0) → ""` 를 돌고 결과가 올바른 UTF-8 인지도 봅니다. 모두 통과했고, 바이트로 자른 `"안녕하세요"[:4]` 는 `utf8.ValidString` 이 `false` 였습니다.

</details>

## 정리

Go 는 넘길 때마다 복사합니다. 구조체는 통째로, 슬라이스·맵·문자열은 머리만 복사됩니다. 함께 고치거나 복사하면 안 되는 것이 들었을 때 포인터를 넘기고, 리시버도 같은 기준으로 고릅니다.

3편에서는 이 타입들을 인터페이스로 이어 붙이고, 실패를 값으로 돌려주는 Go 의 에러 처리를 다룹니다. 이 편에서 본 메서드 집합이 인터페이스 구현을 정하는 규칙이 됩니다.
