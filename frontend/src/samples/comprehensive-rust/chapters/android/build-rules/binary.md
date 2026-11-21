# 러스트 바이너리

간단한 응용 프로그램으로 시작해 보겠습니다. AOSP 체크아웃의 루트에서 다음 파일을 생성합니다:

*hello_rust/Android.bp*:

``` javascript
rust_binary {
    name: "hello_rust",
    crate_name: "hello_rust",
    srcs: ["src/main.rs"],
}
```

*hello_rust/src/main.rs*:

``` rust
//! Rust 데모입니다.

/// 인사말을 표준 출력으로 인쇄합니다.
fn main() {
    println!("Hello from Rust!");
}
```

그런 다음, 이 바이너리를 빌드하고, 가상 디바이스에 넣고, 실행합니다:

``` shell
m hello_rust
adb push "$ANDROID_PRODUCT_OUT/system/bin/hello_rust" /data/local/tmp
adb shell /data/local/tmp/hello_rust
```

``` text
Hello from Rust!
```
