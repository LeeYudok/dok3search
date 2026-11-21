# Method-call expressions

[expr.method](#r-expr.method)

# Method-call expressions

[expr.method.syntax](#r-expr.method.syntax)

**Syntax**\
[MethodCallExpression](#railroad-MethodCallExpression) → [Expression](../expressions.md#grammar-Expression) . [PathExprSegment](../paths.md#grammar-PathExprSegment) ( [CallParams](call-expr.md#grammar-CallParams)? )

Show Railroad

![](../../assets/inline/method-call-expr-1.svg) [expr.method.intro](#r-expr.method.intro)

A *method call* consists of an expression (the *receiver*) followed by a single dot, an expression path segment, and a parenthesized expression-list.

[expr.method.target](#r-expr.method.target)

Method calls are resolved to associated [methods](../items/associated-items.md#methods) on specific traits, either statically dispatching to a method if the exact `self`-type of the left-hand-side is known, or dynamically dispatching if the left-hand-side expression is an indirect [trait object](../types/trait-object.md).

``` playground
#![allow(unused)]
fn main() {
let pi: Result<f32, _> = "3.14".parse();
let log_pi = pi.unwrap_or(1.0).log(2.72);
assert!(1.14 < log_pi && log_pi < 1.15)
}
```

[expr.method.autoref-deref](#r-expr.method.autoref-deref)

When looking up a method call, the receiver may be automatically dereferenced or borrowed in order to call a method. This requires a more complex lookup process than for other functions, since there may be a number of possible methods to call. The following procedure is used:

[expr.method.candidate-receivers](#r-expr.method.candidate-receivers)

The first step is to build a list of candidate receiver types. Obtain these by repeatedly [dereferencing](operator-expr.md#the-dereference-operator) the receiver expression’s type, adding each type encountered to the list, then finally attempting an array [unsized coercion](../type-coercions.md#unsized-coercions) at the end, and adding the result type if that is successful.

[expr.method.candidate-receivers-refs](#r-expr.method.candidate-receivers-refs)

Then, for each candidate `T`, add `&T` and `&mut T` to the list immediately after `T`.

For instance, if the receiver has type `Box<[i32;2]>`, then the candidate types will be `Box<[i32;2]>`, `&Box<[i32;2]>`, `&mut Box<[i32;2]>`, `[i32; 2]` (by dereferencing), `&[i32; 2]`, `&mut [i32; 2]`, `[i32]` (by unsized coercion), `&[i32]`, and finally `&mut [i32]`.

[expr.method.candidate-search](#r-expr.method.candidate-search)

Then, for each candidate type `T`, search for a [visible](../visibility-and-privacy.md) method with a receiver of that type in the following places:

1.  `T`’s inherent methods (methods implemented directly on `T`).
2.  Any of the methods provided by a [visible](../visibility-and-privacy.md) trait implemented by `T`. If `T` is a type parameter, methods provided by trait bounds on `T` are looked up first. Then all remaining methods in scope are looked up.

> ![](../../assets/inline/method-call-expr-2.svg)Note
>
> The lookup is done for each type in order, which can occasionally lead to surprising results. The below code will print “In trait impl!”, because `&self` methods are looked up first, the trait method is found before the struct’s `&mut self` method is found.
>
> ``` playground
> struct Foo {}
>
> trait Bar {
>   fn bar(&self);
> }
>
> impl Foo {
>   fn bar(&mut self) {
>     println!("In struct impl!")
>   }
> }
>
> impl Bar for Foo {
>   fn bar(&self) {
>     println!("In trait impl!")
>   }
> }
>
> fn main() {
>   let mut f = Foo{};
>   f.bar();
> }
> ```

[expr.method.ambiguous-target](#r-expr.method.ambiguous-target)

If this results in multiple possible candidates, then it is an error, and the receiver must be [converted](call-expr.md#disambiguating-function-calls) to an appropriate receiver type to make the method call.

[expr.method.receiver-constraints](#r-expr.method.receiver-constraints)

This process does not take into account the mutability or lifetime of the receiver, or whether a method is `unsafe`. Once a method is looked up, if it can’t be called for one (or more) of those reasons, the result is a compiler error.

[expr.method.ambiguous-search](#r-expr.method.ambiguous-search)

If a step is reached where there is more than one possible method, such as where generic methods or traits are considered the same, then it is a compiler error. These cases require a [disambiguating function call syntax](call-expr.md#disambiguating-function-calls) for method and function invocation.

[expr.method.edition2021](#r-expr.method.edition2021)

> 2021 Edition differences
>
> Before the 2021 edition, during the search for visible methods, if the candidate receiver type is an [array type](../types/array.md), methods provided by the standard library [`IntoIterator`](../../core/iter/traits/collect/trait.IntoIterator.html) trait are ignored.
>
> The edition used for this purpose is determined by the token representing the method name.
>
> This special case may be removed in the future.

> ![](../../assets/inline/method-call-expr-3.svg)Warning
>
> For [trait objects](../types/trait-object.md), if there is an inherent method of the same name as a trait method, it will give a compiler error when trying to call the method in a method call expression. Instead, you can call the method using [disambiguating function call syntax](call-expr.md#disambiguating-function-calls), in which case it calls the trait method, not the inherent method. There is no way to call the inherent method. Just don’t define inherent methods on trait objects with the same name as a trait method and you’ll be fine.
