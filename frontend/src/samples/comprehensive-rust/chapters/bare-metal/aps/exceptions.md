# 예외

AArch64는 4개 상태(SP0을 사용하는 현재 EL, SPx를 사용하는 현재 EL, AArch64를 사용하는 하위 EL, AArch32를 사용하는 하위 EL)의 4가지 예외 타입(동기, IRQ, FIQ, SError)에 대해 16개 항목이 있는 예외 벡터 테이블을 정의합니다. Rust 코드를 호출하기 전에 휘발성 레지스터를 스택에 저장하기 위해 어셈블리에서 이를 구현합니다.

컴파일 오류 예제

``` rust
use log::error;
use smccc::psci::system_off;
use smccc::Hvc;

#[no_mangle]
extern "C" fn sync_exception_current(_elr: u64, _spsr: u64) {
    error!("sync_exception_current");
    system_off::<Hvc>().unwrap();
}

#[no_mangle]
extern "C" fn irq_current(_elr: u64, _spsr: u64) {
    error!("irq_current");
    system_off::<Hvc>().unwrap();
}

#[no_mangle]
extern "C" fn fiq_current(_elr: u64, _spsr: u64) {
    error!("fiq_current");
    system_off::<Hvc>().unwrap();
}

#[no_mangle]
extern "C" fn serr_current(_elr: u64, _spsr: u64) {
    error!("serr_current");
    system_off::<Hvc>().unwrap();
}

#[no_mangle]
extern "C" fn sync_lower(_elr: u64, _spsr: u64) {
    error!("sync_lower");
    system_off::<Hvc>().unwrap();
}

#[no_mangle]
extern "C" fn irq_lower(_elr: u64, _spsr: u64) {
    error!("irq_lower");
    system_off::<Hvc>().unwrap();
}

#[no_mangle]
extern "C" fn fiq_lower(_elr: u64, _spsr: u64) {
    error!("fiq_lower");
    system_off::<Hvc>().unwrap();
}

#[no_mangle]
extern "C" fn serr_lower(_elr: u64, _spsr: u64) {
    error!("serr_lower");
    system_off::<Hvc>().unwrap();
}
```

> **발표자 노트**
>
> - EL은 예외 수준입니다. 오늘 오후의 모든 예는 EL1에서 실행됩니다.
> - 편의상 현재 EL 예외의 SP0과 SPx를 구별하거나 하위 EL 예외의 AArch32와 AArch64를 구별하지 않습니다.
> - 이 예에서는 예외가 실제로 발생할 것으로 예상되지 않으므로 예외를 기록하고 전원을 끕니다.
> - 예외 핸들러와 기본 실행 컨텍스트는 서로 다른 스레드와 거의 비슷하다고 생각할 수 있습니다. [`Send` 및 `Sync`](../../concurrency/send-sync.md)는 스레드와 마찬가지로 이들 간에 공유할 수 있는 항목을 제어합니다. 예를 들어 예외 핸들러와 프로그램의 나머지 부분 간에 값을 공유하려고 하는데 `Send`이지만 `Sync`는 아닌 경우, `Mutex`와 같은 것으로 래핑하고 정적인 것으로 배치해야 합니다.
