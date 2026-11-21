# 서드 파티 크레이트 감사

새 라이브러리 추가에는 Chromium의 표준 [정책](https://chromium.googlesource.com/chromium/src/+/refs/heads/main/docs/rust.md#Third_party-review)이 적용되지만 또한 보안 검토도 적용됩니다. 단일 크레이트뿐만 아니라 전이 종속 항목도 가져올 수 있으므로 검토할 코드가 많을 수 있습니다. 반면에 안전한 Rust 코드에서는 부정적인 부작용이 제한될 수 있습니다. 어떻게 검토해야 할까요?

Chromium은 시간이 지남에 따라 [cargo vet](https://mozilla.github.io/cargo-vet/)를 기반으로 한 프로세스로 전환하는 것을 목표로 합니다.

한편 새로운 크레이트가 추가될 때마다 다음 사항을 확인하고 있습니다.

- 각 크레이트가 사용되는 이유를 이해합니다. 크레이트 간 관계는 어떠한가요? 각 크레이트의 빌드 시스템에 `build.rs` 또는 절차 매크로가 포함된 경우 그 용도를 파악해야 합니다. Chromium이 일반적으로 빌드되는 방식과 호환되나요?
- 각 크레이트가 적절히 잘 유지관리되는지 확인합니다.
- `cd third-party/rust/chromium_crates_io; cargo audit`을 사용하여 알려진 취약점을 확인합니다. 먼저 `cargo install cargo-audit`을 실행해야 합니다. 여기에는 인터넷[2](../cargo.md)에서 여러 종속 항목을 다운로드하는 작업이 포함되어 있습니다.
- `unsafe` 코드가 [2의 법칙](https://chromium.googlesource.com/chromium/src/+/main/docs/security/rule-of-2.md#unsafe-code-in-safe-languages)에 적합한지 확인합니다.
- `fs` 또는 `net` API의 사용 확인
- 악의적으로 삽입되었을 수 있는 잘못된 코드를 찾을 수 있는 충분한 수준으로 모든 코드를 읽습니다. 현실적으로 100% 완벽을 추구할 수는 없습니다. 코드가 너무 많을 때가 많습니다.

이 내용은 가이드라인에 불과합니다. `security@chromium.org`의 검토자와 협력하여 크레이트에 관한 확신을 가질 수 있는 올바른 방법을 찾아보세요.
