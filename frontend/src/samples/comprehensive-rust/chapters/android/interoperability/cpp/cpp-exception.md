# C++ 오류처리

``` rust
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("example/include/example.h");
        fn fallible(depth: usize) -> Result<String>;
    }
}

fn main() {
    if let Err(err) = ffi::fallible(99) {
        eprintln!("오류: {}", err);
        process::exit(1);
    }
}
```

> **발표자 노트**
>
> - `Result`를 반환하도록 선언된 C++ 함수는 C++ 측에서 발생한 예외를 포착하고 이를 호출 Rust 함수에 `Err` 값으로 반환합니다.
> - `Result`를 반환하도록 CXX 브리지에서 선언하지 않은 extern ‘C++’ 함수에서 예외가 발생하면 프로그램은 C++의 `std::terminate`를 호출합니다. 이 동작은 `noexcept` C++ 함수를 통해 발생하는 동일한 예외와 같습니다.
