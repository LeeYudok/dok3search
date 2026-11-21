# `alloc`

`alloc`을 사용하려면 [전역 (힙) 할당자](https://doc.rust-lang.org/stable/std/alloc/trait.GlobalAlloc.html)를 구현해야 합니다.

컴파일 오류 예제

``` rust
#![no_main]
#![no_std]

extern crate alloc;
extern crate panic_halt as _;

use alloc::string::ToString;
use alloc::vec::Vec;
use buddy_system_allocator::LockedHeap;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::<32>::new();

static mut HEAP: [u8; 65536] = [0; 65536];

pub fn entry() {
    // `HEAP`이 여기서만 사용되고 `entry`가 한 번만 호출되므로 안전합니다.
    unsafe {
        // 할당자에게 할당할 메모리를 제공합니다.
        HEAP_ALLOCATOR.lock().init(HEAP.as_mut_ptr() as usize, HEAP.len());
    }

    // 이제 힙 할당이 필요한 작업을 할 수 있습니다.
    let mut v = Vec::new();
    v.push("A string".to_string());
}
```

> **발표자 노트**
>
> - `buddy_system_allocator`는 간단한 버디 시스템 할당자를 구현하는 서드 파티 크레이트입니다. 이 외에도, 다른 크레이트를 사용하거나, 직접 할당자를 만들거나, 이미 존재하는 다른 할당자에 후킹할 수 있습니다.
> - `LockHeap` 타입의 const 매개변수는 할당자의 최대 크기를 2진수로 표현했을 때의 자릿수입니다. 즉, 이 경우처럼 32인 경우 최대 2\*\*32바이트 크기의 영역을 다룰 수 있습니다.
> - 한 바이너리에서 `alloc`에 의존하는 크레이트가 하나라도 있다면 바이너리 전체에서 전역 할당자가 반드시 하나 존재해야 합니다. 일반적으로 전역 할당자를 선언하는 작업은 최상위 바이너리 크레이트에서 이루어집니다.
> - `panic_halt` 크레이트가 연결되어 패닉 핸들러를 가져오도록 하려면 `extern crate panic_halt as _`가 필요합니다.
> - 이 예시 코드는 빌드는 되지만, 진입점이 없기 때문에 실행되지는 않습니다.
