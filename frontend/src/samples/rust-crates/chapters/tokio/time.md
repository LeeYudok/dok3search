# Module tokio::time

## Overview

Utilities for tracking time.

This module provides a number of types for executing code after a set period
of time.

- 

`Sleep` is a future that does no work and completes at a specific `Instant`
in time.

- 

`Interval` is a stream yielding a value at a fixed period. It is
initialized with a `Duration` and repeatedly yields each time the duration
elapses.

- 

`Timeout`: Wraps a future or stream, setting an upper bound to the amount
of time it is allowed to execute. If the future or stream does not
complete in time, then it is canceled and an error is returned.

These types are sufficient for handling a large number of scenarios
involving time.

These types must be used from within the context of the `Runtime`.

**Examples**

Wait 100ms and print “100 ms have elapsed”

```rust
use std::time::Duration;
use tokio::time::sleep;

sleep(Duration::from_millis(100)).await;
println!("100 ms have elapsed");
```

Require that an operation takes no more than 1s.

```rust
use tokio::time::{timeout, Duration};

async fn long_future() {
    // do work here
}

let res = timeout(Duration::from_secs(1), long_future()).await;

if res.is_err() {
    println!("operation timed out");
}
```

A simple example using `interval` to execute a task every two seconds.

The difference between `interval` and `sleep` is that an `interval`
measures the time since the last tick, which means that `.tick().await` may
wait for a shorter time than the duration specified for the interval
if some time has passed between calls to `.tick().await`.

If the tick in the example below was replaced with `sleep`, the task
would only be executed once every three seconds, and not every two
seconds.

```rust
use tokio::time;

async fn task_that_takes_a_second() {
    println!("hello");
    time::sleep(time::Duration::from_secs(1)).await
}

let mut interval = time::interval(time::Duration::from_secs(2));
for _i in 0..5 {
    interval.tick().await;
    task_that_takes_a_second().await;
}
```

## Items

- `error` — Time error types.
- `Instant` — A measurement of a monotonically nondecreasing clock. Opaque and useful only with Duration.
- `Interval` — Interval returned by interval and interval_at.
- `Sleep` — Future returned by sleep and sleep_until.
- `Timeout` — Future returned by timeout and timeout_at.
- `MissedTickBehavior` — Defines the behavior of an Interval when it misses a tick.
- `interval` — Creates new Interval that yields with interval of period. The first tick completes immediately. The default MissedTickBehavior is Burst, but this can be configured by calling set_missed_tick_behavior.
- `interval_at` — Creates new Interval that yields with interval of period with the first tick completing at start. The default MissedTickBehavior is Burst, but this can be configured by calling set_missed_tick_behavior.
- `sleep` — Waits until duration has elapsed.
- `sleep_until` — Waits until deadline is reached.
- `timeout` — Requires a Future to complete before the specified duration has elapsed.
- `timeout_at` — Requires a Future to complete before the specified instant in time.

### enum tokio::time::MissedTickBehavior

```rust
pub enum MissedTickBehavior {
    Burst,
    Delay,
    Skip,
}
```

Defines the behavior of an `Interval` when it misses a tick.

Sometimes, an `Interval`’s tick is missed. For example, consider the
following:

```rust
use tokio::time::{self, Duration};

// ticks every 2 milliseconds
let mut interval = time::interval(Duration::from_millis(2));
for _ in 0..5 {
    interval.tick().await;
    // if this takes more than 2 milliseconds, a tick will be delayed
    task_that_takes_one_to_three_millis().await;
}
```

Generally, a tick is missed if too much time is spent without calling
`Interval::tick()`.

By default, when a tick is missed, `Interval` fires ticks as quickly as it
can until it is “caught up” in time to where it should be.
`MissedTickBehavior` can be used to specify a different behavior for
`Interval` to exhibit. Each variant represents a different strategy.

Note that because the executor cannot guarantee exact precision with timers,
these strategies will only apply when the delay is greater than 5
milliseconds.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### fn tokio::time::interval

```rust
pub fn interval(period: Duration) -> Interval
```

Creates new `Interval` that yields with interval of `period`. The first
tick completes immediately. The default `MissedTickBehavior` is
`Burst`, but this can be configured
by calling `set_missed_tick_behavior`.

An interval will tick indefinitely. At any time, the `Interval` value can
be dropped. This cancels the interval.

This function is equivalent to
`interval_at(Instant::now(), period)`.

**Panics**

This function panics if `period` is zero.

**Examples**

```rust
use tokio::time::{self, Duration};

let mut interval = time::interval(Duration::from_millis(10));

interval.tick().await; // ticks immediately
interval.tick().await; // ticks after 10ms
interval.tick().await; // ticks after 10ms

// approximately 20ms have elapsed.
```

A simple example using `interval` to execute a task every two seconds.

The difference between `interval` and `sleep` is that an `Interval`
measures the time since the last tick, which means that `.tick().await`
may wait for a shorter time than the duration specified for the interval
if some time has passed between calls to `.tick().await`.

If the tick in the example below was replaced with `sleep`, the task
would only be executed once every three seconds, and not every two
seconds.

```rust
use tokio::time;

async fn task_that_takes_a_second() {
    println!("hello");
    time::sleep(time::Duration::from_secs(1)).await
}

let mut interval = time::interval(time::Duration::from_secs(2));
for _i in 0..5 {
    interval.tick().await;
    task_that_takes_a_second().await;
}
```

### fn tokio::time::interval_at

```rust
pub fn interval_at(start: Instant, period: Duration) -> Interval
```

Creates new `Interval` that yields with interval of `period` with the
first tick completing at `start`. The default `MissedTickBehavior` is
`Burst`, but this can be configured
by calling `set_missed_tick_behavior`.

An interval will tick indefinitely. At any time, the `Interval` value can
be dropped. This cancels the interval.

**Panics**

This function panics if `period` is zero.

**Examples**

```rust
use tokio::time::{interval_at, Duration, Instant};

let start = Instant::now() + Duration::from_millis(50);
let mut interval = interval_at(start, Duration::from_millis(10));

interval.tick().await; // ticks after 50ms
interval.tick().await; // ticks after 10ms
interval.tick().await; // ticks after 10ms

// approximately 70ms have elapsed.
```

### fn tokio::time::sleep

```rust
pub fn sleep(duration: Duration) -> Sleep ⓘ
```

Waits until `duration` has elapsed.

Equivalent to `sleep_until(Instant::now() + duration)`. An asynchronous
analog to `std::thread::sleep`.

No work is performed while awaiting on the sleep future to complete. `Sleep`
operates at millisecond granularity and should not be used for tasks that
require high-resolution timers. The implementation is platform specific,
and some platforms (specifically Windows) will provide timers with a
larger resolution than 1 ms.

To run something regularly on a schedule, see `interval`.

**Cancellation**

Canceling a sleep instance is done by dropping the returned future. No additional
cleanup work is required.

**Examples**

Wait 100ms and print “100 ms have elapsed”.

```rust
use tokio::time::{sleep, Duration};

sleep(Duration::from_millis(100)).await;
println!("100 ms have elapsed");
```

See the documentation for the `Sleep` type for more examples.

**Panics**

This function panics if there is no current timer set.

It can be triggered when `Builder::enable_time` or
`Builder::enable_all` are not included in the builder.

It can also panic whenever a timer is created outside of a
Tokio runtime. That is why `rt.block_on(sleep(...))` will panic,
since the function is executed outside of the runtime.
Whereas `rt.block_on(async {sleep(...).await})` doesn’t panic.
And this is because wrapping the function on an async makes it lazy,
and so gets executed inside the runtime successfully without
panicking.

### fn tokio::time::sleep_until

```rust
pub fn sleep_until(deadline: Instant) -> Sleep ⓘ
```

Waits until `deadline` is reached.

No work is performed while awaiting on the sleep future to complete. `Sleep`
operates at millisecond granularity and should not be used for tasks that
require high-resolution timers.

To run something regularly on a schedule, see `interval`.

**Cancellation**

Canceling a sleep instance is done by dropping the returned future. No additional
cleanup work is required.

**Examples**

Wait 100ms and print “100 ms have elapsed”.

```rust
use tokio::time::{sleep_until, Instant, Duration};

sleep_until(Instant::now() + Duration::from_millis(100)).await;
println!("100 ms have elapsed");
```

See the documentation for the `Sleep` type for more examples.

**Panics**

This function panics if there is no current timer set.

It can be triggered when `Builder::enable_time` or
`Builder::enable_all` are not included in the builder.

It can also panic whenever a timer is created outside of a
Tokio runtime. That is why `rt.block_on(sleep(...))` will panic,
since the function is executed outside of the runtime.
Whereas `rt.block_on(async {sleep(...).await})` doesn’t panic.
And this is because wrapping the function on an async makes it lazy,
and so gets executed inside the runtime successfully without
panicking.

### fn tokio::time::timeout

```rust
pub fn timeout<F>(duration: Duration, future: F) -> Timeout<F::IntoFuture> ⓘwhere
    F: IntoFuture,
```

Requires a `Future` to complete before the specified duration has elapsed.

If the future completes before the duration has elapsed, then the completed
value is returned. Otherwise, an error is returned and the future is
canceled.

Note that the future is polled before the timeout is checked, so if the future
does not yield during execution then it is possible for the future to complete
and exceed the timeout without returning an error.

This function returns a future whose return type is `Result``<T,``Elapsed``>`, where `T` is the
return type of the provided future.

If the provided future completes immediately, then the future returned from
this function is guaranteed to complete immediately with an `Ok` variant
no matter the provided duration.

**Cancellation**

Cancelling a timeout is done by dropping the future. No additional cleanup
or other work is required.

The original future may be obtained by calling `Timeout::into_inner`. This
consumes the `Timeout`.

**Examples**

Create a new `Timeout` set to expire in 10 milliseconds.

```rust
use tokio::time::timeout;
use tokio::sync::oneshot;

use std::time::Duration;

let (tx, rx) = oneshot::channel();

// Wrap the future with a `Timeout` set to expire in 10 milliseconds.
if let Err(_) = timeout(Duration::from_millis(10), rx).await {
    println!("did not receive value within 10 ms");
}
```

**Panics**

This function panics if there is no current timer set.

It can be triggered when `Builder::enable_time` or
`Builder::enable_all` are not included in the builder.

It can also panic whenever a timer is created outside of a

...

### fn tokio::time::timeout_at

```rust
pub fn timeout_at<F>(deadline: Instant, future: F) -> Timeout<F::IntoFuture> ⓘwhere
    F: IntoFuture,
```

Requires a `Future` to complete before the specified instant in time.

If the future completes before the instant is reached, then the completed
value is returned. Otherwise, an error is returned.

This function returns a future whose return type is `Result``<T,``Elapsed``>`, where `T` is the
return type of the provided future.

If the provided future completes immediately, then the future returned from
this function is guaranteed to complete immediately with an `Ok` variant
no matter the provided deadline.

**Cancellation**

Cancelling a timeout is done by dropping the future. No additional cleanup
or other work is required.

The original future may be obtained by calling `Timeout::into_inner`. This
consumes the `Timeout`.

**Examples**

Create a new `Timeout` set to expire in 10 milliseconds.

```rust
use tokio::time::{Instant, timeout_at};
use tokio::sync::oneshot;

use std::time::Duration;

let (tx, rx) = oneshot::channel();

// Wrap the future with a `Timeout` set to expire 10 milliseconds into the
// future.
if let Err(_) = timeout_at(Instant::now() + Duration::from_millis(10), rx).await {
    println!("did not receive value within 10 ms");
}
```

**Panics**

This function panics if there is no current timer set.

It can be triggered when `Builder::enable_time` or
`Builder::enable_all` are not included in the builder.

It can also panic whenever a timer is created outside of a
Tokio runtime. That is why `rt.block_on(sleep(...))` will panic,
since the function is executed outside of the runtime.
Whereas `rt.block_on(async {sleep(...).await})` doesn’t panic.
And this is

...

### struct tokio::time::Instant

```rust
pub struct Instant { /* private fields */ }
```

A measurement of a monotonically nondecreasing clock.
Opaque and useful only with `Duration`.

Instants are always guaranteed to be no less than any previously measured
instant when created, and are often useful for tasks such as measuring
benchmarks or timing how long an operation takes.

Note, however, that instants are not guaranteed to be steady. In other
words, each tick of the underlying clock may not be the same length (e.g.
some seconds may be longer than others). An instant may jump forwards or
experience time dilation (slow down or speed up), but it will never go
backwards.

Instants are opaque types that can only be compared to one another. There is
no method to get “the number of seconds” from an instant. Instead, it only
allows measuring the duration between two instants (or comparing two
instants).

The size of an `Instant` struct may vary depending on the target operating
system.

**Note**

This type wraps the inner `std` variant and is used to align the Tokio
clock for uses of `now()`. This can be useful for testing where you can
take advantage of `time::pause()` and `time::advance()`.

**Methods**

- `pub fn now() -> Instant`
- `pub fn from_std(std: Instant) -> Instant`
- `pub fn into_std(self) -> Instant`
- `pub fn duration_since(&self, earlier: Instant) -> Duration`
- `pub fn checked_duration_since(&self, earlier: Instant) -> Option<Duration>`
- `pub fn saturating_duration_since(&self, earlier: Instant) -> Duration`
- `pub fn elapsed(&self) -> Duration`
- `pub fn checked_add(&self, duration: Duration) -> Option<Instant>`
- `pub fn checked_sub(&self, duration: Duration) -> Option<Instant>`
- `fn add(self, other: Duration) -> Instant`
- `fn add_assign(&mut self, rhs: Duration)`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `fn from(time: Instant) -> Instant`
- `fn from(time: Instant) -> Instant`
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
- `fn sub(self, rhs: Instant) -> Duration`
- `fn sub(self, rhs: Duration) -> Instant`
- `fn sub_assign(&mut self, rhs: Duration)`

### struct tokio::time::Interval

```rust
pub struct Interval { /* private fields */ }
```

Interval returned by `interval` and `interval_at`.

This type allows you to wait on a sequence of instants with a certain
duration between each instant. Unlike calling `sleep` in a loop, this lets
you count the time spent between the calls to `sleep` as well.

An `Interval` can be turned into a `Stream` with `IntervalStream`.

**Methods**

- `pub async fn tick(&mut self) -> Instant`
- `pub fn poll_tick(&mut self, cx: &mut Context<'_>) -> Poll<Instant>`
- `pub fn reset(&mut self)`
- `pub fn reset_immediately(&mut self)`
- `pub fn reset_after(&mut self, after: Duration)`
- `pub fn reset_at(&mut self, deadline: Instant)`
- `pub fn missed_tick_behavior(&self) -> MissedTickBehavior`
- `pub fn set_missed_tick_behavior(&mut self, behavior: MissedTickBehavior)`
- `pub fn period(&self) -> Duration`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tokio::time::Sleep

```rust
pub struct Sleep { /* private fields */ }
```

Future returned by `sleep` and `sleep_until`.

This type does not implement the `Unpin` trait, which means that if you
use it with `select!` or by calling `poll`, you have to pin it first.
If you use it with `.await`, this does not apply.

**Examples**

Wait 100ms and print “100 ms have elapsed”.

```rust
use tokio::time::{sleep, Duration};

sleep(Duration::from_millis(100)).await;
println!("100 ms have elapsed");
```

Use with `select!`. Pinning the `Sleep` with `tokio::pin!` is
necessary when the same `Sleep` is selected on multiple times.

```rust
use tokio::time::{self, Duration, Instant};

let sleep = time::sleep(Duration::from_millis(10));
tokio::pin!(sleep);

loop {
    tokio::select! {
        () = &mut sleep => {
            println!("timer elapsed");
            sleep.as_mut().reset(Instant::now() + Duration::from_millis(50));
        },
    }
}
```

Use in a struct with boxing. By pinning the `Sleep` with a `Box`, the
`HasSleep` struct implements `Unpin`, even though `Sleep` does not.

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::time::Sleep;

struct HasSleep {
    sleep: Pin<Box<Sleep>>,
}

impl Future for HasSleep {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        self.sleep.as_mut().poll(cx)
    }
}
```

Use in a struct with pin projection. This method avoids the `Box`, but
the `HasSleep` struct will not be `Unpin` as a consequence.

...

**Methods**

- `pub fn deadline(&self) -> Instant`
- `pub fn is_elapsed(&self) -> bool`
- `pub fn reset(self: Pin<&mut Self>, deadline: Instant)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`

### struct tokio::time::Timeout

```rust
pub struct Timeout<T> { /* private fields */ }
```

Future returned by `timeout` and `timeout_at`.

**Methods**

- `pub fn get_ref(&self) -> &T`
- `pub fn get_mut(&mut self) -> &mut T`
- `pub fn into_inner(self) -> T`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`
