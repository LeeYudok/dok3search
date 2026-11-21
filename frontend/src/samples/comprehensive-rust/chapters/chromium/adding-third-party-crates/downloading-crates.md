# 크레이트 다운로드

`gnrt`라는 도구는 크레이트를 다운로드하는 방법과 `BUILD.gn` 규칙을 생성하는 방법을 알고 있습니다.

시작하려면 다음과 같이 원하는 크레이트를 다운로드합니다.

``` shell
cd chromium/src
vpython3 tools/crates/run_gnrt.py -- vendor
```

> `gnrt` 도구는 Chromium 소스 코드의 일부이지만 이 명령어를 실행하면 `crates.io`에서 종속 항목을 다운로드하고 실행하게 됩니다. 이 보안 관련 결정에 관해서는 [이전 섹션](../cargo.md)을 참고하세요.

이 `vendor` 명령어는 다음을 다운로드할 수 있습니다.

- Your crate
- 직접 및 임시 종속 항목
- Chromium에서 필요한 전체 크레이트 세트를 해결하기 위해 `cargo`에서 요구하는 다른 크레이트의 새 버전입니다.

Chromium은 `//third_party/rust/chromium_crates_io/patches`에 보관되는 일부 크레이트의 패치를 유지관리합니다. 이는 자동으로 다시 적용되지만 패치에 실패하면 직접 조치를 취해야 할 수도 있습니다.
