# 원시 MMIO

대부분의 마이크로컨트롤러는 메모리 매핑 IO를 통해 주변기기에 액세스합니다. micro:bit에서 LED를 켜보겠습니다.

컴파일 오류 예제

``` rust
#![no_main]
#![no_std]

extern crate panic_halt as _;

mod interrupts;

use core::mem::size_of;
use cortex_m_rt::entry;

/// GPIO 포트 0 주변기기 주소
const GPIO_P0: usize = 0x5000_0000;

// GPIO 주변기기 오프셋
const PIN_CNF: usize = 0x700;
const OUTSET: usize = 0x508;
const OUTCLR: usize = 0x50c;

// PIN_CNF 필드
const DIR_OUTPUT: u32 = 0x1;
const INPUT_DISCONNECT: u32 = 0x1 << 1;
const PULL_DISABLED: u32 = 0x0 << 2;
const DRIVE_S0S1: u32 = 0x0 << 8;
const SENSE_DISABLED: u32 = 0x0 << 16;

#[entry]
fn main() -> ! {
    // GPIO 0 핀 21 및 28을 푸시-풀 출력으로 구성합니다.
    let pin_cnf_21 = (GPIO_P0 + PIN_CNF + 21 * size_of::<u32>()) as *mut u32;
    let pin_cnf_28 = (GPIO_P0 + PIN_CNF + 28 * size_of::<u32>()) as *mut u32;
    // 유효한 주변기기 제어 레지스터에 대한 포인터이고
    // 별칭이 없으므로 안전합니다.
    unsafe {
        pin_cnf_21.write_volatile(
            DIR_OUTPUT
                | INPUT_DISCONNECT
                | PULL_DISABLED
                | DRIVE_S0S1
                | SENSE_DISABLED,
        );
        pin_cnf_28.write_volatile(
            DIR_OUTPUT
                | INPUT_DISCONNECT
                | PULL_DISABLED
                | DRIVE_S0S1
                | SENSE_DISABLED,
        );
    }

    // 핀 28을 낮게, 핀 21을 높게 설정하여 LED를 켭니다.
    let gpio0_outset = (GPIO_P0 + OUTSET) as *mut u32;
    let gpio0_outclr = (GPIO_P0 + OUTCLR) as *mut u32;
    // 유효한 주변기기 제어 레지스터에 대한 포인터이고
    // 별칭이 없으므로 안전합니다.
    unsafe {
        gpio0_outclr.write_volatile(1 << 28);
        gpio0_outset.write_volatile(1 << 21);
    }

    loop {}
}
```

> **발표자 노트**
>
> - GPIO 0 핀 21은 LED 매트릭스의 첫 번째 열에 연결되고 핀 28은 첫 번째 행에 연결됩니다.
>
> 아래 명령어로 예제 코드를 실행하세요.
>
> ``` sh
> cargo embed --bin mmio
> ```
