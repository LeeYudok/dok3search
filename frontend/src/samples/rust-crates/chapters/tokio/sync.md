# Module tokio::sync

## Overview

Synchronization primitives for use in asynchronous contexts.

Tokio programs tend to be organized as a set of tasks where each task
operates independently and may be executed on separate physical threads. The
synchronization primitives provided in this module permit these independent
tasks to communicate together.

**Message passing**

The most common form of synchronization in a Tokio program is message
passing. Two tasks operate independently and send messages to each other to
synchronize. Doing so has the advantage of avoiding shared state.

Message passing is implemented using channels. A channel supports sending a
message from one producer task to one or more consumer tasks. There are a
few flavors of channels provided by Tokio. Each channel flavor supports
different message passing patterns. When a channel supports multiple
producers, many separate tasks may send messages. When a channel
supports multiple consumers, many different separate tasks may receive
messages.

Tokio provides many different channel flavors as different message passing
patterns are best handled with different implementations.

**`oneshot` channel**

The `oneshot` channel supports sending a single value from a
single producer to a single consumer. This channel is usually used to send
the result of a computation to a waiter.

Example: using a `oneshot` channel to receive the result of a
computation.

```rust
use tokio::sync::oneshot;

async fn some_computation() -> String {
    "represents the result of the computation".to_string()
}

let (tx, rx) = oneshot::channel();

tokio::spawn(async move {
    let res = some_computation().await;
    tx.send(res).unwrap();
});

// Do other work while the computation is happening in the background

// Wait for the computation result
let res = rx.await.unwrap();
```

Note, if the task produces a computation result as its final
action before terminating, the `JoinHandle` can be used to
receive that value instead of allocating resources for the
`oneshot` channel. Awaiting on `JoinHandle` returns `Result`. If
the task panics, the `Joinhandle` yields `Err` with the panic
cause.

Example:

...

## Items

- `broadcast` — A multi-producer, multi-consumer broadcast queue. Each sent value is seen by all consumers.
- `futures` — Named future types.
- `mpsc` — A multi-producer, single-consumer queue for sending values between asynchronous tasks.
- `oneshot` — A one-shot channel is used for sending a single message between asynchronous tasks. The channel function is used to create a Sender and Receiver handle pair that form the channel.
- `watch` — A multi-producer, multi-consumer channel that only retains the last sent value.
- `AcquireError` — Error returned from the Semaphore::acquire function.
- `Barrier` — A barrier enables multiple tasks to synchronize the beginning of some computation.
- `BarrierWaitResult` — A BarrierWaitResult is returned by wait when all tasks in the Barrier have rendezvoused.
- `MappedMutexGuard` — A handle to a held Mutex that has had a function applied to it via MutexGuard::map.
- `Mutex` — An asynchronous Mutex-like type.
- `MutexGuard` — A handle to a held Mutex. The guard can be held across any .await point as it is Send.
- `Notify` — Notifies a single task to wake up.
- `OnceCell` — A thread-safe cell that can be written to only once.
- `OwnedMappedMutexGuard` — A owned handle to a held Mutex that has had a function applied to it via OwnedMutexGuard::map.
- `OwnedMutexGuard` — An owned handle to a held Mutex.
- `OwnedRwLockMappedWriteGuard` — Owned RAII structure used to release the exclusive write access of a lock when dropped.
- `OwnedRwLockReadGuard` — Owned RAII structure used to release the shared read access of a lock when dropped.
- `OwnedRwLockWriteGuard` — Owned RAII structure used to release the exclusive write access of a lock when dropped.
- `OwnedSemaphorePermit` — An owned permit from the semaphore.
- `RwLock` — An asynchronous reader-writer lock.
- `RwLockMappedWriteGuard` — RAII structure used to release the exclusive write access of a lock when dropped.
- `RwLockReadGuard` — RAII structure used to release the shared read access of a lock when dropped.
- `RwLockWriteGuard` — RAII structure used to release the exclusive write access of a lock when dropped.
- `Semaphore` — Counting semaphore performing asynchronous permit acquisition.
- `SemaphorePermit` — A permit from the semaphore.
- `SetOnce` — A thread-safe cell that can be written to only once.
- `SetOnceError` — Error that can be returned from SetOnce::set.
- `TryLockError` — Error returned from the Mutex::try_lock, RwLock::try_read and RwLock::try_write functions.
- `SetError` — Errors that can be returned from OnceCell::set.
- `TryAcquireError` — Error returned from the Semaphore::try_acquire function.

### enum tokio::sync::SetError

```rust
pub enum SetError<T> {
    AlreadyInitializedError(T),
    InitializingError(T),
}
```

Errors that can be returned from `OnceCell::set`.

**Methods**

- `pub fn is_already_init_err(&self) -> bool`
- `pub fn is_initializing_err(&self) -> bool`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### enum tokio::sync::TryAcquireError

```rust
pub enum TryAcquireError {
    Closed,
    NoPermits,
}
```

Error returned from the `Semaphore::try_acquire` function.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### struct tokio::sync::AcquireError

```rust
pub struct AcquireError(/* private fields */);
```

Error returned from the `Semaphore::acquire` function.

An `acquire` operation can only fail if the semaphore has been
closed.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`

### struct tokio::sync::Barrier

```rust
pub struct Barrier { /* private fields */ }
```

A barrier enables multiple tasks to synchronize the beginning of some computation.

```rust
use tokio::sync::Barrier;
use std::sync::Arc;

let mut handles = Vec::with_capacity(10);
let barrier = Arc::new(Barrier::new(10));
for _ in 0..10 {
    let c = barrier.clone();
    // The same messages will be printed together.
    // You will NOT see any interleaving.
    handles.push(tokio::spawn(async move {
        println!("before wait");
        let wait_result = c.wait().await;
        println!("after wait");
        wait_result
    }));
}

// Will not resolve until all "after wait" messages have been printed
let mut num_leaders = 0;
for handle in handles {
    let wait_result = handle.await.unwrap();
    if wait_result.is_leader() {
        num_leaders += 1;
    }
}

// Exactly one barrier will resolve as the "leader"
assert_eq!(num_leaders, 1);
```

**Methods**

- `pub fn new(n: usize) -> Barrier`
- `pub async fn wait(&self) -> BarrierWaitResult`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tokio::sync::BarrierWaitResult

```rust
pub struct BarrierWaitResult(/* private fields */);
```

A `BarrierWaitResult` is returned by `wait` when all tasks in the `Barrier` have rendezvoused.

**Methods**

- `pub fn is_leader(&self) -> bool`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tokio::sync::MappedMutexGuard

```rust
pub struct MappedMutexGuard<'a, T: ?Sized> { /* private fields */ }
```

A handle to a held `Mutex` that has had a function applied to it via `MutexGuard::map`.

This can be used to hold a subfield of the protected data.

**Methods**

- `pub fn map<U, F>(this: Self, f: F) -> MappedMutexGuard<'a, U>where F: FnOnce(&mut T) -> &mut U,`
- `pub fn try_map<U, F>(this: Self, f: F) -> Result<MappedMutexGuard<'a, U>, Self>where F: FnOnce(&mut T) -> Option<&mut U>,`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deref(&self) -> &Self::Target`
- `fn deref_mut(&mut self) -> &mut Self::Target`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`

### struct tokio::sync::Mutex

```rust
pub struct Mutex<T: ?Sized> { /* private fields */ }
```

An asynchronous `Mutex`-like type.

This type acts similarly to `std::sync::Mutex`, with two major
differences: `lock` is an async method so does not block, and the lock
guard is designed to be held across `.await` points.

Tokio’s Mutex operates on a guaranteed FIFO basis.
This means that the order in which tasks call the `lock` method is
the exact order in which they will acquire the lock.

**Which kind of mutex should you use?**

Contrary to popular belief, it is ok and often preferred to use the ordinary
`Mutex` from the standard library in asynchronous code.

The feature that the async mutex offers over the blocking mutex is the
ability to keep it locked across an `.await` point. This makes the async
mutex more expensive than the blocking mutex, so the blocking mutex should
be preferred in the cases where it can be used. The primary use case for the
async mutex is to provide shared mutable access to IO resources such as a
database connection. If the value behind the mutex is just data, it’s
usually appropriate to use a blocking mutex such as the one in the standard
library or `parking_lot`.

Note that, although the compiler will not prevent the std `Mutex` from holding
its guard across `.await` points in situations where the task is not movable
between threads, this virtually never leads to correct concurrent code in
practice as it can easily lead to deadlocks.

A common pattern is to wrap the `Arc<Mutex<...>>` in a struct that provides
non-async methods for performing operations on the data within, and only
lock the mutex inside these methods. The mini-redis example p

...

**Methods**

- `pub fn new(t: T) -> Selfwhere T: Sized,`
- `pub const fn const_new(t: T) -> Selfwhere T: Sized,`
- `pub async fn lock(&self) -> MutexGuard<'_, T>`
- `pub fn blocking_lock(&self) -> MutexGuard<'_, T>`
- `pub fn blocking_lock_owned(self: Arc<Self>) -> OwnedMutexGuard<T>`
- `pub async fn lock_owned(self: Arc<Self>) -> OwnedMutexGuard<T>`
- `pub fn try_lock(&self) -> Result<MutexGuard<'_, T>, TryLockError>`
- `pub fn get_mut(&mut self) -> &mut T`
- `pub fn try_lock_owned( self: Arc<Self>, ) -> Result<OwnedMutexGuard<T>, TryLockError>`
- `pub fn into_inner(self) -> Twhere T: Sized,`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn from(s: T) -> Self`

### struct tokio::sync::MutexGuard

```rust
pub struct MutexGuard<'a, T: ?Sized> { /* private fields */ }
```

A handle to a held `Mutex`. The guard can be held across any `.await` point
as it is `Send`.

As long as you have this guard, you have exclusive access to the underlying
`T`. The guard internally borrows the `Mutex`, so the mutex will not be
dropped while a guard exists.

The lock is automatically released whenever the guard is dropped, at which
point `lock` will succeed yet again.

**Methods**

- `pub fn map<U, F>(this: Self, f: F) -> MappedMutexGuard<'a, U>where U: ?Sized, F: FnOnce(&mut T) -> &mut U,`
- `pub fn try_map<U, F>(this: Self, f: F) -> Result<MappedMutexGuard<'a, U>, Self>where U: ?Sized, F: FnOnce(&mut T) -> Option<&mut U>,`
- `pub fn mutex(this: &Self) -> &'a Mutex<T>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deref(&self) -> &Self::Target`
- `fn deref_mut(&mut self) -> &mut Self::Target`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`

### struct tokio::sync::Notify

```rust
pub struct Notify { /* private fields */ }
```

Notifies a single task to wake up.

`Notify` provides a basic mechanism to notify a single task of an event.
`Notify` itself does not carry any data. Instead, it is to be used to signal
another task to perform an operation.

A `Notify` can be thought of as a `Semaphore` starting with 0 permits. The
`notified().await` method waits for a permit to become available, and
`notify_one()` sets a permit if there currently are no available
permits.

The synchronization details of `Notify` are similar to
`thread::park` and `Thread::unpark` from std. A `Notify`
value contains a single permit. `notified().await` waits for the permit to
be made available, consumes the permit, and resumes.  `notify_one()` sets
the permit, waking a pending task if there is one.

If `notify_one()` is called before `notified().await`, then the next
call to `notified().await` will complete immediately, consuming the permit.
Any subsequent calls to `notified().await` will wait for a new permit.

If `notify_one()` is called multiple times before `notified().await`,
only a single permit is stored. The next call to `notified().await` will
complete immediately, but the one after will wait for a new permit.

**Examples**

Basic usage.

```rust
use tokio::sync::Notify;
use std::sync::Arc;

let notify = Arc::new(Notify::new());
let notify2 = notify.clone();

let handle = tokio::spawn(async move {
    notify2.notified().await;
    println!("received notification");
});

println!("sending notification");
notify.notify_one();

// Wait for task to receive notification.
handle.await.unwrap();
```

Unbound multi-producer

...

**Methods**

- `pub fn new() -> Notify`
- `pub const fn const_new() -> Notify`
- `pub fn notified(&self) -> Notified<'_>`
- `pub fn notified_owned(self: Arc<Self>) -> OwnedNotified`
- `pub fn notify_one(&self)`
- `pub fn notify_last(&self)`
- `pub fn notify_waiters(&self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Notify`

### struct tokio::sync::OnceCell

```rust
pub struct OnceCell<T> { /* private fields */ }
```

A thread-safe cell that can be written to only once.

A `OnceCell` is typically used for global variables that need to be
initialized once on first use, but need no further changes. The `OnceCell`
in Tokio allows the initialization procedure to be asynchronous.

**Examples**

```rust
use tokio::sync::OnceCell;

async fn some_computation() -> u32 {
    1 + 1
}

static ONCE: OnceCell<u32> = OnceCell::const_new();

let result = ONCE.get_or_init(some_computation).await;
assert_eq!(*result, 2);
```

It is often useful to write a wrapper method for accessing the value.

```rust
use tokio::sync::OnceCell;

static ONCE: OnceCell<u32> = OnceCell::const_new();

async fn get_global_integer() -> &'static u32 {
    ONCE.get_or_init(|| async {
        1 + 1
    }).await
}

let result = get_global_integer().await;
assert_eq!(*result, 2);
```

**Methods**

- `pub fn new() -> Self`
- `pub const fn const_new() -> Self`
- `pub fn new_with(value: Option<T>) -> Self`
- `pub const fn const_new_with(value: T) -> Self`
- `pub fn initialized(&self) -> bool`
- `pub fn get(&self) -> Option<&T>`
- `pub fn get_mut(&mut self) -> Option<&mut T>`
- `pub fn set(&self, value: T) -> Result<(), SetError<T>>`
- `pub async fn get_or_init<F, Fut>(&self, f: F) -> &Twhere F: FnOnce() -> Fut, Fut: Future<Output = T>,`
- `pub async fn get_or_try_init<E, F, Fut>(&self, f: F) -> Result<&T, E>where F: FnOnce() -> Fut, Fut: Future<Output = Result<T, E>>,`
- `pub fn into_inner(self) -> Option<T>`
- `pub fn take(&mut self) -> Option<T>`
- `fn clone(&self) -> OnceCell<T>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `fn default() -> OnceCell<T>`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`
- `fn from(value: T) -> Self`
- `fn eq(&self, other: &OnceCell<T>) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### struct tokio::sync::OwnedMappedMutexGuard

```rust
pub struct OwnedMappedMutexGuard<T: ?Sized, U: ?Sized = T> { /* private fields */ }
```

A owned handle to a held `Mutex` that has had a function applied to it via
`OwnedMutexGuard::map`.

This can be used to hold a subfield of the protected data.

**Methods**

- `pub fn map<S, F>(this: Self, f: F) -> OwnedMappedMutexGuard<T, S>where F: FnOnce(&mut U) -> &mut S,`
- `pub fn try_map<S, F>( this: Self, f: F, ) -> Result<OwnedMappedMutexGuard<T, S>, Self>where F: FnOnce(&mut U) -> Option<&mut S>,`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deref(&self) -> &Self::Target`
- `fn deref_mut(&mut self) -> &mut Self::Target`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`

### struct tokio::sync::OwnedMutexGuard

```rust
pub struct OwnedMutexGuard<T: ?Sized> { /* private fields */ }
```

An owned handle to a held `Mutex`.

This guard is only available from a `Mutex` that is wrapped in an `Arc`. It
is identical to `MutexGuard`, except that rather than borrowing the `Mutex`,
it clones the `Arc`, incrementing the reference count. This means that
unlike `MutexGuard`, it will have the `'static` lifetime.

As long as you have this guard, you have exclusive access to the underlying
`T`. The guard internally keeps a reference-counted pointer to the original
`Mutex`, so even if the lock goes away, the guard remains valid.

The lock is automatically released whenever the guard is dropped, at which
point `lock` will succeed yet again.

**Methods**

- `pub fn map<U, F>(this: Self, f: F) -> OwnedMappedMutexGuard<T, U>where U: ?Sized, F: FnOnce(&mut T) -> &mut U,`
- `pub fn try_map<U, F>( this: Self, f: F, ) -> Result<OwnedMappedMutexGuard<T, U>, Self>where U: ?Sized, F: FnOnce(&mut T) -> Option<&mut U>,`
- `pub fn mutex(this: &Self) -> &Arc<Mutex<T>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deref(&self) -> &Self::Target`
- `fn deref_mut(&mut self) -> &mut Self::Target`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`

### struct tokio::sync::OwnedRwLockMappedWriteGuard

```rust
pub struct OwnedRwLockMappedWriteGuard<T: ?Sized, U: ?Sized = T> { /* private fields */ }
```

Owned RAII structure used to release the exclusive write access of a lock when
dropped.

This structure is created by mapping an `OwnedRwLockWriteGuard`. It is a
separate type from `OwnedRwLockWriteGuard` to disallow downgrading a mapped
guard, since doing so can cause undefined behavior.

**Methods**

- `pub fn map<F, V: ?Sized>(this: Self, f: F) -> OwnedRwLockMappedWriteGuard<T, V>where F: FnOnce(&mut U) -> &mut V,`
- `pub fn try_map<F, V: ?Sized>( this: Self, f: F, ) -> Result<OwnedRwLockMappedWriteGuard<T, V>, Self>where F: FnOnce(&mut U) -> Option<&mut V>,`
- `pub fn rwlock(this: &Self) -> &Arc<RwLock<T>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deref(&self) -> &U`
- `fn deref_mut(&mut self) -> &mut U`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`

### struct tokio::sync::OwnedRwLockReadGuard

```rust
pub struct OwnedRwLockReadGuard<T: ?Sized, U: ?Sized = T> { /* private fields */ }
```

Owned RAII structure used to release the shared read access of a lock when
dropped.

This structure is created by the `read_owned` method on
`RwLock`.

**Methods**

- `pub fn map<F, V: ?Sized>(this: Self, f: F) -> OwnedRwLockReadGuard<T, V>where F: FnOnce(&U) -> &V,`
- `pub fn try_map<F, V: ?Sized>( this: Self, f: F, ) -> Result<OwnedRwLockReadGuard<T, V>, Self>where F: FnOnce(&U) -> Option<&V>,`
- `pub fn rwlock(this: &Self) -> &Arc<RwLock<T>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deref(&self) -> &U`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`

### struct tokio::sync::OwnedRwLockWriteGuard

```rust
pub struct OwnedRwLockWriteGuard<T: ?Sized> { /* private fields */ }
```

Owned RAII structure used to release the exclusive write access of a lock when
dropped.

This structure is created by the `write_owned` method
on `RwLock`.

**Methods**

- `pub fn map<F, U: ?Sized>(this: Self, f: F) -> OwnedRwLockMappedWriteGuard<T, U>where F: FnOnce(&mut T) -> &mut U,`
- `pub fn downgrade_map<F, U: ?Sized>( this: Self, f: F, ) -> OwnedRwLockReadGuard<T, U>where F: FnOnce(&T) -> &U,`
- `pub fn try_map<F, U: ?Sized>( this: Self, f: F, ) -> Result<OwnedRwLockMappedWriteGuard<T, U>, Self>where F: FnOnce(&mut T) -> Option<&mut U>,`
- `pub fn try_downgrade_map<F, U: ?Sized>( this: Self, f: F, ) -> Result<OwnedRwLockReadGuard<T, U>, Self>where F: FnOnce(&T) -> Option<&U>,`
- `pub fn into_mapped(this: Self) -> OwnedRwLockMappedWriteGuard<T>`
- `pub fn downgrade(self) -> OwnedRwLockReadGuard<T>`
- `pub fn rwlock(this: &Self) -> &Arc<RwLock<T>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deref(&self) -> &T`
- `fn deref_mut(&mut self) -> &mut T`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`

### struct tokio::sync::OwnedSemaphorePermit

```rust
pub struct OwnedSemaphorePermit { /* private fields */ }
```

An owned permit from the semaphore.

This type is created by the `acquire_owned` method.

**Methods**

- `pub fn forget(self)`
- `pub fn merge(&mut self, other: Self)`
- `pub fn split(&mut self, n: usize) -> Option<Self>`
- `pub fn semaphore(&self) -> &Arc<Semaphore>`
- `pub fn num_permits(&self) -> usize`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`

### struct tokio::sync::RwLock

```rust
pub struct RwLock<T: ?Sized> { /* private fields */ }
```

An asynchronous reader-writer lock.

This type of lock allows a number of readers or at most one writer at any
point in time. The write portion of this lock typically allows modification
of the underlying data (exclusive access) and the read portion of this lock
typically allows for read-only access (shared access).

In comparison, a `Mutex` does not distinguish between readers or writers
that acquire the lock, therefore causing any tasks waiting for the lock to
become available to yield. An `RwLock` will allow any number of readers to
acquire the lock as long as a writer is not holding the lock.

The priority policy of Tokio’s read-write lock is fair (or
write-preferring), in order to ensure that readers cannot starve
writers. Fairness is ensured using a first-in, first-out queue for the tasks
awaiting the lock; a read lock will not be given out until all write lock
requests that were queued before it have been acquired and released. This is
in contrast to the Rust standard library’s `std::sync::RwLock`, where the
priority policy is dependent on the operating system’s implementation.

The type parameter `T` represents the data that this lock protects. It is
required that `T` satisfies `Send` to be shared across threads. The RAII guards
returned from the locking methods implement `Deref`
(and `DerefMut`
for the `write` methods) to allow access to the content of the lock.

**Examples**

...

**Methods**

- `pub fn new(value: T) -> RwLock<T>where T: Sized,`
- `pub fn with_max_readers(value: T, max_reads: u32) -> RwLock<T>where T: Sized,`
- `pub const fn const_new(value: T) -> RwLock<T>where T: Sized,`
- `pub const fn const_with_max_readers(value: T, max_reads: u32) -> RwLock<T>where T: Sized,`
- `pub async fn read(&self) -> RwLockReadGuard<'_, T>`
- `pub fn blocking_read(&self) -> RwLockReadGuard<'_, T>`
- `pub async fn read_owned(self: Arc<Self>) -> OwnedRwLockReadGuard<T>`
- `pub fn try_read(&self) -> Result<RwLockReadGuard<'_, T>, TryLockError>`
- `pub fn try_read_owned( self: Arc<Self>, ) -> Result<OwnedRwLockReadGuard<T>, TryLockError>`
- `pub async fn write(&self) -> RwLockWriteGuard<'_, T>`
- `pub fn blocking_write(&self) -> RwLockWriteGuard<'_, T>`
- `pub async fn write_owned(self: Arc<Self>) -> OwnedRwLockWriteGuard<T>`
- `pub fn try_write(&self) -> Result<RwLockWriteGuard<'_, T>, TryLockError>`
- `pub fn try_write_owned( self: Arc<Self>, ) -> Result<OwnedRwLockWriteGuard<T>, TryLockError>`
- `pub fn get_mut(&mut self) -> &mut T`
- `pub fn into_inner(self) -> Twhere T: Sized,`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn from(s: T) -> Self`

### struct tokio::sync::RwLockMappedWriteGuard

```rust
pub struct RwLockMappedWriteGuard<'a, T: ?Sized> { /* private fields */ }
```

RAII structure used to release the exclusive write access of a lock when
dropped.

This structure is created by mapping an `RwLockWriteGuard`. It is a
separate type from `RwLockWriteGuard` to disallow downgrading a mapped
guard, since doing so can cause undefined behavior.

**Methods**

- `pub fn map<F, U: ?Sized>(this: Self, f: F) -> RwLockMappedWriteGuard<'a, U>where F: FnOnce(&mut T) -> &mut U,`
- `pub fn try_map<F, U: ?Sized>( this: Self, f: F, ) -> Result<RwLockMappedWriteGuard<'a, U>, Self>where F: FnOnce(&mut T) -> Option<&mut U>,`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deref(&self) -> &T`
- `fn deref_mut(&mut self) -> &mut T`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`

### struct tokio::sync::RwLockReadGuard

```rust
pub struct RwLockReadGuard<'a, T: ?Sized> { /* private fields */ }
```

RAII structure used to release the shared read access of a lock when
dropped.

This structure is created by the `read` method on
`RwLock`.

**Methods**

- `pub fn map<F, U: ?Sized>(this: Self, f: F) -> RwLockReadGuard<'a, U>where F: FnOnce(&T) -> &U,`
- `pub fn try_map<F, U: ?Sized>( this: Self, f: F, ) -> Result<RwLockReadGuard<'a, U>, Self>where F: FnOnce(&T) -> Option<&U>,`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deref(&self) -> &T`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`

### struct tokio::sync::RwLockWriteGuard

```rust
pub struct RwLockWriteGuard<'a, T: ?Sized> { /* private fields */ }
```

RAII structure used to release the exclusive write access of a lock when
dropped.

This structure is created by the `write` method
on `RwLock`.

**Methods**

- `pub fn map<F, U: ?Sized>(this: Self, f: F) -> RwLockMappedWriteGuard<'a, U>where F: FnOnce(&mut T) -> &mut U,`
- `pub fn downgrade_map<F, U: ?Sized>(this: Self, f: F) -> RwLockReadGuard<'a, U>where F: FnOnce(&T) -> &U,`
- `pub fn try_map<F, U: ?Sized>( this: Self, f: F, ) -> Result<RwLockMappedWriteGuard<'a, U>, Self>where F: FnOnce(&mut T) -> Option<&mut U>,`
- `pub fn try_downgrade_map<F, U: ?Sized>( this: Self, f: F, ) -> Result<RwLockReadGuard<'a, U>, Self>where F: FnOnce(&T) -> Option<&U>,`
- `pub fn into_mapped(this: Self) -> RwLockMappedWriteGuard<'a, T>`
- `pub fn downgrade(self) -> RwLockReadGuard<'a, T>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deref(&self) -> &T`
- `fn deref_mut(&mut self) -> &mut T`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`

### struct tokio::sync::Semaphore

```rust
pub struct Semaphore { /* private fields */ }
```

Counting semaphore performing asynchronous permit acquisition.

A semaphore maintains a set of permits. Permits are used to synchronize
access to a shared resource. A semaphore differs from a mutex in that it
can allow more than one concurrent caller to access the shared resource at a
time.

When `acquire` is called and the semaphore has remaining permits, the
function immediately returns a permit. However, if no remaining permits are
available, `acquire` (asynchronously) waits until an outstanding permit is
dropped. At this point, the freed permit is assigned to the caller.

This `Semaphore` is fair, which means that permits are given out in the order
they were requested. This fairness is also applied when `acquire_many` gets
involved, so if a call to `acquire_many` at the front of the queue requests
more permits than currently available, this can prevent a call to `acquire`
from completing, even if the semaphore has enough permits to complete the call
to `acquire`.

To use the `Semaphore` in a poll function, you can use the `PollSemaphore`
utility.

**Memory ordering**

If a task writes some data and then releases a permit, any task that later
acquires a permit is guaranteed to see that data. This makes it safe to use
a semaphore to hand data off between tasks through shared state.

Stated more precisely in terms of atomic memory orderings: acquiring a
permit (via `acquire`, `acquire_many`, `try_acquire`,
`try_acquire_many`, or their `_owned` variants), releasing permits (by
dropping a `SemaphorePermit` or `OwnedSemaphorePermit`, or by calling
`add_permits` or `forget_per

...

**Methods**

- `pub fn new(permits: usize) -> Self`
- `pub const fn const_new(permits: usize) -> Self`
- `pub fn available_permits(&self) -> usize`
- `pub fn add_permits(&self, n: usize)`
- `pub fn forget_permits(&self, n: usize) -> usize`
- `pub async fn acquire(&self) -> Result<SemaphorePermit<'_>, AcquireError>`
- `pub async fn acquire_many( &self, n: u32, ) -> Result<SemaphorePermit<'_>, AcquireError>`
- `pub fn try_acquire(&self) -> Result<SemaphorePermit<'_>, TryAcquireError>`
- `pub fn try_acquire_many( &self, n: u32, ) -> Result<SemaphorePermit<'_>, TryAcquireError>`
- `pub fn blocking_acquire(&self) -> Result<SemaphorePermit<'_>, AcquireError>`
- `pub fn blocking_acquire_many( &self, n: u32, ) -> Result<SemaphorePermit<'_>, AcquireError>`
- `pub async fn acquire_owned( self: Arc<Self>, ) -> Result<OwnedSemaphorePermit, AcquireError>`
- `pub async fn acquire_many_owned( self: Arc<Self>, n: u32, ) -> Result<OwnedSemaphorePermit, AcquireError>`
- `pub fn try_acquire_owned( self: Arc<Self>, ) -> Result<OwnedSemaphorePermit, TryAcquireError>`
- `pub fn try_acquire_many_owned( self: Arc<Self>, n: u32, ) -> Result<OwnedSemaphorePermit, TryAcquireError>`
- `pub fn blocking_acquire_owned( self: Arc<Self>, ) -> Result<OwnedSemaphorePermit, AcquireError>`
- `pub fn blocking_acquire_many_owned( self: Arc<Self>, n: u32, ) -> Result<OwnedSemaphorePermit, AcquireError>`
- `pub fn close(&self)`
- `pub fn is_closed(&self) -> bool`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tokio::sync::SemaphorePermit

```rust
pub struct SemaphorePermit<'a> { /* private fields */ }
```

A permit from the semaphore.

This type is created by the `acquire` method.

**Methods**

- `pub fn forget(self)`
- `pub fn merge(&mut self, other: Self)`
- `pub fn split(&mut self, n: usize) -> Option<Self>`
- `pub fn semaphore(&self) -> &Semaphore`
- `pub fn num_permits(&self) -> usize`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`

### struct tokio::sync::SetOnce

```rust
pub struct SetOnce<T> { /* private fields */ }
```

A thread-safe cell that can be written to only once.

A `SetOnce` is inspired from python’s `asyncio.Event` type. It can be
used to wait until the value of the `SetOnce` is set like a “Event” mechanism.

**Example**

```rust
use tokio::sync::{SetOnce, SetOnceError};

static ONCE: SetOnce<u32> = SetOnce::const_new();

// set the value inside a task somewhere...
tokio::spawn(async move { ONCE.set(20) });

// checking with .get doesn't block main thread
println!("{:?}", ONCE.get());

// wait until the value is set, blocks the thread
println!("{:?}", ONCE.wait().await);

Ok(())
```

A `SetOnce` is typically used for global variables that need to be
initialized once on first use, but need no further changes. The `SetOnce`
in Tokio allows the initialization procedure to be asynchronous.

**Example**

```rust
use tokio::sync::{SetOnce, SetOnceError};
use std::sync::Arc;

let once = SetOnce::new();

let arc = Arc::new(once);
let first_cl = Arc::clone(&arc);
let second_cl = Arc::clone(&arc);

// set the value inside a task
tokio::spawn(async move { first_cl.set(20) }).await.unwrap()?;

// wait inside task to not block the main thread
tokio::spawn(async move {
    // wait inside async context for the value to be set
    assert_eq!(*second_cl.wait().await, 20);
}).await.unwrap();

// subsequent set calls will fail
assert!(arc.set(30).is_err());

println!("{:?}", arc.get());

Ok(())
```

**Methods**

- `pub fn new() -> Self`
- `pub const fn const_new() -> Self`
- `pub fn new_with(value: Option<T>) -> Self`
- `pub const fn const_new_with(value: T) -> Self`
- `pub fn initialized(&self) -> bool`
- `pub fn get(&self) -> Option<&T>`
- `pub fn set(&self, value: T) -> Result<(), SetOnceError<T>>`
- `pub fn into_inner(self) -> Option<T>`
- `pub async fn wait(&self) -> &T`
- `fn clone(&self) -> SetOnce<T>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `fn default() -> SetOnce<T>`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`
- `fn from(value: T) -> Self`
- `fn eq(&self, other: &SetOnce<T>) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### struct tokio::sync::SetOnceError

```rust
pub struct SetOnceError<T>(pub T);
```

Error that can be returned from `SetOnce::set`.

This error means that the `SetOnce` was already initialized when
set was called

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### struct tokio::sync::TryLockError

```rust
pub struct TryLockError(/* private fields */);
```

Error returned from the `Mutex::try_lock`, `RwLock::try_read` and
`RwLock::try_write` functions.

`Mutex::try_lock` operation will only fail if the mutex is already locked.

`RwLock::try_read` operation will only fail if the lock is currently held
by an exclusive writer.

`RwLock::try_write` operation will only fail if the lock is currently held
by any reader or by an exclusive writer.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
