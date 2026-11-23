# Go 강의 13편 — 슬라이스는 배열을 나눠 보는 창이고, 맵은 여덟 칸 묶음의 해시 표다

> 같은 배열을 나눠 쓰는 슬라이스는 언제 서로를 덮어쓰고, 맵은 왜 순서도 없고 동시에 쓰면 죽습니까?

슬라이스는 포인터·길이·용량 세 칸짜리 머리입니다. 용량이 남은 채로 `append` 하면 새 배열을 만들지 않고 같은 배열에 씁니다. 그래서 한 배열에서 나온 두 슬라이스가 같은 칸을 덮어씁니다. 맵은 Go 1.26.2 에서 여덟 칸을 한 묶음(그룹)으로 쓰는 해시 표입니다. 런타임이 순회 순서를 일부러 섞고, 동시 쓰기를 발견하면 `recover` 로도 못 잡는 치명 오류로 멈춥니다. 이 편은 이것을 하나씩 실험으로 보입니다.

> [그림] 칸 네 개짜리 선반 위에 a 와 b 두 괄호가 같은 칸을 덮고, 셋째 칸의 slice 가 지워지고 map 이 들어 있다. 배달 로봇이 그 칸으로 상자를 밀어 넣고, 오른쪽에는 여덟 칸 서랍 두 개짜리 맵 서랍장이 있다

선반이 밑 배열이고 두 괄호가 같은 배열을 보는 슬라이스입니다. 오른쪽 서랍 하나가 맵의 그룹이고, 서랍 위 줄이 칸마다 쓰는 제어 바이트입니다.

`len`·`cap`·`append` 의 기본은 [2편](https://blog.doksam.com/blog/go-course-02-values-and-structs/)에서 봤고, 이 편은 그 안을 엽니다. 출력은 2026-11-18 에 Apple M4 Pro, Go 1.26.2 로 돌렸습니다. 시간 수치는 다른 작업과 함께 돈 상태의 값입니다.

## 슬라이스는 배열을 가리키는 세 칸짜리 머리다

`unsafe.SliceData` 는 슬라이스가 가리키는 배열 주소를, `unsafe.StringData` 는 문자열의 바이트 주소를 돌려줍니다.

```go
s := make([]int64, 4, 8)
t := s[1:3]
fmt.Println(unsafe.SliceData(t) == &s[1], len(t), cap(t))
diff := uintptr(unsafe.Pointer(unsafe.SliceData(t))) - uintptr(unsafe.Pointer(unsafe.SliceData(s)))
fmt.Println("t 의 시작 - s 의 시작 =", diff, "바이트")

str := "안녕하세요"
sub := str[3:9]
d := uintptr(unsafe.Pointer(unsafe.StringData(sub))) - uintptr(unsafe.Pointer(unsafe.StringData(str)))
fmt.Println(sub, "시작 차이 =", d, "바이트")
```

```text
true 2 7
t 의 시작 - s 의 시작 = 8 바이트
녕하 시작 차이 = 3 바이트
```

`t` 는 `s` 의 배열을 8바이트(`int64` 하나) 뒤에서 보는 창입니다. 문자열 조각도 바이트를 복사하지 않고 3바이트(한글 한 글자) 뒤를 가리킵니다. 두 함수는 Go 1.20 기능이라 `go.mod` 를 `go 1.19` 로 낮추면 `unsafe.SliceData requires go1.20 or later` 로 거절됩니다. 머리 크기(슬라이스 24바이트, 문자열 16바이트)는 2편에서 쟀습니다.

## 용량이 남으면 append 는 남의 칸에 쓴다

기본 태그 목록에 태그를 하나 붙여 새 목록을 만드는 함수입니다. 실제 코드에서 흔히 보는 모양입니다.

```go
func withTag(base []string, tag string) []string {
	return append(base, tag)
}

base := make([]string, 0, 4)
base = append(base, "go", "course")
a := withTag(base, "slice")
b := withTag(base, "map")
fmt.Println(a, b)
```

```text
[go course map] [go course map]
```

`a` 가 `slice` 를 잃었습니다. `base` 는 길이 2, 용량 4 라서 두 `append` 가 모두 같은 배열의 셋째 칸에 썼고, 나중 값이 이겼습니다. 호출부는 `base` 를 고친 적이 없으니 원인 찾기가 어렵습니다.

고치는 방법은 둘입니다. 하나는 세 번째 인덱스로 용량을 길이에 맞추는 것입니다(`base[:len(base):len(base)]`, 명세의 "Full slice expressions"). 남는 칸이 없으니 `append` 가 새 배열로 복사합니다. 다른 하나는 `slices.Clone`(Go 1.21)으로 먼저 복사하는 것입니다.

```text
[go course slice] [go course map]
```

남에게서 받은 슬라이스에 `append` 할 때는 그 배열을 함께 쓰는 사람이 있다고 가정합니다.

## 용량은 256 까지 두 배, 그 뒤로는 천천히 늘고 크기 등급에 맞춰 올림된다

`append` 로 하나씩 늘리며 용량이 바뀌는 순간을 찍었습니다. 아래는 컴파일러의 스택 버퍼 최적화(다음 절)를 끈 결과라서 런타임 규칙만 보입니다.

```text
$ go run -gcflags='-d=variablemakethreshold=0' ./experiments/growth
1 2 4 8 16 32 64 128 256 512 848 1280 1792 2560 3408 
1 2 4 8 16 32 64 128 256 512 853 1365 
```

첫 줄은 `int64`, 둘째 줄은 24바이트 구조체입니다.

규칙은 런타임 `runtime/slice.go` 의 `nextslicecap` 에 있습니다(Go 1.26.2 소스 기준).

```go
const threshold = 256
if oldCap < threshold {
	return doublecap
}
for {
	// Transition from growing 2x for small slices
	// to growing 1.25x for large slices.
	newcap += (newcap + 3*threshold) >> 2
	…
```

512 다음은 512 + (512 + 768) ÷ 4 = 832 인데 출력은 848 입니다. `growslice` 가 바이트 수를 메모리 할당기의 크기 등급(size class)으로 올려서입니다. `int64` 는 6,656바이트(832 × 8)가 6,784바이트 등급이 되어 848칸이 됩니다. 24바이트짜리는 19,968바이트가 20,480바이트 등급이 되어 853칸입니다. 등급 표는 `internal/runtime/gc/sizeclasses.go` 에 있고, 이 공식은 구현 세부라 버전마다 바뀔 수 있습니다.

## 처음 몇 칸의 용량은 컴파일러가 정한다

같은 프로그램을 옵션 없이 돌리면 앞부분이 다릅니다.

```text
$ go run ./experiments/growth
4 8 16 32 64 128 256 512 848 1280 1792 2560 3408 
1 2 4 8 16 32 64 128 256 512 853 1365 
$ go run ./experiments/growth2
1 2 3 4 8 16 32 64 128 256 512 848 
```

`growth` 는 슬라이스를 함수 안에서만 쓰고, `growth2` 는 끝에 전역 변수로 내보냅니다.

`go build -gcflags=-m` 은 첫 프로그램에 `append does not escape` 를, 둘째에 `append escapes to heap` 을 찍었습니다. 컴파일러 소스 `cmd/compile/internal/ssagen/ssa.go` 의 주석이 두 전략을 적습니다. 결과가 밖으로 안 나가면 첫 `append` 에서 스택에 작은 배열을 통째로 잡습니다. 나중에 힙으로 옮길 슬라이스는 크기 등급을 한 단계씩 밟습니다. 스택 버퍼의 기본 한도는 32바이트입니다(`cmd/compile/internal/base/flag.go` 의 `VariableMakeThreshold = 32`). `int64` 네 칸이 정확히 32바이트입니다.

정확한 `cap` 값에 기대는 코드는 컴파일러가 바뀌면 깨집니다. 크기를 안다면 `make([]T, 0, n)` 으로 잡는 것이 답입니다.

## 작은 조각 하나가 큰 배열 전체를 붙잡는다

64MiB 버퍼를 만들고 앞 16바이트만 전역 변수에 남긴 뒤 GC 를 돌려 `HeapAlloc` 을 쟀습니다.

```text
시작           0.3 MiB
조각만 보관   64.3 MiB (cap=67108864)
복사해 보관    0.3 MiB (cap=16)
```

조각이 배열의 시작을 가리키는 한 GC 는 64MiB 를 치우지 못합니다. `bytes.Clone`(Go 1.20)으로 16바이트만 복사해 두면 원본이 사라집니다. 큰 응답 본문에서 줄 하나, 키 하나를 오래 들고 있을 때 생기는 메모리 누수입니다.

## 맵은 여덟 칸 그룹으로 된 Swiss Table 이다

Go 1.26.2 의 맵 구현(`internal/runtime/maps/map.go`)은 머리 주석에서 Swiss Table 설계를 따른다고 밝힙니다. 요점은 셋입니다.

- 그룹은 키·값 칸 8개와 8바이트 제어 워드(control word)입니다. 제어 바이트에 칸의 사용 여부와 해시 아래 7비트를 담아 여덟 칸을 한 번에 비교합니다.
- 원소가 8개 이하인 작은 맵은 그룹 하나만 씁니다.
- 그룹 여덟 칸 가운데 평균 7칸까지 채우고(`group.go` 의 `maxAvgGroupLoad = 7`) 넘으면 표를 키웁니다. 표가 1,024칸(`table.go` 의 `maxTableCapacity`)을 넘으면 두 표로 쪼갭니다.

원소 수를 바꿔 가며 `map[int64]int64` 1,000개를 만들고 맵 하나의 힙 바이트를 쟀습니다.

```text
원소    0 개: 맵 하나에 약     48 바이트
원소    1 개: 맵 하나에 약    189 바이트
원소    8 개: 맵 하나에 약    189 바이트
원소    9 개: 맵 하나에 약    373 바이트
원소   14 개: 맵 하나에 약    373 바이트
원소   15 개: 맵 하나에 약    661 바이트
```

1~8개는 그룹 하나라 크기가 같고, 9~14개도 같습니다. 그룹 두 개 16칸에 8칸 중 7칸 비율을 곱하면 14칸이고, 15번째에서 다시 커졌습니다. 소스 주석과 측정이 맞습니다.

순서는 같은 주석이 "Iteration order is unspecified. In the implementation, it is explicitly randomized." 라고 적습니다. 순서가 바뀌는 실험은 2편에 있습니다. 순서가 필요하면 키를 정렬합니다.

## 맵은 키를 지워도 메모리를 돌려주지 않는다

`[64]byte` 값을 100만 개 넣고 지운 뒤 GC 를 돌렸습니다.

```text
100만 개 넣음       160.2 MiB (len=1000000)
delete 로 다 지움   160.2 MiB (len=0)
clear 로 비움       160.1 MiB (len=0)
새 맵으로 바꿈        0.3 MiB (len=0)
```

`delete` 로 다 지워도, Go 1.21 의 `clear` 를 써도 그대로입니다. 맵은 줄어들지 않고, 지운 칸 표시(tombstone)도 표를 키울 때만 정리된다고 소스 주석은 적습니다. 한때 많이 찼다가 줄어든 캐시는 남은 키를 새 맵에 옮겨 담아야 메모리가 돌아옵니다.

## 동시에 쓰면 recover 로도 못 잡는 치명 오류다

잠금 없이 고루틴 둘이 같은 맵에 씁니다. `main` 에는 `recover` 를 걸어 두었습니다.

```go
defer func() { fmt.Println("recover:", recover()) }() // 잡히지 않는다
m := map[int]int{}
var wg sync.WaitGroup
for g := range 2 {
	wg.Go(func() {
		for i := range 1_000_000 {
			m[i] = g
		}
	})
}
wg.Wait()
```

```text
fatal error: concurrent map writes

goroutine 19 [running]:
internal/runtime/maps.fatal({0x1046f3539?, 0x0?})
…
```

세 번 돌려 세 번 모두 멈췄고, 종료 코드는 2, `recover:` 줄은 한 번도 찍히지 않았습니다. 이것은 패닉이 아니라 런타임의 `fatal` 이라 `defer` 가 돌 기회가 없습니다. 맵 소스는 쓰기 중 표시(`m.writing`)를 보고 이 오류를 냅니다. 겹친 순간을 우연히 봤을 때만 멈추는 검사입니다. 고루틴마다 10번만 쓰게 줄였더니 20번 중 13번은 아무 일 없이 끝났고, `-race` 는 `WARNING: DATA RACE` 로 바로 잡았습니다. 공유 맵은 [5편](https://blog.doksam.com/blog/go-course-05-tests-and-concurrency/)의 레이트 리미터처럼 `sync.Mutex` 로 지키고 `-race` 로 확인합니다.

## 문자열 변환은 복사지만 컴파일러가 빼 주는 자리가 있다

문자열은 바꿀 수 없어서 `string(b)` 와 `[]byte(s)` 는 바이트를 복사합니다. 63바이트로 `-benchmem -count 5` 를 돌린 중앙값입니다.

| 코드 | ns/op | B/op | allocs/op |
|---|---:|---:|---:|
| `sink = string(b)` | 11.61 | 64 | 1 |
| `sink = []byte(s)` | 12.99 | 64 | 1 |
| `m[string(b)]` 조회 | 6.156 | 0 | 0 |
| `string(b) == s` 비교 | 1.821 | 0 | 0 |
| `+=` 로 100번 잇기 | 2,248 | 10,736 | 99 |
| `strings.Builder` + `Grow` | 200.9 | 208 | 1 |

맵 조회와 비교는 할당이 0 입니다. 변환한 문자열이 식 밖으로 나가지 않아 컴파일러가 복사를 생략합니다. 반대로 `+=` 는 이을 때마다 새 문자열을 만들어 99번 할당했습니다.

## 한 단계 더 — 미리 잡은 용량과 공유 여부

1,000개를 `append` 로 채워 힙으로 내보내는 벤치마크입니다(5회 중앙값).

| 방식 | ns/op | B/op | allocs/op |
|---|---:|---:|---:|
| `var s []int64` 에서 시작 | 4,023 | 25,208 | 12 |
| `make([]int64, 0, 1000)` | 2,237 | 8,192 | 1 |

0에서 시작하면 늘릴 때마다 새 배열을 잡고 옛 배열을 복사해서 12번 할당하고 바이트도 세 배를 씁니다. 최종 크기를 알면 미리 잡는 쪽이 약 1.8배 빨랐습니다. 마지막으로, 리뷰에서 자주 따지는 "이 연산은 같은 메모리를 쓰는가"를 표로 모읍니다.

| 연산 | 밑 메모리 |
|---|---|
| `t := s[i:j]`, `str[i:j]` | 공유 |
| 슬라이스를 함수에 넘기기 | 머리만 복사, 배열 공유 |
| 용량이 남은 `append` | 공유한 배열에 씀 |
| 용량이 모자란 `append`, `s[i:j:j]` 뒤 `append` | 새 배열로 복사 |
| `slices.Clone`, `bytes.Clone`, `copy` | 복사 |
| `string(b)`, `[]byte(s)` | 복사(맵 조회·비교에서는 생략) |
| 맵을 함수에 넘기기 | 포인터 하나 복사, 같은 맵 |

## 면접에서 이렇게 묻는다

**Q1. 슬라이스를 함수에 넘기면 무엇이 복사됩니까?**
포인터·길이·용량 세 칸짜리 머리만 복사되고 배열은 공유합니다. 원소를 고치면 호출부에도 보입니다. 하지만 `append` 로 늘린 길이는 호출부의 머리에 반영되지 않으니, 늘린 결과는 반환값으로 돌려줍니다.

**Q2. `append` 뒤에 다른 슬라이스의 값이 바뀌는 경우는 언제입니까?**
같은 배열을 보는 슬라이스에 용량이 남아 있을 때입니다. 세 번째 인덱스로 용량을 막거나 `slices.Clone` 으로 복사해 끊습니다.

**Q3. `append` 의 용량은 어떻게 늘어납니까?**
Go 1.26.2 런타임은 256칸까지 두 배로 늘리고, 그 뒤로는 `(newcap + 768) / 4` 씩 더해 약 1.25배로 수렴합니다. 바이트 수는 크기 등급으로 올립니다. 컴파일러가 작은 슬라이스를 스택 버퍼로 시작하기도 하니 정확한 값에 기대지 말고 미리 잡는다고 답합니다.

**Q4. 맵을 동시에 쓰면 어떻게 됩니까?**
런타임이 겹친 쓰기를 보면 `fatal error: concurrent map writes` 로 멈추고, 패닉이 아니라 `recover` 로 잡을 수 없습니다. 발견은 우연에 기대므로 뮤텍스로 지키고 `-race` 로 확인합니다.

**Q5. 맵에서 키를 지우면 메모리가 줄어듭니까?**
줄지 않습니다. 100만 개를 지워도 160MiB 가 그대로였고 `clear` 도 같았습니다. 남은 키를 새 맵으로 옮기면 돌아옵니다.

## 연습

### 연습 1 — 기본 목록을 덮어쓰지 않는 With

위 `withTag` 를 호출부는 그대로 두고 함수만 고칩니다. 용량이 남은 `base` 로 두 번 불러도 두 결과가 각자의 태그를 지키는지 테스트로 확인합니다.

<details>
<summary>풀이</summary>

```go
// With 는 base 뒤에 tag 를 붙인 목록을 돌려준다. base 의 밑 배열에는 쓰지 않는다.
func With(base []string, tag string) []string {
	return append(base[:len(base):len(base)], tag)
}

func TestWithDoesNotShareBackingArray(t *testing.T) {
	base := make([]string, 0, 4) // 용량이 남은 기본 목록
	base = append(base, "go", "course")
	a := With(base, "slice")
	b := With(base, "map")
	if !slices.Equal(a, []string{"go", "course", "slice"}) {
		t.Errorf("a = %v, 원하는 값 [go course slice]", a)
	}
	if !slices.Equal(b, []string{"go", "course", "map"}) {
		t.Errorf("b = %v, 원하는 값 [go course map]", b)
	}
}
```

`go test` 는 통과합니다. 함수를 원래의 `append(base, tag)` 로 되돌리면 `a = [go course map], 원하는 값 [go course slice]` 로 실패해서, 이 테스트가 버그를 잡는다는 것도 확인했습니다.

</details>

### 연습 2 — 첫 줄만 남기기

큰 입력에서 첫 줄을 떼어 오래 들고 있는 함수 `First(data []byte) []byte` 를 씁니다. 결과가 입력의 배열을 붙잡지 않아야 합니다.

<details>
<summary>풀이</summary>

```go
func First(data []byte) []byte {
	line, _, _ := bytes.Cut(data, []byte("\n"))
	return bytes.Clone(line)
}
```

테스트는 원본의 첫 바이트를 바꾼 뒤에도 결과가 `"제목"` 인지, 결과의 용량이 원본보다 작은지 봅니다. 둘 다 통과합니다. `bytes.Clone` 을 빼면 결과가 원본 배열의 창이 되어 첫 검사에서 실패합니다.

</details>

## 정리

슬라이스는 배열을 가리키는 세 칸짜리 머리입니다. 그래서 용량이 남은 `append` 는 같은 배열을 보는 다른 슬라이스의 칸을 덮어씁니다. 맵은 여덟 칸 그룹의 해시 표이고, 순서를 일부러 섞고, 지워도 줄지 않고, 동시 쓰기에는 복구할 수 없는 오류로 멈춥니다. 남의 슬라이스에 `append` 하기 전에 위의 공유 여부 표를 다시 봅니다.

다음 편은 [로드맵](https://blog.doksam.com/blog/go-course-00-roadmap/)의 14편, 인터페이스 값의 내부 표현입니다. 2편과 3편에서 미뤄 둔 `any` 의 16바이트와 typed nil 을 타입·데이터 두 칸으로 풉니다.
