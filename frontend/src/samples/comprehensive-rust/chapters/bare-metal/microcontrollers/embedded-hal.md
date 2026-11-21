# `embedded-hal`

[`embedded-hal`](https://crates.io/crates/embedded-hal) 크레이트는 다양한 마이크로컨트롤러에서 공통적으로 찾아볼 수 있는 주변기기를 추상회 하는 다양한 트레잇을 제공합니다.

- GPIO
- ADC
- I2C, SPI, UART, CAN
- RNG
- 타이머
- 워치독

그러면 다른 크레이트는 이 트레잇들을 활용하여 [드라이버](https://github.com/rust-embedded/awesome-embedded-rust#driver-crates)를 구현합니다. 예를 들어 가속도계 드라이버를 구현할 떄 I2C 또는 SPI 버스 구현을 사용할 수 있습니다.

> **발표자 노트**
>
> - 라스베리 파이에서 돌아가는 리눅스 같은 플랫폼 뿐만 아니라 다른 여러 마이크로컨트롤러에 대한 구현이 있습니다.
> - `embedded-hal`의 ‘async’ 버전에 관한 작업이 진행 중이지만 아직 안정적이지 않습니다.
