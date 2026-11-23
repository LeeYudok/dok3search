# Module tokio::sync::mpsc

## Overview

A multi-producer, single-consumer queue for sending values between
asynchronous tasks.

This module provides two variants of the channel: bounded and unbounded. The
bounded variant has a limit on the number of messages that the channel can
store, and if this limit is reached, trying to send another message will
wait until a message is received from the channel. An unbounded channel has
an infinite capacity, so the `send` method will always complete immediately.
This makes the `UnboundedSender` usable from both synchronous and
asynchronous code.

Similar to the `mpsc` channels provided by `std`, the channel constructor
functions provide separate send and receive handles, `Sender` and
`Receiver` for the bounded channel, `UnboundedSender` and
`UnboundedReceiver` for the unbounded channel. If there is no message to read,
the current task will be notified when a new value is sent. `Sender` and
`UnboundedSender` allow sending values into the channel. If the bounded
channel is at capacity, the send is rejected and the task will be notified
when additional capacity is available. In other words, the channel provides
backpressure.

This channel is also suitable for the single-producer single-consumer
use-case. (Unless you only need to send one message, in which case you
should use the oneshot channel.)

**Disconnection**

When all `Sender` handles have been dropped, it is no longer
possible to send values into the channel. This is considered the termination
event of the stream. Once all senders have been dropped and any remaining
buffered values have been received, `Receiver::recv` returns `None`
(and `Receiver::poll_recv` returns `Poll::Ready(None)`).

If the `Receiver` handle is dropped, then messages can no longer
be read out of the channel. In this case, all further attempts to send will
result in an error. Additionally, all unread messages will be drained from the
channel and dropped.

**Clean Shutdown**

When the `Receiver` is dropped, it is possible for unprocessed messages to
remain in the channel. Instead, it is usually desirable to perform a “clean”
shutdown. To do this, the receiver first calls `close`, which will prevent
any further messages to be sent into the channel. Then, the receiver
consumes the channel to completion, at which point the receiver can be
dropped.

**Communicating between sync and async code**

When you want to communicate between synch

...

## Items

- `error` — Channel error types.
- `OwnedPermit` — Owned permit to send one value into the channel.
- `Permit` — Permits to send one value into the channel.
- `PermitIterator` — An Iterator of Permit that can be used to hold n slots in the channel.
- `Receiver` — Receives values from the associated Sender.
- `Sender` — Sends values to the associated Receiver.
- `UnboundedReceiver` — Receive values from the associated UnboundedSender.
- `UnboundedSender` — Send values to the associated UnboundedReceiver.
- `WeakSender` — A sender that does not prevent the channel from being closed.
- `WeakUnboundedSender` — An unbounded sender that does not prevent the channel from being closed.
- `channel` — Creates a bounded mpsc channel for communicating between asynchronous tasks with backpressure.
- `unbounded_channel` — Creates an unbounded mpsc channel for communicating between asynchronous tasks without backpressure.

### fn tokio::sync::mpsc::channel

```rust
pub fn channel<T>(buffer: usize) -> (Sender<T>, Receiver<T>)
```

Creates a bounded mpsc channel for communicating between asynchronous tasks
with backpressure.

The channel will buffer up to the provided number of messages.  Once the
buffer is full, attempts to send new messages will wait until a message is
received from the channel. The provided buffer capacity must be at least 1.

All data sent on `Sender` will become available on `Receiver` in the same
order as it was sent.

The `Sender` can be cloned to `send` to the same channel from multiple code
locations. Only one `Receiver` is supported.

If the `Receiver` is disconnected while trying to `send`, the `send` method
will return a `SendError`. Similarly, if `Sender` is disconnected while
trying to `recv`, the `recv` method will return `None`.

**Panics**

Panics if the buffer capacity is 0, or too large. Currently the maximum
capacity is `Semaphore::MAX_PERMITS`.

**Examples**

```rust
use tokio::sync::mpsc;

let (tx, mut rx) = mpsc::channel(100);

tokio::spawn(async move {
    for i in 0..10 {
        if let Err(_) = tx.send(i).await {
            println!("receiver dropped");
            return;
        }
    }
});

while let Some(i) = rx.recv().await {
     println!("got = {}", i);
}
```

### fn tokio::sync::mpsc::unbounded_channel

```rust
pub fn unbounded_channel<T>() -> (UnboundedSender<T>, UnboundedReceiver<T>)
```

Creates an unbounded mpsc channel for communicating between asynchronous
tasks without backpressure.

A `send` on this channel will always succeed as long as the receive half has
not been closed. If the receiver falls behind, messages will be arbitrarily
buffered.

Note that the amount of available system memory is an implicit bound to
the channel. Using an `unbounded` channel has the ability of causing the
process to run out of memory. In this case, the process will be aborted.

### struct tokio::sync::mpsc::OwnedPermit

```rust
pub struct OwnedPermit<T> { /* private fields */ }
```

Owned permit to send one value into the channel.

This is identical to the `Permit` type, except that it moves the sender
rather than borrowing it.

`OwnedPermit` values are returned by `Sender::reserve_owned()` and
`Sender::try_reserve_owned()` and are used to guarantee channel capacity
before generating a message to send.

**Methods**

- `pub fn send(self, value: T) -> Sender<T>`
- `pub fn release(self) -> Sender<T>`
- `pub fn same_channel(&self, other: &Self) -> bool`
- `pub fn same_channel_as_sender(&self, sender: &Sender<T>) -> bool`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`

### struct tokio::sync::mpsc::Permit

```rust
pub struct Permit<'a, T> { /* private fields */ }
```

Permits to send one value into the channel.

`Permit` values are returned by `Sender::reserve()` and `Sender::try_reserve()`
and are used to guarantee channel capacity before generating a message to send.

**Methods**

- `pub fn send(self, value: T)`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`

### struct tokio::sync::mpsc::PermitIterator

```rust
pub struct PermitIterator<'a, T> { /* private fields */ }
```

An `Iterator` of `Permit` that can be used to hold `n` slots in the channel.

`PermitIterator` values are returned by `Sender::reserve_many()` and `Sender::try_reserve_many()`
and are used to guarantee channel capacity before generating `n` messages to send.

**Methods**

- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`
- `1.0.0 · fn len(&self) -> usize`
- `fn is_empty(&self) -> bool`
- `fn next(&mut self) -> Option<Self::Item>`
- `fn size_hint(&self) -> (usize, Option<usize>)`
- `fn next_chunk<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,`
- `1.0.0 (const: unstable) · fn count(self) -> usizewhere Self: Sized,`
- `1.0.0 (const: unstable) · fn last(self) -> Option<Self::Item>where Self: Sized,`
- `fn advance_by(&mut self, n: usize) -> Result<(), NonZero<usize>>`
- `1.0.0 (const: unstable) · fn nth(&mut self, n: usize) -> Option<Self::Item>`
- `1.28.0 (const: unstable) · fn step_by(self, step: usize) -> StepBy<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn chain<U>(self, other: U) -> Chain<Self, <U as IntoIterator>::IntoIter> ⓘwhere Self: Sized, U: IntoIterator<Item = Self::Item>,`
- `1.0.0 (const: unstable) · fn zip<U>(self, other: U) -> Zip<Self, <U as IntoIterator>::IntoIter> ⓘwhere Self: Sized, U: IntoIterator,`
- `fn intersperse_with<G>(self, separator: G) -> IntersperseWith<Self, G> ⓘwhere Self: Sized, G: FnMut() -> Self::Item,`
- `1.0.0 (const: unstable) · fn map<B, F>(self, f: F) -> Map<Self, F> ⓘwhere Self: Sized, F: FnMut(Self::Item) -> B,`
- `1.21.0 (const: unstable) · fn for_each<F>(self, f: F)where Self: Sized, F: FnMut(Self::Item),`
- `1.0.0 (const: unstable) · fn filter<P>(self, predicate: P) -> Filter<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.0.0 (const: unstable) · fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F> ⓘwhere Self: Sized, F: FnMut(Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn enumerate(self) -> Enumerate<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn peekable(self) -> Peekable<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.0.0 (const: unstable) · fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.57.0 (const: unstable) · fn map_while<B, P>(self, predicate: P) -> MapWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn skip(self, n: usize) -> Skip<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn take(self, n: usize) -> Take<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn scan<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F> ⓘwhere Self: Sized, F: FnMut(&mut St, Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F> ⓘwhere Self: Sized, U: IntoIterator, F: FnMut(Self::Item) -> U,`
- `fn map_windows<F, R, const N: usize>(self, f: F) -> MapWindows<Self, F, N> ⓘwhere Self: Sized, F: FnMut(&[Self::Item; N]) -> R,`
- `1.0.0 (const: unstable) · fn fuse(self) -> Fuse<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn inspect<F>(self, f: F) -> Inspect<Self, F> ⓘwhere Self: Sized, F: FnMut(&Self::Item),`
- `1.0.0 (const: unstable) · fn by_ref(&mut self) -> &mut Selfwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn collect<B>(self) -> Bwhere B: FromIterator<Self::Item>, Self: Sized,`
- `fn collect_into<E>(self, collection: &mut E) -> &mut Ewhere E: Extend<Self::Item>, Self: Sized,`
- `1.0.0 (const: unstable) · fn partition<B, F>(self, f: F) -> (B, B)where Self: Sized, B: Default + Extend<Self::Item>, F: FnMut(&Self::Item) -> bool,`
- `fn is_partitioned<P>(self, predicate: P) -> boolwhere Self: Sized, P: FnMut(Self::Item) -> bool,`
- `1.27.0 (const: unstable) · fn try_fold<B, F, R>(&mut self, init: B, f: F) -> Rwhere Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Output = B>,`
- `1.27.0 (const: unstable) · fn try_for_each<F, R>(&mut self, f: F) -> Rwhere Self: Sized, F: FnMut(Self::Item) -> R, R: Try<Output = ()>,`
- `1.0.0 (const: unstable) · fn fold<B, F>(self, init: B, f: F) -> Bwhere Self: Sized, F: FnMut(B, Self::Item) -> B,`

### struct tokio::sync::mpsc::Receiver

```rust
pub struct Receiver<T> { /* private fields */ }
```

Receives values from the associated `Sender`.

Instances are created by the `channel` function.

This receiver can be turned into a `Stream` using `ReceiverStream`.

**Methods**

- `pub async fn recv(&mut self) -> Option<T>`
- `pub async fn recv_many(&mut self, buffer: &mut Vec<T>, limit: usize) -> usize`
- `pub fn try_recv(&mut self) -> Result<T, TryRecvError>`
- `pub fn blocking_recv(&mut self) -> Option<T>`
- `pub fn blocking_recv_many(&mut self, buffer: &mut Vec<T>, limit: usize) -> usize`
- `pub fn close(&mut self)`
- `pub fn is_closed(&self) -> bool`
- `pub fn is_empty(&self) -> bool`
- `pub fn len(&self) -> usize`
- `pub fn capacity(&self) -> usize`
- `pub fn max_capacity(&self) -> usize`
- `pub fn poll_recv(&mut self, cx: &mut Context<'_>) -> Poll<Option<T>>`
- `pub fn poll_recv_many( &mut self, cx: &mut Context<'_>, buffer: &mut Vec<T>, limit: usize, ) -> Poll<usize>`
- `pub fn sender_strong_count(&self) -> usize`
- `pub fn sender_weak_count(&self) -> usize`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`

### struct tokio::sync::mpsc::Sender

```rust
pub struct Sender<T> { /* private fields */ }
```

Sends values to the associated `Receiver`.

Instances are created by the `channel` function.

To convert the `Sender` into a `Sink` or use it in a poll function, you can
use the `PollSender` utility.

**Methods**

- `pub async fn send(&self, value: T) -> Result<(), SendError<T>>`
- `pub async fn closed(&self)`
- `pub fn try_send(&self, message: T) -> Result<(), TrySendError<T>>`
- `pub async fn send_timeout( &self, value: T, timeout: Duration, ) -> Result<(), SendTimeoutError<T>>`
- `pub fn blocking_send(&self, value: T) -> Result<(), SendError<T>>`
- `pub fn is_closed(&self) -> bool`
- `pub async fn reserve(&self) -> Result<Permit<'_, T>, SendError<()>>`
- `pub async fn reserve_many( &self, n: usize, ) -> Result<PermitIterator<'_, T>, SendError<()>>`
- `pub async fn reserve_owned(self) -> Result<OwnedPermit<T>, SendError<()>>`
- `pub fn try_reserve(&self) -> Result<Permit<'_, T>, TrySendError<()>>`
- `pub fn try_reserve_many( &self, n: usize, ) -> Result<PermitIterator<'_, T>, TrySendError<()>>`
- `pub fn try_reserve_owned(self) -> Result<OwnedPermit<T>, TrySendError<Self>>`
- `pub fn same_channel(&self, other: &Self) -> bool`
- `pub fn capacity(&self) -> usize`
- `pub fn downgrade(&self) -> WeakSender<T>`
- `pub fn max_capacity(&self) -> usize`
- `pub fn strong_count(&self) -> usize`
- `pub fn weak_count(&self) -> usize`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`

### struct tokio::sync::mpsc::UnboundedReceiver

```rust
pub struct UnboundedReceiver<T> { /* private fields */ }
```

Receive values from the associated `UnboundedSender`.

Instances are created by the `unbounded_channel` function.

This receiver can be turned into a `Stream` using `UnboundedReceiverStream`.

**Methods**

- `pub async fn recv(&mut self) -> Option<T>`
- `pub async fn recv_many(&mut self, buffer: &mut Vec<T>, limit: usize) -> usize`
- `pub fn try_recv(&mut self) -> Result<T, TryRecvError>`
- `pub fn blocking_recv(&mut self) -> Option<T>`
- `pub fn blocking_recv_many(&mut self, buffer: &mut Vec<T>, limit: usize) -> usize`
- `pub fn close(&mut self)`
- `pub fn is_closed(&self) -> bool`
- `pub fn is_empty(&self) -> bool`
- `pub fn len(&self) -> usize`
- `pub fn poll_recv(&mut self, cx: &mut Context<'_>) -> Poll<Option<T>>`
- `pub fn poll_recv_many( &mut self, cx: &mut Context<'_>, buffer: &mut Vec<T>, limit: usize, ) -> Poll<usize>`
- `pub fn sender_strong_count(&self) -> usize`
- `pub fn sender_weak_count(&self) -> usize`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`

### struct tokio::sync::mpsc::UnboundedSender

```rust
pub struct UnboundedSender<T> { /* private fields */ }
```

Send values to the associated `UnboundedReceiver`.

Instances are created by the `unbounded_channel` function.

**Methods**

- `pub fn send(&self, message: T) -> Result<(), SendError<T>>`
- `pub async fn closed(&self)`
- `pub fn is_closed(&self) -> bool`
- `pub fn same_channel(&self, other: &Self) -> bool`
- `pub fn downgrade(&self) -> WeakUnboundedSender<T>`
- `pub fn strong_count(&self) -> usize`
- `pub fn weak_count(&self) -> usize`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`

### struct tokio::sync::mpsc::WeakSender

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
use tokio::sync::mpsc::channel;

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
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`

### struct tokio::sync::mpsc::WeakUnboundedSender

```rust
pub struct WeakUnboundedSender<T> { /* private fields */ }
```

An unbounded sender that does not prevent the channel from being closed.

If all `UnboundedSender` instances of a channel were dropped and only
`WeakUnboundedSender` instances remain, the channel is closed.

In order to send messages, the `WeakUnboundedSender` needs to be upgraded using
`WeakUnboundedSender::upgrade`, which returns `Option<UnboundedSender>`. It returns `None`
if all `UnboundedSender`s have been dropped, and otherwise it returns an `UnboundedSender`.

**Examples**

```rust
use tokio::sync::mpsc::unbounded_channel;

let (tx, _rx) = unbounded_channel::<i32>();
let tx_weak = tx.downgrade();

// Upgrading will succeed because `tx` still exists.
assert!(tx_weak.upgrade().is_some());

// If we drop `tx`, then it will fail.
drop(tx);
assert!(tx_weak.clone().upgrade().is_none());
```

**Methods**

- `pub fn upgrade(&self) -> Option<UnboundedSender<T>>`
- `pub fn strong_count(&self) -> usize`
- `pub fn weak_count(&self) -> usize`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`
