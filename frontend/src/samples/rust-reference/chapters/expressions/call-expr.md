# Call expressions

[expr.call](#r-expr.call)

# Call expressions

[expr.call.syntax](#r-expr.call.syntax)

**Syntax**\
[CallExpression](#railroad-CallExpression) → [Expression](../expressions.md#grammar-Expression) ( [CallParams](#grammar-CallParams)? )

[CallParams](#railroad-CallParams) → [Expression](../expressions.md#grammar-Expression) ( , [Expression](../expressions.md#grammar-Expression) )\* ,?

Show Railroad

![](../../assets/inline/call-expr-1.svg) ![](../../assets/inline/call-expr-2.svg) [expr.call.intro](#r-expr.call.intro)

A *call expression* calls a function. The syntax of a call expression is an expression, called the *function operand*, followed by a parenthesized comma-separated list of expression, called the *argument operands*.

[expr.call.convergence](#r-expr.call.convergence)

If the function eventually returns, then the expression completes.

[expr.call.trait](#r-expr.call.trait)

For [non-function types](../types/function-item.md), the expression `f(...)` uses the method on one of the following traits based on the function operand:

- [`Fn`](../../core/ops/function/trait.Fn.html) or [`AsyncFn`](../../core/ops/async_function/trait.AsyncFn.html) — shared reference.
- [`FnMut`](../../core/ops/function/trait.FnMut.html) or [`AsyncFnMut`](../../core/ops/async_function/trait.AsyncFnMut.html) — mutable reference.
- [`FnOnce`](../../core/ops/function/trait.FnOnce.html) or [`AsyncFnOnce`](../../core/ops/async_function/trait.AsyncFnOnce.html) — value.

[expr.call.autoref-deref](#r-expr.call.autoref-deref)

An automatic borrow will be taken if needed. The function operand will also be [automatically dereferenced](field-expr.md#automatic-dereferencing) as required.

Some examples of call expressions:

``` playground
#![allow(unused)]
fn main() {
fn add(x: i32, y: i32) -> i32 { 0 }
let three: i32 = add(1i32, 2i32);
let name: &'static str = (|| "Rust")();
}
```

[expr.call.desugar](#r-expr.call.desugar)

## Disambiguating function calls

[expr.call.desugar.fully-qualified](#r-expr.call.desugar.fully-qualified)

All function calls are sugar for a more explicit [fully-qualified syntax](../paths.md#qualified-paths).

[expr.call.desugar.ambiguity](#r-expr.call.desugar.ambiguity)

Function calls may need to be fully qualified, depending on the ambiguity of a call in light of in-scope items.

> ![](../../assets/inline/call-expr-3.svg)Note
>
> In the past, the terms “Unambiguous Function Call Syntax”, “Universal Function Call Syntax”, or “UFCS”, have been used in documentation, issues, RFCs, and other community writings. However, these terms lack descriptive power and potentially confuse the issue at hand. We mention them here for searchability’s sake.

[expr.call.desugar.limits](#r-expr.call.desugar.limits)

Several situations often occur which result in ambiguities about the receiver or referent of method or associated function calls. These situations may include:

- Multiple in-scope traits define methods with the same name for the same types
- Auto-`deref` is undesirable; for example, distinguishing between methods on a smart pointer itself and the pointer’s referent
- Methods which take no arguments, like [`default()`](../../core/default/trait.Default.html#tymethod.default), and return properties of a type, like [`size_of()`](../../core/mem/fn.size_of.html)

[expr.call.desugar.explicit-path](#r-expr.call.desugar.explicit-path)

To resolve the ambiguity, the programmer may refer to their desired method or function using more specific paths, types, or traits.

For example,

``` playground
trait Pretty {
    fn print(&self);
}

trait Ugly {
    fn print(&self);
}

struct Foo;
impl Pretty for Foo {
    fn print(&self) {}
}

struct Bar;
impl Pretty for Bar {
    fn print(&self) {}
}
impl Ugly for Bar {
    fn print(&self) {}
}

fn main() {
    let f = Foo;
    let b = Bar;

    // we can do this because we only have one item called `print` for `Foo`s
    f.print();
    // more explicit, and, in the case of `Foo`, not necessary
    Foo::print(&f);
    // if you're not into the whole brevity thing
    <Foo as Pretty>::print(&f);

    // b.print(); // Error: multiple 'print' found
    // Bar::print(&b); // Still an error: multiple `print` found

    // necessary because of in-scope items defining `print`
    <Bar as Pretty>::print(&b);
}
```

Refer to [RFC 132](https://github.com/rust-lang/rfcs/blob/master/text/0132-ufcs.md) for further details and motivations.
