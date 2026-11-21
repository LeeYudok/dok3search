# Go 에러 처리: %w, errors.Is, errors.As

실패한 작업을 오류에 붙여 호출자에게 반환한다. 호출부에서 원인을 구분해야 하면 `fmt.Errorf`의 `%w`로 원본 오류를 감싼다. `errors.Is`는 특정 오류와 일치하는지 확인하고, `errors.As`는 지정한 타입을 꺼낸다.

```go
package config

import (
    "errors"
    "fmt"
    "io/fs"
    "os"
)

func Load(path string) ([]byte, error) {
    data, err := os.ReadFile(path)
    if err != nil {
        return nil, fmt.Errorf("load config %q: %w", path, err)
    }
    return data, nil
}

func IsMissing(err error) bool {
    return errors.Is(err, fs.ErrNotExist)
}
```

오류 한 건은 CLI 진입점이나 HTTP 경계처럼 최종 처리할 곳에서 기록한다. 외부 응답에는 내부 경로나 세션 정보가 담긴 원문을 그대로 내보내지 않는다. 실패를 기능으로 처리하는 코드에는 파일 없음처럼 실제 분기할 오류를 테스트에 넣는다.

[Go 공식 에러 처리 가이드](https://go.dev/blog/errors-go1.13)
