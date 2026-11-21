# 다른 프로젝트

- [RTIC](https://rtic.rs/)
  - “실시간 인터럽트 기반 동시 실행(Real-Time Interrupt-driven Concurrency)”
  - 공유 리소스 관리, 메시지 전달, 태스크 스케줄링, 타이머 대기열 지원
- [Embassy](https://embassy.dev/)
  - 우선순위, 타이머, 네트워킹, USB가 포함된 `async` 실행자
- [TockOS](https://www.tockos.org/documentation/getting-started)
  - 선점형 스케줄링 및 MMU를 지원하는, 보안에 중점을 둔 실시간 운영체제
- [Hubris](https://hubris.oxide.computer/)
  - Oxide Computer Company에서 만든 마이크로커널 기반 실시간 운영체제로, 메모리 보호, 권한이 없이 수행되는 드라이버 등을 지원함.
- [FreeRTOS용 바인딩](https://github.com/lobaro/FreeRTOS-rust)
- `std`가 구현된 플랫폼도 있습니다(예: [esp-idf](https://esp-rs.github.io/book/overview/using-the-standard-library.html)).

> **발표자 노트**
>
> - RTIC는 실시간 운영체제로 볼 수도 있고, 동시성 지원을 위한 프레임워크로 볼 수도 있습니다.
>   - HAL을 포함하지는 않습니다.
>   - 스케줄링은 커널이 아니라 Cortex-M NVIC(Nested Virtual Interrupt Controller)로 구현이 됩니다.
>   - Cortex-M 전용입니다.
> - Google에서는 Titan 보안 키에 사용되는 Haven 마이크로컨트롤러에서 TockOS를 사용합니다.
> - FreeRTOS는 대부분 C로 작성되지만, 애플리케이션을 Rust로 작성할 수 있도록 해 주는 Rust 바인딩이 제공됩니다.
