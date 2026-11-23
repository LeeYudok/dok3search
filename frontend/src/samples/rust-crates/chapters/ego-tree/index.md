# Crate ego-tree

## Overview

Vec-backed ID-tree.

**Behavior**

- Trees have at least a root node;

- Nodes have zero or more ordered children;

- Nodes have at most one parent;

- Nodes can be detached (orphaned) but not removed;

- Node parent, next sibling, previous sibling, first child and last child
can be accessed in constant time;

- All methods perform in constant time;

- All iterators perform in linear time.

**Examples**

```rust
let mut tree = ego_tree::Tree::new('a');
let mut root = tree.root_mut();
root.append('b');
let mut c = root.append('c');
c.append('d');
c.append('e');
```

```rust
#[macro_use] extern crate ego_tree;
let tree = tree!('a' => { 'b', 'c' => { 'd', 'e' } });
```

## Items

- `iter` — Iterators.
- `tree` — Creates a tree from expressions.
- `NodeId` — Node ID.
- `NodeMut` — Node mutator.
- `NodeRef` — Node reference.
- `Tree` — Vec-backed ID-tree.

### macro ego-tree::tree

```rust
macro_rules! tree {
    (@ $n:ident { }) => { ... };
    (@ $n:ident { $value:expr_2021 }) => { ... };
    (@ $n:ident { $value:expr_2021, $($tail:tt)* }) => { ... };
    (@ $n:ident { $value:expr_2021 => $children:tt }) => { ... };
    (@ $n:ident { $value:expr_2021 => $children:tt, $($tail:tt)* }) => { ... };
    (@ $n:ident { @ $subtree:expr_2021 $(, $($tail:tt)*)? }) => { ... };
    ($root:expr_2021) => { ... };
    ($root:expr_2021 => $children:tt) => { ... };
}
```

Creates a tree from expressions.

**Examples**

```rust
#[macro_use] extern crate ego_tree;
let tree = tree!("root");
```

```rust
#[macro_use] extern crate ego_tree;
let tree = tree! {
    "root" => {
        "child a",
        "child b" => {
            "grandchild a",
            "grandchild b",
        },
        "child c",
    }
};
```

Compose trees using the `@` marker:

```rust
#[macro_use] extern crate ego_tree;
let subtree = tree! {
    "foo" => { "bar", "baz" }
};
let new_tree = tree! {
    "root" => {
        "child x",
        "child y",
        @ subtree,
    }
};
```

### struct ego-tree::NodeId

```rust
pub struct NodeId(/* private fields */);
```

Node ID.

Index into a `Tree`-internal `Vec`.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn hash<__H: Hasher>(&self, state: &mut __H)`
- `1.3.0 · fn hash_slice<H>(data: &[Self], state: &mut H)where H: Hasher, Self: Sized,`
- `fn cmp(&self, other: &Self) -> Ordering`
- `1.21.0 (const: unstable) · fn max(self, other: Self) -> Selfwhere Self: Sized,`
- `1.21.0 (const: unstable) · fn min(self, other: Self) -> Selfwhere Self: Sized,`
- `1.50.0 (const: unstable) · fn clamp(self, min: Self, max: Self) -> Selfwhere Self: Sized,`
- `fn clamp_to<R>(self, range: R) -> Selfwhere Self: Sized, R: ClampBounds<Self>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn partial_cmp(&self, other: &Self) -> Option<Ordering>`
- `1.0.0 (const: unstable) · fn lt(&self, other: &Rhs) -> bool`
- `1.0.0 (const: unstable) · fn le(&self, other: &Rhs) -> bool`
- `1.0.0 (const: unstable) · fn gt(&self, other: &Rhs) -> bool`
- `1.0.0 (const: unstable) · fn ge(&self, other: &Rhs) -> bool`

### struct ego-tree::NodeMut

```rust
pub struct NodeMut<'a, T: 'a> { /* private fields */ }
```

Node mutator.

**Methods**

- `pub fn sort(&mut self)where T: Ord,`
- `pub fn sort_by<F>(&mut self, compare: F)where F: FnMut(NodeRef<'_, T>, NodeRef<'_, T>) -> Ordering,`
- `pub fn sort_by_key<K, F>(&mut self, f: F)where F: FnMut(NodeRef<'_, T>) -> K, K: Ord,`
- `pub fn id(&self) -> NodeId`
- `pub fn tree(&mut self) -> &mut Tree<T>`
- `pub fn value(&mut self) -> &mut T`
- `pub fn as_ref(&mut self) -> NodeRef<'_, T>`
- `pub fn parent(&mut self) -> Option<NodeMut<'_, T>>`
- `pub fn into_parent(self) -> Result<Self, Self>`
- `pub fn prev_sibling(&mut self) -> Option<NodeMut<'_, T>>`
- `pub fn into_prev_sibling(self) -> Result<Self, Self>`
- `pub fn next_sibling(&mut self) -> Option<NodeMut<'_, T>>`
- `pub fn into_next_sibling(self) -> Result<Self, Self>`
- `pub fn first_child(&mut self) -> Option<NodeMut<'_, T>>`
- `pub fn into_first_child(self) -> Result<Self, Self>`
- `pub fn last_child(&mut self) -> Option<NodeMut<'_, T>>`
- `pub fn into_last_child(self) -> Result<Self, Self>`
- `pub fn has_siblings(&self) -> bool`
- `pub fn has_children(&self) -> bool`
- `pub fn for_each_ancestor<'b, F>(&'b mut self, f: F)where F: FnMut(&mut NodeMut<'b, T>),`
- `pub fn for_each_next_sibling<'b, F>(&'b mut self, f: F)where F: FnMut(&mut NodeMut<'b, T>),`
- `pub fn for_each_prev_sibling<'b, F>(&'b mut self, f: F)where F: FnMut(&mut NodeMut<'b, T>),`
- `pub fn for_each_sibling<F>(&mut self, f: F)where F: for<'b> FnMut(&mut NodeMut<'b, T>),`
- `pub fn for_each_child<F>(&mut self, f: F)where F: for<'b> FnMut(&mut NodeMut<'b, T>),`
- `pub fn for_each_descendant<F>(&mut self, f: F)where F: FnMut(&mut NodeMut<'_, T>),`
- `pub fn append(&mut self, value: T) -> NodeMut<'_, T>`
- `pub fn prepend(&mut self, value: T) -> NodeMut<'_, T>`
- `pub fn append_subtree(&mut self, subtree: Tree<T>) -> NodeMut<'_, T>`
- `pub fn prepend_subtree(&mut self, subtree: Tree<T>) -> NodeMut<'_, T>`
- `pub fn insert_before(&mut self, value: T) -> NodeMut<'_, T>`
- `pub fn insert_after(&mut self, value: T) -> NodeMut<'_, T>`
- `pub fn detach(&mut self)`
- `pub fn append_id(&mut self, new_child_id: NodeId) -> NodeMut<'_, T>`
- `pub fn prepend_id(&mut self, new_child_id: NodeId) -> NodeMut<'_, T>`
- `pub fn insert_id_before(&mut self, new_sibling_id: NodeId) -> NodeMut<'_, T>`
- `pub fn insert_id_after(&mut self, new_sibling_id: NodeId) -> NodeMut<'_, T>`
- `pub fn reparent_from_id_append(&mut self, from_id: NodeId)`
- `pub fn reparent_from_id_prepend(&mut self, from_id: NodeId)`
- `pub fn clone_subtree(&mut self) -> NodeMut<'_, T>where T: Clone,`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct ego-tree::NodeRef

```rust
pub struct NodeRef<'a, T: 'a> { /* private fields */ }
```

Node reference.

**Methods**

- `pub fn ancestors(&self) -> Ancestors<'a, T>`
- `pub fn prev_siblings(&self) -> PrevSiblings<'a, T>`
- `pub fn next_siblings(&self) -> NextSiblings<'a, T>`
- `pub fn first_children(&self) -> FirstChildren<'a, T>`
- `pub fn last_children(&self) -> LastChildren<'a, T>`
- `pub fn children(&self) -> Children<'a, T>`
- `pub fn traverse(&self) -> Traverse<'a, T>`
- `pub fn descendants(&self) -> Descendants<'a, T>`
- `pub fn id(&self) -> NodeId`
- `pub fn tree(&self) -> &'a Tree<T>`
- `pub fn value(&self) -> &'a T`
- `pub fn parent(&self) -> Option<Self>`
- `pub fn prev_sibling(&self) -> Option<Self>`
- `pub fn next_sibling(&self) -> Option<Self>`
- `pub fn first_child(&self) -> Option<Self>`
- `pub fn last_child(&self) -> Option<Self>`
- `pub fn has_siblings(&self) -> bool`
- `pub fn has_children(&self) -> bool`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn from(node: NodeMut<'a, T>) -> Self`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### struct ego-tree::Tree

```rust
pub struct Tree<T> { /* private fields */ }
```

Vec-backed ID-tree.

Always contains at least a root node.

**Methods**

- `pub fn values(&self) -> Values<'_, T>`
- `pub fn values_mut(&mut self) -> ValuesMut<'_, T>`
- `pub fn nodes(&self) -> Nodes<'_, T>`
- `pub fn new(root: T) -> Self`
- `pub fn with_capacity(root: T, capacity: usize) -> Self`
- `pub fn get(&self, id: NodeId) -> Option<NodeRef<'_, T>>`
- `pub fn get_mut(&mut self, id: NodeId) -> Option<NodeMut<'_, T>>`
- `pub unsafe fn get_unchecked(&self, id: NodeId) -> NodeRef<'_, T>`
- `pub unsafe fn get_unchecked_mut(&mut self, id: NodeId) -> NodeMut<'_, T>`
- `pub fn root(&self) -> NodeRef<'_, T>`
- `pub fn root_mut(&mut self) -> NodeMut<'_, T>`
- `pub fn orphan(&mut self, value: T) -> NodeMut<'_, T>`
- `pub fn extend_tree(&mut self, other_tree: Tree<T>) -> NodeMut<'_, T>`
- `pub fn map<F, U>(self, transform: F) -> Tree<U>where F: FnMut(T) -> U,`
- `pub fn map_ref<F, U>(&self, transform: F) -> Tree<U>where F: FnMut(&T) -> U,`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn hash<__H: Hasher>(&self, state: &mut __H)`
- `1.3.0 · fn hash_slice<H>(data: &[Self], state: &mut H)where H: Hasher, Self: Sized,`
- `fn into_iter(self) -> Self::IntoIter`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
