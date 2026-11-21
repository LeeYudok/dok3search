# Expressions

[expr](#r-expr)

# Expressions

[expr.syntax](#r-expr.syntax)

**Syntax**\
[Expression](#railroad-Expression) →\
      [ExpressionWithoutBlock](#grammar-ExpressionWithoutBlock)\
    \| [ExpressionWithBlock](#grammar-ExpressionWithBlock)

[ExpressionWithoutBlock](#railroad-ExpressionWithoutBlock) →\
    [OuterAttribute](attributes.md#grammar-OuterAttribute)\* [ExpressionWithoutBlockNoAttrs](#grammar-ExpressionWithoutBlockNoAttrs)

[ExpressionWithoutBlockNoAttrs](#railroad-ExpressionWithoutBlockNoAttrs) →\
      [LiteralExpression](expressions/literal-expr.md#grammar-LiteralExpression)\
    \| [PathExpression](expressions/path-expr.md#grammar-PathExpression)\
    \| [OperatorExpression](expressions/operator-expr.md#grammar-OperatorExpression)\
    \| [GroupedExpression](expressions/grouped-expr.md#grammar-GroupedExpression)\
    \| [ArrayExpression](expressions/array-expr.md#grammar-ArrayExpression)\
    \| [AwaitExpression](expressions/await-expr.md#grammar-AwaitExpression)\
    \| [IndexExpression](expressions/array-expr.md#grammar-IndexExpression)\
    \| [TupleExpression](expressions/tuple-expr.md#grammar-TupleExpression)\
    \| [TupleIndexingExpression](expressions/tuple-expr.md#grammar-TupleIndexingExpression)\
    \| [StructExpression](expressions/struct-expr.md#grammar-StructExpression)\
    \| [CallExpression](expressions/call-expr.md#grammar-CallExpression)\
    \| [MethodCallExpression](expressions/method-call-expr.md#grammar-MethodCallExpression)\
    \| [FieldExpression](expressions/field-expr.md#grammar-FieldExpression)\
    \| [ClosureExpression](expressions/closure-expr.md#grammar-ClosureExpression)\
    \| [AsyncBlockExpression](expressions/block-expr.md#grammar-AsyncBlockExpression)\
    \| [ContinueExpression](expressions/loop-expr.md#grammar-ContinueExpression)\
    \| [BreakExpression](expressions/loop-expr.md#grammar-BreakExpression)\
    \| [RangeExpression](expressions/range-expr.md#grammar-RangeExpression)\
    \| [ReturnExpression](expressions/return-expr.md#grammar-ReturnExpression)\
    \| [UnderscoreExpression](expressions/underscore-expr.md#grammar-UnderscoreExpression)\
    \| [MacroInvocation](macros.md#grammar-MacroInvocation)

[ExpressionWithBlock](#railroad-ExpressionWithBlock) →\
    [OuterAttribute](attributes.md#grammar-OuterAttribute)\* [ExpressionWithBlockNoAttrs](#grammar-ExpressionWithBlockNoAttrs)

[ExpressionWithBlockNoAttrs](#railroad-ExpressionWithBlockNoAttrs) →\
      [BlockExpression](expressions/block-expr.md#grammar-BlockExpression)\
    \| [ConstBlockExpression](expressions/block-expr.md#grammar-ConstBlockExpression)\
    \| [UnsafeBlockExpression](expressions/block-expr.md#grammar-UnsafeBlockExpression)\
    \| [LoopExpression](expressions/loop-expr.md#grammar-LoopExpression)\
    \| [IfExpression](expressions/if-expr.md#grammar-IfExpression)\
    \| [MatchExpression](expressions/match-expr.md#grammar-MatchExpression)

Show Railroad

![](../assets/inline/expressions-1.svg) ![](../assets/inline/expressions-2.svg) ![](../assets/inline/expressions-3.svg) ![](../assets/inline/expressions-4.svg) ![](../assets/inline/expressions-5.svg) [expr.intro](#r-expr.intro)

An expression may have two roles: it always produces a *value*, and it may have *effects* (otherwise known as “side effects”).

[expr.evaluation](#r-expr.evaluation)

An expression *evaluates to* a value, and has effects during *evaluation*.

[expr.operands](#r-expr.operands)

Many expressions contain sub-expressions, called the *operands* of the expression.

[expr.behavior](#r-expr.behavior)

The meaning of each kind of expression dictates several things:

- Whether or not to evaluate the operands when evaluating the expression
- The order in which to evaluate the operands
- How to combine the operands’ values to obtain the value of the expression

[expr.structure](#r-expr.structure)

In this way, the structure of expressions dictates the structure of execution. Blocks are just another kind of expression, so blocks, statements, expressions, and blocks again can recursively nest inside each other to an arbitrary depth.

> ![](../assets/inline/expressions-6.svg)Note
>
> We give names to the operands of expressions so that we may discuss them, but these names are not stable and may be changed.

[expr.precedence](#r-expr.precedence)

## Expression precedence

The precedence of Rust operators and expressions is ordered as follows, going from strong to weak. Binary Operators at the same precedence level are grouped in the order given by their associativity.

 | Operator/Expression | 
 | Associativity | 

 | [Paths](expressions/path-expr.md#r-expr.path) | 
 |  | 

 | [Method calls](expressions/method-call-expr.md#r-expr.method) | 
 |  | 

 | [Field expressions](expressions/field-expr.md#r-expr.field) | 
 | left to right | 

 | [Function calls](expressions/call-expr.md#r-expr.call), [array indexing](expressions/array-expr.md#r-expr.array.index) | 
 |  | 

 | [?](expressions/operator-expr.md#r-expr.try) | 
 |  | 

 | Unary [-](expressions/operator-expr.md#r-expr.negate) [!](expressions/operator-expr.md#r-expr.negate) [*](expressions/operator-expr.md#r-expr.deref) [borrow](expressions/operator-expr.md#r-expr.operator.borrow) | 
 |  | 

 | [as](expressions/operator-expr.md#r-expr.as) | 
 | left to right | 

 | [*](expressions/operator-expr.md#r-expr.arith-logic) [/](expressions/operator-expr.md#r-expr.arith-logic) [%](expressions/operator-expr.md#r-expr.arith-logic) | 
 | left to right | 

 | [+](expressions/operator-expr.md#r-expr.arith-logic) [-](expressions/operator-expr.md#r-expr.arith-logic) | 
 | left to right | 

 | [<<](expressions/operator-expr.md#r-expr.arith-logic) [>>](expressions/operator-expr.md#r-expr.arith-logic) | 
 | left to right | 

 | [&](expressions/operator-expr.md#r-expr.arith-logic) | 
 | left to right | 

 | [^](expressions/operator-expr.md#r-expr.arith-logic) | 
 | left to right | 

 | [|](expressions/operator-expr.md#r-expr.arith-logic) | 
 | left to right | 

 | [==](expressions/operator-expr.md#r-expr.cmp) [!=](expressions/operator-expr.md#r-expr.cmp) [<](expressions/operator-expr.md#r-expr.cmp) [>](expressions/operator-expr.md#r-expr.cmp) [<=](expressions/operator-expr.md#r-expr.cmp) [>=](expressions/operator-expr.md#r-expr.cmp) | 
 | Require parentheses | 

 | [&&](expressions/operator-expr.md#r-expr.bool-logic) | 
 | left to right | 

 | [||](expressions/operator-expr.md#r-expr.bool-logic) | 
 | left to right | 

 | [..](expressions/range-expr.md#r-expr.range) [..=](expressions/range-expr.md#r-expr.range) | 
 | Require parentheses | 

 | [=](expressions/operator-expr.md#r-expr.assign) [+=](expressions/operator-expr.md#r-expr.compound-assign) [-=](expressions/operator-expr.md#r-expr.compound-assign) [*=](expressions/operator-expr.md#r-expr.compound-assign) [/=](expressions/operator-expr.md#r-expr.compound-assign) [%=](expressions/operator-expr.md#r-expr.compound-assign)

[&=](expressions/operator-expr.md#r-expr.compound-assign) [|=](expressions/operator-expr.md#r-expr.compound-assign) [^=](expressions/operator-expr.md#r-expr.compound-assign) [<<=](expressions/operator-expr.md#r-expr.compound-assign) [>>=](expressions/operator-expr.md#r-expr.compound-assign) | 
 | right to left | 

 | [return](expressions/return-expr.md#r-expr.return) [break](expressions/loop-expr.md#r-expr.loop.break) [closures](expressions/closure-expr.md#r-expr.closure) | 
 |  | 

[expr.operand-order](#r-expr.operand-order)

## Evaluation order of operands

[expr.operand-order.default](#r-expr.operand-order.default)

The following list of expressions all evaluate their operands the same way, as described after the list. Other expressions either don’t take operands or evaluate them conditionally as described on their respective pages.

- Dereference expression
- Error propagation expression
- Negation expression
- Arithmetic and logical binary operators
- Comparison operators
- Type cast expression
- Grouped expression
- Array expression
- Await expression
- Index expression
- Tuple expression
- Tuple index expression
- Struct expression
- Call expression
- Method call expression
- Field expression
- Break expression
- Range expression
- Return expression

[expr.operand-order.operands-before-primary](#r-expr.operand-order.operands-before-primary)

The operands of these expressions are evaluated prior to applying the effects of the expression. Expressions taking multiple operands are evaluated left to right as written in the source code.

> ![](../assets/inline/expressions-7.svg)Note
>
> Which subexpressions are the operands of an expression is determined by expression precedence as per the previous section.

For example, the two `next` method calls will always be called in the same order:

``` playground
#![allow(unused)]
fn main() {
// Using vec instead of array to avoid references
// since there is no stable owned array iterator
// at the time this example was written.
let mut one_two = vec![1, 2].into_iter();
assert_eq!(
    (1, 2),
    (one_two.next().unwrap(), one_two.next().unwrap())
);
}
```

> ![](../assets/inline/expressions-8.svg)Note
>
> Since this is applied recursively, these expressions are also evaluated from innermost to outermost, ignoring siblings until there are no inner subexpressions.

[expr.place-value](#r-expr.place-value)

## Place expressions and value expressions

[expr.place-value.intro](#r-expr.place-value.intro)

Expressions are divided into two main categories: place expressions and value expressions; there is also a third, minor category of expressions called assignee expressions. Within each expression, operands may likewise occur in either place context or value context. The evaluation of an expression depends both on its own category and the context it occurs within.

[expr.place-value.place-memory-location](#r-expr.place-value.place-memory-location)

A *place expression* is an expression that represents a memory location.

[expr.place-value.place-expr-kinds](#r-expr.place-value.place-expr-kinds)

These expressions are [paths](expressions/path-expr.md) which refer to local variables, [static variables](items/static-items.md), [dereferences](expressions/operator-expr.md#the-dereference-operator) (`*expr`), [array indexing](expressions/array-expr.md#array-and-slice-indexing-expressions) expressions (`expr[expr]`), [field](expressions/field-expr.md) references (`expr.f`) and parenthesized place expressions.

[expr.place-value.value-expr-kinds](#r-expr.place-value.value-expr-kinds)

All other expressions are value expressions.

[expr.place-value.value-result](#r-expr.place-value.value-result)

A *value expression* is an expression that represents an actual value.

[expr.place-value.place-context](#r-expr.place-value.place-context)

The following contexts are *place expression* contexts:

- The left operand of a [compound assignment](expressions/operator-expr.md#compound-assignment-expressions) expression.
- The operand of a unary [borrow](expressions/operator-expr.md#borrow-operators), [raw borrow](expressions/operator-expr.md#raw-borrow-operators) or [dereference](expressions/operator-expr.md#the-dereference-operator) operator.
- The operand of a [field expression](expressions/field-expr.md#r-expr.field).
- The indexed operand of an [array indexing expression](expressions/array-expr.md#r-expr.array.index).
- The tuple operand of a [tuple indexing expression](expressions/tuple-expr.md#r-expr.tuple-index).
- The operand of any [implicit borrow](#implicit-borrows).
- The initializer of a [let statement](statements.md#let-statements).
- The [scrutinee](glossary.md#scrutinee) of an [`if let`](expressions/if-expr.md#if-let-patterns), [`match`](expressions/match-expr.md), or [`while let`](expressions/loop-expr.md#while-let-patterns) expression.
- The base of a [functional update](expressions/struct-expr.md#functional-update-syntax) struct expression.

> ![](../assets/inline/expressions-9.svg)Note
>
> Historically, place expressions were called *lvalues* and value expressions were called *rvalues*.

[expr.place-value.assignee](#r-expr.place-value.assignee)

An *assignee expression* is an expression that appears in the left operand of an [assignment](expressions/operator-expr.md#assignment-expressions) expression. Explicitly, the assignee expressions are:

- Place expressions.
- [Underscores](expressions/underscore-expr.md).
- [Tuples](expressions/tuple-expr.md) of assignee expressions.
- [Slices](expressions/array-expr.md#r-expr.array.index) of assignee expressions.
- [Tuple structs](items/structs.md#r-items.struct.tuple) of assignee expressions.
- [Structs](expressions/struct-expr.md#r-expr.struct) of assignee expressions (with optionally named fields).
- [Unit structs](items/structs.md#r-items.struct.unit)

[expr.place-value.parenthesis](#r-expr.place-value.parenthesis)

Arbitrary parenthesisation is permitted inside assignee expressions.

[expr.move](#r-expr.move)

### Moved and copied types

[expr.move.intro](#r-expr.move.intro)

When a place expression is evaluated in a value expression context, or is bound by value in a pattern, it denotes the value held *in* that memory location.

[expr.move.copy](#r-expr.move.copy)

If the type of that value implements [`Copy`](special-types-and-traits.md#copy), then the value will be copied.

[expr.move.requires-sized](#r-expr.move.requires-sized)

In the remaining situations, if that type is [`Sized`](special-types-and-traits.md#sized), then it may be possible to move the value.

[expr.move.movable-place](#r-expr.move.movable-place)

Only the following place expressions may be moved out of:

- [Variables](variables.md) which are not currently borrowed.
- [Temporary values](#temporaries).
- [Fields](expressions/field-expr.md) of a place expression which can be moved out of and don’t implement [`Drop`](special-types-and-traits.md#drop).
- The result of [dereferencing](expressions/operator-expr.md#the-dereference-operator) an expression with type [`Box<T>`](special-types-and-traits.md#boxt) and that can also be moved out of.

[expr.move.deinitialization](#r-expr.move.deinitialization)

After moving out of a place expression that evaluates to a local variable, the location is deinitialized and cannot be read from again until it is reinitialized.

[expr.move.place-invalid](#r-expr.move.place-invalid)

In all other cases, trying to use a place expression in a value expression context is an error.

[expr.mut](#r-expr.mut)

### Mutability

[expr.mut.intro](#r-expr.mut.intro)

For a place expression to be [assigned](expressions/operator-expr.md#assignment-expressions) to, mutably [borrowed](expressions/operator-expr.md#borrow-operators), [implicitly mutably borrowed](#implicit-borrows), or bound to a pattern containing `ref mut`, it must be *mutable*. We call these *mutable place expressions*. In contrast, other place expressions are called *immutable place expressions*.

[expr.mut.valid-places](#r-expr.mut.valid-places)

The following expressions can be mutable place expression contexts:

- Mutable [variables](variables.md) which are not currently borrowed.
- [Mutable `static` items](items/static-items.md#mutable-statics).
- [Temporary values](#temporaries).
- [Fields](expressions/field-expr.md): this evaluates the subexpression in a mutable place expression context.
- [Dereferences](expressions/operator-expr.md#the-dereference-operator) of a `*mut T` pointer.
- Dereference of a variable, or field of a variable, with type `&mut T`. Note: This is an exception to the requirement of the next rule.
- Dereferences of a type that implements `DerefMut`: this then requires that the value being dereferenced is evaluated in a mutable place expression context.
- [Array indexing](expressions/array-expr.md#array-and-slice-indexing-expressions) of a type that implements `IndexMut`: this then evaluates the value being indexed, but not the index, in mutable place expression context.

[expr.temporary](#r-expr.temporary)

### Temporaries

When using a value expression in most place expression contexts, a temporary unnamed memory location is created and initialized to that value. The expression evaluates to that location instead, except if [promoted](destructors.md#constant-promotion) to a `static`. The [drop scope](destructors.md#drop-scopes) of the temporary is usually the end of the enclosing statement.

[expr.super-macros](#r-expr.super-macros)

### Super macros

[expr.super-macros.intro](#r-expr.super-macros.intro)

Certain built-in macros may create [temporaries](#r-expr.temporary) whose [scopes](destructors.md#r-destructors.scope.temporary) may be [extended](destructors.md#r-destructors.scope.lifetime-extension). These temporaries are *super temporaries* and these macros are *super macros*. [Invocations](macros.md#r-macro.invocation) of these macros are *super macro call expressions*. Arguments to these macros may be *super operands*.

> ![](../assets/inline/expressions-10.svg)Note
>
> When a super macro call expression is an [extending expression](destructors.md#r-destructors.scope.lifetime-extension.exprs), its super operands are [extending expressions](destructors.md#r-destructors.scope.lifetime-extension.exprs) and the [scopes](destructors.md#r-destructors.scope.temporary) of the super temporaries are [extended](destructors.md#r-destructors.scope.lifetime-extension). See [destructors.scope.lifetime-extension.exprs](destructors.md#r-destructors.scope.lifetime-extension.exprs).

[expr.super-macros.format_args](#r-expr.super-macros.format_args)

#### `format_args!`

[expr.super-macros.format_args.super-operands](#r-expr.super-macros.format_args.super-operands)

Except for the format string argument, all arguments passed to [`format_args!`](../core/macro.format_args.html) are *super operands*.

``` playground
#![allow(unused)]
fn main() {
fn temp() -> String { String::from("") }
// Due to the call being an extending expression and the argument
// being a super operand, the inner block is an extending expression,
// so the scope of the temporary created in its trailing expression
// is extended.
let _ = format_args!("{}", { &temp() }); // OK
}
```

[expr.super-macros.format_args.super-temporaries](#r-expr.super-macros.format_args.super-temporaries)

The super operands of [`format_args!`](../core/macro.format_args.html) are [implicitly borrowed](#r-expr.implicit-borrow) and are therefore [place expression contexts](#r-expr.place-value). When a [value expression](#r-expr.place-value) is passed as an argument, it creates a *super temporary*.

``` playground
#![allow(unused)]
fn main() {
fn temp() -> String { String::from("") }
let x = format_args!("{}", temp());
x; // <-- The temporary is extended, allowing use here.
}
```

The expansion of a call to [`format_args!`](../core/macro.format_args.html) sometimes creates other internal *super temporaries*.

``` playground
#![allow(unused)]
fn main() {
let x = {
    // This call creates an internal temporary.
    let x = format_args!("{:?}", 0);
    x // <-- The temporary is extended, allowing its use here.
}; // <-- The temporary is dropped here.
x; // ERROR
}
```

``` playground
#![allow(unused)]
fn main() {
// This call doesn't create an internal temporary.
let x = { let x = format_args!("{}", 0); x };
x; // OK
}
```

> ![](../assets/inline/expressions-11.svg)Note
>
> The details of when [`format_args!`](../core/macro.format_args.html) does or does not create internal temporaries are currently unspecified.

[expr.super-macros.pin](#r-expr.super-macros.pin)

#### `pin!`

[expr.super-macros.pin.super-operands](#r-expr.super-macros.pin.super-operands)

The argument to [`pin!`](../core/pin/macro.pin.html) is a *super operand*.

``` playground
#![allow(unused)]
fn main() {
use core::pin::pin;
fn temp() {}
// As above for `format_args!`.
let _ = pin!({ &temp() }); // OK
}
```

[expr.super-macros.pin.super-temporaries](#r-expr.super-macros.pin.super-temporaries)

The argument to [`pin!`](../core/pin/macro.pin.html) is a [value expression context](#r-expr.place-value) and creates a *super temporary*.

``` playground
#![allow(unused)]
fn main() {
use core::pin::pin;
fn temp() {}
// The argument is evaluated into a super temporary.
let x = pin!(temp());
// The temporary is extended, allowing its use here.
x; // OK
}
```

[expr.implicit-borrow](#r-expr.implicit-borrow)

### Implicit borrows

[expr.implicit-borrow-intro](#r-expr.implicit-borrow-intro)

Certain expressions will treat an expression as a place expression by implicitly borrowing it. For example, it is possible to compare two unsized [slices](types/slice.md) for equality directly, because the `==` operator implicitly borrows its operands:

``` playground
#![allow(unused)]
fn main() {
let c = [1, 2, 3];
let d = vec![1, 2, 3];
let a: &[i32];
let b: &[i32];
a = &c;
b = &d;
// ...
*a == *b;
// Equivalent form:
::std::cmp::PartialEq::eq(&*a, &*b);
}
```

[expr.implicit-borrow.application](#r-expr.implicit-borrow.application)

Implicit borrows may be taken in the following expressions:

- Left operand in [method-call](expressions/method-call-expr.md) expressions.
- Left operand in [field](expressions/field-expr.md) expressions.
- Left operand in [call expressions](expressions/call-expr.md).
- Left operand in [array indexing](expressions/array-expr.md#array-and-slice-indexing-expressions) expressions.
- Operand of the [dereference operator](expressions/operator-expr.md#the-dereference-operator) (`*`).
- Operands of [comparison](expressions/operator-expr.md#comparison-operators).
- Left operands of the [compound assignment](expressions/operator-expr.md#compound-assignment-expressions).
- Arguments to [`format_args!`](../core/macro.format_args.html) except the format string.

[expr.overload](#r-expr.overload)

## Overloading traits

Many of the following operators and expressions can also be overloaded for other types using traits in `std::ops` or `std::cmp`. These traits also exist in `core::ops` and `core::cmp` with the same names.

[expr.attr](#r-expr.attr)

## Expression attributes

[expr.attr.restriction](#r-expr.attr.restriction)

[Outer attributes](attributes.md) before an expression are allowed only in a few specific cases:

- Before an expression used as a [statement](statements.md).
- Elements of [array expressions](expressions/array-expr.md), [tuple expressions](expressions/tuple-expr.md), [call expressions](expressions/call-expr.md), and tuple-style [struct](expressions/struct-expr.md) expressions.
- The tail expression of [block expressions](expressions/block-expr.md).

[expr.attr.never-before](#r-expr.attr.never-before)

They are never allowed before:

- [Range](expressions/range-expr.md) expressions.
- Binary operator expressions ([ArithmeticOrLogicalExpression](expressions/operator-expr.md#grammar-ArithmeticOrLogicalExpression), [ComparisonExpression](expressions/operator-expr.md#grammar-ComparisonExpression), [LazyBooleanExpression](expressions/operator-expr.md#grammar-LazyBooleanExpression), [TypeCastExpression](expressions/operator-expr.md#grammar-TypeCastExpression), [AssignmentExpression](expressions/operator-expr.md#grammar-AssignmentExpression), [CompoundAssignmentExpression](expressions/operator-expr.md#grammar-CompoundAssignmentExpression)).
