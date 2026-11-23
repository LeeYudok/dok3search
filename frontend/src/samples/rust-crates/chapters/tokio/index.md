# Crate tokio

## Overview

A runtime for writing reliable network applications without compromising speed.

Tokio is an event-driven, non-blocking I/O platform for writing asynchronous
applications with the Rust programming language. At a high level, it
provides a few major components:

- Tools for working with asynchronous tasks, including
synchronization primitives and channels and timeouts, sleeps, and
intervals.

- APIs for performing asynchronous I/O, including TCP and UDP sockets,
filesystem operations, and process and signal management.

- A runtime for executing asynchronous code, including a task scheduler,
an I/O driver backed by the operating system’s event queue (`epoll`, `kqueue`,
`IOCP`, etc…), and a high performance timer.

Guide level documentation is found on the website.

**A Tour of Tokio**

Tokio consists of a number of modules that provide a range of functionality
essential for implementing asynchronous applications in Rust. In this
section, we will take a brief tour of Tokio, summarizing the major APIs and
their uses.

The easiest way to get started is to enable all features. Do this by
enabling the `full` feature flag:

```rust
tokio = { version = "1", features = ["full"] }
```

**Authoring applications**

Tokio is great for writing applications and most users in this case shouldn’t
worry too much about what features they should pick. If you’re unsure, we suggest
going with `full` to ensure that you don’t run into any road blocks while you’re
building your application.

**Example**

This example shows the quickest way to get started with Tokio.

```rust
tokio = { version = "1", features = ["full"] }
```

**Authoring libraries**

As a library author your goal should be to provide the lightest weight crate
that is based on Tokio. To achieve this you should ensure that you only enable
the features you need. This allows users to pick up your crate without having
to enable unnecessary features.

**Example**

This example shows how you may want to import features for a library that just
needs to `tokio::spawn` and use a `TcpStream`.

```rust
tokio = { version = "1", features = ["rt", "net"] }
```

**Working With Tasks**

Asynchronous programs in Rust are based around lightweight, non-blocking
units of execution called tasks. The `tokio::task` module provides
important tools for working with tasks:

- The `spawn` function and `JoinHandle` type, for scheduling a new tas

...

## Items

- `fs` — Asynchronous file utilities.
- `io` — Traits, helpers, and type definitions for asynchronous I/O functionality.
- `net` — TCP/UDP/Unix bindings for tokio.
- `runtime` — The Tokio runtime.
- `signal` — Asynchronous signal handling for Tokio.
- `stream` — Due to the Stream trait’s inclusion in std landing later than Tokio’s 1.0 release, most of the Tokio stream utilities have been moved into the tokio-stream crate.
- `sync` — Synchronization primitives for use in asynchronous contexts.
- `task` — Asynchronous green-threads.
- `time` — Utilities for tracking time.
- `join` — Waits on multiple concurrent branches, returning when all branches complete.
- `pin` — Pins a value on the stack.
- `select` — Waits on multiple concurrent branches, returning when the first branch completes, cancelling the remaining branches.
- `task_local` — Declares a new task-local key of type tokio::task::LocalKey.
- `try_join` — Waits on multiple concurrent branches, returning when all branches complete with Ok(_) or on the first Err(_).
- `main` — Marks async function to be executed by the selected runtime. This macro helps set up a Runtime without requiring the user to use Runtime or Builder directly.
- `test` — Marks async function to be executed by runtime, suitable to test environment. This macro helps set up a Runtime without requiring the user to use Runtime or Builder directly.

### macro tokio::join

```rust
macro_rules! join {
    ($(biased;)? $($future:expr),*) => { ... };
}
```

Waits on multiple concurrent branches, returning when all branches
complete.

The `join!` macro must be used inside of async functions, closures, and
blocks.

The `join!` macro takes a list of async expressions and evaluates them
concurrently on the same task. Each async expression evaluates to a future
and the futures from each expression are multiplexed on the current task.

When working with async expressions returning `Result`, `join!` will wait
for all branches complete regardless if any complete with `Err`. Use
`try_join!` to return early when `Err` is encountered.

**Notes**

The supplied futures are stored inline and do not require allocating a
`Vec`.

**Runtime characteristics**

By running all async expressions on the current task, the expressions are
able to run concurrently but not in parallel. This means all
expressions are run on the same thread and if one branch blocks the thread,
all other expressions will be unable to continue. If parallelism is
required, spawn each async expression using `tokio::spawn` and pass the
join handle to `join!`.

**Fairness**

By default, `join!`’s generated future rotates which contained
future is polled first whenever it is woken.

This behavior can be overridden by adding `biased;` to the beginning of the
macro usage. See the examples for details. This will cause `join` to poll
the futures in the order they appear from top to bottom.

You may want this if your futures may interact in a way where known polling order is significant.

But there is an important caveat to this mode. It becomes your responsibility
to ensure that the

...

### macro tokio::pin

```rust
macro_rules! pin {
    ($($x:ident),*) => { ... };
    ($(
            let $x:ident = $init:expr;
    )*) => { ... };
}
```

Pins a value on the stack.

Calls to `async fn` return anonymous `Future` values that are `!Unpin`.
These values must be pinned before they can be polled. Calling `.await` will
handle this, but consumes the future. If it is required to call `.await` on
a `&mut _` reference, the caller is responsible for pinning the future.

Pinning may be done by allocating with `Box::pin` or by using the stack
with the `pin!` macro.

The following will fail to compile:

ⓘ
```rust
async fn my_async_fn() {
    // async logic here
}

#[tokio::main]
async fn main() {
    let mut future = my_async_fn();
    (&mut future).await;
}
```

To make this work requires pinning:

```rust
use tokio::pin;

async fn my_async_fn() {
    // async logic here
}

let future = my_async_fn();
pin!(future);

(&mut future).await;
```

Pinning is useful when using `select!` and stream operators that require `T: Stream + Unpin`.

**Usage**

The `pin!` macro takes identifiers as arguments. It does not work
with expressions.

The following does not compile as an expression is passed to `pin!`.

ⓘ
```rust
async fn my_async_fn() {
    // async logic here
}

#[tokio::main]
async fn main() {
    let mut future = pin!(my_async_fn());
    (&mut future).await;
}
```

**Examples**

Using with select:

...

### macro tokio::select

```rust
macro_rules! select {
    {
        $(
            biased;
        )?
        $(
            $bind:pat = $fut:expr $(, if $cond:expr)? => $handler:expr,
        )*
        $(
            else => $els:expr $(,)?
        )?
    } => { ... };
}
```

Waits on multiple concurrent branches, returning when the first branch
completes, cancelling the remaining branches.

The `select!` macro must be used inside of async functions, closures, and
blocks.

The `select!` macro accepts one or more branches with the following pattern:

```rust
<pattern> = <async expression> (, if <precondition>)? => <handler>,
```

Additionally, the `select!` macro may include a single, optional `else`
branch, which evaluates if none of the other branches match their patterns:

```rust
else => <expression>
```

The macro aggregates all `<async expression>` expressions and runs them
concurrently on the current task. Once the first expression
completes with a value that matches its `<pattern>`, the `select!` macro
returns the result of evaluating the completed branch’s `<handler>`
expression.

Additionally, each branch may include an optional `if` precondition. If the
precondition returns `false`, then the branch is disabled. The provided
`<async expression>` is still evaluated but the resulting future is never
polled. This capability is useful when using `select!` within a loop.

The complete lifecycle of a `select!` expression is as follows:

- Evaluate all provided `<precondition>` expressions. If the precondition
returns `false`, disable the branch for the remainder of the current call
to `select!`. Re-entering `select!` due to a loop clears the “disabled”
state.

- Aggregate the `<async expression>`s from each branch, including the
disabled ones. If the branch is disabled, `<async expression>` is still
evaluated, but the resulting future is not

...

### macro tokio::task_local

```rust
macro_rules! task_local {
    () => { ... };
    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty; $($rest:tt)*) => { ... };
    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty) => { ... };
}
```

Declares a new task-local key of type `tokio::task::LocalKey`.

**Syntax**

The macro wraps any number of static declarations and makes them local to the current task.
Publicity and attributes for each static is preserved. For example:

**Examples**

```rust
task_local! {
    pub static ONE: u32;

    #[allow(unused)]
    static TWO: f32;
}
```

See `LocalKey` documentation for more
information.

### macro tokio::try_join

```rust
macro_rules! try_join {
    ($(biased;)? $($future:expr),*) => { ... };
}
```

Waits on multiple concurrent branches, returning when all branches
complete with `Ok(_)` or on the first `Err(_)`.

The `try_join!` macro must be used inside of async functions, closures, and
blocks.

Similar to `join!`, the `try_join!` macro takes a list of async
expressions and evaluates them concurrently on the same task. Each async
expression evaluates to a future and the futures from each expression are
multiplexed on the current task. The `try_join!` macro returns when all
branches return with `Ok` or when the first branch returns with `Err`.

**Notes**

The supplied futures are stored inline and do not require allocating a
`Vec`.

**Runtime characteristics**

By running all async expressions on the current task, the expressions are
able to run concurrently but not in parallel. This means all
expressions are run on the same thread and if one branch blocks the thread,
all other expressions will be unable to continue. If parallelism is
required, spawn each async expression using `tokio::spawn` and pass the
join handle to `try_join!`.

**Fairness**

By default, `try_join!`’s generated future rotates which
contained future is polled first whenever it is woken.

This behavior can be overridden by adding `biased;` to the beginning of the
macro usage. See the examples for details. This will cause `try_join` to poll
the futures in the order they appear from top to bottom.

You may want this if your futures may interact in a way where known polling order is significant.

But there is an important caveat to this mode. It becomes your responsibility
to ensure that the polling or

...
