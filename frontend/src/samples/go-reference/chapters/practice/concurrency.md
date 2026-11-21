# Go 동시성: context 취소, 타임아웃, 고루틴 종료

고루틴을 시작하기 전에 누가 종료시키고 결과를 받는지 정한다. 요청에서 파생한 `context.Context`를 함수 첫 인자로 전달한다. `context.WithTimeout`으로 만든 취소 함수는 작업이 일찍 끝나도 호출한다.

```go
package worker

import (
    "context"
    "time"
)

func Wait(ctx context.Context, ready <-chan struct{}) error {
    ctx, cancel := context.WithTimeout(ctx, 2*time.Second)
    defer cancel()

    select {
    case <-ctx.Done():
        return ctx.Err()
    case <-ready:
        return nil
    }
}
```

채널 수신이나 송신이 막힐 수 있는 곳에서도 종료 신호를 받게 한다. 여러 고루틴이 같은 맵이나 카운터를 수정하면 잠금 범위를 정하고, 잠금을 잡은 채 오래 걸리는 I/O를 하지 않는다. 정상 완료와 취소 경로를 둘 다 확인하고 공유 상태 변경에는 `go test -race ./...`를 실행한다.

[Go 공식 Context 가이드](https://go.dev/blog/context)
