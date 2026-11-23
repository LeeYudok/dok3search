# Module tokio::sync::watch

## Overview

A multi-producer, multi-consumer channel that only retains the last sent
value.

This channel is useful for watching for changes to a value from multiple
points in the code base, for example, changes to configuration values.

**Usage**

`channel` returns a `Sender` / `Receiver` pair. These are the producer
and consumer halves of the channel. The channel is created with an initial
value.

Each `Receiver` independently tracks the last value seen by its caller.

To access the current value stored in the channel and mark it as seen
by a given `Receiver`, use `Receiver::borrow_and_update()`.

To access the current value without marking it as seen, use
`Receiver::borrow()`. (If the value has already been marked seen,
`Receiver::borrow()` is equivalent to `Receiver::borrow_and_update()`.)

For more information on when to use these methods, see
here.

**Change notifications**

The `Receiver` half provides an asynchronous `changed` method. This
method is ready when a new, unseen value is sent via the `Sender` half.

- `Receiver::changed()` returns:

- `Ok(())` on receiving a new value.

- `Err(``RecvError``)` if the
channel has been closed AND the current value is seen.

- If the current value is unseen when calling `changed`, then
`changed` will return immediately. If the current value is seen, then
it will sleep until either a new message is sent via the `Sender` half,
or the `Sender` is dropped.

- On completion, the `changed` method marks the new value as seen.

- At creation, the initial value is considered seen. In other words,
`Receiver::changed()` will not return until a subsequent value is sent.

- New `Receiver` instances can be created with `Sender::subscribe()`.
The current value at the time the `Receiver` is created is considered
seen.

**`changed` versus `has_changed`**

The `Receiver` half provides two methods for checking for changes
in the channel, `has_changed` and `changed`.

- 

`has_changed` is a synchronous method that checks whether the current
value is seen or not and returns a boolean. This method does not mark the
value as seen.

- 

`changed` is an asynchronous method that will return once an unseen
value is in the channel. This method does mark the value as seen.

Note there are two behavioral differences on when these two methods return
an error.

- `has_changed` errors if and only if the channel is closed.

- `changed` errors if the chan

...

## Items

- `error` — Watch error types.
- `Receiver` — Receives values from the associated Sender.
- `Ref` — Returns a reference to the inner value.
- `Sender` — Sends values to the associated Receiver.
- `channel` — Creates a new watch channel, returning the “send” and “receive” handles.

### fn tokio::sync::watch::channel

```rust
pub fn channel<T>(init: T) -> (Sender<T>, Receiver<T>)
```

Creates a new watch channel, returning the “send” and “receive” handles.

All values sent by `Sender` will become visible to the `Receiver` handles.
Only the last value sent is made available to the `Receiver` half. All
intermediate values are dropped.

**Examples**

The following example prints `hello! world! `.

```rust
use tokio::sync::watch;
use tokio::time::{Duration, sleep};

let (tx, mut rx) = watch::channel("hello");

tokio::spawn(async move {
    // Use the equivalent of a "do-while" loop so the initial value is
    // processed before awaiting the `changed()` future.
    loop {
        println!("{}! ", *rx.borrow_and_update());
        if rx.changed().await.is_err() {
            break;
        }
    }
});

sleep(Duration::from_millis(100)).await;
tx.send("world")?;
```

### struct tokio::sync::watch::Receiver

```rust
pub struct Receiver<T> { /* private fields */ }
```

Receives values from the associated `Sender`.

Instances are created by the `channel` function.

To turn this receiver into a `Stream`, you can use the `WatchStream`
wrapper.

**Methods**

- `pub fn borrow(&self) -> Ref<'_, T>`
- `pub fn borrow_and_update(&mut self) -> Ref<'_, T>`
- `pub fn has_changed(&self) -> Result<bool, RecvError>`
- `pub fn is_closed(&self) -> bool`
- `pub fn mark_changed(&mut self)`
- `pub fn mark_unchanged(&mut self)`
- `pub async fn changed(&mut self) -> Result<(), RecvError>`
- `pub async fn wait_for( &mut self, f: impl FnMut(&T) -> bool, ) -> Result<Ref<'_, T>, RecvError>`
- `pub fn same_channel(&self, other: &Self) -> bool`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`

### struct tokio::sync::watch::Ref

```rust
pub struct Ref<'a, T> { /* private fields */ }
```

Returns a reference to the inner value.

Outstanding borrows hold a read lock on the inner value. This means that
long-lived borrows could cause the producer half to block. It is recommended
to keep the borrow as short-lived as possible. Additionally, if you are
running in an environment that allows `!Send` futures, you must ensure that
the returned `Ref` type is never held alive across an `.await` point,
otherwise, it can lead to a deadlock.

The priority policy of the lock is dependent on the underlying lock
implementation, and this type does not guarantee that any particular policy
will be used. In particular, a producer which is waiting to acquire the lock
in `send` might or might not block concurrent calls to `borrow`, e.g.:

Potential deadlock example

```rust
// Task 1 (on thread A)    |  // Task 2 (on thread B)
let _ref1 = rx.borrow();   |
                           |  // will block
                           |  let _ = tx.send(());
// may deadlock            |
let _ref2 = rx.borrow();   |
```

**Methods**

- `pub fn has_changed(&self) -> bool`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deref(&self) -> &T`

### struct tokio::sync::watch::Sender

```rust
pub struct Sender<T> { /* private fields */ }
```

Sends values to the associated `Receiver`.

Instances are created by the `channel` function.

**Methods**

- `pub fn new(init: T) -> Self`
- `pub fn send(&self, value: T) -> Result<(), SendError<T>>`
- `pub fn send_modify<F>(&self, modify: F)where F: FnOnce(&mut T),`
- `pub fn send_if_modified<F>(&self, modify: F) -> boolwhere F: FnOnce(&mut T) -> bool,`
- `pub fn send_replace(&self, value: T) -> T`
- `pub fn borrow(&self) -> Ref<'_, T>`
- `pub fn is_closed(&self) -> bool`
- `pub async fn closed(&self)`
- `pub fn subscribe(&self) -> Receiver<T>`
- `pub fn receiver_count(&self) -> usize`
- `pub fn sender_count(&self) -> usize`
- `pub fn same_channel(&self, other: &Self) -> bool`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`
