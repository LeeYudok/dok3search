# Union types

[type.union](#r-type.union)

# Union types

[type.union.intro](#r-type.union.intro)

A *union type* is a nominal, heterogeneous C-like union, denoted by the name of a [`union` item](../items/unions.md).

[type.union.access](#r-type.union.access)\
Tests Tests with this rule:

- [tests/ui/union/union-transmute.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/union-transmute.rs)

Unions have no notion of an “active field”. Instead, every union access transmutes parts of the content of the union to the type of the accessed field.

[type.union.safety](#r-type.union.safety)\
Tests Tests with this rule:

- [tests/ui/union/access_union_field.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/access_union_field.rs)
- [tests/ui/union/union-unsafe.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/union-unsafe.rs)

Since transmutes can cause unexpected or undefined behaviour, `unsafe` is required to read from a union field.

[type.union.constraint](#r-type.union.constraint)\
Tests Tests with this rule:

- [tests/ui/union/field_checks.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/field_checks.rs)
- [tests/ui/union/issue-41073.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/issue-41073.rs)
- [tests/ui/union/issue-81199.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/issue-81199.rs)

Union field types are also restricted to a subset of types which ensures that they never need dropping. See the [item](../items/unions.md) documentation for further details.

[type.union.layout](#r-type.union.layout)

The memory layout of a `union` is undefined by default (in particular, fields do *not* have to be at offset 0), but the `#[repr(...)]` attribute can be used to fix a layout.
