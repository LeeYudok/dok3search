# Module serde_json::map

## Overview

A map of String to serde_json::Value.

By default the map is backed by a `BTreeMap`. Enable the `preserve_order`
feature of serde_json to use `IndexMap` instead.

In builds without the `std` feature, `preserve_order` hashes keys with
[`foldhash::fast::RandomState`]. Foldhash provides minimal HashDoS
resistance, but does not protect against interactive attackers. Builds with
`std` use the standard library’s `std::hash::RandomState`.

## Items

- `IntoIter` — An owning iterator over a serde_json::Map’s entries.
- `IntoValues` — An owning iterator over a serde_json::Map’s values.
- `Iter` — An iterator over a serde_json::Map’s entries.
- `IterMut` — A mutable iterator over a serde_json::Map’s entries.
- `Keys` — An iterator over a serde_json::Map’s keys.
- `Map` — Represents a JSON key/value type.
- `OccupiedEntry` — An occupied Entry. It is part of the Entry enum.
- `VacantEntry` — A vacant Entry. It is part of the Entry enum.
- `Values` — An iterator over a serde_json::Map’s values.
- `ValuesMut` — A mutable iterator over a serde_json::Map’s values.
- `Entry` — A view into a single entry in a map, which may either be vacant or occupied. This enum is constructed from the entry method on Map.

### enum serde_json::map::Entry

```rust
pub enum Entry<'a> {
    Vacant(VacantEntry<'a>),
    Occupied(OccupiedEntry<'a>),
}
```

A view into a single entry in a map, which may either be vacant or occupied.
This enum is constructed from the `entry` method on `Map`.

**Methods**

- `pub fn key(&self) -> &String`
- `pub fn or_insert(self, default: Value) -> &'a mut Value`
- `pub fn or_insert_with<F>(self, default: F) -> &'a mut Valuewhere F: FnOnce() -> Value,`
- `pub fn and_modify<F>(self, f: F) -> Selfwhere F: FnOnce(&mut Value),`

### struct serde_json::map::IntoIter

```rust
pub struct IntoIter { /* private fields */ }
```

An owning iterator over a serde_json::Map’s entries.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn next_back(&mut self) -> Option<Self::Item>`
- `fn next_chunk_back<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,`
- `fn advance_back_by(&mut self, n: usize) -> Result<(), NonZero<usize>>`
- `1.37.0 (const: unstable) · fn nth_back(&mut self, n: usize) -> Option<Self::Item>`
- `1.27.0 (const: unstable) · fn try_rfold<B, F, R>(&mut self, init: B, f: F) -> Rwhere Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Output = B>,`
- `1.27.0 (const: unstable) · fn rfold<B, F>(self, init: B, f: F) -> Bwhere Self: Sized, F: FnMut(B, Self::Item) -> B,`
- `1.27.0 (const: unstable) · fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>where Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `fn len(&self) -> usize`
- `fn is_empty(&self) -> bool`
- `fn next(&mut self) -> Option<Self::Item>`
- `fn size_hint(&self) -> (usize, Option<usize>)`
- `fn next_chunk<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,`
- `1.0.0 (const: unstable) · fn count(self) -> usizewhere Self: Sized,`
- `1.0.0 (const: unstable) · fn last(self) -> Option<Self::Item>where Self: Sized,`
- `fn advance_by(&mut self, n: usize) -> Result<(), NonZero<usize>>`
- `1.0.0 (const: unstable) · fn nth(&mut self, n: usize) -> Option<Self::Item>`
- `1.28.0 (const: unstable) · fn step_by(self, step: usize) -> StepBy<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn chain<U>(self, other: U) -> Chain<Self, <U as IntoIterator>::IntoIter> ⓘwhere Self: Sized, U: IntoIterator<Item = Self::Item>,`
- `1.0.0 (const: unstable) · fn zip<U>(self, other: U) -> Zip<Self, <U as IntoIterator>::IntoIter> ⓘwhere Self: Sized, U: IntoIterator,`
- `fn intersperse(self, separator: Self::Item) -> Intersperse<Self> ⓘwhere Self: Sized, Self::Item: Clone,`
- `fn intersperse_with<G>(self, separator: G) -> IntersperseWith<Self, G> ⓘwhere Self: Sized, G: FnMut() -> Self::Item,`
- `1.0.0 (const: unstable) · fn map<B, F>(self, f: F) -> Map<Self, F> ⓘwhere Self: Sized, F: FnMut(Self::Item) -> B,`
- `1.21.0 (const: unstable) · fn for_each<F>(self, f: F)where Self: Sized, F: FnMut(Self::Item),`
- `1.0.0 (const: unstable) · fn filter<P>(self, predicate: P) -> Filter<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.0.0 (const: unstable) · fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F> ⓘwhere Self: Sized, F: FnMut(Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn enumerate(self) -> Enumerate<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn peekable(self) -> Peekable<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.0.0 (const: unstable) · fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.57.0 (const: unstable) · fn map_while<B, P>(self, predicate: P) -> MapWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn skip(self, n: usize) -> Skip<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn take(self, n: usize) -> Take<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn scan<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F> ⓘwhere Self: Sized, F: FnMut(&mut St, Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F> ⓘwhere Self: Sized, U: IntoIterator, F: FnMut(Self::Item) -> U,`
- `1.29.0 (const: unstable) · fn flatten(self) -> Flatten<Self> ⓘwhere Self: Sized, Self::Item: IntoIterator,`
- `fn map_windows<F, R, const N: usize>(self, f: F) -> MapWindows<Self, F, N> ⓘwhere Self: Sized, F: FnMut(&[Self::Item; N]) -> R,`
- `1.0.0 (const: unstable) · fn fuse(self) -> Fuse<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn inspect<F>(self, f: F) -> Inspect<Self, F> ⓘwhere Self: Sized, F: FnMut(&Self::Item),`
- `1.0.0 (const: unstable) · fn by_ref(&mut self) -> &mut Selfwhere Self: Sized,`

### struct serde_json::map::IntoValues

```rust
pub struct IntoValues { /* private fields */ }
```

An owning iterator over a serde_json::Map’s values.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn next_back(&mut self) -> Option<Self::Item>`
- `fn next_chunk_back<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,`
- `fn advance_back_by(&mut self, n: usize) -> Result<(), NonZero<usize>>`
- `1.37.0 (const: unstable) · fn nth_back(&mut self, n: usize) -> Option<Self::Item>`
- `1.27.0 (const: unstable) · fn try_rfold<B, F, R>(&mut self, init: B, f: F) -> Rwhere Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Output = B>,`
- `1.27.0 (const: unstable) · fn rfold<B, F>(self, init: B, f: F) -> Bwhere Self: Sized, F: FnMut(B, Self::Item) -> B,`
- `1.27.0 (const: unstable) · fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>where Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `fn len(&self) -> usize`
- `fn is_empty(&self) -> bool`
- `fn next(&mut self) -> Option<Self::Item>`
- `fn size_hint(&self) -> (usize, Option<usize>)`
- `fn next_chunk<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,`
- `1.0.0 (const: unstable) · fn count(self) -> usizewhere Self: Sized,`
- `1.0.0 (const: unstable) · fn last(self) -> Option<Self::Item>where Self: Sized,`
- `fn advance_by(&mut self, n: usize) -> Result<(), NonZero<usize>>`
- `1.0.0 (const: unstable) · fn nth(&mut self, n: usize) -> Option<Self::Item>`
- `1.28.0 (const: unstable) · fn step_by(self, step: usize) -> StepBy<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn chain<U>(self, other: U) -> Chain<Self, <U as IntoIterator>::IntoIter> ⓘwhere Self: Sized, U: IntoIterator<Item = Self::Item>,`
- `1.0.0 (const: unstable) · fn zip<U>(self, other: U) -> Zip<Self, <U as IntoIterator>::IntoIter> ⓘwhere Self: Sized, U: IntoIterator,`
- `fn intersperse(self, separator: Self::Item) -> Intersperse<Self> ⓘwhere Self: Sized, Self::Item: Clone,`
- `fn intersperse_with<G>(self, separator: G) -> IntersperseWith<Self, G> ⓘwhere Self: Sized, G: FnMut() -> Self::Item,`
- `1.0.0 (const: unstable) · fn map<B, F>(self, f: F) -> Map<Self, F> ⓘwhere Self: Sized, F: FnMut(Self::Item) -> B,`
- `1.21.0 (const: unstable) · fn for_each<F>(self, f: F)where Self: Sized, F: FnMut(Self::Item),`
- `1.0.0 (const: unstable) · fn filter<P>(self, predicate: P) -> Filter<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.0.0 (const: unstable) · fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F> ⓘwhere Self: Sized, F: FnMut(Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn enumerate(self) -> Enumerate<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn peekable(self) -> Peekable<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.0.0 (const: unstable) · fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.57.0 (const: unstable) · fn map_while<B, P>(self, predicate: P) -> MapWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn skip(self, n: usize) -> Skip<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn take(self, n: usize) -> Take<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn scan<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F> ⓘwhere Self: Sized, F: FnMut(&mut St, Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F> ⓘwhere Self: Sized, U: IntoIterator, F: FnMut(Self::Item) -> U,`
- `fn map_windows<F, R, const N: usize>(self, f: F) -> MapWindows<Self, F, N> ⓘwhere Self: Sized, F: FnMut(&[Self::Item; N]) -> R,`
- `1.0.0 (const: unstable) · fn fuse(self) -> Fuse<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn inspect<F>(self, f: F) -> Inspect<Self, F> ⓘwhere Self: Sized, F: FnMut(&Self::Item),`
- `1.0.0 (const: unstable) · fn by_ref(&mut self) -> &mut Selfwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn collect<B>(self) -> Bwhere B: FromIterator<Self::Item>, Self: Sized,`

### struct serde_json::map::Iter

```rust
pub struct Iter<'a> { /* private fields */ }
```

An iterator over a serde_json::Map’s entries.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn next_back(&mut self) -> Option<Self::Item>`
- `fn next_chunk_back<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,`
- `fn advance_back_by(&mut self, n: usize) -> Result<(), NonZero<usize>>`
- `1.37.0 (const: unstable) · fn nth_back(&mut self, n: usize) -> Option<Self::Item>`
- `1.27.0 (const: unstable) · fn try_rfold<B, F, R>(&mut self, init: B, f: F) -> Rwhere Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Output = B>,`
- `1.27.0 (const: unstable) · fn rfold<B, F>(self, init: B, f: F) -> Bwhere Self: Sized, F: FnMut(B, Self::Item) -> B,`
- `1.27.0 (const: unstable) · fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>where Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `fn len(&self) -> usize`
- `fn is_empty(&self) -> bool`
- `fn next(&mut self) -> Option<Self::Item>`
- `fn size_hint(&self) -> (usize, Option<usize>)`
- `fn next_chunk<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,`
- `1.0.0 (const: unstable) · fn count(self) -> usizewhere Self: Sized,`
- `1.0.0 (const: unstable) · fn last(self) -> Option<Self::Item>where Self: Sized,`
- `fn advance_by(&mut self, n: usize) -> Result<(), NonZero<usize>>`
- `1.0.0 (const: unstable) · fn nth(&mut self, n: usize) -> Option<Self::Item>`
- `1.28.0 (const: unstable) · fn step_by(self, step: usize) -> StepBy<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn chain<U>(self, other: U) -> Chain<Self, <U as IntoIterator>::IntoIter> ⓘwhere Self: Sized, U: IntoIterator<Item = Self::Item>,`
- `1.0.0 (const: unstable) · fn zip<U>(self, other: U) -> Zip<Self, <U as IntoIterator>::IntoIter> ⓘwhere Self: Sized, U: IntoIterator,`
- `fn intersperse(self, separator: Self::Item) -> Intersperse<Self> ⓘwhere Self: Sized, Self::Item: Clone,`
- `fn intersperse_with<G>(self, separator: G) -> IntersperseWith<Self, G> ⓘwhere Self: Sized, G: FnMut() -> Self::Item,`
- `1.0.0 (const: unstable) · fn map<B, F>(self, f: F) -> Map<Self, F> ⓘwhere Self: Sized, F: FnMut(Self::Item) -> B,`
- `1.21.0 (const: unstable) · fn for_each<F>(self, f: F)where Self: Sized, F: FnMut(Self::Item),`
- `1.0.0 (const: unstable) · fn filter<P>(self, predicate: P) -> Filter<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.0.0 (const: unstable) · fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F> ⓘwhere Self: Sized, F: FnMut(Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn enumerate(self) -> Enumerate<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn peekable(self) -> Peekable<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.0.0 (const: unstable) · fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.57.0 (const: unstable) · fn map_while<B, P>(self, predicate: P) -> MapWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn skip(self, n: usize) -> Skip<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn take(self, n: usize) -> Take<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn scan<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F> ⓘwhere Self: Sized, F: FnMut(&mut St, Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F> ⓘwhere Self: Sized, U: IntoIterator, F: FnMut(Self::Item) -> U,`
- `1.29.0 (const: unstable) · fn flatten(self) -> Flatten<Self> ⓘwhere Self: Sized, Self::Item: IntoIterator,`
- `fn map_windows<F, R, const N: usize>(self, f: F) -> MapWindows<Self, F, N> ⓘwhere Self: Sized, F: FnMut(&[Self::Item; N]) -> R,`
- `1.0.0 (const: unstable) · fn fuse(self) -> Fuse<Self> ⓘwhere Self: Sized,`

### struct serde_json::map::IterMut

```rust
pub struct IterMut<'a> { /* private fields */ }
```

A mutable iterator over a serde_json::Map’s entries.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn next_back(&mut self) -> Option<Self::Item>`
- `fn next_chunk_back<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,`
- `fn advance_back_by(&mut self, n: usize) -> Result<(), NonZero<usize>>`
- `1.37.0 (const: unstable) · fn nth_back(&mut self, n: usize) -> Option<Self::Item>`
- `1.27.0 (const: unstable) · fn try_rfold<B, F, R>(&mut self, init: B, f: F) -> Rwhere Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Output = B>,`
- `1.27.0 (const: unstable) · fn rfold<B, F>(self, init: B, f: F) -> Bwhere Self: Sized, F: FnMut(B, Self::Item) -> B,`
- `1.27.0 (const: unstable) · fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>where Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `fn len(&self) -> usize`
- `fn is_empty(&self) -> bool`
- `fn next(&mut self) -> Option<Self::Item>`
- `fn size_hint(&self) -> (usize, Option<usize>)`
- `fn next_chunk<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,`
- `1.0.0 (const: unstable) · fn count(self) -> usizewhere Self: Sized,`
- `1.0.0 (const: unstable) · fn last(self) -> Option<Self::Item>where Self: Sized,`
- `fn advance_by(&mut self, n: usize) -> Result<(), NonZero<usize>>`
- `1.0.0 (const: unstable) · fn nth(&mut self, n: usize) -> Option<Self::Item>`
- `1.28.0 (const: unstable) · fn step_by(self, step: usize) -> StepBy<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn chain<U>(self, other: U) -> Chain<Self, <U as IntoIterator>::IntoIter> ⓘwhere Self: Sized, U: IntoIterator<Item = Self::Item>,`
- `1.0.0 (const: unstable) · fn zip<U>(self, other: U) -> Zip<Self, <U as IntoIterator>::IntoIter> ⓘwhere Self: Sized, U: IntoIterator,`
- `fn intersperse(self, separator: Self::Item) -> Intersperse<Self> ⓘwhere Self: Sized, Self::Item: Clone,`
- `fn intersperse_with<G>(self, separator: G) -> IntersperseWith<Self, G> ⓘwhere Self: Sized, G: FnMut() -> Self::Item,`
- `1.0.0 (const: unstable) · fn map<B, F>(self, f: F) -> Map<Self, F> ⓘwhere Self: Sized, F: FnMut(Self::Item) -> B,`
- `1.21.0 (const: unstable) · fn for_each<F>(self, f: F)where Self: Sized, F: FnMut(Self::Item),`
- `1.0.0 (const: unstable) · fn filter<P>(self, predicate: P) -> Filter<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.0.0 (const: unstable) · fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F> ⓘwhere Self: Sized, F: FnMut(Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn enumerate(self) -> Enumerate<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn peekable(self) -> Peekable<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.0.0 (const: unstable) · fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.57.0 (const: unstable) · fn map_while<B, P>(self, predicate: P) -> MapWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn skip(self, n: usize) -> Skip<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn take(self, n: usize) -> Take<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn scan<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F> ⓘwhere Self: Sized, F: FnMut(&mut St, Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F> ⓘwhere Self: Sized, U: IntoIterator, F: FnMut(Self::Item) -> U,`
- `1.29.0 (const: unstable) · fn flatten(self) -> Flatten<Self> ⓘwhere Self: Sized, Self::Item: IntoIterator,`
- `fn map_windows<F, R, const N: usize>(self, f: F) -> MapWindows<Self, F, N> ⓘwhere Self: Sized, F: FnMut(&[Self::Item; N]) -> R,`
- `1.0.0 (const: unstable) · fn fuse(self) -> Fuse<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn inspect<F>(self, f: F) -> Inspect<Self, F> ⓘwhere Self: Sized, F: FnMut(&Self::Item),`
- `1.0.0 (const: unstable) · fn by_ref(&mut self) -> &mut Selfwhere Self: Sized,`

### struct serde_json::map::Keys

```rust
pub struct Keys<'a> { /* private fields */ }
```

An iterator over a serde_json::Map’s keys.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn next_back(&mut self) -> Option<Self::Item>`
- `fn next_chunk_back<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,`
- `fn advance_back_by(&mut self, n: usize) -> Result<(), NonZero<usize>>`
- `1.37.0 (const: unstable) · fn nth_back(&mut self, n: usize) -> Option<Self::Item>`
- `1.27.0 (const: unstable) · fn try_rfold<B, F, R>(&mut self, init: B, f: F) -> Rwhere Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Output = B>,`
- `1.27.0 (const: unstable) · fn rfold<B, F>(self, init: B, f: F) -> Bwhere Self: Sized, F: FnMut(B, Self::Item) -> B,`
- `1.27.0 (const: unstable) · fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>where Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `fn len(&self) -> usize`
- `fn is_empty(&self) -> bool`
- `fn next(&mut self) -> Option<Self::Item>`
- `fn size_hint(&self) -> (usize, Option<usize>)`
- `fn next_chunk<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,`
- `1.0.0 (const: unstable) · fn count(self) -> usizewhere Self: Sized,`
- `1.0.0 (const: unstable) · fn last(self) -> Option<Self::Item>where Self: Sized,`
- `fn advance_by(&mut self, n: usize) -> Result<(), NonZero<usize>>`
- `1.0.0 (const: unstable) · fn nth(&mut self, n: usize) -> Option<Self::Item>`
- `1.28.0 (const: unstable) · fn step_by(self, step: usize) -> StepBy<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn chain<U>(self, other: U) -> Chain<Self, <U as IntoIterator>::IntoIter> ⓘwhere Self: Sized, U: IntoIterator<Item = Self::Item>,`
- `1.0.0 (const: unstable) · fn zip<U>(self, other: U) -> Zip<Self, <U as IntoIterator>::IntoIter> ⓘwhere Self: Sized, U: IntoIterator,`
- `fn intersperse(self, separator: Self::Item) -> Intersperse<Self> ⓘwhere Self: Sized, Self::Item: Clone,`
- `fn intersperse_with<G>(self, separator: G) -> IntersperseWith<Self, G> ⓘwhere Self: Sized, G: FnMut() -> Self::Item,`
- `1.0.0 (const: unstable) · fn map<B, F>(self, f: F) -> Map<Self, F> ⓘwhere Self: Sized, F: FnMut(Self::Item) -> B,`
- `1.21.0 (const: unstable) · fn for_each<F>(self, f: F)where Self: Sized, F: FnMut(Self::Item),`
- `1.0.0 (const: unstable) · fn filter<P>(self, predicate: P) -> Filter<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.0.0 (const: unstable) · fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F> ⓘwhere Self: Sized, F: FnMut(Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn enumerate(self) -> Enumerate<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn peekable(self) -> Peekable<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.0.0 (const: unstable) · fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.57.0 (const: unstable) · fn map_while<B, P>(self, predicate: P) -> MapWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn skip(self, n: usize) -> Skip<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn take(self, n: usize) -> Take<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn scan<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F> ⓘwhere Self: Sized, F: FnMut(&mut St, Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F> ⓘwhere Self: Sized, U: IntoIterator, F: FnMut(Self::Item) -> U,`
- `1.29.0 (const: unstable) · fn flatten(self) -> Flatten<Self> ⓘwhere Self: Sized, Self::Item: IntoIterator,`
- `fn map_windows<F, R, const N: usize>(self, f: F) -> MapWindows<Self, F, N> ⓘwhere Self: Sized, F: FnMut(&[Self::Item; N]) -> R,`
- `1.0.0 (const: unstable) · fn fuse(self) -> Fuse<Self> ⓘwhere Self: Sized,`

### struct serde_json::map::Map

```rust
pub struct Map<K, V> { /* private fields */ }
```

Represents a JSON key/value type.

**Methods**

- `pub fn new() -> Self`
- `pub fn with_capacity(capacity: usize) -> Self`
- `pub fn clear(&mut self)`
- `pub fn get<Q>(&self, key: &Q) -> Option<&Value>where String: Borrow<Q>, Q: ?Sized + Ord + Eq + Hash,`
- `pub fn contains_key<Q>(&self, key: &Q) -> boolwhere String: Borrow<Q>, Q: ?Sized + Ord + Eq + Hash,`
- `pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut Value>where String: Borrow<Q>, Q: ?Sized + Ord + Eq + Hash,`
- `pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&String, &Value)>where String: Borrow<Q>, Q: ?Sized + Ord + Eq + Hash,`
- `pub fn insert(&mut self, k: String, v: Value) -> Option<Value>`
- `pub fn remove<Q>(&mut self, key: &Q) -> Option<Value>where String: Borrow<Q>, Q: ?Sized + Ord + Eq + Hash,`
- `pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(String, Value)>where String: Borrow<Q>, Q: ?Sized + Ord + Eq + Hash,`
- `pub fn append(&mut self, other: &mut Self)`
- `pub fn entry<S>(&mut self, key: S) -> Entry<'_>where S: Into<String>,`
- `pub fn len(&self) -> usize`
- `pub fn is_empty(&self) -> bool`
- `pub fn iter(&self) -> Iter<'_>`
- `pub fn iter_mut(&mut self) -> IterMut<'_>`
- `pub fn keys(&self) -> Keys<'_>`
- `pub fn values(&self) -> Values<'_>`
- `pub fn values_mut(&mut self) -> ValuesMut<'_>`
- `pub fn into_values(self) -> IntoValues`
- `pub fn retain<F>(&mut self, f: F)where F: FnMut(&String, &mut Value) -> bool,`
- `pub fn sort_keys(&mut self)`
- `fn clone(&self) -> Self`
- `fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`
- `fn default() -> Self`
- `fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>where D: Deserializer<'de>,`
- `fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_enum<V>( self, _name: &'static str, _variants: &'static [&'static str], visitor: V, ) -> Result<V::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>where V: Visitor<'de>,`
- `fn deserialize_bool<V>( self, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i8<V>( self, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i16<V>( self, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i32<V>( self, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i64<V>( self, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_i128<V>( self, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u8<V>( self, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u16<V>( self, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u32<V>( self, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`
- `fn deserialize_u64<V>( self, visitor: V, ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>where V: Visitor<'de>,`

### struct serde_json::map::OccupiedEntry

```rust
pub struct OccupiedEntry<'a> { /* private fields */ }
```

An occupied Entry. It is part of the `Entry` enum.

**Methods**

- `pub fn key(&self) -> &String`
- `pub fn get(&self) -> &Value`
- `pub fn get_mut(&mut self) -> &mut Value`
- `pub fn into_mut(self) -> &'a mut Value`
- `pub fn insert(&mut self, value: Value) -> Value`
- `pub fn remove(self) -> Value`
- `pub fn remove_entry(self) -> (String, Value)`

### struct serde_json::map::VacantEntry

```rust
pub struct VacantEntry<'a> { /* private fields */ }
```

A vacant Entry. It is part of the `Entry` enum.

**Methods**

- `pub fn key(&self) -> &String`
- `pub fn insert(self, value: Value) -> &'a mut Value`

### struct serde_json::map::Values

```rust
pub struct Values<'a> { /* private fields */ }
```

An iterator over a serde_json::Map’s values.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn next_back(&mut self) -> Option<Self::Item>`
- `fn next_chunk_back<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,`
- `fn advance_back_by(&mut self, n: usize) -> Result<(), NonZero<usize>>`
- `1.37.0 (const: unstable) · fn nth_back(&mut self, n: usize) -> Option<Self::Item>`
- `1.27.0 (const: unstable) · fn try_rfold<B, F, R>(&mut self, init: B, f: F) -> Rwhere Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Output = B>,`
- `1.27.0 (const: unstable) · fn rfold<B, F>(self, init: B, f: F) -> Bwhere Self: Sized, F: FnMut(B, Self::Item) -> B,`
- `1.27.0 (const: unstable) · fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>where Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `fn len(&self) -> usize`
- `fn is_empty(&self) -> bool`
- `fn next(&mut self) -> Option<Self::Item>`
- `fn size_hint(&self) -> (usize, Option<usize>)`
- `fn next_chunk<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,`
- `1.0.0 (const: unstable) · fn count(self) -> usizewhere Self: Sized,`
- `1.0.0 (const: unstable) · fn last(self) -> Option<Self::Item>where Self: Sized,`
- `fn advance_by(&mut self, n: usize) -> Result<(), NonZero<usize>>`
- `1.0.0 (const: unstable) · fn nth(&mut self, n: usize) -> Option<Self::Item>`
- `1.28.0 (const: unstable) · fn step_by(self, step: usize) -> StepBy<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn chain<U>(self, other: U) -> Chain<Self, <U as IntoIterator>::IntoIter> ⓘwhere Self: Sized, U: IntoIterator<Item = Self::Item>,`
- `1.0.0 (const: unstable) · fn zip<U>(self, other: U) -> Zip<Self, <U as IntoIterator>::IntoIter> ⓘwhere Self: Sized, U: IntoIterator,`
- `fn intersperse(self, separator: Self::Item) -> Intersperse<Self> ⓘwhere Self: Sized, Self::Item: Clone,`
- `fn intersperse_with<G>(self, separator: G) -> IntersperseWith<Self, G> ⓘwhere Self: Sized, G: FnMut() -> Self::Item,`
- `1.0.0 (const: unstable) · fn map<B, F>(self, f: F) -> Map<Self, F> ⓘwhere Self: Sized, F: FnMut(Self::Item) -> B,`
- `1.21.0 (const: unstable) · fn for_each<F>(self, f: F)where Self: Sized, F: FnMut(Self::Item),`
- `1.0.0 (const: unstable) · fn filter<P>(self, predicate: P) -> Filter<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.0.0 (const: unstable) · fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F> ⓘwhere Self: Sized, F: FnMut(Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn enumerate(self) -> Enumerate<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn peekable(self) -> Peekable<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.0.0 (const: unstable) · fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.57.0 (const: unstable) · fn map_while<B, P>(self, predicate: P) -> MapWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn skip(self, n: usize) -> Skip<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn take(self, n: usize) -> Take<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn scan<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F> ⓘwhere Self: Sized, F: FnMut(&mut St, Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F> ⓘwhere Self: Sized, U: IntoIterator, F: FnMut(Self::Item) -> U,`
- `fn map_windows<F, R, const N: usize>(self, f: F) -> MapWindows<Self, F, N> ⓘwhere Self: Sized, F: FnMut(&[Self::Item; N]) -> R,`
- `1.0.0 (const: unstable) · fn fuse(self) -> Fuse<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn inspect<F>(self, f: F) -> Inspect<Self, F> ⓘwhere Self: Sized, F: FnMut(&Self::Item),`

### struct serde_json::map::ValuesMut

```rust
pub struct ValuesMut<'a> { /* private fields */ }
```

A mutable iterator over a serde_json::Map’s values.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn next_back(&mut self) -> Option<Self::Item>`
- `fn next_chunk_back<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,`
- `fn advance_back_by(&mut self, n: usize) -> Result<(), NonZero<usize>>`
- `1.37.0 (const: unstable) · fn nth_back(&mut self, n: usize) -> Option<Self::Item>`
- `1.27.0 (const: unstable) · fn try_rfold<B, F, R>(&mut self, init: B, f: F) -> Rwhere Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Output = B>,`
- `1.27.0 (const: unstable) · fn rfold<B, F>(self, init: B, f: F) -> Bwhere Self: Sized, F: FnMut(B, Self::Item) -> B,`
- `1.27.0 (const: unstable) · fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>where Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `fn len(&self) -> usize`
- `fn is_empty(&self) -> bool`
- `fn next(&mut self) -> Option<Self::Item>`
- `fn size_hint(&self) -> (usize, Option<usize>)`
- `fn next_chunk<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,`
- `1.0.0 (const: unstable) · fn count(self) -> usizewhere Self: Sized,`
- `1.0.0 (const: unstable) · fn last(self) -> Option<Self::Item>where Self: Sized,`
- `fn advance_by(&mut self, n: usize) -> Result<(), NonZero<usize>>`
- `1.0.0 (const: unstable) · fn nth(&mut self, n: usize) -> Option<Self::Item>`
- `1.28.0 (const: unstable) · fn step_by(self, step: usize) -> StepBy<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn chain<U>(self, other: U) -> Chain<Self, <U as IntoIterator>::IntoIter> ⓘwhere Self: Sized, U: IntoIterator<Item = Self::Item>,`
- `1.0.0 (const: unstable) · fn zip<U>(self, other: U) -> Zip<Self, <U as IntoIterator>::IntoIter> ⓘwhere Self: Sized, U: IntoIterator,`
- `fn intersperse_with<G>(self, separator: G) -> IntersperseWith<Self, G> ⓘwhere Self: Sized, G: FnMut() -> Self::Item,`
- `1.0.0 (const: unstable) · fn map<B, F>(self, f: F) -> Map<Self, F> ⓘwhere Self: Sized, F: FnMut(Self::Item) -> B,`
- `1.21.0 (const: unstable) · fn for_each<F>(self, f: F)where Self: Sized, F: FnMut(Self::Item),`
- `1.0.0 (const: unstable) · fn filter<P>(self, predicate: P) -> Filter<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.0.0 (const: unstable) · fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F> ⓘwhere Self: Sized, F: FnMut(Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn enumerate(self) -> Enumerate<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn peekable(self) -> Peekable<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.0.0 (const: unstable) · fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.57.0 (const: unstable) · fn map_while<B, P>(self, predicate: P) -> MapWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn skip(self, n: usize) -> Skip<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn take(self, n: usize) -> Take<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn scan<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F> ⓘwhere Self: Sized, F: FnMut(&mut St, Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F> ⓘwhere Self: Sized, U: IntoIterator, F: FnMut(Self::Item) -> U,`
- `fn map_windows<F, R, const N: usize>(self, f: F) -> MapWindows<Self, F, N> ⓘwhere Self: Sized, F: FnMut(&[Self::Item; N]) -> R,`
- `1.0.0 (const: unstable) · fn fuse(self) -> Fuse<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn inspect<F>(self, f: F) -> Inspect<Self, F> ⓘwhere Self: Sized, F: FnMut(&Self::Item),`
- `1.0.0 (const: unstable) · fn by_ref(&mut self) -> &mut Selfwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn collect<B>(self) -> Bwhere B: FromIterator<Self::Item>, Self: Sized,`
- `fn collect_into<E>(self, collection: &mut E) -> &mut Ewhere E: Extend<Self::Item>, Self: Sized,`
