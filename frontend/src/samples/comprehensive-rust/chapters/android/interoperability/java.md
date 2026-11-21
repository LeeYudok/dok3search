# Java와의 상호운용성

자바는 [Java Native Interface(JNI)](https://en.wikipedia.org/wiki/Java_Native_Interface)를 통해 공유 라이브러리를 로드할 수 있습니다. [`jni` 크레이트](https://docs.rs/jni/)를 사용하여 JNI 라이브러리를 만들 수 있습니다.

먼저, 자바로 내보낼 러스트 함수를 생성합니다:

*interoperability/java/src/lib.rs*:

컴파일 오류 예제

``` rust
#![allow(unused)]
fn main() {
//! Rust <-> Java FFI 데모입니다.

use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;

/// HelloWorld::hello method implementation.
#[no_mangle]
pub extern "system" fn Java_HelloWorld_hello(
    env: JNIEnv,
    _class: JClass,
    name: JString,
) -> jstring {
    let input: String = env.get_string(name).unwrap().into();
    let greeting = format!("Hello, {input}!");
    let output = env.new_string(greeting).unwrap();
    output.into_inner()
}
}
```

*interoperability/java/Android.bp*:

``` javascript
rust_ffi_shared {
    name: "libhello_jni",
    crate_name: "hello_jni",
    srcs: ["src/lib.rs"],
    rustlibs: ["libjni"],
}
```

We then call this function from Java:

*interoperability/java/HelloWorld.java*:

``` java
class HelloWorld {
    private static native String hello(String name);

    static {
        System.loadLibrary("hello_jni");
    }

    public static void main(String[] args) {
        String output = HelloWorld.hello("Alice");
        System.out.println(output);
    }
}
```

*interoperability/java/Android.bp*:

``` javascript
java_binary {
    name: "helloworld_jni",
    srcs: ["HelloWorld.java"],
    main_class: "HelloWorld",
    required: ["libhello_jni"],
}
```

마지막으로 바이너리를 빌드, 싱크, 실행합니다:

``` shell
m helloworld_jni
adb sync  # requires adb root && adb remount
adb shell /system/bin/helloworld_jni
```
