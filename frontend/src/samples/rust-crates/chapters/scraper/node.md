# Module scraper::node

## Overview

HTML nodes.

## Items

- `Attrs` — Iterator over attributes.
- `Classes` — Iterator over classes.
- `Comment` — An HTML comment.
- `Doctype` — A doctype.
- `Element` — An HTML element.
- `ProcessingInstruction` — HTML Processing Instruction.
- `Text` — HTML text.
- `Node` — An HTML node.
- `Attributes` — A Map of attributes that doesn’t preserve the order of the attributes. Please enable the deterministic feature for order-preserving (de)serialization.
- `AttributesIter` — An iterator over a node’s attributes.

### enum scraper::node::Node

```rust
pub enum Node {
    Document,
    Fragment,
    Doctype(Doctype),
    Comment(Comment),
    Text(Text),
    Element(Element),
    ProcessingInstruction(ProcessingInstruction),
}
```

An HTML node.

**Methods**

- `pub fn is_document(&self) -> bool`
- `pub fn is_fragment(&self) -> bool`
- `pub fn is_doctype(&self) -> bool`
- `pub fn is_comment(&self) -> bool`
- `pub fn is_text(&self) -> bool`
- `pub fn is_element(&self) -> bool`
- `pub fn as_doctype(&self) -> Option<&Doctype>`
- `pub fn as_comment(&self) -> Option<&Comment>`
- `pub fn as_text(&self) -> Option<&Text>`
- `pub fn as_element(&self) -> Option<&Element>`
- `pub fn as_processing_instruction(&self) -> Option<&ProcessingInstruction>`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### struct scraper::node::Attrs

```rust
pub struct Attrs<'a> { /* private fields */ }
```

Iterator over attributes.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn next(&mut self) -> Option<(&'a str, &'a str)>`
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
- `1.29.0 (const: unstable) · fn flatten(self) -> Flatten<Self> ⓘwhere Self: Sized, Self::Item: IntoIterator,`
- `fn map_windows<F, R, const N: usize>(self, f: F) -> MapWindows<Self, F, N> ⓘwhere Self: Sized, F: FnMut(&[Self::Item; N]) -> R,`
- `1.0.0 (const: unstable) · fn fuse(self) -> Fuse<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn inspect<F>(self, f: F) -> Inspect<Self, F> ⓘwhere Self: Sized, F: FnMut(&Self::Item),`
- `1.0.0 (const: unstable) · fn by_ref(&mut self) -> &mut Selfwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn collect<B>(self) -> Bwhere B: FromIterator<Self::Item>, Self: Sized,`
- `fn try_collect<B>( &mut self, ) -> <<Self::Item as Try>::Residual as Residual<B>>::TryTypewhere Self: Sized, Self::Item: Try, <Self::Item as Try>::Residual: Residual<B>, B: FromIterator<<Self::Item as Try>::Output>,`
- `fn collect_into<E>(self, collection: &mut E) -> &mut Ewhere E: Extend<Self::Item>, Self: Sized,`
- `1.0.0 (const: unstable) · fn partition<B, F>(self, f: F) -> (B, B)where Self: Sized, B: Default + Extend<Self::Item>, F: FnMut(&Self::Item) -> bool,`
- `fn is_partitioned<P>(self, predicate: P) -> boolwhere Self: Sized, P: FnMut(Self::Item) -> bool,`
- `1.27.0 (const: unstable) · fn try_fold<B, F, R>(&mut self, init: B, f: F) -> Rwhere Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Output = B>,`
- `1.27.0 (const: unstable) · fn try_for_each<F, R>(&mut self, f: F) -> Rwhere Self: Sized, F: FnMut(Self::Item) -> R, R: Try<Output = ()>,`
- `1.0.0 (const: unstable) · fn fold<B, F>(self, init: B, f: F) -> Bwhere Self: Sized, F: FnMut(B, Self::Item) -> B,`

### struct scraper::node::Classes

```rust
pub struct Classes<'a> { /* private fields */ }
```

Iterator over classes.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn next(&mut self) -> Option<&'a str>`
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
- `1.29.0 (const: unstable) · fn flatten(self) -> Flatten<Self> ⓘwhere Self: Sized, Self::Item: IntoIterator,`
- `fn map_windows<F, R, const N: usize>(self, f: F) -> MapWindows<Self, F, N> ⓘwhere Self: Sized, F: FnMut(&[Self::Item; N]) -> R,`
- `1.0.0 (const: unstable) · fn fuse(self) -> Fuse<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn inspect<F>(self, f: F) -> Inspect<Self, F> ⓘwhere Self: Sized, F: FnMut(&Self::Item),`
- `1.0.0 (const: unstable) · fn by_ref(&mut self) -> &mut Selfwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn collect<B>(self) -> Bwhere B: FromIterator<Self::Item>, Self: Sized,`
- `fn try_collect<B>( &mut self, ) -> <<Self::Item as Try>::Residual as Residual<B>>::TryTypewhere Self: Sized, Self::Item: Try, <Self::Item as Try>::Residual: Residual<B>, B: FromIterator<<Self::Item as Try>::Output>,`
- `fn collect_into<E>(self, collection: &mut E) -> &mut Ewhere E: Extend<Self::Item>, Self: Sized,`
- `1.0.0 (const: unstable) · fn partition<B, F>(self, f: F) -> (B, B)where Self: Sized, B: Default + Extend<Self::Item>, F: FnMut(&Self::Item) -> bool,`
- `fn is_partitioned<P>(self, predicate: P) -> boolwhere Self: Sized, P: FnMut(Self::Item) -> bool,`
- `1.27.0 (const: unstable) · fn try_fold<B, F, R>(&mut self, init: B, f: F) -> Rwhere Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Output = B>,`
- `1.27.0 (const: unstable) · fn try_for_each<F, R>(&mut self, f: F) -> Rwhere Self: Sized, F: FnMut(Self::Item) -> R, R: Try<Output = ()>,`
- `1.0.0 (const: unstable) · fn fold<B, F>(self, init: B, f: F) -> Bwhere Self: Sized, F: FnMut(B, Self::Item) -> B,`

### struct scraper::node::Comment

```rust
pub struct Comment {
    pub comment: StrTendril,
}
```

An HTML comment.

**Methods**

- `1.0.0 · pub fn len(&self) -> usize`
- `1.0.0 · pub fn is_empty(&self) -> bool`
- `1.9.0 · pub fn is_char_boundary(&self, index: usize) -> bool`
- `1.91.0 · pub fn floor_char_boundary(&self, index: usize) -> usize`
- `1.91.0 · pub fn ceil_char_boundary(&self, index: usize) -> usize`
- `1.0.0 · pub fn as_bytes(&self) -> &[u8]`
- `1.0.0 · pub fn as_ptr(&self) -> *const u8`
- `1.20.0 · pub fn get<I>(&self, i: I) -> Option<&<I as SliceIndex<str>>::Output>where I: SliceIndex<str>,`
- `1.20.0 · pub unsafe fn get_unchecked<I>(&self, i: I) -> &<I as SliceIndex<str>>::Outputwhere I: SliceIndex<str>,`
- `1.0.0 · pub unsafe fn slice_unchecked(&self, begin: usize, end: usize) -> &str`
- `1.4.0 · pub fn split_at(&self, mid: usize) -> (&str, &str)`
- `1.80.0 · pub fn split_at_checked(&self, mid: usize) -> Option<(&str, &str)>`
- `1.0.0 · pub fn chars(&self) -> Chars<'_>`
- `1.0.0 · pub fn char_indices(&self) -> CharIndices<'_>`
- `1.0.0 · pub fn bytes(&self) -> Bytes<'_>`
- `1.1.0 · pub fn split_whitespace(&self) -> SplitWhitespace<'_>`
- `1.34.0 · pub fn split_ascii_whitespace(&self) -> SplitAsciiWhitespace<'_>`
- `1.0.0 · pub fn lines(&self) -> Lines<'_>`
- `1.0.0 · pub fn lines_any(&self) -> LinesAny<'_>`
- `1.8.0 · pub fn encode_utf16(&self) -> EncodeUtf16<'_>`
- `1.0.0 · pub fn contains<P>(&self, pat: P) -> boolwhere P: Pattern,`
- `1.0.0 · pub fn starts_with<P>(&self, pat: P) -> boolwhere P: Pattern,`
- `1.0.0 · pub fn ends_with<P>(&self, pat: P) -> boolwhere P: Pattern, <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,`
- `1.0.0 · pub fn find<P>(&self, pat: P) -> Option<usize>where P: Pattern,`
- `1.0.0 · pub fn rfind<P>(&self, pat: P) -> Option<usize>where P: Pattern, <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,`
- `1.0.0 · pub fn split<P>(&self, pat: P) -> Split<'_, P> ⓘwhere P: Pattern,`
- `1.51.0 · pub fn split_inclusive<P>(&self, pat: P) -> SplitInclusive<'_, P> ⓘwhere P: Pattern,`
- `1.0.0 · pub fn rsplit<P>(&self, pat: P) -> RSplit<'_, P> ⓘwhere P: Pattern, <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,`
- `1.0.0 · pub fn split_terminator<P>(&self, pat: P) -> SplitTerminator<'_, P> ⓘwhere P: Pattern,`
- `1.0.0 · pub fn rsplit_terminator<P>(&self, pat: P) -> RSplitTerminator<'_, P> ⓘwhere P: Pattern, <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,`
- `1.0.0 · pub fn splitn<P>(&self, n: usize, pat: P) -> SplitN<'_, P> ⓘwhere P: Pattern,`
- `1.0.0 · pub fn rsplitn<P>(&self, n: usize, pat: P) -> RSplitN<'_, P> ⓘwhere P: Pattern, <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,`
- `1.52.0 · pub fn split_once<P>(&self, delimiter: P) -> Option<(&str, &str)>where P: Pattern,`
- `1.52.0 · pub fn rsplit_once<P>(&self, delimiter: P) -> Option<(&str, &str)>where P: Pattern, <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,`
- `1.2.0 · pub fn matches<P>(&self, pat: P) -> Matches<'_, P> ⓘwhere P: Pattern,`
- `1.2.0 · pub fn rmatches<P>(&self, pat: P) -> RMatches<'_, P> ⓘwhere P: Pattern, <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,`
- `1.5.0 · pub fn match_indices<P>(&self, pat: P) -> MatchIndices<'_, P> ⓘwhere P: Pattern,`
- `1.5.0 · pub fn rmatch_indices<P>(&self, pat: P) -> RMatchIndices<'_, P> ⓘwhere P: Pattern, <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,`
- `1.0.0 · pub fn trim(&self) -> &str`
- `1.30.0 · pub fn trim_start(&self) -> &str`

### struct scraper::node::Doctype

```rust
pub struct Doctype {
    pub name: StrTendril,
    pub public_id: StrTendril,
    pub system_id: StrTendril,
}
```

A doctype.

**Methods**

- `pub fn name(&self) -> &str`
- `pub fn public_id(&self) -> &str`
- `pub fn system_id(&self) -> &str`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### struct scraper::node::Element

```rust
pub struct Element {
    pub name: QualName,
    pub attrs: Attributes,
    /* private fields */
}
```

An HTML element.

This is the data stored in a `Node::Element` node. It is distinct from the
`Element` trait re-exported at the crate root, which is
implemented by `ElementRef` for CSS selector matching.
To access the underlying element of a selected element, call
`ElementRef::value`.

**Methods**

- `pub fn name(&self) -> &str`
- `pub fn id(&self) -> Option<&str>`
- `pub fn has_class(&self, class: &str, case_sensitive: CaseSensitivity) -> bool`
- `pub fn classes(&self) -> Classes<'_>`
- `pub fn attr(&self, attr: &str) -> Option<&str>`
- `pub fn attrs(&self) -> Attrs<'_>`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### struct scraper::node::ProcessingInstruction

```rust
pub struct ProcessingInstruction {
    pub target: StrTendril,
    pub data: StrTendril,
}
```

HTML Processing Instruction.

**Methods**

- `1.0.0 · pub fn len(&self) -> usize`
- `1.0.0 · pub fn is_empty(&self) -> bool`
- `1.9.0 · pub fn is_char_boundary(&self, index: usize) -> bool`
- `1.91.0 · pub fn floor_char_boundary(&self, index: usize) -> usize`
- `1.91.0 · pub fn ceil_char_boundary(&self, index: usize) -> usize`
- `1.0.0 · pub fn as_bytes(&self) -> &[u8]`
- `1.0.0 · pub fn as_ptr(&self) -> *const u8`
- `1.20.0 · pub fn get<I>(&self, i: I) -> Option<&<I as SliceIndex<str>>::Output>where I: SliceIndex<str>,`
- `1.20.0 · pub unsafe fn get_unchecked<I>(&self, i: I) -> &<I as SliceIndex<str>>::Outputwhere I: SliceIndex<str>,`
- `1.0.0 · pub unsafe fn slice_unchecked(&self, begin: usize, end: usize) -> &str`
- `1.4.0 · pub fn split_at(&self, mid: usize) -> (&str, &str)`
- `1.80.0 · pub fn split_at_checked(&self, mid: usize) -> Option<(&str, &str)>`
- `1.0.0 · pub fn chars(&self) -> Chars<'_>`
- `1.0.0 · pub fn char_indices(&self) -> CharIndices<'_>`
- `1.0.0 · pub fn bytes(&self) -> Bytes<'_>`
- `1.1.0 · pub fn split_whitespace(&self) -> SplitWhitespace<'_>`
- `1.34.0 · pub fn split_ascii_whitespace(&self) -> SplitAsciiWhitespace<'_>`
- `1.0.0 · pub fn lines(&self) -> Lines<'_>`
- `1.0.0 · pub fn lines_any(&self) -> LinesAny<'_>`
- `1.8.0 · pub fn encode_utf16(&self) -> EncodeUtf16<'_>`
- `1.0.0 · pub fn contains<P>(&self, pat: P) -> boolwhere P: Pattern,`
- `1.0.0 · pub fn starts_with<P>(&self, pat: P) -> boolwhere P: Pattern,`
- `1.0.0 · pub fn ends_with<P>(&self, pat: P) -> boolwhere P: Pattern, <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,`
- `1.0.0 · pub fn find<P>(&self, pat: P) -> Option<usize>where P: Pattern,`
- `1.0.0 · pub fn rfind<P>(&self, pat: P) -> Option<usize>where P: Pattern, <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,`
- `1.0.0 · pub fn split<P>(&self, pat: P) -> Split<'_, P> ⓘwhere P: Pattern,`
- `1.51.0 · pub fn split_inclusive<P>(&self, pat: P) -> SplitInclusive<'_, P> ⓘwhere P: Pattern,`
- `1.0.0 · pub fn rsplit<P>(&self, pat: P) -> RSplit<'_, P> ⓘwhere P: Pattern, <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,`
- `1.0.0 · pub fn split_terminator<P>(&self, pat: P) -> SplitTerminator<'_, P> ⓘwhere P: Pattern,`
- `1.0.0 · pub fn rsplit_terminator<P>(&self, pat: P) -> RSplitTerminator<'_, P> ⓘwhere P: Pattern, <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,`
- `1.0.0 · pub fn splitn<P>(&self, n: usize, pat: P) -> SplitN<'_, P> ⓘwhere P: Pattern,`
- `1.0.0 · pub fn rsplitn<P>(&self, n: usize, pat: P) -> RSplitN<'_, P> ⓘwhere P: Pattern, <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,`
- `1.52.0 · pub fn split_once<P>(&self, delimiter: P) -> Option<(&str, &str)>where P: Pattern,`
- `1.52.0 · pub fn rsplit_once<P>(&self, delimiter: P) -> Option<(&str, &str)>where P: Pattern, <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,`
- `1.2.0 · pub fn matches<P>(&self, pat: P) -> Matches<'_, P> ⓘwhere P: Pattern,`
- `1.2.0 · pub fn rmatches<P>(&self, pat: P) -> RMatches<'_, P> ⓘwhere P: Pattern, <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,`
- `1.5.0 · pub fn match_indices<P>(&self, pat: P) -> MatchIndices<'_, P> ⓘwhere P: Pattern,`
- `1.5.0 · pub fn rmatch_indices<P>(&self, pat: P) -> RMatchIndices<'_, P> ⓘwhere P: Pattern, <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,`
- `1.0.0 · pub fn trim(&self) -> &str`
- `1.30.0 · pub fn trim_start(&self) -> &str`

### struct scraper::node::Text

```rust
pub struct Text {
    pub text: StrTendril,
}
```

HTML text.

**Methods**

- `1.0.0 · pub fn len(&self) -> usize`
- `1.0.0 · pub fn is_empty(&self) -> bool`
- `1.9.0 · pub fn is_char_boundary(&self, index: usize) -> bool`
- `1.91.0 · pub fn floor_char_boundary(&self, index: usize) -> usize`
- `1.91.0 · pub fn ceil_char_boundary(&self, index: usize) -> usize`
- `1.0.0 · pub fn as_bytes(&self) -> &[u8]`
- `1.0.0 · pub fn as_ptr(&self) -> *const u8`
- `1.20.0 · pub fn get<I>(&self, i: I) -> Option<&<I as SliceIndex<str>>::Output>where I: SliceIndex<str>,`
- `1.20.0 · pub unsafe fn get_unchecked<I>(&self, i: I) -> &<I as SliceIndex<str>>::Outputwhere I: SliceIndex<str>,`
- `1.0.0 · pub unsafe fn slice_unchecked(&self, begin: usize, end: usize) -> &str`
- `1.4.0 · pub fn split_at(&self, mid: usize) -> (&str, &str)`
- `1.80.0 · pub fn split_at_checked(&self, mid: usize) -> Option<(&str, &str)>`
- `1.0.0 · pub fn chars(&self) -> Chars<'_>`
- `1.0.0 · pub fn char_indices(&self) -> CharIndices<'_>`
- `1.0.0 · pub fn bytes(&self) -> Bytes<'_>`
- `1.1.0 · pub fn split_whitespace(&self) -> SplitWhitespace<'_>`
- `1.34.0 · pub fn split_ascii_whitespace(&self) -> SplitAsciiWhitespace<'_>`
- `1.0.0 · pub fn lines(&self) -> Lines<'_>`
- `1.0.0 · pub fn lines_any(&self) -> LinesAny<'_>`
- `1.8.0 · pub fn encode_utf16(&self) -> EncodeUtf16<'_>`
- `1.0.0 · pub fn contains<P>(&self, pat: P) -> boolwhere P: Pattern,`
- `1.0.0 · pub fn starts_with<P>(&self, pat: P) -> boolwhere P: Pattern,`
- `1.0.0 · pub fn ends_with<P>(&self, pat: P) -> boolwhere P: Pattern, <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,`
- `1.0.0 · pub fn find<P>(&self, pat: P) -> Option<usize>where P: Pattern,`
- `1.0.0 · pub fn rfind<P>(&self, pat: P) -> Option<usize>where P: Pattern, <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,`
- `1.0.0 · pub fn split<P>(&self, pat: P) -> Split<'_, P> ⓘwhere P: Pattern,`
- `1.51.0 · pub fn split_inclusive<P>(&self, pat: P) -> SplitInclusive<'_, P> ⓘwhere P: Pattern,`
- `1.0.0 · pub fn rsplit<P>(&self, pat: P) -> RSplit<'_, P> ⓘwhere P: Pattern, <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,`
- `1.0.0 · pub fn split_terminator<P>(&self, pat: P) -> SplitTerminator<'_, P> ⓘwhere P: Pattern,`
- `1.0.0 · pub fn rsplit_terminator<P>(&self, pat: P) -> RSplitTerminator<'_, P> ⓘwhere P: Pattern, <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,`
- `1.0.0 · pub fn splitn<P>(&self, n: usize, pat: P) -> SplitN<'_, P> ⓘwhere P: Pattern,`
- `1.0.0 · pub fn rsplitn<P>(&self, n: usize, pat: P) -> RSplitN<'_, P> ⓘwhere P: Pattern, <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,`
- `1.52.0 · pub fn split_once<P>(&self, delimiter: P) -> Option<(&str, &str)>where P: Pattern,`
- `1.52.0 · pub fn rsplit_once<P>(&self, delimiter: P) -> Option<(&str, &str)>where P: Pattern, <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,`
- `1.2.0 · pub fn matches<P>(&self, pat: P) -> Matches<'_, P> ⓘwhere P: Pattern,`
- `1.2.0 · pub fn rmatches<P>(&self, pat: P) -> RMatches<'_, P> ⓘwhere P: Pattern, <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,`
- `1.5.0 · pub fn match_indices<P>(&self, pat: P) -> MatchIndices<'_, P> ⓘwhere P: Pattern,`
- `1.5.0 · pub fn rmatch_indices<P>(&self, pat: P) -> RMatchIndices<'_, P> ⓘwhere P: Pattern, <P as Pattern>::Searcher<'a>: for<'a> ReverseSearcher<'a>,`
- `1.0.0 · pub fn trim(&self) -> &str`
- `1.30.0 · pub fn trim_start(&self) -> &str`

### type scraper::node::Attributes

```rust
pub type Attributes = Vec<(QualName, StrTendril)>;
```

A Map of attributes that doesn’t preserve the order of the attributes.
Please enable the `deterministic` feature for order-preserving
(de)serialization.

### type scraper::node::AttributesIter

```rust
pub type AttributesIter<'a> = Iter<'a, (QualName, StrTendril)>;
```

An iterator over a node’s attributes.
