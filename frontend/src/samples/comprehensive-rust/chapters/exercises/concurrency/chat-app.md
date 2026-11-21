# 채팅 애플리케이션

이 연습에서는 새로운 지식을 사용하여 브로드캐스트 채팅 애플리케이션을 구현해 보겠습니다. 클라이언트가 연결하고 메시지를 게시하는 채팅 서버가 있습니다. 클라이언트는 표준 입력에서 사용자 메시지를 읽고 서버로 전송합니다. 채팅 서버는 수신하는 각 메시지를 모든 클라이언트에 브로드캐스트합니다.

For this, we use [a broadcast channel](https://docs.rs/tokio/latest/tokio/sync/broadcast/fn.channel.html) on the server, and [`tokio_websockets`](https://docs.rs/tokio-websockets/) for the communication between the client and the server.

새 Cargo 프로젝트를 만들고 다음 종속 항목을 추가합니다.

*Cargo.toml*:

``` toml
[package]
name = "chat-async"
version = "0.1.0"
edition = "2021"

[dependencies]
futures-util = { version = "0.3.30", features = ["sink"] }
http = "1.1.0"
tokio = { version = "1.36.0", features = ["full"] }
tokio-websockets = { version = "0.7.0", features = ["client", "fastrand", "server", "sha1_smol"] }
```

## 필수 API

You are going to need the following functions from `tokio` and [`tokio_websockets`](https://docs.rs/tokio-websockets/). Spend a few minutes to familiarize yourself with the API.

- [StreamExt::next()](https://docs.rs/futures-util/0.3.28/futures_util/stream/trait.StreamExt.html#method.next) implemented by `WebSocketStream`: for asynchronously reading messages from a Websocket Stream.
- [SinkExt::send()](https://docs.rs/futures-util/0.3.28/futures_util/sink/trait.SinkExt.html#method.send) implemented by `WebSocketStream`: for asynchronously sending messages on a Websocket Stream.
- [Lines::next_line()](https://docs.rs/tokio/latest/tokio/io/struct.Lines.html#method.next_line)은 표준 입력에서 사용자 메시지를 비동기식으로 읽는 데 사용됩니다.
- [Sender::subscribe()](https://docs.rs/tokio/latest/tokio/sync/broadcast/struct.Sender.html#method.subscribe)는 브로드캐스트 채널 구독에 사용됩니다.

## Two binaries

Normally in a Cargo project, you can have only one binary, and one `src/main.rs` file. In this project, we need two binaries. One for the client, and one for the server. You could potentially make them two separate Cargo projects, but we are going to put them in a single Cargo project with two binaries. For this to work, the client and the server code should go under `src/bin` (see the [documentation](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#binaries)).

Copy the following server and client code into `src/bin/server.rs` and `src/bin/client.rs`, respectively. Your task is to complete these files as described below.

*src/bin/server.rs*:

컴파일 오류 예제

``` rust
use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {

    // TODO: 힌트는 아래의 작업 설명을 참고하세요.

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);

    let listener = TcpListener::bind("127.0.0.1:2000").await?;
    println!("포트 2000에서 수신 대기");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("{addr:?}의 새 연결");
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            // 원시 TCP 스트림을 websocket에 래핑합니다.
            let ws_stream = ServerBuilder::new().accept(socket).await?;

            handle_connection(addr, ws_stream, bcast_tx).await
        });
    }
}
```

*src/bin/client.rs*:

컴파일 오류 예제

``` rust
use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:2000"))
            .connect()
            .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();


    // TODO: 힌트는 아래의 작업 설명을 참고하세요.

}
```

## Running the binaries

Run the server with:

``` shell
cargo run --bin server
```

and the client with:

``` shell
cargo run --bin client
```

## 태스크

- `src/bin/server.rs`에서 `handle_connection` 함수를 구현합니다.
  - 힌트: 연속 루프에서 두 작업을 동시에 실행하는 경우 `tokio::select!`를 사용하세요. 한 작업은 클라이언트에서 메시지를 수신하여 브로드캐스트합니다. 다른 하나는 서버가 수신한 메시지를 클라이언트로 보냅니다.
- `src/bin/client.rs`에서 main 함수를 완료합니다.
  - 힌트: 이전과 마찬가지로 연속 루프에서 두 작업을 동시에 실행하는 경우 `tokio::select!`를 사용하세요. (1) 표준 입력에서 사용자 메시지를 읽고 서버로 보냅니다. (2) 서버에서 메시지를 수신하고 사용자에게 표시합니다.
- 선택사항: 작업을 완료하면 메시지 발신자를 제외한 모든 클라이언트에게 메시지를 브로드캐스트하도록 코드를 변경합니다.
