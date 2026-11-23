# Module tower::util::future

## Overview

Future types

## Items

- `optional` — Future types for Optional.
- `AndThenFuture` — Response future from AndThen services.
- `EitherResponseFuture` — Response future for Either.
- `MapErrFuture` — Response future from MapErr services.
- `MapResponseFuture` — Response future from MapResponse services.
- `MapResultFuture` — Response future from MapResult services.
- `ThenFuture` — Response future from Then services.

### struct tower::util::future::AndThenFuture

```rust
pub struct AndThenFuture<F1, F2: TryFuture, N> { /* private fields */ }
```

Response future from `AndThen` services.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`

### struct tower::util::future::EitherResponseFuture

```rust
pub struct EitherResponseFuture<A, B> { /* private fields */ }
```

Response future for `Either`.

**Methods**

- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`

### struct tower::util::future::MapErrFuture

```rust
pub struct MapErrFuture<F, N> { /* private fields */ }
```

Response future from `MapErr` services.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`

### struct tower::util::future::MapResponseFuture

```rust
pub struct MapResponseFuture<F, N> { /* private fields */ }
```

Response future from `MapResponse` services.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`

### struct tower::util::future::MapResultFuture

```rust
pub struct MapResultFuture<F, N> { /* private fields */ }
```

Response future from `MapResult` services.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`

### struct tower::util::future::ThenFuture

```rust
pub struct ThenFuture<F1, F2, N> { /* private fields */ }
```

Response future from `Then` services.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`
