# `aarch64-paging`

[`aarch64-paging`](https://crates.io/crates/aarch64-paging) 크레이트를 사용하면 AArch64 가상 메모리 시스템 아키텍처에 따라 페이지 테이블을 만들 수 있습니다.

컴파일 오류 예제

``` rust
use aarch64_paging::{
    idmap::IdMap,
    paging::{Attributes, MemoryRegion},
};

const ASID: usize = 1;
const ROOT_LEVEL: usize = 1;

// ID 매핑을 사용하여 새 페이지 테이블을 만듭니다.
let mut idmap = IdMap::new(ASID, ROOT_LEVEL);
// 2MiB 메모리 영역을 읽기 전용으로 매핑합니다.
idmap.map_range(
    &MemoryRegion::new(0x80200000, 0x80400000),
    Attributes::NORMAL | Attributes::NON_GLOBAL | Attributes::READ_ONLY,
).unwrap();
// `TTBR0_EL1`을 설정하여 페이지 테이블을 활성화합니다.
idmap.activate();
```

> **발표자 노트**
>
> - 현재는 EL1만 지원하지만 다른 익셉션 레벨(Exception Level: EL)도 어렵지 않게 추가할 수 있습니다.
> - Android에서 [보호된 VM 펌웨어](https://cs.android.com/android/platform/superproject/+/master:packages/modules/Virtualization/pvmfw/)에 사용됩니다.
> - 이 예시를 간단하게 실행하는 방법은 없습니다. 실제 하드웨어 또는 QEMU에서 실행해야 하기 때문입니다.
