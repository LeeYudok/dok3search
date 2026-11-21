# C++ 브리지 선언

``` rust
#[cxx::bridge]
mod ffi {
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

결과는 대략 다음과 같은 Rust입니다.

``` rust
#[repr(C)]
pub struct BlobstoreClient {
    _private: ::cxx::private::Opaque,
}

pub fn new_blobstore_client() -> ::cxx::UniquePtr<BlobstoreClient> {
    extern "C" {
        #[link_name = "org$blobstore$cxxbridge1$new_blobstore_client"]
        fn __new_blobstore_client() -> *mut BlobstoreClient;
    }
    unsafe { ::cxx::UniquePtr::from_raw(__new_blobstore_client()) }
}

impl BlobstoreClient {
    pub fn put(&self, parts: &mut MultiBuf) -> u64 {
        extern "C" {
            #[link_name = "org$blobstore$cxxbridge1$BlobstoreClient$put"]
            fn __put(
                _: &BlobstoreClient,
                parts: *mut ::cxx::core::ffi::c_void,
            ) -> u64;
        }
        unsafe {
            __put(self, parts as *mut MultiBuf as *mut ::cxx::core::ffi::c_void)
        }
    }
}

// ...
```

> **발표자 노트**
>
> - 프로그래머는 자신이 입력한 시그니쳐가 정확하다고 보장할 필요가 없습니다. CXX는 시그니쳐가C++에서 선언된 것과 정확히 일치하는지를 체크하기 위해 정적으로 assertion을 실행합니다.
> - `unsafe extern` 블록을 사용하면 Rust에서 안전하게 호출할 수 있는 C++ 함수를 선언할 수 있습니다.
