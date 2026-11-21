# Go 테스트 작성: _test.go, 테이블 테스트, E2E

`store.go`의 단위 테스트는 같은 폴더의 `store_test.go`에 둔다. `package board`는 비공개 함수에 접근하고, `package board_test`는 패키지를 import해 공개 API로 동작을 검증한다. 일반 빌드에서는 `_test.go`가 빠진다.

다음은 `board` 패키지 밖에서 저장소의 동작만 확인하는 예다. 예제의 import 경로와 공개 함수는 실제 모듈에 맞춘다.

```go
package board_test

import (
    "testing"

    "example.com/bridge/internal/board"
)

func TestValidName(t *testing.T) {
    cases := []struct {
        name  string
        input string
        want  bool
    }{
        {"normal", "backend", true},
        {"empty", "", false},
    }
    for _, tc := range cases {
        t.Run(tc.name, func(t *testing.T) {
            if got := board.ValidName(tc.input); got != tc.want {
                t.Errorf("ValidName(%q) = %v; want %v", tc.input, got, tc.want)
            }
        })
    }
}
```

파일 테스트에는 `t.TempDir()`를 쓴다. HTTP 테스트는 `httptest`로 라우터와 미들웨어를 거쳐 실제 응답을 검사한다. `go test ./...`로 모듈 전체를 확인하고, 공유 상태나 고루틴을 바꿨다면 지원 환경에서 `go test -race ./...`도 실행한다. 바이너리를 실행해 여러 패키지와 프로세스를 함께 검증하는 시나리오가 늘면 `tests/e2e/`를 둘 수 있다.

[프로젝트 구조](project-layout.md) · [Go 공식 testing 문서](https://pkg.go.dev/testing#hdr-Overview)
