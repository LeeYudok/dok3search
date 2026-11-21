# Using it

로거를 사용하려면 먼저 초기화해야 합니다.

컴파일 오류 예제

``` rust
#![no_main]
#![no_std]

mod exceptions;
mod logger;
mod pl011;

use crate::pl011::Uart;
use core::panic::PanicInfo;
use log::{error, info, LevelFilter};
use smccc::psci::system_off;
use smccc::Hvc;

/// 기본 PL011 UART의 기본 주소입니다.
const PL011_BASE_ADDRESS: *mut u32 = 0x900_0000 as _;

#[no_mangle]
extern "C" fn main(x0: u64, x1: u64, x2: u64, x3: u64) {
    // `PL011_BASE_ADDRESS`가 PL011 기기의 기본 주소이고
    // 이 주소 범위에 액세스하는 다른 항목이 없으므로 안전합니다.
    let uart = unsafe { Uart::new(PL011_BASE_ADDRESS) };
    logger::init(uart, LevelFilter::Trace).unwrap();

    info!("main({x0:#x}, {x1:#x}, {x2:#x}, {x3:#x})");

    assert_eq!(x1, 42);

    system_off::<Hvc>().unwrap();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("{info}");
    system_off::<Hvc>().unwrap();
    loop {}
}
```

> **발표자 노트**
>
> - 패닉 핸들러가 이제 패닉의 세부정보를 기록할 수 있습니다.
> - `src/bare-metal/aps/examples`에서 `make qemu_logger`를 사용하여 QEMU에서 예를 실행합니다.
