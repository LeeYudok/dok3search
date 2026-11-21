# `buddy_system_allocator`

[`buddy_system_allocator`](https://crates.io/crates/buddy_system_allocator)는 버디 시스템 할당자를 구현하는 서드 파티 크레이트입니다. 이 크레이트의 [`LockedHeap`](https://docs.rs/buddy_system_allocator/0.9.0/buddy_system_allocator/struct.LockedHeap.html)은 [`GlobalAlloc`](https://doc.rust-lang.org/core/alloc/trait.GlobalAlloc.html)를 구현합니다. 따라서 여러분은 버디 시스템 할당자를 ‘alloc’ 크레이트를 통해서 사용할 수 있습니다([이전](../alloc.md)에 확인함). 또는 다른 주소 공간을 할당하는 데 사용할 수 있습니다. 예를 들어 PCI BAR에 MMIO 공간을 할당할 수 있습니다.

컴파일 오류 예제

``` rust
use buddy_system_allocator::FrameAllocator;
use core::alloc::Layout;

fn main() {
    let mut allocator = FrameAllocator::<32>::new();
    allocator.add_frame(0x200_0000, 0x400_0000);

    let layout = Layout::from_size_align(0x100, 0x100).unwrap();
    let bar = allocator
        .alloc_aligned(layout)
        .expect("Failed to allocate 0x100 byte MMIO region");
    println!("Allocated 0x100 byte MMIO region at {:#x}", bar);
}
```

> **발표자 노트**
>
> - PCI BAR는 BAR영역의 크기에 맞추어 정렬됩니다.
> - `src/bare-metal/useful-crates/allocator-example/`에서 `cargo run`을 사용하여 예시를 실행합니다(종속성 문제로 인해 플레이그라운드에서는 실행되지 않습니다).
