# 상호운용성

In some situations, it’s necessary to modify data behind a shared (read-only) reference. For example, a shared data structure might have an internal cache, and wish to update that cache from read-only methods.

The “interior mutability” pattern allows exclusive (mutable) access behind a shared reference. The standard library provides several ways to do this, all while still ensuring safety, typically by performing a runtime check.

## `RefCell`

``` rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default)]
struct Node {
    value: i64,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: i64) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { value, ..Node::default() }))
    }

    fn sum(&self) -> i64 {
        self.value + self.children.iter().map(|c| c.borrow().sum()).sum::<i64>()
    }
}

fn main() {
    let root = Node::new(1);
    root.borrow_mut().children.push(Node::new(5));
    let subtree = Node::new(10);
    subtree.borrow_mut().children.push(Node::new(11));
    subtree.borrow_mut().children.push(Node::new(12));
    root.borrow_mut().children.push(subtree);

    println!("그래프: {root:#?}");
    println!("그래프 합계: {}", root.borrow().sum());
}
```

## `Cell`

`Cell` wraps a value and allows getting or setting the value, even with a shared reference to the `Cell`. However, it does not allow any references to the value. Since there are no references, borrowing rules cannot be broken.

> **발표자 노트**
>
> This slide should take about 10 minutes.
>
> The main thing to take away from this slide is that Rust provides *safe* ways to modify data behind a shared reference. There are a variety of ways to ensure that safety, and `RefCell` and `Cell` are two of them.
>
> - `RefCell` enforces Rust’s usual borrowing rules (either multiple shared references or a single exclusive reference) with a runtime check. In this case, all borrows are very short and never overlap, so the checks always succeed.
>
> - `Rc` only allows shared (read-only) access to its contents, since its purpose is to allow (and count) many references. But we want to modify the value, so we need interior mutability.
>
> - `Cell` is a simpler means to ensure safety: it has a `set` method that takes `&self`. This needs no runtime check, but requires moving values, which can have its own cost.
>
> - Demonstrate that reference loops can be created by adding `root` to `subtree.children`.
>
> - `self.value`를 증가시키는 메서드인`fn inc(&mut self)`를 추가하고 그 메서드를 자식노드에서 호출하세요. 그러면 `thread 'main' panicked at 'already borrowed: BorrowMutError'` 런타임 패닉이 발생함을 보이세요.
