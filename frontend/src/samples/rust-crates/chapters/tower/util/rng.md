# Module tower::util::rng

## Overview

PRNG utilities for tower middleware.

This module provides a generic `Rng` trait and a `HasherRng` that
implements the trait based on `RandomState` or any other `Hasher`.

These utilities replace tower’s internal usage of `rand` with these smaller,
more lightweight methods. Most of the implementations are extracted from
their corresponding `rand` implementations.

## Items

- `HasherRng` — A Rng implementation that uses a Hasher to generate the random values. The implementation uses an internal counter to pass to the hasher for each iteration of Rng::next_u64.
- `Rng` — A simple PRNG trait for use within tower middleware.

### struct tower::util::rng::HasherRng

```rust
pub struct HasherRng<H = RandomState> { /* private fields */ }
```

A `Rng` implementation that uses a `Hasher` to generate the random
values. The implementation uses an internal counter to pass to the hasher
for each iteration of `Rng::next_u64`.

**Default**

This hasher has a default type of `RandomState` which just uses the
libstd method of getting a random u64.

**Methods**

- `pub fn new() -> Self`
- `pub fn with_hasher(hasher: H) -> Self`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn next_u64(&mut self) -> u64`
- `fn next_f64(&mut self) -> f64`
- `fn next_range(&mut self, range: Range<u64>) -> u64`

### trait tower::util::rng::Rng

```rust
pub trait Rng {
    // Required method
    fn next_u64(&mut self) -> u64;

    // Provided methods
    fn next_f64(&mut self) -> f64 { ... }
    fn next_range(&mut self, range: Range<u64>) -> u64 { ... }
}
```

A simple PRNG trait for use within tower middleware.

**Methods**

- `fn next_u64(&mut self) -> u64`
- `fn next_f64(&mut self) -> f64`
- `fn next_range(&mut self, range: Range<u64>) -> u64`
- `fn next_u64(&mut self) -> u64`
