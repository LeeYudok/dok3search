# CXX Error Handling: QR Example

[QR 코드 생성기](https://source.chromium.org/chromium/chromium/src/+/main:components/qr_code_generator/qr_code_generator_ffi_glue.rs;l=10)에서와 같이 간단한 부울로 성공을 나타낼 수 있는 경우: 성공을 나타내는 부울을 반환하고 out 매개변수를 사용하여 결과를 기록합니다.

``` rust
#[cxx::bridge(namespace = "qr_code_generator")]
mod ffi {
    extern "Rust" {
        fn generate_qr_code_using_rust(
            data: &[u8],
            min_version: i16,
            out_pixels: Pin<&mut CxxVector<u8>>,
            out_qr_size: &mut usize,
        ) -> bool;
    }
}
```

> **발표자 노트**
>
> Students may be curious about the semantics of the `out_qr_size` output. This is not the size of the vector, but the size of the QR code (and admittedly it is a bit redundant - this is the square root of the size of the vector).
>
> It may be worth pointing out the importance of initializing `out_qr_size` before calling into the Rust function. Creation of a Rust reference that points to uninitialized memory results in Undefined Behavior (unlike in C++, when only the act of dereferencing such memory results in UB).
>
> If students ask about `Pin`, then explain why CXX needs it for mutable references to C++ data: the answer is that C++ data can’t be moved around like Rust data, because it may contain self-referential pointers.
