# Go 강의 24편 — 라이브 코딩은 질문으로 시작해 테스트와 -race 로 끝낸다

> 45분 라이브 코딩에서 무엇을 먼저 말하고, 어떤 순서로 코드를 쓰고, 무엇으로 끝냅니까?

질문으로 시작해 테스트로 끝냅니다. 처음 5분은 입력·출력·동시 호출 여부를 묻고 함수 모양을 합의합니다. 그다음 가장 단순한 정답과 테스트 하나를 먼저 통과시키고, 남은 시간에 동시성·에러·취소를 더합니다. 마지막은 `go test -race` 결과와 복잡도, 남은 위험을 말하는 것으로 닫습니다. 이 편은 이 순서로 과제 두 개를 풀고, 일부러 틀린 판이 실제로 어떻게 실패하는지 보입니다.

> [그림] 화이트보드 앞의 배달 로봇 옆에 45분 시계가 있고, 선반 끝에서 가장 오래된 상자가 떨어지며, 책상에 질문 카드가 쌓여 있다

그림의 선반이 LRU(Least Recently Used, 가장 오래 안 쓴 것부터 버림) 캐시입니다. 새 상자는 앞에 놓이고, 넘치면 가장 오래 안 쓴 상자가 끝에서 떨어집니다.

출력은 모두 2026-11-18 에 맥(Apple M4 Pro)과 Go 1.26.2 로 돌린 결과이고, 시간 수치는 다른 작업과 함께 돈 상태에서 쟀습니다.

## 45분은 질문 5분, 정답 15분, 테스트와 개선 20분으로 나눈다

아래는 제가 권하는 배분입니다. 정해진 규칙이 아니라 판단입니다.

| 시간 | 할 일 | 말로 할 것 |
|---|---|---|
| 0~5분 | 문제 확인 | 용량이 0 이면? 여러 고루틴이 부르나? 실패는 어떻게 알리나? |
| 5~10분 | 시그니처 합의 | `New(capacity, ttl, now)` 처럼 시계를 받는 이유 |
| 10~25분 | 가장 단순한 정답 + 테스트 하나 | "먼저 맞게, 그다음 빠르게" |
| 25~35분 | 표 주도 테스트, `-race`, 에러·취소 | 어떤 경우를 표에 넣었는지 |
| 35~45분 | 복잡도와 개선 | 시간·공간 복잡도, 남긴 위험, 다음에 할 일 |

막혀도 말을 멈추지 않습니다. 면접관이 볼 수 있는 것은 화면의 코드와 말뿐이라, 무엇을 확인하는 중인지 소리 내어 말합니다.

## 과제 A: LRU 캐시는 맵 하나와 연결 리스트 하나로 O(1) 이 된다

처음 5분에 묻는 것은 세 가지입니다. 넘치면 무엇을 버리나(가장 오래 안 쓴 것), 여러 고루틴이 부르나(그렇다), 만료가 있나(1분)입니다.

가장 단순한 정답은 만료 없이 용량만 지킵니다. `container/list` 의 앞쪽을 최근으로 두고, 맵은 키에서 리스트 원소로 바로 갑니다. 핵심은 `Put` 입니다.

```go
func (c *Cache) Put(key string, val int) {
	c.mu.Lock()
	defer c.mu.Unlock()
	if el, ok := c.items[key]; ok {
		el.Value.(*entry).val = val
		c.ll.MoveToFront(el)
		return
	}
	c.items[key] = c.ll.PushFront(&entry{key, val})
	if c.ll.Len() > c.cap {
		old := c.ll.Back()
		c.ll.Remove(old)
		delete(c.items, old.Value.(*entry).key)
	}
}
```

`Get` 도 같은 락을 잡고 `MoveToFront` 를 부릅니다. 만료를 더할 때는 항목에 `expires` 를 두고, 시계를 `now func() time.Time` 으로 받습니다. 그래야 테스트가 1분을 기다리지 않습니다. 시계를 주입하는 방법은 [5편](https://blog.doksam.com/blog/go-course-05-tests-and-concurrency/)에서 다뤘습니다.

```go
if c.ttl > 0 && !c.now().Before(e.expires) {
	c.remove(el)
	return zero, false
}
```

만료된 항목은 읽을 때 지웁니다(게으른 삭제). 따로 청소 고루틴을 두지 않으니 멈출 방법을 정할 필요가 없습니다. 대신 `Len` 은 아직 안 읽은 만료 항목도 셉니다. 이 차이를 먼저 말해 두면 면접관이 묻기 전에 트레이드오프(trade-off, 맞바꿈)를 보인 셈입니다.

테스트는 동작 순서를 표로 적습니다. 경우마다 이름을 붙였습니다.

```go
{"만료 시각이 되면 없다", []step{
	{op: "put", key: "a", val: 1}, {op: "wait", wait: 59 * time.Second},
	{op: "get", key: "a", val: 1, ok: true}, {op: "wait", wait: time.Second},
	{op: "get", key: "a", ok: false},
}},
```

이런 경우 넷(용량 초과, 다시 넣기, 만료, 만료 연장)과 고루틴 8개가 동시에 쓰는 시험을 `-race` 로 돌려 모두 통과했습니다.

**흔한 실수는 `Get` 에 읽기 락을 잡는 것입니다.** 이름이 "읽기"라서 `RWMutex.RLock` 을 잡지만, `MoveToFront` 는 리스트를 고치는 쓰기입니다. 그렇게 고친 판은 `-race` 없이 통과하고 `-race` 로는 실패했습니다(출력 일부, 경로는 줄였습니다).

```text
$ go test -count=1 ./rlock
ok  	example.com/broken/rlock	0.347s
$ go test -race -count=1 ./rlock
==================
WARNING: DATA RACE
Read at 0x00c0000a46c0 by goroutine 9:
  container/list.(*List).move()
      …/container/list/list.go:122 +0x130
  container/list.(*List).MoveToFront()
  …
  example.com/broken/rlock.(*Cache).Get()
      …/rlock/rlock.go:32 +0xc4
…
--- FAIL: TestConcurrentGet (0.00s)
    testing.go:1712: race detected during execution of test
```

## 과제 B: 크롤러는 동시성 상한, 중복 제거, 취소, 에러 묶기를 함께 본다

처음 5분에는 네 가지를 묻습니다. 깊이 제한, 같은 주소 중복, 동시에 읽을 수, 한 주소의 실패가 전체를 멈추는지입니다. 여기서는 실패한 주소를 모아 끝에 함께 돌려주기로 합의했다고 둡니다.

```go
var visit func(u string, d int)
visit = func(u string, d int) {
	select {
	case sem <- struct{}{}: // 동시에 읽는 수를 workers 개로 막는다
	case <-ctx.Done():
		return
	}
	links, err := fetch(ctx, u)
	<-sem
	mu.Lock()
	defer mu.Unlock()
	if err != nil {
		errs = append(errs, fmt.Errorf("%s: %w", u, err))
		return
	}
	fetched = append(fetched, u)
	if d == 0 {
		return
	}
	for _, l := range links {
		if !seen[l] {
			seen[l] = true
			wg.Go(func() { visit(l, d-1) })
		}
	}
}
```

호출하는 쪽은 `wg.Go(func() { visit(start, depth) })` 뒤에 `wg.Wait()` 로 모두 끝나기를 기다립니다. 그다음 `context.Cause(ctx)` 를 에러 목록에 더하고 `errors.Join` 으로 묶습니다. `wg.Go` 는 Go 1.25, `context.Cause` 와 `errors.Join` 은 Go 1.20 에 들어왔습니다(`api/go1.N.txt`). 테스트는 두 판 모두 `httptest` 로 만든 가짜 사이트를 깊이 0·1·2 로 읽고, `/d` 가 주는 500 이 `errors.Is(err, ErrStatus)` 로 잡히는지 봅니다. 깊이 2 의 실제 결과입니다.

```text
읽은 경로: / /a /b /c
에러: /d: unexpected status: 500
```

**누수 시험은 `runtime.NumGoroutine` 으로 합니다.** 취소된 뒤 고루틴 수가 시작 때보다 많으면 실패입니다. 결과를 보낼 때 취소를 보지 않고 작업자를 기다리지도 않는 판은 이렇게 실패했습니다.

```text
--- FAIL: TestCancelLeaksNothing (1.00s)
    leak_test.go:24: 고루틴 2 → 6, 새는 고루틴이 있다
```

이 시험을 쓰다 겪은 함정도 있습니다. 처음에는 `Crawl` 이 돌아오자마자 수를 셌습니다. `-count=5` 로 여섯 번 돌리자 두 번이 `고루틴 3 → 4` 로 실패했습니다. `Wait` 가 돌아온 순간에도 마지막 고루틴은 `Done` 을 부른 뒤 빠져나가는 중일 수 있습니다. 1초까지 다시 세게 고친 뒤 마흔 번 돌려 모두 통과했습니다.

## 질문 은행은 앞 편의 면접 문항에서 골랐다

| 주제 | 질문 | 답의 뼈대 | 복습할 편 |
|---|---|---|---|
| 값·포인터 | 리시버는 어떻게 고르나 | 고치거나 복사하면 안 되거나 크면 포인터, 한 타입 안에서 섞지 않는다 | 2 |
| 슬라이스 | `append` 뒤 다른 슬라이스가 바뀌는 때 | 같은 배열에 용량이 남을 때, 세 번째 인덱스나 `slices.Clone` | 2, 13 |
| 인터페이스 | nil 을 돌려줬는데 `err != nil` | 타입 칸이 찬 typed nil, 리터럴 `nil` 을 돌려준다 | 3, 14 |
| 에러 | `%w` 와 `%v` | `%w` 만 원인을 남긴다, 숨길 때는 `%v` | 3, 9 |
| 에러 | 에러는 어디서 기록하나 | 맨 위 한 곳, 아래 층은 맥락만 붙여 반환 | 9 |
| HTTP | 서버 타임아웃 네 가지 | ReadHeader·Read·Write·Idle, 기본값 없음, 헤더 타임아웃은 필수 | 4, 10 |
| 종료 | 그레이스풀 종료 순서 | 준비 확인 내림 → `Shutdown` → 처리 중 요청 대기, 반환을 기다린다 | 6, 10 |
| DB | 커넥션 풀 고갈 | 줄 서다 context 제한에 실패, `Stats()` 의 InUse·WaitCount | 11 |
| 테스트 | 시간에 기대는 코드 | 시계 주입이나 `testing/synctest` | 5, 12 |
| 맵 | 동시에 쓰면 | `recover` 로 못 잡는 치명 오류, 뮤텍스로 지킨다 | 13 |
| 동시성 | 채널과 뮤텍스 선택 | 소유권을 넘기면 채널, 함께 고치면 뮤텍스, 숫자 하나면 atomic | 5, 18 |
| 동시성 | `-race` 통과면 안전한가 | 아니다, 실행된 경로만 본다 | 5, 19 |
| 채널 | 채널은 누가 닫나 | 보내는 쪽, 닫힌 채널에 보내면 패닉 | 18 |
| 스케줄러 | `for {}` 가 다른 고루틴을 굶기나 | 비동기 선점이 끊는다 | 17 |
| GC | GOGC 와 GOMEMLIMIT | 비율과 천장, 목표는 낮은 쪽 | 16 |

13편 이후는 [로드맵](https://blog.doksam.com/blog/go-course-00-roadmap/)에서 찾아 읽습니다.

## 채점 기준은 다섯 칸으로 본다

| 항목 | 충분하다고 볼 모습 |
|---|---|
| 정확성 | 합의한 경우가 모두 맞고, 경계값(용량 1, 깊이 0)을 스스로 짚는다 |
| 테스트 | 표 주도 테스트에 이름이 있고, 실패 메시지에 받은 값과 원하는 값이 있다 |
| 동시성 안전 | `-race` 로 돌렸고, 공유 상태마다 누가 지키는지 말한다 |
| 에러와 취소 | `%w` 로 감싸고, 취소 뒤 고루틴이 남지 않음을 보인다 |
| 소통 | 막힌 곳과 버린 대안을 말하고, 남은 위험으로 끝낸다 |

## 한 단계 더 — 복잡도와 선택의 근거를 숫자로 말한다

LRU 의 `Get`·`Put` 은 맵 조회와 리스트의 앞뒤 조작만 하므로 평균 O(1) 이고, 공간은 O(용량) 입니다. 락이 하나라 고루틴이 많으면 여기서 줄을 섭니다. 다음 개선으로 키를 해시해 여러 조각으로 나누는 방법을 말할 수 있습니다.

크롤러는 두 가지로 짰습니다. 위의 세마포어(semaphore, 동시에 들어갈 수를 세는 채널) 판은 주소마다 고루틴을 띄우고 동시 읽기만 막습니다. 고정 작업자 판은 고루틴 8개가 일을 받고, 조정자 하나가 방문 기록을 혼자 가져 뮤텍스가 없습니다. 가지 10개, 깊이 3(1,111쪽), 한 쪽에 2ms 인 가짜 사이트를 작업자 8개로 다섯 번씩 읽었습니다.

| 판 | 고루틴 수 최댓값(중앙값) | 걸린 시간(중앙값) |
|---|---:|---:|
| 세마포어 | 940~953 | 297~342ms |
| 고정 작업자 | 9 | 294~351ms |

세 번 돌린 범위입니다. 시간은 비슷하고 고루틴 수는 100배 넘게 다릅니다. 고정 작업자 판의 조정자는 보낼 일이 없을 때 nil 채널로 보내기 case 를 끕니다.

```go
var send chan job // 보낼 일이 없으면 nil 이라 이 case 는 꺼진다
var next job
if len(queue) > 0 {
	send, next = jobs, queue[0]
}
select {
case send <- next:
	queue = queue[1:]
	pending++
case r := <-results:
	pending--
	// … 결과를 기록하고 새 링크를 queue 에 넣는다(줄임)
case <-ctx.Done():
	break loop
}
```

작업자는 결과를 보낼 때도 `ctx.Done()` 을 함께 기다리고, 조정자는 돌아오기 전에 작업자를 `Wait` 로 기다립니다. 위의 누수 판은 이 둘을 빠뜨렸습니다. 세마포어 판은 읽을 쪽 수만큼 기다리는 고루틴이 생기므로, 쪽 수가 늘면 그만큼 스택 메모리가 늘어납니다. 그래서 저는 면접에서 세마포어 판으로 먼저 맞히고, 이 표를 근거로 고정 작업자 판을 개선안으로 냅니다.

시니어 면접은 같은 코드에 "왜 이것을 골랐나"를 더 묻는다고 봅니다. 답은 취향이 아니라 이런 측정과 버린 대안이어야 합니다.

## 면접에서 이렇게 묻는다

**Q1. 가장 어려웠던 동시성 버그는 무엇입니까?**
증상을 숫자로 말하고(고루틴 3 → 4, 여섯 번 중 두 번), 재현 방법(`-count`, `-race`, `NumGoroutine`), 원인, 고친 방법, 재발을 막는 시험 순서로 답합니다. 위 누수 시험의 함정처럼 작은 사례도 이 순서면 충분합니다.

**Q2. Go 를 고른 이유와 한계는 무엇입니까?**
고치고 다시 빌드하는 주기가 짧고 배포가 실행 파일 하나라는 점을 잰 숫자로 말합니다. 한계로는 정규식에 전후방 탐색이 없는 점과, 런타임 때문에 다른 언어에 끼워 넣기 번거로운 점을 듭니다. 근거는 [Go 와 Rust 를 쓰면 좋은 점](https://blog.doksam.com/blog/why-go-and-rust/)에 적었습니다.

**Q3. 운영에서 메모리가 계속 늘면 무엇부터 봅니까?**
고루틴 수가 같이 느는지부터 봅니다. 그다음 gctrace 와 힙 목표, 힙 프로파일, 지워도 줄지 않는 맵, 커넥션 풀 지표 순서로 좁힙니다. 각각 [로드맵](https://blog.doksam.com/blog/go-course-00-roadmap/)의 17·18편, 16편, 21편, 13편과 [11편](https://blog.doksam.com/blog/go-course-11-database-sql/)에서 다룹니다.

## 연습

### 연습 1 — 밀려난 항목을 알리는 콜백

LRU 에 `OnEvict func(key string, val int)` 를 더합니다. 콜백 안에서 `Get` 을 불러도 멈추지 않아야 합니다.

<details>
<summary>풀이</summary>

밀려난 항목을 락 안에서 모으고, 락을 푼 뒤에 콜백을 부릅니다. 락을 쥔 채 콜백을 부르는 판은 콜백의 `Get` 이 같은 락을 기다려 멈췄습니다.

```text
--- FAIL: TestOnEvictMayCallBack (1.00s)
    underlock_test.go:24: Put 이 1초 안에 끝나지 않았다: 콜백을 락 안에서 부른다
```

```go
c.mu.Unlock()
if c.OnEvict != nil {
	for _, e := range evicted {
		c.OnEvict(e.key, e.val)
	}
}
```

락을 쥔 채 남의 코드를 부르지 않는다는 규칙입니다.

</details>

### 연습 2 — 같은 호스트에는 간격을 두고 요청하기

제 데이터 수집 도구는 요청 사이에 쉽니다. 크롤러의 `Fetch` 를 감싸 같은 호스트로 가는 요청이 20ms 간격으로 시작하게 하고, 기다리는 중에 취소되면 바로 돌아오게 합니다.

<details>
<summary>풀이</summary>

락 안에서 다음 자리를 먼저 잡고, 락 밖에서 타이머와 `ctx.Done()` 을 함께 기다립니다.

```go
mu.Lock()
now := time.Now()
at := next[u.Host]
if at.Before(now) {
	at = now
}
next[u.Host] = at.Add(gap)
mu.Unlock()
t := time.NewTimer(at.Sub(now))
defer t.Stop()
select {
case <-t.C:
case <-ctx.Done():
	return nil, ctx.Err()
}
return fetch(ctx, raw)
```

시험은 다섯 요청의 마지막 시작이 80ms 뒤 이후인지, 1시간 간격에서 10ms 제한으로 취소하면 `DeadlineExceeded` 로 바로 돌아오는지 봅니다. `-race -count=3` 으로 통과했습니다.

</details>

## 정리

1편의 질문은 "Go 프로젝트는 무엇으로 시작하고 무엇이 남나"였습니다. 마지막 편의 답은 이렇습니다. 면접에서도 질문과 합의로 시작하고, 표 주도 테스트와 `-race` 를 통과한 코드와 잰 숫자를 남깁니다. 이 연재가 입사를 보장하지는 않습니다. 다만 무엇을 왜 그렇게 짰는지 실험으로 말할 재료는 됩니다.

복습은 [로드맵](https://blog.doksam.com/blog/go-course-00-roadmap/)을 따라 5편 → 18편 → 19편 순서로 동시성부터 다시 읽기를 권합니다. 라이브 코딩에서 가장 많이 틀리는 곳이 거기입니다.
