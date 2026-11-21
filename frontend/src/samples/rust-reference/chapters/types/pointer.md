# Pointer types

[type.pointer](#r-type.pointer)

# Pointer types

[type.pointer.intro](#r-type.pointer.intro)

All pointers are explicit first-class values. They can be moved or copied, stored into data structs, and returned from functions.

[type.pointer.reference](#r-type.pointer.reference)

## References (`&` and `&mut`)

[type.pointer.reference.syntax](#r-type.pointer.reference.syntax)

**Syntax**\
[ReferenceType](#railroad-ReferenceType) → & [Lifetime](../trait-bounds.md#grammar-Lifetime)? mut? [TypeNoBounds](../types.md#grammar-TypeNoBounds)

Show Railroad

![](../../assets/inline/pointer-1.svg) [type.pointer.reference.shared](#r-type.pointer.reference.shared)

### Shared references (`&`)

[type.pointer.reference.shared.intro](#r-type.pointer.reference.shared.intro)

Shared references point to memory which is owned by some other value.

[type.pointer.reference.shared.constraint-mutation](#r-type.pointer.reference.shared.constraint-mutation)

When a shared reference to a value is created, it prevents direct mutation of the value. [Interior mutability](../interior-mutability.md) provides an exception for this in certain circumstances. As the name suggests, any number of shared references to a value may exist. A shared reference type is written `&type`, or `&'a type` when you need to specify an explicit lifetime.

[type.pointer.reference.shared.copy](#r-type.pointer.reference.shared.copy)

Copying a reference is a “shallow” operation: it involves only copying the pointer itself, that is, pointers are `Copy`. Releasing a reference has no effect on the value it points to, but referencing of a [temporary value](../expressions.md#temporaries) will keep it alive during the scope of the reference itself.

[type.pointer.reference.mut](#r-type.pointer.reference.mut)

### Mutable references (`&mut`)

[type.pointer.reference.mut.intro](#r-type.pointer.reference.mut.intro)

Mutable references point to memory which is owned by some other value. A mutable reference type is written `&mut type` or `&'a mut type`.

[type.pointer.reference.mut.copy](#r-type.pointer.reference.mut.copy)

A mutable reference (that hasn’t been borrowed) is the only way to access the value it points to, so is not `Copy`.

[type.pointer.raw](#r-type.pointer.raw)

## Raw pointers (`*const` and `*mut`)

[type.pointer.raw.syntax](#r-type.pointer.raw.syntax)

**Syntax**\
[RawPointerType](#railroad-RawPointerType) → \* ( mut \| const ) [TypeNoBounds](../types.md#grammar-TypeNoBounds)

Show Railroad

![](../../assets/inline/pointer-2.svg) [type.pointer.raw.intro](#r-type.pointer.raw.intro)

Raw pointers are pointers without safety or liveness guarantees. Raw pointers are written as `*const T` or `*mut T`. For example `*const i32` means a raw pointer to a 32-bit integer.

[type.pointer.raw.copy](#r-type.pointer.raw.copy)

Copying or dropping a raw pointer has no effect on the lifecycle of any other value.

[type.pointer.raw.safety](#r-type.pointer.raw.safety)

Dereferencing a raw pointer is an [`unsafe` operation](../unsafety.md).

This can also be used to convert a raw pointer to a reference by reborrowing it (`&*` or `&mut *`). Raw pointers are generally discouraged; they exist to support interoperability with foreign code, and writing performance-critical or low-level functions.

[type.pointer.raw.cmp](#r-type.pointer.raw.cmp)

When comparing raw pointers they are compared by their address, rather than by what they point to. When comparing raw pointers to [dynamically sized types](../dynamically-sized-types.md) they also have their additional data compared.

[type.pointer.raw.constructor](#r-type.pointer.raw.constructor)

Raw pointers can be created directly using `&raw const` for `*const` pointers and `&raw mut` for `*mut` pointers.

[type.pointer.smart](#r-type.pointer.smart)

## Smart pointers

The standard library contains additional ‘smart pointer’ types beyond references and raw pointers.

[type.pointer.validity](#r-type.pointer.validity)

## Bit validity

[type.pointer.validity.pointer-fragment](#r-type.pointer.validity.pointer-fragment)

Despite pointers and references being similar to `usize`s in the machine code emitted on most platforms, the semantics of transmuting a reference or pointer type to a non-pointer type is currently undecided. Thus, it may not be valid to transmute a pointer or reference type, `P`, to a `[u8; size_of::<P>()]`.

[type.pointer.validity.raw](#r-type.pointer.validity.raw)

For thin raw pointers (i.e., for `P = *const T` or `P = *mut T` for `T: Sized`), the inverse direction (transmuting from an integer or array of integers to `P`) is always valid. However, the pointer produced via such a transmutation may not be dereferenced (not even if `T` has [size zero](../glossary.md#r-glossary.zst)).
