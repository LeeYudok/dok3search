# 타입 별칭

타입 별칭은 다른 타입의 이름을 생성합니다. 두 타입은 서로 바꿔서 사용할 수 있습니다.

``` rust
enum CarryableConcreteItem {
    Left,
    Right,
}

type Item = CarryableConcreteItem;

// 별칭은 다음과 같이 길고 복잡한 타입에서 더 유용합니다.
use std::cell::RefCell;
use std::sync::{Arc, RwLock};
type PlayerInventory = RwLock<Vec<Arc<RefCell<Item>>>>;
```

> **발표자 노트**
>
> This slide should take about 2 minutes.
>
> C 프로그래머는 이를 `typedef`와 유사한 것으로 인식합니다.
