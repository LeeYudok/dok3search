# Module axum::response::sse

## Overview

Server-Sent Events (SSE) responses.

**Example**

```rust
use axum::{
    Router,
    routing::get,
    response::sse::{Event, KeepAlive, Sse},
};
use std::{time::Duration, convert::Infallible};
use tokio_stream::StreamExt as _ ;
use futures_util::stream::{self, Stream};

let app = Router::new().route("/sse", get(sse_handler));

async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // A `Stream` that repeats an event every second
    let stream = stream::repeat_with(|| Event::default().data("hi!"))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(KeepAlive::default())
}
```

## Items

- `Event` — Server-sent event
- `EventDataWriter` — Expose Event as a std::fmt::Write such that any form of data can be written as data safely.
- `KeepAlive` — Configure the interval between keep-alive messages, the content of each message, and the associated stream.
- `KeepAliveStream` — A wrapper around a stream that produces keep-alive events
- `Sse` — An SSE response

### struct axum::response::sse::Event

```rust
pub struct Event { /* private fields */ }
```

Server-sent event

**Methods**

- `pub fn into_data_writer(self) -> EventDataWriter`
- `pub fn data<T>(self, data: T) -> Selfwhere T: AsRef<str>,`
- `pub fn json_data<T>(self, data: T) -> Result<Self, Error>where T: Serialize,`
- `pub fn comment<T>(self, comment: T) -> Eventwhere T: AsRef<str>,`
- `pub fn event<T>(self, event: T) -> Eventwhere T: AsRef<str>,`
- `pub fn retry(self, duration: Duration) -> Event`
- `pub fn id<T>(self, id: T) -> Eventwhere T: AsRef<str>,`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`

### struct axum::response::sse::EventDataWriter

```rust
pub struct EventDataWriter { /* private fields */ }
```

Expose `Event` as a `std::fmt::Write`
such that any form of data can be written as data safely.

This also ensures that newline characters `\r` and `\n`
correctly trigger a split with a new `data: ` prefix.

**Panics**

Panics if any `data` has already been written prior to the first write
of this `EventDataWriter` instance.

**Methods**

- `pub fn into_event(self) -> Event`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn write_str(&mut self, s: &str) -> Result`
- `1.1.0 · fn write_char(&mut self, c: char) -> Result<(), Error>`
- `1.0.0 · fn write_fmt(&mut self, args: Arguments<'_>) -> Result<(), Error>`

### struct axum::response::sse::KeepAlive

```rust
pub struct KeepAlive { /* private fields */ }
```

Configure the interval between keep-alive messages, the content
of each message, and the associated stream.

**Methods**

- `pub fn new() -> Self`
- `pub fn interval(self, time: Duration) -> Self`
- `pub fn text<I>(self, text: I) -> Selfwhere I: AsRef<str>,`
- `pub fn event(self, event: Event) -> Self`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`

### struct axum::response::sse::KeepAliveStream

```rust
pub struct KeepAliveStream<S> { /* private fields */ }
```

A wrapper around a stream that produces keep-alive events

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn poll_next( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Option<Self::Item>>`
- `fn size_hint(&self) -> (usize, Option<usize>)`

### struct axum::response::sse::Sse

```rust
pub struct Sse<S> { /* private fields */ }
```

An SSE response

**Methods**

- `pub fn new(stream: S) -> Selfwhere S: TryStream<Ok = Event> + Send + 'static, S::Error: Into<BoxError>,`
- `pub fn keep_alive(self, keep_alive: KeepAlive) -> Sse<KeepAliveStream<S>>`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn into_response(self) -> Response`
