# Unions

[items.union](#r-items.union)

# Unions

[items.union.syntax](#r-items.union.syntax)

**Syntax**\
[Union](#railroad-Union) →\
    union [IDENTIFIER](../identifiers.md#grammar-IDENTIFIER) [GenericParams](generics.md#grammar-GenericParams)? [WhereClause](generics.md#grammar-WhereClause)? { [StructFields](structs.md#grammar-StructFields)? }

Show Railroad

![](../../assets/inline/unions-1.svg) [items.union.intro](#r-items.union.intro)

A union declaration uses the same syntax as a struct declaration, except with `union` in place of `struct`.

[items.union.namespace](#r-items.union.namespace)

A union declaration defines the given name in the [type namespace](../names/namespaces.md) of the module or block where it is located.

``` playground
#![allow(unused)]
fn main() {
#[repr(C)]
union MyUnion {
    f1: u32,
    f2: f32,
}
}
```

[items.union.common-storage](#r-items.union.common-storage)\
Tests Tests with this rule:

- [tests/ui/union/union-overwrite.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/union-overwrite.rs)
- [tests/ui/union/union-transmute.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/union-transmute.rs)

The key property of unions is that all fields of a union share common storage. As a result, writes to one field of a union can overwrite its other fields, and size of a union is determined by the size of its largest field.

[items.union.field-restrictions](#r-items.union.field-restrictions)\
Tests Tests with this rule:

- [tests/ui/union/field_checks.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/field_checks.rs)
- [tests/ui/union/issue-41073.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/issue-41073.rs)
- [tests/ui/union/issue-81199.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/issue-81199.rs)
- [tests/ui/union/union-with-drop-fields.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/union-with-drop-fields.rs)

Union field types are restricted to the following subset of types:

[items.union.field-copy](#r-items.union.field-copy)\
Tests Tests with this rule:

- [tests/ui/union/field_checks.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/field_checks.rs)
- [tests/ui/union/issue-99375.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/issue-99375.rs)
- [tests/ui/union/union-manuallydrop-rpass.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/union-manuallydrop-rpass.rs)

- `Copy` types

[items.union.field-references](#r-items.union.field-references)\
Tests Tests with this rule:

- [tests/ui/union/field_checks.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/field_checks.rs)

- References (`&T` and `&mut T` for arbitrary `T`)

[items.union.field-manually-drop](#r-items.union.field-manually-drop)\
Tests Tests with this rule:

- [tests/ui/union/field_checks.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/field_checks.rs)
- [tests/ui/union/union-manuallydrop-rpass.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/union-manuallydrop-rpass.rs)

- `ManuallyDrop<T>` (for arbitrary `T`)

[items.union.field-tuple](#r-items.union.field-tuple)\
Tests Tests with this rule:

- [tests/ui/union/field_checks.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/field_checks.rs)
- [tests/ui/union/issue-99375.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/issue-99375.rs)

- Tuples and arrays containing only allowed union field types

[items.union.drop](#r-items.union.drop)\
Tests Tests with this rule:

- [tests/ui/union/field_checks.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/field_checks.rs)
- [tests/ui/union/union-drop-assign.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/union-drop-assign.rs)
- [tests/ui/union/union-drop.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/union-drop.rs)

This restriction ensures, in particular, that union fields never need to be dropped. Like for structs and enums, it is possible to `impl Drop` for a union to manually define what happens when it gets dropped.

[items.union.fieldless](#r-items.union.fieldless)\
Tests Tests with this rule:

- [tests/ui/union/union-empty.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/union-empty.rs)

Unions without any fields are not accepted by the compiler, but can be accepted by macros.

[items.union.init](#r-items.union.init)

## Initialization of a union

[items.union.init.intro](#r-items.union.init.intro)\
Tests Tests with this rule:

- [tests/ui/union/union-fields-2.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/union-fields-2.rs)

A value of a union type can be created using the same syntax that is used for struct types, except that it must specify exactly one field:

``` playground
#![allow(unused)]
fn main() {
union MyUnion { f1: u32, f2: f32 }

let u = MyUnion { f1: 1 };
}
```

[items.union.init.result](#r-items.union.init.result)

The expression above creates a value of type `MyUnion` and initializes the storage using field `f1`. The union can be accessed using the same syntax as struct fields:

``` playground
#![allow(unused)]
fn main() {
union MyUnion { f1: u32, f2: f32 }

let u = MyUnion { f1: 1 };
let f = unsafe { u.f1 };
}
```

[items.union.fields](#r-items.union.fields)

## Reading and writing union fields

[items.union.fields.intro](#r-items.union.fields.intro)

Unions have no notion of an “active field”. Instead, every union access just interprets the storage as the type of the field used for the access.

[items.union.fields.read](#r-items.union.fields.read)\
Tests Tests with this rule:

- [tests/ui/union/union-overwrite.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/union-overwrite.rs)

Reading a union field reads the bits of the union at the field’s type.

[items.union.fields.offset](#r-items.union.fields.offset)

Fields might have a non-zero offset (except when [the C representation](../type-layout.md#reprc-unions) is used); in that case the bits starting at the offset of the fields are read.

[items.union.fields.validity](#r-items.union.fields.validity)

It is the programmer’s responsibility to make sure that the data is valid at the field’s type. Failing to do so results in [undefined behavior](../behavior-considered-undefined.md). For example, reading the value `3` from a field of the [boolean type](../types/boolean.md) is undefined behavior. Effectively, writing to and then reading from a union with [the C representation](../type-layout.md#reprc-unions) is analogous to a [`transmute`](../../core/intrinsics/fn.transmute.html) from the type used for writing to the type used for reading.

[items.union.fields.read-safety](#r-items.union.fields.read-safety)\
Tests Tests with this rule:

- [tests/ui/union/access_union_field.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/access_union_field.rs)
- [tests/ui/union/union-assignop.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/union-assignop.rs)
- [tests/ui/union/union-unsafe.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/union-unsafe.rs)

Consequently, all reads of union fields have to be placed in `unsafe` blocks:

``` playground
#![allow(unused)]
fn main() {
union MyUnion { f1: u32, f2: f32 }
let u = MyUnion { f1: 1 };

unsafe {
    let f = u.f1;
}
}
```

Commonly, code using unions will provide safe wrappers around unsafe union field accesses.

[items.union.fields.write-safety](#r-items.union.fields.write-safety)\
Tests Tests with this rule:

- [tests/ui/union/union-assignop.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/union-assignop.rs)
- [tests/ui/union/union-unsafe.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/union-unsafe.rs)

In contrast, writes to union fields are safe, since they just overwrite arbitrary data, but cannot cause undefined behavior. (Note that union field types can never have drop glue, so a union field write will never implicitly drop anything.)

[items.union.pattern](#r-items.union.pattern)

## Pattern matching on unions

[items.union.pattern.intro](#r-items.union.pattern.intro)\
Tests Tests with this rule:

- [tests/ui/union/union-pat-refutability.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/union-pat-refutability.rs)

Another way to access union fields is to use pattern matching.

[items.union.pattern.one-field](#r-items.union.pattern.one-field)\
Tests Tests with this rule:

- [tests/ui/union/union-fields-2.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/union-fields-2.rs)

Pattern matching on union fields uses the same syntax as struct patterns, except that the pattern must specify exactly one field.

[items.union.pattern.safety](#r-items.union.pattern.safety)\
Tests Tests with this rule:

- [tests/ui/union/union-pat-in-param.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/union-pat-in-param.rs)
- [tests/ui/union/union-pat-refutability.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/union-pat-refutability.rs)
- [tests/ui/union/union-unsafe.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/union-unsafe.rs)
- [tests/ui/union/union_destructure.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/union_destructure.rs)

Since pattern matching is like reading the union with a particular field, it has to be placed in `unsafe` blocks as well.

``` playground
#![allow(unused)]
fn main() {
union MyUnion { f1: u32, f2: f32 }

fn f(u: MyUnion) {
    unsafe {
        match u {
            MyUnion { f1: 10 } => { println!("ten"); }
            MyUnion { f2 } => { println!("{}", f2); }
        }
    }
}
}
```

> ![](../../assets/inline/unions-2.svg)Warning
>
> The order in which the subpatterns of a pattern are tested is not specified. A union field named in a pattern may be read even when the pattern as a whole does not match. Reading a union field is undefined behavior unless it holds a valid value of its type (see [items.union.fields.validity](#r-items.union.fields.validity)). Nothing else in the pattern can be relied on to prevent the read.
>
> In particular, when implementing a C-style tagged union, avoid matching the tag and the corresponding union field within a single pattern: the union field may be read even when the tag does not match.
>
> To read a union field only when a condition holds, test the condition and read the field in separate steps whose evaluation order is specified. For a C tagged union, match on the tag first and read the union field within the matched arm:
>
> ``` playground
> #![allow(unused)]
> fn main() {
> #[repr(u32)]
> enum Tag { I, F }
>
> #[repr(C)]
> union U {
>     i: i32,
>     f: f32,
> }
>
> #[repr(C)]
> struct Value {
>     tag: Tag,
>     u: U,
> }
>
> fn is_zero(v: Value) -> bool {
>     match v.tag {
>         Tag::I => unsafe { v.u.i == 0 },
>         Tag::F => unsafe { v.u.f == 0.0 },
>     }
> }
> }
> ```

[items.union.ref](#r-items.union.ref)

## References to union fields

[items.union.ref.intro](#r-items.union.ref.intro)

Since union fields share common storage, gaining write access to one field of a union can give write access to all its remaining fields.

[items.union.ref.borrow](#r-items.union.ref.borrow)\
Tests Tests with this rule:

- [tests/ui/union/union-borrow-move-parent-sibling.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/union-borrow-move-parent-sibling.rs)

Borrow checking rules have to be adjusted to account for this fact. As a result, if one field of a union is borrowed, all its remaining fields are borrowed as well for the same lifetime.

``` playground
#![allow(unused)]
fn main() {
union MyUnion { f1: u32, f2: f32 }
// ERROR: cannot borrow `u` (via `u.f2`) as mutable more than once at a time
fn test() {
    let mut u = MyUnion { f1: 1 };
    unsafe {
        let b1 = &mut u.f1;
//                    ---- first mutable borrow occurs here (via `u.f1`)
        let b2 = &mut u.f2;
//                    ^^^^ second mutable borrow occurs here (via `u.f2`)
        *b1 = 5;
    }
//  - first borrow ends here
    assert_eq!(unsafe { u.f1 }, 5);
}
}
```

[items.union.ref.use](#r-items.union.ref.use)\
Tests Tests with this rule:

- [tests/ui/union/union-trait-impl.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/union-trait-impl.rs)

As you could see, in many aspects (except for layouts, safety, and ownership) unions behave exactly like structs, largely as a consequence of inheriting their syntactic shape from structs. This is also true for many unmentioned aspects of Rust language (such as privacy, name resolution, type inference, generics, trait implementations, inherent implementations, coherence, pattern checking, etc etc etc).
