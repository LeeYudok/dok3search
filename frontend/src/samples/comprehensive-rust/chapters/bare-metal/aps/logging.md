# 로깅

[`log`](https://crates.io/crates/log) 크레이트의 로깅 매크로를 사용할 수 있으면 좋습니다. 이는 `Log` 트레잇을 구현하면 됩니다.

컴파일 오류 예제

``` rust
use crate::pl011::Uart;
use core::fmt::Write;
use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};
use spin::mutex::SpinMutex;

static LOGGER: Logger = Logger { uart: SpinMutex::new(None) };

struct Logger {
    uart: SpinMutex<Option<Uart>>,
}

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        writeln!(
            self.uart.lock().as_mut().unwrap(),
            "[{}] {}",
            record.level(),
            record.args()
        )
        .unwrap();
    }

    fn flush(&self) {}
}

/// UART 로거를 초기화합니다.
pub fn init(uart: Uart, max_level: LevelFilter) -> Result<(), SetLoggerError> {
    LOGGER.uart.lock().replace(uart);

    log::set_logger(&LOGGER)?;
    log::set_max_level(max_level);
    Ok(())
}
```

> **발표자 노트**
>
> - `log`함수 안에서 `unwrap`하는 것은 괜찮습니다.. 왜냐하면 `set_logger`를 호출하기 전에 `LOGGER`를 초기화하기 때문입니다.
