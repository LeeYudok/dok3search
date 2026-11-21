# Await expressions

[expr.await](#r-expr.await)

# Await expressions

[expr.await.syntax](#r-expr.await.syntax)

**Syntax**\
[AwaitExpression](#railroad-AwaitExpression) → [Expression](../expressions.md#grammar-Expression) . await

Show Railroad

![](../../assets/inline/await-expr-1.svg) [expr.await.intro](#r-expr.await.intro)

An `await` expression is a syntactic construct for suspending a computation provided by an implementation of `std::future::IntoFuture` until the given future is ready to produce a value.

[expr.await.construct](#r-expr.await.construct)

The syntax for an await expression is an expression with a type that implements the [`IntoFuture`](../../core/future/into_future/trait.IntoFuture.html) trait, called the *future operand*, then the token `.`, and then the `await` keyword.

[expr.await.allowed-positions](#r-expr.await.allowed-positions)

Await expressions are legal only within an [async context](block-expr.md#async-context), like an [`async fn`](../items/functions.md#async-functions), [`async` closure](closure-expr.md#async-closures), or [`async` block](block-expr.md#async-blocks).

[expr.await.effects](#r-expr.await.effects)

More specifically, an await expression has the following effect.

1.  Create a future by calling [`IntoFuture::into_future`](../../core/future/into_future/trait.IntoFuture.html#tymethod.into_future) on the future operand.
2.  Evaluate the future to a [future](../../core/future/future/trait.Future.html) `tmp`;
3.  Pin `tmp` using [`Pin::new_unchecked`](../../core/pin/struct.Pin.html#method.new_unchecked);
4.  This pinned future is then polled by calling the [`Future::poll`](../../core/future/future/trait.Future.html#tymethod.poll) method and passing it the current [task context](#task-context);
5.  If the call to `poll` returns [`Poll::Pending`](../../core/task/poll/enum.Poll.html#variant.Pending), then the future returns `Poll::Pending`, suspending its state so that, when the surrounding async context is re-polled, execution returns to step 3;
6.  Otherwise the call to `poll` must have returned [`Poll::Ready`](../../core/task/poll/enum.Poll.html#variant.Ready), in which case the value contained in the [`Poll::Ready`](../../core/task/poll/enum.Poll.html#variant.Ready) variant is used as the result of the `await` expression itself.

[expr.await.edition2018](#r-expr.await.edition2018)

> 2018 Edition differences
>
> Await expressions are only available beginning with Rust 2018.

[expr.await.task](#r-expr.await.task)

## Task context

The task context refers to the [`Context`](../../core/task/wake/struct.Context.html) which was supplied to the current [async context](block-expr.md#async-context) when the async context itself was polled. Because `await` expressions are only legal in an async context, there must be some task context available.

[expr.await.desugar](#r-expr.await.desugar)

## Approximate desugaring

Effectively, an await expression is roughly equivalent to the following non-normative desugaring:

``` rust
match operand.into_future() {
    mut pinned => loop {
        let mut pin = unsafe { Pin::new_unchecked(&mut pinned) };
        match Pin::future::poll(Pin::borrow(&mut pin), &mut current_context) {
            Poll::Ready(r) => break r,
            Poll::Pending => yield Poll::Pending,
        }
    }
}
```

where the `yield` pseudo-code returns `Poll::Pending` and, when re-invoked, resumes execution from that point. The variable `current_context` refers to the context taken from the async environment.
