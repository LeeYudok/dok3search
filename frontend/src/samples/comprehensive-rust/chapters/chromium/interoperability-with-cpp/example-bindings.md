# 예제

CXX에서는 전체 C++/Rust 경계가 `.rs` 소스 코드 내의 `cxx::bridge` ’modules’에 선언되어야 합니다.

``` rust
#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type MultiBuf;

        fn next_chunk(buf: &mut MultiBuf) -> &[u8];
    }

    unsafe extern "C++" {
        include!("example/include/blobstore.h");

        type BlobstoreClient;

        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
        fn put(self: &BlobstoreClient, buf: &mut MultiBuf) -> Result<u64>;
    }
}

// Rust 유형 및 함수의 정의는 여기를 참고하세요.
```

> **발표자 노트**
>
> 참고:
>
> - Although this looks like a regular Rust `mod`, the `#[cxx::bridge]` procedural macro does complex things to it. The generated code is quite a bit more sophisticated - though this does still result in a `mod` called `ffi` in your code.
> - Rust에서 C++’의 `std::unique_ptr` 기본 지원
> - Native support for Rust slices in C++
> - C++에서 Rust 호출 및 Rust 유형(상단)
> - Rust에서 C++ 호출 및 C++ 유형(하단)
>
> **일반적인 오해**: C++ 헤더가 Rust에서 파싱되는 *것처럼 보이지만* 오해의 소지가 있습니다. 이 헤더는 Rust에서 해석되지 않으며 C++ 컴파일러의 이점을 위해 생성된 C++ 코드에 단순히 `#include`됩니다.
