# 태스크

Rust의 태스크(작업) 시스템은 경량 스레딩의 한 종류로 볼 수 있습니다.

하나의 작업에는, 실행자가 이 작업을 진행하기 위해 계속 폴링하는, 최상위 future가 한 개 있습니다. 이 future에는 `poll` 메서드가 폴링하는 중첩된 future가 한 개 이상 있을 수 있습니다. 이러한 중첩된 future는 일반적인 함수 호출 스택하고 비슷한 역할을 합니다. 한 작업 안에서 여러 자식 future들을 폴링하면, 타이머를 켜는 것과 어떤 I/O작업을 동시에 수행시킨 후 타이머와 I/O 중 먼저 끝나는 것을 기다리는 것과 같은동시성도 구현할 수 있습니다.

컴파일 오류 예제

``` rust
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    println!("포트 {}에서 수신 대기", listener.local_addr()?.port());

    loop {
        let (mut socket, addr) = listener.accept().await?;

        println!("{addr:?}에서 연결");

        tokio::spawn(async move {
            socket.write_all(b"누구세요?\n").await.expect("소켓 오류");

            let mut buf = vec![0; 1024];
            let name_size = socket.read(&mut buf).await.expect("소켓 오류");
            let name = std::str::from_utf8(&buf[..name_size]).unwrap().trim();
            let reply = format!("{name}님, 전화해 주셔서 감사합니다.\n");
            socket.write_all(reply.as_bytes()).await.expect("소켓 오류");
        });
    }
}
```

> **발표자 노트**
>
> 이 예제를, 로컬 컴퓨터에 만들어 둔 `src/main.rs`에 복사하고 거기에서 실행하세요.
>
> [nc](https://www.unix.com/man-page/linux/1/nc/) 또는 [telnet](https://www.unix.com/man-page/linux/1/telnet/)과 같은 TCP 연결 도구를 사용하여 연결해 보세요.
>
> - 수강생들에게 이 서버에 몇 개의 클라이언트가 연결되면 이 서버의 상태가 어떻게 변할지 그림을 그려보도록 하세요. 어떤 태스크들이 있는지, 이 태스크들의 Future는 어떤 상태에 있는지 물어봅니다.
>
> - This is the first time we’ve seen an `async` block. This is similar to a closure, but does not take any arguments. Its return value is a Future, similar to an `async fn`.
>
> - Async 블록을 함수로 리팩터링하고 `?`를 사용하여 오류 처리를 개선해 봅시다.
