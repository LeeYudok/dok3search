# `spin`

`std::sync::Mutex` 및 `std::sync`의 기타 동기화 프리미티브는 `core` 또는 `alloc`에서 사용할 수 없습니다. 그러면 어떻게 동기화 또는 interior mutability와 같은 기능이 필요할 경우 어떻게 해야 할까요?

[`spin`](https://crates.io/crates/spin) 크레이트는 이러한 동기화 프리미티브들을 스핀록으로 구현하고 있습니다.

컴파일 오류 예제

``` rust
use spin::mutex::SpinMutex;

static counter: SpinMutex<u32> = SpinMutex::new(0);

fn main() {
    println!("count: {}", counter.lock());
    *counter.lock() += 2;
    println!("count: {}", counter.lock());
}
```

> **발표자 노트**
>
> - 인터럽트 핸들러에서 락을 걸 경우, 교착 상태가 발생하지 않도록 주의하세요.
> - `spin` also has a ticket lock mutex implementation; equivalents of `RwLock`, `Barrier` and `Once` from `std::sync`; and `Lazy` for lazy initialisation.
> - [`once_cell`](https://crates.io/crates/once_cell) 크레이트에는 지연된 초기화를 위한 몇 가지 유용한 타입이 있는데 `spin::once::Once`와는 약간 다른 접근 방식을 사용합니다.
> - Rust 플레이그라운드에는 `spin`이 포함되어 있으므로 이 예시는 인라인으로 실행됩니다.
