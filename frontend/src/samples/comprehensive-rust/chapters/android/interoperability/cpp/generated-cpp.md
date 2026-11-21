# 생성된 C++

``` rust
#[cxx::bridge]
mod ffi {
    // C++에 노출된 Rust 타입 및 메서드 시그니쳐입니다.
    extern "Rust" {
        type MultiBuf;

        fn next_chunk(buf: &mut MultiBuf) -> &[u8];
    }
}
```

그 결과는 대략 다음과 같은 C++입니다.

``` cpp
struct MultiBuf final : public ::rust::Opaque {
  ~MultiBuf() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};

::rust::Slice<::std::uint8_t const> next_chunk(::org::blobstore::MultiBuf &buf) noexcept;
```
