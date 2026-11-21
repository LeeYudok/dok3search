# 표준 라이브러리

Rust comes with a standard library which helps establish a set of common types used by Rust libraries and programs. This way, two libraries can work together smoothly because they both use the same `String` type.

In fact, Rust contains several layers of the Standard Library: `core`, `alloc` and `std`.

- `core` includes the most basic types and functions that don’t depend on `libc`, allocator or even the presence of an operating system.
- `alloc`은 `Vec`, `Box`, `Arc`와 같이 전역 힙 할당이 필요한 타입을 포함합니다.
- 임베디드 러스트 응용프로그램은 주로 `core`만 사용하거나 가끔 `alloc`을 함께 사용합니다.
