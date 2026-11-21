# 코드를 생성하는 스크립트 빌드

`ninja`가 파일 누락에 관한 불만을 제기하는 경우 `build.rs`에서 소스 코드 파일을 작성하는지 확인합니다.

그렇다면 [`gnrt_config.toml`](../configuring-gnrt-config-toml.md)을 수정하여 `build-script-outputs`를 크레이트에 추가합니다. 이것이 전이 종속 항목, 즉 Chromium 코드가 직접 종속되면 안 되는 종속 항목인 경우 `allow-first-party-usage=false`도 추가합니다. 이 파일에는 이미 다음과 같은 몇 가지 예가 있습니다.

``` toml
[crate.unicode-linebreak]
allow-first-party-usage = false
build-script-outputs = [ "tables.rs" ]
```

이제 [`gnrt.py -- gen`](../generating-gn-build-rules.md)을 다시 실행하여 `BUILD.gn` 파일을 다시 생성하고 이 특정 출력 파일이 후속 빌드 단계의 입력이라고 ninja에 알립니다.
