# Go 강의 16편 — GC 는 힙이 목표에 닿으면 돌고, 그 목표를 GOGC 와 GOMEMLIMIT 가 정한다

> Go 의 GC 는 언제 돌고, 무엇을 조절하면 메모리와 CPU 를 맞바꿀 수 있습니까?

GC 는 힙이 목표 크기에 닿으면 돕니다. 목표는 지난 GC 뒤 살아 남은 힙에 `GOGC` 퍼센트만큼 더한 크기이고, 기본값 100 이면 두 배입니다. `GOMEMLIMIT` 를 주면 목표가 그 천장 아래로 끌려 내려옵니다. `GOGC` 를 올리면 메모리를 더 쓰고 GC 를 덜 돌리며, 내리면 반대입니다. 이 편은 살아 있는 힙 27MB 위로 256바이트 객체 200만 개를 만들고 버리는 프로그램 하나로 이 관계를 숫자로 봅니다.

> [그림] 배달 로봇이 바닥에 상자를 쌓고, 청소 로봇이 흰·회·검 양동이로 상자에 표시를 한다. 벽에는 GOGC 다이얼, 천장 가까이에 GOMEMLIMIT 점선이 있다

상자 더미가 힙이고 꼬리표 색이 GC 의 삼색 표시입니다. 다이얼은 다음 청소까지 쌓을 양을, 천장선은 넘지 말 높이를 정합니다.

힙에 무엇이 올라가는지는 로드맵의 [15편(이스케이프 분석)](https://blog.doksam.com/blog/go-course-00-roadmap/)이 다룹니다. 출력은 2026-11-18 에 Apple M4 Pro(14코어), macOS, Go 1.26.2 로 돌렸습니다. 다른 작업과 함께 돈 상태라 시간은 비율로 읽습니다. MB 는 출력 그대로 2^20 바이트입니다.

## 실험 프로그램은 살아 있는 힙 위로 짧게 사는 객체를 흘려보낸다

`node` 는 248바이트이고, 벤치마크로 재면 한 번에 256바이트(`256 B/op`)가 잡힙니다. 10만 개를 붙잡아 두고 200만 개를 만들고 버립니다. 아래는 예제 코드에서 통계 부분을 줄인 것입니다.

```go
type node struct {
	next *node
	buf  [240]byte
}

var (
	live []*node
	sink *node
)

func main() {
	// … 플래그 처리 생략. 기본값은 live 100,000개, churn 2,000,000개
	live = make([]*node, *liveN)
	for i := range live {
		live[i] = new(node)
	}
	for i := range *churn {
		n := new(node)
		sink = n
		if i%100 == 0 { // 100개에 하나는 살아 있는 집합의 한 칸을 바꾼다
			live[(i/100)%len(live)] = n
		}
		// … 1만 개마다 runtime/metrics 로 힙 크기를 읽어 최대값을 남긴다
	}
	// … GC 횟수, 힙 최대, 걸린 시간, GC CPU 비율, 멈춤 합계를 찍는다
}
```

```text
$ ./gcload
GC 19회, 힙 최대 53MB, 마지막 목표 55MB, 걸린 시간 121ms, GC CPU 2.5%, 멈춤 합계 472µs
```

## gctrace 한 줄에 GC 한 번이 다 들어 있다

`GODEBUG=gctrace=1` 을 주면 GC 마다 표준 에러에 한 줄이 나옵니다.

```text
$ GODEBUG=gctrace=1 ./gcload
…
gc 10 @0.044s 2%: 0.009+0.82+0.010 ms clock, 0.13+0.024/1.9/0.73+0.14 ms cpu, 52->54->27 MB, 55 MB goal, 0 MB stacks, 0 MB globals, 14 P
…
```

`go doc runtime` 의 gctrace 절이 형식을 `gc # @#s #%: #+#+# ms clock, #+#/#/#+# ms cpu, #->#-># MB, # MB goal, # MB stacks, #MB globals, # P` 로 적고, 형식은 바뀔 수 있다고 밝힙니다. 위 줄을 그 설명대로 읽으면 이렇습니다.

| 필드 | 값 | 뜻 |
|---|---|---|
| `gc 10 @0.044s` | 10번째, 시작 후 0.044초 | GC 번호와 시각 |
| `2%` | 2% | 시작 뒤 GC 에 쓴 시간 비율 |
| `0.009+0.82+0.010 ms clock` | 벽시계 시간 | 멈춤(sweep termination) + 동시 표시 + 멈춤(mark termination) |
| `0.13+0.024/1.9/0.73+0.14 ms cpu` | CPU 시간 | 표시 단계를 assist / background / idle 로 나눴다 |
| `52->54->27 MB` | 시작·끝·생존 | GC 시작 때 힙, 끝날 때 힙, 살아 있는 힙 |
| `55 MB goal` | 55MB | 다음 GC 의 목표 |
| `14 P` | 14 | 쓴 프로세서 수 |

세계를 멈추는(STW) 구간은 앞뒤 두 칸뿐이라 이 GC 의 멈춤은 0.019ms 입니다. 표시는 0.82ms 동안 프로그램과 함께 돌았습니다. 살아 있는 힙 27MB 의 두 배 남짓인 55MB 가 다음 목표입니다.

## GOGC 는 다음 GC 까지 더 쌓아 둘 비율이다

`go doc runtime/debug.SetGCPercent` 에 따르면 직전 GC 뒤 살아 남은 데이터 대비 새 할당의 비율이 이 값에 닿으면 GC 가 돕니다. `runtime/mgc.go` 주석의 예는 "GOGC=100 에서 4M 를 쓰면 8M 에서 다시 GC"입니다(Go 1.26.2 소스 기준). 같은 프로그램을 `GOGC` 만 바꿔 세 번씩 돌린 가운데값입니다.

| 설정 | GC 횟수 | 힙 최대 | 목표 | 걸린 시간 | GC CPU | 멈춤 합계 |
|---|---:|---:|---:|---:|---:|---:|
| `GOGC=50` | 39 | 40MB | 41MB | 142ms | 4.1% | 874µs |
| `GOGC=100` (기본) | 19 | 53MB | 55MB | 121ms | 2.5% | 472µs |
| `GOGC=200` | 10 | 82MB | 84MB | 114ms | 1.5% | 346µs |
| `GOGC=off` | 0 | 511MB | 없음 | 59ms | 0.0% | 0s |

목표는 살아 있는 힙 27MB 의 1.5배, 2배, 3배로 움직였습니다. `GOGC` 를 두 배로 올릴 때마다 GC 횟수는 절반 가까이 줄고 힙은 그만큼 커졌습니다. `GOGC=off` 는 GC 없이 59ms 에 끝났지만 힙이 511MB 까지 커졌습니다. 기본값과의 시간 차이 62ms 는 이 부하에서 GC 에 든 비용으로 읽힙니다. `GOGC=off` 의 목표 칸에는 런타임이 매우 큰 값(8532210231509MB)을 찍었습니다.

## GOMEMLIMIT 는 힙이 아니라 Go 가 쥔 메모리 전체의 천장이다

`debug.SetMemoryLimit`(환경변수 `GOMEMLIMIT`)는 Go 1.19 에 들어왔습니다(`api/go1.19.txt`). 문서는 이 한도가 부드러운(soft) 한도이고 `GOGC=off` 여도 지켜지며, 기준이 `MemStats.Sys - MemStats.HeapReleased` 라고 적습니다. 실행 파일 자체나 C 코드가 쓴 메모리는 들어가지 않습니다.

| 설정 | GC 횟수 | 힙 최대 | 걸린 시간 | GC CPU |
|---|---:|---:|---:|---:|
| `GOGC=off GOMEMLIMIT=128MiB` | 6 | 111MB | 137ms | 1.8% |
| `GOGC=off GOMEMLIMIT=64MiB` | 27 | 53MB | 192ms | 4.4% |
| `GOGC=100 GOMEMLIMIT=40MiB` | 154 | 31MB | 225ms | 10.5% |
| `GOGC=off GOMEMLIMIT=20MiB` | 366 | 30MB | 637ms | 9.6% |

`GOGC=off` 에 한도만 주면 GC 는 천장 근처에서만 돕니다. 힙이 511MB 까지 커지던 프로그램이 128MiB 한도에서는 힙 111MB, GC 6번으로 끝났습니다. 64MiB 한도에서 힙이 53MB 에 머문 것은 한도가 힙 말고도 런타임이 쥔 메모리 전체를 세기 때문입니다.

한도를 살아 있는 힙(27MB)보다 낮게 주면 GC 가 366번 돌고 시간이 다섯 배가 됐습니다. 그래도 프로그램은 끝났습니다. 문서도 너무 낮은 한도에서 GC 가 거의 쉬지 않고 돌 수 있지만 프로그램은 진행할 수 있다고 적습니다. `runtime/mgclimit.go` 는 이런 악순환을 막는 GC CPU 제한기를 두고 그 한계를 50% 로 설명합니다. 한도는 컨테이너 메모리 제한보다 여유를 두고 낮게 잡습니다. Go 가 세지 않는 메모리가 그 차이를 씁니다.

## GC 는 프로그램과 함께 표시하고, 쓰기 장벽으로 따라잡는다

`runtime/mgc.go` 머리 주석에 따르면 Go 의 GC 는 동시(concurrent) 표시·청소(mark and sweep) GC 입니다. 쓰기 장벽(write barrier)을 씁니다. 세대 구분(generational)도, 객체 옮기기(compacting)도 하지 않습니다. 순서는 이렇습니다(Go 1.26.2 소스 기준).

1. 세계를 멈추고 청소를 마무리한 뒤 쓰기 장벽을 켠다.
2. 세계를 다시 돌린다. 스택·전역에서 닿는 객체를 회색으로 칠하고, 회색을 하나씩 검사해 검은색으로 바꾼다. 검사하며 만난 포인터는 회색이 된다.
3. 회색이 바닥나면 다시 세계를 멈추고 표시를 끝낸다.
4. 끝까지 흰색으로 남은 객체를 프로그램과 함께 치운다.

표시하는 동안에도 프로그램은 포인터를 바꿉니다. 쓰기 장벽은 덮어쓴 포인터와 새 포인터를 함께 회색으로 칠해 놓치는 객체가 없게 합니다. 표시 도중 새로 만든 객체는 바로 검은색이 됩니다. 할당이 빠르면 할당하는 고루틴이 표시를 거듭니다(assist). gctrace 의 cpu 칸에서 `/` 로 나뉜 세 숫자의 첫째가 그 시간입니다.

이 툴체인(1.26.2)에서는 Green Tea 표시 방식이 기본입니다. `internal/buildcfg/exp.go` 의 기본 설정에 `GreenTeaGC: true` 가 있습니다. `runtime/mgcmark_greenteagc.go` 주석에 따르면 같은 메모리 구역(span)의 객체를 모았다가 한꺼번에 검사해 메모리 접근을 몰아줍니다. `GOEXPERIMENT=nogreenteagc` 로 다시 빌드해 일곱 번씩 번갈아 돌렸습니다.

| 빌드 | GC 횟수 | GC CPU (가운데값) | 걸린 시간 (가운데값) |
|---|---:|---:|---:|
| 기본(Green Tea) | 19 | 2.7% | 130ms |
| `GOEXPERIMENT=nogreenteagc` | 20~21 | 3.4% | 128ms |

GC 에 쓴 CPU 비율은 일곱 쌍 모두 기본 빌드가 낮았습니다. 걸린 시간은 차이가 없었습니다. 객체 크기와 포인터 모양이 다른 프로그램에서는 결과가 달라질 수 있습니다.

## 할당을 줄이는 것이 가장 확실한 조절이다

GC 일은 할당한 양에 비례합니다. 같은 알림 글을 버퍼에 쓰는 세 방식을 `-benchmem` 으로 쟀습니다(`-count 5` 가운데값).

| 방식 | ns/op | B/op | allocs/op |
|---|---:|---:|---:|
| 부를 때마다 새 `bytes.Buffer` | 90.40 | 448 | 2 |
| 크기를 먼저 잡은 새 버퍼(`Grow`) | 77.44 | 384 | 1 |
| `sync.Pool` 에서 빌린 버퍼 | 27.18 | 0 | 0 |

`sync.Pool` 은 캐시가 아닙니다. `sync/pool.go` 문서는 넣어 둔 값이 알림 없이 언제든 사라질 수 있다고 적습니다. GC 가 시작할 때 `poolCleanup` 이 주 캐시를 희생(victim) 캐시로 옮기고 그 전 희생 캐시를 버립니다. 값 하나를 넣고 GC 를 0~2번 돌린 뒤 꺼냈습니다.

```text
$ GOMAXPROCS=1 go run ./cmd/poolgc
넣은 뒤 GC 0번 → New 호출 누계 0
넣은 뒤 GC 1번 → New 호출 누계 0
넣은 뒤 GC 2번 → New 호출 누계 1
```

P 가 하나면 GC 한 번은 버티고 두 번이면 사라집니다. `GOMAXPROCS` 를 기본(14)으로 두면 GC 한 번 뒤에도 `New` 가 불렸습니다. 값은 P 마다 하나씩 있는 전용 칸(private)에 들어갑니다. `getSlow` 소스를 보면 희생 캐시에서는 지금 P 의 전용 칸과 모든 P 의 공유 칸만 봅니다. GC 뒤 고루틴이 다른 P 에서 돌면 전에 넣은 칸을 찾지 못합니다. 그래서 풀은 다시 만들 수 있는 임시 버퍼에만 씁니다.

## 한 단계 더 — 페이서, 통계 읽기, 정리 함수

**언제 시작할지는 페이서(pacer)가 정합니다.** `runtime/mgcpacer.go` 는 배경 표시가 GOMAXPROCS 의 25%(`gcBackgroundUtilization = 0.25`)만 쓰고 assist 없이 목표에 맞춰 끝나도록 시작 시점을 잡는다고 적습니다. 할당이 예상보다 빠르면 assist 가 그 차이를 메웁니다. 할당이 몰리면 요청을 처리하던 고루틴이 표시를 거들게 되니, 그 순간의 지연이 늘 수 있습니다.

**통계는 `runtime/metrics` 로 읽습니다.** `runtime.ReadMemStats` 는 소스에서 `stopTheWorld(stwReadMemStats)` 를 부릅니다. Go 1.16 에 들어온 `runtime/metrics.Read` 와 견주면 이렇습니다.

| 방법 | ns/op (가운데값) |
|---|---:|
| `runtime.ReadMemStats` | 24,563 |
| `metrics.Read` (지표 2개) | 48.22 |

차이는 약 500배이고, 실험 프로그램도 GC 횟수와 목표를 `metrics.Read` 로 읽었습니다.

**자원 정리는 `runtime.AddCleanup` 으로 겁니다.** Go 1.24 에 들어왔고(`api/go1.24.txt`), `go doc runtime.SetFinalizer` 는 새 코드라면 덜 틀리기 쉬운 `AddCleanup` 을 고려하라고 적습니다. 정리 함수는 객체가 아니라 자원(fd)만 받습니다. 객체를 잡으면 객체가 영원히 살아 정리되지 않습니다.

```go
type File struct {
	fd   int
	name string
}

func open(fd int) *File {
	f := &File{fd: fd, name: "app.log"}
	runtime.AddCleanup(f, func(fd int) { done <- fd }, fd)
	return f
}
```

```text
$ go run ./cmd/cleanup
열었다: fd 7
정리 함수가 돌았다: fd 7
```

`main` 에서 `f = nil` 뒤 `runtime.GC()` 를 부르고 `done` 을 1초 기다렸습니다. 문서대로 정리 함수는 프로그램이 끝나기 전에 돈다는 보장이 없습니다. 파일 닫기 같은 일은 여전히 `defer f.Close()` 가 맡고, 정리 함수는 닫기를 잊은 경우의 안전망입니다.

| 증상 | 손잡이 | 대가 |
|---|---|---|
| GC CPU 가 높고 메모리는 남는다 | `GOGC` 올리기 | 힙이 커진다 |
| 컨테이너가 메모리 초과로 죽는다 | `GOMEMLIMIT` 를 제한보다 낮게 | 한도 근처에서 GC 가 잦아진다 |
| 같은 임시 버퍼를 계속 만든다 | `sync.Pool`, 크기 미리 잡기 | 코드가 늘고, 풀은 GC 때 비워진다 |
| 할당 자체가 많다 | 이스케이프 줄이기(로드맵 15편) | 설계를 바꿔야 한다 |

## 면접에서 이렇게 묻는다

**Q1. 삼색 표시와 쓰기 장벽을 설명해 보세요.**
흰색은 아직 닿지 않은 객체, 회색은 닿았지만 안을 검사하지 않은 객체, 검은색은 검사가 끝난 객체입니다. 회색이 바닥나면 흰색은 쓰레기입니다. 표시가 프로그램과 함께 돌기 때문에 쓰기 장벽이 포인터 쓰기를 가로채 덮어쓴 값과 새 값을 회색으로 칠합니다. `runtime/mgc.go` 머리 주석의 설명입니다.

**Q2. GOGC 와 GOMEMLIMIT 는 어떻게 다릅니까?**
`GOGC` 는 살아 있는 힙 대비 얼마나 더 쌓을지 정하는 비율이고, `GOMEMLIMIT` 는 Go 가 쥔 메모리 전체의 부드러운 천장입니다. 둘 다 주면 목표는 둘 중 낮은 쪽을 따릅니다(`runtime/mgcpacer.go` 의 `heapGoalInternal`). 실험에서 `GOGC=100` 에 40MiB 한도를 주자 GC 가 19번에서 154번으로 늘었습니다.

**Q3. gctrace 에서 멈춤 시간은 어디에 있습니까?**
`ms clock` 세 숫자 중 첫째와 셋째가 세계를 멈춘 구간이고, 가운데는 프로그램과 함께 돈 표시 시간입니다. 위 예에서는 0.009ms 와 0.010ms 였습니다.

**Q4. sync.Pool 을 캐시로 써도 됩니까?**
안 됩니다. 문서가 값이 알림 없이 사라질 수 있다고 적고, 런타임은 GC 마다 풀을 희생 캐시로 옮겼다가 다음 GC 에 버립니다. 언제든 다시 만들 수 있는 임시 버퍼에만 씁니다.

**Q5. GC 부담을 줄이려면 무엇부터 합니까?**
먼저 할당을 잽니다(`-benchmem`, 힙 프로파일). 그다음 할당 자체를 줄이고, 그래도 남으면 `GOGC`·`GOMEMLIMIT` 로 메모리와 CPU 를 맞바꿉니다. 손잡이부터 돌리면 원인은 그대로 남습니다.

## 연습

### 연습 1 — GOGC=300 의 목표를 먼저 계산하기

살아 있는 힙이 27MB 안팎일 때 `GOGC=300` 의 목표 힙과 GC 횟수를 예상한 뒤 `GODEBUG=gctrace=1` 로 확인합니다.

<details>
<summary>풀이</summary>

목표는 살아 있는 힙의 (1 + 300/100) = 4배이니 110MB 안팎, GC 횟수는 `GOGC=200`(10회)보다 적을 것입니다.

```text
$ GOGC=300 GODEBUG=gctrace=1 ./gcload
…
gc 6 @0.082s 1%: 0.022+1.1+0.016 ms clock, 0.31+0.13/2.0/1.4+0.23 ms cpu, 107->110->28 MB, 112 MB goal, 0 MB stacks, 0 MB globals, 14 P
gc 7 @0.104s 1%: 0.023+1.1+0.022 ms clock, 0.32+0.18/2.0/1.4+0.31 ms cpu, 109->111->27 MB, 113 MB goal, 0 MB stacks, 0 MB globals, 14 P
GC 6회, 힙 최대 108MB, 마지막 목표 112MB, 걸린 시간 115ms, GC CPU 1.1%, 멈춤 합계 329µs
```

목표 112~113MB, GC 6회였습니다(요약 줄은 반복 구간의 GC 만 셉니다). 생존 힙 27~28MB 의 4배에 반올림이 더해진 값입니다.

</details>

### 연습 2 — 정리 함수가 돌지 않는 File

본문 예제에서 `name` 필드를 지우고 `type File struct{ fd int }` 로 바꾸면 결과가 어떻게 바뀌는지 돌려 보고 이유를 찾습니다.

<details>
<summary>풀이</summary>

```text
$ go run .
열었다: fd 7
1초 안에 정리되지 않았다
```

세 번 모두 정리 함수가 돌지 않았습니다. `go doc runtime.AddCleanup` 은 런타임이 공간을 아끼려고 객체 여러 개를 한 할당 칸에 묶을 수 있다고 적습니다. 묶인 칸에 아직 쓰는 객체가 있으면 정리 함수가 영영 안 돌 수 있습니다. 대개 16바이트 이하의 포인터 없는 객체가 그렇게 묶입니다. 8바이트에 포인터가 없는 `File{fd int}` 가 여기에 걸렸고, 문자열 필드를 더하자 정리 함수가 돌았습니다.

</details>

## 정리

GC 는 힙이 목표에 닿으면 돌고, 목표는 살아 있는 힙 × (1 + GOGC/100) 이 기본이며 `GOMEMLIMIT` 가 그 위를 막습니다. 이 실험에서 `GOGC` 를 두 배로 올릴 때마다 GC 횟수는 절반 가까이 줄고 힙은 그만큼 늘었습니다. 손잡이를 돌리기 전에 할당부터 줄이는 편이 대가가 적습니다.

다음은 로드맵의 [17편(고루틴 스케줄러 G·M·P)](https://blog.doksam.com/blog/go-course-00-roadmap/)입니다. GC 의 배경 표시 작업자도 `go gcBgMarkWorker(…)` 로 띄운 고루틴입니다. 고루틴이 스레드 위에 어떻게 올라가는지를 봅니다.
