# 브리지 모듈

CXX는 각 언어에서 다른 언어로 노출되는 함수 서명에 관한 설명을 사용합니다. `#[cxx::bridge]` 속성 매크로로 주석이 달린 Rust 모듈에서 extern 블록을 사용하여 이 설명을 제공합니다.

``` rust
#[allow(unsafe_op_in_unsafe_fn)]
#[cxx::bridge(namespace = "org::blobstore")]
mod ffi {
    // 두 언어 모두에 표시되는 필드가 있는 공유 구조체입니다.
    struct BlobMetadata {
        size: usize,
        tags: Vec<String>,
    }

    // C++에 노출된 Rust 타입 및 메서드 시그니쳐입니다.
    extern "Rust" {
        type MultiBuf;

        fn next_chunk(buf: &mut MultiBuf) -> &[u8];
    }

    // Rust에 노출된 C++ 타입 및 함수 시그니쳐입니다.
    unsafe extern "C++" {
        include!("include/blobstore.h");

        type BlobstoreClient;

        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
        fn put(self: Pin<&mut BlobstoreClient>, parts: &mut MultiBuf) -> u64;
        fn tag(self: Pin<&mut BlobstoreClient>, blobid: u64, tag: &str);
        fn metadata(&self, blobid: u64) -> BlobMetadata;
    }
}
```

> **발표자 노트**
>
> - 브리지는 일반적으로 크레이트 내의 `ffi` 모듈에 선언됩니다.
> - 브리지 모듈에서 이루어진 선언으로부터 CXX는 일치하는 Rust 및 C++ 타입/함수 정의를 생성하여 이러한 항목을 두 언어 모두에 노출합니다.
> - 생성된 Rust 코드를 보려면 [cargo-expand](https://github.com/dtolnay/cargo-expand)를 사용하여 확장된 proc 매크로를 확인하세요. 대부분의 예에서는 `cargo expand ::ffi`를 사용하여 `ffi` 모듈만 확장합니다(Android 프로젝트에는 적용되지 않음).
> - 생성된 C++ 코드를 보려면 `target/cxxbridge`를 확인하세요.
