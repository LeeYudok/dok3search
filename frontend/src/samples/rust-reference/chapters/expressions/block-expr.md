# Block expressions

[expr.block](#r-expr.block)

# Block expressions

[expr.block.syntax](#r-expr.block.syntax)

**Syntax**\
[BlockExpression](#railroad-BlockExpression) →\
    {\
        [InnerAttribute](../attributes.md#grammar-InnerAttribute)\*\
        [Statements](#grammar-Statements)?\
    }

[BlockExpressionNoInnerAttributes](#railroad-BlockExpressionNoInnerAttributes) →\
    {\
        [Statements](#grammar-Statements)?\
    }

[Statements](#railroad-Statements) →\
      [Statement](../statements.md#grammar-Statement)+\
    \| [Statement](../statements.md#grammar-Statement)+ [ExpressionWithoutBlock](../expressions.md#grammar-ExpressionWithoutBlock)\
    \| [ExpressionWithoutBlock](../expressions.md#grammar-ExpressionWithoutBlock)

Show Railroad

![](../../assets/inline/block-expr-1.svg) ![](../../assets/inline/block-expr-2.svg) ![](../../assets/inline/block-expr-3.svg) [expr.block.intro](#r-expr.block.intro)

A *block expression*, or *block*, is a control flow expression and anonymous namespace scope for items and variable declarations.

[expr.block.sequential-evaluation](#r-expr.block.sequential-evaluation)

As a control flow expression, a block sequentially executes its component non-item declaration statements and then its final optional expression.

[expr.block.namespace](#r-expr.block.namespace)

As an anonymous namespace scope, item declarations are only in scope inside the block itself and variables declared by `let` statements are in scope from the next statement until the end of the block. See the [scopes](../names/scopes.md) chapter for more details.

[expr.block.inner-attributes](#r-expr.block.inner-attributes)

The syntax for a block is `{`, then any [inner attributes](../attributes.md), then any number of [statements](../statements.md), then an optional expression, called the final operand, and finally a `}`.

[expr.block.statements](#r-expr.block.statements)

Statements are usually required to be followed by a semicolon, with two exceptions:

1.  Item declaration statements do not need to be followed by a semicolon.
2.  Expression statements usually require a following semicolon except if its outer expression is a flow control expression.

[expr.block.null-statement](#r-expr.block.null-statement)

Furthermore, extra semicolons between statements are allowed, but these semicolons do not affect semantics.

[expr.block.evaluation](#r-expr.block.evaluation)

When evaluating a block expression, each statement, except for item declaration statements, is executed sequentially.

[expr.block.result](#r-expr.block.result)

Then the final operand is executed, if given.

[expr.block.value-trailing-expr](#r-expr.block.value-trailing-expr)

When a block contains a [final operand](#r-expr.block.inner-attributes), the block has the type and value of that final operand.

``` playground
#![allow(unused)]
fn main() {
let x: u8 = { 0u8 }; // `0u8` is the final operand.
assert_eq!(x, 0);
let x: u8 = { (); 0u8 }; // As above.
assert_eq!(x, 0);
}
```

[expr.block.value-no-trailing-expr](#r-expr.block.value-no-trailing-expr)

When a block does not contain a [final operand](#r-expr.block.inner-attributes) and the block does not diverge, the block has [unit type](../types/tuple.md#r-type.tuple.unit) and [unit value](../types/tuple.md#r-type.tuple.unit).

``` playground
#![allow(unused)]
fn main() {
let x: () = {}; // Has no final operand.
assert_eq!(x, ());
let x: () = { 0u8; }; // As above.
assert_eq!(x, ());
}
```

[expr.block.value-diverges-no-trailing-expr](#r-expr.block.value-diverges-no-trailing-expr)

When a block does not contain a [final operand](#r-expr.block.inner-attributes) and the block [diverges](#r-expr.block.diverging), the block has the [never type](../types/never.md#r-type.never) and has no final value (because its type is [uninhabited](../glossary.md#r-glossary.uninhabited)).

``` playground
#![allow(unused)]
fn main() {
fn f() -> ! { loop {}; } // Diverges and has no final operand.
//          ^^^^^^^^^^^^
// The body of a function is a block expression.
}
```

> ![](../../assets/inline/block-expr-4.svg)Note
>
> Observe that a block having no final operand is distinct from having an explicit final operand with unit type. E.g., even though this block diverges, the type of the block is [unit](../types/tuple.md#r-type.tuple.unit) rather than [never](../types/never.md#r-type.never).
>
> ``` playground
> #![allow(unused)]
> fn main() {
> fn f() -> ! { loop {}; () } // ERROR: Mismatched types.
> //          ^^^^^^^^^^^^^^^ This block has unit type.
> }
> ```

> ![](../../assets/inline/block-expr-5.svg)Note
>
> As a control flow expression, if a block expression is the outer expression of an expression statement, the expected type is `()` unless it is followed immediately by a semicolon.

[expr.block.diverging](#r-expr.block.diverging)

A block is considered to be [diverging](../divergence.md#r-divergence) if all reachable control flow paths contain a diverging expression, unless that expression is a [place expression](../expressions.md#r-expr.place-value.place-memory-location) that is not read from.

``` playground
#![allow(unused)]
fn main() {
fn no_control_flow() -> ! {
    // There are no conditional statements, so this entire function body is diverging.
    loop {}
}

fn control_flow_diverging() -> ! {
    // All paths are diverging, so this entire function body is diverging.
    if true {
        loop {}
    } else {
        loop {}
    }
}

fn control_flow_not_diverging() -> () {
    // Some paths are not diverging, so this entire block is not diverging.
    if true {
        ()
    } else {
        loop {}
    }
}

fn diverging_place_read(x: !) -> ! {
    // A read of a place expression produces a diverging block.
    let _x = x;
}
}
```

``` playground
#![allow(unused)]
fn main() {
fn diverging_place_not_read(x: !) -> ! {
    // Assignment to `_` means the place is not read.
    let _ = x;
} // ERROR: Mismatched types.
}
```

[expr.block.value](#r-expr.block.value)

Blocks are always [value expressions](../expressions.md#place-expressions-and-value-expressions) and evaluate the last operand in value expression context.

> ![](../../assets/inline/block-expr-6.svg)Note
>
> This can be used to force moving a value if really needed. For example, the following example fails on the call to `consume_self` because the struct was moved out of `s` in the block expression.
>
> ``` playground
> #![allow(unused)]
> fn main() {
> struct Struct;
>
> impl Struct {
>     fn consume_self(self) {}
>     fn borrow_self(&self) {}
> }
>
> fn move_by_block_expression() {
>     let s = Struct;
>
>     // Move the value out of `s` in the block expression.
>     (&{ s }).borrow_self();
>
>     // Fails to execute because `s` is moved out of.
>     s.consume_self();
> }
> }
> ```

[expr.block.async](#r-expr.block.async)

## `async` blocks

[expr.block.async.syntax](#r-expr.block.async.syntax)

**Syntax**\
[AsyncBlockExpression](#railroad-AsyncBlockExpression) → async move? [BlockExpression](#grammar-BlockExpression)

Show Railroad

![](../../assets/inline/block-expr-7.svg) [expr.block.async.intro](#r-expr.block.async.intro)

An *async block* is a variant of a block expression which evaluates to a future.

[expr.block.async.future-result](#r-expr.block.async.future-result)

The final expression of the block, if present, determines the result value of the future.

[expr.block.async.anonymous-type](#r-expr.block.async.anonymous-type)

Executing an async block is similar to executing a closure expression: its immediate effect is to produce and return an anonymous type.

[expr.block.async.future](#r-expr.block.async.future)

Whereas closures return a type that implements one or more of the [`std::ops::Fn`](../../core/ops/function/trait.Fn.html) traits, however, the type returned for an async block implements the [`std::future::Future`](../../core/future/future/trait.Future.html) trait.

[expr.block.async.layout-unspecified](#r-expr.block.async.layout-unspecified)

The actual data format for this type is unspecified.

> ![](../../assets/inline/block-expr-8.svg)Note
>
> The future type that rustc generates is roughly equivalent to an enum with one variant per `await` point, where each variant stores the data needed to resume from its corresponding point.

[expr.block.async.edition2018](#r-expr.block.async.edition2018)

> 2018 Edition differences
>
> Async blocks are only available beginning with Rust 2018.

[expr.block.async.capture](#r-expr.block.async.capture)

### Capture modes

Async blocks capture variables from their environment using the same [capture modes](../types/closure.md#capture-modes) as closures. Like closures, when written `async { .. }` the capture mode for each variable will be inferred from the content of the block. `async move { .. }` blocks however will move all referenced variables into the resulting future.

[expr.block.async.context](#r-expr.block.async.context)

### Async context

Because async blocks construct a future, they define an **async context** which can in turn contain [`await` expressions](await-expr.md). Async contexts are established by async blocks as well as the bodies of async functions, whose semantics are defined in terms of async blocks.

[expr.block.async.function](#r-expr.block.async.function)

### Control-flow operators

[expr.block.async.function.intro](#r-expr.block.async.function.intro)

Async blocks act like a function boundary, much like closures.

[expr.block.async.function.return-try](#r-expr.block.async.function.return-try)

Therefore, the `?` operator and `return` expressions both affect the output of the future, not the enclosing function or other context. That is, `return <expr>` from within an async block will return the result of `<expr>` as the output of the future. Similarly, if `<expr>?` propagates an error, that error is propagated as the result of the future.

[expr.block.async.function.control-flow](#r-expr.block.async.function.control-flow)

Finally, the `break` and `continue` keywords cannot be used to branch out from an async block. Therefore the following is illegal:

``` playground
#![allow(unused)]
fn main() {
loop {
    async move {
        break; // error[E0267]: `break` inside of an `async` block
    }
}
}
```

[expr.block.const](#r-expr.block.const)

## `const` blocks

[expr.block.const.syntax](#r-expr.block.const.syntax)

**Syntax**\
[ConstBlockExpression](#railroad-ConstBlockExpression) → const [BlockExpression](#grammar-BlockExpression)

Show Railroad

![](../../assets/inline/block-expr-9.svg) [expr.block.const.intro](#r-expr.block.const.intro)

A *const block* is a variant of a block expression whose body evaluates at compile-time instead of at runtime.

[expr.block.const.context](#r-expr.block.const.context)

Const blocks allows you to define a constant value without having to define new [constant items](../items/constant-items.md), and thus they are also sometimes referred as *inline consts*. It also supports type inference so there is no need to specify the type, unlike [constant items](../items/constant-items.md).

[expr.block.const.generic-params](#r-expr.block.const.generic-params)

Const blocks have the ability to reference generic parameters in scope, unlike [free](../glossary.md#free-item) constant items. They are desugared to constant items with generic parameters in scope (similar to associated constants, but without a trait or type they are associated with). For example, this code:

``` playground
#![allow(unused)]
fn main() {
fn foo<T>() -> usize {
    const { std::mem::size_of::<T>() + 1 }
}
}
```

is equivalent to:

``` playground
#![allow(unused)]
fn main() {
fn foo<T>() -> usize {
    {
        struct Const<T>(T);
        impl<T> Const<T> {
            const CONST: usize = std::mem::size_of::<T>() + 1;
        }
        Const::<T>::CONST
    }
}
}
```

[expr.block.const.evaluation](#r-expr.block.const.evaluation)

If the const block expression is executed at runtime, then the constant is guaranteed to be evaluated, even if its return value is ignored:

``` playground
#![allow(unused)]
fn main() {
fn foo<T>() -> usize {
    // If this code ever gets executed, then the assertion has definitely
    // been evaluated at compile-time.
    const { assert!(std::mem::size_of::<T>() > 0); }
    // Here we can have unsafe code relying on the type being non-zero-sized.
    /* ... */
    42
}
}
```

[expr.block.const.not-executed](#r-expr.block.const.not-executed)

If the const block expression is not executed at runtime, it may or may not be evaluated:

``` playground
#![allow(unused)]
fn main() {
if false {
    // The panic may or may not occur when the program is built.
    const { panic!(); }
}
}
```

[expr.block.unsafe](#r-expr.block.unsafe)

## `unsafe` blocks

[expr.block.unsafe.syntax](#r-expr.block.unsafe.syntax)

**Syntax**\
[UnsafeBlockExpression](#railroad-UnsafeBlockExpression) → unsafe [BlockExpression](#grammar-BlockExpression)

Show Railroad

![](../../assets/inline/block-expr-10.svg) [expr.block.unsafe.intro](#r-expr.block.unsafe.intro)

*See [`unsafe` blocks](../unsafe-keyword.md#unsafe-blocks-unsafe-) for more information on when to use `unsafe`*.

A block of code can be prefixed with the `unsafe` keyword to permit [unsafe operations](../unsafety.md). Examples:

``` playground
#![allow(unused)]
fn main() {
unsafe {
    let b = [13u8, 17u8];
    let a = &b[0] as *const u8;
    assert_eq!(*a, 13);
    assert_eq!(*a.offset(1), 17);
}

unsafe fn an_unsafe_fn() -> i32 { 10 }
let a = unsafe { an_unsafe_fn() };
}
```

[expr.block.label](#r-expr.block.label)

## Labeled block expressions

Labeled block expressions are documented in the [Loops and other breakable expressions](loop-expr.md#r-expr.loop.block-labels) section.

[expr.block.attributes](#r-expr.block.attributes)

## Attributes on block expressions

[expr.block.attributes.inner-attributes](#r-expr.block.attributes.inner-attributes)

[Inner attributes](../attributes.md) are allowed directly after the opening brace of a block expression in the following situations:

- [Function](../items/functions.md) and [method](../items/associated-items.md#methods) bodies.
- Loop bodies ([`loop`](loop-expr.md#infinite-loops), [`while`](loop-expr.md#predicate-loops), and [`for`](loop-expr.md#iterator-loops)).
- Block expressions used as a [statement](../statements.md).
- Block expressions as elements of [array expressions](array-expr.md), [tuple expressions](tuple-expr.md), [call expressions](call-expr.md), and tuple-style [struct](struct-expr.md) expressions.
- A block expression as the tail expression of another block expression.

[expr.block.attributes.valid](#r-expr.block.attributes.valid)

The attributes that have meaning on a block expression are [`cfg`](../conditional-compilation.md) and [the lint check attributes](../attributes/diagnostics.md#lint-check-attributes).

For example, this function returns `true` on unix platforms and `false` on other platforms.

``` playground
#![allow(unused)]
fn main() {
fn is_unix_platform() -> bool {
    #[cfg(unix)] { true }
    #[cfg(not(unix))] { false }
}
}
```
