# Bindgen 사용하기

[bindgen](https://rust-lang.github.io/rust-bindgen/introduction.html)는 C 헤더파일에서 러스트 바인딩을 자동으로 생성하는 도구입니다.

먼저 작은 C라이브러리를 만들어 보겠습니다:

*interoperability/bindgen/libbirthday.h*:

``` c
typedef struct card {
  const char* name;
  int years;
} card;

void print_card(const card* card);
```

*interoperability/bindgen/libbirthday.c*:

``` c
#include <stdio.h>
#include "libbirthday.h"

void print_card(const card* card) {
  printf("+--------------\n");
  printf("| %s님, 생일 축하합니다.\n", card->name);
  printf("| %i주년을 축하합니다!\n", card->years);
  printf("+--------------\n");
}
```

`Android.bp` 파일에 아래를 추가합니다:

*interoperability/bindgen/Android.bp*:

``` javascript
cc_library {
    name: "libbirthday",
    srcs: ["libbirthday.c"],
}
```

라이브러리에 대한 헤더 파일을 만듭니다(이 예시에서는 반드시 필요한 것은 아닙니다.):

*interoperability/bindgen/libbirthday_wrapper.h*:

``` c
#include "libbirthday.h"
```

이제 바인딩을 자동으로 생성할 수 있습니다:

*interoperability/bindgen/Android.bp*:

``` javascript
rust_bindgen {
    name: "libbirthday_bindgen",
    crate_name: "birthday_bindgen",
    wrapper_src: "libbirthday_wrapper.h",
    source_stem: "bindings",
    static_libs: ["libbirthday"],
}
```

마침내, 러스트 프로그램에서 바인딩을 사용할 수 있습니다:

*interoperability/bindgen/Android.bp*:

``` javascript
rust_binary {
    name: "print_birthday_card",
    srcs: ["main.rs"],
    rustlibs: ["libbirthday_bindgen"],
}
```

*interoperability/bindgen/main.rs*:

컴파일 오류 예제

``` rust
//! Bindgen 데모입니다.

use birthday_bindgen::{card, print_card};

fn main() {
    let name = std::ffi::CString::new("피터").unwrap();
    let card = card { name: name.as_ptr(), years: 42 };
    // SAFETY: `print_card` is safe to call with a valid `card` pointer.
    unsafe {
        print_card(&card as *const card);
    }
}
```

빌드하고, 가상 디바이스에 넣고, 실행합니다:

``` shell
m print_birthday_card
adb push "$ANDROID_PRODUCT_OUT/system/bin/print_birthday_card" /data/local/tmp
adb shell /data/local/tmp/print_birthday_card
```

마지막으로, 바인딩이 잘 작동하는지 확인하기 위해, 자동 생성된 테스트를 실행해 보겠습니다:

*interoperability/bindgen/Android.bp*:

``` javascript
rust_test {
    name: "libbirthday_bindgen_test",
    srcs: [":libbirthday_bindgen"],
    crate_name: "libbirthday_bindgen_test",
    test_suites: ["general-tests"],
    auto_gen_config: true,
    clippy_lints: "none", // 생성된 파일, 린트 작업 건너뛰기
    lints: "none",
}
```

``` shell
atest libbirthday_bindgen_test
```
