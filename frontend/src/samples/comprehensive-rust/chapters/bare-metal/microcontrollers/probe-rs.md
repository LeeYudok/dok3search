# `probe-rs` and `cargo-embed`

[probe-rs](https://probe.rs/)는 임베디드 시스템을 위한 도구 모임입니다. OpenOCD와 비슷하지만, Rust에 더 잘 통합되어 있습니다.

- SWD (Serial Wire Debug) and JTAG via CMSIS-DAP, ST-Link and J-Link probes
- GDB 스텁 및 Microsoft DAP(디버그 어댑터 프로토콜) 서버
- Cargo 통합

`cargo-embed` is a cargo subcommand to build and flash binaries, log RTT (Real Time Transfers) output and connect GDB. It’s configured by an `Embed.toml` file in your project directory.

> **발표자 노트**
>
> - [CMSIS-DAP](https://arm-software.github.io/CMSIS_5/DAP/html/index.html)는 Arm에서 정의한 프로토콜로, USB를 통해 Arm Cortex 프로세서의 CoreSight 디버그 액세스 포트에 접근할 수 있게 해 줍니다. BBC micro:bit에 있는 온보드 디버거도 이 프로토콜을 지원합니다.
> - ST-Link는 ST Microelectronics사에서 만든 in-circuit 디버거들이며, J-Link는 SEGGER사의 in-circuit 디버거들입니다.
> - 디버그 액세스 포트의 물리적인 구성은 일반적으로 5핀 JTAG 인터페이스 혹은, 2핀 Serial Wire Debug 인터페이스 입니다.
> - probe-rs는 라이브러리로 구현되어 있어서, 다른 도구들에 통합되기가 쉽습니다.
> - [Microsoft 디버그 어댑터 프로토콜](https://microsoft.github.io/debug-adapter-protocol/)을 사용하면 VSCode나 다른 IDE 상에서 마이크로컨트롤러에서 수행중인 코드를 디버깅 할 수 있습니다.
> - cargo-embed는 probe-rs 라이브러리를 사용하여 빌드된 바이너리입니다.
> - RTT(Real Time Transfers)는 여러 링 버퍼를 통해 디버그 호스트와 타겟 간에 데이터를 전송하는 메커니즘입니다.
