# 취소

future가 누락되면 다시 폴링할 수 없다는 의미입니다. 이를 \_취소_라고 하며, `await` 지점에서 발생할 수 있습니다. future가 취소되더라도 시스템이 올바르게 작동할 수 있도록 주의를 기울여야 합니다. 예를 들어 교착 상태가 되거나 데이터가 손실되면 안 됩니다.

컴파일 오류 예제

``` rust
use std::io::{self, ErrorKind};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt, DuplexStream};

struct LinesReader {
    stream: DuplexStream,
}

impl LinesReader {
    fn new(stream: DuplexStream) -> Self {
        Self { stream }
    }

    async fn next(&mut self) -> io::Result<Option<String>> {
        let mut bytes = Vec::new();
        let mut buf = [0];
        while self.stream.read(&mut buf[..]).await? != 0 {
            bytes.push(buf[0]);
            if buf[0] == b'\n' {
                break;
            }
        }
        if bytes.is_empty() {
            return Ok(None);
        }
        let s = String::from_utf8(bytes)
            .map_err(|_| io::Error::new(ErrorKind::InvalidData, "not UTF-8"))?;
        Ok(Some(s))
    }
}

async fn slow_copy(source: String, mut dest: DuplexStream) -> std::io::Result<()> {
    for b in source.bytes() {
        dest.write_u8(b).await?;
        tokio::time::sleep(Duration::from_millis(10)).await
    }
    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let (client, server) = tokio::io::duplex(5);
    let handle = tokio::spawn(slow_copy("hi\nthere\n".to_owned(), client));

    let mut lines = LinesReader::new(server);
    let mut interval = tokio::time::interval(Duration::from_millis(60));
    loop {
        tokio::select! {
            _ = interval.tick() => println!("틱!"),
            line = lines.next() => if let Some(l) = line? {
                print!("{}", l)
            } else {
                break
            },
        }
    }
    handle.await.unwrap()?;
    Ok(())
}
```

> **발표자 노트**
>
> - 컴파일러는 취소 안전에 도움이 되지 않습니다. API 문서를 읽고 `async fn`의 상태를 고려해야 합니다.
>
> - `panic` 및 `?`와 달리 취소는 오류 처리가 아닌 일반적인 제어 흐름의 일부입니다.
>
> - 이 예에서는 문자열의 일부가 손실됩니다.
>
>   - `tick()` 브랜치가 먼저 완료될 때마다 `next()` 및 `buf`가 삭제됩니다.
>
>   - 다음과 같이 `buf`를 구조체의 일부로 만들어 `LinesReader`가 취소되지 않도록 할 수 있습니다.
>
>     컴파일 오류 예제
>
>     ``` rust
>     #![allow(unused)]
>     fn main() {
>     struct LinesReader {
>         stream: DuplexStream,
>         bytes: Vec<u8>,
>         buf: [u8; 1],
>     }
>
>     impl LinesReader {
>         fn new(stream: DuplexStream) -> Self {
>             Self { stream, bytes: Vec::new(), buf: [0] }
>         }
>         async fn next(&mut self) -> io::Result<Option<String>> {
>             // buf 및 bytes 접두사를 self로 지정합니다.
>             // ...
>             let raw = std::mem::take(&mut self.bytes);
>             let s = String::from_utf8(raw)
>             // ...
>         }
>     }
>     }
>     ```
>
> - [`Interval::tick`](https://docs.rs/tokio/latest/tokio/time/struct.Interval.html#method.tick)은 틱이 ‘delivered’ 됐는지 추적하므로 취소에 안전합니다.
>
> - [`AsyncReadExt::read`](https://docs.rs/tokio/latest/tokio/io/trait.AsyncReadExt.html#method.read)는 데이터를 반환하거나 읽지 않으므로 취소에 안전합니다.
>
> - [`AsyncBufReadExt::read_line`](https://docs.rs/tokio/latest/tokio/io/trait.AsyncBufReadExt.html#method.read_line)은 예와 유사하며 취소에 안전하지 *않습니다*. 자세한 내용과 대안은 관련 문서를 참고하세요.
