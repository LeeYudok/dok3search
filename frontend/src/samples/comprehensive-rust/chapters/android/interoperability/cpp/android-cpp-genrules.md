# Building in Android

두 개의 genrule을 만듭니다. 하나는 CXX 헤더를 생성하고 다른 하나는 CXX 소스 파일을 생성합니다. 그런 다음 `cc_library_static`에 대한 입력으로 사용됩니다.

``` javascript
// lib.rs의 Rust 내보내기 함수에 대한
// C++ 바인딩이 포함된 C++ 헤더를 생성합니다.
genrule {
    name: "libcxx_test_bridge_header",
    tools: ["cxxbridge"],
    cmd: "$(location cxxbridge) $(in) --header > $(out)",
    srcs: ["lib.rs"],
    out: ["lib.rs.h"],
}

// Rust가 호출하는 C++ 코드를 생성합니다.
genrule {
    name: "libcxx_test_bridge_code",
    tools: ["cxxbridge"],
    cmd: "$(location cxxbridge) $(in) > $(out)",
    srcs: ["lib.rs"],
    out: ["lib.rs.cc"],
}
```

> **발표자 노트**
>
> - `cxxbridge` 도구는 C++ 측의 브리지 모듈을 생성하는 독립형 도구입니다. 이 도구는 Android에 포함되어 있으며 Soong 도구로 사용할 수 있습니다.
> - 일반적으로 Rust 소스 파일이 `lib.rs`인 경우 헤더 파일의 이름은 `lib.rs.h`이고 소스 파일의 이름은 `lib.rs.cc`가 됩니다. 그러나 이 이름 지정 규칙은 강제되지 않습니다.
