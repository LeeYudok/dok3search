# Go 강의 23편 — 설계 과제는 접수와 발송을 큐로 나누는 데서 시작한다

> "알림 발송 서비스를 설계해 보세요" 같은 과제에서 무엇을 어떤 순서로 말하고, Go 로는 어디까지 보여 줄 수 있습니까?

요구사항, 용량, API, 저장소, 동시성, 장애, 관측 순서로 말합니다. 뼈대는 하나입니다. 접수는 요청을 큐에 적고 바로 202 로 답하고, 발송은 워커가 바깥 업체의 속도에 맞춰 합니다. Go 로는 이 뼈대를 표준 라이브러리만으로 약 300줄에 짤 수 있습니다. 멱등 키, 백프레셔(backpressure, 넘치는 요청을 되돌려 보내기), 재시도, 데드 레터, 종료 때 비우기를 테스트와 실행 결과로 보입니다.

> [그림] 배달 로봇이 번호표 기계가 달린 접수대에 상자를 내려놓는다. 상자는 컨베이어를 타고 작업 로봇 셋에게 가고, 실패한 상자는 다시 시도 고리를 돌다가 빨간 데드 레터 통으로 떨어진다

번호표 기계가 멱등 키이고 컨베이어가 큐입니다. 정한 횟수만큼 실패한 상자는 다시 시도 고리를 벗어나 데드 레터 통으로 갑니다.

이 사이트의 문의 접수도 같은 모양입니다. 검증과 IP 당 제한을 거친 문의를 알림 채널마다 한 줄씩 큐 테이블에 넣고 끝냅니다. 발송은 별도 워커가 채널마다 상태와 재시도를 관리합니다. DB 가 내려가면 접수는 503 으로 답하고 화면은 그대로 뜹니다. 이 편은 그 구조를 일반화한 알림 서비스를 면접 순서대로 풉니다. 출력은 2026-11-18 에 맥(Apple M4 Pro)과 Go 1.26.2 로 돌린 결과입니다.

## 요구사항부터 묻고, 넣을 것과 뺄 것을 표로 적는다

과제 한 문장에는 답을 가르는 질문이 숨어 있습니다. 중복 발송을 허용하는지, 순서가 중요한지, 얼마나 늦어도 되는지, 기록을 얼마나 보관하는지 먼저 묻습니다. 답을 못 받으면 가정을 말하고 적어 둡니다.

| 구분 | 이번 설계에 넣는 것 | 빼는 것 |
|---|---|---|
| 기능 | 요청 하나를 채널 여러 개로 보낸다, 작업 상태 조회 | 템플릿, 예약 발송, 수신 거부 |
| 보장 | 접수한 요청을 잃지 않는다, 같은 요청을 두 번 접수하지 않는다 | 정확히 한 번 발송 |
| 성능 | 접수는 바로 답한다, 발송은 몇 초 늦어도 된다(가정) | 채널 사이의 순서 |

## 용량은 잰 숫자에 가정을 곱해 추정한다

먼저 접수 쪽 처리량을 쟀습니다. 메모리 큐를 쓰는 서버에 Go 로 짠 부하 생성기(`cmd/loadgen`)가 클라이언트 32개로 5초씩 요청을 보냈습니다. 같은 맥 안의 루프백이고 다른 작업과 함께 돈 상태라, 운영 수치가 아니라 이 기계의 값입니다.

```text
요청 381046건, 76209 req/s, p50 319µs, p99 1.584ms, 상태 map[202:381046]
요청 364986건, 72997 req/s, p50 314µs, p99 1.904ms, 상태 map[202:364986]
요청 399324건, 79865 req/s, p50 280µs, p99 1.697ms, 상태 map[202:399324]
```

발송 쪽은 가짜 발송기에 지연을 넣고 워커 8개가 초당 보낸 수를 세 번씩 셌습니다.

| 발송 한 건 지연 | 계산(워커 ÷ 지연) | 잰 값 |
|---|---:|---:|
| 20ms | 8 ÷ 0.02 = 400/s | 400~402/s |
| 50ms | 8 ÷ 0.05 = 160/s | 160/s |

동시에 처리 중인 수 = 처리량 × 걸리는 시간(리틀의 법칙)이 그대로 맞습니다. 여기에 가정을 곱합니다. 하루 요청 1,000만 건, 평균의 10배인 피크, 요청당 채널 2개가 가정입니다. 업체 응답 200ms 와 작업 한 건 1KB 도 가정입니다.

```text
평균 요청      10,000,000 ÷ 86,400 ≈ 116/s    피크 ≈ 1,160/s
피크 작업      1,160 × 2 = 2,320/s
동시 발송      2,320 × 0.2s = 464  → 워커 약 470개
접수 여유      76,209 ÷ 1,160 ≈ 66배 (잰 값 기준)
저장           하루 작업 2,000만 × 1KB ≈ 20GB, 30일 ≈ 600GB
```

병목은 접수가 아니라 발송입니다. 그래서 둘을 큐로 나눕니다. 접수는 빨리 적고 답하고, 큐가 속도 차이를 흡수합니다.

## API 는 멱등 키 하나로 재시도를 안전하게 만든다

클라이언트는 타임아웃이 나면 같은 요청을 다시 보냅니다. 알림이 두 번 나가지 않게 `Idempotency-Key` 헤더를 받습니다(멱등, 여러 번 해도 결과가 같음).

```go
sum := sha256.Sum256(raw)
a.mu.Lock() // 같은 키가 동시에 두 번 와도 한 번만 넣는다
defer a.mu.Unlock()
if prev, ok := a.seen[key]; ok {
	if prev.hash != sum {
		http.Error(w, "같은 키로 다른 요청을 보냈다", http.StatusConflict)
		return
	}
	w.Header().Set("Idempotent-Replayed", "true")
	writeJSON(w, http.StatusAccepted, prev.body)
	return
}
```

```text
$ curl -s -i -X POST $U/v1/notifications -H 'Idempotency-Key: order-1001' \
    -d '{"channels":["email","sms","push"],"body":"주문이 접수됐습니다"}'
HTTP/1.1 202 Accepted
Content-Type: application/json

{"jobs":[{"id":"j1","channel":"email","state":"pending","attempts":0},…]}
$ (같은 명령 한 번 더)
HTTP/1.1 202 Accepted
Content-Type: application/json
Idempotent-Replayed: true
…
$ (같은 키, 다른 본문)
409
```

출력에서 `Date` 줄과 겹치는 본문은 줄였습니다. 같은 키에 다른 본문은 클라이언트 버그라서 409 입니다. 메모리 맵은 예제용입니다. 운영에서는 (키, 본문 해시, 응답) 을 DB 에 유니크 제약으로 둡니다([11편](https://blog.doksam.com/blog/go-course-11-database-sql/)). 그래야 서버가 여러 대여도 한 번만 들어갑니다.

## 작업은 상태 기계로 그리고, 보장은 최소 한 번으로 정한다

| 지금 상태 | 다음 상태 | 언제 |
|---|---|---|
| pending | sending | 워커가 집는다 |
| sending | sent | 성공 |
| sending | pending | 실패해서 다시 기다린다, 또는 보내는 도중에 멈췄다 |
| sending | dead | 시도 횟수를 다 썼다 |

DB 라면 작업 테이블에 상태, 시도 횟수, 다음 시도 시각, 마지막 에러를 둡니다. 상태와 다음 시도 시각으로 찾는 색인도 둡니다. 여러 워커가 같은 행을 집지 않게 행을 잠그고 가져갑니다. 이 부분은 설계만 적었고 이 편에서 돌리지 않았습니다.

문제는 sending 입니다. 보내는 도중에 멈추면 업체가 받았는지 알 수 없습니다. 예제의 워커는 이때 작업을 pending 으로 되돌리고, 다음 실행이 다시 보냅니다. 같은 알림이 두 번 갈 수 있습니다. 이것이 최소 한 번(at-least-once) 전달입니다. 받는 쪽이 작업 ID 로 중복을 걸러야 하는 이유입니다.

## 동시성은 크기가 정해진 큐와 워커 수로 정한다

```go
func (d *Dispatcher) Submit(channels []string, body string) ([]Job, error) {
	d.mu.Lock()
	defer d.mu.Unlock()
	if d.closed {
		return nil, ErrClosed
	}
	if len(d.queue)+len(channels) > cap(d.queue) {
		d.Rejected.Add(1)
		return nil, ErrBusy
	}
	// … 채널마다 Job 을 만들어 d.queue <- j
```

큐는 버퍼 채널이고 그 크기가 상한입니다. 요청의 채널이 모두 들어갈 자리가 없으면 하나도 넣지 않고 `ErrBusy` 를 돌려주고, 핸들러는 503 과 `Retry-After: 1` 로 답합니다. 큐에 넣는 쪽은 이 잠금 안에만 있으니 자리를 확인한 뒤의 송신은 막히지 않습니다. 큐 크기 2에 채널 둘짜리 요청을 두 번 보내는 `TestBackpressure` 가 202, 503 을 고정합니다.

워커는 `wg.Go`(Go 1.25)로 띄우고 `for j := range d.queue` 로 큐가 닫힐 때까지 돕니다. 워커 수가 곧 동시 발송 상한이고, 리틀의 법칙으로 정한 숫자가 여기 들어갑니다.

## 실패는 세어서 다시 보내고, 끝까지 실패하면 데드 레터로 뺀다

```go
func Backoff(attempt int, base, max time.Duration, rnd func() float64) time.Duration {
	d := base << (attempt - 1)
	if d > max || d <= 0 {
		d = max
	}
	half := d / 2
	return half + time.Duration(rnd()*float64(half))
}
```

대기 시간은 실패할 때마다 두 배로 늘리고 상한에서 자릅니다. 절반에서 전부 사이를 무작위로 고르는 것이 지터(jitter, 흔들기)입니다. 같은 순간에 실패한 작업이 한꺼번에 다시 몰리지 않게 합니다. 시도 횟수를 다 쓴 작업은 dead 로 뺍니다. 몇 번을 보내도 실패하는 독이 든 메시지(poison message)가 워커를 붙잡지 않게 하려는 것입니다.

가짜 발송기는 sms 를 두 번, push 를 아홉 번 먼저 실패시킵니다. 이 서버에 요청 하나를 보냈습니다(최대 4회, 첫 대기 100ms).

```text
level=INFO msg=sent job=j1 channel=email attempt=1
level=WARN msg=retry job=j2 channel=sms attempt=1 wait=93ms err="upstream 503"
level=WARN msg=retry job=j3 channel=push attempt=1 wait=69ms err="upstream 503"
level=WARN msg=retry job=j3 channel=push attempt=2 wait=166ms err="upstream 503"
level=WARN msg=retry job=j2 channel=sms attempt=2 wait=142ms err="upstream 503"
level=INFO msg=sent job=j2 channel=sms attempt=3
level=WARN msg=retry job=j3 channel=push attempt=3 wait=235ms err="upstream 503"
level=ERROR msg="dead letter" job=j3 channel=push attempt=4 err="upstream 503"
```

대기 시간이 50~100ms, 100~200ms, 200~400ms 구간 안에서 흔들립니다.

## 종료는 접수부터 닫고, 남은 일을 정한 시간만큼 비운다

```go
_ = srv.Shutdown(ctx)
d.Close()
select {
case <-done:
case <-time.After(*drain):
	stopRun()
	<-done
}
```

새 요청을 먼저 막고, 큐를 닫고, 워커가 남은 작업을 보낼 시간을 줍니다. HTTP 서버 쪽 종료 순서는 [10편](https://blog.doksam.com/blog/go-course-10-http-in-production/)에서 다뤘습니다. 시간이 넘으면 워커의 context 를 취소하고 못 보낸 작업은 pending 으로 남깁니다. 이번 서버는 발송에 100ms 가 걸리고 sms 가 계속 실패합니다. 요청 셋(작업 여섯)을 넣고 바로 SIGTERM 을 보냈습니다(비우는 시간 400ms, 출력 일부 줄임).

```text
level=INFO msg=sent job=j1 channel=email attempt=1
level=INFO msg=sent job=j3 channel=email attempt=1
level=WARN msg=retry job=j4 channel=sms attempt=1 wait=77ms err="upstream 503"
level=WARN msg="left pending" job=j4 channel=sms
level=WARN msg="left pending" job=j5 channel=email
…
level=WARN msg=drained stats="map[accepted:6 dead:0 queue_depth:0 rejected:0 retried:3 sent:2]"
```

SIGTERM 뒤에 보낸 요청은 연결부터 거절됐습니다(curl 종료 코드 7). 메모리 큐라서 pending 작업은 프로세스와 함께 사라집니다. 큐를 DB 에 두는 이유가 이 줄에 있습니다.

## 관측은 숫자 여섯 개로 시작한다

표준 라이브러리 `expvar` 로 숫자를 `/debug/vars` 에 내보냈습니다(첫 실행 뒤의 값).

```text
"notify": {"accepted":3,"dead":1,"queue_depth":0,"rejected":0,"retried":5,"sent":2}
```

| 숫자 | 이상 신호 | 먼저 볼 것 |
|---|---|---|
| queue_depth | 계속 오른다 | 발송이 접수를 못 따라간다. 워커 수, 업체 지연 |
| dead | 늘어난다 | 한 채널의 장애나 잘못된 요청 |
| rejected | 0 이 아니다 | 큐가 찼고 백프레셔가 동작 중이다 |
| retried ÷ sent | 비율이 오른다 | 업체가 불안정하다 |

로그에는 작업 ID, 채널, 시도 횟수를 늘 남기고, 경보는 사람이 할 일이 있는 숫자에만 겁니다.

## 한 단계 더 — 큐를 어디에 둘 것인가와 정확히 한 번

| 큐 위치 | 프로세스가 죽으면 | 순서 | 운영 부담 |
|---|---|---|---|
| 메모리 채널 | 잃는다(위 실행) | 큐 안에서는 들어온 순서 | 없다 |
| DB 테이블 | 남는다 | 색인과 쿼리가 정한다 | 이미 쓰는 DB 면 작다 |
| 메시지 브로커 | 설정에 따라 남는다 | 브로커가 정한다 | 서버를 하나 더 운영한다 |

판단은 이렇습니다. 이미 DB 를 쓰고 처리량이 그 DB 에 부담이 되지 않는 동안은 테이블 큐로 시작합니다. 여러 소비자나 더 큰 처리량이 필요해지면 브로커로 옮깁니다. 이 사이트의 문의 접수가 테이블 큐입니다.

정확히 한 번이 어려운 이유는 "보냈다"를 기록하는 일과 실제 전달이 서로 다른 시스템에서 일어나서입니다. 업체에 보낸 직후, 기록하기 전에 멈추면 위의 sending 문제가 생깁니다. 그래서 최소 한 번 전달에 멱등 키를 더해 결과만 한 번이 되게 만듭니다. 업무 데이터와 알림 요청을 한 트랜잭션에 쓰고 별도 워커가 옮기는 아웃박스(outbox) 패턴도 같은 생각입니다. 주문은 저장됐는데 알림 요청만 사라지는 틈을 없앱니다.

## 면접에서 이렇게 묻는다

**Q1. 설계 과제를 받으면 무엇부터 묻습니까?**
중복 허용, 순서, 지연 목표, 보관 기간입니다. 이 넷이 큐의 위치와 보장 수준을 가릅니다. 답을 못 받으면 가정을 말하고 표로 적습니다.

**Q2. 같은 요청이 두 번 오면 어떻게 막습니까?**
`Idempotency-Key` 와 본문 해시를 저장합니다. 같은 키·같은 본문은 처음 응답을, 다른 본문은 409 를 돌려줍니다. 동시에 두 번 오는 경우는 유니크 제약이나 잠금으로 막습니다.

**Q3. 발송이 접수보다 느리면 어떻게 됩니까?**
큐가 차이를 흡수합니다. 큐 크기에 상한을 두고 넘으면 503 과 `Retry-After` 로 되돌려 보냅니다. 워커 수는 처리량 × 지연으로 정하고 실측(워커 8, 20ms → 초당 400)으로 확인합니다.

**Q4. 최소 한 번과 정확히 한 번은 무엇이 다릅니까?**
보내는 도중에 멈추면 전달 여부를 알 수 없어 다시 보내야 합니다. 그래서 전달은 최소 한 번이고, 받는 쪽이 ID 로 중복을 걸러 결과를 한 번으로 만듭니다.

**Q5. 배포할 때 진행 중인 작업은 어떻게 됩니까?**
접수를 먼저 닫고 큐를 닫은 뒤 정한 시간만큼 비웁니다. 넘으면 취소하고 pending 으로 남겨 다음 프로세스가 이어서 보냅니다. 메모리 큐라면 이 작업은 사라진다는 점도 말합니다.

## 연습

### 연습 1 — 데드 레터를 다시 넣기

원인을 고친 뒤 운영자가 dead 작업을 다시 보내게 `Redrive(id string) error` 를 만듭니다. dead 가 아닌 작업에는 `ErrNotDead` 를 돌려줍니다.

<details>
<summary>풀이</summary>

```go
func (d *Dispatcher) Redrive(id string) error {
	d.mu.Lock()
	defer d.mu.Unlock()
	j, ok := d.jobs[id]
	switch {
	case !ok:
		return ErrNotFound
	case j.State != Dead:
		return ErrNotDead
	case d.closed:
		return ErrClosed
	case len(d.queue) == cap(d.queue):
		return ErrBusy
	}
	j.State, j.Attempts, j.LastErr = Pending, 0, ""
	d.Dead.Add(-1)
	d.queue <- j
	return nil
}
```

push 가 네 번 실패해 dead 가 된 작업을 다시 넣으면 다섯 번째 시도에서 sent 가 됩니다. `TestRedrive` 가 `-race` 로 통과했습니다. `Submit` 과 같은 잠금 안에서 넣어야 송신이 막히지 않습니다.

</details>

### 연습 2 — 채널마다 동시 발송 수를 제한하기

sms 업체가 동시에 2건만 받는다고 합시다. 워커가 8개여도 sms 는 동시에 2건을 넘지 않게 합니다.

<details>
<summary>풀이</summary>

채널마다 크기 2인 버퍼 채널을 세마포어(semaphore, 동시 개수 제한)로 둡니다.

```go
func (d *Dispatcher) send(ctx context.Context, j *Job) error {
	if sem := d.limit[j.Channel]; sem != nil {
		select {
		case sem <- struct{}{}:
			defer func() { <-sem }()
		case <-ctx.Done():
			return ctx.Err()
		}
	}
	return d.sender.Send(ctx, d.snapshot(j))
}
```

sms 작업 40개를 워커 8개로 보내며 동시 최댓값을 셌습니다. 제한이 없으면 8, 제한하면 2 였고 세 번 모두 같았습니다. 다만 자리를 기다리는 워커는 다른 채널 일도 못 합니다. sms 가 밀리면 email 도 늦어지니, 채널마다 큐와 워커를 따로 두는 것이 다음 단계입니다.

</details>

## 정리

설계 과제는 요구사항, 용량, API, 저장소, 동시성, 장애, 관측 순서로 풉니다. 알림 서비스의 뼈대는 접수와 발송을 큐로 나누는 것입니다. 멱등 키, 크기를 정한 큐, 재시도와 데드 레터, 종료 때 비우기가 그 위에 붙습니다. Go 표준 라이브러리만으로 이 전부를 약 300줄과 테스트로 보일 수 있습니다.

[로드맵의 24편](https://blog.doksam.com/blog/go-course-00-roadmap/)에서는 앞 편들의 질문을 모아 모의 면접과 라이브 코딩으로 연습합니다.
