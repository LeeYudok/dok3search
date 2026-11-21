# HAL 크레이트들

다양한 주변 장치에 대한 래퍼를 제공하는 [HAL 크레이트](https://github.com/rust-embedded/awesome-embedded-rust#hal-implementation-crates)들이 있습니다. 이 크레이트들은 일반적으로 [`embedded-hal`](https://crates.io/crates/embedded-hal)의 트레잇을 구현합니다.

컴파일 오류 예제

``` rust
#![no_main]
#![no_std]

extern crate panic_halt as _;

use cortex_m_rt::entry;
use nrf52833_hal::gpio::{p0, Level};
use nrf52833_hal::pac::Peripherals;
use nrf52833_hal::prelude::*;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();

    // GPIO 포트 0의 HAL 래퍼를 만듭니다.
    let gpio0 = p0::Parts::new(p.P0);

    // GPIO 0 핀 21 및 28을 푸시-풀 출력으로 구성합니다.
    let mut col1 = gpio0.p0_28.into_push_pull_output(Level::High);
    let mut row1 = gpio0.p0_21.into_push_pull_output(Level::Low);

    // 핀 28을 낮게, 핀 21을 높게 설정하여 LED를 켭니다.
    col1.set_low().unwrap();
    row1.set_high().unwrap();

    loop {}
}
```

> **발표자 노트**
>
> - `set_low` 및 `set_high`는 `embedded_hal` `OutputPin` 트레잇의 메서드입니다.
> - 다양한 STM32, GD32, nRF, NXP, MSP430, AVR, PIC 마이크로컨트롤러를 비롯한 많은 Cortex-M 및 RISC-V 기기를 위한 HAL 크레이트가 있습니다
>
> 아래 명령어로 예제 코드를 실행하세요.
>
> ``` sh
> cargo embed --bin hal
> ```
