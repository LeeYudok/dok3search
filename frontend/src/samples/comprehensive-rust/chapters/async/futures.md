# Future

[`Future`](https://doc.rust-lang.org/std/future/trait.Future.html)는 트레잇입니다.이 트레잇은 아직 완료되지 않았을 수도 있는 작업을 나타냅니다. Future는 `poll` 함수를 통해 폴링될 수 있으며, 이 함수는 [`Poll`](https://doc.rust-lang.org/std/task/enum.Poll.html)을 반환합니다.

``` rust
#![allow(unused)]
fn main() {
use std::pin::Pin;
use std::task::Context;

pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),
    Pending,
}
}
```

비동기 함수는 `impl Future`를 반환합니다. 새로운 타입을 만들고 이 타입이 `Future`를 구현하게 할 수도 있지만 일반적이지는 않습니다. 예를 들어 `tokio::spawn`가 리턴하는 `JoinHandle`은 `Future`를 구현하며, 이를 통해 생성된 스레드에 join할 수 있습니다.

Future에 `.await`를 호출하면, 해당 Future가 준비될 때까지 현재 비동기 함수가 일시 중지됩니다. 그런 다음 Future가 준비가 되면, 그 값이 `.await` 구문의 값이 됩니다.

> **발표자 노트**
>
> - `Future` 와 `Poll` 타입의 실제 정의는 위에 보이는 그대로 입니다. 링크를 클릭하면 Rust 문서에서 한 번 더 확인할 수 있습니다.
>
> - 본 강의의 목적은 비동기 코드를 작성하는데 있기 때문에, 새로운 비동기 프리미티브를 만드는데 필요한 `Pin`과 `Context`는 다루지 않습니다. 이들에 대해 간단히 설명하자면:
>
>   - `Context`를 사용하면 Future가 이벤트가 발생할 때 다시 폴링되도록 예약할 수 있습니다.
>
>   - `Pin`을 사용하면 메모리에서 Future의 위치가 고정되기 때문에 해당 future의 포인터가 항상 유효하게 유지됩니다. 이는 `.await` 후에 참조를 유효한 상태로 유지하기 위해 필요합니다.
