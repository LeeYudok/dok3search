# Go 프로젝트 구조: go.mod, cmd, internal

작은 실행 파일 하나면 루트에 `go.mod`와 `main.go`부터 둔다. 명령이 여럿이거나 실행 진입점과 구현을 분리할 이유가 생기면 다음처럼 나눈다.

```text
go.mod
cmd/bridge/main.go
internal/board/store.go
internal/board/store_test.go
internal/dispatch/dispatch.go
```

`main`은 설정을 읽고 의존성을 연결한 뒤 실행 결과를 종료 코드로 바꾼다. 보드 저장, 실행기 호출처럼 독립적으로 검증할 동작은 책임에 맞는 패키지에 둔다. `internal`은 상위 디렉터리 트리 밖에서의 import를 Go 도구가 제한한다. 여러 명령이 내부 코드를 공유해도 `go.mod` 하나로 관리할 수 있다.

기존 저장소에 기능을 추가할 때는 먼저 `go.mod`와 인접 코드의 import 방향을 읽는다. 호출하는 쪽에서 필요한 메서드만 가진 인터페이스를 정의한다. 실제로 독립 배포하거나 버전을 따로 관리할 때 별도 모듈을 고려한다.

[테스트 배치](testing.md) · [Go 공식 모듈 구성 가이드](https://go.dev/doc/modules/layout)
