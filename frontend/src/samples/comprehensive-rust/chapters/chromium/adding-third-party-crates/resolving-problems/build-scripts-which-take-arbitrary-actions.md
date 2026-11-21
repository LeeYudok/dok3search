# C++를 빌드하거나 임의의 작업을 실행하는 스크립트 빌드

일부 크레이트는 [`cc`](https://crates.io/crates/cc) 크레이트를 사용하여 C/C++ 라이브러리를 빌드하고 연결합니다. 다른 크레이트는 빌드 스크립트 내에서 [`bindgen`](https://crates.io/crates/bindgen)을 사용하여 C/C++를 파싱합니다. 이러한 작업은 Chromium 컨텍스트에서는 지원되지 않습니다. — Google의 gn, ninja, LLVM 빌드 시스템은 빌드 작업 간의 관계를 매우 구체적으로 표현합니다.

옵션은 다음과 같습니다.

- 이 크레이트는 피하세요.
- 크레이트에 패치를 적용합니다.

패치는 `third_party/rust/chromium_crates_io/patches/<crate>`에 보관해야 합니다. 예는 [`cxx` 크레이트에 대한 패치](https://source.chromium.org/chromium/chromium/src/+/main:third_party/rust/chromium_crates_io/patches/cxx/)를 참고하세요. 그리고 크레이트를 업그레이드할 때마다 `gnrt`에 의해 자동으로 적용됩니다.
