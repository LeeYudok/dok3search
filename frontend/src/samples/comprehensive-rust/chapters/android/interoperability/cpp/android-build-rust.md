# Building in Android

`libcxx` 및 `cc_library_static`에 종속되는 `rust_binary`를 생성합니다.

``` javascript
rust_binary {
    name: "cxx_test",
    srcs: ["lib.rs"],
    rustlibs: ["libcxx"],
    static_libs: ["libcxx_test_cpp"],
}
```
