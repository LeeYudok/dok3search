# 무경계 채널

`mpsc::channel()` 함수는 경계가 없는 비동기 채널을 생성합니다:

``` rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

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
