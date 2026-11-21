# use, super, self

모듈은 `use`를 사용하여 다른 모듈의 심볼을 내 스코프로 가져올 수 있습니다. 일반적으로 각 모듈의 상단에 다음과 같은 내용이 옵니다:

``` rust
use std::collections::HashSet;
use std::process::abort;
```

## 경로

경로는 아래와 같이 구분합니다:

1.  상대 경로:

    - `foo` 또는 `self::foo`는 현재 모듈 내부의 `foo`를 가리킵니다,
    - `super::foo`는 부모 모듈의 `foo`를 가리킵니다.

2.  절대 경로:

    - `crate::foo`는 현재 크레이트 루트의 `foo`를 가리킵니다,
    - `bar::foo`는 `bar`크레이트의 `foo`를 가리킵니다.

> **발표자 노트**
>
> This slide should take about 8 minutes.
>
> - 더 짧은 경로에서 기호를 ’다시 내보내기’하는 것이 일반적입니다. 예를 들어 크레이트의 최상위 `lib.rs`는
>
>   ``` rust
>   mod storage;
>
>   pub use storage::disk::DiskStorage;
>   pub use storage::network::NetworkStorage;
>   ```
>
>   편리하고 짧은 경로로 다른 크레이트에서 `DiskStorage` 및 `NetworkStorage`를 사용할 수 있도록 할 수 있습니다.
>
> - 대부분의 경우 모듈에 나타나는 항목만 ‘use’ 처리 되어야 합니다. 그러나 트레잇을 구현하는 타입이 이미 범위 내에 있더라도 해당 트레잇에서 메서드를 호출하려면 트레잇이 범위 내에 있어야 합니다. 예를 들어 `Read` 트레잇을 구현하는 타입에서 `read_to_string` 메서드를 사용하려면 `use std::io::Read`를 사용해야 합니다.
>
> - `use` 문에는 와일드 카드(`use std::io::*`)를 사용할 수 있습니다. 이는 가져오는 항목이 명확하지 않고 시간이 지남에 따라 변경될 수 있으므로 권장되지 않습니다.
