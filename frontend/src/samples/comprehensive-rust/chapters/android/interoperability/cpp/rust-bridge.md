# Rust Bridge Declarations

``` rust
#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type MyType; // 불투명 타입입니다.
        fn foo(&self); // `MyType`의 메서드입니다.
        fn bar() -> Box<MyType>; // Free function
    }
}

struct MyType(i32);

impl MyType {
    fn foo(&self) {
        println!("{}", self.0);
    }
}

fn bar() -> Box<MyType> {
    Box::new(MyType(123))
}
```

> **발표자 노트**
>
> - `extern "Rust"`에 선언된 항목은 상위 모듈의 범위 내에 있는 항목을 참조합니다.
> - CXX 코드 생성기는 `extern "Rust"` 섹션을 사용하여 상응하는 C++ 선언이 포함된 C++ 헤더 파일을 생성합니다. 생성된 헤더는 파일 확장자가 .rs.h인 경우를 제외하고 브리지가 포함된 Rust 소스 파일과 동일한 경로를 갖습니다.
