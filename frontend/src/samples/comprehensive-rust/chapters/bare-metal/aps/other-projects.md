# 다른 프로젝트

- [oreboot](https://github.com/oreboot/oreboot)
  - “coreboot without the C”
  - Supports x86, aarch64 and RISC-V.
  - Relies on LinuxBoot rather than having many drivers itself.
- [Rust RaspberryPi OS tutorial](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials)
  - Initialisation, UART driver, simple bootloader, JTAG, exception levels, exception handling, page tables
  - Some dodginess around cache maintenance and initialisation in Rust, not necessarily a good example to copy for production code.
- [`cargo-call-stack`](https://crates.io/crates/cargo-call-stack)
  - Static analysis to determine maximum stack usage.

> **발표자 노트**
>
> - RaspberryPi OS 튜토리얼에서는 MMU와 캐시가 사용 설정되기 전에 Rust 코드를 실행합니다. 이렇게 하면 메모리(예: 스택)를 읽고 쓸 수 있습니다. 하지만 다음과 같은 예외가 적용됩니다.
>   - MMU와 캐시가 없으면 정렬되지 않은 액세스에 오류가 발생합니다. 컴파일러가 정렬되지 않은 액세스를 생성하지 않도록 `+strict-align`을 설정하는 `aarch64-unknown-none`으로 빌드되므로 문제가 없지만, 일반적으로 반드시 그런 것은 아닙니다.
>   - VM에서 실행한다면 캐시 일관성 문제가 발생할 수 있습니다. 문제는 VM은 캐시가 사용 중지된 상태로 메모리에 직접 액세스하는 반면 호스트에는 동일한 메모리에 대해 캐시할 수 있는 별칭이 있다는 점입니다. 호스트가 메모리에 명시적으로 액세스하지 않더라도 추측 액세스는 캐시 채우기로 이어질 수 있으며 둘 중 하나의 변경사항이 손실됩니다. 이번에도 하이퍼바이저 없이 하드웨어에서 직접 실행되는 이 특정 경우에는 문제가 없지만, 일반적으로 좋은 패턴은 아닙니다.
