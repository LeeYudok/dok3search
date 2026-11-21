# Chromium C++에 의존하는 Rust 코드

위의 타겟을 일부 Chromium C++ 타겟의 `deps`에 추가하기만 하면 됩니다.

``` gn
import("//build/rust/rust_static_library.gni")

rust_static_library("my_rust_lib") {
  crate_root = "lib.rs"
  sources = [ "lib.rs" ]
}

# or source_set, static_library etc.
component("preexisting_cpp") {
  deps = [ ":my_rust_lib" ]
}
```

> **발표자 노트**
>
> We'll see that this relationship only works if the Rust code exposes plain C APIs which can be called from C++, or if we use a C++/Rust interop tool.
