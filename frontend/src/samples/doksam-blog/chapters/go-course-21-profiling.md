# Go 강의 21편 — 느린 곳은 프로파일로 찾고, 고친 효과는 같은 벤치마크로 증명한다

> 느리다는 말을 숫자로 바꾸고, 고친 효과를 어떻게 증명합니까?

벤치마크로 기준 숫자(ns/op, B/op, allocs/op)를 여러 번 재고, 프로파일로 시간이 어디로 가는지 찾고, 고친 뒤 같은 벤치마크를 같은 조건에서 다시 재서 가운데값을 견줍니다. 이 편은 한국어 글 검사기를 줄인 코드에 병목 하나를 심어 두고 이 순서로 찾아 7배 빠르게 만듭니다. 도구의 숫자도 확인합니다. 이 맥의 CPU 프로파일은 맨 위에 엉뚱한 함수를 올려놓았습니다.

> [그림] 초시계 아래를 달리는 배달 로봇, CPU 상자에 청진기를 댄 의사 로봇, 병목 막대에 동그라미를 친 그래프

초시계가 벤치마크, 청진기가 프로파일러입니다. 막대 하나가 유난히 높으면 거기부터 고칩니다.

선수 편은 로드맵의 [15·16·17편](https://blog.doksam.com/blog/go-course-00-roadmap/)입니다. 출력은 2026-11-18 에 Apple M4 Pro(14코어), macOS, Go 1.26.2 로 다른 작업과 함께 돌린 것이라 시간은 비율로 읽습니다.

## 결과를 버린 벤치마크는 아무것도 재지 않는다

부작용 없는 함수 `Mix` 를 세 방식으로 쟀습니다. 옛 방식은 `for i := 0; i < b.N; i++` 이고, Go 1.24 부터는 `for b.Loop()` 를 씁니다.

```go
func BenchmarkMixOldDiscard(b *testing.B) {
	for i := 0; i < b.N; i++ {
		Mix(uint64(i)) // 결과를 버린다
	}
}

func BenchmarkMixLoop(b *testing.B) {
	var i uint64
	for b.Loop() {
		Mix(i)
		i++
	}
}
```

| 벤치마크(`-count 5` 가운데값) | ns/op |
|---|---:|
| 옛 방식, 결과 버림 | 0.2289 |
| 옛 방식, 빈 루프 | 0.2359 |
| 옛 방식, 전역 변수에 담음 | 0.5527 |
| `b.Loop`, 결과 버림 | 1.687 |
| `b.Loop`, 빈 루프 | 1.630 |

결과를 버린 옛 방식은 빈 루프와 시간이 같았습니다. `-gcflags=-m` 이 `inlining call to Mix` 를 찍었으니, 인라인된 계산이 쓰이지 않아 지워진 것입니다. `go doc testing.B.Loop` 는 루프 안 호출의 인자와 결과를 `runtime.KeepAlive` 로 살려 둔다고 적습니다. 대신 `b.Loop` 는 반복마다 약 1.6ns 의 틀 비용이 붙어서, 나노초 단위 함수는 빈 루프와 함께 잽니다.

## 기준 숫자는 -benchmem 과 -count 로 만든다

검사기의 처음 판은 문장마다 정규식을 다시 컴파일합니다. 일부러 심은 병목입니다.

```go
func CheckV1(text string) []Finding {
	var out []Finding
	for i, s := range sentences(text) {
		for _, r := range Rules {
			re := regexp.MustCompile(r.Pattern)
			for _, m := range re.FindAllString(s, -1) {
				out = append(out, Finding{i, m, r.Hint})
			}
		}
	}
	return out
}
```

```text
$ go test -run X -bench 'CheckV1$' -benchmem -count 10 ./lint
BenchmarkCheckV1-14    	     516	   2192193 ns/op	 5436445 B/op	   57977 allocs/op
…
```

200문장 글 한 편에 2.2ms, 5.4MB, 할당 57,977번입니다. 한 번 잰 값은 흔들리므로 열 번 재서 가운데값을 씁니다. 비교 도구 benchstat 은 이 맥에 깔려 있지 않아서, 가운데값과 변화율만 찍는 80줄이 안 되는 `benchmed` 를 예제에 두었습니다.

## CPU 프로파일은 맨 위부터 믿지 않는다

```text
$ go test -run X -bench 'CheckV1$' -benchtime 6s -cpuprofile cpu-v1.out -memprofile mem-v1.out ./lint
$ go tool pprof -top -nodecount=3 cpu-v1.out
Duration: 7.63s, Total samples = 8940ms (117.18%)
      flat  flat%   sum%        cum   cum%
    3790ms 42.39% 42.39%     3790ms 42.39%  runtime.kevent
     790ms  8.84% 51.23%      790ms  8.84%  runtime.pthread_cond_wait
     720ms  8.05% 59.28%      720ms  8.05%  runtime.madvise
```

맨 위가 네트워크 대기용 시스템 콜 `kevent` 입니다. CPU 만 쓰는 벤치마크인데 말이 안 됩니다. 런타임 소스(Go 1.26.2 기준)의 `runtime/signal_unix.go` 는 프로세스 전체 타이머(`setitimer`)로 신호를 만들고, 스레드 단위 타이머를 함께 쓰는 플랫폼으로 리눅스를 적습니다. 맥(`runtime/os_darwin.go`)은 프로세스 타이머만 쓰고, 이 맥에서는 표본이 시스템 콜 중인 스레드로 쏠렸습니다. `GOGC=off` 로 돌리자 `madvise` 가 95% 를 가져갔습니다. 리눅스에서는 재 보지 못했습니다.

그래서 내 함수 아래만 걸러 봅니다. `flat` 은 그 함수 자신이 쓴 시간이고, `cum` 은 그 함수가 부른 함수까지 합친 시간입니다.

```text
$ go tool pprof -top -cum -focus=lint.CheckV1 cpu-v1.out
      flat  flat%   sum%        cum   cum%
     0.01s  0.11%  0.11%      1.84s 20.58%  example.com/profiling/lint.CheckV1
         0     0%  0.11%      1.54s 17.23%  regexp.MustCompile
         0     0%  0.45%      0.86s  9.62%  regexp/syntax.Parse (inline)
$ go tool pprof -list 'lint.CheckV1' cpu-v1.out
         .      1.54s     54:			re := regexp.MustCompile(r.Pattern)
         .      270ms     55:			for _, m := range re.FindAllString(s, -1) {
```

출력은 줄였습니다. `CheckV1` 1.84초 가운데 1.54초(84%)가 54행의 컴파일입니다.

## 힙 프로파일은 신호 없이 할당을 센다

힙 프로파일은 할당한 바이트 수로 표본을 뽑습니다(`go doc runtime.MemProfileRate`: 평균 512KiB 마다 하나). 그래서 위 문제가 없습니다.

```text
$ go tool pprof -top -sample_index=alloc_space mem-v1.out
Showing nodes accounting for 15470.51MB, 95.39% of 16218.82MB total
      flat  flat%   sum%        cum   cum%
 6848.13MB 42.22% 42.22%  6848.13MB 42.22%  regexp/syntax.(*compiler).inst (inline)
 5053.04MB 31.16% 73.38%  5053.04MB 31.16%  regexp/syntax.(*parser).newRegexp (inline)
 1183.17MB  7.30% 80.67% 15758.01MB 97.16%  regexp.compile
```

`alloc_space` 는 쌓인 할당 총량이라 GC 부담을 봅니다. 같은 파일을 `inuse_space` 로 보면 2.01MB 뿐이고, 누수처럼 지금 붙잡은 메모리는 이쪽으로 봅니다. 할당 총량의 97% 가 정규식 컴파일 아래여서 CPU 와 힙이 같은 곳을 가리킵니다.

## 고친 뒤에는 같은 벤치마크로 다시 잰다

규칙은 처음 한 번만 컴파일합니다. `sync.OnceValue` 는 Go 1.21 부터 있습니다.

```go
var compiled = sync.OnceValue(func() []*regexp.Regexp {
	res := make([]*regexp.Regexp, len(Rules))
	for i, r := range Rules {
		res[i] = regexp.MustCompile(r.Pattern)
	}
	return res
})
```

`-count 10` 으로 재고 `benchmed` 로 가운데값을 뽑았습니다.

| 열 번 잰 값 | 처음 판 | 고친 판 | 비율 |
|---|---:|---:|---:|
| ns/op 가운데값 | 2,105,717 | 301,687 | 7.0배 |
| B/op | 약 5,438,000 | 약 67,900 | 80배 |
| allocs/op | 57,977 | 367 | 158배 |

두 판의 결과가 같은지는 `TestCheckMatchesV1` 이 확인합니다. 고친 판을 다시 프로파일하면 `Check` 가 전체 표본의 82.78% 이고, 그 안의 `regexp.(*Regexp).FindAllString` 이 79.77% 입니다. 다음 병목은 정규식 실행입니다. 할당이 줄자 `kevent` 쏠림도 10.90% 로 줄었습니다.

## 잠금 경합은 뮤텍스·블록 프로파일이 따로 보여 준다

잠금 하나로 맵을 지키는 카운터를 `b.RunParallel` 로 두드렸습니다.

```text
$ go test -run X -bench Counter -benchtime 2s -mutexprofile mutex.out -blockprofile block.out ./contend
$ go tool pprof -top mutex.out
    17.58s 64.95% 64.95%     20.85s 77.03%  sync.(*Mutex).Unlock (inline)
$ go tool pprof -top block.out
    16.81s 76.84% 76.84%     16.81s 76.84%  sync.(*Mutex).Lock (inline)
```

출력은 맨 윗줄만 남겼습니다. `go doc runtime/pprof.Profile` 에 따르면 뮤텍스 프로파일은 잠금을 쥔 쪽의 `Unlock` 에, 블록 프로파일은 기다린 쪽이 막힌 곳에 시간을 적습니다. 고칠 곳은 뮤텍스 프로파일이 가리킵니다.

## 운영 서버의 pprof 는 루프백 전용 리스너에 연다

`net/http/pprof` 는 import 하기만 해도 `init` 에서 `http.DefaultServeMux` 에 `/debug/pprof/` 를 등록합니다(소스 `net/http/pprof/pprof.go`). 공개 API 는 자기 `ServeMux` 를 쓰고, pprof 는 `-debug` 플래그 주소(기본 `127.0.0.1:6060`)에 따로 엽니다.

```go
debug := http.NewServeMux()
debug.HandleFunc("/debug/pprof/", pprof.Index)
debug.HandleFunc("/debug/pprof/profile", pprof.Profile)
// … symbol, trace 핸들러도 같은 식으로 붙인다
go func() {
	srv := &http.Server{Addr: *debugAddr, Handler: debug, ReadHeaderTimeout: 5 * time.Second}
	log.Println(srv.ListenAndServe())
}()
```

```text
$ ./pprof-expose.sh
공개 주소 /debug/pprof/: 404
디버그 주소 /debug/pprof/: 200
-default-mux 로 띄운 공개 주소 /debug/pprof/: 200
공개 주소 /debug/pprof/cmdline: ./server -addr 127.0.0.1:18531 -debug 127.0.0.1:18532 -default-mux
```

`DefaultServeMux` 로 공개하면 남이 CPU 프로파일을 뜨고 실행 인자까지 읽습니다. 부하를 주는 동안 디버그 주소에서 3초를 떴습니다.

```text
$ go tool pprof -top 'http://127.0.0.1:18522/debug/pprof/profile?seconds=3'
Fetching profile over HTTP from http://127.0.0.1:18522/debug/pprof/profile?seconds=3
Saved profile in …/pprof.server.samples.cpu.001.pb.gz
```

받은 파일은 `PPROF_TMPDIR`(기본 `$HOME/pprof`)에 남습니다. 이 서버 프로파일도 맨 위는 `runtime.usleep` 이었고, `-focus=CheckV1` 로 거르니 7.82% 였습니다.

## 실행 추적은 기다린 시간을 보여 준다

CPU 프로파일은 도는 시간만 셉니다. 차례를 기다린 시간은 실행 추적(trace)에 있고, 웹 화면 대신 `go tool trace -pprof` 로 표를 뽑았습니다.

```text
$ go test -run CheckAllMatchesSerial -trace trace.out ./lint
$ go tool trace -pprof=sched trace.out > sched.pprof
$ go tool pprof -top sched.pprof
Type: delay
   41.64ms 30.82% 30.82%    41.82ms 30.95%  runtime.chansend1
```

출력은 줄였습니다. `go tool trace -h` 는 `sched` 를 스케줄러 지연(scheduler latency) 프로파일로 설명합니다. 실행할 준비가 됐는데 돌지 못한 시간이고, 이 실행에서는 `runtime.chansend1` 스택이 가장 컸습니다.

## PGO 의 효과는 이 예제에서 잡음 안이었다

프로파일 기반 최적화(PGO)는 잰 CPU 프로파일을 컴파일러에 줍니다. `-pgo` 기본값 `auto` 는 main 패키지 폴더의 `default.pgo` 를 씁니다(`go help build`). 여기서는 고친 판의 프로파일을 직접 줬습니다.

```text
$ go build -pgo=off -gcflags='all=-m' ./lint 2>&1 | grep -c "inlining call"
14640
$ go build -pgo=check.pgo -gcflags='all=-m' ./lint 2>&1 | grep -c "inlining call"
14755
```

`regexp` 의 `(*Regexp).doExecute`·`allMatches` 같은 뜨거운 호출이 새로 인라인됐습니다. 그러나 벤치마크를 열 번씩 세 쌍 재니 -6.0%, -1.4%, +0.7% 였습니다. 같은 기계의 다른 부하가 만든 흔들림보다 크다고 말할 수 없습니다.

## 한 단계 더 — 프로파일러가 재는 방식과 측정 함정

`runtime/pprof/pprof.go` 의 `StartCPUProfile` 주석은 표본 주기를 100Hz 로 고정한 이유로, 운영체제가 약 500Hz 넘게 신호를 주지 못하고 표본마다 스택을 읽는 비용이 든다는 점을 듭니다. CPU 하나가 3초 내내 돌아도 표본은 약 300개라, 짧은 프로파일의 몇 퍼센트 차이는 읽지 않습니다.

| 함정 | 이 편에서 본 것 | 대처 |
|---|---|---|
| 결과를 버린 루프 | 빈 루프와 같은 0.23ns | `b.Loop`, 빈 루프와 함께 잰다 |
| 프로파일러 자체의 부담 | 뮤텍스 프로파일을 켜자 121.5 → 202.3ns/op(한 번 잰 값) | 시간 비교와 프로파일을 따로 돌린다 |
| 맥 CPU 프로파일 쏠림 | 맨 위가 `kevent`·`madvise` | `-focus`·`-cum`, 힙 프로파일로 교차 확인 |

## 면접에서 이렇게 묻는다

**Q1. 벤치마크가 0.2ns/op 처럼 비현실적으로 빠르게 나왔습니다. 무엇을 의심합니까?**
결과를 버린 계산이 인라인된 뒤 지워진 경우입니다. 빈 루프와 같은 시간이면 확정입니다. `b.Loop` 를 쓰거나 결과를 바깥 변수에 담고, `-gcflags=-m` 으로 인라인 여부를 봅니다.

**Q2. pprof 의 flat 과 cum 은 어떻게 다릅니까?**
flat 은 그 함수 자신의 시간, cum 은 부른 함수까지 합친 시간입니다. 이 편의 `CheckV1` 은 flat 0.01초, cum 1.84초였습니다. cum 이 큰 함수를 `-list` 로 열어 시간이 몰린 줄을 찾습니다.

**Q3. 힙 프로파일의 alloc_space 와 inuse_space 는 언제 봅니까?**
alloc_space 는 누적 할당량이라 GC 부담과 할당이 몰린 곳을 찾습니다. inuse_space 는 지금 살아 있는 메모리라 누수를 찾습니다.

**Q4. 운영 서버에 pprof 를 켜도 됩니까?**
루프백이나 내부망 전용 리스너에만 엽니다. `net/http/pprof` 는 import 만으로 `DefaultServeMux` 에 붙어서, 공개 서버가 그 mux 를 쓰면 남이 프로파일과 실행 인자를 가져갑니다. 뮤텍스 프로파일은 이 편의 벤치마크를 121.5 에서 202.3ns/op 로 늦췄으니 상시로 켜 두지 않습니다.

## 연습

### 연습 1 — 잠금을 나눠 경합 줄이기

`Counter` 를 키 해시로 64칸으로 나눠 칸마다 잠그도록 바꾸고, 벤치마크와 뮤텍스 프로파일로 효과를 증명합니다.

<details>
<summary>풀이</summary>

```go
func (s *Sharded) Add(key string) {
	p := &s.parts[maphash.String(s.seed, key)%shards]
	p.mu.Lock()
	p.m[key]++
	p.mu.Unlock()
}
```

`-count 5` 가운데값은 잠금 하나가 121.5ns/op, 64칸이 41.33ns/op 였습니다. 뮤텍스 프로파일의 경합 합계는 잠금 하나가 연산 약 1,202만 번에 28.92초, 64칸이 약 5,749만 번에 4.54초로, 연산 하나당 약 30분의 1입니다. 여러 고루틴이 더한 합계가 맞는지는 `-race` 로 돌린 `TestShardedCountsEveryAdd` 가 봅니다.

</details>

### 연습 2 — 느린 한 번만 실행 추적으로 남기기

처리가 10ms 를 넘긴 순간의 직전 실행 추적만 파일로 남깁니다.

<details>
<summary>풀이</summary>

Go 1.25 의 `runtime/trace.FlightRecorder` 는 최근 기록을 메모리에 돌려 담다가 `WriteTo` 때 씁니다.

```go
fr := trace.NewFlightRecorder(trace.FlightRecorderConfig{MinAge: 2 * time.Second, MaxBytes: 4 << 20})
if err := fr.Start(); err != nil {
	fmt.Fprintln(os.Stderr, err)
	os.Exit(1)
}
defer fr.Stop()
// … 처리마다 시간을 재다가 10ms 를 넘기면
n, err := fr.WriteTo(f)
```

```text
$ go run ./cmd/flight
150번째 처리 25ms, 직전 기록 165929바이트를 slow.trace 에 썼다
```

긴 글 한 편만 기록됐고(세 번 돌려 22~25ms·164~166KB), 파일은 `go tool trace` 로 읽혔습니다.

</details>

## 정리

느리다는 말은 벤치마크로 숫자가 되고, 프로파일로 위치가 되고, 같은 조건의 재측정으로 증명이 됩니다. 이 편의 병목은 문장마다 하던 정규식 컴파일이었고, 한 번만 하자 7.0배 빨라졌습니다. 다음에 재실 때는 도구의 숫자도 다른 도구로 교차 확인합니다.

3단계는 여기까지입니다. 로드맵의 [22편](https://blog.doksam.com/blog/go-course-00-roadmap/)부터는 면접·설계 단계로, 코드 리뷰에서 걸리는 Go 관용구를 다룹니다.
