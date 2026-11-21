# Building in Android

`cc_library_static`을 만들어 CXX에서 생성된 헤더와 소스 파일을 비롯하여 C++ 라이브러리를 빌드합니다.

``` javascript
cc_library_static {
    name: "libcxx_test_cpp",
    srcs: ["cxx_test.cpp"],
    generated_headers: [
        "cxx-bridge-header",
        "libcxx_test_bridge_header"
    ],
    generated_sources: ["libcxx_test_bridge_code"],
}
```

> **발표자 노트**
>
> - `libcxx_test_bridge_header` 및 `libcxx_test_bridge_code`가 CXX에서 생성된 C++ 바인딩의 종속 항목이라는 점을 지적합니다. 다음 슬라이드에서 설정 방법을 알아봅니다.
> - 일반적인 CXX 정의를 가져오려면 `cxx-bridge-header` 라이브러리도 사용해야 합니다.
> - Android에서 CXX를 사용하는 방법에 관한 전체 문서는 [Android 문서](https://source.android.com/docs/setup/build/rust/building-rust-modules/android-rust-patterns#rust-cpp-interop-using-cxx)에서 확인할 수 있습니다. 학생들이 나중에 이 안내를 어디에서 다시 찾을 수 있는지 알 수 있도록 이 링크를 수업에 공유하는 것이 좋습니다.
