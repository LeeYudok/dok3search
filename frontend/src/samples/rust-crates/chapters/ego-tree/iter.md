# Module ego-tree::iter

## Overview

Iterators.

## Items

- `Ancestors` — Iterator over ancestors.
- `Children` — Iterator over children.
- `Descendants` — Iterator over a node and its descendants.
- `FirstChildren` — Iterator over first children.
- `IntoIter` — Iterator that moves out of a tree in insert order.
- `LastChildren` — Iterator over last children.
- `NextSiblings` — Iterator over next siblings.
- `Nodes` — Iterator over nodes in insert order.
- `PrevSiblings` — Iterator over previous siblings.
- `Traverse` — Iterator which traverses a subtree.
- `Values` — Iterator over values in insert order.
- `ValuesMut` — Mutable iterator over values in insert order.
- `Edge` — Open or close edge of a node.

### enum ego-tree::iter::Edge

```rust
pub enum Edge<'a, T: 'a> {
    Open(NodeRef<'a, T>),
    Close(NodeRef<'a, T>),
}
```

Open or close edge of a node.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### struct ego-tree::iter::Ancestors

```rust
pub struct Ancestors<'a, T: 'a>(/* private fields */);
```

Iterator over ancestors.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn next(&mut self) -> Option<Self::Item>`
- `fn next_chunk<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,`
- `1.0.0 (const: unstable) · fn size_hint(&self) -> (usize, Option<usize>)`
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
- `fn collect_into<E>(self, collection: &mut E) -> &mut Ewhere E: Extend<Self::Item>, Self: Sized,`
- `1.0.0 (const: unstable) · fn partition<B, F>(self, f: F) -> (B, B)where Self: Sized, B: Default + Extend<Self::Item>, F: FnMut(&Self::Item) -> bool,`
- `fn is_partitioned<P>(self, predicate: P) -> boolwhere Self: Sized, P: FnMut(Self::Item) -> bool,`
- `1.27.0 (const: unstable) · fn try_fold<B, F, R>(&mut self, init: B, f: F) -> Rwhere Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Output = B>,`
- `1.27.0 (const: unstable) · fn try_for_each<F, R>(&mut self, f: F) -> Rwhere Self: Sized, F: FnMut(Self::Item) -> R, R: Try<Output = ()>,`
- `1.0.0 (const: unstable) · fn fold<B, F>(self, init: B, f: F) -> Bwhere Self: Sized, F: FnMut(B, Self::Item) -> B,`
- `1.51.0 (const: unstable) · fn reduce<F>(self, f: F) -> Option<Self::Item>where Self: Sized, F: FnMut(Self::Item, Self::Item) -> Self::Item,`

### struct ego-tree::iter::Children

```rust
pub struct Children<'a, T: 'a> { /* private fields */ }
```

Iterator over children.

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
- `fn next(&mut self) -> Option<Self::Item>`
- `fn next_chunk<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,`
- `1.0.0 (const: unstable) · fn size_hint(&self) -> (usize, Option<usize>)`
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

### struct ego-tree::iter::Descendants

```rust
pub struct Descendants<'a, T: 'a>(/* private fields */);
```

Iterator over a node and its descendants.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn next(&mut self) -> Option<Self::Item>`
- `fn next_chunk<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,`
- `1.0.0 (const: unstable) · fn size_hint(&self) -> (usize, Option<usize>)`
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
- `fn collect_into<E>(self, collection: &mut E) -> &mut Ewhere E: Extend<Self::Item>, Self: Sized,`
- `1.0.0 (const: unstable) · fn partition<B, F>(self, f: F) -> (B, B)where Self: Sized, B: Default + Extend<Self::Item>, F: FnMut(&Self::Item) -> bool,`
- `fn is_partitioned<P>(self, predicate: P) -> boolwhere Self: Sized, P: FnMut(Self::Item) -> bool,`
- `1.27.0 (const: unstable) · fn try_fold<B, F, R>(&mut self, init: B, f: F) -> Rwhere Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Output = B>,`
- `1.27.0 (const: unstable) · fn try_for_each<F, R>(&mut self, f: F) -> Rwhere Self: Sized, F: FnMut(Self::Item) -> R, R: Try<Output = ()>,`
- `1.0.0 (const: unstable) · fn fold<B, F>(self, init: B, f: F) -> Bwhere Self: Sized, F: FnMut(B, Self::Item) -> B,`
- `1.51.0 (const: unstable) · fn reduce<F>(self, f: F) -> Option<Self::Item>where Self: Sized, F: FnMut(Self::Item, Self::Item) -> Self::Item,`

### struct ego-tree::iter::FirstChildren

```rust
pub struct FirstChildren<'a, T: 'a>(/* private fields */);
```

Iterator over first children.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn next(&mut self) -> Option<Self::Item>`
- `fn next_chunk<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,`
- `1.0.0 (const: unstable) · fn size_hint(&self) -> (usize, Option<usize>)`
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
- `fn collect_into<E>(self, collection: &mut E) -> &mut Ewhere E: Extend<Self::Item>, Self: Sized,`
- `1.0.0 (const: unstable) · fn partition<B, F>(self, f: F) -> (B, B)where Self: Sized, B: Default + Extend<Self::Item>, F: FnMut(&Self::Item) -> bool,`
- `fn is_partitioned<P>(self, predicate: P) -> boolwhere Self: Sized, P: FnMut(Self::Item) -> bool,`
- `1.27.0 (const: unstable) · fn try_fold<B, F, R>(&mut self, init: B, f: F) -> Rwhere Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Output = B>,`
- `1.27.0 (const: unstable) · fn try_for_each<F, R>(&mut self, f: F) -> Rwhere Self: Sized, F: FnMut(Self::Item) -> R, R: Try<Output = ()>,`
- `1.0.0 (const: unstable) · fn fold<B, F>(self, init: B, f: F) -> Bwhere Self: Sized, F: FnMut(B, Self::Item) -> B,`
- `1.51.0 (const: unstable) · fn reduce<F>(self, f: F) -> Option<Self::Item>where Self: Sized, F: FnMut(Self::Item, Self::Item) -> Self::Item,`

### struct ego-tree::iter::IntoIter

```rust
pub struct IntoIter<T>(/* private fields */);
```

Iterator that moves out of a tree in insert order.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn next_back(&mut self) -> Option<Self::Item>`
- `fn next_chunk_back<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,`
- `fn advance_back_by(&mut self, n: usize) -> Result<(), NonZero<usize>>`
- `1.37.0 (const: unstable) · fn nth_back(&mut self, n: usize) -> Option<Self::Item>`
- `1.27.0 (const: unstable) · fn try_rfold<B, F, R>(&mut self, init: B, f: F) -> Rwhere Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Output = B>,`
- `1.27.0 (const: unstable) · fn rfold<B, F>(self, init: B, f: F) -> Bwhere Self: Sized, F: FnMut(B, Self::Item) -> B,`
- `1.27.0 (const: unstable) · fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>where Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.0.0 · fn len(&self) -> usize`
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

### struct ego-tree::iter::LastChildren

```rust
pub struct LastChildren<'a, T: 'a>(/* private fields */);
```

Iterator over last children.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn next(&mut self) -> Option<Self::Item>`
- `fn next_chunk<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,`
- `1.0.0 (const: unstable) · fn size_hint(&self) -> (usize, Option<usize>)`
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
- `fn collect_into<E>(self, collection: &mut E) -> &mut Ewhere E: Extend<Self::Item>, Self: Sized,`
- `1.0.0 (const: unstable) · fn partition<B, F>(self, f: F) -> (B, B)where Self: Sized, B: Default + Extend<Self::Item>, F: FnMut(&Self::Item) -> bool,`
- `fn is_partitioned<P>(self, predicate: P) -> boolwhere Self: Sized, P: FnMut(Self::Item) -> bool,`
- `1.27.0 (const: unstable) · fn try_fold<B, F, R>(&mut self, init: B, f: F) -> Rwhere Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Output = B>,`
- `1.27.0 (const: unstable) · fn try_for_each<F, R>(&mut self, f: F) -> Rwhere Self: Sized, F: FnMut(Self::Item) -> R, R: Try<Output = ()>,`
- `1.0.0 (const: unstable) · fn fold<B, F>(self, init: B, f: F) -> Bwhere Self: Sized, F: FnMut(B, Self::Item) -> B,`
- `1.51.0 (const: unstable) · fn reduce<F>(self, f: F) -> Option<Self::Item>where Self: Sized, F: FnMut(Self::Item, Self::Item) -> Self::Item,`

### struct ego-tree::iter::NextSiblings

```rust
pub struct NextSiblings<'a, T: 'a>(/* private fields */);
```

Iterator over next siblings.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn next(&mut self) -> Option<Self::Item>`
- `fn next_chunk<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,`
- `1.0.0 (const: unstable) · fn size_hint(&self) -> (usize, Option<usize>)`
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
- `fn collect_into<E>(self, collection: &mut E) -> &mut Ewhere E: Extend<Self::Item>, Self: Sized,`
- `1.0.0 (const: unstable) · fn partition<B, F>(self, f: F) -> (B, B)where Self: Sized, B: Default + Extend<Self::Item>, F: FnMut(&Self::Item) -> bool,`
- `fn is_partitioned<P>(self, predicate: P) -> boolwhere Self: Sized, P: FnMut(Self::Item) -> bool,`
- `1.27.0 (const: unstable) · fn try_fold<B, F, R>(&mut self, init: B, f: F) -> Rwhere Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Output = B>,`
- `1.27.0 (const: unstable) · fn try_for_each<F, R>(&mut self, f: F) -> Rwhere Self: Sized, F: FnMut(Self::Item) -> R, R: Try<Output = ()>,`
- `1.0.0 (const: unstable) · fn fold<B, F>(self, init: B, f: F) -> Bwhere Self: Sized, F: FnMut(B, Self::Item) -> B,`
- `1.51.0 (const: unstable) · fn reduce<F>(self, f: F) -> Option<Self::Item>where Self: Sized, F: FnMut(Self::Item, Self::Item) -> Self::Item,`

### struct ego-tree::iter::Nodes

```rust
pub struct Nodes<'a, T: 'a> { /* private fields */ }
```

Iterator over nodes in insert order.

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
- `1.0.0 · fn len(&self) -> usize`
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

### struct ego-tree::iter::PrevSiblings

```rust
pub struct PrevSiblings<'a, T: 'a>(/* private fields */);
```

Iterator over previous siblings.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn next(&mut self) -> Option<Self::Item>`
- `fn next_chunk<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,`
- `1.0.0 (const: unstable) · fn size_hint(&self) -> (usize, Option<usize>)`
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
- `fn collect_into<E>(self, collection: &mut E) -> &mut Ewhere E: Extend<Self::Item>, Self: Sized,`
- `1.0.0 (const: unstable) · fn partition<B, F>(self, f: F) -> (B, B)where Self: Sized, B: Default + Extend<Self::Item>, F: FnMut(&Self::Item) -> bool,`
- `fn is_partitioned<P>(self, predicate: P) -> boolwhere Self: Sized, P: FnMut(Self::Item) -> bool,`
- `1.27.0 (const: unstable) · fn try_fold<B, F, R>(&mut self, init: B, f: F) -> Rwhere Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Output = B>,`
- `1.27.0 (const: unstable) · fn try_for_each<F, R>(&mut self, f: F) -> Rwhere Self: Sized, F: FnMut(Self::Item) -> R, R: Try<Output = ()>,`
- `1.0.0 (const: unstable) · fn fold<B, F>(self, init: B, f: F) -> Bwhere Self: Sized, F: FnMut(B, Self::Item) -> B,`
- `1.51.0 (const: unstable) · fn reduce<F>(self, f: F) -> Option<Self::Item>where Self: Sized, F: FnMut(Self::Item, Self::Item) -> Self::Item,`

### struct ego-tree::iter::Traverse

```rust
pub struct Traverse<'a, T: 'a> { /* private fields */ }
```

Iterator which traverses a subtree.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn next(&mut self) -> Option<Self::Item>`
- `fn next_chunk<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,`
- `1.0.0 (const: unstable) · fn size_hint(&self) -> (usize, Option<usize>)`
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
- `fn collect_into<E>(self, collection: &mut E) -> &mut Ewhere E: Extend<Self::Item>, Self: Sized,`
- `1.0.0 (const: unstable) · fn partition<B, F>(self, f: F) -> (B, B)where Self: Sized, B: Default + Extend<Self::Item>, F: FnMut(&Self::Item) -> bool,`
- `fn is_partitioned<P>(self, predicate: P) -> boolwhere Self: Sized, P: FnMut(Self::Item) -> bool,`
- `1.27.0 (const: unstable) · fn try_fold<B, F, R>(&mut self, init: B, f: F) -> Rwhere Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Output = B>,`
- `1.27.0 (const: unstable) · fn try_for_each<F, R>(&mut self, f: F) -> Rwhere Self: Sized, F: FnMut(Self::Item) -> R, R: Try<Output = ()>,`
- `1.0.0 (const: unstable) · fn fold<B, F>(self, init: B, f: F) -> Bwhere Self: Sized, F: FnMut(B, Self::Item) -> B,`
- `1.51.0 (const: unstable) · fn reduce<F>(self, f: F) -> Option<Self::Item>where Self: Sized, F: FnMut(Self::Item, Self::Item) -> Self::Item,`

### struct ego-tree::iter::Values

```rust
pub struct Values<'a, T: 'a>(/* private fields */);
```

Iterator over values in insert order.

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
- `1.0.0 · fn len(&self) -> usize`
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

### struct ego-tree::iter::ValuesMut

```rust
pub struct ValuesMut<'a, T: 'a>(/* private fields */);
```

Mutable iterator over values in insert order.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn next_back(&mut self) -> Option<Self::Item>`
- `fn next_chunk_back<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,`
- `fn advance_back_by(&mut self, n: usize) -> Result<(), NonZero<usize>>`
- `1.37.0 (const: unstable) · fn nth_back(&mut self, n: usize) -> Option<Self::Item>`
- `1.27.0 (const: unstable) · fn try_rfold<B, F, R>(&mut self, init: B, f: F) -> Rwhere Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Output = B>,`
- `1.27.0 (const: unstable) · fn rfold<B, F>(self, init: B, f: F) -> Bwhere Self: Sized, F: FnMut(B, Self::Item) -> B,`
- `1.27.0 (const: unstable) · fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>where Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.0.0 · fn len(&self) -> usize`
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
