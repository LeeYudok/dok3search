# Go 성능 진단: 벤치마크와 pprof

검색 응답이나 변환 속도가 느리면, 먼저 문제가 재현되는 입력 크기와 처리 구간을 기록한다. 벤치마크에서 같은 입력으로 변경 전후를 비교하고 `ns/op`, `B/op`, `allocs/op`를 본다.

```sh
go test ./internal/search -run '^$' -bench . -benchmem -count 5
go test ./internal/search -run '^$' -bench . -cpuprofile /tmp/search.cpu
go tool pprof -top /tmp/search.cpu
```

`./internal/search`는 대상 패키지 경로로 바꾼다. CPU 시간이 길면 CPU 프로필을, 할당이나 메모리 사용량이 크면 힙 프로필을 확인한다. I/O 대기와 락 경합이 의심되면 블록 프로필이나 실행 추적을 검토한다. 수정 후 같은 입력으로 다시 측정하고 기능 테스트도 실행한다.

[Go 공식 진단 가이드](https://go.dev/doc/diagnostics)
