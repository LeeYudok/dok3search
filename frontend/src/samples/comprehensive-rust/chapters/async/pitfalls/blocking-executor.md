# 실행자(executor)를 블록시킴

대부분의 비동기 런타임은 IO 작업만 동시에 실행되도록 허용합니다. 즉, CPU를 블럭하는 태스크가 있는 경우, 이는 실행자(executor)를 블럭하게 되며, 그 결과로 다른 태스크가 실행되지 않습니다. 이 문제를 해결하는 간단한 방법은, 항상 async를 지원하는 메서드를 사용하는 것입니다.

컴파일 오류 예제

``` rust
use futures::future::join_all;
use std::time::Instant;

async fn sleep_ms(start: &Instant, id: u64, duration_ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(duration_ms));
    println!(
        "future {id}은(는) {duration_ms}밀리초 동안 절전 모드였고 {}밀리초 후에 완료됨",
        start.elapsed().as_millis()
    );
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let start = Instant::now();
    let sleep_futures = (1..=10).map(|t| sleep_ms(&start, t, t * 10));
    join_all(sleep_futures).await;
}
```

> **발표자 노트**
>
> - 코드를 실행하여 sleep들이 동시에 진행되지 않고 순차적으로으로 진행되는지 확인하세요.
>
> - `flavor`를 `"current_thread"` 로 설정하면 모든 태스크가 하나의 스레드에서 수행됩니다. 이렇게 하면 문제 상황이 더 분명히 드러납니다. 그러나 이 버그는 멀티스레드인 경우에도 여전히 존재합니다.
>
> - `std::thread::sleep`을 `tokio::time::sleep`으로 바꾸고 그 결과를 `await`해 보세요.
>
> - 또 다른 해결 방법은 `tokio::task::spawn_blocking`입니다. 이는 실제 스레드를 생성하고, 그 스레드에 대한 핸들을 future로 변환함으로써 실행자가 블록되는 것을 막습니다.
>
> - 태스크를 OS 스레드라고 생각하면 안 됩니다. 태스크와 OS스레드는 일대일 매핑 관계에 있지 않습니다. 대부분의 실행자는 하나의 OS 스레드에서 최대한 많은 태스크를 수행하도록 설계되어 있습니다. 이점은 FFI를 통해 다른 라이브러리와 상호작용할 때 특히 문제가 됩니다. 예를 들어, 해당 라이브러리가 스레드 로컬 저장소를 이용하거나 특정 OS 스레드에 매핑될 수 있습니다(예: CUDA). 이러한 상황에서는 `tokio::task::spawn_blocking`을 사용하는 것이 좋습니다.
>
> - 동기화 뮤텍스를 주의해서 사용하세요. `.await` 위에 뮤텍스를 적용하면 다른 작업이 차단될 수 있으며 해당 작업은 동일한 스레드에서 실행 중일 수 있습니다.
