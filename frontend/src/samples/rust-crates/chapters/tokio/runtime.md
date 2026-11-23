# Module tokio::runtime

## Overview

The Tokio runtime.

Unlike other Rust programs, asynchronous applications require runtime
support. In particular, the following runtime services are necessary:

- An I/O event loop, called the driver, which drives I/O resources and
dispatches I/O events to tasks that depend on them.

- A scheduler to execute tasks that use these I/O resources.

- A timer for scheduling work to run after a set period of time.

Tokio’s `Runtime` bundles all of these services as a single type, allowing
them to be started, shut down, and configured together. However, often it is
not required to configure a `Runtime` manually, and a user may just use the
`tokio::main` attribute macro, which creates a `Runtime` under the hood.

**Choose your runtime**

Here is the rules of thumb to choose the right runtime for your application.

```rust
   +------------------------------------------------------+
   | Do you want work-stealing or multi-thread scheduler? |
   +------------------------------------------------------+
                   | Yes              | No
                   |                  |
                   |                  |
                   v                  |
     +------------------------+       |
     | Multi-threaded Runtime |       |
     +------------------------+       |
                                      |
                                      V
                     +--------------------------------+
                     | Do you execute `!Send` Future? |
                     +--------------------------------+
                           | Yes                 | No
                           |                     |
                           V                     |
                   +---------------+             |
                   | Local Runtime |             |
                   +---------------+             |
                                                 |
                                                 v
                                     +------------------------+
                                     | Current-thread Runtime |
                                     +------------------------+
```

The above decision tree is not exhaustive. there are other factors that
may influence your decision.

**Bridging with sync code**

See https://tokio.rs/tokio/topics/bridging for details.

**NUMA awareness**

The tokio runtime is not NUMA (Non-Uniform Memo

...

## Items

- `Builder` — Builds Tokio Runtime with custom configuration values.
- `EnterGuard` — Runtime context guard.
- `Handle` — Handle to the runtime.
- `Id` — An opaque ID that uniquely identifies a runtime relative to all other currently running runtimes.
- `LocalOptions` — LocalRuntime-only config options
- `LocalRuntime` — A local Tokio runtime.
- `Runtime` — The Tokio runtime.
- `RuntimeMetrics` — Handle to the runtime’s metrics.
- `TryCurrentError` — Error returned by try_current when no Runtime has been started
- `RuntimeFlavor` — The flavor of a Runtime.
- `is_rt_shutdown_err` — Checks whether the given error was emitted by Tokio when shutting down its runtime.
- `worker_index` — Returns the index of the current worker thread, if called from a runtime worker thread.

### enum tokio::runtime::RuntimeFlavor

```rust
#[non_exhaustive]pub enum RuntimeFlavor {
    CurrentThread,
    MultiThread,
}
```

The flavor of a `Runtime`.

This is the return type for `Handle::runtime_flavor`.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### fn tokio::runtime::is_rt_shutdown_err

```rust
pub fn is_rt_shutdown_err(err: &Error) -> bool
```

Checks whether the given error was emitted by Tokio when shutting down its runtime.

**Examples**

```rust
use tokio::runtime::Runtime;
use tokio::net::TcpListener;

fn main() {
    let rt1 = Runtime::new().unwrap();
    let rt2 = Runtime::new().unwrap();

    let listener = rt1.block_on(async {
        TcpListener::bind("127.0.0.1:0").await.unwrap()
    });

    drop(rt1);

    rt2.block_on(async {
        let res = listener.accept().await;
        assert!(res.is_err());
        assert!(tokio::runtime::is_rt_shutdown_err(res.as_ref().unwrap_err()));
    });
}
```

### fn tokio::runtime::worker_index

```rust
pub fn worker_index() -> Option<usize>
```

Returns the index of the current worker thread, if called from a
runtime worker thread.

The returned value is a 0-based index matching the worker indices
used by `RuntimeMetrics` methods such as
`worker_total_busy_duration`.

Returns `None` when called from outside a runtime worker thread
(for example, from a blocking thread or a non-Tokio thread). On the
multi-thread runtime, the thread that calls `Runtime::block_on` is
not a worker thread, so this also returns `None` there.

For the current-thread runtime, this returns `Some(0)` when called from
the thread that currently owns the runtime driver. If multiple threads
call `Runtime::block_on` concurrently, calls on threads that do not
own the driver return `None`. A `LocalRuntime` can only be driven from
its owning thread, so calls inside `block_on` always return `Some(0)`.

Note that the result may change across `.await` points, as the
task may be moved to a different worker thread by the scheduler.

**Examples**

```rust
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let index = tokio::spawn(async {
        tokio::runtime::worker_index()
    }).await.unwrap();
    println!("Task ran on worker {:?}", index);
}
```

### struct tokio::runtime::Builder

```rust
pub struct Builder { /* private fields */ }
```

Builds Tokio Runtime with custom configuration values.

Methods can be chained in order to set the configuration values. The
Runtime is constructed by calling `build`.

New instances of `Builder` are obtained via `Builder::new_multi_thread`
or `Builder::new_current_thread`.

See function level documentation for details on the various configuration
settings.

**Examples**

```rust
use tokio::runtime::Builder;

fn main() {
    // build runtime
    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("my-custom-name")
        .thread_stack_size(3 * 1024 * 1024)
        .build()
        .unwrap();

    // use runtime ...
}
```

**Methods**

- `pub fn new_current_thread() -> Builder`
- `pub fn new_multi_thread() -> Builder`
- `pub fn enable_all(&mut self) -> &mut Self`
- `pub fn worker_threads(&mut self, val: usize) -> &mut Self`
- `pub fn max_blocking_threads(&mut self, val: usize) -> &mut Self`
- `pub fn thread_name(&mut self, val: impl Into<String>) -> &mut Self`
- `pub fn name(&mut self, val: impl Into<String>) -> &mut Self`
- `pub fn thread_name_fn<F>(&mut self, f: F) -> &mut Selfwhere F: Fn() -> String + Send + Sync + 'static,`
- `pub fn thread_stack_size(&mut self, val: usize) -> &mut Self`
- `pub fn on_thread_start<F>(&mut self, f: F) -> &mut Selfwhere F: Fn() + Send + Sync + 'static,`
- `pub fn on_thread_stop<F>(&mut self, f: F) -> &mut Selfwhere F: Fn() + Send + Sync + 'static,`
- `pub fn on_thread_park<F>(&mut self, f: F) -> &mut Selfwhere F: Fn() + Send + Sync + 'static,`
- `pub fn on_thread_unpark<F>(&mut self, f: F) -> &mut Selfwhere F: Fn() + Send + Sync + 'static,`
- `pub fn build(&mut self) -> Result<Runtime>`
- `pub fn build_local(&mut self, _options: LocalOptions) -> Result<LocalRuntime>`
- `pub fn thread_keep_alive(&mut self, duration: Duration) -> &mut Self`
- `pub fn global_queue_interval(&mut self, val: u32) -> &mut Self`
- `pub fn event_interval(&mut self, val: u32) -> &mut Self`
- `pub fn enable_io(&mut self) -> &mut Self`
- `pub fn max_io_events_per_tick(&mut self, capacity: usize) -> &mut Self`
- `pub fn max_io_events_per_busy_tick(&mut self, capacity: usize) -> &mut Self`
- `pub fn enable_time(&mut self) -> &mut Self`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`

### struct tokio::runtime::EnterGuard

```rust
pub struct EnterGuard<'a> { /* private fields */ }
```

Runtime context guard.

Returned by `Runtime::enter` and `Handle::enter`, the context guard exits
the runtime context on drop.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tokio::runtime::Handle

```rust
pub struct Handle { /* private fields */ }
```

Handle to the runtime.

The handle is internally reference-counted and can be freely cloned. A handle can be
obtained using the `Runtime::handle` method.

**Methods**

- `pub fn enter(&self) -> EnterGuard<'_>`
- `pub fn current() -> Self`
- `pub fn try_current() -> Result<Self, TryCurrentError>`
- `pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output> ⓘwhere F: Future + Send + 'static, F::Output: Send + 'static,`
- `pub fn spawn_blocking<F, R>(&self, func: F) -> JoinHandle<R> ⓘwhere F: FnOnce() -> R + Send + 'static, R: Send + 'static,`
- `pub fn block_on<F: Future>(&self, future: F) -> F::Output`
- `pub fn runtime_flavor(&self) -> RuntimeFlavor`
- `pub fn id(&self) -> Id`
- `pub fn name(&self) -> Option<&str>`
- `pub fn metrics(&self) -> RuntimeMetrics`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tokio::runtime::Id

```rust
pub struct Id(/* private fields */);
```

An opaque ID that uniquely identifies a runtime relative to all other currently
running runtimes.

**Notes**

- Runtime IDs are unique relative to other currently running runtimes.
When a runtime completes, the same ID may be used for another runtime.

- Runtime IDs are not sequential, and do not indicate the order in which
runtimes are started or any other data.

- The runtime ID of the currently running task can be obtained from the
Handle.

**Examples**

```rust
use tokio::runtime::Handle;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
  println!("Current runtime id: {}", Handle::current().id());
}
```

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn hash<__H: Hasher>(&self, state: &mut __H)`
- `1.3.0 · fn hash_slice<H>(data: &[Self], state: &mut H)where H: Hasher, Self: Sized,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### struct tokio::runtime::LocalOptions

```rust
#[non_exhaustive]pub struct LocalOptions { /* private fields */ }
```

`LocalRuntime`-only config options

Currently, there are no such options, but in the future, things like `!Send + !Sync` hooks may
be added.

Use `LocalOptions::default()` to create the default set of options. This type is used with
`Builder::build_local`.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`

### struct tokio::runtime::LocalRuntime

```rust
pub struct LocalRuntime { /* private fields */ }
```

A local Tokio runtime.

This runtime is capable of driving tasks which are not `Send + Sync` without the use of a
`LocalSet`, and thus supports `spawn_local` without the need for a `LocalSet` context.

This runtime cannot be moved between threads or driven from different threads.

This runtime is incompatible with `LocalSet`. You should not attempt to drive a `LocalSet` within a
`LocalRuntime`.

Currently, this runtime supports one flavor, which is internally identical to `current_thread`,
save for the aforementioned differences related to `spawn_local`.

For more general information on how to use runtimes, see the module docs.

**Methods**

- `pub fn new() -> Result<LocalRuntime>`
- `pub fn handle(&self) -> &Handle`
- `pub fn spawn_local<F>(&self, future: F) -> JoinHandle<F::Output> ⓘwhere F: Future + 'static, F::Output: 'static,`
- `pub fn spawn_blocking<F, R>(&self, func: F) -> JoinHandle<R> ⓘwhere F: FnOnce() -> R + Send + 'static, R: Send + 'static,`
- `pub fn block_on<F: Future>(&self, future: F) -> F::Output`
- `pub fn enter(&self) -> EnterGuard<'_>`
- `pub fn shutdown_timeout(self, duration: Duration)`
- `pub fn shutdown_background(self)`
- `pub fn metrics(&self) -> RuntimeMetrics`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`

### struct tokio::runtime::Runtime

```rust
pub struct Runtime { /* private fields */ }
```

The Tokio runtime.

The runtime provides an I/O driver, task scheduler, timer, and
blocking pool, necessary for running asynchronous tasks.

Instances of `Runtime` can be created using `new`, or `Builder`.
However, most users will use the `#[tokio::main]` annotation on
their entry point instead.

See module level documentation for more details.

**Shutdown**

Shutting down the runtime is done by dropping the value, or calling
`shutdown_background` or `shutdown_timeout`.

Tasks spawned through `Runtime::spawn` keep running until they yield.
Then they are dropped. They are not guaranteed to run to completion, but
might do so if they do not yield until completion.

Blocking functions spawned through `Runtime::spawn_blocking` keep running
until they return.

The thread initiating the shutdown blocks until all spawned work has been
stopped. This can take an indefinite amount of time. The `Drop`
implementation waits forever for this.

The `shutdown_background` and `shutdown_timeout` methods can be used if
waiting forever is undesired. When the timeout is reached, spawned work that
did not stop in time and threads running it are leaked. The work continues
to run until one of the stopping conditions is fulfilled, but the thread
initiating the shutdown is unblocked.

Once the runtime has been dropped, any outstanding I/O resources bound to
it will no longer function. Calling any method on them will result in an
error.

**Sharing**

There are several ways to establish shared access to a Tokio runtime:

- Using an `Arc<Runtime>`.

- Using a `Handle`.

- Entering the runtime context.

...

**Methods**

- `pub fn new() -> Result<Runtime>`
- `pub fn handle(&self) -> &Handle`
- `pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output> ⓘwhere F: Future + Send + 'static, F::Output: Send + 'static,`
- `pub fn spawn_blocking<F, R>(&self, func: F) -> JoinHandle<R> ⓘwhere F: FnOnce() -> R + Send + 'static, R: Send + 'static,`
- `pub fn block_on<F: Future>(&self, future: F) -> F::Output`
- `pub fn enter(&self) -> EnterGuard<'_>`
- `pub fn shutdown_timeout(self, duration: Duration)`
- `pub fn shutdown_background(self)`
- `pub fn metrics(&self) -> RuntimeMetrics`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`

### struct tokio::runtime::RuntimeMetrics

```rust
pub struct RuntimeMetrics { /* private fields */ }
```

Handle to the runtime’s metrics.

This handle is internally reference-counted and can be freely cloned. A
`RuntimeMetrics` handle is obtained using the `Runtime::metrics` method.

**Methods**

- `pub fn num_workers(&self) -> usize`
- `pub fn num_alive_tasks(&self) -> usize`
- `pub fn global_queue_depth(&self) -> usize`
- `pub fn worker_total_busy_duration(&self, worker: usize) -> Duration`
- `pub fn worker_park_count(&self, worker: usize) -> u64`
- `pub fn worker_park_unpark_count(&self, worker: usize) -> u64`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tokio::runtime::TryCurrentError

```rust
pub struct TryCurrentError { /* private fields */ }
```

Error returned by `try_current` when no Runtime has been started

**Methods**

- `pub fn is_missing_context(&self) -> bool`
- `pub fn is_thread_local_destroyed(&self) -> bool`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
