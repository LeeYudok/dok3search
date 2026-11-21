# Generating `gn` Build Rules

크레이트를 다운로드한 후에는 다음과 같이 `BUILD.gn` 파일을 생성합니다.

``` shell
vpython3 tools/crates/run_gnrt.py -- gen
```

이제 `git status`를 실행합니다. 다음을 확인할 수 있습니다.

- `third_party/rust/chromium_crates_io/vendor`에 하나 이상의 새 크레이트 소스 코드가 있습니다.
- `third_party/rust/<crate name>/v<major semver version>`에 새 `BUILD.gn`이 하나 이상 있습니다.
- 적절한 `README.chromium`

The “major semver version” is a [Rust “semver” version number](https://doc.rust-lang.org/cargo/reference/semver.html).

특히 `third_party/rust`에서 생성된 항목을 자세히 살펴보세요.

> **발표자 노트**
>
> semver에 관해 좀 더 이야기합니다. 특히 Chromium에서는 호환되지 않는 크레이트 버전을 여러 개 허용하는 방식이 있는데 이는 권장되지 않지만 Cargo 생태계에서는 때때로 필요합니다.
