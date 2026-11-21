# 러스트 라이브러리

`rust_library`를 사용하여 안드로이드용 새 러스트 라이브러리를 만듭니다.

여기서 두 개의 라이브러리에 대한 의존성을 선언합니다:

- 아래에 정의한 `libgreeting`.
- [`external/rust/crates/`](https://cs.android.com/android/platform/superproject/+/master:external/rust/crates/)에 존재하는 `libtextwrap`.

*hello_rust/Android.bp*:

``` javascript
rust_binary {
    name: "hello_rust_with_dep",
    crate_name: "hello_rust_with_dep",
    srcs: ["src/main.rs"],
    rustlibs: [
        "libgreetings",
        "libtextwrap",
    ],
    prefer_rlib: true, // Need this to avoid dynamic link error.
}

rust_library {
    name: "libgreetings",
    crate_name: "인사말",
    srcs: ["src/lib.rs"],
}
```

*hello_rust/src/main.rs*:

``` rust
//! Rust 데모입니다.

use greetings::greeting;
use textwrap::fill;

/// 인사말을 표준 출력으로 인쇄합니다.
fn main() {
    println!("{}", fill(&greeting("Bob"), 24));
}
```

*hello_rust/src/lib.rs*:

``` rust
//! 인사말 라이브러리입니다.

/// `name`에게 인사합니다.
pub fn greeting(name: &str) -> String {
    format!("{name}님, 안녕하세요. 만나서 반갑습니다.")
}
```

이전처럼, 빌드하고, 가상 디바이스로 넣고, 실행합니다:

``` shell
m hello_rust_with_dep
adb push "$ANDROID_PRODUCT_OUT/system/bin/hello_rust_with_dep" /data/local/tmp
adb shell /data/local/tmp/hello_rust_with_dep
```

``` text
Hello Bob, it is very
nice to meet you!
```
