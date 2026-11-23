# Go 강의 8편 — 버전은 요구한 최소 버전 가운데 가장 큰 것으로 정해진다

> 의존성 버전은 누가 정하고, 어떻게 고정하고, 어떻게 올립니까?

버전은 `go.mod` 의 `require` 가 적은 최소 버전들로 정해집니다. 빌드는 그 가운데 가장 큰 것을 쓰고(최소 버전 선택, MVS), 새 버전이 나와도 저절로 올리지 않습니다. 내용은 `go.sum` 의 해시가 고정해서, 같은 버전 번호로 다른 코드가 오면 빌드가 멈춥니다. 올리는 일은 사람이 `go get` 으로 합니다. 이 편은 로컬 파일 프록시에 가짜 모듈을 올려 인터넷 없이 이 규칙을 확인합니다.

> [그림] 주문서 두 장이 각각 최소 v1.1 과 최소 v1.2 를 요구하고, 배달 로봇은 둘 중 큰 v1.2 상자를 go.sum 봉인을 붙인 채 들고 있다. 선반에는 최신 v1.3 이 그대로 있고, 아래 칸에는 경로가 다른 v2 가 따로 놓여 있다

주문서가 `require`, 선반이 모듈 프록시입니다. 로봇은 요구 가운데 큰 v1.2 를 꺼내고, 아무도 요구하지 않은 v1.3 은 두고 갑니다.

## 실험은 로컬 파일 프록시로 한다

`go help goproxy` 에 따르면 `file:///` 주소도 모듈 프록시가 됩니다. 폴더에 `<모듈>/@v/list` 와 버전마다 `.info`·`.mod`·`.zip` 을 두면 됩니다. 올린 모듈은 이렇습니다.

| 모듈 | 버전 | 내용 |
|---|---|---|
| `example.com/textutil` | v1.0.0 ~ v1.2.0 | `Shorten(s, n)` 이 앞에서 n 글자를 남긴다 |
| 〃 | v1.3.0 | 바이트로 잘라 한글을 깨뜨리는 버그 |
| 〃 | v1.3.1 | 버그를 고치고 v1.3.0 을 `retract`(철회) |
| `example.com/textutil/v2` | v2.0.0 | 자른 뒤 `…` 를 붙인다 |
| `example.com/a` | v1.0.0 | textutil v1.1.0 이상을 요구 |
| `example.com/b` | v1.0.0 | textutil v1.2.0 이상을 요구 |

`GOPROXY=file://$PWD/proxy`, `GOSUMDB=off`(가짜 모듈이라), `GOTOOLCHAIN=local` 을 주고 모듈 캐시도 실험 폴더 안에 둡니다. 출력은 2026-11-18 에 맥(Apple M4 Pro), Go 1.26.2 로 돌렸습니다.

## go get 은 require 와 go.sum 두 곳에 적는다

모듈 `example.com/shop` 을 만들고 `main.go` 에서 textutil 을 import 한 뒤 받습니다.

```text
$ go get example.com/textutil@v1.1.0
go: downloading example.com/textutil v1.1.0
go: added example.com/textutil v1.1.0
$ cat go.sum
example.com/textutil v1.1.0 h1:GPbgEvfScSGJ5x8wvkZHDPgq+Fjjbzcn/qxSFivCMLQ=
example.com/textutil v1.1.0/go.mod h1:Gkda2Wj9h0n3VwqsvkFa4OToEEHd0BA5KtcjYOPC8+k=
```

`go.mod` 에는 `require` 한 줄이, `go.sum` 에는 두 줄이 생깁니다. 첫 줄은 모듈 zip 내용의 해시이고, `/go.mod` 로 끝나는 줄은 `go.mod` 파일만의 해시입니다. `go help module-auth` 에 따르면 `go` 명령은 zip 이나 `go.mod` 를 캐시에 받을 때마다 해시를 계산해 이 값과 비교합니다.

`go get` 직후의 줄에는 코드가 직접 import 하는데도 `// indirect` 가 붙어 있었고, `go mod tidy` 를 돌리자 지워졌습니다. tidy 가 소스를 읽어 `go.mod` 를 맞추기 때문입니다(`go help mod tidy`).

## 빌드는 요구한 최소 버전 가운데 가장 큰 것을 쓴다

textutil 을 직접 쓰지 않고 a 와 b 만 쓰는 프로그램 `app` 을 만들고 `go mod tidy` 를 돌렸습니다.

```text
$ cat go.mod
module example.com/app

go 1.26.2

require (
	example.com/a v1.0.0
	example.com/b v1.0.0
)

require example.com/textutil v1.2.0 // indirect
$ go mod graph        # go@·toolchain@ 줄은 뺐다
example.com/app example.com/a@v1.0.0
example.com/app example.com/b@v1.0.0
example.com/app example.com/textutil@v1.2.0
example.com/a@v1.0.0 example.com/textutil@v1.1.0
example.com/b@v1.0.0 example.com/textutil@v1.2.0
```

a 는 v1.1.0, b 는 v1.2.0 을 요구했고 빌드는 v1.2.0 을 씁니다. 프록시의 v1.3.1 은 아무도 요구하지 않아 고르지 않았습니다. 공식 문서(go.dev/ref/mod)는 이 방식을 최소 버전 선택(Minimal Version Selection, MVS)이라 부릅니다. 모듈마다 그래프가 요구한 최소 버전 가운데 가장 큰 것을 씁니다.

`// indirect` 는 메인 모듈이 직접 import 하지 않는 모듈입니다. 이 사이트의 Go 서버는 PostgreSQL 드라이버 pgx 하나를 직접 쓰고, pgx 가 데려온 다섯 개가 `// indirect` 로 적혀 있습니다. 왜 들어왔는지는 `go mod why -m example.com/textutil` 이 `example.com/app` → `example.com/a` → `example.com/textutil` 경로로 보여 줍니다.

## 올리는 일은 사람이 명시한다

새 버전이 있는지는 `go list -m -u all` 로 봅니다. 대괄호 안이 올릴 수 있는 버전입니다.

```text
$ go list -m -u all
example.com/shop
example.com/a v1.0.0
example.com/b v1.0.0
example.com/textutil v1.2.0 [v1.3.1]
$ go list -m -versions example.com/textutil
example.com/textutil v1.0.0 v1.1.0 v1.2.0 v1.3.1
```

목록에서 v1.3.0 이 빠졌습니다. v1.3.1 의 `go.mod` 에 `retract v1.3.0 // 한글을 바이트 단위로 잘라 깨뜨린다` 를 적었기 때문이고, `-retracted` 를 붙여야 보입니다. 직접 고르면 경고와 함께 받아집니다.

```text
$ go get example.com/textutil@v1.3.0
go: downloading example.com/textutil v1.3.0
go: warning: example.com/textutil@v1.3.0: retracted by module author: 한글을 바이트 단위로 잘라 깨뜨린다
go: to switch to the latest unretracted version, run:
	go get example.com/textutil@latest
go: upgraded example.com/textutil v1.2.0 => v1.3.0
$ go run .
[a] 가� [b] 가� v1.3.0
$ go get example.com/textutil@latest
go: downloading example.com/textutil v1.3.1
go: upgraded example.com/textutil v1.3.0 => v1.3.1
```

철회는 버전을 지우지 않고 새로 고르는 사람만 피하게 합니다. 한꺼번에 올릴 때 쓰는 `-u` 는 의존성을 새 마이너·패치 버전으로, `-u=patch` 는 패치까지만 올립니다(`go help get`).

## 주 버전이 바뀌면 import 경로가 바뀐다

호환되지 않는 v2 는 모듈 경로 끝에 `/v2` 를 붙인 다른 모듈입니다. 그래서 한 빌드에 v1 과 v2 가 함께 들어갑니다.

```go
import (
	"example.com/textutil"
	textutilv2 "example.com/textutil/v2"
)
```

```text
$ go run .
[b] 가나다라마
v1.3.1 가나다라마
v2.0.0 가나다라마…
```

v1 을 쓰는 의존성은 두고 내 코드만 v2 로 옮길 수 있습니다. 공식 문서는 같은 주 버전 안에서 호환을 깨지 않는다고 약속합니다(import compatibility rule). v1.1.0 을 요구한 a 에 v1.2.0 을 줘도 되는 근거입니다.

## go.sum 은 같은 버전의 다른 내용을 막는다

프록시를 복사해(`$EVIL`) textutil v1.3.1 의 zip 만 고친 코드로 바꾸고, 빈 캐시로 빌드했습니다. 누가 같은 버전 번호로 다른 코드를 올린 상황입니다.

```text
$ GOPROXY=file://$EVIL GOMODCACHE=$EMPTY go build ./...
go: downloading example.com/textutil v1.3.1
go: downloading example.com/textutil/v2 v2.0.0
go: downloading example.com/b v1.0.0
verifying example.com/textutil@v1.3.1: checksum mismatch
	downloaded: h1:Kgo91M6c2WHmVt11Pw7G4Dy+ucQPcA62SbeW4f1FXt8=
	go.sum:     h1:Y8DzPmcZ+2eFfJl8k9jNRrlMKYcFelOs7ra/yn8XCnI=

SECURITY ERROR
This download does NOT match an earlier download recorded in go.sum.
…
```

`go.sum` 은 잠금 파일이 아닙니다. 버전은 `go.mod` 가 정하고, `go.sum` 은 내용이 처음 받은 것과 같은지만 봅니다. `GOSUMDB` 설정에 따라 공개 체크섬 데이터베이스와도 대조합니다(`go help module-auth`). 캐시 상태는 `go mod verify` 가 `all modules verified` 로 답합니다.

## 빌드 명령은 go.mod 를 고치지 않는다

`go.mod` 에 없는 모듈을 import 한 채 빌드하면 멈춥니다. vendor 폴더가 없으면 `-mod=readonly` 로 동작하기 때문입니다(`go help build`). 환경의 `GOFLAGS` 에 `-mod=mod` 가 있으면 빌드가 `go.mod` 를 조용히 고칩니다.

```text
$ go build -o /dev/null .
main.go:7:2: no required module provides package example.com/a; to add it:
	go get example.com/a
$ GOFLAGS=-mod=mod go build -o /dev/null .
go: finding module for package example.com/a
go: found example.com/a in example.com/a v1.0.0
$ diff ../../shop/go.mod go.mod
10c10,13
< require example.com/textutil/v2 v2.0.0
---
> require (
> 	example.com/a v1.0.0
> 	example.com/textutil/v2 v2.0.0
> )
```

그래서 제 한국어 글 검사기의 빌드 래퍼는 `GOFLAGS=-mod=readonly GOWORK=off GOTOOLCHAIN=local` 을 못 박습니다. 의존성을 바꾸는 일은 `go get`·`go mod tidy` 로 따로 합니다.

## replace 와 go.work 는 로컬 코드로 바꿔 끼운다

의존성의 버그를 내 PC 에서 고쳐 보고 싶을 때 씁니다. 버전 문자열만 `v1.3.1-local` 로 바꾼 사본을 옆 폴더에 두었습니다.

```text
$ go mod edit -replace example.com/textutil=../textutil-local
$ go run .
[b] 가나다라마
v1.3.1-local 가나다라마
v2.0.0 가나다라마…
```

`replace` 는 빌드 전체에 걸려 b 도 로컬 사본을 씁니다. 커밋에 섞이면 다른 사람의 PC 에서는 `../textutil-local` 이 없어 빌드가 깨집니다. `go.work` 는 `go.mod` 를 건드리지 않고 같은 일을 합니다.

```text
$ cd .. && go work init ./shop ./textutil-local && cd shop
$ go env GOWORK
…/08-modules-and-versions/go.work
$ go run . | sed -n 2p
v1.3.1-local 가나다라마
$ GOWORK=off go run . | sed -n 2p
v1.3.1 가나다라마
```

`shop` 폴더는 그대로인데 결과가 바뀌었습니다. `go` 명령은 현재 폴더부터 위로 `go.work` 를 찾기 때문입니다(`go help environment`). 상위 폴더에 남은 `go.work` 하나가 빌드를 바꾸니 검사기 래퍼는 `GOWORK=off` 를 줍니다.

## vendor 와 의존성 0개는 오프라인에서 빌드된다

`go mod vendor` 는 쓰는 패키지의 소스를 `vendor/` 로 복사하고 버전을 `vendor/modules.txt` 에 적습니다. `vendor/` 가 있고 `go` 줄이 1.14 이상이면 `-mod=vendor` 로 동작해서(`go help build`), `GOPROXY=off` 와 빈 캐시로도 빌드됐습니다. 의존성이 0개인 1편의 `nextday`, 제 데이터 수집 도구와 글 검사기도 같은 조건에서 빌드됐습니다. 폐쇄망에 들고 가도 되는 이유입니다.

`go` 줄은 요구이고 `toolchain` 줄은 바람입니다. `GOTOOLCHAIN=local` 에서 `go 1.99` 는 `go.mod requires go >= 1.99` 로 거절됐고, `go 1.22` 에 `toolchain go1.99.0` 을 더한 모듈은 그대로 돌았습니다. 기본값 `auto` 에서는 go1.99.0 을 받으려다 `toolchain not available` 로 멈췄습니다.

## 한 단계 더 — MVS 는 잠금 파일 없이 같은 결과를 낸다

같은 조건(1.1 이상, 1.2 이상, 최신 1.3.1)을 세 도구에 줬습니다. npm 은 함께 설치된 semver 라이브러리의 `maxSatisfying` 으로, pip 은 빈 휠을 만들어 `pip install --dry-run --no-index` 로 봤습니다.

| 도구 | 조건 | 고른 버전 | 1.4.0 을 더 올린 뒤 |
|---|---|---|---|
| Go (MVS) | `require v1.1.0`, `require v1.2.0` | v1.2.0 | v1.2.0 (`-u` 에 `[v1.4.0]` 만 보인다) |
| npm 의 semver 7.8.6 | `^1.1.0 ^1.2.0` | 1.3.1 | 1.4.0 |
| pip 25.0.1 | `>=1.1`, `>=1.2` | 1.3.1 | 1.4.0 |

npm 과 pip 은 범위를 만족하는 가장 높은 버전을 고릅니다. 그래서 설치하는 날에 따라 결과가 달라지고, 잠금 파일로 결과를 고정합니다. MVS 는 `go.mod` 파일들만 보고 정합니다. 프록시에 v1.4.0 을 더 올려도 `app` 은 v1.2.0 그대로였습니다. 같은 `go.mod` 면 언제 빌드해도 같은 버전이 나옵니다.

대가도 있습니다. 버그 수정 버전도 저절로 들어오지 않습니다. 정기적으로 `go list -m -u all` 을 보고, 알려진 취약점은 Go 팀의 `govulncheck`(golang.org/x/vuln)로 점검합니다. 이 실험은 오프라인이라 govulncheck 는 돌리지 않았습니다.

`h1:` 해시는 zip 의 항목 이름과 내용으로 계산됩니다. 처음 만든 zip 에 디렉터리 항목이 섞였더니, 빌드는 됐는데 `go mod verify` 가 `dir has been modified` 로 실패했습니다. 풀어 둔 폴더와 항목 목록이 달랐기 때문입니다. 프록시를 직접 만든다면 zip 에 파일만 넣습니다.

## 면접에서 이렇게 묻는다

**Q1. Go 는 의존성 버전을 어떻게 고릅니까?**
최소 버전 선택(MVS)입니다. 그래프의 `go.mod` 들이 요구한 최소 버전 가운데 모듈마다 가장 큰 것을 씁니다. 새 버전이 나와도 요구가 바뀌기 전에는 그대로라 잠금 파일 없이 재현됩니다.

**Q2. go.sum 은 package-lock.json 같은 잠금 파일입니까?**
아닙니다. 버전은 `go.mod` 가 정하고, `go.sum` 은 zip 과 `go.mod` 의 해시로 내용이 같은지만 봅니다. 같은 버전의 다른 내용이 오면 `SECURITY ERROR` 로 멈춥니다.

**Q3. `// indirect` 는 무엇입니까?**
메인 모듈이 직접 import 하지 않는 모듈입니다. 직접 쓰는 의존성이 데려온 모듈도 지금 Go 는 `go.mod` 에 적고 이 표시를 붙입니다. 표시는 `go mod tidy` 가 소스를 보고 맞춥니다.

**Q4. v2 를 내면 왜 import 경로가 바뀝니까?**
MVS 는 큰 버전이 작은 버전을 대신할 수 있다고 전제합니다. 호환이 깨진 버전은 경로를 바꿔 다른 모듈로 둬야 이 전제가 지켜지고, v1 과 v2 가 한 빌드에 함께 들어가 차례로 옮길 수 있습니다.

**Q5. replace 와 go.work 는 언제 쓰고 무엇을 조심합니까?**
의존성을 로컬 코드로 바꿔 시험할 때 씁니다. `replace` 가 커밋에 섞이거나 상위 폴더에 `go.work` 가 남으면 빌드가 깨지거나 결과가 바뀝니다. CI 는 `GOWORK=off`, `-mod=readonly` 로 막고, 취약점은 `govulncheck` 로 점검한다고 덧붙입니다.

## 연습

### 연습 1 — 요구를 빼면 버전이 내려가는가

`app` 에서 b 를 쓰지 않게 고치고 `go mod tidy` 를 돌립니다. textutil 은 a 가 요구한 v1.1.0 으로 내려갈까요?

<details>
<summary>풀이</summary>

```text
$ go mod tidy && cat go.mod
module example.com/app

go 1.26.2

require example.com/a v1.0.0

require example.com/textutil v1.2.0 // indirect
$ go get example.com/textutil@v1.1.0
go: downgraded example.com/textutil v1.2.0 => v1.1.0
```

내려가지 않습니다. 메인 모듈의 `go.mod` 가 이미 v1.2.0 을 요구하고 있어서 그 줄도 MVS 의 입력입니다. 내리려면 `go get …@v1.1.0` 으로 명시합니다. 버전은 올리든 내리든 사람이 정합니다.

</details>

### 연습 2 — CI 에서 모듈 상태를 검사하기

`go.mod`·`go.sum` 이 정리되지 않았거나 캐시가 손상됐으면 실패하는 명령을 만듭니다.

<details>
<summary>풀이</summary>

```bash
go mod tidy -diff && go mod verify && GOFLAGS=-mod=readonly GOWORK=off go build ./...
```

`go help mod tidy` 에 따르면 `-diff` 는 파일을 고치지 않고 바뀔 내용을 diff 로 찍고, 바뀔 것이 있으면 0 이 아닌 코드로 끝납니다. a 를 더 쓰지 않는 `shop` 에서 돌리면 `example.com/a v1.0.0` 줄을 지우는 diff 를 찍고 1 로 끝났습니다.

정리한 뒤에는 `go mod verify` 가 `all modules verified` 를 찍고 빌드가 통과합니다.

</details>

## 정리

버전은 `go.mod` 들이 요구한 최소 버전 가운데 가장 큰 것으로 정해지고, `go.sum` 이 그 내용을 고정합니다. 새 버전은 저절로 들어오지 않으니 `go list -m -u all` 로 보고 `go get` 으로 올립니다. 빌드할 때는 `-mod=readonly` 와 `GOWORK=off` 로 이 결정이 바뀌지 않게 합니다.

9편에서는 에러 설계를 다룹니다. 계층마다 에러를 감싸고 기록은 한 곳에서만 하는 흐름을 만듭니다.
