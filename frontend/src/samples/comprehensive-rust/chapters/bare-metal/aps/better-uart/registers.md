# Multiple registers

구조체를 사용하여 UART 레지스터의 메모리 레이아웃을 나타낼 수 있습니다.

컴파일 오류 예제

``` rust
#[repr(C, align(4))]
struct Registers {
    dr: u16,
    _reserved0: [u8; 2],
    rsr: ReceiveStatus,
    _reserved1: [u8; 19],
    fr: Flags,
    _reserved2: [u8; 6],
    ilpr: u8,
    _reserved3: [u8; 3],
    ibrd: u16,
    _reserved4: [u8; 2],
    fbrd: u8,
    _reserved5: [u8; 3],
    lcr_h: u8,
    _reserved6: [u8; 3],
    cr: u16,
    _reserved7: [u8; 3],
    ifls: u8,
    _reserved8: [u8; 3],
    imsc: u16,
    _reserved9: [u8; 2],
    ris: u16,
    _reserved10: [u8; 2],
    mis: u16,
    _reserved11: [u8; 2],
    icr: u16,
    _reserved12: [u8; 2],
    dmacr: u8,
    _reserved13: [u8; 3],
}
```

> **발표자 노트**
>
> - [`#[repr(C)]`](https://doc.rust-lang.org/reference/type-layout.html#the-c-representation)는 C와 동일한 규칙에 따라 구조체 필드를 순서대로 배치하도록 컴파일러에 지시합니다. 기본 Rust 표현을 사용하면 컴파일러가 원하는 대로 필드의 순서를 변경할 수 있으므로 구조체에서 예측 가능한 레이아웃을 사용하는 데 필요합니다.
