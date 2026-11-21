# 나침반

I2C 나침반에서 방향을 읽고 판독값을 직렬 포트에 기록하세요. 시간이 있으면 어떻게든 LED에 표시하거나 버튼을 사용하세요.

힌트:

- [`lsm303agr`](https://docs.rs/lsm303agr/latest/lsm303agr/) 및 [`microbit-v2`](https://docs.rs/microbit-v2/latest/microbit/) 크레이트 및 [micro:bit 하드웨어](https://tech.microbit.org/hardware/)의 문서를 확인하세요.
- LSM303AGR 관성 측정 장치는 내부 I2C 버스에 연결됩니다.
- TWI는 I2C의 또 다른 이름이므로 I2C 마스터 주변기기는 TWIM이라고 합니다.
- LSM303AGR 드라이버에는 `embedded_hal::blocking::i2c::WriteRead` 트레잇을 구현하는 것이 필요합니다. [`microbit::hal::Twim`](https://docs.rs/microbit-v2/latest/microbit/hal/struct.Twim.html) 구조체가 이를 구현합니다.
- 다양한 핀과 주변기기에 관한 필드가 있는 [`microbit::Board`](https://docs.rs/microbit-v2/latest/microbit/struct.Board.html) 구조체가 있습니다.
- 원하는 경우 [nRF52833 데이터시트](https://infocenter.nordicsemi.com/pdf/nRF52833_PS_v1.5.pdf)를 확인할 수도 있지만 이 연습에서 반드시 필요한 것은 아닙니다.

[연습 템플릿](https://google.github.io/comprehensive-rust/ko/comprehensive-rust-exercises.zip)을 다운로드하고 `compass` 디렉터리에서 다음 파일을 찾습니다.

*src/main.rs*:

컴파일 오류 예제

``` rust
#![no_main]
#![no_std]

extern crate panic_halt as _;

use core::fmt::Write;
use cortex_m_rt::entry;
use microbit::{hal::uarte::{Baudrate, Parity, Uarte}, Board};

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();

    // Configure serial port.
    let mut serial = Uarte::new(
        board.UARTE0,
        board.uart.into(),
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    );

    // Use the system timer as a delay provider.
    let mut delay = Delay::new(board.SYST);

    // Set up the I2C controller and Inertial Measurement Unit.
    // TODO

    writeln!(serial, "Ready.").unwrap();

    loop {
        // Read compass data and log it to the serial port.
        // TODO
    }
}
```

*Cargo.toml* (변경할 필요가 없음):

``` toml
[workspace]

[package]
name = "compass"
version = "0.1.0"
edition = "2021"
publish = false

[dependencies]
cortex-m-rt = "0.7.3"
embedded-hal = "1.0.0"
lsm303agr = "0.3.0"
microbit-v2 = "0.13.0"
panic-halt = "0.2.0"
```

*Embed.toml* (변경할 필요가 없음):

``` toml
[default.general]
chip = "nrf52833_xxAA"

[debug.gdb]
enabled = true

[debug.reset]
halt_afterwards = true
```

*.cargo/config.toml* (변경할 필요가 없음):

``` toml
[build]
target = "thumbv7em-none-eabihf" # Cortex-M4F

[target.'cfg(all(target_arch = "arm", target_os = "none"))']
rustflags = ["-C", "link-arg=-Tlink.x"]
```

다음을 사용하여 Linux에서 직렬 출력을 확인하세요.

``` sh
picocom --baud 115200 --imap lfcrlf /dev/ttyACM0
```

또는 다음과 같이 Mac OS(기기 이름은 약간 다를 수 있음)에서 확인하세요.

``` sh
picocom --baud 115200 --imap lfcrlf /dev/tty.usbmodem14502
```

Ctrl+A Ctrl+Q를 사용하여 picocom을 종료합니다.
