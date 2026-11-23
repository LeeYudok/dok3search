# Go 강의 20편 — 제네릭은 상자 포장을 없애 주지만 메서드 호출을 빠르게 하지는 않는다

> 인터페이스로 되는 일에 제네릭을 쓰면 무엇이 좋아지고, 무엇을 치릅니까?

제네릭은 값을 `any` 상자에 담지 않고 타입 그대로 다룹니다. 타입 단언이 사라지고, 정수 1,000개를 담는 실험에서 할당이 1,000번에서 0번으로 줄었습니다. 대신 Go 는 타입마다 코드를 찍지 않습니다. 메모리 모양(GC shape)이 같은 타입끼리 함수 본문 하나를 나눠 씁니다. 그 본문에서 제약의 메서드를 부르면 사전(dictionary)을 거쳐 간접 호출하므로 인라인되지 않습니다. 이 실험에서는 인터페이스 호출보다도 느렸습니다. 그래서 자료를 담고 옮기는 코드에는 제네릭을, 동작을 갈아 끼우는 코드에는 인터페이스를 씁니다.

![배달 로봇 옆의 Sum[T] 틀 기계가 상자를 찍어 내고, 오른쪽 선반에서 int 와 MyInt, *A 와 *B 가 같은 모양 칸에 함께 놓여 있다](/images/blog/go-course-20.svg)

선반 칸의 이름은 실제 실행 파일에서 뽑은 심볼 이름입니다. 타입 다섯 개로 `Sum` 을 불렀는데 함수 본문은 세 개만 생겼습니다.

인터페이스 값의 구조는 로드맵의 [14편](https://blog.doksam.com/blog/go-course-00-roadmap/), 인터페이스 기초는 [3편](https://blog.doksam.com/blog/go-course-03-interfaces-and-errors/)에 있습니다. 출력은 2026-11-18 에 Apple M4 Pro, Go 1.26.2 로 돌렸습니다. 벤치마크는 다른 작업과 함께 돈 상태라 비율로 읽습니다.

## 제약은 받을 타입의 집합을 정하고 ~ 는 이름 붙은 타입까지 넓힌다

```go
type Number interface {
	~int | ~int64 | ~float64
}

func Sum[T Number](xs []T) T {
	var total T
	for _, x := range xs {
		total += x
	}
	return total
}

func Max[T cmp.Ordered](xs []T) (m T, ok bool) {
	for i, x := range xs {
		if i == 0 || x > m {
			m = x
		}
	}
	return m, len(xs) > 0
}
```

`[T Number]` 가 타입 매개변수이고, 제약 `Number` 는 인터페이스로 쓴 타입 집합입니다. `cmp.Ordered` 는 `<` 를 쓸 수 있는 타입 집합으로 Go 1.21 에 들어왔습니다. `~int64` 는 밑바탕(underlying) 타입이 `int64` 인 모든 타입입니다. `~` 를 빼고 `type Won int64` 를 넘기면 거절됩니다.

```text
./main.go:20:17: Won does not satisfy Number (possibly missing ~ for int64 in Number)
```

`time.Duration` 처럼 이름 붙은 숫자 타입이 흔하니, 숫자 제약에는 보통 `~` 를 붙입니다.

## 타입 인자는 인자에서 추론하고, 인자에 없으면 직접 적는다

```go
func Zero[T any]() T {
	var z T
	return z
}

fmt.Println(Map([]int{1, 2}, func(n int) string { return fmt.Sprint(n * 10) }))
fmt.Println(Zero())
```

`Map` 은 인자의 타입에서 `T=int`, `U=string` 을 추론합니다. `Zero()` 는 추론할 근거가 없어서 빌드가 멈춥니다.

```text
./main.go:20:18: in call to Zero, cannot infer T (declared at ./main.go:5:11)
```

`Zero[int]()` 처럼 적으면 됩니다. 빈 집합을 만드는 `NewSet[string]()` 도 같은 이유로 타입을 적습니다.

## comparable 은 == 를 허락할 뿐 패닉이 없다고 약속하지 않는다

```go
func Index[T comparable](xs []T, v T) int {
	for i, x := range xs {
		if x == v {
			return i
		}
	}
	return -1
}

xs := []any{1, "go", []int{1}}
fmt.Println(Index(xs, any("go")))
fmt.Println(Index(xs, any([]int{1})))
```

```text
1
panic: runtime error: comparing uncomparable type []int
```

`any` 는 `comparable` 을 만족하지만, 안에 든 값이 슬라이스면 비교하는 순간 패닉이 납니다. 명세(`doc/go_spec.html`)는 인터페이스가 `comparable` 을 "구현하지는 않지만 만족한다"고 적고, 이 예외에 [Go 1.20] 을 붙입니다. 실제로 `go.mod` 의 `go` 줄을 1.19 로 내리면 빌드가 거절됩니다.

```text
./main.go:16:19: any to satisfy comparable requires go1.20 or later (-lang was set to go1.19; check go.mod)
```

`map[T]struct{}` 로 만든 `Set[T comparable]` 에 `[]int` 를 담은 `any` 를 넣어도 `hash of unhashable type []int` 패닉이 납니다. 제약이 통과해도 인터페이스 타입 인자는 런타임에 다시 봐야 합니다.

## 표준 라이브러리의 제네릭 함수가 반복 코드를 지운다

제 한국어 글 검사기는 외부 의존성 없이 표준 라이브러리의 제네릭 함수만 씁니다.

| 함수 | 도입 | 검사기에서 하는 일 |
|---|---|---|
| `slices.Contains` | 1.21 | 산문 모드가 검사할 절 이름인지 본다 |
| `slices.SortStableFunc` | 1.21 | 검사 결과를 줄 번호로 안정 정렬한다 |
| `slices.Concat` | 1.22 | 기본 규칙과 프로필 규칙을 잇는다 |
| `cmp.Or` | 1.22 | 규칙 수준이 비었으면 "오류"를 쓴다 |
| `maps.Keys` | 1.23 | (예제) 키 반복자를 돌려준다 |

도입 버전은 `$(go env GOROOT)/api/go1.N.txt` 에서 찾았습니다. Go 1.23 의 `maps.Keys` 는 슬라이스가 아니라 반복자 `iter.Seq[K]` 를 돌려주므로, 정렬된 키는 `slices.Sorted(maps.Keys(m))` 로 얻습니다.

## 반복자는 yield 가 false 를 돌려주면 멈춘다

Go 1.23 부터 `func(yield func(T) bool)` 모양의 함수를 `range` 로 돕니다. `iter.Seq[T]` 가 이 모양의 이름입니다.

```go
func Count(n int) func(func(int) bool) {
	return func(yield func(int) bool) {
		for i := 0; i < n; i++ {
			if !yield(i) {
				fmt.Println("멈춤 신호를 받았다")
				return
			}
		}
	}
}

for v := range Count(10) {
	if v == 2 {
		break
	}
	fmt.Println(v)
}
```

```text
0
1
멈춤 신호를 받았다
```

`break` 를 만나면 `yield` 가 `false` 를 돌려주고, 반복자는 그것을 보고 돌아와야 합니다. `go` 줄을 1.22 로 내리면 `cannot range over Count(10) … requires go1.23 or later` 로 거절됩니다. 반복자를 감싸는 `Filter` 를 0~99 를 내는 원본에 걸고 짝수 세 개에서 멈췄습니다. 원본은 5번만 값을 냈습니다. 필요한 만큼만 당겨 오는 것이 반복자의 장점입니다.

## Go 는 타입마다가 아니라 모양마다 코드를 만든다

더하는 함수를 인자로 받는 `Sum[T any]` 를 타입 다섯 개로 부르고 실행 파일의 심볼을 봤습니다. 인라인되어 사라지지 않게 `//go:noinline` 을 붙였습니다.

```go
type MyInt int
type A struct{ x int }
type B struct{ s string }

//go:noinline
func Sum[T any](xs []T, add func(T, T) T) T {
	var total T
	for _, x := range xs {
		total = add(total, x)
	}
	return total
}

Sum([]int{1, 2}, addInt)
Sum([]MyInt{3, 4}, addMyInt)
Sum([]float64{0.5}, addFloat)
Sum([]*A{{1}}, lastA)
Sum([]*B{{"b"}}, lastB)
```

```text
$ go tool nm shapes | grep main.Sum | grep -v .func
10009f730 T main.Sum[go.shape.*uint8]
10009f7d0 T main.Sum[go.shape.float64]
10009f860 T main.Sum[go.shape.int]
$ go tool nm shapes | grep 'main\..*dict'
100177f00 R main..dict.Sum[*main.A]
100177f20 R main..dict.Sum[*main.B]
100177ee0 R main..dict.Sum[float64]
100177ea0 R main..dict.Sum[int]
100177ec0 R main..dict.Sum[main.MyInt]
```

함수 본문(`T`)은 셋, 사전(`R`, 읽기 전용 자료)은 다섯입니다. `int` 와 `MyInt` 는 밑바탕 타입이 같아 `go.shape.int` 본문 하나를 쓰고, `*A` 와 `*B` 는 `go.shape.*uint8` 하나를 씁니다. 타입마다 다른 정보는 사전에 담겨 호출할 때 함께 넘어갑니다.

컴파일러 소스 `cmd/compile/internal/noder/reader.go` 의 `shapify` 주석이 규칙을 적습니다(Go 1.26.2 소스 기준). 기본 인터페이스(메서드만 있는 제약, `any` 포함)로 제한된 포인터는 원소 타입이 코드에 영향을 주지 않으니 `*byte` 를 모양으로 씁니다. 그 밖에는 밑바탕 타입이 모양입니다. 타입마다 본문을 찍으면 실행 파일이 커지고, 모두 상자에 담으면 할당이 생깁니다. 모양별 본문은 그 사이를 고른 방식입니다.

## 제약의 메서드를 부르면 인라인이 막힌다

같은 일을 구체 타입, 제네릭, 인터페이스로 짜서 원소 1,000개로 쟀습니다. 정수 더하기의 인터페이스 판은 `Add(int) int` 메서드를 가진 값의 슬라이스를 돕니다. 메서드 호출은 아래 `Total` 과, 같은 몸통을 `[]Rect` 와 `[]Shape` 로 쓴 함수를 견줬습니다.

```go
type Shape interface{ Area() float64 }

type Rect struct{ W, H float64 }

func (r Rect) Area() float64 { return r.W * r.H }

func Total[T Shape](ss []T) float64 {
	t := 0.0
	for _, s := range ss {
		t += s.Area()
	}
	return t
}
```

`-count 5` 의 중앙값이고 할당은 모두 0이었습니다.

| 일 | 구체 타입 | 제네릭 | 인터페이스 |
|---|---:|---:|---:|
| 정수 더하기 (`Sum`) | 270.2 ns | 276.8 ns (1.02배) | 1,101 ns (4.07배) |
| 메서드 호출 (`Area()`) | 803.3 ns | 3,116 ns (3.88배) | 2,821 ns (3.51배) |

연산자만 쓰는 `Sum` 은 구체 타입과 같습니다. 제약의 메서드 `Area()` 를 부르는 `Total[T Shape]` 는 인터페이스보다도 약 10% 느렸습니다. `-gcflags=-m` 이 이유를 보여 줍니다. 구체 타입 쪽에는 `inlining call to Rect.Area` 가 찍히지만, `Total[go.shape.struct { W float64; H float64 }]` 본문 안의 `s.Area()` 에는 그 줄이 없습니다. 같은 본문을 여러 타입이 나눠 쓰니 어느 `Area` 인지 사전에서 찾아 간접 호출합니다.

## 제네릭으로 안 되는 것도 알아 둔다

| 하려는 것 | 결과 |
|---|---|
| 메서드에 타입 매개변수 달기 | `syntax error: method must have no type parameters` |
| 타입 매개변수 값으로 타입 스위치 | `cannot use type switch on type parameter value v …` |
| 특정 타입만 따로 구현(특수화) | 문법이 없다 |

타입 스위치는 `switch any(v).(type)` 처럼 `any` 로 바꾸면 됩니다. 메서드에서 새 타입이 필요하면 `func MapBox[T, U any](b Box[T], f func(T) U) Box[U]` 처럼 일반 함수로 뺍니다. 제네릭 타입 별칭(`type Set[T comparable] = map[T]struct{}`)은 Go 1.26.2 컴파일러에서 `go` 줄이 1.23 이상일 때 받았고, 1.22 이하면 `generic type alias requires go1.23 or later` 로 거절했습니다.

## 한 단계 더 — 상자 포장 비용과 고르는 기준

제네릭의 이득이 가장 크게 보이는 곳은 메서드 호출이 아니라 상자 포장입니다. 1000~1999 정수 1,000개를 `[]any` 와 `[]int` 에 담았습니다.

```go
func KeepAny(dst []any, v int) []any { return append(dst, v) }
func Keep[T any](dst []T, v T) []T   { return append(dst, v) }
```

```text
BenchmarkKeepAny-14        	  181462	      6621 ns/op	    8000 B/op	    1000 allocs/op
BenchmarkKeepGeneric-14    	  715299	      1773 ns/op	       0 B/op	       0 allocs/op
```

다섯 번 가운데 중앙값 줄입니다. `-gcflags=-m` 은 `KeepAny` 에 `v escapes to heap` 을 찍습니다. `int` 를 `any` 에 담으려고 값을 힙에 복사하기 때문입니다. 제네릭 쪽은 `[]int` 그대로라 할당이 없고 약 3.7배 빨랐습니다. 할당이 왜 힙으로 가는지는 로드맵의 [15편](https://blog.doksam.com/blog/go-course-00-roadmap/)에서 다룹니다.

고르는 기준은 이렇게 정리합니다.

| 상황 | 고르는 것 | 이유 |
|---|---|---|
| 담고 옮기고 고르는 코드(컨테이너, `Sum`, `Uniq`) | 제네릭 | 상자 포장과 타입 단언이 없다 |
| 동작을 갈아 끼우는 코드(저장소, 핸들러) | 인터페이스 | 실행 중에 구현을 바꾼다. 제네릭도 간접 호출이다 |
| 타입마다 처리가 전혀 다르다 | 타입 스위치나 함수 따로 | 특수화가 없다 |
| 타입 몇 개에 대한 반복 코드 | 그냥 두 번 쓴다 | 읽기 쉬움이 먼저다 |

## 면접에서 이렇게 묻는다

**Q1. 제약에서 `~int` 와 `int` 는 무엇이 다릅니까?**
`int` 는 `int` 하나, `~int` 는 밑바탕 타입이 `int` 인 모든 타입입니다. `type Won int64` 를 `~` 없는 제약에 넘기면 "possibly missing ~" 로 거절된다는 실제 에러를 예로 듭니다.

**Q2. `comparable` 제약을 통과했으면 `==` 가 안전합니까?**
아닙니다. Go 1.20 부터 `any` 같은 인터페이스도 `comparable` 을 만족해서, 안의 동적 타입이 슬라이스면 런타임에 `comparing uncomparable type` 패닉이 납니다.

**Q3. Go 제네릭은 어떻게 구현됩니까?**
GC shape 별로 함수 본문을 하나씩 만들고 타입별 정보는 사전으로 넘깁니다. 밑바탕 타입이 같은 타입과, `any` 같은 제약의 포인터 타입들이 본문을 나눠 씁니다. 그래서 연산자만 쓰는 코드는 구체 타입만큼 빠르고, 제약의 메서드 호출은 간접 호출이라 인라인되지 않습니다.

**Q4. 인터페이스와 제네릭은 어떻게 고릅니까?**
값을 담고 옮기면 제네릭, 동작을 실행 중에 바꾸면 인터페이스입니다. `[]any` 에 정수 1,000개를 담으면 할당이 1,000번 생기고 `[]T` 는 0번이라는 측정을 근거로 댑니다.

**Q5. range-over-func 반복자에서 `yield` 의 반환값은 왜 확인합니까?**
소비자가 `break` 하면 `false` 가 옵니다. 무시하고 계속 부르면 `range function continued iteration after function for loop body returned false` 패닉이 납니다. 확인하면 원본이 필요한 만큼만 값을 냅니다.

## 연습

### 연습 1 — 순서를 지키는 Uniq

`Uniq[T comparable](xs []T) []T` 를 만듭니다. 처음 나온 순서를 지키고, 빈 입력에는 nil 이 아닌 빈 슬라이스를 돌려줍니다. `Uniq([]any{1, []int{1}})` 은 어떻게 되는지도 봅니다.

<details>
<summary>풀이</summary>

```go
func Uniq[T comparable](xs []T) []T {
	seen := make(map[T]struct{}, len(xs))
	out := make([]T, 0, len(xs))
	for _, x := range xs {
		if _, ok := seen[x]; ok {
			continue
		}
		seen[x] = struct{}{}
		out = append(out, x)
	}
	return out
}
```

`Uniq([]string{"b", "a", "b", "c", "a"})` 는 `[b a c]` 입니다. `[]any{1, []int{1}}` 은 맵 키로 쓰는 순간 `panic: runtime error: hash of unhashable type []int` 가 납니다. 테스트는 `recover` 로 이 메시지를 확인합니다.

</details>

### 연습 2 — 원본을 멈추는 Take

`Take[T any](seq iter.Seq[T], n int) iter.Seq[T]` 를 만듭니다. 끝없이 자연수를 내는 원본에 걸었을 때 원본이 정확히 3번만 값을 내야 합니다.

<details>
<summary>풀이</summary>

```go
func Take[T any](seq iter.Seq[T], n int) iter.Seq[T] {
	return func(yield func(T) bool) {
		if n <= 0 {
			return
		}
		i := 0
		for v := range seq {
			if !yield(v) {
				return
			}
			i++
			if i == n {
				return
			}
		}
	}
}
```

`slices.Collect(Take(naturals, 3))` 는 `[0 1 2]` 이고, 원본이 값을 낸 횟수는 3이었습니다. n 번째 값을 낸 뒤 바로 돌아오는 것이 핵심입니다. 다음 값을 당긴 뒤에 세면 원본이 한 번 더 일합니다.

</details>

## 정리

제네릭은 값을 상자에 담지 않아 할당과 타입 단언을 없앱니다. Go 는 모양이 같은 타입끼리 함수 본문을 나눠 쓰고 사전으로 차이를 넘깁니다. 그래서 연산자만 쓰는 코드는 구체 타입만큼 빠르고, 제약의 메서드 호출은 인터페이스처럼 간접 호출입니다.

로드맵의 [21편](https://blog.doksam.com/blog/go-course-00-roadmap/)에서는 이번에 쓴 벤치마크를 제대로 재는 법을 다룹니다. pprof 와 실행 추적으로 느린 곳을 찾고, 고친 효과를 숫자로 보입니다.
