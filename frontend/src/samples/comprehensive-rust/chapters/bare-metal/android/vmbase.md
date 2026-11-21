# vmbase

[vmbase](https://android.googlesource.com/platform/packages/modules/Virtualization/+/refs/heads/master/vmbase/) 라이브러리는, aarch64의 crosvm에서 실행되는 VM을 타겟하여, 진입점, UART 콘솔 로깅, 링커 스크립트, 빌드 룰 등에 대한 기본 구현들을 제공합니다.

컴파일 오류 예제

``` rust
#![no_main]
#![no_std]

use vmbase::{main, println};

main!(main);

pub fn main(arg0: u64, arg1: u64, arg2: u64, arg3: u64) {
    println!("Hello world");
}
```

> **발표자 노트**
>
> - `main!` 매크로는 `vmbase` 진입점에서 호출될 main 함수를 표시합니다.
> - `vmbase`가 제공하는 진입점은 콘솔을 초기화 하며, main 함수가 리턴하면 PSCI_SYSTEM_OFF 메시지를 PSCI를 통해 보내어서 VM을 종료시킵니다.
