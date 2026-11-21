# Chromium에서 CXX 사용

## Chromium에서 cxx 사용

Chromium에서는 Rust를 사용하려는 리프 노드마다 독립적인 `#[cxx::bridge] mod`를 정의합니다. 일반적으로 `rust_static_library`마다 하나씩 있습니다. 추가하기만 하면 됩니다.

``` gn
cxx_bindings = [ "my_rust_file.rs" ]
   # 모든 소스 파일이 아닌 #[cxx::bridge]가 포함된 파일 목록
allow_unsafe = true
```

`crate_root` 및 `sources`와 함께 기존 `rust_static_library` 타겟에 추가합니다.

C++ 헤더는 적절한 위치에 생성됩니다. 따라서

``` cpp
#include 'ui/base/my_rust_file.rs.h'
```

`//base`에서 Chromium C++ 유형으로 변환하는 또는 Chromium C++ 유형에서 CXX Rust 유형으로 변환하는 유틸리티 함수를 확인할 수 있습니다. 예: [`SpanToRustSlice`](https://source.chromium.org/chromium/chromium/src/+/main:base/containers/span_rust.h;l=21)

> **발표자 노트**
>
> `allow_unsafe = true`가 계속 필요한 이유가 궁금할 수 있습니다.
>
> 광범위한 의미의 답변은 일반적인 Rust 표준에서는 C/C++ 코드가 ‘안전하지’ 않다는 것입니다. Rust에서 C/C++를 여기저기 호출하면 메모리에 임의적인 작업을 할 수 있으며 Rust 자체 데이터 레이아웃의 안전성이 손상될 수 있습니다. C/C++ 상호 운용성에 `unsafe` 키워드가 *너무 많으면* 이러한 키워드의 신호대 잡음비에 해를 끼칠 수 있으며 [논란의 소지가 있습니다](https://steveklabnik.com/writing/the-cxx-debate). 그러나 엄격하게는 외부 코드를 Rust 바이너리로 가져오면 Rust의 관점에서 예기치 않은 동작이 발생할 수 있습니다.
>
> The narrow answer lies in the diagram at the top of [this page](../interoperability-with-cpp.md) — behind the scenes, CXX generates Rust `unsafe` and `extern "C"` functions just like we did manually in the previous section.
