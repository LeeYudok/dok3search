# 마이크로컨트롤러

`cortex_m_rt` 크레이트는 Cortex M 마이크로컨트롤러를 초기화 하는 핸들러를 제공합니다.

컴파일 오류 예제

``` rust
#![no_main]
#![no_std]

extern crate panic_halt as _;

mod interrupts;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    loop {}
}
```

이제, 주변장치에 액세스하는 방법을 알아보겠습니다. 가장 기계에 가까운 낮은 단계에서 시작해서 점점 추상화 수준을 올리겠습니다.

> **발표자 노트**
>
> - `cortex_m_rt::entry` 매크로는 진입점으로 사용되는 함수가`fn() -> !` 타입(즉, 리턴하지 않는)임을 요구합니다. 만약, 리턴하게 되면, 프로그램 수행 후 리셋 핸들러로 돌아가게 되는 것인데 이는 말이 되지 않기 때문입니다.
> - `cargo embed --bin minimal`을 사용하여 예시를 실행합니다.
