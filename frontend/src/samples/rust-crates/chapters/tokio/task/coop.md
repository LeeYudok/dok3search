# Module tokio::task::coop

## Overview

Utilities for improved cooperative scheduling.

**Cooperative scheduling**

A single call to `poll` on a top-level task may potentially do a lot of
work before it returns `Poll::Pending`. If a task runs for a long period of
time without yielding back to the executor, it can starve other tasks
waiting on that executor to execute them, or drive underlying resources.
Since Rust does not have a runtime, it is difficult to forcibly preempt a
long-running task. Instead, this module provides an opt-in mechanism for
futures to collaborate with the executor to avoid starvation.

Consider a future like this one:

```rust
async fn drop_all<I: Stream + Unpin>(mut input: I) {
    while let Some(_) = input.next().await {}
}
```

It may look harmless, but consider what happens under heavy load if the
input stream is always ready. If we spawn `drop_all`, the task will never
yield, and will starve other tasks and resources on the same executor.

To account for this, Tokio has explicit yield points in a number of library
functions, which force tasks to return to the executor periodically.

**unconstrained**

If necessary, `task::unconstrained` lets you opt a future out of Tokio’s cooperative
scheduling. When a future is wrapped with `unconstrained`, it will never be forced to yield to
Tokio. For example:

```rust
use tokio::{task, sync::mpsc};

let fut = async {
    let (tx, mut rx) = mpsc::unbounded_channel();

    for i in 0..1000 {
        let _ = tx.send(());
        // This will always be ready. If coop was in effect, this code would be forced to yield
        // periodically. However, if left unconstrained, then this code will never yield.
        rx.recv().await;
    }
};

task::coop::unconstrained(fut).await;
```

## Items

- `Coop` — Future wrapper to ensure cooperative scheduling created by cooperative.
- `RestoreOnPending` — Value returned by the poll_proceed method.
- `Unconstrained` — Future for the unconstrained method.
- `consume_budget` — Consumes a unit of budget and returns the execution back to the Tokio runtime if the task’s coop budget was exhausted.
- `cooperative` — Creates a wrapper future that makes the inner future cooperate with the Tokio scheduler.
- `has_budget_remaining` — Returns true if there is still budget left on the task.
- `poll_proceed` — Decrements the task budget and returns Poll::Pending if the budget is depleted. This indicates that the task should yield to the scheduler. Otherwise, returns RestoreOnPending which can be used to commit the budget consumption.
- `unconstrained` — Turn off cooperative scheduling for a future. The future will never be forced to yield by Tokio. Using this exposes your service to starvation if the unconstrained future never yields otherwise.

### fn tokio::task::coop::consume_budget

```rust
pub async fn consume_budget()
```

Consumes a unit of budget and returns the execution back to the Tokio
runtime if the task’s coop budget was exhausted.

The task will only yield if its entire coop budget has been exhausted.
This function can be used in order to insert optional yield points into long
computations that do not use Tokio resources like sockets or semaphores,
without redundantly yielding to the runtime each time.

**Examples**

Make sure that a function which returns a sum of (potentially lots of)
iterated values is cooperative.

```rust
async fn sum_iterator(input: &mut impl std::iter::Iterator<Item=i64>) -> i64 {
    let mut sum: i64 = 0;
    while let Some(i) = input.next() {
        sum += i;
        tokio::task::consume_budget().await
    }
    sum
}
```

### fn tokio::task::coop::cooperative

```rust
pub fn cooperative<F: Future>(fut: F) -> Coop<F> ⓘ
```

Creates a wrapper future that makes the inner future cooperate with the Tokio scheduler.

When polled, the wrapper will first call `poll_proceed` to consume task budget, and
immediately yield if the budget has been depleted. If budget was available, the inner future
is polled. The budget consumption will be made final using `RestoreOnPending::made_progress`
if the inner future resolves to its final value.

**Examples**

When you call `recv` on the `Receiver` of a `tokio::sync::mpsc`
channel, task budget will automatically be consumed when the next value is returned.
This makes tasks that use Tokio mpsc channels automatically cooperative.

If you’re using `futures::channel::mpsc`
instead, automatic task budget consumption will not happen. This example shows how can use
`cooperative` to make `futures::channel::mpsc` channels cooperate with the scheduler in the
same way Tokio channels do.

```rust
use tokio::task::coop::cooperative;
use futures::channel::mpsc::Receiver;
use futures::stream::StreamExt;

async fn receive_next<T>(receiver: &mut Receiver<T>) -> Option<T> {
    // Use `StreamExt::next` to obtain a `Future` that resolves to the next value
    let recv_future = receiver.next();
    // Wrap it a cooperative wrapper
    let coop_future = cooperative(recv_future);
    // And await
    coop_future.await
}
```

### fn tokio::task::coop::has_budget_remaining

```rust
pub fn has_budget_remaining() -> bool
```

Returns `true` if there is still budget left on the task.

**Examples**

This example defines a `Timeout` future that requires a given `future` to complete before the
specified duration elapses. If it does, its result is returned; otherwise, an error is returned
and the future is canceled.

Note that the future could exhaust the budget before we evaluate the timeout. Using `has_budget_remaining`,
we can detect this scenario and ensure the timeout is always checked.

```rust
pub struct Timeout<T> {
    future: T,
    delay: Pin<Box<Sleep>>,
}

impl<T> Future for Timeout<T>
where
    T: Future + Unpin,
{
    type Output = Result<T::Output, ()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = Pin::into_inner(self);
        let future = Pin::new(&mut this.future);
        let delay = Pin::new(&mut this.delay);

        // check if the future is ready
        let had_budget_before = coop::has_budget_remaining();
        if let Poll::Ready(v) = future.poll(cx) {
            return Poll::Ready(Ok(v));
        }
        let has_budget_now = coop::has_budget_remaining();

        // evaluate the timeout
        if let (true, false) = (had_budget_before, has_budget_now) {
            // it is the underlying future that exhausted the budget
            ready!(pin!(coop::unconstrained(delay)).poll(cx));
        } else {
            ready!(delay.poll(cx));
        }
        return Poll::Ready(Err(()));
    }
}
```

### fn tokio::task::coop::poll_proceed

```rust
pub fn poll_proceed(cx: &mut Context<'_>) -> Poll<RestoreOnPending>
```

Decrements the task budget and returns `Poll::Pending` if the budget is depleted.
This indicates that the task should yield to the scheduler. Otherwise, returns
`RestoreOnPending` which can be used to commit the budget consumption.

The returned `RestoreOnPending` will revert the budget to its former
value when dropped unless `RestoreOnPending::made_progress`
is called. It is the caller’s responsibility to do so when it was able to
make progress after the call to `poll_proceed`.
Restoring the budget automatically ensures the task can try to make progress in some other
way.

Note that `RestoreOnPending` restores the budget as it was before `poll_proceed`.
Therefore, if the budget is further adjusted between when `poll_proceed` returns and
`RestoreOnPending` is dropped, those adjustments are erased unless the caller indicates
that progress was made.

**Examples**

This example wraps the `futures::channel::mpsc::UnboundedReceiver` to
cooperate with the Tokio scheduler. Each time a value is received, task budget
is consumed. If no budget is available, the task yields to the scheduler.

...

### fn tokio::task::coop::unconstrained

```rust
pub fn unconstrained<F>(inner: F) -> Unconstrained<F> ⓘ
```

Turn off cooperative scheduling for a future. The future will never be forced to yield by
Tokio. Using this exposes your service to starvation if the unconstrained future never yields
otherwise.

See also the usage example in the task module.

### struct tokio::task::coop::Coop

```rust
pub struct Coop<F: Future> { /* private fields */ }
```

Future wrapper to ensure cooperative scheduling created by `cooperative`.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`

### struct tokio::task::coop::RestoreOnPending

```rust
pub struct RestoreOnPending(/* private fields */);
```

Value returned by the `poll_proceed` method.

**Methods**

- `pub fn made_progress(&self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`

### struct tokio::task::coop::Unconstrained

```rust
pub struct Unconstrained<F> { /* private fields */ }
```

Future for the `unconstrained` method.

**Methods**

- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`
