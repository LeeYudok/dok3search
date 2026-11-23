# Go 강의 6편 — 운영 서버에는 실행 파일 하나와 헬스 체크만 올린다

> Go 서비스를 운영 서버에 올릴 때 무엇이 필요하고 무엇이 필요 없습니까?

필요한 것은 넷입니다. 정적으로 링크한 실행 파일, 살아 있는지 답하는 `/healthz`, 종료 신호에 처리 중인 요청을 끝내고 내려가는 코드, 올리기 전에 돌리는 gofmt → vet → test 입니다. 언어 런타임, 셸, 패키지 관리자, 외부 라이브러리는 필요 없습니다. 이 편에서는 앞 편들의 메시지 수신 서비스 `inbox` 를 이 모양으로 다듬습니다.

> [그림] 배달 로봇이 gofmt·vet·test 문 세 개를 지나 상자 하나를 들고 빈 컨테이너로 간다. 컨테이너 위 말풍선에 GET /healthz → ok 가 적혀 있다

검사 문 셋을 통과한 상자 하나만 컨테이너에 들어갑니다. 바깥에서는 `/healthz` 로 상태를 묻습니다. 출력은 2026-11-18 에 맥(Apple M4 Pro)과 Go 1.26.2 로 돌린 결과입니다.

## 실행 파일은 정적으로 링크해 하나로 만든다

옵션의 뜻과 크기 비교는 [1편](https://blog.doksam.com/blog/go-course-01-first-binary/)에 있습니다.

```bash
$ GOOS=linux GOARCH=amd64 CGO_ENABLED=0 go build -trimpath -ldflags="-s -w" -o bin/inbox-linux-amd64 .
$ file bin/inbox-linux-amd64
bin/inbox-linux-amd64: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), statically linked, Go BuildID=…, BuildID[sha1]=…, stripped
$ go list -m all
example.com/inbox
```

서버에 깔 공유 라이브러리도 외부 모듈도 없습니다. HTTP, JSON, 내장 파일(`go:embed`), 구조화 로그, 테스트가 모두 표준 라이브러리에 있습니다.

## 컨테이너의 마지막 단계에는 실행 파일만 담는다

이 사이트의 운영 Dockerfile 을 `inbox` 에 맞게 줄였습니다.

```dockerfile
# 1) 검사하고 빌드한다. 2) 실행 파일만 셸 없는 이미지에 담는다.
FROM golang:1.26-alpine AS build
WORKDIR /src
COPY go.mod ./
RUN go mod download
COPY . .
RUN test -z "$(gofmt -l .)" && go vet ./... && go test ./...
RUN CGO_ENABLED=0 go build -trimpath -ldflags="-s -w" -o /inbox .

FROM gcr.io/distroless/static-debian12:nonroot
COPY --from=build /inbox /inbox
ENV PORT=8080
EXPOSE 8080
USER nonroot
ENTRYPOINT ["/inbox"]
```

이번에는 이 PC 의 컨테이너 실행기(podman)에 연결되지 않아 이미지 빌드는 돌리지 못했습니다. 빌드 단계의 명령만 로컬에서 그대로 돌렸습니다.

```bash
$ go mod download
go: no module dependencies to download
$ sh -c 'test -z "$(gofmt -l .)" && go vet ./... && go test ./...'
ok  	example.com/inbox	(cached)
```

검사를 빌드 안에 두면 테스트가 깨진 코드는 이미지가 되지 못합니다. 마지막 단계의 distroless static 에 무엇이 있는지는 공식 README(2026-11-18 확인)로 봤습니다.

| 항목 | distroless static | 운영에 주는 영향 |
|---|---|---|
| 셸·패키지 관리자 | 없다 | 셸로 들어갈 수 없고 `curl` 도 없다 |
| CA 인증서, 시간대 정보 | 있다 | HTTPS 호출과 시간대 변환은 된다 |
| C 라이브러리(glibc) | 없다. `base-nossl` 부터 있다 | cgo 로 링크한 바이너리는 기댈 곳이 없다 |

마지막 줄 때문에 `CGO_ENABLED=0` 을 빼면 안 됩니다. 안을 들여다볼 때는 README 가 안내하는 `:debug` 태그(busybox 셸 포함)를 씁니다. 지금 README 는 debian13 이름으로 설명하니, 이 사이트의 debian12 태그를 올릴 때 다시 봅니다.

## 헬스 체크는 엔드포인트와 실행 파일 플래그 두 쪽에 둔다

`/healthz` 는 200 과 `ok` 를 돌려주고, 중간 캐시가 옛 응답을 들고 있지 않게 `Cache-Control: no-store` 를 붙입니다.

컨테이너 헬스 체크 명령은 이미지 안에서 도는데, distroless 에는 `curl` 도 `wget` 도 없습니다. 그래서 실행 파일에 `-healthcheck` 플래그를 둡니다. 3초 제한으로 자기 `/healthz` 를 불러 200 이면 0, 아니면 에러를 찍고 1 로 끝납니다. 이 사이트의 컨테이너 설정도 헬스 체크 명령에 이 플래그를 적습니다.

```text
$ PORT=18089 ./bin/inbox -healthcheck; echo $?
0
(서버를 멈춘 뒤)
$ PORT=18089 ./bin/inbox -healthcheck; echo $?
Get "http://127.0.0.1:18089/healthz": dial tcp 127.0.0.1:18089: connect: connection refused
1
```

`/healthz` 는 DB 같은 의존 대상을 보지 않습니다. 이 사이트의 서버도 DB 가 없으면 화면은 그대로 내보내고 문의 접수만 503 으로 답합니다. 헬스 체크가 DB 까지 보면 DB 가 잠깐 멈출 때 컨테이너가 모두 재시작 대상이 됩니다. 그래서 생존 확인(liveness)과 준비 확인(readiness, 요청을 받을 준비가 됐는지)을 나눕니다. 준비 확인은 연습 1에서 만듭니다.

## 종료 신호를 받으면 처리 중인 요청을 끝내고 내려간다

`man podman-stop` 은 컨테이너를 멈출 때 SIGTERM 을 보내고 10초 뒤 SIGKILL 을 보낸다고 적습니다. SIGTERM 에 바로 죽으면 처리 중인 요청이 끊깁니다. `run` 은 `signal.NotifyContext`(Go 1.16)로 신호를 `context` 취소로 바꿔 `serve` 에 넘깁니다.

```go
// serve 는 ctx 가 끝날 때까지 요청을 받는다. 끝나면 새 연결을 막고,
// 처리 중인 요청이 끝나기를 grace 만큼 기다린 뒤 돌아온다.
func serve(ctx context.Context, srv *http.Server, ln net.Listener, grace time.Duration, log *slog.Logger) error {
	errc := make(chan error, 1)
	go func() { errc <- srv.Serve(ln) }()
	select {
	case err := <-errc:
		return err
	case <-ctx.Done():
	}
	log.Info("종료 신호를 받았다", "grace", grace.String())
	shutdown, cancel := context.WithTimeout(context.Background(), grace)
	defer cancel()
	if err := srv.Shutdown(shutdown); err != nil {
		return fmt.Errorf("처리 중인 요청을 기다리지 못했다: %w", err)
	}
	if err := <-errc; !errors.Is(err, http.ErrServerClosed) {
		return err
	}
	return nil
}
```

리스너를 밖에서 받으니 테스트도 빈 포트로 같은 함수를 시험합니다. 실제로는 `curl --limit-rate 1k` 로 4,541바이트 메시지를 천천히 올리는 도중 SIGTERM 을 보냈습니다.

```text
SIGTERM 보냄
curl: HTTP 201, 4.439429초
curl 응답: {"id":1}
inbox 종료 코드: 0
{"time":"2026-11-18T01:30:24.330036+09:00","level":"INFO","msg":"서버 시작","addr":"[::]:18086","version":"unknown"}
{"time":"2026-11-18T01:30:25.83597+09:00","level":"INFO","msg":"종료 신호를 받았다","grace":"10s"}
{"time":"2026-11-18T01:30:28.979261+09:00","level":"INFO","msg":"서버 종료"}
```

신호 뒤 3.1초 동안 업로드를 기다렸고 요청은 201 로 끝났습니다. `grace` 는 실행기가 SIGKILL 을 보내기 전까지의 시간보다 짧게 잡습니다. 위의 10초는 podman 기본값과 같으니 운영에서는 둘 중 하나를 조정합니다. 로그는 `log/slog`(Go 1.21)의 JSON 핸들러로 표준 출력에 씁니다. 한 줄이 JSON 하나라 수집기가 필드로 나눠 읽습니다. 이 사이트의 서버는 DB 주소를 해석하지 못해도 실패 사실만 남기고 주소 값은 찍지 않습니다.

## 버전은 실행 파일에 넣고 엔드포인트로 확인한다

`inbox` 는 빌드 정보(`runtime/debug.ReadBuildInfo`, `Settings` 는 Go 1.18)의 `vcs.revision` 을 `/api/version` 으로 돌려줍니다. 버리는 git 저장소에 소스를 복사해 빌드했습니다.

```text
== git 안에서 빌드
	build	vcs.revision=4614b457845b4081cdc0072c2cccffa356fd9487
	build	vcs.modified=false
{"version":"4614b457845b"}
== -buildvcs=false
{"version":"unknown"}
== 커밋하지 않은 수정이 있을 때
{"version":"4614b457845b-dirty"}
```

출력은 줄였습니다. 이 방식은 빌드하는 곳에 `.git` 이 있어야 해서, 소스만 복사하는 위 Dockerfile 에서는 `unknown` 이 됩니다. 방법은 셋입니다.

| 방법 | 필요한 것 | 함정 |
|---|---|---|
| 빌드 정보의 `vcs.revision` | 빌드 위치에 `.git` | 소스만 복사한 빌드에서는 비어 있다 |
| `-ldflags "-X main.version=…"` | 빌드 명령에 값 전달 | 변수 이름을 틀려도 오류 없이 기본값이 남는다 |
| 버전 파일을 이미지에 복사 | 배포 스크립트가 파일을 쓴다 | 파일을 빠뜨리면 `unknown` |

`-X main.version=671a948f622f` 는 값을 바꿨지만, 대문자로 틀린 `-X main.Version=oops` 는 오류 없이 기본값 `dev` 를 찍었습니다. 이 사이트는 세 번째 방법을 씁니다. 컨테이너 빌드 도구가 환경변수 줄을 글자 그대로 캐시해 값이 안 바뀌던 일을 겪은 뒤 정했습니다.

## 의존성은 표준 라이브러리에 없는 것만 들인다

`inbox` 는 외부 모듈이 0개입니다. 이 사이트의 서버는 표준 라이브러리에 없는 PostgreSQL 드라이버 하나를 직접 들였고, 그 드라이버가 간접 의존성 5개를 데려왔습니다. 이 기준이 에이전트와 일할 때 유리한 이유는 [Go 와 Rust 를 쓰면 좋은 점](https://blog.doksam.com/blog/why-go-and-rust/)에 적었습니다. 들인 뒤에는 `go mod why` 로 이유를 확인합니다.

`go mod why -m example.com/nope` 처럼 물으면 필요한 모듈은 import 사슬이 나오고, 필요 없는 모듈은 `(main module does not need module example.com/nope)` 가 나옵니다. 그런 모듈은 `go mod tidy` 로 지울 후보입니다.

## 고칠 때마다 gofmt → vet → test → build 를 한 번에 돌린다

```bash
#!/usr/bin/env bash
# 코드를 고칠 때마다 돌리는 검사. 하나라도 실패하면 그 자리에서 멈춘다.
set -euo pipefail
cd "$(dirname "$0")/.."
unformatted="$(gofmt -l .)"
if [ -n "$unformatted" ]; then
  echo "gofmt 가 필요한 파일:" >&2
  echo "$unformatted" >&2
  exit 1
fi
go vet ./...
go test -race ./...
CGO_ENABLED=0 go build -trimpath -ldflags='-s -w' -o bin/inbox .
echo "검사 통과"
```

`gofmt -l` 은 형식이 틀린 파일이 있어도 종료 코드 0 을 돌려줍니다. 그래서 출력이 비었는지로 판단합니다. 형식을 깬 파일을 두면 스크립트가 그 이름을 찍고 1 로 멈췄습니다. 단계마다 다섯 번 잰 중앙값입니다.

| 단계 | 중앙값 |
|---|---:|
| `gofmt -l .` | 0.01초 미만 |
| `go vet ./...` | 0.15초 |
| `go test -count=1 ./...` | 1.17초 |
| `go test -race -count=1 ./...` | 2.26초 |
| `go build` (위 옵션) | 0.10초 |

다른 작업이 함께 돌아 부하 평균이 8~23 이던 때입니다. 한 바퀴가 3초 안쪽이라 고칠 때마다 돌려도 됩니다. 에이전트 지침(AGENTS.md)에는 "고친 뒤 `scripts/check.sh` 를 돌린다" 한 줄이면 됩니다.

## 한 단계 더 — Shutdown 을 기다리지 않으면 요청이 끊긴다

`go doc net/http.Server.Shutdown` 은 `Shutdown` 을 부르면 `ListenAndServe` 가 곧바로 `ErrServerClosed` 를 돌려주니, 프로그램은 끝나지 말고 `Shutdown` 의 반환을 기다리라고 적습니다. 자주 보이는 코드는 신호를 받은 고루틴이 `Shutdown` 을 부르고, `main` 은 `ListenAndServe` 가 돌아오면 끝납니다. 3초 걸리는 요청 도중 SIGTERM 을 보내 두 방식을 비교했습니다.

```text
== -wait=false
ListenAndServe 가 돌아왔다
서버 종료 코드: 0
curl: HTTP 000, 1.000579초
curl 종료 코드: 52
== -wait=true
ListenAndServe 가 돌아왔다
done
curl: HTTP 200, 3.001639초
Shutdown 이 끝났다
서버 종료 코드: 0
curl 종료 코드: 0
```

기다리지 않은 쪽은 신호 순간 프로세스가 끝났고, 클라이언트는 응답 없이 끊겼습니다. curl 매뉴얼에서 종료 코드 52 는 서버가 아무것도 답하지 않았다는 뜻입니다. 서버 종료 코드는 둘 다 0 이라 운영 기록으로는 차이가 보이지 않습니다.

`Shutdown` 은 남은 연결을 주기적으로 확인합니다. `net/http/server.go` 를 보면 간격이 1ms 에서 두 배씩 늘어 500ms 에서 멈춥니다. 300ms 요청 하나를 둔 종료 테스트가 0.54초 걸린 이유입니다.

컨테이너 안의 CPU 수도 `go` 줄에 따라 다르게 읽습니다. `go doc runtime.GOMAXPROCS` 는 리눅스에서 cgroup CPU 제한을 기본값에 반영하고, 언어 버전 1.24 이하에서는 이 동작이 꺼진다고 적습니다. `go` 줄만 바꿔 리눅스용으로 빌드했습니다.

```text
== go 1.24
	build	DefaultGODEBUG=containermaxprocs=0,cryptocustomrand=1,decoratemappings=0,tlssecpmlkem=0,tlssha1=1,updatemaxprocs=0,urlstrictcolons=0,x509sha256skid=0
== go 1.26.2
(DefaultGODEBUG 줄 없음)
```

툴체인이 같아도 `go` 줄이 1.24 면 옛 동작이 실행 파일에 박힙니다. `go` 줄을 올리는 일도 런타임을 바꾸는 배포입니다. 컨테이너 안에서 직접 돌려 보지는 못했습니다.

## 면접에서 이렇게 묻는다

**Q1. distroless static 이미지에 넣을 때 `CGO_ENABLED=0` 이 왜 필요합니까?**
그 이미지에는 glibc 가 없어서 C 라이브러리에 동적으로 기대는 바이너리는 돌 곳이 없습니다. `file` 로 `statically linked` 를 확인한다고 덧붙입니다.

**Q2. 셸 없는 이미지의 장단점은 무엇입니까?**
침입자가 쓸 도구와 취약점 점검 대상이 줄어듭니다. 대신 셸로 들어가 디버깅할 수 없고 `curl` 도 없어서, 헬스 체크를 실행 파일 플래그로 두고 들여다볼 때는 `:debug` 이미지를 씁니다.

**Q3. 헬스 체크가 DB 연결까지 확인해야 합니까?**
생존 확인은 프로세스 자신만 봅니다. DB 를 넣으면 DB 장애가 모든 인스턴스의 재시작으로 번집니다. 의존 대상의 상태는 준비 확인이나 기능별 503 으로 따로 냅니다.

**Q4. 그레이스풀 종료(graceful shutdown, 처리 중인 요청을 마치고 종료)의 순서는?**
신호를 `context` 취소로 받아 준비 확인을 내리고, `Shutdown` 이 리스너와 쉬는 연결을 닫은 뒤 처리 중인 연결을 제한 시간까지 기다립니다. `ListenAndServe` 가 곧바로 돌아오니 `Shutdown` 의 반환을 기다려야 한다는 함정까지 말합니다.

## 연습

### 연습 1 — 종료 신호를 받으면 준비 확인부터 내리기

`GET /readyz` 를 더합니다. 평소에는 200, 종료 신호 뒤에는 503 입니다. 503 으로 바꾼 뒤 정한 시간(drain)만큼 요청을 계속 받다가 `Shutdown` 하고, 그동안 `/healthz` 는 200 이어야 합니다.

<details>
<summary>풀이</summary>

여러 고루틴이 읽는 깃발이라 `atomic.Bool`(Go 1.19)을 씁니다.

```go
mux.HandleFunc("GET /readyz", func(w http.ResponseWriter, r *http.Request) {
	if draining.Load() {
		http.Error(w, "draining", http.StatusServiceUnavailable)
		return
	}
	fmt.Fprintln(w, "ready")
})
```

`serve` 에서는 `ctx.Done()` 다음에 `draining.Store(true)` 와 `time.Sleep(drain)` 을 두고 그 뒤에 `Shutdown` 합니다. 테스트는 종료 전 `/readyz` 200, drain 중 `/readyz` 503 과 `/healthz` 200, 종료 뒤 연결 거부를 차례로 보고, `-race` 와 함께 0.30초에 통과했습니다.

`Shutdown` 은 리스너부터 닫으므로 503 을 보여 줄 시간은 `Shutdown` 앞에 따로 줘야 합니다.

</details>

### 연습 2 — 로그에서 비밀 값 가리기

DB 접속 문자열을 담는 타입을 만들어 `slog` 로 찍으면 값이 가려지게 합니다. `fmt` 로 찍으면 어떻게 되는지도 봅니다.

<details>
<summary>풀이</summary>

`slog.LogValuer`(Go 1.21)를 구현합니다.

```go
type secret string

func (secret) LogValue() slog.Value { return slog.StringValue("[숨김]") }
```

```text
{"level":"INFO","msg":"DB 연결","dsn":"[숨김]"}
fmt 로 찍으면: postgres://app:example-password@db.example.com/app
```

출력을 고정하려고 시각 항목은 `ReplaceAttr` 로 뺐습니다. `LogValue` 는 `slog` 만 가립니다. `String()` 을 더하면 `%v`·`%s` 도 `[숨김]` 이 됐지만 `%#v` 는 여전히 원래 값을 찍었습니다. 가장 안전한 방법은 비밀 값을 로그 호출에 넘기지 않는 것입니다.

</details>

## 정리

운영 서버에 올리는 것은 정적 실행 파일 하나와 그것을 확인하는 `/healthz` 뿐입니다. 종료 신호에는 `Shutdown` 이 끝날 때까지 기다렸다 내려가고, 올리기 전에는 검사 스크립트 하나를 돌립니다. 지금 운영 중인 서버가 `ListenAndServe` 가 돌아오는 순간 끝나지 않는지 한 번 확인해 보십시오.

여기까지가 1단계입니다. 7편부터는 이 서비스를 여럿이 고칠 수 있게 패키지를 나누고 의존 방향을 정합니다.
