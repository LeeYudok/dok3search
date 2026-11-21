# 주변기기 액세스 크레이트

[`svd2rust`](https://crates.io/crates/svd2rust) 크레이트를 이용하면 메모리 매핑된 주변장치를 기술하는 [CMSIS-SVD](https://www.keil.com/pack/doc/CMSIS/SVD/html/index.html) 파일로부터 Rust 래퍼를 생성할 수 있습니다.

컴파일 오류 예제

``` rust
#![no_main]
#![no_std]

extern crate panic_halt as _;

use cortex_m_rt::entry;
use nrf52833_pac::Peripherals;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let gpio0 = p.P0;

    // GPIO 0 핀 21 및 28을 푸시-풀 출력으로 구성합니다.
    gpio0.pin_cnf[21].write(|w| {
        w.dir().output();
        w.input().disconnect();
        w.pull().disabled();
        w.drive().s0s1();
        w.sense().disabled();
        w
    });
    gpio0.pin_cnf[28].write(|w| {
        w.dir().output();
        w.input().disconnect();
        w.pull().disabled();
        w.drive().s0s1();
        w.sense().disabled();
        w
    });

    // 핀 28을 낮게, 핀 21을 높게 설정하여 LED를 켭니다.
    gpio0.outclr.write(|w| w.pin28().clear());
    gpio0.outset.write(|w| w.pin21().set());

    loop {}
}
```

> **발표자 노트**
>
> - SVD(System View Description) 파일은 일반적으로 실리콘 공급업체에서 제공하는 XML 파일로, 기기의 메모리 맵을 기술합니다.
>   - 주변기기, 레지스터, 필드, 값으로 구성되며 이름, 설명, 주소 등이 포함됩니다.
>   - SVD 파일에는 버그가 있을 수 있고 불완전하기 때문에, 이러한 문제들을 패치하는 다양한 프로젝트들이 있습니다.
> - `cortex-m-rt`는 무엇보다도 벡터 테이블을 제공합니다.
> - `cargo install cargo-binutils`을 실행한 후, `cargo objdump --bin pac -- -d --no-show-raw-insn`을 실행하여 생성된 바이너리의 내용을 확인할 수 있습니다.
>
> 아래 명령어로 예제 코드를 실행하세요.
>
> ``` sh
> cargo embed --bin pac
> ```
