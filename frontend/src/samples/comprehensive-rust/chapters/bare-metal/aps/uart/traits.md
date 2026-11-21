# More traits

`Debug` 트레잇을 상속했습니다. 몇 가지 트레잇을 추가로 더 구현하는 것도 유용할 수 있습니다.

컴파일 오류 예제

``` rust
use core::fmt::{self, Write};

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.as_bytes() {
            self.write_byte(*c);
        }
        Ok(())
    }
}

// 모든 컨텍스트에서 액세스할 수 있는 기기 메모리에 대한
// 포인터만 포함하므로 안전합니다.
unsafe impl Send for Uart {}
```

> **발표자 노트**
>
> - `Write`를 구현하면 `write!` 및 `writeln!` 매크로를 `Uart` 타입과 함께 사용할 수 있습니다.
> - `src/bare-metal/aps/examples`에서 `make qemu_minimal`을 사용하여 QEMU에서 예를 실행합니다.
