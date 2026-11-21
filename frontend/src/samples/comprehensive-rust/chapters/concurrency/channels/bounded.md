# 경계 채널

With bounded (synchronous) channels, `send` can block the current thread:

``` rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::sync_channel(3);

    thread::spawn(move || {
        let thread_id = thread::current().id();
        for i in 1..10 {
            tx.send(format!("메시지 {i}")).unwrap();
            println!("{thread_id:?}: 보낸 메시지 {i}");
        }
        println!("{thread_id:?}: 완료");
    });
    thread::sleep(Duration::from_millis(100));

    for msg in rx.iter() {
        println!("기본: {msg} 받음");
    }
}
```

> **발표자 노트**
>
> - `send`를 호출하면 채널에 새 메시지를 위한 공간이 확보될 때까지 현재 스레드가 차단됩니다. 채널에서 읽는 사람이 없는 경우 스레드가 무기한 차단될 수 있습니다.
> - `send` 호출은 오류와 함께 중단됩니다. 따라서 채널이 닫히면 `Result`를 반환합니다. 수신자를 삭제하면 채널이 닫힙니다.
> - A bounded channel with a size of zero is called a “rendezvous channel”. Every send will block the current thread until another thread calls `recv`.
