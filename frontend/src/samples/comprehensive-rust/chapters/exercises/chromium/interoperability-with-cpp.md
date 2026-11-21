# Exercise: Interoperability with C++

## 1부

- 이전에 만든 Rust 파일에서, C++에서 호출할 `hello_from_rust`라는 단일 함수를 지정하는 `#[cxx::bridge]`를 추가합니다. 이 함수는 매개변수를 사용하지 않고 값을 반환하지 않습니다.
- 이전의 `hello_from_rust` 함수를 수정하여 `extern "C"` 및 `#[no_mangle]`을 삭제합니다. 이 함수는 이제 표준 Rust 함수입니다.
- `gn` 타겟을 수정하여 이러한 바인딩을 빌드합니다.
- C++ 코드에서 `hello_from_rust`의 정방향 선언을 삭제합니다. 대신 생성된 헤더 파일을 포함합니다.
- 빌드 및 실행

## 2부

CXX를 사용해 보는 것도 좋은 방법입니다. Chromium의 Rust가 실제로 얼마나 유연한지 생각해 보는 데 도움이 됩니다.

Some things to try:

- Rust에서 C++로 다시 호출 필요한 사항은 다음과 같습니다.
  - `cxx::bridge`에서 `include!`할 수 있는 추가 헤더 파일입니다. 새 헤더 파일에서 C++ 함수를 선언해야 합니다.
  - 이러한 함수를 호출하거나 [여기에 설명된 대로](https://cxx.rs/extern-c++.html#functions-and-member-functions) `#[cxx::bridge]`에서 `unsafe` 키워드를 지정하는 `unsafe` 블록입니다.
  - `#include "third_party/rust/cxx/v1/crate/include/cxx.h"`가 필요할 수도 있습니다.
- C++에서 Rust로 C++ 문자열을 전달합니다.
- C++ 객체 참조를 Rust로 전달합니다.
- 의도적으로 `#[cxx::bridge]`에서 일치하지 않는 Rust 함수 서명을 가져와서 표시되는 오류에 익숙해집니다.
- 의도적으로 `#[cxx::bridge]`에서 일치하지 않는 C++ 함수 서명을 가져와서 표시되는 오류에 익숙해집니다.
- Rust가 C++ 객체를 소유할 수 있도록 C++에서 일부 유형의 `std::unique_ptr`을 Rust로 전달합니다.
- Rust 객체를 만들어 C++로 전달하여 C++에서 소유하도록 합니다. 힌트: `Box` 필요
- C++ 유형에 일부 메서드를 선언합니다. Rust에서 이를 호출하세요.
- Rust 유형에 일부 메서드를 선언합니다. C++에서 이를 호출하세요.

## 3부

지금까지 CXX 상호 운용성의 강점과 한계를 이해했으니, 인터페이스가 충분히 간단한 Chromium의 Rust 사용 사례를 생각해 보세요. 해당 인터페이스를 정의하는 방법을 스케치합니다.

## 도움을 받을 수 있는 곳

- [`cxx` 바인딩 참조](https://cxx.rs/bindings.html)
- [`rust_static_library` gn 템플릿](https://source.chromium.org/chromium/chromium/src/+/main:build/rust/rust_static_library.gni;l=16)

> **발표자 노트**
>
> As students explore Part Two, they're bound to have lots of questions about how to achieve these things, and also how CXX works behind the scenes.
>
> 다음과 같은 문제가 있을 수 있습니다.
>
> - 유형 Y로 유형 X의 변수를 초기화하는 데 문제가 있습니다. 여기서 X와 Y는 모두 함수 유형입니다. 이는 C++ 함수가 `cxx::bridge`의 선언과 일치하지 않기 때문입니다.
> - C++ 참조를 Rust 참조로 자유롭게 변환할 수 있는 것 같습니다. 이렇게 하면 UB가 발생하지 않을까요? CXX의 *불투명* 유형의 경우 발생하지 않습니다. 크기가 0이기 때문입니다. CXX 사소한 유형의 경우 UB를 유발하는 것이 \_가능_하지만 CXX의 설계상 이러한 예를 만들기가 상당히 어렵습니다.
