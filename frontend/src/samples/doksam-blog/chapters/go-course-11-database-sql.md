# Go 강의 11편 — sql.Open 은 연결을 열지 않고, *sql.DB 는 연결 풀이다

> `sql.Open` 은 연결을 여는 함수가 아닌데, 그럼 연결은 언제 생기고 몇 개까지 생깁니까?

연결은 쿼리가 처음 필요로 할 때 생깁니다. `*sql.DB` 는 연결 하나가 아니라 연결 풀(connection pool, 연결을 모아 두고 돌려 쓰는 곳)입니다. 기본값에서는 열린 연결 수에 상한이 없고, 쉬는 연결은 2개까지만 남깁니다. 상한을 정하면 넘친 쿼리는 줄을 서고, 닫지 않은 `Rows` 는 자리를 돌려주지 않습니다. 이 편에서는 이것을 `db.Stats()` 로 직접 봅니다.

> [그림] 자리가 네 개뿐인 주차장이 꽉 차서 차단기가 내려와 있고, 상자를 든 배달 로봇이 그 앞에서 기다린다. 4번 자리의 차는 시동이 켜진 채 운전자가 없다

주차 자리가 연결이고 차단기 앞의 로봇이 기다리는 쿼리입니다. 다 쓰고도 비우지 않은 4번 자리의 차가 닫지 않은 `Rows` 입니다.

실험은 2026-11-18 에 순수 Go SQLite 드라이버 `modernc.org/sqlite` v1.61.1(SQLite 3.54.1), Apple M4 Pro, Go 1.26.2 로 돌렸습니다.

## sql.Open 은 인자만 확인하고 연결은 쿼리가 만든다

```go
bad, err := sql.Open("sqlite", "file:/no/such/dir/app.db")
fmt.Println("잘못된 경로 Open 에러:", err, "/ 열린 연결:", bad.Stats().OpenConnections)
fmt.Println("잘못된 경로 Ping 에러:", bad.PingContext(ctx))
```

```text
잘못된 경로 Open 에러: <nil> / 열린 연결: 0
잘못된 경로 Ping 에러: unable to open database file (14)
Ping 전 열린 연결: 0
Ping 에러: <nil>
Ping 뒤 열린 연결: 1 / 쉬는 연결: 1
```

뒤의 세 줄은 같은 프로그램이 임시 폴더의 파일로 다시 연 결과입니다. 없는 폴더라도 `Open` 은 성공하고, 연결은 `Ping` 이 처음 만듭니다. `sql.go` 의 `Open` 주석도 확인하려면 `Ping` 을 부르라고 적고, `DB` 는 고루틴 여럿이 함께 쓰는 풀이니 `Open` 은 한 번만 부르라고 적습니다.

그래서 서버는 시작할 때 시간 제한을 건 `PingContext` 를 한 번 부릅니다. 이 사이트의 Go 서버는 5초 제한으로 붙어 보고, 실패해도 화면은 내보내며 문의 저장만 503 으로 답합니다.

## *sql.DB 는 풀이고, 상한을 넘은 쿼리는 줄을 선다

상한 4에서 무거운 쿼리 20개를 `wg.Go`(Go 1.25)로 동시에 던졌습니다. 이어서 상한을 1로 줄이고 트랜잭션이 연결을 쥔 동안 다른 쿼리를 보냈습니다.

```go
db.SetMaxOpenConns(1)
tx, _ := db.BeginTx(context.Background(), nil) // 하나뿐인 연결을 쥔다
ctx, cancel := context.WithTimeout(context.Background(), 200*time.Millisecond)
defer cancel()
err = db.QueryRowContext(ctx, "SELECT 1").Scan(new(int))
```

```text
실행 중 가장 많이 열린 연결: 4
20개 동시 실행 뒤: open=2 inUse=0 idle=2 waitCount=16 maxIdleClosed=2
최대 1개로 줄인 뒤: open=1 inUse=0 idle=1 waitCount=16 maxIdleClosed=3
기다리다 끝난 쿼리: context deadline exceeded (202 ms)
트랜잭션이 쥔 동안: open=1 inUse=1 idle=0 waitCount=17 maxIdleClosed=3
커밋을 기다린 쿼리: err=<nil> (100 ms)
끝난 뒤: open=1 inUse=0 idle=1 waitCount=18 maxIdleClosed=3
```

연결은 4개까지만 열렸고 16개(20 − 4)가 기다렸습니다. 끝난 뒤 2개만 남은 것은 쉬는 연결의 기본 상한이 2이기 때문입니다. 트랜잭션이 연결을 쥐자 다음 쿼리는 200ms 제한에 걸렸고, 커밋 뒤에는 기다리던 쿼리가 그 연결을 받았습니다.

| 설정 | 기본값(`sql.go` 주석) | 하는 일 |
|---|---|---|
| `SetMaxOpenConns` | 0 = 상한 없음 | 열린 연결 수의 상한 |
| `SetMaxIdleConns` | 2 | 쓰고 돌아온 연결을 몇 개 남길지. 넘치면 닫는다 |
| `SetConnMaxLifetime` | 0 = 나이로 닫지 않음 | 정한 나이가 된 연결을 닫고 새로 연다 |
| `SetConnMaxIdleTime` (Go 1.15) | 0 = 쉰 시간으로 닫지 않음 | 오래 쉰 연결을 닫는다 |

풀 크기는 정확성도 정합니다. SQLite 파일에 상한 없이 20개를 동시에 저장하자 `database is locked (5) (SQLITE_BUSY)` 로 실패했고, 상한 1에서는 모두 들어갔습니다.

## 닫지 않은 Rows 는 연결을 놓지 않는다

```go
func firstName(ctx context.Context, db *sql.DB) (string, error) {
	rows, err := db.QueryContext(ctx, "SELECT name FROM users ORDER BY id")
	if err != nil {
		return "", err
	}
	if rows.Next() {
		var name string
		err := rows.Scan(&name)
		return name, err // rows.Close 를 부르지 않았다
	}
	return "", rows.Err()
}
```

첫 행만 읽고 돌아오니 남은 행 때문에 `rows` 가 열린 채로 남습니다. 상한 1에서 이 함수를 부르고 다음 쿼리를 300ms 제한으로 보냈습니다. 고친 쪽은 `QueryContext` 바로 뒤에 `defer rows.Close()` 를 더했습니다.

```text
[Close 없음] 첫 이름=kim err=<nil> inUse=1
[Close 없음] 다음 쿼리: context deadline exceeded
[defer Close] 첫 이름=kim err=<nil> inUse=0
[defer Close] 다음 쿼리: <nil>
```

제한 없는 `context.Background()` 로 보내면 돌아오지 않습니다. 다른 고루틴이 없는 실험 프로그램에서는 런타임이 교착을 알아챘습니다(줄임).

```text
fatal error: all goroutines are asleep - deadlock!
…
	database/sql/sql.go:1369 +0x45c
```

`sql.go` 1369행은 연결과 `ctx.Done()` 가운데 먼저 오는 것을 기다리는 `select` 입니다. 서버에서는 다른 고루틴 때문에 이 감지도 없이 요청만 쌓입니다. `QueryContext` 뒤에는 바로 `defer rows.Close()` 를 쓰고 반복 뒤에 `rows.Err()` 를 봅니다. `QueryRowContext` 는 `Scan` 이 닫아 줍니다.

## 시간 제한은 context 로 주고, 쿼리가 실제로 멈추는지 확인한다

재귀 CTE 로 1억까지 세는 쿼리에 시간 제한을 걸었습니다. 연결이 하나뿐이니 쿼리가 계속 돌았다면 바로 다음 `SELECT 1` 이 기다렸을 것입니다.

```text
제한 100ms: n=0 err=context deadline exceeded 걸린 시간=100ms
  바로 다음 SELECT 1: err=<nil> 걸린 시간=0ms
제한 500ms: n=0 err=context deadline exceeded 걸린 시간=502ms
  바로 다음 SELECT 1: err=<nil> 걸린 시간=0ms
```

제한 없이 돌리면 23.4·23.7·30.7초가 걸렸습니다(세 번, 다른 빌드와 함께 돈 상태). 이 드라이버는 context 가 끝나면 실행을 끊고 연결을 돌려줍니다. 드라이버마다 다르니 새 드라이버를 들이면 이 실험을 돌립니다.

## 없는 행은 ErrNoRows, NULL 은 Null[T] 로 받는다

행이 없으면 `Scan` 이 `sql.ErrNoRows` 를 돌려줍니다. 저장소는 `errors.Is(err, sql.ErrNoRows)` 일 때 자기 센티널 `ErrNotFound` 를 `%w` 로 감싸 돌려주고, 호출하는 쪽은 `database/sql` 을 몰라도 됩니다. NULL 이 든 열은 `string` 으로 받지 못합니다.

```text
string: sql: Scan error on column index 0, name "reply_to": converting NULL to string is unsupported
NullString: {String: Valid:false}
Null[string]: {V:a@example.com Valid:true}
*string: true
```

타입마다 있는 `sql.NullString` 이 예전 방식이고, Go 1.22 부터는 제네릭 `sql.Null[T]` 하나로 됩니다. 포인터로 받으면 NULL 이 `nil` 이 됩니다.

## 값은 플레이스홀더로만 넘긴다

```go
func namesUnsafe(ctx context.Context, db *sql.DB, owner string) ([]string, error) {
	q := "SELECT title FROM notes WHERE owner = '" + owner + "'" // 하면 안 되는 예
	return collect(db.QueryContext(ctx, q))
}

func namesSafe(ctx context.Context, db *sql.DB, owner string) ([]string, error) {
	return collect(db.QueryContext(ctx, "SELECT title FROM notes WHERE owner = ?", owner))
}
```

```text
이어 붙이기: ["kim 의 메모" "lee 의 비밀" "park 의 비밀"] err=<nil>
플레이스홀더: [] err=<nil>
```

입력 `kim' OR '1'='1` 이 이어 붙인 쪽에서는 조건문이 되어 남의 행까지 읽었습니다. 플레이스홀더(placeholder, 값 자리 표시)는 값을 문장과 따로 넘겨 이름 하나로만 쓰입니다. 표 이름 같은 식별자는 `?` 로 넘길 수 없어 `SELECT count(*) FROM ?` 가 `near "?": syntax error` 로 실패했습니다. 식별자는 허용 목록(map)에서 골라 붙입니다.

## 트랜잭션은 defer Rollback 으로 열고 Commit 으로 닫는다

```go
func transfer(ctx context.Context, db *sql.DB, from, to int64, amount int) error {
	tx, err := db.BeginTx(ctx, nil)
	if err != nil {
		return fmt.Errorf("트랜잭션 시작: %w", err)
	}
	defer tx.Rollback() // Commit 뒤에 불리면 아무것도 하지 않는다

	if _, err := tx.ExecContext(ctx,
		"UPDATE accounts SET balance = balance - ? WHERE id = ?", amount, from); err != nil {
		return fmt.Errorf("출금 %d: %w", from, err)
	}
	res, err := tx.ExecContext(ctx,
		"UPDATE accounts SET balance = balance + ? WHERE id = ?", amount, to)
	if err != nil {
		return fmt.Errorf("입금 %d: %w", to, err)
	}
	if n, _ := res.RowsAffected(); n == 0 {
		return fmt.Errorf("입금 %d: %w", to, ErrNoAccount)
	}
	return tx.Commit()
}
```

```text
처음: 1:100 2:0
1→2 30: <nil> | 1:70 2:30
1→9 30: 입금 9: 계좌 없음 | 1:70 2:30
1→2 500: 출금 1: constraint failed: CHECK constraint failed: balance >= 0 (275) | 1:70 2:30
…
Commit 뒤 Rollback: sql: transaction has already been committed or rolled back true
…
ctx 취소 뒤 Commit: context canceled | 1:70 2:30
```

없는 계좌로 보낸 이체는 출금 뒤에 실패했지만 잔액은 그대로입니다. 어느 `return` 으로 나가도 `defer tx.Rollback()` 이 되돌리고, 커밋 뒤에는 `sql.ErrTxDone` 만 돌려줍니다. `BeginTx` 주석대로 context 가 취소되면 `Commit` 이 에러를 내고 변경은 남지 않았습니다.

`…` 자리는 `TxOptions` 실험입니다. 이 드라이버는 `LevelReadCommitted` 와 `ReadOnly: true` 를 에러 없이 받고도 읽기 전용 트랜잭션 안의 `UPDATE` 를 막지 않았습니다. 이 옵션은 드라이버가 해석하니 쓰는 드라이버로 시험합니다.

## 저장소는 마이그레이션과 함께 만든다

스키마 SQL 은 `migrations/001_messages.sql`, `002_reply_to_index.sql` 로 두고 `//go:embed migrations/*.sql` 로 실행 파일에 넣습니다. 번호 순서대로 파일 하나를 트랜잭션 하나로 적용하고, 적용한 번호를 `schema_version` 표에 남깁니다.

```text
1번째 실행: 적용 2개, err=<nil>
2번째 실행: 적용 0개, err=<nil>
schema_version: 1,2
```

두 번 돌려도 같으니 서버가 시작할 때마다 부릅니다. 핸들러는 `Get(ctx, id)` 하나만 담은 인터페이스를 직접 선언하고, 테스트는 `t.TempDir()` 의 SQLite 파일로 200·404·400·503 을 표 하나로 봅니다(`go test -race` 통과).

## 한 단계 더 — 풀이 연결을 나눠 주는 방식

`sql.go` 의 `conn` 함수는 쉬는 연결이 있으면 가장 최근에 쉰 것부터 꺼냅니다. 오래 쉰 연결을 빨리 닫으려는 선택이라고 주석에 적혀 있습니다. 상한에 닿았으면 버퍼 1짜리 채널로 요청을 만들어 `connRequests` 에 넣고 `waitCount` 를 올린 뒤, 그 채널과 `ctx.Done()` 을 함께 기다립니다. 연결이 돌아오면 `putConnDBLocked` 가 기다리는 요청 하나를 `TakeRandom()` 으로 고릅니다. **온 순서대로 주지 않습니다.** 고갈되면 운 나쁜 요청이 계속 밀리니 요청마다 시간 제한을 둡니다.

`DB` 에서 만든 준비된 문장(`Stmt`)은 연결마다 다시 준비됩니다. `Prepare` 호출을 세는 드라이버 래퍼를 끼우고, 상한 3에서 같은 `Stmt` 를 30번 동시에 돌렸습니다.

```text
PrepareContext 직후 Prepare 횟수: 1
30번 실행 뒤 Prepare 횟수: 3 / 지금 열린 연결: 2 / 쉬는 자리가 없어 닫힌 연결: 1
```

연결 세 개에서 한 번씩, `Stmt` 주석대로 스스로 다시 준비했습니다.

이 사이트의 Go 서버는 PostgreSQL 드라이버 pgx 의 자체 풀(`pgxpool`)을 씁니다. 두 풀의 기본값을 소스에서 읽어 비교했습니다. pgx 는 읽기만 하고 실행하지 않았습니다.

| 항목 | `database/sql` | `pgxpool` (v5.13.0 소스) |
|---|---|---|
| 만들 때 연결 | 만들지 않는다 | 만들지 않는다 |
| 최대 연결 | 0 = 상한 없음 | 4 와 CPU 수 중 큰 값 |
| 쉬는 연결 | 2개까지 남김 | 개수 설정 없음, 쉰 시간으로 닫음 |
| 연결 수명 | 제한 없음 | 1시간 |
| 쉰 시간 제한 | 제한 없음 | 30분 |
| 주기 점검 | 없음 | 1분마다 |

가장 큰 차이는 최대 연결 수입니다. `database/sql` 은 상한이 없으니 운영에서는 DB 가 받는 연결 수를 서버 인스턴스 수로 나눠 `SetMaxOpenConns` 를 정합니다.

## 면접에서 이렇게 묻는다

**Q1. `sql.Open` 직후 DB 가 꺼져 있으면 어떻게 됩니까?**
`Open` 은 성공하고 첫 `Ping` 이나 쿼리가 실패합니다. 시작할 때 시간 제한을 건 `PingContext` 로 확인합니다.

**Q2. 커넥션 풀이 고갈되면 무슨 일이 생기고, 어떻게 찾습니까?**
새 쿼리가 줄을 서고 context 제한에 걸려 실패합니다. `db.Stats()` 의 `InUse`·`WaitCount`·`WaitDuration` 을 지표로 봅니다. 원인은 주로 닫지 않은 `Rows`, 오래 연 트랜잭션, 너무 작은 상한입니다.

**Q3. `defer tx.Rollback()` 을 쓰는 이유는 무엇입니까?**
어느 `return` 으로 나가도 되돌리기 위해서이고, 커밋 뒤에는 `ErrTxDone` 만 돌려줍니다.

**Q4. SQL 주입은 어떻게 막고, 표 이름 같은 식별자는 어떻게 합니까?**
값은 플레이스홀더로만 넘깁니다. 식별자는 플레이스홀더로 넘길 수 없으니 허용 목록에서 골라 붙입니다.

**Q5. N+1 문제는 무엇이고 어떻게 고칩니까?**
목록을 읽고 항목마다 쿼리를 더 보내 왕복이 1+N 번이 되는 패턴입니다. JOIN 이나 `IN` 으로 한 번에 읽고 Go 에서 묶습니다.

## 연습

### 연습 1 — 잔액 부족을 센티널 에러로 돌려주기

위 `transfer` 는 잔액 부족을 CHECK 제약 에러로만 알립니다. `errors.Is(err, ErrInsufficient)` 로 가를 수 있게 고치고, 같은 계좌끼리의 이체는 `ErrSameAccount` 로 거절합니다.

<details>
<summary>풀이</summary>

트랜잭션 안에서 잔액을 먼저 읽고 판단합니다.

```go
var balance int
err = tx.QueryRowContext(ctx, "SELECT balance FROM accounts WHERE id = ?", from).Scan(&balance)
// … ErrNoRows 는 ErrNoAccount 로, 다른 에러는 감싸서 돌려준다
if balance < amount {
	return fmt.Errorf("출금 %d (잔액 %d, 요청 %d): %w", from, balance, amount, ErrInsufficient)
}
```

`from == to` 는 트랜잭션 전에 거절합니다. 네 경우를 표 주도 테스트로 돌렸고 실패한 세 경우 모두 잔액이 그대로였습니다. 읽기와 쓰기 사이에 다른 트랜잭션이 끼어들 틈을 없애려면 조건을 `UPDATE … WHERE id = ? AND balance >= ?` 에 넣고 `RowsAffected` 로 판단합니다.

</details>

### 연습 2 — N+1 쿼리를 한 번으로 줄이기

메시지 목록을 읽은 뒤 메시지마다 `tags` 표를 한 번씩 읽는 함수가 있습니다. 메시지 10개일 때 쿼리가 몇 번 도는지 세고, JOIN 한 번으로 같은 결과를 내게 고칩니다.

<details>
<summary>풀이</summary>

함수가 `QueryContext` 하나만 있는 인터페이스를 받게 하면 테스트에서 호출 수를 세는 래퍼를 끼울 수 있습니다.

```go
rows, err := db.QueryContext(ctx, `SELECT m.id, t.tag FROM messages m
		JOIN tags t ON t.message_id = m.id ORDER BY m.id, t.tag`)
```

```text
tags_test.go:57: 메시지 10개: N+1 쿼리 11번, JOIN 쿼리 1번
```

두 결과가 같은지는 `reflect.DeepEqual` 로 확인했습니다.

</details>

## 정리

`sql.Open` 은 연결을 열지 않습니다. 연결은 쿼리가 필요로 할 때 풀에서 생기고, 기본값에서는 상한 없이 늘며 쉬는 연결은 2개만 남습니다. 상한, 요청마다의 시간 제한, `defer rows.Close()`, `defer tx.Rollback()` 을 한 묶음으로 씁니다.

12편에서는 테스트 설계를 다룹니다. 표 주도 테스트에 골든 파일과 퍼즈 테스트를 더해 회귀를 막는 법을 봅니다.
