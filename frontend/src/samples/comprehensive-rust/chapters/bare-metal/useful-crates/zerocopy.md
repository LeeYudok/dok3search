# `zerocopy`

Fuchsia팀이 만든 [`zerocopy`](https://docs.rs/zerocopy/) 크레이트는 바이트 시퀀스를 다른 타입으로 안전하게 변환하기 위한 트레잇 및 매크로를 제공합니다.

컴파일 오류 예제

``` rust
use zerocopy::AsBytes;

#[repr(u32)]
#[derive(AsBytes, Debug, Default)]
enum RequestType {
    #[default]
    In = 0,
    Out = 1,
    Flush = 4,
}

#[repr(C)]
#[derive(AsBytes, Debug, Default)]
struct VirtioBlockRequest {
    request_type: RequestType,
    reserved: u32,
    sector: u64,
}

fn main() {
    let request = VirtioBlockRequest {
        request_type: RequestType::Flush,
        sector: 42,
        ..Default::default()
    };

    assert_eq!(
        request.as_bytes(),
        &[4, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0]
    );
}
```

이 크레이트는 휘발성(volatile) 읽기 및 쓰기를 사용하지 않으므로 MMIO에 적합하지 않지만, 하드웨어와 공유되거나(예: DMA에서) 외장 인터페이스를 통해 전송되는 구조체를 다루는 데에는 유용할 수 있습니다.

> **발표자 노트**
>
> - 어떤 타입이 가능한 모든 바이트 패턴들에 대해 올바른 값을 가질 때에만 , 그 타입이 `FromBytes`를 구현할 수 있습니다. 그렇게 해서 신뢰할 수 없는 바이트 시퀀스를 안전하게 해당 타입으로 변환할 수 있습니다.
> - 위 코드에서 정의한 타입에 대해 `FromBytes`를 구현하려고 하면 에러가 발생합니다. `RequestType`은 가능한 모든 u32 값을 식별자로 받아들이지 않기 때문입니다. 즉 모든 바이트 패턴이 유효한 `RequestType`값은 아닙니다.
> - `zerocopy::byteorder`에는 바이트 오더에 따른 서로 다른 표현 방식을 지원하는 숫자 타입을 제공합니다.
> - `src/bare-metal/useful-crates/zerocopy-example/`에서 `cargo run`을 사용하여 예시를 실행합니다(종속성 문제로 인해 플레이그라운드에서는 실행되지 않습니다).
