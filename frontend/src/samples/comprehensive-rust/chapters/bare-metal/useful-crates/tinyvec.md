# `tinyvec`

힙에 메모리 할당하지 않고 크기 조절이 가능한 컨테이너(예: `Vec` 같은)가 필요할 때가 있습니다. [`tinyvec`](https://crates.io/crates/tinyvec)을 사용하면 됩니다. `tinyvec`에서 벡터는 배열 또는 슬라이스로부터 생성이 되며, 이들은 정적으로 할당되었거나 스택에 할당되어 있을 수 있습니다.`tinyvec`은 현재 벡터 안에 얼마나 많은 엘리먼트들이 존재하는 지를 추적하고 있으며, 할당된 양보다 더 많이 사용하려고 하면 패닉을 발생시킵니다.

컴파일 오류 예제

``` rust
use tinyvec::{array_vec, ArrayVec};

fn main() {
    let mut numbers: ArrayVec<[u32; 5]> = array_vec!(42, 66);
    println!("{numbers:?}");
    numbers.push(7);
    println!("{numbers:?}");
    numbers.remove(1);
    println!("{numbers:?}");
}
```

> **발표자 노트**
>
> - `tinyvec`를 사용하려면 엘리먼트의 타입이 `Default`를 통해 초기화 될 수 있어야 합니다.
> - Rust 플레이그라운드에는 `tinyvec`가 포함되어 있으므로 이 예시는 인라인으로 실행됩니다.
