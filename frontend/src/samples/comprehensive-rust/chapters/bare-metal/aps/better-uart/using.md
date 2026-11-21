# Using it

드라이버를 사용하여 직렬 콘솔에 쓰고 수신되는 바이트를 에코하는 간단한 프로그램을 작성해 보겠습니다.

컴파일 오류 예제

``` rust
#![no_main]
#![no_std]

mod exceptions;
mod pl011;

use crate::pl011::Uart;
use core::fmt::Write;
use core::panic::PanicInfo;
use log::error;
use smccc::psci::system_off;
use smccc::Hvc;

/// 기본 PL011 UART의 기본 주소입니다.
const PL011_BASE_ADDRESS: *mut u32 = 0x900_0000 as _;

#[no_mangle]
extern "C" fn main(x0: u64, x1: u64, x2: u64, x3: u64) {
    // `PL011_BASE_ADDRESS`가 PL011 기기의 기본 주소이고
    // 이 주소 범위에 액세스하는 다른 항목이 없으므로 안전합니다.
    let mut uart = unsafe { Uart::new(PL011_BASE_ADDRESS) };

    writeln!(uart, "main({x0:#x}, {x1:#x}, {x2:#x}, {x3:#x})").unwrap();

    loop {
        if let Some(byte) = uart.read_byte() {
            uart.write_byte(byte);
            match byte {
                b'\r' => {
                    uart.write_byte(b'\n');
                }
                b'q' => break,
                _ => {}
            }
        }
    }

    writeln!(uart, "안녕!").unwrap();
    system_off::<Hvc>().unwrap();
}
```

> **발표자 노트**
>
> - [인라인 어셈블리](../inline-assembly.md) 예에서와 같이 이 `main` 함수는 `entry.S`의 진입점 코드에서 호출됩니다. 자세한 내용은 해당 발표자 노트를 참고하세요.
> - `src/bare-metal/aps/examples`에서 `make qemu`를 사용하여 QEMU에서 예를 실행합니다.
