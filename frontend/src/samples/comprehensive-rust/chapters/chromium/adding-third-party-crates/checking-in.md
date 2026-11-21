# Chromium 소스 코드로 크레이트 확인

`git status`는 다음을 표시해야 합니다.

- `//third_party/rust/chromium_crates_io`의 크레이트 코드
- `//third_party/rust/<crate>/<version>`의 메타데이터(`BUILD.gn` 및 `README.chromium`)

후자의 위치에 `OWNERS` 파일도 추가하세요.

이 모든 것을 `Cargo.toml` 및 `gnrt_config.toml` 변경사항과 함께 Chromium 저장소에 배치해야 합니다.

**중요**: `git add -f`를 사용해야 합니다. 그러지 않으면 `.gitignore` 파일로 인해 일부 파일을 건너뛸 수 있습니다.

이렇게 하면 포용적이지 않은 언어로 인해 사전 제출 검사가 실패할 수도 있습니다. 이는 Rust 크레이트 데이터에는 git 브랜치 이름이 포함되는 경향이 있고, 많은 프로젝트에서는 여전히 포용적이지 않은 용어를 거기서 사용하기 때문입니다. 따라서 다음을 실행해야 할 수 있습니다.

``` shell
infra/update_inclusive_language_presubmit_exempt_dirs.sh > infra/inclusive_language_presubmit_exempt_dirs.txt
git add -p infra/inclusive_language_presubmit_exempt_dirs.txt # add whatever changes are yours
```
