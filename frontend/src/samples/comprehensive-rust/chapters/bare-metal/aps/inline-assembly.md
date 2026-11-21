# 인라인 어셈블리

Sometimes we need to use assembly to do things that aren’t possible with Rust code. For example, to make an HVC (hypervisor call) to tell the firmware to power off the system:

컴파일 오류 예제

``` rust
#![no_main]
#![no_std]

use core::arch::asm;
use core::panic::PanicInfo;

mod exceptions;

const PSCI_SYSTEM_OFF: u32 = 0x84000008;

#[no_mangle]
extern "C" fn main(_x0: u64, _x1: u64, _x2: u64, _x3: u64) {
    // 선언된 레지스터만 사용하고 메모리로는
    // 아무것도 하지 않으므로 안전합니다.
    unsafe {
        asm!("hvc #0",
            inout("w0") PSCI_SYSTEM_OFF => _,
            inout("w1") 0 => _,
            inout("w2") 0 => _,
            inout("w3") 0 => _,
            inout("w4") 0 => _,
            inout("w5") 0 => _,
            inout("w6") 0 => _,
            inout("w7") 0 => _,
            options(nomem, nostack)
        );
    }

    loop {}
}
```

실제로 이를 실행하려면 이러한 모든 함수를 위한 래퍼가 포함된 [`smccc`](https://crates.io/crates/smccc) 크레이트를 사용하세요.

> **발표자 노트**
>
> - PSCI (Power State Coordination Interface)는 시스템 및 CPU 전원 상태를 관리하는 Arm의 표준 인터페이스입니다. 이 인터페이스는 EL3 펌웨어와 하이퍼바이저에 의해 구현됩니다.
> - `0 => _` 문법은 인라인 어셈블리 코드를 실행하기 전에 레지스터를 0으로 초기화하고 그 후에는 그 레지스터의 값을 무시함을 의미합니다. 호출 시 레지스터의 값이 덮어 써질 수 있으므로 `in` 대신 `inout`을 사용해야 합니다.
> - 이 `main` 함수는 `#[no_mangle]` 및 `extern "C"`여야 합니다. 왜냐하면 이 함수는 Rust 코드가 아닌, 어셈블러로 작성된 `entry.S`에서 호출되기 때문입니다.
> - `_x0`–`_x3`는 `x0`에서 `x3` 레지스터들의 값입니다. 이 레지스터들은 일반적으로 부트로더에서 디바이스 트리에 대한 포인터 등을 전달할 때 사용됩니다. 표준 aarch64 호출 규약(`extern "C"`에서 사용하도록 지정)에 따라 레지스터 `x0`에서 `x7`이 함수에 전달된 처음 8개 인수에 사용되므로 `entry.S`는 이러한 레지스터를 변경하지 않는지 확인하는 것 외에는 특별히 할 작업이 없습니다.
> - `src/bare-metal/aps/examples`에서 `make qemu_psci`를 입력하면 예제 코드가 QEMU에서 수행됩니다.
