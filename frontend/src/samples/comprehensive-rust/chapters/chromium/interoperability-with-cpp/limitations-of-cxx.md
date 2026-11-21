# CXX 제한사항

## CXX 제한사항

CXX를 사용할 때 단연 가장 유용한 페이지는 [유형 참조](https://cxx.rs/bindings.html)입니다.

CXX는 기본적으로 다음과 같은 사례에 적합합니다.

- Rust-C++ 인터페이스는 매우 단순하여 모두 선언할 수 있습니다.
- 이미 CXX에서 기본적으로 지원하는 유형만 사용하고 있습니다(예: `std::unique_ptr`, `std::string`, `&[u8]` 등).

많은 제한이 있습니다. 예를 들어 Rust의 `Option` 유형은 지원되지 않습니다.

이러한 제한사항으로 인해 임의의 Rust-C++ 상호 운용성이 아닌 잘 격리된 ’리프 노드’의 경우에만 Chromium에서 Rust를 사용할 수 있습니다. Chromium에서 Rust 사용 사례를 고려할 때 좋은 출발점은 언어 경계의 CXX 바인딩 초안을 작성하여 충분히 단순하게 표시되는지 확인하는 것입니다.

> **발표자 노트**
>
> In addition, right now, Rust code in one component cannot depend on Rust code in another, due to linking details in our component build. That's another reason to restrict Rust to use in leaf nodes.
>
> CXX의 다른 어려운 문제도 논의해야 합니다. 예를 들면 다음과 같습니다.
>
> - 오류 처리는 C++ 예외를 기반으로 합니다(다음 슬라이드에 나와 있음).
> - 함수 포인터는 사용하기 어색합니다.
