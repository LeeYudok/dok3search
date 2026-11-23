# Go 강의 14편 — 인터페이스 값은 두 칸이고, 비용은 뒤 칸을 채울 때 생긴다

> 인터페이스 값 안에는 무엇이 들어 있고, nil 비교·타입 단언·메서드 호출은 각각 얼마를 치릅니까?

인터페이스 값은 두 칸입니다. 앞 칸은 타입(메서드가 있는 인터페이스면 itab), 뒤 칸은 데이터의 주소입니다. nil 비교는 두 칸이 다 비었는지 봅니다. 단언은 앞 칸을 비교하고, 메서드 호출은 itab 의 함수 표를 거칩니다. 이 맥에서 단언과 타입 스위치는 2ns 안팎이었습니다. 비싼 것은 뒤 칸을 채우려고 힙에 사본을 만드는 일입니다. 0~255 정수, 상수, 크기 0 값, 빈 문자열, 포인터는 할당 없이 들어갔고 나머지는 1번씩 할당했습니다.

> [그림] 배달 로봇이 앞 칸에 int 꼬리표, 뒤 칸에 42 상자가 든 두 칸짜리 상자를 들고 달린다. 바닥의 상자 하나는 *Msg 꼬리표만 있고 뒤 칸이 비어 nil 아님 도장이, 다른 상자는 두 칸이 다 비어 nil 도장이 찍혀 있다

꼬리표 칸이 차 있으면 값 칸이 비어도 nil 이 아닙니다.

[3편](https://blog.doksam.com/blog/go-course-03-interfaces-and-errors/)은 typed nil 과 `any` 의 크기 16바이트를 보였습니다. 이 편은 두 칸을 직접 읽고, 칸을 채우고 비교하는 비용을 잽니다. 출력은 2026-11-18 에 Apple M4 Pro, Go 1.26.2 로 돌린 결과입니다. 런타임 설명은 Go 1.26.2 소스 기준입니다.

## 두 칸을 직접 읽으면 nil 이 두 가지로 보인다

런타임 소스 `runtime/runtime2.go` 의 `eface` 는 `_type` 포인터와 `data` 포인터 두 칸입니다. 같은 모양의 구조체로 `any` 를 덮어 읽었습니다.

```go
type eface struct {
	typ  unsafe.Pointer
	data unsafe.Pointer
}

func words(v any) eface { return *(*eface)(unsafe.Pointer(&v)) }
```

```text
nil any      typ=false data=false v==nil=true
nil *Msg     typ=true  data=false v==nil=false
int 42       typ=true  data=true  v==nil=false
Msg 값        typ=true  data=true  v==nil=false
&m           typ=true  data=true  v==nil=false
int 두 개의 타입 칸이 같다: true
int 와 Msg 의 타입 칸이 같다: false
&m 의 데이터 칸 == &m: true
Msg 값의 데이터 칸 == &m: false
```

`v == nil` 은 두 칸이 모두 빌 때만 참입니다. 타입 칸은 타입마다 하나인 타입 정보를 가리켜서, 같은 타입이면 주소가 같습니다. 포인터는 데이터 칸에 그대로 들어가고(`&m` 과 같은 주소), 구조체 값은 사본의 주소가 들어갑니다.

## 메서드가 있는 인터페이스의 앞 칸은 itab 이다

`error` 나 `fmt.Stringer` 처럼 메서드가 있으면 앞 칸이 `itab` 을 가리킵니다. Go 1.26.2 의 `runtime/runtime2.go` 에서 `itab` 은 `internal/abi/iface.go` 의 `ITab` 별칭입니다.

```go
type ITab struct {
	Inter *InterfaceType
	Type  *Type
	Hash  uint32     // copy of Type.Hash. Used for type switches.
	Fun   [1]uintptr // variable sized. fun[0]==0 means Type does not implement Inter.
}
```

인터페이스 타입, 구체 타입, 해시, 메서드 표입니다. 같은 `Msg` 를 `Namer`·`Stringer` 두 인터페이스에 넣고 앞 칸을 읽었습니다.

```text
Namer 두 값의 itab 이 같다: true
Namer 와 Stringer 의 itab 이 같다: false
두 itab 의 구체 타입 칸이 같다: true
itab 의 구체 타입 == any 의 타입 칸: true
메서드 표 첫 칸이 채워져 있다: true
nil Namer 의 itab: <nil>
```

itab 은 (인터페이스, 구체 타입) 쌍마다 하나입니다. 메서드 호출은 itab 의 `Fun` 에서 함수 주소를 꺼내 간접 호출하는 것입니다.

## 값을 넣을 때 할당이 생기는 경우는 정해져 있다

패키지 변수 `Sink any` 에 여러 값을 넣는 벤치마크를 `-benchmem -count 5` 로 돌렸습니다. 값은 실행 중에 만든 슬라이스에서 읽어 상수로 접히지 않게 했습니다.

| 넣은 값 | 중앙값 | 할당 |
|---|---:|---:|
| 정수 0~255 | 1.85 ns | 0 |
| 정수 1000~1255 | 5.79 ns | 8 B, 1회 |
| 정수 상수 `1_000_000` | 1.69 ns | 0 |
| `byte` | 1.84 ns | 0 |
| 포인터 | 1.80 ns | 0 |
| 16바이트 구조체 | 8.18 ns | 16 B, 1회 |
| `struct{}` | 1.85 ns | 0 |
| 빈 문자열 | 1.87 ns | 0 |
| 문자열 `"12345"` | 11.32 ns | 16 B, 1회 |

다른 작업과 함께 돈 상태라 시간은 비율만 봅니다. 어디서 할당을 피했는지는 컴파일러 진단으로 보입니다.

```text
$ go test -gcflags='-d=escapedebug=1' -run '^$' ./boxing
boxing/boxing_test.go:40:10: convert: using global for interface value: .stmp_1
boxing/boxing_test.go:47:14: convert: using global for single-byte interface value: .autotmp_5
boxing/boxing_test.go:69:15: convert: using global for zero-sized interface value: .stmp_2
```

상수(40행), `byte`(47행), `struct{}`(69행)는 컴파일러가 전역 데이터를 가리키게 했습니다. 정수와 문자열은 `go tool objdump` 로 보면 크기와 상관없이 `runtime.convT64`·`runtime.convTstring` 을 부릅니다. 갈림은 런타임 안에 있습니다(`runtime/iface.go`).

```go
func convT64(val uint64) (x unsafe.Pointer) {
	if val < uint64(len(staticuint64s)) {
		x = unsafe.Pointer(&staticuint64s[val])
	} else {
		x = mallocgc(8, uint64Type, false)
		*(*uint64)(x) = val
	}
	return
}
```

`staticuint64s` 는 0~255 를 미리 담은 길이 256 배열입니다. 같은 수를 두 번 넣고 데이터 칸을 비교하면 경계가 그대로 보입니다.

```text
   7: 두 번 넣은 데이터 칸이 같다=true
 255: 두 번 넣은 데이터 칸이 같다=true
 256: 두 번 넣은 데이터 칸이 같다=false
1000: 두 번 넣은 데이터 칸이 같다=false
```

순서는 컴파일러 소스 `cmd/compile/internal/walk/convert.go` 의 `dataWord` 가 정합니다. 포인터 모양이면 그대로 넣습니다. 다음으로 크기 0, 한 바이트 정수·bool, 읽기 전용 전역, 함수 밖으로 나가지 않는 1024바이트 이하 값(스택 임시 변수) 순으로 할당을 피합니다. 모두 아니면 런타임 `convT*` 를 부릅니다. 스택 임시 변수를 쓰면 같은 7 을 두 번 넣어도 스택의 다른 자리를 가리켜 `false` 가 나왔고, `-m` 은 `n does not escape` 를 찍었습니다. 이스케이프 분석은 [로드맵의 15편](https://blog.doksam.com/blog/go-course-00-roadmap/)에서 다룹니다.

## 인터페이스 비교는 타입 칸부터 본다

```go
var a, b any = []int{1}, []int{1}
var c any = 1
fmt.Println("다른 타입끼리:", a == c)
fmt.Println("같은 타입끼리:", a == b)
```

```text
다른 타입끼리: false
panic: runtime error: comparing uncomparable type []int
```

이 편의 패닉 출력은 스택을 줄였습니다. `a == c` 는 타입 칸이 달라 바로 거짓입니다. `a == b` 는 타입 칸이 같아 값을 비교하려다 패닉이 났습니다. 컴파일러 소스 `cmd/compile/internal/compare/compare.go` 의 `EqInterface` 주석은 `s.tab == t.tab` 을 먼저 평가하고 같을 때만 `ifaceeq` 를 부른다고 적습니다. `runtime/alg.go` 의 `efaceeq` 는 그 타입에 비교 함수가 없으면 이 패닉을 냅니다. 명세(`doc/go_spec.html`)의 Comparison operators 절도 같은 내용입니다. 동적 타입이 같고 그 타입이 비교 불가능하면 실행 중 패닉입니다. `map[any]bool` 에 이 슬라이스를 키로 넣으면 `hash of unhashable type []int` 패닉이 납니다. `go vet` 은 둘 다 잡지 않았습니다.

## 단언과 타입 스위치는 성공해도 실패해도 2ns 안팎이다

`any` 에 `*Msg` 를 넣어 두고 5번씩 쟀습니다. 모두 0 할당입니다.

| 연산 | 중앙값 |
|---|---:|
| 구체 타입 단언 `v.(*Msg)` | 1.89 ns |
| 인터페이스 단언 `v.(Namer)` 성공 | 2.01 ns |
| 인터페이스 단언 `v.(Sizer)` 실패(쉼표 ok) | 1.95 ns |
| 구체 타입 case 넷인 타입 스위치 | 1.87 ns |
| 인터페이스 case 둘인 타입 스위치 | 1.91 ns |

`go tool objdump` 로 보면 구체 타입 단언은 타입 칸과 상수를 비교하는 `CMP` 하나입니다. 인터페이스 단언에는 `runtime.typeAssert` 호출이 있지만 캐시를 놓칠 때만 탑니다. 그 함수의 주석은 호출 지점마다 캐시를 두어 다음에는 생성 코드가 런타임을 부르지 않게 한다고 적습니다. 그래서 차이가 0.1ns 정도입니다. 쉼표 ok 없이 실패하면 빠진 메서드 이름까지 나옵니다.

```text
panic: interface conversion: int is not fmt.Stringer: missing method String
```

## 포인터 리시버 메서드가 있으면 *T 만 인터페이스를 채운다

```text
./main.go:11:18: cannot use Counter{} (value of struct type Counter) as Incer value in variable declaration: Counter does not implement Incer (method Inc has pointer receiver)
```

명세의 Method sets 절을 따르면 `T` 의 메서드 집합에는 리시버가 `T` 인 메서드만 듭니다. `*T` 에는 두 리시버가 모두 듭니다. 위 실험에서 값을 넣으면 데이터 칸이 원본이 아닌 사본을 가리켰습니다. 원본을 고치려는 `Inc` 를 사본에 붙일 수 없으니, 처음부터 `&Counter{}` 를 넣습니다. `w.(http.Flusher)` 처럼 인터페이스를 다른 인터페이스로 바꿔 보는 코드도 실행 중 itab 을 찾는 인터페이스 단언입니다.

## 구체 타입이 보이면 컴파일러가 직접 호출로 바꾼다

```go
func Local() float64 {
	var s Shape = Square{2}
	return s.Area()
}

func Param(s Shape) float64 { return s.Area() }
func Caller() float64       { return Param(Square{3}) }

var Global Shape = Square{4}

func FromGlobal() float64 { return Global.Area() }
```

```text
$ go build -gcflags=-m ./devirt/snippet
devirt/snippet/s.go:10:15: devirtualizing s.Area to Square
devirt/snippet/s.go:14:43: inlining call to Param
devirt/snippet/s.go:14:43: devirtualizing s.Area to Square
```

`Local` 은 바로, `Caller` 는 `Param` 을 인라인한 뒤 직접 호출이 됐습니다. `Param` 자체와 패키지 변수를 부르는 `FromGlobal` 은 간접 호출로 남았습니다.

## 한 단계 더 — itab 은 한 번 만들고, 비용은 fmt 에서 샌다

**itab 은 처음 쓸 때 한 번 만듭니다.** `runtime/iface.go` 의 `getitab` 은 전역 해시 표 `itabTable`(처음 512칸)을 잠금 없이 먼저 찾습니다. 없으면 잠그고 다시 찾은 뒤 GC 가 관리하지 않는 메모리(`persistentalloc`)에 만들어 넣습니다. 타입 스위치에 쓰는 쌍은 컴파일러가 미리 만들어 둔다는 주석도 있습니다. 실행 중 itab 비용은 첫 한 번이고, 그 뒤는 캐시 조회입니다.

**`...any` 를 받는 API 는 값마다 박싱합니다.** `fmt.Sprint(n)` 과 `strconv.Itoa(n)` 을 `n = 123456` 으로 쟀습니다.

| 함수 | 중앙값 | 할당 |
|---|---:|---:|
| `fmt.Sprint(n)` | 34.18 ns | 16 B, 2회 |
| `strconv.Itoa(n)` | 11.73 ns | 8 B, 1회 |

`-m` 은 `fmt.Sprint(n)` 줄에 `n escapes to heap` 을 찍었습니다. 결과 문자열 말고 1회는 `n` 을 인터페이스에 담은 박싱입니다.

| 고를 것 | 맞는 곳 | 치르는 것 |
|---|---|---|
| 구체 타입 | 구현이 하나, 뜨거운 경로 | 갈아 끼우기 어렵다 |
| 작은 인터페이스 | 저장소·시계처럼 바꿔 끼울 경계 | 탈가상화가 안 되면 간접 호출, 값이면 박싱 |
| `any` | JSON, 로그 인자처럼 타입을 모르는 곳 | 박싱, 단언, 실행 중 패닉 위험 |
| 제네릭 | 타입만 다른 같은 알고리즘 | [로드맵의 20편](https://blog.doksam.com/blog/go-course-00-roadmap/)에서 다룬다 |

## 면접에서 이렇게 묻는다

**Q1. 인터페이스 값은 메모리에서 어떻게 생겼습니까?**
두 단어입니다. 빈 인터페이스는 타입 포인터와 데이터 포인터, 메서드가 있는 인터페이스는 itab 포인터와 데이터 포인터입니다. itab 은 인터페이스 타입, 구체 타입, 해시, 메서드 표를 갖고 (인터페이스, 구체 타입) 쌍마다 하나라서 캐시됩니다.

**Q2. `any` 에 int 를 넣으면 할당이 생깁니까?**
경우에 따라 다릅니다. 0~255 는 런타임의 `staticuint64s` 를 가리켜 0회입니다. 상수는 읽기 전용 전역이라, 함수 밖으로 나가지 않는 값은 스택 사본이라 0회입니다. 그 밖의 값은 8바이트 1회입니다. 포인터는 데이터 칸에 그대로 들어가 할당이 없습니다.

**Q3. 두 인터페이스 값을 `==` 로 비교하면 어떻게 됩니까?**
타입 칸을 먼저 비교하고, 같을 때만 값을 비교합니다. 동적 타입이 같은데 슬라이스·맵·함수처럼 비교할 수 없으면 실행 중 패닉이 납니다. 맵 키로 넣을 때도 해시 패닉이 납니다.

**Q4. 타입 단언과 타입 스위치는 비쌉니까?**
아닙니다. 구체 타입 단언은 타입 칸 비교입니다. 인터페이스 단언은 호출 지점 캐시로 itab 을 찾습니다. 이 맥에서 둘 다 2ns 안팎이었습니다. 실제 비용은 값을 인터페이스에 넣을 때의 박싱에서 납니다.

**Q5. 값 타입이 인터페이스를 만족하지 못한다는 에러는 왜 납니까?**
포인터 리시버 메서드는 `*T` 의 메서드 집합에만 들어갑니다. 인터페이스에 값을 넣으면 사본이 들어가 원본을 고칠 수 없으므로 `&T{}` 를 넣습니다.

## 연습

### 연습 1 — 할당 횟수 예측하기

`Sink any` 에 `bool`, `int16` 200 과 300, 빈 문자열과 `"go"`, `struct{}`, `[2]int`, 포인터를 넣을 때 할당이 몇 번인지 먼저 적고, `testing.AllocsPerRun` 으로 확인합니다.

<details>
<summary>풀이</summary>

```go
cases := []struct {
	name string
	box  func()
	want float64
}{
	{"bool", func() { Sink = flag }, 0},
	{"int16 200", func() { Sink = i16s }, 0},
	{"int16 300", func() { Sink = i16 }, 1},
	{"빈 문자열", func() { Sink = empt }, 0},
	{"문자열 go", func() { Sink = str }, 1},
	{"struct{}", func() { Sink = none }, 0},
	{"[2]int", func() { Sink = arr }, 1},
	{"포인터 &x", func() { Sink = &x }, 0},
}
for _, c := range cases {
	if got := testing.AllocsPerRun(100, c.box); got != c.want {
		t.Errorf("%s: 할당 = %v, 원하는 값 %v", c.name, got, c.want)
	}
}
```

값은 패키지 변수에서 읽어 상수로 접히지 않게 했고, `go test` 가 통과합니다. `int16` 은 `convT16` 이 256 미만만 `staticuint64s` 로 보내서 200 과 300 이 갈립니다. `bool` 은 컴파일러가 한 바이트 값으로 처리합니다.

</details>

### 연습 2 — typed nil 까지 잡는 IsNil

`IsNil(v any) bool` 이 nil 인터페이스와 nil 을 담은 포인터·맵·슬라이스·함수·채널에서 참을 돌려주게 만듭니다.

<details>
<summary>풀이</summary>

```go
func IsNil(v any) bool {
	if v == nil {
		return true // 타입 칸까지 빈 진짜 nil
	}
	rv := reflect.ValueOf(v)
	switch rv.Kind() {
	case reflect.Pointer, reflect.Map, reflect.Slice, reflect.Func, reflect.Chan, reflect.Interface, reflect.UnsafePointer:
		return rv.IsNil() // 타입 칸은 찼고 데이터가 nil
	}
	return false // int·구조체는 nil 이 될 수 없다
}
```

표 주도 테스트 9건(nil 인터페이스, nil 포인터·맵·슬라이스·함수·채널, 0, 구조체, 포인터)이 통과합니다. `reflect.Pointer` 는 Go 1.18 부터 있는 이름입니다. 이런 함수는 경계에서 확인할 때만 쓰고, 근본 처방은 실패가 없을 때 리터럴 `nil` 을 돌려주는 것입니다.

</details>

## 정리

인터페이스 값은 타입(또는 itab)과 데이터 주소 두 칸입니다. nil 비교와 단언은 앞 칸을 보는 일이라 싸고, 비용은 뒤 칸을 채우려 힙 사본을 만들 때 납니다. 다음에 인터페이스 쪽 코드가 느리면 호출보다 박싱을 먼저 의심하고 `-benchmem` 으로 할당부터 봅니다.

[로드맵의 15편](https://blog.doksam.com/blog/go-course-00-roadmap/)에서는 값이 스택에 남을지 힙으로 갈지를 정하는 이스케이프 분석을 `-gcflags=-m` 으로 읽습니다.
