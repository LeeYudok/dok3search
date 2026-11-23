# Module tokio::sync::oneshot

## Overview

A one-shot channel is used for sending a single message between
asynchronous tasks. The `channel` function is used to create a
`Sender` and `Receiver` handle pair that form the channel.

The `Sender` handle is used by the producer to send the value.
The `Receiver` handle is used by the consumer to receive the value.

Each handle can be used on separate tasks.

Since the `send` method is not async, it can be used anywhere. This includes
sending between two runtimes, and using it from non-async code.

If the `Receiver` is closed before receiving a message which has already
been sent, the message will remain in the channel until the receiver is
dropped, at which point the message will be dropped immediately.

**Examples**

```rust
use tokio::sync::oneshot;

let (tx, rx) = oneshot::channel();

tokio::spawn(async move {
    if let Err(_) = tx.send(3) {
        println!("the receiver dropped");
    }
});

match rx.await {
    Ok(v) => println!("got = {:?}", v),
    Err(_) => println!("the sender dropped"),
}
```

If the sender is dropped without sending, the receiver will fail with
`error::RecvError`:

```rust
use tokio::sync::oneshot;

let (tx, rx) = oneshot::channel::<u32>();

tokio::spawn(async move {
    drop(tx);
});

match rx.await {
    Ok(_) => panic!("This doesn't happen"),
    Err(_) => println!("the sender dropped"),
}
```

To use a `oneshot` channel in a `tokio::select!` loop, add `&mut` in front of
the channel.

```rust
use tokio::sync::oneshot;
use tokio::time::{interval, sleep, Duration};

let (send, mut recv) = oneshot::channel();
let mut interval = interval(Duration::from_millis(100));

tokio::spawn(async move {
    sleep(Duration::from_secs(1)).await;
    send.send("shut down").unwrap();
});

loop {
    tokio::select! {
        _ = interval.tick() => println!("Another 100ms"),
        msg = &mut recv => {
            println!("Got message: {}", msg.unwrap());
            break;
        }
    }
}
```

To use a `Sender` from a destructor, put it in an `Option` and call
`Option::take`.

...

## Items

- `error` — Oneshot error types.
- `Receiver` — Receives a value from the associated Sender.
- `Sender` — Sends a value to the associated Receiver.
- `channel` — Creates a new one-shot channel for sending single values across asynchronous tasks.

### fn tokio::sync::oneshot::channel

```rust
pub fn channel<T>() -> (Sender<T>, Receiver<T>)
```

Creates a new one-shot channel for sending single values across asynchronous
tasks.

The function returns separate “send” and “receive” handles. The `Sender`
handle is used by the producer to send the value. The `Receiver` handle is
used by the consumer to receive the value.

Each handle can be used on separate tasks.

**Examples**

```rust
use tokio::sync::oneshot;

let (tx, rx) = oneshot::channel();

tokio::spawn(async move {
    if let Err(_) = tx.send(3) {
        println!("the receiver dropped");
    }
});

match rx.await {
    Ok(v) => println!("got = {:?}", v),
    Err(_) => println!("the sender dropped"),
}
```

### struct tokio::sync::oneshot::Receiver

```rust
pub struct Receiver<T> { /* private fields */ }
```

Receives a value from the associated `Sender`.

A pair of both a `Sender` and a `Receiver`  are created by the
`channel` function.

This channel has no `recv` method because the receiver itself implements the
`Future` trait. To receive a `Result<T, ``error::RecvError``>`, `.await` the `Receiver` object directly.

The `poll` method on the `Future` trait is allowed to spuriously return
`Poll::Pending` even if the message has been sent. If such a spurious
failure happens, then the caller will be woken when the spurious failure has
been resolved so that the caller can attempt to receive the message again.
Note that receiving such a wakeup does not guarantee that the next call will
succeed — it could fail with another spurious failure. (A spurious failure
does not mean that the message is lost. It is just delayed.)

**Cancel safety**

Awaiting a `&mut Receiver<T>` is cancel safe. If it is used as a branch in
`tokio::select!` and another branch completes first, it is
guaranteed that no message was received on this
channel.

**Examples**

```rust
use tokio::sync::oneshot;

let (tx, rx) = oneshot::channel();

tokio::spawn(async move {
    if let Err(_) = tx.send(3) {
        println!("the receiver dropped");
    }
});

match rx.await {
    Ok(v) => println!("got = {:?}", v),
    Err(_) => println!("the sender dropped"),
}
```

If the sender is dropped without sending, the receiver will fail with
`error::RecvError`:

...

**Methods**

- `pub fn close(&mut self)`
- `pub fn is_terminated(&self) -> bool`
- `pub fn is_empty(&self) -> bool`
- `pub fn try_recv(&mut self) -> Result<T, TryRecvError>`
- `pub fn blocking_recv(self) -> Result<T, RecvError>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`
- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`

### struct tokio::sync::oneshot::Sender

```rust
pub struct Sender<T> { /* private fields */ }
```

Sends a value to the associated `Receiver`.

A pair of both a `Sender` and a `Receiver`  are created by the
`channel` function.

**Examples**

```rust
use tokio::sync::oneshot;

let (tx, rx) = oneshot::channel();

tokio::spawn(async move {
    if let Err(_) = tx.send(3) {
        println!("the receiver dropped");
    }
});

match rx.await {
    Ok(v) => println!("got = {:?}", v),
    Err(_) => println!("the sender dropped"),
}
```

If the sender is dropped without sending, the receiver will fail with
`error::RecvError`:

```rust
use tokio::sync::oneshot;

let (tx, rx) = oneshot::channel::<u32>();

tokio::spawn(async move {
    drop(tx);
});

match rx.await {
    Ok(_) => panic!("This doesn't happen"),
    Err(_) => println!("the sender dropped"),
}
```

To use a `Sender` from a destructor, put it in an `Option` and call
`Option::take`.

```rust
use tokio::sync::oneshot;

struct SendOnDrop {
    sender: Option<oneshot::Sender<&'static str>>,
}
impl Drop for SendOnDrop {
    fn drop(&mut self) {
        if let Some(sender) = self.sender.take() {
            // Using `let _ =` to ignore send errors.
            let _ = sender.send("I got dropped!");
        }
    }
}

let (send, recv) = oneshot::channel();

let send_on_drop = SendOnDrop { sender: Some(send) };
drop(send_on_drop);

assert_eq!(recv.await, Ok("I got dropped!"));
```

**Methods**

- `pub fn send(self, t: T) -> Result<(), T>`
- `pub async fn closed(&mut self)`
- `pub fn is_closed(&self) -> bool`
- `pub fn poll_closed(&mut self, cx: &mut Context<'_>) -> Poll<()>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`
