# 로깅

`log` 크레이트를 사용하면 안드로이드 디바이스 안에서 수행될 때에는 `logcat`으로, 호스트에서 수행될 때에는 `stdout`으로 로그가 자동으로 출력이 되도록 할 수 있습니다:

*hello_rust_logs/Android.bp*:

``` javascript
rust_binary {
    name: "hello_rust_logs",
    crate_name: "hello_rust_logs",
    srcs: ["src/main.rs"],
    rustlibs: [
        "liblog_rust",
        "liblogger",
    ],
    host_supported: true,
}
```

*hello_rust_logs/src/main.rs*:

``` rust
//! Rust 로깅 데모입니다.

use log::{debug, error, info};

/// 인사말을 기록합니다.
fn main() {
    logger::init(
        logger::Config::default()
            .with_tag_on_device("rust")
            .with_min_level(log::Level::Trace),
    );
    debug!("프로그램을 시작하는 중입니다.");
    info!("잘 진행되고 있습니다.");
    error!("문제가 발생했습니다!");
}
```

빌드하고, 가상 디바이스에 넣고, 실행합니다:

``` shell
m hello_rust_logs
adb push "$ANDROID_PRODUCT_OUT/system/bin/hello_rust_logs" /data/local/tmp
adb shell /data/local/tmp/hello_rust_logs
```

`adb logcat`커맨드로 로그를 확인합니다:

``` shell
adb logcat -s rust
```

``` text
09-08 08:38:32.454  2420  2420 D rust: hello_rust_logs: Starting program.
09-08 08:38:32.454  2420  2420 I rust: hello_rust_logs: Things are going fine.
09-08 08:38:32.454  2420  2420 E rust: hello_rust_logs: Something went wrong!
```
