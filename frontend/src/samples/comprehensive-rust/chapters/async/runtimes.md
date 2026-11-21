# 비동기 런타임들

비동기 \_런타임_은 *리액터* (비동기식 작업 실행을 지원)와 *실행자* (futures를 실행)의 두 가지 역할을 합니다. Rust 언어 자체에서 기본 제공하는 비동기 런타임은 없습니다. 그러나 다음과 같은 비동기 런타임 크레잇들이 있습니다.

- [Tokio](https://tokio.rs/): performant, with a well-developed ecosystem of functionality like [Hyper](https://hyper.rs/) for HTTP or [Tonic](https://github.com/hyperium/tonic) for gRPC.
- [async-std](https://async.rs/): aims to be a “std for async”, and includes a basic runtime in `async::task`.
- [smol](https://docs.rs/smol/latest/smol/): simple and lightweight

여러 대규모 애플리케이션에는 자체 런타임이 있는 경우도 있습니다. 예들 들어 [Fuchsia](https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/src/lib/fuchsia-async/src/lib.rs)가 있습니다.

> **발표자 노트**
>
> - Rust 플레이그라운드에서는 위에 나열된 비동기 런타임 중에서 Tokio만 사용할 수 있습니다. 또한 Rust 플레이그라운드는 I/O를 허용하지 않으므로 async를 가지고 할 수 있는 많은 흥미로운 작업들이 불가능 합니다.
>
> - Futures는 실행자가 폴링하지 않는 한 아무것도 하지 않는다는 점에서(I/O 작업조차 시작하지 않음) “비활성” 상태입니다. 이는 사용되지 않는 경우에도 완료될 때 까지 실행되는, 자바 스크립트의 promise와 다릅니다.
