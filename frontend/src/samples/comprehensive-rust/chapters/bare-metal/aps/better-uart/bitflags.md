# 비트플래그

[`bitflags`](https://crates.io/crates/bitflags) 크레이트는 비트플래그를 사용하는 데 유용합니다.

컴파일 오류 예제

``` rust
use bitflags::bitflags;

bitflags! {
    /// UART 플래그 레지스터의 플래그입니다.
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    struct Flags: u16 {
        /// 보내려면 지웁니다.
        const CTS = 1 << 0;
        /// 데이터 세트가 준비되었습니다.
        const DSR = 1 << 1;
        /// 데이터 이동통신사 감지
        const DCD = 1 << 2;
        /// UART에서 데이터를 전송 중입니다.
        const BUSY = 1 << 3;
        /// 수신 FIFO가 비어 있습니다.
        const RXFE = 1 << 4;
        /// 전송 FIFO가 가득 찼습니다.
        const TXFF = 1 << 5;
        /// 수신 FIFO가 가득 찼습니다.
        const RXFF = 1 << 6;
        /// 전송 FIFO가 비어 있습니다.
        const TXFE = 1 << 7;
        /// 벨소리 표시기입니다.
        const RI = 1 << 8;
    }
}
```

> **발표자 노트**
>
> - `bitflags!` 매크로는 플래그를 가져오고 설정하기 위한 여러 메서드 구현과 함께 `Flags(u16)`와 같은 새로운 타입을 생성합니다.
