# Unsafety

[safety](#r-safety)

# Unsafety

[safety.intro](#r-safety.intro)

Unsafe operations are those that can potentially violate the memory-safety guarantees of Rust’s static semantics.

[safety.unsafe-ops](#r-safety.unsafe-ops)

The following language level features cannot be used in the safe subset of Rust:

[safety.unsafe-deref](#r-safety.unsafe-deref)

- Dereferencing a [raw pointer](types/pointer.md).

[safety.unsafe-static](#r-safety.unsafe-static)

- Reading or writing a [mutable](items/static-items.md#mutable-statics) or unsafe [external](items/external-blocks.md) static variable.

[safety.unsafe-union-access](#r-safety.unsafe-union-access)\
Tests Tests with this rule:

- [tests/ui/union/union-unsafe.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/union-unsafe.rs)

- Accessing a field of a [`union`](items/unions.md), other than to assign to it.

[safety.unsafe-call](#r-safety.unsafe-call)

- Calling an unsafe function.

[safety.unsafe-target-feature-call](#r-safety.unsafe-target-feature-call)

- Calling a safe function marked with a [`target_feature`](attributes/codegen.md#r-attributes.codegen.target_feature) from a function that does not have a `target_feature` attribute enabling the same features (see [attributes.codegen.target_feature.safety-restrictions](attributes/codegen.md#r-attributes.codegen.target_feature.safety-restrictions)).

[safety.unsafe-impl](#r-safety.unsafe-impl)

- Implementing an [unsafe trait](items/traits.md#unsafe-traits).

[safety.unsafe-extern](#r-safety.unsafe-extern)

- Declaring an [`extern`](items/external-blocks.md) block[1](#footnote-extern-2024).

[safety.unsafe-attribute](#r-safety.unsafe-attribute)

- Applying an [unsafe attribute](attributes.md) to an item.

------------------------------------------------------------------------

1.  

    Prior to the 2024 edition, extern blocks were allowed to be declared without `unsafe`. [↩](#fr-extern-2024-1)
