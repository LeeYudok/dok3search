# Module scraper::html

## Overview

HTML documents and fragments.

## Items

- `Html` — An HTML tree.
- `HtmlTreeSink` — Wraps Html instances as sinks to drive parsing
- `Select` — Iterator over elements matching a selector.

### struct scraper::html::Html

```rust
pub struct Html {
    pub errors: Vec<Cow<'static, str>>,
    pub quirks_mode: QuirksMode,
    pub tree: Tree<Node>,
}
```

An HTML tree.

Parsing does not fail hard. Instead, the `quirks_mode` is set and errors are added to the
`errors` field. The `tree` will still be populated as best as possible.

Implements the `TreeSink` trait from the `html5ever` crate, which allows HTML to be parsed.

**Methods**

- `pub fn new_document() -> Self`
- `pub fn new_fragment() -> Self`
- `pub fn parse_document(document: &str) -> Self`
- `pub fn parse_fragment(fragment: &str) -> Self`
- `pub fn select<'a, 'b>(&'a self, selector: &'b Selector) -> Select<'a, 'b>`
- `pub fn root_element(&self) -> ElementRef<'_>`
- `pub fn html(&self) -> String`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn select(self, selector: &Selector) -> Self::Select<'_>`
- `fn serialize<S: Serializer>( &self, serializer: &mut S, traversal_scope: TraversalScope, ) -> Result<(), Error>`

### struct scraper::html::HtmlTreeSink

```rust
pub struct HtmlTreeSink(pub RefCell<Html>);
```

Wraps `Html` instances as sinks to drive parsing

**Methods**

- `pub fn new(html: Html) -> Self`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn finish(self) -> Html`
- `fn parse_error(&self, msg: Cow<'static, str>)`
- `fn set_quirks_mode(&self, mode: QuirksMode)`
- `fn get_document(&self) -> Self::Handle`
- `fn same_node(&self, x: &Self::Handle, y: &Self::Handle) -> bool`
- `fn elem_name<'a>(&'a self, target: &Self::Handle) -> Ref<'a, QualName>`
- `fn create_element( &self, name: QualName, attrs: Vec<Attribute>, _flags: ElementFlags, ) -> Self::Handle`
- `fn create_comment(&self, text: StrTendril) -> Self::Handle`
- `fn append_doctype_to_document( &self, name: StrTendril, public_id: StrTendril, system_id: StrTendril, )`
- `fn append(&self, parent: &Self::Handle, child: NodeOrText<Self::Handle>)`
- `fn append_before_sibling( &self, sibling: &Self::Handle, new_node: NodeOrText<Self::Handle>, )`
- `fn remove_from_parent(&self, target: &Self::Handle)`
- `fn reparent_children(&self, node: &Self::Handle, new_parent: &Self::Handle)`
- `fn add_attrs_if_missing(&self, target: &Self::Handle, attrs: Vec<Attribute>)`
- `fn get_template_contents(&self, target: &Self::Handle) -> Self::Handle`
- `fn mark_script_already_started(&self, _node: &Self::Handle)`
- `fn create_pi(&self, target: StrTendril, data: StrTendril) -> Self::Handle`
- `fn append_based_on_parent_node( &self, element: &Self::Handle, prev_element: &Self::Handle, child: NodeOrText<Self::Handle>, )`
- `fn pop(&self, _node: &Self::Handle)`
- `fn associate_with_form( &self, _target: &Self::Handle, _form: &Self::Handle, _nodes: (&Self::Handle, Option<&Self::Handle>), )`
- `fn is_mathml_annotation_xml_integration_point( &self, _handle: &Self::Handle, ) -> bool`
- `fn set_current_line(&self, _line_number: u64)`
- `fn allow_declarative_shadow_roots( &self, _intended_parent: &Self::Handle, ) -> bool`
- `fn attach_declarative_shadow( &self, _location: &Self::Handle, _template: &Self::Handle, _attrs: &[Attribute], ) -> bool`
- `fn maybe_clone_an_option_into_selectedcontent(&self, option: &Self::Handle)`

### struct scraper::html::Select

```rust
pub struct Select<'a, 'b> { /* private fields */ }
```

Iterator over elements matching a selector.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `fn next_back(&mut self) -> Option<Self::Item>`
- `fn next_chunk_back<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,`
- `fn advance_back_by(&mut self, n: usize) -> Result<(), NonZero<usize>>`
- `1.37.0 (const: unstable) · fn nth_back(&mut self, n: usize) -> Option<Self::Item>`
- `1.27.0 (const: unstable) · fn try_rfold<B, F, R>(&mut self, init: B, f: F) -> Rwhere Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Output = B>,`
- `1.27.0 (const: unstable) · fn rfold<B, F>(self, init: B, f: F) -> Bwhere Self: Sized, F: FnMut(B, Self::Item) -> B,`
- `1.27.0 (const: unstable) · fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>where Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `fn next(&mut self) -> Option<ElementRef<'a>>`
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
