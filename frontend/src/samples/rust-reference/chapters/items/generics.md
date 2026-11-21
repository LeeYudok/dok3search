# Generic parameters

[items.generics](#r-items.generics)

# Generic parameters

[items.generics.syntax](#r-items.generics.syntax)

**Syntax**\
[GenericParams](#railroad-GenericParams) → \< ( [GenericParam](#grammar-GenericParam) ( , [GenericParam](#grammar-GenericParam) )\* ,? )? \>

[GenericParam](#railroad-GenericParam) → [OuterAttribute](../attributes.md#grammar-OuterAttribute)\* ( [LifetimeParam](#grammar-LifetimeParam) \| [TypeParam](#grammar-TypeParam) \| [ConstParam](#grammar-ConstParam) )

[LifetimeParam](#railroad-LifetimeParam) → [Lifetime](../trait-bounds.md#grammar-Lifetime) ( : [LifetimeBounds](../trait-bounds.md#grammar-LifetimeBounds)? )?

[TypeParam](#railroad-TypeParam) → [IDENTIFIER](../identifiers.md#grammar-IDENTIFIER) ( : [Bounds](../trait-bounds.md#grammar-Bounds)? )? ( = [Type](../types.md#grammar-Type) )?

[ConstParam](#railroad-ConstParam) →\
    const [IDENTIFIER](../identifiers.md#grammar-IDENTIFIER) : [Type](../types.md#grammar-Type)\
    ( = ( [BlockExpression](../expressions/block-expr.md#grammar-BlockExpression) \| [IDENTIFIER](../identifiers.md#grammar-IDENTIFIER) \| -? [LiteralExpression](../expressions/literal-expr.md#grammar-LiteralExpression) ) )?

Show Railroad

![](../../assets/inline/generics-1.svg) ![](../../assets/inline/generics-2.svg) ![](../../assets/inline/generics-3.svg) ![](../../assets/inline/generics-4.svg) ![](../../assets/inline/generics-5.svg) [items.generics.intro](#r-items.generics.intro)

[Functions](functions.md), [type aliases](type-aliases.md), [structs](structs.md), [enumerations](enumerations.md), [unions](unions.md), [traits](traits.md), and [implementations](implementations.md) may be *parameterized* by types, constants, and lifetimes. These parameters are listed in angle brackets (`<...>`), usually immediately after the name of the item and before its definition. For implementations, which don’t have a name, they come directly after `impl`.

[items.generics.decl-order](#r-items.generics.decl-order)

The order of generic parameters is restricted to lifetime parameters and then type and const parameters intermixed.

[items.generics.duplicate-params](#r-items.generics.duplicate-params)

The same parameter name may not be declared more than once in a [GenericParams](#grammar-GenericParams) list.

Some examples of items with type, const, and lifetime parameters:

``` playground
#![allow(unused)]
fn main() {
fn foo<'a, T>() {}
trait A<U> {}
struct Ref<'a, T> where T: 'a { r: &'a T }
struct InnerArray<T, const N: usize>([T; N]);
struct EitherOrderWorks<const N: bool, U>(U);
}
```

[items.generics.scope](#r-items.generics.scope)

Generic parameters are in scope within the item definition where they are declared. They are not in scope for items declared within the body of a function as described in [item declarations](../statements.md#item-declarations). See [generic parameter scopes](../names/scopes.md#generic-parameter-scopes) for more details.

[items.generics.builtin-generic-types](#r-items.generics.builtin-generic-types)

[References](../types/pointer.md#shared-references-), [raw pointers](../types/pointer.md#raw-pointers-const-and-mut), [arrays](../types/array.md), [slices](../types/slice.md), [tuples](../types/tuple.md), and [function pointers](../types/function-pointer.md) have lifetime or type parameters as well, but are not referred to with path syntax.

[items.generics.invalid-lifetimes](#r-items.generics.invalid-lifetimes)

`'_` and `'static` are not valid lifetime parameter names.

[items.generics.const](#r-items.generics.const)

### Const generics

[items.generics.const.intro](#r-items.generics.const.intro)

*Const generic parameters* allow items to be generic over constant values.

[items.generics.const.namespace](#r-items.generics.const.namespace)

The const identifier introduces a name in the [value namespace](../names/namespaces.md) for the constant parameter, and all instances of the item must be instantiated with a value of the given type.

[items.generics.const.allowed-types](#r-items.generics.const.allowed-types)

The only allowed types of const parameters are `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `char` and `bool`.

[items.generics.const.use](#r-items.generics.const.use)

Const parameters can be used anywhere a [const item](constant-items.md) can be used, with the exception that when used in a [type](../types.md) or [array repeat expression](../expressions/array-expr.md), it must be standalone (as described below). That is, they are allowed in the following places:

1.  As an applied const to any type which forms a part of the signature of the item in question.
2.  As part of a const expression used to define an [associated const](associated-items.md#associated-constants), or as a parameter to an [associated type](associated-items.md#associated-types).
3.  As a value in any runtime expression in the body of any functions in the item.
4.  As a parameter to any type used in the body of any functions in the item.
5.  As a part of the type of any fields in the item.

``` playground
#![allow(unused)]
fn main() {
// Examples where const generic parameters can be used.

// Used in the signature of the item itself.
fn foo<const N: usize>(arr: [i32; N]) {
    // Used as a type within a function body.
    let x: [i32; N];
    // Used as an expression.
    println!("{}", N * 2);
}

// Used as a field of a struct.
struct Foo<const N: usize>([i32; N]);

impl<const N: usize> Foo<N> {
    // Used as an associated constant.
    const CONST: usize = N * 4;
}

trait Trait {
    type Output;
}

impl<const N: usize> Trait for Foo<N> {
    // Used as an associated type.
    type Output = [i32; N];
}
}
```

``` playground
#![allow(unused)]
fn main() {
// Examples where const generic parameters cannot be used.
fn foo<const N: usize>() {
    // Cannot use in item definitions within a function body.
    const BAD_CONST: [usize; N] = [1; N];
    static BAD_STATIC: [usize; N] = [1; N];
    fn inner(bad_arg: [usize; N]) {
        let bad_value = N * 2;
    }
    type BadAlias = [usize; N];
    struct BadStruct([usize; N]);
}
}
```

[items.generics.const.standalone](#r-items.generics.const.standalone)

As a further restriction, const parameters may only appear as a standalone argument inside of a [type](../types.md) or [array repeat expression](../expressions/array-expr.md). In those contexts, they may only be used as a single segment [path expression](../expressions/path-expr.md), possibly inside a [block](../expressions/block-expr.md) (such as `N` or `{N}`). That is, they cannot be combined with other expressions.

``` playground
#![allow(unused)]
fn main() {
// Examples where const parameters may not be used.

// Not allowed to combine in other expressions in types, such as the
// arithmetic expression in the return type here.
fn bad_function<const N: usize>() -> [u8; {N + 1}] {
    // Similarly not allowed for array repeat expressions.
    [1; {N + 1}]
}
}
```

[items.generics.const.argument](#r-items.generics.const.argument)

A const argument in a [path](../paths.md) specifies the const value to use for that item.

[items.generics.const.argument-const-expr](#r-items.generics.const.argument-const-expr)

The argument must either be an [inferred const](#r-items.generics.const.inferred) or be a [const expression](../const_eval.md#constant-expressions) of the type ascribed to the const parameter. The const expression must be a [block expression](../expressions/block-expr.md) (surrounded with braces) unless it is a single path segment (an [IDENTIFIER](../identifiers.md#grammar-IDENTIFIER)) or a [literal](../expressions/literal-expr.md) (with a possibly leading `-` token).

> ![](../../assets/inline/generics-6.svg)Note
>
> This syntactic restriction is necessary to avoid requiring infinite lookahead when parsing an expression inside of a type.

``` playground
#![allow(unused)]
fn main() {
struct S<const N: i64>;
const C: i64 = 1;
fn f<const N: i64>() -> S<N> { S }

let _ = f::<1>(); // Literal.
let _ = f::<-1>(); // Negative literal.
let _ = f::<{ 1 + 2 }>(); // Constant expression.
let _ = f::<C>(); // Single segment path.
let _ = f::<{ C + 1 }>(); // Constant expression.
let _: S<1> = f::<_>(); // Inferred const.
let _: S<1> = f::<(((_)))>(); // Inferred const.
}
```

> ![](../../assets/inline/generics-7.svg)Note
>
> In a generic argument list, an [inferred const](#r-items.generics.const.inferred) is parsed as an [inferred type](../types/inferred.md#grammar-InferredType) but then semantically treated as a separate kind of [const generic argument](#r-items.generics.const.argument).

[items.generics.const.inferred](#r-items.generics.const.inferred)\
Tests Tests with this rule:

- [tests/ui/const-generics/generic_arg_infer/paren_infer.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/const-generics/generic_arg_infer/paren_infer.rs)

Where a const argument is expected, an `_` (optionally surrounded by any number of matching parentheses), called the *inferred const* ([path rules](../paths.md#r-paths.expr.complex-const-params), [array expression rules](../expressions/array-expr.md#r-expr.array.length-restriction)), can be used instead. This asks the compiler to infer the const argument if possible based on surrounding information.

``` playground
#![allow(unused)]
fn main() {
fn make_buf<const N: usize>() -> [u8; N] {
    [0; _]
    //  ^ Infers `N`.
}
let _: [u8; 1024] = make_buf::<_>();
//                             ^ Infers `1024`.
}
```

> ![](../../assets/inline/generics-8.svg)Note
>
> An [inferred const](#r-items.generics.const.inferred) is not semantically an [expression](../expressions.md#grammar-Expression) and so is not accepted within braces.
>
> ``` playground
> #![allow(unused)]
> fn main() {
> fn f() -> [u8; N] { [0; _] }
> let _: [_; 1] = f::<{ _ }>();
> //                    ^ ERROR `_` not allowed here
> }
> ```

[items.generics.const.inferred-constraint](#r-items.generics.const.inferred-constraint)

The inferred const cannot be used in item signatures.

``` playground
#![allow(unused)]
fn main() {
fn f<const N: usize>(x: [u8; N]) -> [u8; _] { x }
//                                       ^ ERROR not allowed
}
```

[items.generics.const.type-ambiguity](#r-items.generics.const.type-ambiguity)

When there is ambiguity if a generic argument could be resolved as either a type or const argument, it is always resolved as a type. Placing the argument in a block expression can force it to be interpreted as a const argument.

``` playground
#![allow(unused)]
fn main() {
type N = u32;
struct Foo<const N: usize>;
// The following is an error, because `N` is interpreted as the type alias `N`.
fn foo<const N: usize>() -> Foo<N> { todo!() } // ERROR
// Can be fixed by wrapping in braces to force it to be interpreted as the `N`
// const parameter:
fn bar<const N: usize>() -> Foo<{ N }> { todo!() } // ok
}
```

[items.generics.const.variance](#r-items.generics.const.variance)

Unlike type and lifetime parameters, const parameters can be declared without being used inside of a parameterized item, with the exception of implementations as described in [generic implementations](implementations.md#generic-implementations):

``` playground
#![allow(unused)]
fn main() {
// ok
struct Foo<const N: usize>;
enum Bar<const M: usize> { A, B }

// ERROR: unused parameter
struct Baz<T>;
struct Biz<'a>;
struct Unconstrained;
impl<const N: usize> Unconstrained {}
}
```

[items.generics.const.exhaustiveness](#r-items.generics.const.exhaustiveness)

When resolving a trait bound obligation, the exhaustiveness of all implementations of const parameters is not considered when determining if the bound is satisfied. For example, in the following, even though all possible const values for the `bool` type are implemented, it is still an error that the trait bound is not satisfied:

``` playground
#![allow(unused)]
fn main() {
struct Foo<const B: bool>;
trait Bar {}
impl Bar for Foo<true> {}
impl Bar for Foo<false> {}

fn needs_bar(_: impl Bar) {}
fn generic<const B: bool>() {
    let v = Foo::<B>;
    needs_bar(v); // ERROR: trait bound `Foo<B>: Bar` is not satisfied
}
}
```

[items.generics.where](#r-items.generics.where)

## Where clauses

[items.generics.where.syntax](#r-items.generics.where.syntax)

**Syntax**\
[WhereClause](#railroad-WhereClause) → where ( [WhereClauseItem](#grammar-WhereClauseItem) , )\* [WhereClauseItem](#grammar-WhereClauseItem)?

[WhereClauseItem](#railroad-WhereClauseItem) →\
      [LifetimeWhereClauseItem](#grammar-LifetimeWhereClauseItem)\
    \| [TypeBoundWhereClauseItem](#grammar-TypeBoundWhereClauseItem)

[LifetimeWhereClauseItem](#railroad-LifetimeWhereClauseItem) → [Lifetime](../trait-bounds.md#grammar-Lifetime) : [LifetimeBounds](../trait-bounds.md#grammar-LifetimeBounds)?

[TypeBoundWhereClauseItem](#railroad-TypeBoundWhereClauseItem) → [ForLifetimes](../trait-bounds.md#grammar-ForLifetimes)? [Type](../types.md#grammar-Type) : [Bounds](../trait-bounds.md#grammar-Bounds)?

Show Railroad

![](../../assets/inline/generics-9.svg) ![](../../assets/inline/generics-10.svg) ![](../../assets/inline/generics-11.svg) ![](../../assets/inline/generics-12.svg) [items.generics.where.intro](#r-items.generics.where.intro)

*Where clauses* provide another way to specify bounds on type and lifetime parameters as well as a way to specify bounds on types that aren’t type parameters.

[items.generics.where.higher-ranked-lifetimes](#r-items.generics.where.higher-ranked-lifetimes)

The `for` keyword can be used to introduce [higher-ranked lifetimes](../trait-bounds.md#higher-ranked-trait-bounds). It only allows [LifetimeParam](#grammar-LifetimeParam) parameters.

``` playground
#![allow(unused)]
fn main() {
struct A<T>
where
    T: Iterator,            // Could use A<T: Iterator> instead
    T::Item: Copy,          // Bound on an associated type
    String: PartialEq<T>,   // Bound on `String`, using the type parameter
    i32: Default,           // Allowed, but not useful
{
    f: T,
}
}
```

[items.generics.attributes](#r-items.generics.attributes)

## Attributes

Generic lifetime and type parameters allow [attributes](../attributes.md) on them. There are no built-in attributes that do anything in this position, although custom derive attributes may give meaning to it.

This example shows using a custom derive attribute to modify the meaning of a generic parameter.

``` rust
// Assume that the derive for MyFlexibleClone declared `my_flexible_clone` as
// an attribute it understands.
#[derive(MyFlexibleClone)]
struct Foo<#[my_flexible_clone(unbounded)] H> {
    a: *const H
}
```
