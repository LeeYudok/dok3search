# Module tokio::sync::broadcast

## Overview

A multi-producer, multi-consumer broadcast queue. Each sent value is seen by
all consumers.

A `Sender` is used to broadcast values to all connected `Receiver`
values. `Sender` handles are clone-able, allowing concurrent send and
receive actions. `Sender` and `Receiver` are both `Send` and `Sync` as
long as `T` is `Send`.

When a value is sent, all `Receiver` handles are notified and will
receive the value. The value is stored once inside the channel and cloned on
demand for each receiver. Once all receivers have received a clone of the
value, the value is released from the channel.

A channel is created by calling `channel`, specifying the maximum number
of messages the channel can retain at any given time.

New `Receiver` handles are created by calling `Sender::subscribe`. The
returned `Receiver` will receive values sent after the call to
`subscribe`.

This channel is also suitable for the single-producer multi-consumer
use-case, where a single sender broadcasts values to many receivers.

**Lagging**

As sent messages must be retained until all `Receiver` handles receive
a clone, broadcast channels are susceptible to the “slow receiver” problem.
In this case, all but one receiver are able to receive values at the rate
they are sent. Because one receiver is stalled, the channel starts to fill
up.

This broadcast channel implementation handles this case by setting a hard
upper bound on the number of values the channel may retain at any given
time. This upper bound is passed to the `channel` function as an argument.
The provided capacity is rounded up to the next power of two; that
rounded size is the number of messages the ring buffer can hold, and is what
lag detection is based on. For example, `channel(3)` allocates a buffer of
length 4, so a receiver only lags once it falls more than 4 messages behind
the sender.

If a value is sent when the channel is at capacity, the oldest value
currently held by the channel is overwritten. This frees up space for the
new value. Any receiver that has not yet seen the overwritten value will
return `RecvError::Lagged` the next time `recv` (or
`try_recv`) is called. The error carries the number of
messages that were dropped before the receiver’s cursor and are therefore
no longer available.

Returning `RecvError::Lagged` does not close or disconnect the
receiver. The lagging receiver’s internal cursor is advanced to the o

...

## Items

- `error` — Broadcast error types
- `Receiver` — Receiving-half of the broadcast channel.
- `Sender` — Sending-half of the broadcast channel.
- `WeakSender` — A sender that does not prevent the channel from being closed.
- `channel` — Create a bounded, multi-producer, multi-consumer channel where each sent value is broadcasted to all active receivers.

### fn tokio::sync::broadcast::channel

```rust
pub fn channel<T: Clone>(capacity: usize) -> (Sender<T>, Receiver<T>)
```

Create a bounded, multi-producer, multi-consumer channel where each sent
value is broadcasted to all active receivers.

Note: The provided `capacity` is rounded up to the next power of
two. That rounded size is the number of messages the internal ring buffer
can retain, and is what lag detection uses. For example,
`channel(3)` behaves as if the capacity were 4.

All data sent on `Sender` will become available on every active
`Receiver` in the same order as it was sent.

The `Sender` can be cloned to `send` to the same channel from multiple
points in the process or it can be used concurrently from an `Arc`. New
`Receiver` handles are created by calling `Sender::subscribe`.

If all `Receiver` handles are dropped, the `send` method will return a
`SendError`. Similarly, if all `Sender` handles are dropped, the `recv`
method will return a `RecvError`.

**Examples**

```rust
use tokio::sync::broadcast;

let (tx, mut rx1) = broadcast::channel(16);
let mut rx2 = tx.subscribe();

tokio::spawn(async move {
    assert_eq!(rx1.recv().await.unwrap(), 10);
    assert_eq!(rx1.recv().await.unwrap(), 20);
});

tokio::spawn(async move {
    assert_eq!(rx2.recv().await.unwrap(), 10);
    assert_eq!(rx2.recv().await.unwrap(), 20);
});

tx.send(10).unwrap();
tx.send(20).unwrap();
```

**Panics**

This will panic if `capacity` is equal to `0` or exceeds `usize::MAX / 2`.

This pre-allocates space for `capacity` messages. Allocation failure may result in a panic or
an allocation error.

### struct tokio::sync::broadcast::Receiver

```rust
pub struct Receiver<T> { /* private fields */ }
```

Receiving-half of the `broadcast` channel.

Must not be used concurrently. Messages may be retrieved using
`recv`.

To turn this receiver into a `Stream`, you can use the `BroadcastStream`
wrapper.

**Examples**

```rust
use tokio::sync::broadcast;

let (tx, mut rx1) = broadcast::channel(16);
let mut rx2 = tx.subscribe();

tokio::spawn(async move {
    assert_eq!(rx1.recv().await.unwrap(), 10);
    assert_eq!(rx1.recv().await.unwrap(), 20);
});

tokio::spawn(async move {
    assert_eq!(rx2.recv().await.unwrap(), 10);
    assert_eq!(rx2.recv().await.unwrap(), 20);
});

tx.send(10).unwrap();
tx.send(20).unwrap();
```

**Methods**

- `pub fn len(&self) -> usize`
- `pub fn is_empty(&self) -> bool`
- `pub fn same_channel(&self, other: &Self) -> bool`
- `pub fn sender_strong_count(&self) -> usize`
- `pub fn sender_weak_count(&self) -> usize`
- `pub fn is_closed(&self) -> bool`
- `pub fn resubscribe(&self) -> Self`
- `pub async fn recv(&mut self) -> Result<T, RecvError>`
- `pub fn try_recv(&mut self) -> Result<T, TryRecvError>`
- `pub fn blocking_recv(&mut self) -> Result<T, RecvError>`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`

### struct tokio::sync::broadcast::Sender

```rust
pub struct Sender<T> { /* private fields */ }
```

Sending-half of the `broadcast` channel.

May be used from many threads. Messages can be sent with
`send`.

**Examples**

```rust
use tokio::sync::broadcast;

let (tx, mut rx1) = broadcast::channel(16);
let mut rx2 = tx.subscribe();

tokio::spawn(async move {
    assert_eq!(rx1.recv().await.unwrap(), 10);
    assert_eq!(rx1.recv().await.unwrap(), 20);
});

tokio::spawn(async move {
    assert_eq!(rx2.recv().await.unwrap(), 10);
    assert_eq!(rx2.recv().await.unwrap(), 20);
});

tx.send(10).unwrap();
tx.send(20).unwrap();
```

**Methods**

- `pub fn new(capacity: usize) -> Self`
- `pub fn send(&self, value: T) -> Result<usize, SendError<T>>`
- `pub fn subscribe(&self) -> Receiver<T>`
- `pub fn downgrade(&self) -> WeakSender<T>`
- `pub fn len(&self) -> usize`
- `pub fn is_empty(&self) -> bool`
- `pub fn receiver_count(&self) -> usize`
- `pub fn same_channel(&self, other: &Self) -> bool`
- `pub async fn closed(&self)`
- `pub fn strong_count(&self) -> usize`
- `pub fn weak_count(&self) -> usize`
- `fn clone(&self) -> Sender<T>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`

### struct tokio::sync::broadcast::WeakSender

```rust
pub struct WeakSender<T> { /* private fields */ }
```

A sender that does not prevent the channel from being closed.

If all `Sender` instances of a channel were dropped and only `WeakSender`
instances remain, the channel is closed.

In order to send messages, the `WeakSender` needs to be upgraded using
`WeakSender::upgrade`, which returns `Option<Sender>`. It returns `None`
if all `Sender`s have been dropped, and otherwise it returns a `Sender`.

**Examples**

```rust
use tokio::sync::broadcast::channel;

let (tx, _rx) = channel::<i32>(15);
let tx_weak = tx.downgrade();

// Upgrading will succeed because `tx` still exists.
assert!(tx_weak.upgrade().is_some());

// If we drop `tx`, then it will fail.
drop(tx);
assert!(tx_weak.clone().upgrade().is_none());
```

**Methods**

- `pub fn upgrade(&self) -> Option<Sender<T>>`
- `pub fn strong_count(&self) -> usize`
- `pub fn weak_count(&self) -> usize`
- `fn clone(&self) -> WeakSender<T>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`
