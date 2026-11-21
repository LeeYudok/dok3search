# Chromium Rust 정책

Chromium에서는 아직 퍼스트 파티 Rust를 허용하지 않습니다. 단, Chromium의 [Area Tech Leads](https://source.chromium.org/chromium/chromium/src/+/main:ATL_OWNERS)에서 승인한 드문 경우는 예외입니다.

서드 파티 라이브러리에 관한 Chromium의 정책은 [여기](https://chromium.googlesource.com/chromium/src/+/main/docs/adding_to_third_party.md#rust)에 설명되어 있습니다. Rust는 성능이나 보안을 위해 최상의 옵션인 경우 등 다양한 상황에서 서드 파티 라이브러리에 허용됩니다.

C/C++ API를 직접 노출하는 Rust 라이브러리는 극소수이므로 이러한 거의 모든 라이브러리에는 소량의 퍼스트 파티 글루 코드가 필요합니다.

![Rust · Language · boundary · Existing · crate · Crate · API · Chromium · C++ · wrapper](../../assets/chromium/policy-diagram-1.svg)

> 특정 서드 파티 크레이트의 퍼스트 파티 Rust 글루 코드는 일반적으로 `third_party/rust/<crate>/<version>/wrapper`에 보관해야 합니다.

따라서 오늘 과정에서는 다음과 같은 내용을 중점적으로 다룹니다.

- 서드 파티 Rust 라이브러리(‘crates’) 가져오기
- Chromium C++에서 이러한 크레이트를 사용할 수 있도록 글루 코드를 작성합니다.

시간이 지나면서 이 정책이 변경되면 교육 과정도 이에 맞게 변경됩니다.
