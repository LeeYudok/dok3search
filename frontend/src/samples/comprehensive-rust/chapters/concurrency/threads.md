# 스레드

러스트의 스레드는 다른 언어의 스레드와 유사하게 동작합니다:

``` rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("스레드의 개수: {i}!");
            thread::sleep(Duration::from_millis(5));
        }
    });

    for i in 1..5 {
        println!("기본 스레드: {i}");
        thread::sleep(Duration::from_millis(5));
    }
}
```

- 스레드는 모두 데몬 스레드입니다. 따라서 메인 스레드는 이 스레드들이 끝나기를 기다리지 않습니다.
- 한 스레드에서 발생한 패닉은 다른 스레드에게 영향을 끼치지 않습니다.
  - 패닉은 추가정보(페이로드)를 포함할 수 있으며, 이는 `downcast_ref`로 풀어볼 수 있습니다.

> **발표자 노트**
>
> - Rust thread APIs look not too different from e.g. C++ ones.
>
> - Run the example.
>
>   - 5ms timing is loose enough that main and spawned threads stay mostly in lockstep.
>   - Notice that the program ends before the spawned thread reaches 10!
>   - This is because main ends the program and spawned threads do not make it persist.
>     - Compare to pthreads/C++ std::thread/boost::thread if desired.
>
> - How do we wait around for the spawned thread to complete?
>
> - [`thread::spawn`](https://doc.rust-lang.org/std/thread/fn.spawn.html) returns a `JoinHandle`. Look at the docs.
>
>   - `JoinHandle` has a [`.join()`](https://doc.rust-lang.org/std/thread/struct.JoinHandle.html#method.join) method that blocks.
>
> - Use `let handle = thread::spawn(...)` and later `handle.join()` to wait for the thread to finish and have the program count all the way to 10.
>
> - Now what if we want to return a value?
>
> - Look at docs again:
>
>   - [`thread::spawn`](https://doc.rust-lang.org/std/thread/fn.spawn.html)’s closure returns `T`
>   - `JoinHandle` [`.join()`](https://doc.rust-lang.org/std/thread/struct.JoinHandle.html#method.join) returns `thread::Result<T>`
>
> - Use the `Result` return value from `handle.join()` to get access to the returned value.
>
> - Ok, what about the other case?
>
>   - Trigger a panic in the thread. Note that this doesn’t panic `main`.
>   - Access the panic payload. This is a good time to talk about [`Any`](https://doc.rust-lang.org/std/any/index.html).
>
> - Now we can return values from threads! What about taking inputs?
>
>   - Capture something by reference in the thread closure.
>   - An error message indicates we must move it.
>   - Move it in, see we can compute and then return a derived value.
>
> - If we want to borrow?
>
>   - Main kills child threads when it returns, but another function would just return and leave them running.
>   - That would be stack use-after-return, which violates memory safety!
>   - How do we avoid this? see next slide.
