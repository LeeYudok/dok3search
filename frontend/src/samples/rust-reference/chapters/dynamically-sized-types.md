# Dynamically sized types

[dynamic-sized](#r-dynamic-sized)

# Dynamically sized types

[dynamic-sized.intro](#r-dynamic-sized.intro)

Most types have a fixed size that is known at compile time and implement the trait [`Sized`](special-types-and-traits.md#sized). A type with a size that is known only at run-time is called a *dynamically sized type* (*DST*) or, informally, an unsized type. [Slices](types/slice.md), [trait objects](types/trait-object.md), and [str](types/str.md) are examples of DSTs.

[dynamic-sized.restriction](#r-dynamic-sized.restriction)

Such types can only be used in certain cases:

[dynamic-sized.pointer-types](#r-dynamic-sized.pointer-types)

- [Pointer types](types/pointer.md) to DSTs are sized but have twice the size of pointers to sized types, since they also store *metadata*:
  - Pointers to slices store the number of elements; pointers to `str` store the length in bytes.
  - Pointers to trait objects store a pointer to a vtable.
  - Pointers to a struct or tuple with an [unsized tail](#r-dynamic-sized.tail) store the same metadata as a pointer to that tail.

[dynamic-sized.question-sized](#r-dynamic-sized.question-sized)

- DSTs can be provided as type arguments to generic type parameters having the special `?Sized` bound. They can also be used for associated type definitions when the corresponding associated type declaration has a `?Sized` bound. By default, any type parameter or associated type has a `Sized` bound, unless it is relaxed using `?Sized`.

[dynamic-sized.trait-impl](#r-dynamic-sized.trait-impl)

- Traits may be implemented for <span class="abbr" title="dynamically sized
  types">DSTs. Unlike with generic type parameters, `Self: ?Sized` is the default in trait definitions.

[dynamic-sized.struct-field](#r-dynamic-sized.struct-field)

- Structs may contain a DST as the last field; this makes the struct itself a DST.

> ![](../assets/inline/dynamically-sized-types-1.svg)Note
>
> [Variables](variables.md), function parameters, [const](items/constant-items.md) items, and [static](items/static-items.md) items must be `Sized`.

[dynamic-sized.tail](#r-dynamic-sized.tail)

The *unsized tail* of a type is the dynamically sized component that the [metadata](#r-dynamic-sized.pointer-types) of a pointer to the type describes. A [slice](types/slice.md) (`[T]`) and a [`str`](types/str.md) are each their own unsized tail, described by a length; a [trait object](types/trait-object.md) (`dyn Trait`) is its own unsized tail, described by a pointer to a vtable. When a struct (per [dynamic-sized.struct-field](#r-dynamic-sized.struct-field)) or a tuple has an unsized last field, its unsized tail is the unsized tail of that field. A sized type has no unsized tail.
