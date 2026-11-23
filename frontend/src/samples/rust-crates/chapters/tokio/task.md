# Module tokio::task

## Overview

Asynchronous green-threads.

**What are Tasks?**

A task is a light weight, non-blocking unit of execution. A task is similar
to an OS thread, but rather than being managed by the OS scheduler, they are
managed by the Tokio runtime. Another name for this general pattern is
green threads. If you are familiar with Go’s goroutines, Kotlin’s
coroutines, or Erlang’s processes, you can think of Tokio’s tasks as
something similar.

Key points about tasks include:

- 

Tasks are light weight. Because tasks are scheduled by the Tokio
runtime rather than the operating system, creating new tasks or switching
between tasks does not require a context switch and has fairly low
overhead. Creating, running, and destroying large numbers of tasks is
quite cheap, especially compared to OS threads.

- 

Tasks are scheduled cooperatively. Most operating systems implement
preemptive multitasking. This is a scheduling technique where the
operating system allows each thread to run for a period of time, and then
preempts it, temporarily pausing that thread and switching to another.
Tasks, on the other hand, implement cooperative multitasking. In
cooperative multitasking, a task is allowed to run until it yields,
indicating to the Tokio runtime’s scheduler that it cannot currently
continue executing. When a task yields, the Tokio runtime switches to
executing the next task.

- 

Tasks are non-blocking. Typically, when an OS thread performs I/O or
must synchronize with another thread, it blocks, allowing the OS to
schedule another thread. When a task cannot continue executing, it must
yield instead, allowing the Tokio runtime to schedule another task. Tasks
should generally not perform system calls or other operations that could
block a thread, as this would prevent other tasks running on the same
thread from executing as well. Instead, this module provides APIs for
running blocking operations in an asynchronous context.

**Working with Tasks**

This module provides the following APIs for working with tasks:

**Spawning**

Perhaps the most important function in this module is `task::spawn`. This
function can be thought of as an async equivalent to the standard library’s
`thread::spawn`. It takes an `async` block or other
future, and creates a new task to run that work concurrently:

```rust
use tokio::task;

task::spawn(async {
    // perform some work here...
});
```

Like `std::thre

...

## Items

- `coop` — Utilities for improved cooperative scheduling.
- `futures` — Task-related futures.
- `AbortHandle` — An owned permission to abort a spawned task, without awaiting its completion.
- `Id` — An opaque ID that uniquely identifies a task relative to all other currently running tasks.
- `JoinError` — Task failed to execute to completion.
- `JoinHandle` — An owned permission to join on a task (await its termination).
- `JoinSet` — A collection of tasks spawned on a Tokio runtime.
- `LocalEnterGuard` — Context guard for LocalSet
- `LocalKey` — A key for task-local data.
- `LocalSet` — A set of tasks which are executed on the same thread.
- `block_in_place` — Runs the provided blocking function on the current thread without blocking the executor.
- `id` — Returns the Id of the currently running task.
- `spawn` — Spawns a new asynchronous task, returning a JoinHandle for it.
- `spawn_blocking` — Runs the provided closure on a thread where blocking is acceptable.
- `spawn_local` — Spawns a !Send future on the current LocalSet or LocalRuntime.
- `try_id` — Returns the Id of the currently running task, or None if called outside of a task.
- `yield_now` — Yields execution back to the Tokio runtime.

### fn tokio::task::block_in_place

```rust
pub fn block_in_place<F, R>(f: F) -> Rwhere
    F: FnOnce() -> R,
```

Runs the provided blocking function on the current thread without
blocking the executor.

In general, issuing a blocking call or performing a lot of compute in a
future without yielding is problematic, as it may prevent the executor
from driving other tasks forward. Calling this function informs the
executor that the currently executing task is about to block the thread,
so the executor is able to hand off any other tasks it has to a new
worker thread before that happens. See the CPU-bound tasks and blocking
code section for more information.

Be aware that although this function avoids starving other independently
spawned tasks, any other code running concurrently in the same task will
be suspended during the call to `block_in_place`. This can happen e.g.
when using the `join!` macro. To avoid this issue, use
`spawn_blocking` instead of `block_in_place`.

Note that this function cannot be used within a `current_thread` runtime
because in this case there are no other worker threads to hand off tasks
to. On the other hand, calling the function outside a runtime is
allowed. In this case, `block_in_place` just calls the provided closure
normally.

Code running behind `block_in_place` cannot be cancelled. When you shut
down the executor, it will wait indefinitely for all blocking operations
to finish. You can use `shutdown_timeout` to stop waiting for them
after a certain timeout. Be aware that this will still not cancel the
tasks — they are simply allowed to keep running after the method
returns.

**Examples**

...

### fn tokio::task::id

```rust
pub fn id() -> Id
```

Returns the `Id` of the currently running task.

**Panics**

This function panics if called from outside a task. Please note that calls
to `block_on` do not have task IDs, so the method will panic if called from
within a call to `block_on`. For a version of this function that doesn’t
panic, see `task::try_id()`.

### fn tokio::task::spawn

```rust
pub fn spawn<F>(future: F) -> JoinHandle<F::Output> ⓘwhere
    F: Future + Send + 'static,
    F::Output: Send + 'static,
```

Spawns a new asynchronous task, returning a
`JoinHandle` for it.

The provided future will start running in the background immediately
when `spawn` is called, even if you don’t await the returned
`JoinHandle`.

Spawning a task enables the task to execute concurrently to other tasks. The
spawned task may execute on the current thread, or it may be sent to a
different thread to be executed. The specifics depend on the current
`Runtime` configuration. In a
running runtime, the task will start immediately in the
background. On a blocked runtime, the user must drive the runtime forward (for
example, by calling `Runtime::block_on`).

It is guaranteed that spawn will not synchronously poll the task being spawned.
This means that calling spawn while holding a lock does not pose a risk of
deadlocking with the spawned task.

There is no guarantee that a spawned task will execute to completion.
When a runtime is shutdown, all outstanding tasks are dropped,
regardless of the lifecycle of that task.

This function must be called from the context of a Tokio runtime. Tasks running on
the Tokio runtime are always inside its context, but you can also enter the context
using the `Runtime::enter` method.

**Examples**

In this example, a server is started and `spawn` is used to start a new task
that processes each received connection.

...

### fn tokio::task::spawn_blocking

```rust
pub fn spawn_blocking<F, R>(f: F) -> JoinHandle<R> ⓘwhere
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
```

Runs the provided closure on a thread where blocking is acceptable.

In general, issuing a blocking call or performing a lot of compute in a
future without yielding is problematic, as it may prevent the executor from
driving other futures forward. This function runs the provided closure on a
thread dedicated to blocking operations. See the CPU-bound tasks and
blocking code section for more information.

Tokio will spawn more blocking threads when they are requested through this
function until the upper limit configured on the `Builder` is reached.
After reaching the upper limit, the tasks are put in a queue.
The thread limit is very large by default, because `spawn_blocking` is often
used for various kinds of IO operations that cannot be performed
asynchronously.  When you run CPU-bound code using `spawn_blocking`, you
should keep this large upper limit in mind. When running many CPU-bound
computations, a semaphore or some other synchronization primitive should be
used to limit the number of computations executed in parallel. Specialized
CPU-bound executors, such as rayon, may also be a good fit.

This function is intended for non-async operations that eventually finish on
their own. If you want to spawn an ordinary thread, you should use
`thread::spawn` instead.

Be aware that tasks spawned using `spawn_blocking` cannot be aborted
because they are not async. If you call `abort` on a `spawn_blocking`
task, then this will not have any effect, and the task will continue
running normally. The exception is if the task has not started running
yet; in that case, calling `abort` m

...

### fn tokio::task::spawn_local

```rust
pub fn spawn_local<F>(future: F) -> JoinHandle<F::Output> ⓘwhere
    F: Future + 'static,
    F::Output: 'static,
```

Spawns a `!Send` future on the current `LocalSet` or `LocalRuntime`.

This is possible when either using one of these types explicitly, or by
opting to use the `"local"` runtime flavor in `tokio::main`:

ⓘ
```rust
#[tokio::main(flavor = "local")]
```

The spawned future will run on the same thread that called `spawn_local`.

The provided future will start running in the background immediately
when `spawn_local` is called, even if you don’t await the returned
`JoinHandle`.

**Panics**

This function panics if called outside of a `LocalSet` or `LocalRuntime`.

Note that if `tokio::spawn` is used from within a `LocalSet`, the
resulting new task will not be inside the `LocalSet`, so you must use
`spawn_local` if you want to stay within the `LocalSet`.

**Examples**

With `LocalSet`:

```rust
use std::rc::Rc;
use tokio::task;

let nonsend_data = Rc::new("my nonsend data...");

let local = task::LocalSet::new();

// Run the local task set.
local.run_until(async move {
    let nonsend_data = nonsend_data.clone();
    task::spawn_local(async move {
        println!("{}", nonsend_data);
        // ...
    }).await.unwrap();
}).await;
```

With local runtime flavor.

```rust
#[tokio::main(flavor = "local")]
async fn main() {
    let join = tokio::task::spawn_local(async {
        println!("my nonsend data...")
    });

   join.await.unwrap()
 }

```

### fn tokio::task::try_id

```rust
pub fn try_id() -> Option<Id>
```

Returns the `Id` of the currently running task, or `None` if called outside
of a task.

This function is similar to  `task::id()`, except
that it returns `None` rather than panicking if called outside of a task
context.

### fn tokio::task::yield_now

```rust
pub async fn yield_now()
```

Yields execution back to the Tokio runtime.

A task yields by awaiting on `yield_now()`, and may resume when that future
completes (with no output.) The current task will be re-added as a pending
task at the back of the pending queue. Any other pending tasks will be
scheduled. No other waking is required for the task to continue.

See also the usage example in the task module.

**Non-guarantees**

This function may not yield all the way up to the executor if there are any
special combinators above it in the call stack. For example, if a
`tokio::select!` has another branch complete during the same poll as the
`yield_now()`, then the yield is not propagated all the way up to the
runtime.

It is generally not guaranteed that the runtime behaves like you expect it
to when deciding which task to schedule next after a call to `yield_now()`.
In particular, the runtime may choose to poll the task that just ran
`yield_now()` again immediately without polling any other tasks first. For
example, the runtime will not drive the IO driver between every poll of a
task, and this could result in the runtime polling the current task again
immediately even if there is another task that could make progress if that
other task is waiting for a notification from the IO driver.

In general, changes to the order in which the runtime polls tasks is not
considered a breaking change, and your program should be correct no matter
which order the runtime polls your tasks in.

### struct tokio::task::AbortHandle

```rust
pub struct AbortHandle { /* private fields */ }
```

An owned permission to abort a spawned task, without awaiting its completion.

Unlike a `JoinHandle`, an `AbortHandle` does not represent the
permission to await the task’s completion, only to terminate it.

The task may be aborted by calling the `AbortHandle::abort` method.
Dropping an `AbortHandle` releases the permission to terminate the task
— it does not abort the task.

Be aware that tasks spawned using `spawn_blocking` cannot be aborted
because they are not async. If you call `abort` on a `spawn_blocking` task,
then this will not have any effect, and the task will continue running
normally. The exception is if the task has not started running yet; in that
case, calling `abort` may prevent the task from starting.

**Methods**

- `pub fn abort(&self)`
- `pub fn is_finished(&self) -> bool`
- `pub fn id(&self) -> Id`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`

### struct tokio::task::Id

```rust
pub struct Id(/* private fields */);
```

An opaque ID that uniquely identifies a task relative to all other currently
running tasks.

A task’s ID may be re-used for another task only once both of the
following happen:

- The task itself exits.

- There is no active `JoinHandle` associated with this task.

A `JoinHandle` is considered active in the following situations:

- You are explicitly holding a `JoinHandle`, `AbortHandle`, or
`tokio_util::task::AbortOnDropHandle`.

- The task is being tracked by a `JoinSet` or `tokio_util::task::JoinMap`.

**Notes**

- Task IDs are not sequential, and do not indicate the order in which
tasks are spawned, what runtime a task is spawned on, or any other data.

- The task ID of the currently running task can be obtained from inside the
task via the `task::try_id()` and
`task::id()` functions and from outside the task via
the `JoinHandle::id()` function.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn hash<__H: Hasher>(&self, state: &mut __H)`
- `1.3.0 · fn hash_slice<H>(data: &[Self], state: &mut H)where H: Hasher, Self: Sized,`
- `fn cmp(&self, other: &Self) -> Ordering`
- `1.21.0 (const: unstable) · fn max(self, other: Self) -> Selfwhere Self: Sized,`
- `1.21.0 (const: unstable) · fn min(self, other: Self) -> Selfwhere Self: Sized,`
- `1.50.0 (const: unstable) · fn clamp(self, min: Self, max: Self) -> Selfwhere Self: Sized,`
- `fn clamp_to<R>(self, range: R) -> Selfwhere Self: Sized, R: ClampBounds<Self>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn partial_cmp(&self, other: &Self) -> Option<Ordering>`
- `1.0.0 (const: unstable) · fn lt(&self, other: &Rhs) -> bool`
- `1.0.0 (const: unstable) · fn le(&self, other: &Rhs) -> bool`
- `1.0.0 (const: unstable) · fn gt(&self, other: &Rhs) -> bool`
- `1.0.0 (const: unstable) · fn ge(&self, other: &Rhs) -> bool`

### struct tokio::task::JoinError

```rust
pub struct JoinError { /* private fields */ }
```

Task failed to execute to completion.

**Methods**

- `pub fn is_cancelled(&self) -> bool`
- `pub fn is_panic(&self) -> bool`
- `pub fn into_panic(self) -> Box<dyn Any + Send + 'static>`
- `pub fn try_into_panic(self) -> Result<Box<dyn Any + Send + 'static>, JoinError>`
- `pub fn id(&self) -> Id`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(src: JoinError) -> Error`

### struct tokio::task::JoinHandle

```rust
pub struct JoinHandle<T> { /* private fields */ }
```

An owned permission to join on a task (await its termination).

This can be thought of as the equivalent of `std::thread::JoinHandle`
for a Tokio task rather than a thread. Note that the background task
associated with this `JoinHandle` started running immediately when you
called spawn, even if you have not yet awaited the `JoinHandle`.

A `JoinHandle` detaches the associated task when it is dropped, which
means that there is no longer any handle to the task, and no way to `join`
on it.

This `struct` is created by the `task::spawn` and `task::spawn_blocking`
functions.

It is guaranteed that the destructor of the spawned task has finished
before task completion is observed via `JoinHandle` `await`,
`JoinHandle::is_finished` or `AbortHandle::is_finished`.

**Cancel safety**

Awaiting a `&mut JoinHandle<T>` is cancel safe. If it is used as a
branch in `tokio::select!` and another branch completes first, then it
is guaranteed that the output of the task is not lost.

If a `JoinHandle` is dropped, then the task continues running in the
background and its return value is lost.

**Examples**

Creation from `task::spawn`:

```rust
use tokio::task;

let join_handle: task::JoinHandle<_> = task::spawn(async {
    // some work here
});
```

Creation from `task::spawn_blocking`:

```rust
use tokio::task;

let join_handle: task::JoinHandle<_> = task::spawn_blocking(|| {
    // some blocking work here
});
```

The generic parameter `T` in `JoinHandle<T>` is the return type of the spawned task.
If the return value is an `i32`, the join handle has type `JoinHandle<i32>`:

...

**Methods**

- `pub fn abort(&self)`
- `pub fn is_finished(&self) -> bool`
- `pub fn abort_handle(&self) -> AbortHandle`
- `pub fn id(&self) -> Id`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`
- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`

### struct tokio::task::JoinSet

```rust
pub struct JoinSet<T> { /* private fields */ }
```

A collection of tasks spawned on a Tokio runtime.

A `JoinSet` can be used to await the completion of some or all of the tasks
in the set. The set is not ordered, and the tasks will be returned in the
order they complete.

All of the tasks must have the same return type `T`.

When the `JoinSet` is dropped, all tasks in the `JoinSet` are immediately aborted.

**Examples**

Spawn multiple tasks and wait for them.

```rust
use tokio::task::JoinSet;

let mut set = JoinSet::new();

for i in 0..10 {
    set.spawn(async move { i });
}

let mut seen = [false; 10];
while let Some(res) = set.join_next().await {
    let idx = res.unwrap();
    seen[idx] = true;
}

for i in 0..10 {
    assert!(seen[i]);
}
```

**Task ID guarantees**

While a task is tracked in a `JoinSet`, that task’s ID is unique relative
to all other running tasks in Tokio. For this purpose, tracking a task in a
`JoinSet` is equivalent to holding a `JoinHandle` to it. See the task ID
documentation for more info.

**Methods**

- `pub fn new() -> Self`
- `pub fn len(&self) -> usize`
- `pub fn is_empty(&self) -> bool`
- `pub fn spawn<F>(&mut self, task: F) -> AbortHandlewhere F: Future<Output = T> + Send + 'static, T: Send,`
- `pub fn spawn_on<F>(&mut self, task: F, handle: &Handle) -> AbortHandlewhere F: Future<Output = T> + Send + 'static, T: Send,`
- `pub fn spawn_local<F>(&mut self, task: F) -> AbortHandlewhere F: Future<Output = T> + 'static,`
- `pub fn spawn_local_on<F>( &mut self, task: F, local_set: &LocalSet, ) -> AbortHandlewhere F: Future<Output = T> + 'static,`
- `pub fn spawn_blocking<F>(&mut self, f: F) -> AbortHandlewhere F: FnOnce() -> T + Send + 'static, T: Send,`
- `pub fn spawn_blocking_on<F>(&mut self, f: F, handle: &Handle) -> AbortHandlewhere F: FnOnce() -> T + Send + 'static, T: Send,`
- `pub async fn join_next(&mut self) -> Option<Result<T, JoinError>>`
- `pub async fn join_next_with_id(&mut self) -> Option<Result<(Id, T), JoinError>>`
- `pub fn try_join_next(&mut self) -> Option<Result<T, JoinError>>`
- `pub fn try_join_next_with_id(&mut self) -> Option<Result<(Id, T), JoinError>>`
- `pub async fn shutdown(&mut self)`
- `pub async fn join_all(self) -> Vec<T>`
- `pub fn abort_all(&mut self)`
- `pub fn detach_all(&mut self)`
- `pub fn poll_join_next( &mut self, cx: &mut Context<'_>, ) -> Poll<Option<Result<T, JoinError>>>`
- `pub fn poll_join_next_with_id( &mut self, cx: &mut Context<'_>, ) -> Poll<Option<Result<(Id, T), JoinError>>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`
- `fn extend<I>(&mut self, iter: I)where I: IntoIterator<Item = F>,`
- `fn extend_one(&mut self, item: T)`
- `fn extend_reserve(&mut self, additional: usize)`
- `fn from_iter<I: IntoIterator<Item = F>>(iter: I) -> Self`

### struct tokio::task::LocalEnterGuard

```rust
pub struct LocalEnterGuard { /* private fields */ }
```

Context guard for `LocalSet`

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`

### struct tokio::task::LocalKey

```rust
pub struct LocalKey<T: 'static> { /* private fields */ }
```

A key for task-local data.

This type is generated by the `task_local!` macro.

Unlike `std::thread::LocalKey`, `tokio::task::LocalKey` will
not lazily initialize the value on first access. Instead, the
value is first initialized when the future containing
the task-local is first polled by a futures executor, like Tokio.

**Examples**

```rust
tokio::task_local! {
    static NUMBER: u32;
}

NUMBER.scope(1, async move {
    assert_eq!(NUMBER.get(), 1);
}).await;

NUMBER.scope(2, async move {
    assert_eq!(NUMBER.get(), 2);

    NUMBER.scope(3, async move {
        assert_eq!(NUMBER.get(), 3);
    }).await;
}).await;
```

**Methods**

- `pub fn scope<F>(&'static self, value: T, f: F) -> TaskLocalFuture<T, F> ⓘwhere F: Future,`
- `pub fn sync_scope<F, R>(&'static self, value: T, f: F) -> Rwhere F: FnOnce() -> R,`
- `pub fn with<F, R>(&'static self, f: F) -> Rwhere F: FnOnce(&T) -> R,`
- `pub fn try_with<F, R>(&'static self, f: F) -> Result<R, AccessError>where F: FnOnce(&T) -> R,`
- `pub fn get(&'static self) -> T`
- `pub fn try_get(&'static self) -> Result<T, AccessError>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tokio::task::LocalSet

```rust
pub struct LocalSet { /* private fields */ }
```

A set of tasks which are executed on the same thread.

In some cases, it is necessary to run one or more futures that do not
implement `Send` and thus are unsafe to send between threads. In these
cases, a local task set may be used to schedule one or more `!Send`
futures to run together on the same thread.

For example, the following code will not compile:

ⓘ
```rust
use std::rc::Rc;

#[tokio::main]
async fn main() {
    // `Rc` does not implement `Send`, and thus may not be sent between
    // threads safely.
    let nonsend_data = Rc::new("my nonsend data...");

    let nonsend_data = nonsend_data.clone();
    // Because the `async` block here moves `nonsend_data`, the future is `!Send`.
    // Since `tokio::spawn` requires the spawned future to implement `Send`, this
    // will not compile.
    tokio::spawn(async move {
        println!("{}", nonsend_data);
        // ...
    }).await.unwrap();
}
```

**Use with `run_until`**

To spawn `!Send` futures, we can use a local task set to schedule them
on the thread calling `Runtime::block_on`. When running inside of the
local task set, we can use `task::spawn_local`, which can spawn
`!Send` futures. For example:

...

**Methods**

- `pub fn new() -> LocalSet`
- `pub fn enter(&self) -> LocalEnterGuard`
- `pub fn spawn_local<F>(&self, future: F) -> JoinHandle<F::Output> ⓘwhere F: Future + 'static, F::Output: 'static,`
- `pub fn block_on<F>(&self, rt: &Runtime, future: F) -> F::Outputwhere F: Future,`
- `pub async fn run_until<F>(&self, future: F) -> F::Outputwhere F: Future,`
- `pub fn id(&self) -> Id`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `fn default() -> LocalSet`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`
- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`
