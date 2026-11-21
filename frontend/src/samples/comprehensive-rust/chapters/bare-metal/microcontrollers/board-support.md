# Board support crates

보드 지원 크레이트들은, 특정 보드를 더 손쉽게 사용할 수 있게 해 주는 더 높은 수준의 추상화를 제공합니다.

컴파일 오류 예제

``` rust
#![no_main]
#![no_std]

extern crate panic_halt as _;

use cortex_m_rt::entry;
use microbit::hal::prelude::*;
use microbit::Board;

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();

    board.display_pins.col1.set_low().unwrap();
    board.display_pins.row1.set_high().unwrap();

    loop {}
}
```

> **발표자 노트**
>
> - 이 경우 보드 지원 크레이트는 좀 더 직관적인 이름들과 적당한 수준의 초기화를 제공합니다.
> - 이 크레이트는 마이크로컨트롤 밖에 있는 (즉, 보드에 설치된) 장치에 대한 드라이버도 포함할 수 있습니다.
>   - `microbit-v2`에는 LED 매트릭스를 위한 간단한 드라이버가 포함되어 있습니다.
>
> 아래 명령어로 예제 코드를 실행하세요.
>
> ``` sh
> cargo embed --bin board_support
> ```
