# `unsafe` Rust 코드 포함

`rust_static_library`에는 안전하지 않은 Rust 코드가 기본적으로 금지되어 있으므로 컴파일되지 않습니다. 안전하지 않은 Rust 코드가 필요하면 gn 타겟에 `allow_unsafe = true`를 추가하세요. (이 과정의 후반부에 이것이 필요한 상황을 살펴봅니다.)

``` gn
import("//build/rust/rust_static_library.gni")

rust_static_library("my_rust_lib") {
  crate_root = "lib.rs"
  sources = [
    "lib.rs",
    "hippopotamus.rs"
  ]
  allow_unsafe = true
}
```
