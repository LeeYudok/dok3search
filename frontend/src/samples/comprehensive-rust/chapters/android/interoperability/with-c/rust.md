# C에서 러스트 호출

러스트에서 타입과 함수를 C로 내보내는 것은 간단합니다:

*interoperability/rust/libanalyze/analyze.rs*

``` rust
//! Rust FFI 데모입니다.
#![deny(improper_ctypes_definitions)]

use std::os::raw::c_int;

/// 수치를 분석합니다.
#[no_mangle]
pub extern "C" fn analyze_numbers(x: c_int, y: c_int) {
    if x < y {
        println!("x({x})가 가장 작습니다.");
    } else {
        println!("y({y})는 x({x})보다 클 수 있습니다.");
    }
}
```

*interoperability/rust/libanalyze/analyze.h*

``` c
#ifndef ANALYSE_H
#define ANALYSE_H

extern "C" {
void analyze_numbers(int x, int y);
}

#endif
```

*interoperability/rust/libanalyze/Android.bp*

``` javascript
rust_ffi {
    name: "libanalyze_ffi",
    crate_name: "analyze_ffi",
    srcs: ["analyze.rs"],
    include_dirs: ["."],
}
```

이제 이 러스트 함수를 C바이너리에서 호출할 수 있습니다:

*interoperability/rust/analyze/main.c*

``` c
#include "analyze.h"

int main() {
  analyze_numbers(10, 20);
  analyze_numbers(123, 123);
  return 0;
}
```

*interoperability/rust/analyze/Android.bp*

``` javascript
cc_binary {
    name: "analyze_numbers",
    srcs: ["main.c"],
    static_libs: ["libanalyze_ffi"],
}
```

빌드하고, 가상 디바이스에 넣고, 실행합니다:

``` shell
m analyze_numbers
adb push "$ANDROID_PRODUCT_OUT/system/bin/analyze_numbers" /data/local/tmp
adb shell /data/local/tmp/analyze_numbers
```

> **발표자 노트**
>
> `#[no_mangle]`은 러스트의 네임 맹글링(name mangling)을 비활성화하므로 외부로 노출되는 심볼의 이름은 함수의 이름 그대로가 됩니다. 심볼 이름을 바꾸고 싶다면 `#[export_name = "some_name"]`을 사용합니다.
