# 빌드 규칙

Rust 코드는 일반적으로 `cargo`를 사용하여 빌드됩니다. Chromium은 효율성을 위해 `gn` 및 `ninja`로 빌드됩니다. 정적 규칙은 최대 동시 로드를 허용합니다. Rust도 예외는 아닙니다.

## Chromium에 Rust 코드 추가

일부 기존 Chromium `BUILD.gn` 파일에서 `rust_static_library`를 선언합니다.

``` gn
import("//build/rust/rust_static_library.gni")

rust_static_library("my_rust_lib") {
  crate_root = "lib.rs"
  sources = [ "lib.rs" ]
}
```

다른 Rust 타겟에도 `deps`를 추가할 수 있습니다. 나중에 서드 파티 코드에 의존하기 위해 이를 사용합니다.

> **발표자 노트**
>
> 크레이트 루트 *및* 소스 전체 목록 \_둘 다_를 지정해야 합니다. `crate_root`는 컴파일 단위의 루트 파일(일반적으로 `lib.rs`)을 나타내는 Rust 컴파일러에 제공되는 파일입니다. `sources`는 재빌드가 필요한 시점을 결정하기 위해 `ninja`에 필요한 모든 소스 파일의 전체 목록입니다.
>
> (Rust에서는 크레이트 전체가 컴파일 단위이므로 Rust `source_set`와 같은 것은 없습니다. `static_library`가 최소 단위입니다.)
>
> 학생들은 [gn의 내장 Rust 정적 라이브러리 지원](https://gn.googlesource.com/gn/+/main/docs/reference.md#func_static_library)을 사용하는 대신 gn 템플릿이 필요한 이유를 궁금해할 수 있습니다. 대답은 이 템플릿이 CXX 상호 운용성, Rust 기능, 단위 테스트를 지원한다는 것입니다. 이 중 일부는 나중에 사용하게 됩니다.
