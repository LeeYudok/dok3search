# Special types and traits

[lang-types](#r-lang-types)

# Special types and traits

[lang-types.intro](#r-lang-types.intro)

Certain types and traits that exist in [the standard library](https://doc.rust-lang.org/1.100.0/std/index.html) are known to the Rust compiler. This chapter documents the special features of these types and traits.

[lang-types.box](#r-lang-types.box)

## `Box<T>`

[lang-types.box.intro](#r-lang-types.box.intro)

[`Box<T>`](https://doc.rust-lang.org/1.100.0/alloc/boxed/struct.Box.html) has a few special features that Rust doesn’t currently allow for user defined types.

[lang-types.box.deref](#r-lang-types.box.deref)

- The [dereference operator](expressions/operator-expr.md#the-dereference-operator) for `Box<T>` produces a place which can be [moved from](expressions.md#r-expr.move.movable-place). This means that the `*` operator and the destructor of `Box<T>` are built-in to the language.

[lang-types.box.receiver](#r-lang-types.box.receiver)

- [Methods](items/associated-items.md#associated-functions-and-methods) can take `Box<Self>` as a receiver.

[lang-types.box.fundamental](#r-lang-types.box.fundamental)

- A trait may be implemented for `Box<T>` in the same crate as `T`, which the [orphan rules](items/implementations.md#trait-implementation-coherence) prevent for other generic types.

[lang-types.rc](#r-lang-types.rc)

## `Rc<T>`

[lang-types.rc.receiver](#r-lang-types.rc.receiver)

[Methods](items/associated-items.md#associated-functions-and-methods) can take [`Rc<Self>`](https://doc.rust-lang.org/1.100.0/alloc/rc/struct.Rc.html) as a receiver.

[lang-types.arc](#r-lang-types.arc)

## `Arc<T>`

[lang-types.arc.receiver](#r-lang-types.arc.receiver)

[Methods](items/associated-items.md#associated-functions-and-methods) can take [`Arc<Self>`](https://doc.rust-lang.org/1.100.0/alloc/sync/struct.Arc.html) as a receiver.

[lang-types.pin](#r-lang-types.pin)

## `Pin<P>`

[lang-types.pin.receiver](#r-lang-types.pin.receiver)

[Methods](items/associated-items.md#associated-functions-and-methods) can take [`Pin<P>`](https://doc.rust-lang.org/1.100.0/core/pin/struct.Pin.html) as a receiver.

[lang-types.unsafe-cell](#r-lang-types.unsafe-cell)

## `UnsafeCell<T>`

[lang-types.unsafe-cell.interior-mut](#r-lang-types.unsafe-cell.interior-mut)

[`std::cell::UnsafeCell<T>`](https://doc.rust-lang.org/1.100.0/core/cell/struct.UnsafeCell.html) is used for [interior mutability](interior-mutability.md). It ensures that the compiler doesn’t perform optimisations that are incorrect for such types.

[lang-types.unsafe-cell.read-only-alloc](#r-lang-types.unsafe-cell.read-only-alloc)

It also ensures that [`static` items](items/static-items.md) which have a type with interior mutability aren’t placed in memory marked as read only.

[lang-types.phantom-data](#r-lang-types.phantom-data)

## `PhantomData<T>`

[`std::marker::PhantomData<T>`](https://doc.rust-lang.org/1.100.0/core/marker/struct.PhantomData.html) is a [zero-sized](glossary.md#r-glossary.zst), minimum alignment, type that is considered to own a `T` for the purposes of [variance](subtyping.md#variance), [drop check](https://doc.rust-lang.org/1.100.0/nomicon/dropck.html), and [auto traits](#auto-traits).

[lang-types.va-list](#r-lang-types.va-list)

## `VaList<'_>`

[`VaList`](https://doc.rust-lang.org/1.100.0/core/ffi/va_list/struct.VaList.html) is used for [C-variadic functions](items/functions.md#r-items.fn.c-variadic).

> ![](../assets/inline/special-types-and-traits-1.svg)Note
>
> [`VaList`](https://doc.rust-lang.org/1.100.0/core/ffi/va_list/struct.VaList.html) is ABI compatible with the C `va_list` type; see [items.fn.c-variadic.abi-compatibility](items/functions.md#r-items.fn.c-variadic.abi-compatibility).

[lang-types.ops](#r-lang-types.ops)

## Operator traits

The traits in [`std::ops`](https://doc.rust-lang.org/1.100.0/core/ops/index.html) and [`std::cmp`](https://doc.rust-lang.org/1.100.0/core/cmp/index.html) are used to overload [operators](expressions/operator-expr.md), [indexing expressions](expressions/array-expr.md#array-and-slice-indexing-expressions), and [call expressions](expressions/call-expr.md).

[lang-types.deref](#r-lang-types.deref)

## `Deref` and `DerefMut`

As well as overloading the unary `*` operator, [`Deref`](https://doc.rust-lang.org/1.100.0/core/ops/deref/trait.Deref.html) and [`DerefMut`](https://doc.rust-lang.org/1.100.0/core/ops/deref/trait.DerefMut.html) are also used in [method resolution](expressions/method-call-expr.md) and [deref coercions](type-coercions.md#coercion-types).

[lang-types.drop](#r-lang-types.drop)

## `Drop`

The [`Drop`](https://doc.rust-lang.org/1.100.0/core/ops/drop/trait.Drop.html) trait provides a [destructor](destructors.md), to be run whenever a value of this type is to be destroyed.

[lang-types.copy](#r-lang-types.copy)

## `Copy`

[lang-types.copy.intro](#r-lang-types.copy.intro)

The [`Copy`](https://doc.rust-lang.org/1.100.0/core/marker/trait.Copy.html) trait changes the semantics of a type implementing it.

[lang-types.copy.behavior](#r-lang-types.copy.behavior)

Values whose type implements `Copy` are copied rather than moved upon assignment.

[lang-types.copy.constraint](#r-lang-types.copy.constraint)\
Tests Tests with this rule:

- [tests/ui/union/union-copy.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/union-copy.rs)

`Copy` can only be implemented for types which do not implement `Drop`, and whose fields are all `Copy`. For enums, this means all fields of all variants have to be `Copy`. For unions, this means all variants have to be `Copy`.

[lang-types.copy.builtin-types](#r-lang-types.copy.builtin-types)

`Copy` is implemented by the compiler for

[lang-types.copy.tuple](#r-lang-types.copy.tuple)

- [Tuples](types/tuple.md) of `Copy` types

[lang-types.copy.fn-pointer](#r-lang-types.copy.fn-pointer)

- [Function pointers](types/function-pointer.md)

[lang-types.copy.fn-item](#r-lang-types.copy.fn-item)

- [Function items](types/function-item.md)

[lang-types.copy.closure](#r-lang-types.copy.closure)

- [Closures](types/closure.md) that capture no values or that only capture values of `Copy` types

[lang-types.clone](#r-lang-types.clone)

## `Clone`

[lang-types.clone.intro](#r-lang-types.clone.intro)

The [`Clone`](https://doc.rust-lang.org/1.100.0/core/clone/trait.Clone.html) trait is a supertrait of `Copy`, so it also needs compiler generated implementations.

[lang-types.clone.builtin-types](#r-lang-types.clone.builtin-types)

It is implemented by the compiler for the following types:

[lang-types.clone.builtin-copy](#r-lang-types.clone.builtin-copy)

- Types with a built-in `Copy` implementation (see above)

[lang-types.clone.tuple](#r-lang-types.clone.tuple)

- [Tuples](types/tuple.md) of `Clone` types

[lang-types.clone.closure](#r-lang-types.clone.closure)

- [Closures](types/closure.md) that only capture values of `Clone` types or capture no values from the environment

[lang-types.send](#r-lang-types.send)

## `Send`

The [`Send`](https://doc.rust-lang.org/1.100.0/core/marker/trait.Send.html) trait indicates that a value of this type is safe to send from one thread to another.

[lang-types.sync](#r-lang-types.sync)

## `Sync`

[lang-types.sync.intro](#r-lang-types.sync.intro)

The [`Sync`](https://doc.rust-lang.org/1.100.0/core/marker/trait.Sync.html) trait indicates that a value of this type is safe to share between multiple threads.

[lang-types.sync.static-constraint](#r-lang-types.sync.static-constraint)

This trait must be implemented for all types used in immutable [`static` items](items/static-items.md).

[lang-types.termination](#r-lang-types.termination)

## `Termination`

The [`Termination`](https://doc.rust-lang.org/1.100.0/std/process/trait.Termination.html) trait indicates the acceptable return types for the [main function](crates-and-source-files.md#main-functions) and [test functions](attributes/testing.md#the-test-attribute).

[lang-types.auto-traits](#r-lang-types.auto-traits)

## Auto traits

[lang-types.auto-traits.intro](#r-lang-types.auto-traits.intro)

The [`Send`](https://doc.rust-lang.org/1.100.0/core/marker/trait.Send.html), [`Sync`](https://doc.rust-lang.org/1.100.0/core/marker/trait.Sync.html), [`Unpin`](https://doc.rust-lang.org/1.100.0/core/marker/trait.Unpin.html), [`UnwindSafe`](https://doc.rust-lang.org/1.100.0/core/panic/unwind_safe/trait.UnwindSafe.html), and [`RefUnwindSafe`](https://doc.rust-lang.org/1.100.0/core/panic/unwind_safe/trait.RefUnwindSafe.html) traits are *auto traits*. Auto traits have special properties.

[lang-types.auto-traits.auto-impl](#r-lang-types.auto-traits.auto-impl)

If no explicit implementation or negative implementation is written out for an auto trait for a given type, then the compiler implements it automatically according to the following rules:

[lang-types.auto-traits.builtin-composite](#r-lang-types.auto-traits.builtin-composite)

- `&T`, `&mut T`, `*const T`, `*mut T`, `[T; n]`, and `[T]` implement the trait if `T` does.

[lang-types.auto-traits.fn-item-pointer](#r-lang-types.auto-traits.fn-item-pointer)

- Function item types and function pointers automatically implement the trait.

[lang-types.auto-traits.aggregate](#r-lang-types.auto-traits.aggregate)

- Structs, enums, unions, and tuples implement the trait if all of their fields do.

[lang-types.auto-traits.closure](#r-lang-types.auto-traits.closure)

- Closures implement the trait if the types of all of their captures do. A closure that captures a `T` by shared reference and a `U` by value implements any auto traits that both `&T` and `U` do.

[lang-types.auto-traits.generic-impl](#r-lang-types.auto-traits.generic-impl)

For generic types (counting the built-in types above as generic over `T`), if a generic implementation is available, then the compiler does not automatically implement it for types that could use the implementation except that they do not meet the requisite trait bounds. For instance, the standard library implements `Send` for all `&T` where `T` is `Sync`; this means that the compiler will not implement `Send` for `&T` if `T` is `Send` but not `Sync`.

[lang-types.auto-traits.negative](#r-lang-types.auto-traits.negative)

Auto traits can also have negative implementations, shown as `impl !AutoTrait for T` in the standard library documentation, that override the automatic implementations. For example `*mut T` has a negative implementation of `Send`, and so `*mut T` is not `Send`, even if `T` is. There is currently no stable way to specify additional negative implementations; they exist only in the standard library.

[lang-types.auto-traits.trait-object-marker](#r-lang-types.auto-traits.trait-object-marker)

Auto traits may be added as an additional bound to any [trait object](types/trait-object.md), even though normally only one trait is allowed. For instance, `Box<dyn Debug + Send + UnwindSafe>` is a valid type.

[lang-types.sized](#r-lang-types.sized)

## `Sized`

[lang-types.sized.intro](#r-lang-types.sized.intro)

The [`Sized`](https://doc.rust-lang.org/1.100.0/core/marker/trait.Sized.html) trait indicates that the size of this type is known at compile-time; that is, it’s not a [dynamically sized type](dynamically-sized-types.md).

[lang-types.sized.implicit-sized](#r-lang-types.sized.implicit-sized)

[Type parameters](types/parameters.md) (except `Self` in traits) are `Sized` by default, as are [associated types](items/associated-items.md#associated-types).

[lang-types.sized.implicit-impl](#r-lang-types.sized.implicit-impl)

`Sized` is always implemented automatically by the compiler, not by [implementation items](items/implementations.md).

[lang-types.sized.relaxation](#r-lang-types.sized.relaxation)

These implicit `Sized` bounds may be relaxed by using the special `?Sized` bound.
