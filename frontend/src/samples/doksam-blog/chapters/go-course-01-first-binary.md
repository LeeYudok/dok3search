# Go 강의 1편 — 모듈 하나로 시작해 실행 파일 하나로 끝난다

> Go 프로젝트는 무엇으로 시작하고, 끝나면 무엇이 남습니까?

`go.mod` 파일 하나로 시작하고, 실행 파일 하나가 남습니다. 형식 맞추기(`gofmt`), 정적 검사(`go vet`), 테스트(`go test`), 빌드(`go build`)가 모두 Go 배포판에 들어 있어 도구를 고를 단계가 없습니다. 이 편에서는 다음 평일을 알려 주는 명령줄 도구 `nextday` 를 만듭니다. 네 명령을 돌리고, 맥에서 리눅스·윈도우용 실행 파일까지 만듭니다.

> [그림] 왼쪽 폴더에 go.mod 와 main.go 가 꽂혀 있고, 가운데 go build 기계 앞에 fmt·vet·test 표시등이 켜져 있다. 배달 로봇이 기계에서 나온 상자 하나를 들고 달린다

그림의 배달 로봇이 연재의 주인공입니다. 들고 있는 상자가 실행 파일이고, 옆의 상자는 같은 코드로 만든 다른 운영체제용입니다.

연재는 4단계 24편이고, 왜 Go 인지는 [Go 와 Rust 를 쓰면 좋은 점](https://blog.doksam.com/blog/why-go-and-rust/)에 적었습니다. 출력은 2026-11-18 에 맥(Apple M4 Pro)과 Go 1.26.2 로 돌린 결과입니다.

## 설치하면 도구까지 함께 깔린다

공식 설치 파일은 go.dev/dl 에 있고, 맥은 Homebrew(`brew install go`)로도 깝니다.

```bash
$ go version
go version go1.26.2 darwin/arm64
```

실행 파일은 `go` 와 `gofmt` 둘뿐이고, 테스트·정적 검사·의존성 관리는 `go` 의 하위 명령입니다. 다른 언어에서 따로 고르던 도구와 견주면 이렇습니다.

| 할 일 | Python | Node | Java | Go |
|---|---|---|---|---|
| 프로젝트·의존성 | `pyproject.toml` + pip | `package.json` + npm | `pom.xml` + Maven | `go.mod` + `go` |
| 형식 | black 등 별도 | prettier 등 별도 | 별도 도구 | `gofmt` (함께 설치) |
| 정적 검사 | ruff·mypy 등 | eslint·tsc | 컴파일러 + 별도 도구 | 컴파일러 + `go vet` |
| 테스트 | pytest | jest 등 | JUnit | `go test` |
| 배포물 | 소스 + 인터프리터 | 소스 + `node_modules` | JAR + JVM | 실행 파일 하나 |

형식이 하나뿐이라 에이전트가 고쳐도 들여쓰기로 diff 가 더러워지지 않습니다.

## 모듈은 go.mod 하나, 패키지는 폴더 하나다

```bash
$ mkdir nextday && cd nextday
$ go mod init example.com/nextday
go: creating new go.mod: module example.com/nextday
$ cat go.mod
module example.com/nextday

go 1.26.2
```

인자는 모듈 경로이고 보통 저장소 주소를 씁니다. `go` 줄은 최소 Go 버전입니다. 폴더는 제 데이터 수집 도구가 쓰는 배치를 줄여 둡니다.

```text
nextday/
├── go.mod
├── cmd/nextday/main.go        # 실행 파일마다 폴더 하나
├── cmd/nextday/main_test.go
└── internal/workday/
    ├── workday.go             # 모듈 안에서만 쓰는 코드
    └── workday_test.go
```

- 패키지는 폴더 단위입니다. 이 사이트의 Go 서버는 `main.go`·`contact.go`·`static.go` 세 파일이 한 패키지 `main` 입니다.
- import 경로는 모듈 경로 + 폴더입니다. `example.com/nextday/internal/workday` 처럼 부릅니다.
- 대문자로 시작하는 이름만 패키지 밖에서 보입니다. `public`·`private` 키워드는 없고, 접근의 단위는 클래스가 아니라 패키지입니다.

`internal` 은 관례가 아니라 `go` 명령이 강제하는 규칙입니다. 옆에 다른 모듈을 만들어 `replace` 로 이 모듈을 붙이고 `internal/workday` 를 import 하면 빌드가 거절됩니다.

```text
$ go build .
package example.com/other
	main.go:7:2: use of internal package example.com/nextday/internal/workday not allowed
```

밖에 약속한 적이 없는 코드라서 이름과 시그니처를 마음대로 바꿔도 됩니다.

## main 은 종료 코드만 넘기고 일은 run 이 한다

```go
// Package workday 는 주말을 건너뛴 날짜를 계산한다.
package workday

import "time"

// Next 는 d 다음의 가장 가까운 평일을 돌려준다.
func Next(d time.Time) time.Time {
	for {
		d = d.AddDate(0, 0, 1)
		if d.Weekday() != time.Saturday && d.Weekday() != time.Sunday {
			return d
		}
	}
}
```

Go 의 반복문은 `for` 하나이고, 조건 없는 `for` 가 `while (true)` 입니다. 실행 파일 쪽은 이렇습니다.

```go
package main

import (
	"flag"
	"fmt"
	"io"
	"os"
	"time"

	"example.com/nextday/internal/workday"
)

func main() { os.Exit(run(os.Args[1:], os.Stdout, os.Stderr)) }

func run(args []string, stdout, stderr io.Writer) int {
	fs := flag.NewFlagSet("nextday", flag.ContinueOnError)
	fs.SetOutput(stderr)
	date := fs.String("date", "", "기준 날짜 YYYY-MM-DD (비우면 오늘)")
	if err := fs.Parse(args); err != nil {
		return 2
	}
	day := time.Now()
	if *date != "" {
		t, err := time.Parse("2006-01-02", *date)
		if err != nil {
			fmt.Fprintln(stderr, "-date 형식은 YYYY-MM-DD 다")
			return 2
		}
		day = t
	}
	next := workday.Next(day)
	fmt.Fprintln(stdout, next.Format("2006-01-02"), next.Weekday())
	return 0
}
```

`main` 은 한 줄이고 일은 `run` 이 합니다. 제 데이터 수집 도구와 예약 실행기도 이 모양입니다. 인자와 출력을 매개변수로 받으니 테스트가 프로세스 없이 `run` 을 부릅니다. 종료 코드는 반환값이 됩니다. 0 은 성공, 1 은 실패, 2 는 사용법 오류입니다. 제 데이터 수집 도구는 받은 자료의 날짜가 요청과 다르면 3 으로 끝나고, 부르는 스크립트는 출력 문장 대신 이 숫자로 다음 단계를 정합니다.

날짜 형식 `"2006-01-02"` 는 `%Y-%m-%d` 같은 기호 대신 기준 시각을 원하는 모양으로 적은 것입니다. `go doc time.Layout` 은 기준 시각을 `01/02 03:04:05PM '06 -0700` 으로 설명합니다. 같은 값이 Go 1.20 부터 `time.DateOnly` 상수로도 있습니다.

## go run 은 시험할 때, go build 는 남길 때 쓴다

```bash
$ go run ./cmd/nextday -date 2026-11-20
2026-11-23 Monday
$ go run ./cmd/nextday -date 11/20
-date 형식은 YYYY-MM-DD 다
exit status 2
$ echo $?
1
$ go build -o bin/nextday ./cmd/nextday
$ ./bin/nextday -date 11/20; echo $?
-date 형식은 YYYY-MM-DD 다
2
```

프로그램이 2 로 끝나도 `go run` 자신은 1 로 끝납니다(`go help run` 에도 적혀 있습니다). 종료 코드를 보는 스크립트에는 `go build` 로 만든 파일을 씁니다.

## gofmt 와 go vet 은 컴파일러가 보지 않는 것을 본다

형식이 틀렸고 `%s` 자리에 정수를 넘긴 파일입니다. 컴파일되고 실행도 됩니다. 결과만 틀립니다.

```go
package main
import "fmt"
func main(){
  count:=3
     fmt.Printf("%s개 남았다\n",count)
}
```

```bash
$ go run .
%!s(int=3)개 남았다
$ gofmt -l .
main.go
$ go vet ./...
main.go:5:18: fmt.Printf format %s has arg count of wrong type int
```

`gofmt -l` 은 형식이 다른 파일을 찍고 `-w` 는 고칩니다. `go vet` 은 컴파일은 되지만 틀렸을 가능성이 높은 코드를 찾습니다. `go test` 도 printf 를 포함한 vet 검사 열 가지를 먼저 돌려서(`go help test`), 위 파일 옆에 빈 테스트를 두면 `[build failed]` 로 멈춥니다.

## 테스트는 _test.go 파일에 둔다

```go
func TestNextSkipsWeekend(t *testing.T) {
	fri := time.Date(2026, 11, 20, 0, 0, 0, 0, time.UTC) // 금요일
	got := Next(fri).Format("2006-01-02")
	if got != "2026-11-23" {
		t.Errorf("Next(금요일) = %s, 원하는 값 2026-11-23", got)
	}
}
```

```bash
$ go test ./...
ok  	example.com/nextday/cmd/nextday	0.387s
ok  	example.com/nextday/internal/workday	0.789s
```

테스트 함수는 `Test` 로 시작해 `*testing.T` 를 받고, 실패 메시지에 받은 값과 원하는 값을 함께 씁니다. `run` 의 종료 코드도 같은 방식으로 시험합니다.

## 다른 운영체제용 실행 파일은 환경변수 두 개로 만든다

```bash
$ GOOS=linux GOARCH=amd64 go build -o bin/nextday-linux-amd64 ./cmd/nextday
$ GOOS=windows GOARCH=amd64 go build -o bin/nextday-windows-amd64.exe ./cmd/nextday
$ file bin/nextday-linux-amd64 bin/nextday-windows-amd64.exe
bin/nextday-linux-amd64:       ELF 64-bit LSB executable, x86-64, version 1 (SYSV), statically linked, Go BuildID=…, BuildID[sha1]=…, with debug_info, not stripped
bin/nextday-windows-amd64.exe: PE32+ executable (console) x86-64, for MS Windows
```

교차 컴파일러를 따로 깔지 않습니다. 리눅스용은 정적 링크로 나와 받는 서버에 깔 라이브러리가 없습니다. `go tool dist list` 는 47가지 운영체제·아키텍처 조합을 보여 줍니다.

제 한국어 글 검사기는 릴리스마다 세 플랫폼용을 `CGO_ENABLED=0 go build -trimpath -buildvcs=false -ldflags='-s -w'` 로 만듭니다. `CGO_ENABLED=0` 은 C 를 부르는 기능(cgo)을, `-trimpath` 는 빌드한 PC 의 경로를, `-buildvcs=false` 는 git 커밋 정보를, `-s -w` 는 심볼 표와 디버그 정보(DWARF)를 뺍니다.

| 대상 | 옵션 | 크기(바이트) |
|---|---|---:|
| darwin/arm64 (이 맥) | 없음 | 2,667,794 |
| windows/amd64 | 없음 | 2,751,488 |
| linux/amd64 | 없음 | 2,674,092 |
| linux/amd64 | `CGO_ENABLED=0 -trimpath -ldflags='-s -w'` | 1,765,538 |
| linux/arm64 | 같음 | 1,704,098 |

리눅스 amd64 는 약 34% 줄었습니다. `CGO_ENABLED=0` 을 적는 이유는 기본값이 빌드하는 곳마다 달라서입니다(`go doc cmd/cgo`). 이 맥의 네이티브 빌드는 `CGO_ENABLED=1`, 리눅스 교차 빌드는 `CGO_ENABLED=0` 으로 기록됐습니다.

## 한 단계 더 — go build 가 안에서 하는 일

**빌드 결과는 캐시에 남습니다.** 빈 캐시 폴더(`GOCACHE`)를 주고 `go build -x` 로 보면 compile 이 63번, link 가 1번 돕니다. 표준 라이브러리까지 `nextday` 가 기대는 패키지를 모두 컴파일하기 때문입니다. 시간은 한 번씩 쟀습니다(2026-11-18, Apple M4 Pro, Go 1.26.2).

| 상황 | 걸린 시간 |
|---|---:|
| 빈 캐시에서 처음 빌드 | 1.56초 |
| 바꾼 것 없이 다시 빌드 | 0.09초 |
| `workday.go` 에 주석 한 줄 더하고 빌드 | 0.10초 |

`go help cache` 는 캐시가 소스, 컴파일러, 옵션의 변화를 반영한다고 적습니다. 바뀐 패키지만 다시 컴파일해서 고치고 빌드하는 주기가 1초 안쪽입니다.

**실행 파일에는 빌드 정보가 들어 있습니다.** `go version -m` 으로 보면 줄인 리눅스 바이너리에 `-trimpath=true`, `CGO_ENABLED=0`, `GOOS=linux` 같은 옵션이 남아 있습니다. 프로그램 안에서는 `runtime/debug.ReadBuildInfo` 로 같은 정보를 읽고, 6편에서 헬스 엔드포인트에 버전을 실을 때 씁니다. 다만 `-trimpath` 를 붙인 빌드에는 `-ldflags` 가 기록되지 않았습니다. 빌드 명령을 스크립트로 저장소에 남기는 이유입니다.

**`-trimpath` 는 경로 누출을 막습니다.** 옵션 없이 만든 바이너리에서는 `strings` 로 이 PC 의 작업 폴더 경로가 2건 나왔고, 붙이면 0건이었습니다.

**`-s -w` 로 줄여도 패닉 위치는 남습니다.** 0으로 나누는 프로그램을 `-trimpath -ldflags='-s -w'` 로 빌드해 돌렸습니다.

```text
panic: runtime error: integer divide by zero

goroutine 1 [running]:
main.divide(...)
	example.com/strip/main.go:3
main.main()
	example.com/strip/main.go:5 +0x20
```

런타임은 스택을 풀 때 DWARF 가 아니라 별도 표(런타임 소스의 `pclntable`)에서 함수와 줄을 찾습니다. 그래서 운영 바이너리를 줄여도 패닉 로그의 위치는 잃지 않습니다.

## 면접에서 이렇게 묻는다

**Q1. GOPATH 방식과 모듈 방식은 무엇이 다릅니까?**
모듈 방식은 `go.mod` 가 의존성 버전을 적어서 코드를 어느 폴더에 두든 빌드됩니다. `go help gopath` 에 따르면 모듈을 쓸 때 GOPATH 는 import 해석에 쓰이지 않고, 내려받은 소스를 두는 자리(`GOPATH/pkg/mod`)로만 남습니다.

**Q2. `internal` 폴더는 무엇을 막습니까?**
그 부모 폴더 밖의 코드가 import 하는 것을 `go` 명령이 빌드에서 거절합니다. 공개하지 않은 코드라 자유롭게 바꿀 수 있으니, 공개 API 표면을 줄이는 도구라고 답합니다.

**Q3. 맥에서 리눅스 서버용 바이너리를 어떻게 만들고, `CGO_ENABLED=0` 은 왜 붙입니까?**
`GOOS=linux GOARCH=amd64 go build` 입니다. cgo 기본값이 네이티브 빌드와 교차 빌드에서 달라서, 어디서 빌드해도 같은 정적 바이너리를 얻으려고 명시합니다. 실제 값은 `go version -m` 으로 확인합니다.

**Q4. `go vet` 은 컴파일러와 무엇이 다릅니까?**
컴파일러는 문법과 타입을 보고, `go vet` 은 컴파일은 되지만 틀렸을 가능성이 높은 코드를 봅니다. `Printf` 형식과 인자 타입 불일치가 대표입니다. `go test` 가 그중 열 가지를 먼저 돌린다는 점도 말합니다.

## 연습

### 연습 1 — 휴일도 건너뛰게 하기

`-holidays 2026-12-25,2027-01-01` 처럼 받은 날짜도 건너뛰게 고칩니다. 2026-12-24(목) 다음 평일은 2026-12-28(월)이어야 하고, 형식이 틀린 날짜는 종료 코드 2 로 거절합니다.

<details>
<summary>풀이</summary>

휴일을 날짜 문자열 집합(`map[string]bool`)으로 받습니다. 제 예약 실행기가 휴장일을 다루는 방식과 같습니다.

`Next` 는 `holidays map[string]bool` 를 더 받아 조건에 `!holidays[d.Format("2006-01-02")]` 를 더합니다. 목록은 이렇게 읽습니다.

```go
func ParseHolidays(s string) (map[string]bool, error) {
	set := map[string]bool{}
	for _, f := range strings.Split(s, ",") {
		f = strings.TrimSpace(f)
		if f == "" {
			continue
		}
		if _, err := time.Parse("2006-01-02", f); err != nil {
			return nil, fmt.Errorf("휴일 %q: %w", f, err)
		}
		set[f] = true
	}
	return set, nil
}
```

`run` 에 `-holidays` 플래그를 더하고, `ParseHolidays` 가 에러를 내면 2 를 돌려줍니다.

```bash
$ go run ./cmd/nextday -date 2026-12-24 -holidays 2026-12-25
2026-12-28 Monday
$ go run ./cmd/nextday -date 2026-12-24 -holidays 2026-12-25,12/31
휴일 "12/31": parsing time "12/31" as "2006-01-02": cannot parse "12/31" as "2006"
exit status 2
```

`Next(d, nil)` 도 됩니다. nil 맵은 읽으면 제로 값(`false`)을 돌려주고, 쓰면 패닉이 납니다. 맵은 2편에서 봅니다.

</details>

### 연습 2 — os.Exit 는 defer 를 건너뛰는가

아래 프로그램을 돌려 보고, `main` 에서 바로 `os.Exit` 를 부르지 않고 `run` 에 일을 맡기는 이유를 설명합니다.

```go
func main() {
	defer fmt.Println("정리: 임시 파일을 지운다")
	fmt.Println("일을 마쳤다")
	os.Exit(1)
}
```

<details>
<summary>풀이</summary>

```bash
$ go run .
일을 마쳤다
exit status 1
```

"정리" 줄이 나오지 않습니다. `os.Exit` 는 그 자리에서 끝내고 걸어 둔 `defer` 를 실행하지 않습니다. 파일 닫기와 임시 파일 지우기를 `defer` 로 걸어 두면 모두 빠집니다. `func main() { os.Exit(run(...)) }` 모양에서는 `run` 이 반환하며 안의 `defer` 를 모두 돌린 뒤 `os.Exit` 가 불립니다.

</details>

## 정리

Go 프로젝트는 `go.mod` 하나로 시작해 실행 파일 하나로 끝납니다. 형식·검사·테스트·빌드 도구가 `go` 명령 안에 있고, 다른 운영체제용 빌드도 환경변수 두 개면 됩니다.

2편에서는 이 실행 파일을 채울 타입을 다룹니다. Go 는 값을 넘길 때 복사합니다. 언제 복사하고 언제 포인터로 공유하는지 구조체와 메서드로 확인합니다.
