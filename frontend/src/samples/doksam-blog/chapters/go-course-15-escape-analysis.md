# Go 강의 15편 — 주소가 함수 밖으로 나가는 값만 힙으로 간다

> 같은 `new` 와 `&T{}` 인데 어떤 것은 공짜이고 어떤 것은 가비지 컬렉터에 짐이 됩니까?

값을 어디에 둘지는 문법이 아니라 컴파일러의 이스케이프 분석(escape analysis, 값이 함수 밖으로 나가는지 따지는 분석)이 정합니다. 주소가 함수가 끝난 뒤에도 쓰일 수 있으면 힙에, 아니면 스택에 둡니다. 스택은 함수가 끝나면 통째로 비워지고, 힙은 할당 비용에 더해 가비지 컬렉터(GC)가 치울 일을 남깁니다. 이 편에서는 판정이 갈리는 함수들을 `go build -gcflags=-m` 으로 빌드하고 벤치마크로 차이를 잽니다.

> [그림] 스택 서랍에는 상자가 남고, 꼬리표를 단 상자만 힙 창고로 가며 창고 앞에서 GC 청소 로봇이 비질을 한다

서랍은 함수가 끝날 때 통째로 비워집니다. 꼬리표가 붙은 상자만 창고로 가고 청소 로봇이 따로 치웁니다.

출력은 2026-11-18 에 Apple M4 Pro(macOS)·Go 1.26.2 로 돌린 결과입니다. 벤치마크는 다른 작업과 함께 돈 `-count 5` 중앙값입니다.

## 판정은 문법이 아니라 주소가 어디까지 가는지로 갈린다

```go
type Point struct{ X, Y int }

//go:noinline
func NewPointPtr(x, y int) *Point { p := Point{x, y}; return &p }

//go:noinline
func NewPointVal(x, y int) Point { return Point{x, y} }

func SumLocal() int { p := &Point{1, 2}; return p.X + p.Y }

func Show(x int) { fmt.Println(x) }
```

```text
$ go build -gcflags=-m ./esc 2>&1 | grep point.go | grep -v inlin
esc/point.go:9:37: moved to heap: p
esc/point.go:14:28: &Point{...} does not escape
esc/point.go:16:31: ... argument does not escape
esc/point.go:16:32: x escapes to heap
```

`:=` 로 만든 `p` 는 주소가 반환값으로 나가서 힙으로 옮겨졌고, `&Point{...}` 는 함수 안에서만 쓰여 스택에 남았습니다.

컴파일러 소스(`cmd/compile/internal/escape/escape.go` 머리 주석, Go 1.26.2 소스 기준)는 지켜야 할 불변식 두 가지를 적습니다. 스택 객체를 가리키는 포인터는 힙에 저장되지 않고, 그 객체보다 오래 살지 않아야 합니다. 그래서 변수와 할당을 꼭짓점("location"), 대입을 간선으로 하는 데이터 흐름 그래프를 만듭니다. 주소가 힙이나 더 오래 사는 곳으로 흐르는 길을 찾습니다.

## -gcflags=-m 은 판정과 이유를 찍는다

자주 보는 문구는 이렇습니다. 이 편의 출력은 그 절에 해당하는 줄만 옮겼습니다.

| 문구 | 뜻 |
|---|---|
| `moved to heap: p` | 변수 `p` 자체가 힙에 놓인다 |
| `&T{...} escapes to heap` | 이 표현식이 만든 값이 힙에 놓인다 |
| `does not escape` | 스택에 둔다 |
| `leaking param: p` | 인자 `p`(포인터 값)가 힙이나 전역으로 흘러간다 |
| `leaking param content: ps` | `ps` 자체가 아니라 `ps` 가 가리키는 것이 흘러간다 |
| `leaking param: ps to result ~r0 level=1` | `ps` 를 한 번 따라간 값이 반환값으로 나간다 |

```go
var last *Point

func Keep(p *Point)            { last = p }
func First(ps []*Point) *Point { return ps[0] }
func Remember(ps []*Point)     { last = ps[0] }
```

```text
esc/leak.go:5:11: leaking param: p
esc/leak.go:6:12: leaking param: ps to result ~r0 level=1
esc/leak.go:7:15: leaking param content: ps
```

이유가 궁금하면 `-m=2` 를 줍니다. `NewPointPtr` 의 `p` 는 이렇게 설명됩니다.

```text
esc/point.go:9:37: p escapes to heap in NewPointPtr:
esc/point.go:9:37:   flow: ~r0 ← &p:
esc/point.go:9:37:     from &p (address-of) at esc/point.go:9:62
esc/point.go:9:37:     from return &p (return) at esc/point.go:9:55
```

## 인터페이스로 부르면 상대를 몰라 힙으로 보낸다

```go
type Shape interface{ Area() int }
type Rect struct{ W, H int }

func (r *Rect) Area() int { return r.W * r.H }

//go:noinline
func AreaOf(s Shape) int { return s.Area() }

func UseIface() int  { r := &Rect{2, 3}; return AreaOf(r) }
func UseDirect() int { r := &Rect{2, 3}; return r.Area() }
```

```text
esc/shape.go:9:13: leaking param: s
esc/shape.go:11:29: &Rect{...} escapes to heap
esc/shape.go:12:29: &Rect{...} does not escape
```

`s.Area()` 는 실행할 때 정해지는 메서드라 컴파일러가 인자의 행방을 모릅니다. 그래서 `s` 가 힙으로 흐른다고 보고 넘긴 `&Rect` 도 힙에 둡니다. `Show` 의 `x escapes to heap` 도 같은 이유입니다. `-m=2` 는 그 끝을 `fmt.Fprintln(os.Stdout, fmt.a...) (call parameter)` 로 보여 줍니다. 인터페이스에 값을 담는 비용은 [3편](https://blog.doksam.com/blog/go-course-03-interfaces-and-errors/)에서 봤고, 내부 표현은 [로드맵의 14편](https://blog.doksam.com/blog/go-course-00-roadmap/)에서 다룹니다.

## 클로저는 붙잡은 변수를 함께 데려간다

```go
func Counter() func() int { n := 0; return func() int { n++; return n } }
func CountLocal() int     { n := 0; inc := func() { n++ }; inc(); inc(); return n }
```

```text
esc/closure.go:3:29: moved to heap: n
esc/closure.go:3:44: func literal escapes to heap
```

`-m=2` 에는 `Counter capturing by ref: n` 이 찍힙니다. 클로저가 반환되니 붙잡은 `n` 도 힙으로 갔습니다. 그 자리에서 부르는 `CountLocal` 은 클로저가 인라인되어 할당이 없습니다.

## make 는 크기와 담는 것에 따라 갈린다

```go
func VarBuf(n int) byte { buf := make([]byte, n); buf[0] = 1; return buf[0] }
func BigArray() byte    { var a [1 << 20]byte; a[0] = 1; return a[0] }
```

```text
esc/buf.go:3:38: make([]byte, n) does not escape
esc/buf.go:4:31: moved to heap: a
```

`a` 의 이유는 `-m=2` 에서 `from a (too large for stack)` 로 나옵니다. 스택 상한은 명시한 변수가 128KiB, `new`·`&T{}`·`make` 가 64KiB 입니다(`cmd/compile/internal/ir/cfg.go`, Go 1.26.2 소스 기준).

`make([]byte, n)` 은 "탈출하지 않음"으로 찍혔지만 실제 할당은 `n` 에 따라 달랐습니다.

| `n` | ns/op | B/op | allocs/op |
|---:|---:|---:|---:|
| 16 | 1.775 | 0 | 0 |
| 32 | 2.068 | 0 | 0 |
| 33 | 10.22 | 48 | 1 |
| 64 | 10.92 | 64 | 1 |

Go 1.26.2 컴파일러는 실행 중에 크기가 정해지는 `make` 에 32바이트 스택 자리를 둡니다. 넘치면 힙에서 받습니다(`cmd/compile/internal/walk/builtin.go` 와 `base/flag.go`). 33바이트 요청에 48바이트가 잡힌 것은 힙의 크기 등급(`internal/runtime/gc/sizeclasses.go`) 때문입니다.

포인터를 모으는 슬라이스는 원소를 힙으로 보냅니다.

```go
func Collect() int {
	pts := make([]*Point, 0, 4)
	for i := range 4 {
		pts = append(pts, &Point{i, i})
	}
	return pts[3].X
}
```

```text
esc/buf.go:9:21: &Point{...} escapes to heap
```

`-m=2` 는 이유를 `append(pts, &Point{...}) (call parameter)` 로 적습니다. 슬라이스는 스택에 남았지만 원소 넷이 따로 힙에 갔습니다. 값 슬라이스 `[]Point` 로 바꾼 판은 할당이 없었습니다(64B·4회 대 0회).

## 인라인되면 판정을 다시 한다

```go
func newPointInl(x, y int) *Point { p := Point{x, y}; return &p }
func SumViaInl() int              { p := newPointInl(1, 2); return p.X + p.Y }
func SumViaNoinline() int         { p := NewPointPtr(1, 2); return p.X + p.Y }
```

`newPointInl` 자체는 `moved to heap: p` 로 찍히지만, `SumViaInl` 안으로 인라인되면 주소가 그 함수를 벗어나지 않아 할당이 사라집니다.

| 벤치마크 | ns/op | B/op | allocs/op |
|---|---:|---:|---:|
| `NewPointPtr` (인라인 안 됨) | 7.768 | 16 | 1 |
| `NewPointVal` | 2.058 | 0 | 0 |
| `SumViaNoinline` | 7.720 | 16 | 1 |
| `SumViaInl` | 1.762 | 0 | 0 |
| `UseIface` | 25.41 | 16 | 1 |
| `UseDirect` | 1.758 | 0 | 0 |

할당 수는 다시 돌려도 같았고, 시간은 할당이 있는 쪽만 흔들렸습니다(`UseIface` 8.3~25.7ns, 할당 없는 쪽 1.7~2.2ns). 다른 작업과 함께 돈 상태라 할당 경로가 부하를 탄 것으로 봅니다. 비교는 `allocs/op` 부터 봅니다.

`escape.go` 주석에 따르면 함수 경계 너머는 "parameter tags" 라는 요약으로만 전달됩니다. 인라인은 그 경계를 지워 호출한 쪽에서 다시 판단하게 합니다. 포인터를 돌려주는 작은 생성자도 인라인되면 공짜일 수 있으니 잽니다.

## 값 반환과 부른 쪽 버퍼가 할당을 줄인다

`strconv.Itoa(1234567890)` 은 14.81ns·1회 할당, 다시 쓰는 버퍼에 `strconv.AppendInt` 로 붙이면 7.804ns·0회였습니다. `io.Reader` 의 `Read(p []byte)` 가 같은 설계입니다. 부른 쪽이 버퍼를 만들어 넘기니 받는 쪽은 할당하지 않고, 버퍼는 호출마다 다시 쓰입니다. `strconv.Append*` 와 `fmt.Appendf`(Go 1.19)도 같습니다.

모든 함수를 이렇게 바꾸지는 않습니다. 고칠 곳은 프로파일과 `allocs/op` 로 고릅니다([로드맵의 21편](https://blog.doksam.com/blog/go-course-00-roadmap/)).

## 한 단계 더 — 고루틴 스택은 자라면서 옮겨 간다

스택 크기는 고정되지 않습니다. 런타임 소스(Go 1.26.2 기준)를 보면 새 고루틴은 `startingStackSize` 로 시작합니다. 처음 값은 `stackMin = 2048` 에서 나온 2KiB 이고, GC 때마다 평균 스택 크기로 조정됩니다(`runtime/stack.go`). 모자라면 `newstack` 이 두 배(`newsize := oldsize * 2`) 크기의 새 스택을 받고 `copystack` 이 옮깁니다. 이때 스택을 가리키는 포인터를 차이(`adjinfo.delta`)만큼 고칩니다. 상한은 64비트에서 1GB 입니다(`runtime/proc.go` 주석).

지역 변수의 주소를 쥔 채 재귀를 10만 번 했습니다. `-m` 에 `moved to heap: x` 가 없으니 `x` 는 스택에 있습니다.

```text
$ go run .
주소 바뀜=true *p=42
StackInuse 224 KiB → 16608 KiB
```

스택이 옮겨져 `x` 의 주소는 바뀌었고, `p` 로 읽은 값은 그대로였습니다. 런타임이 포인터를 고쳤기 때문입니다. 상한을 `debug.SetMaxStack(1 << 20)` 으로 낮추면 같은 재귀가 이렇게 끝납니다(종료 코드 2).

```text
runtime: goroutine stack exceeds 1048576-byte limit
runtime: sp=… stack=[…]
fatal error: stack overflow
```

할당을 줄일지 정하는 표입니다.

| 힙으로 가는 이유 | 고치는 법 | 고칠 가치 |
|---|---|---|
| 지역 변수 주소를 반환 | 작은 구조체는 값으로 반환 | 호출이 잦으면 크다 |
| 인터페이스·`fmt` 로 넘김 | 뜨거운 경로에서는 구체 타입으로 부름 | 로그 한 줄이면 작다 |
| 클로저가 변수를 붙잡고 반환됨 | 반환하지 않고 그 자리에서 부른다 | 대개 작다 |
| 실행 중 정해지는 큰 `make` | 버퍼를 부른 쪽에서 받아 재사용 | 루프 안이면 크다 |
| 포인터 슬라이스에 `append` | 값 슬라이스로 | 원소가 많으면 크다 |

## 면접에서 이렇게 묻는다

**Q1. 이스케이프 분석은 무엇을 정합니까?**
변수와 할당을 스택에 둘지 힙에 둘지를 컴파일할 때 정합니다. 기준은 주소가 함수보다 오래 살 수 있는 곳으로 흐르는가입니다. `&T{}` 도 스택에 남고 지역 변수도 힙으로 갈 수 있음을 `-gcflags=-m` 으로 보입니다.

**Q2. `leaking param` 과 `leaking param content` 는 어떻게 다릅니까?**
앞은 받은 포인터 값이 힙이나 전역으로 흐르고, 뒤는 그 포인터가 가리키는 대상만 흐른다는 뜻입니다. `to result ~r0` 가 붙으면 반환값으로 나간다는 뜻이라 부른 쪽 판정에 따라 갈립니다.

**Q3. 인터페이스를 거치면 왜 할당이 생깁니까?**
동적 호출의 대상을 컴파일러가 모르면 인자의 행방도 모르니 힙으로 흐른다고 가정합니다. 같은 메서드를 구체 타입으로 직접 부르면 스택에 남습니다. 이 실험에서는 호출마다 16바이트 할당 1회와 0회로 갈렸습니다.

**Q4. 고루틴 스택이 넘치면 어떻게 됩니까?**
작게(처음 2KiB) 시작해 모자랄 때마다 두 배 크기로 새로 받아 복사하고, 스택을 가리키는 포인터를 고칩니다. 상한(64비트 1GB, `debug.SetMaxStack` 으로 조정)을 넘으면 `fatal error: stack overflow` 로 프로세스가 끝납니다.

**Q5. 할당을 줄이는 일은 어떤 순서로 합니까?**
프로파일로 뜨거운 경로를 찾고, `-benchmem` 의 `allocs/op` 로 확인하고, `-gcflags=-m` 으로 이유를 본 뒤 고칩니다. 값 반환, 부른 쪽 버퍼, 값 슬라이스처럼 코드를 덜 해치는 고침부터 넣습니다.

## 연습

### 연습 1 — 판정을 먼저 예측하기

`-gcflags=-m` 을 돌리기 전에 `NewBuffer` 안과 `UseBuffer` 안의 판정을 예측합니다.

```go
type Buffer struct{ data []byte }

func NewBuffer() *Buffer   { return &Buffer{data: make([]byte, 0, 16)} }
func (b *Buffer) Len() int { return len(b.data) }
func UseBuffer() int       { b := NewBuffer(); return b.Len() }
```

<details>
<summary>풀이</summary>

```text
solutions/predict/predict.go:6:37: &Buffer{...} escapes to heap
solutions/predict/predict.go:6:55: make([]byte, 0, 16) escapes to heap
solutions/predict/predict.go:7:7: b does not escape
solutions/predict/predict.go:8:44: &Buffer{...} does not escape
solutions/predict/predict.go:8:44: make([]byte, 0, 16) does not escape
```

`NewBuffer` 를 따로 보면 둘 다 반환값으로 나가 힙입니다. `UseBuffer` 에서는 `NewBuffer` 와 `Len` 이 인라인되어 둘 다 스택에 남았습니다. `testing.AllocsPerRun` 으로 재 보니 `UseBuffer` 는 0회, `NewBuffer` 는 2회였습니다.

</details>

### 연습 2 — 호출마다 생기는 할당 없애기

`Label(id int) string` 은 `fmt.Sprintf("id=%d", id)` 를 돌려줍니다. 같은 글자를 할당 없이 만드는 함수를 쓰고 벤치마크로 확인합니다.

<details>
<summary>풀이</summary>

```go
func AppendLabel(dst []byte, id int) []byte {
	dst = append(dst, "id="...)
	return strconv.AppendInt(dst, int64(id), 10)
}
```

| 벤치마크 | ns/op | B/op | allocs/op |
|---|---:|---:|---:|
| `Label` | 42.10 | 24 | 2 |
| `AppendLabel` (버퍼 재사용) | 6.858 | 0 | 0 |

`Label` 의 2회는 `id` 를 `any` 에 담는 8바이트와 결과 문자열 16바이트입니다. 처음에 `Label(123456)` 처럼 상수로 쟀을 때는 1회였습니다. 인라인 뒤 상수는 읽기 전용 데이터에 두고 그 주소를 쓰기 때문입니다(`cmd/compile/internal/walk/order.go` 의 `addrTemp`). `VarBuf(33)` 처럼 상수를 넘겨도 크기가 정해져 0회였습니다. 벤치마크 입력은 실제처럼 바뀌는 값으로 줍니다.

</details>

## 정리

같은 `&T{}` 라도 주소가 함수 밖으로 나가면 힙, 아니면 스택입니다. 판정은 `-gcflags=-m` 으로 보이고, 인터페이스 호출·반환되는 클로저·큰 `make`·포인터 슬라이스·인라인 여부가 판정을 바꿉니다.

다음은 [로드맵의 16편](https://blog.doksam.com/blog/go-course-00-roadmap/) 가비지 컬렉터입니다. 힙의 값이 치워지는 모습을 `GODEBUG=gctrace=1` 로 보고 `GOGC`·`GOMEMLIMIT` 로 조절합니다.
