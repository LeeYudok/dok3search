# Interior mutability

[interior-mut](#r-interior-mut)

# Interior mutability

[interior-mut.intro](#r-interior-mut.intro)

Sometimes a type needs to be mutated while having multiple aliases. In Rust this is achieved using a pattern called *interior mutability*.

[interior-mut.shared-ref](#r-interior-mut.shared-ref)

A type has interior mutability if its internal state can be changed through a [shared reference](types/pointer.md#shared-references-) to it.

[interior-mut.no-constraint](#r-interior-mut.no-constraint)

This goes against the usual [requirement](behavior-considered-undefined.md) that the value pointed to by a shared reference is not mutated.

[interior-mut.unsafe-cell](#r-interior-mut.unsafe-cell)

[`std::cell::UnsafeCell<T>`](https://doc.rust-lang.org/1.100.0/core/cell/struct.UnsafeCell.html) type is the only allowed way to disable this requirement. When `UnsafeCell<T>` is immutably aliased, it is still safe to mutate, or obtain a mutable reference to, the `T` it contains.

[interior-mut.mut-unsafe-cell](#r-interior-mut.mut-unsafe-cell)

As with all other types, it is undefined behavior to have multiple `&mut UnsafeCell<T>` aliases.

[interior-mut.abstraction](#r-interior-mut.abstraction)

Other types with interior mutability can be created by using `UnsafeCell<T>` as a field. The standard library provides a variety of types that provide safe interior mutability APIs.

[interior-mut.ref-cell](#r-interior-mut.ref-cell)

For example, [`std::cell::RefCell<T>`](https://doc.rust-lang.org/1.100.0/core/cell/struct.RefCell.html) uses run-time borrow checks to ensure the usual rules around multiple references.

[interior-mut.atomic](#r-interior-mut.atomic)

The [`std::sync::atomic`](https://doc.rust-lang.org/1.100.0/core/sync/atomic/index.html) module contains types that wrap a value that is only accessed with atomic operations, allowing the value to be shared and mutated across threads.
