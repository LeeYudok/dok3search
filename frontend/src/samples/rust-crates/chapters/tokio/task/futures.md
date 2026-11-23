# Module tokio::task::futures

## Overview

Task-related futures.

## Items

- `TaskLocalFuture` — A future that sets a value T of a task local for the future F during its execution.

### struct tokio::task::futures::TaskLocalFuture

```rust
pub struct TaskLocalFuture<T, F>where
    T: 'static,{ /* private fields */ }
```

A future that sets a value `T` of a task local for the future `F` during
its execution.

The value of the task-local must be `'static` and will be dropped on the
completion of the future.

Created by the function `LocalKey::scope`.

**Examples**

```rust
tokio::task_local! {
    static NUMBER: u32;
}

NUMBER.scope(1, async move {
    println!("task local value: {}", NUMBER.get());
}).await;
```

**Methods**

- `pub fn take_value(self: Pin<&mut Self>) -> Option<T>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`
- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`
