# Destructors

[destructors](#r-destructors)

# Destructors

[destructors.intro](#r-destructors.intro)

When an [initialized](glossary.md#initialized) [variable](variables.md) or [temporary](expressions.md#temporaries) goes out of [scope](#drop-scopes), its *destructor* is run or it is *dropped*. [Assignment](expressions/operator-expr.md#assignment-expressions) also runs the destructor of its left-hand operand, if it’s initialized. If a variable has been partially initialized, only its initialized fields are dropped.

[destructors.operation](#r-destructors.operation)

The destructor of a type `T` consists of:

1.  If `T: Drop`, calling [`<T as core::ops::Drop>::drop`](../core/ops/drop/trait.Drop.html#method.drop)
2.  Recursively running the destructor of all of its fields.
    - The fields of a [struct](types/struct.md) are dropped in declaration order.
    - The fields of the active [enum variant](types/enum.md) are dropped in declaration order.
    - The fields of a [tuple](types/tuple.md) are dropped in order.
    - The elements of an [array](types/array.md) or owned [slice](types/slice.md) are dropped from the first element to the last.
    - The variables that a [closure](types/closure.md) captures by move are dropped in an unspecified order.
    - [Trait objects](types/trait-object.md) run the destructor of the underlying type.
    - Other types don’t result in any further drops.

[destructors.drop_in_place](#r-destructors.drop_in_place)

If a destructor must be run manually, such as when implementing your own smart pointer, [`core::ptr::drop_in_place`](../core/ptr/fn.drop_in_place.html) can be used.

Some examples:

``` playground
#![allow(unused)]
fn main() {
struct PrintOnDrop(&'static str);

impl Drop for PrintOnDrop {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}

let mut overwritten = PrintOnDrop("drops when overwritten");
overwritten = PrintOnDrop("drops when scope ends");

let tuple = (PrintOnDrop("Tuple first"), PrintOnDrop("Tuple second"));

let moved;
// No destructor run on assignment.
moved = PrintOnDrop("Drops when moved");
// Drops now, but is then uninitialized.
moved;

// Uninitialized does not drop.
let uninitialized: PrintOnDrop;

// After a partial move, only the remaining fields are dropped.
let mut partial_move = (PrintOnDrop("first"), PrintOnDrop("forgotten"));
// Perform a partial move, leaving only `partial_move.0` initialized.
core::mem::forget(partial_move.1);
// When partial_move's scope ends, only the first field is dropped.
}
```

[destructors.scope](#r-destructors.scope)

## Drop scopes

[destructors.scope.intro](#r-destructors.scope.intro)

Each variable or temporary is associated to a *drop scope*. When control flow leaves a drop scope all variables associated to that scope are dropped in reverse order of declaration (for variables) or creation (for temporaries).

[destructors.scope.desugaring](#r-destructors.scope.desugaring)

Drop scopes can be determined by replacing [`for`](expressions/loop-expr.md#iterator-loops), [`if`](expressions/if-expr.md#if-expressions), and [`while`](expressions/loop-expr.md#predicate-loops) expressions with equivalent expressions using [`match`](expressions/match-expr.md), [`loop`](expressions/loop-expr.md#infinite-loops) and `break`.

[destructors.scope.operators](#r-destructors.scope.operators)

Overloaded operators are not distinguished from built-in operators and [binding modes](patterns.md#binding-modes) are not considered.

[destructors.scope.list](#r-destructors.scope.list)

Given a function, or closure, there are drop scopes for:

[destructors.scope.function](#r-destructors.scope.function)

- The entire function

[destructors.scope.statement](#r-destructors.scope.statement)

- Each [statement](statements.md)

[destructors.scope.expression](#r-destructors.scope.expression)

- Each [expression](expressions.md)

[destructors.scope.block](#r-destructors.scope.block)

- Each block, including the function body
  - In the case of a [block expression](expressions/block-expr.md), the scope for the block and the expression are the same scope.

[destructors.scope.match-arm](#r-destructors.scope.match-arm)

- Each arm of a `match` expression

[destructors.scope.nesting](#r-destructors.scope.nesting)

Drop scopes are nested within one another as follows:

[destructors.scope.nesting-function](#r-destructors.scope.nesting-function)

- The entire function scope is the outer most scope.

[destructors.scope.nesting-function-body](#r-destructors.scope.nesting-function-body)

- The function body block is contained within the scope of the entire function.

[destructors.scope.nesting-expr-statement](#r-destructors.scope.nesting-expr-statement)

- The parent of the expression in an expression statement is the scope of the statement.

[destructors.scope.nesting-let-initializer](#r-destructors.scope.nesting-let-initializer)

- The parent of the initializer of a [`let` statement](statements.md#let-statements) is the `let` statement’s scope.

[destructors.scope.nesting-statement](#r-destructors.scope.nesting-statement)

- The parent of a statement scope is the scope of the block that contains the statement.

[destructors.scope.nesting-match-guard](#r-destructors.scope.nesting-match-guard)

- The parent of the expression for a `match` guard is the scope of the arm that the guard is for.

[destructors.scope.nesting-match-arm](#r-destructors.scope.nesting-match-arm)

- The parent of the expression after the `=>` in a `match` expression is the scope of the arm that it’s in.

[destructors.scope.nesting-match](#r-destructors.scope.nesting-match)

- The parent of the arm scope is the scope of the `match` expression that it belongs to.

[destructors.scope.nesting-other](#r-destructors.scope.nesting-other)

- The parent of all other scopes is the scope of the immediately enclosing expression.

[destructors.scope.nesting-drop-order](#r-destructors.scope.nesting-drop-order)

When multiple scopes are left at once, such as when returning from a function, variables are dropped from the inside outwards.

[destructors.scope.params](#r-destructors.scope.params)

### Scopes of function parameters

All function parameters are in the scope of the entire function body, so are dropped last when evaluating the function. Each actual function parameter is dropped after any bindings introduced in that parameter’s pattern.

``` playground
#![allow(unused)]
fn main() {
struct PrintOnDrop(&'static str);
impl Drop for PrintOnDrop {
    fn drop(&mut self) {
        println!("drop({})", self.0);
    }
}
// Drops `y`, then the second parameter, then `x`, then the first parameter
fn patterns_in_parameters(
    (x, _): (PrintOnDrop, PrintOnDrop),
    (_, y): (PrintOnDrop, PrintOnDrop),
) {}

// drop order is 3 2 0 1
patterns_in_parameters(
    (PrintOnDrop("0"), PrintOnDrop("1")),
    (PrintOnDrop("2"), PrintOnDrop("3")),
);
}
```

[destructors.scope.bindings](#r-destructors.scope.bindings)

### Scopes of local variables

[destructors.scope.bindings.let](#r-destructors.scope.bindings.let)

Local variables declared in a `let` statement are associated to the scope of the block that contains the `let` statement.

``` playground
#![allow(unused)]
fn main() {
struct PrintOnDrop(&'static str);
impl Drop for PrintOnDrop {
    fn drop(&mut self) {
        println!("drop({})", self.0);
    }
}
let declared_first = PrintOnDrop("Dropped last in outer scope");
{
    let declared_in_block = PrintOnDrop("Dropped in inner scope");
}
let declared_last = PrintOnDrop("Dropped first in outer scope");
}
```

[destructors.scope.bindings.match-arm](#r-destructors.scope.bindings.match-arm)

Local variables declared in a `match` expression or pattern-matching `match` guard are associated to the arm scope of the `match` arm that they are declared in.

``` playground
#![allow(unused)]
fn main() {
#![allow(irrefutable_let_patterns)]
struct PrintOnDrop(&'static str);
impl Drop for PrintOnDrop {
    fn drop(&mut self) {
        println!("drop({})", self.0);
    }
}
match PrintOnDrop("Dropped last in the first arm's scope") {
    // When guard evaluation succeeds, control-flow stays in the arm and
    // values may be moved from the scrutinee into the arm's bindings,
    // causing them to be dropped in the arm's scope.
    x if let y = PrintOnDrop("Dropped second in the first arm's scope")
        && let z = PrintOnDrop("Dropped first in the first arm's scope") =>
    {
        let declared_in_block = PrintOnDrop("Dropped in inner scope");
        // Pattern-matching guards' bindings and temporaries are dropped in
        // reverse order, dropping each guard condition operand's bindings
        // before its temporaries. Lastly, variables bound by the arm's
        // pattern are dropped.
    }
    _ => unreachable!(),
}

match PrintOnDrop("Dropped in the enclosing temporary scope") {
    // When guard evaluation fails, control-flow leaves the arm scope,
    // causing bindings and temporaries from earlier pattern-matching
    // guard condition operands to be dropped. This occurs before evaluating
    // the next arm's guard or body.
    _ if let y = PrintOnDrop("Dropped in the first arm's scope")
        && false => unreachable!(),
    // When a guard is executed multiple times due to self-overlapping
    // or-patterns, control-flow leaves the arm scope when the guard fails
    // and re-enters the arm scope before executing the guard again.
    _ | _ if let y = PrintOnDrop("Dropped in the second arm's scope twice")
        && false => unreachable!(),
    _ => {},
}
}
```

[destructors.scope.bindings.patterns](#r-destructors.scope.bindings.patterns)

Variables in patterns are dropped in reverse order of declaration within the pattern.

``` playground
#![allow(unused)]
fn main() {
struct PrintOnDrop(&'static str);
impl Drop for PrintOnDrop {
    fn drop(&mut self) {
        println!("drop({})", self.0);
    }
}
let (declared_first, declared_last) = (
    PrintOnDrop("Dropped last"),
    PrintOnDrop("Dropped first"),
);
}
```

[destructors.scope.bindings.or-patterns](#r-destructors.scope.bindings.or-patterns)

For the purpose of drop order, [or-patterns](patterns.md#or-patterns) declare bindings in the order given by the first subpattern.

``` playground
#![allow(unused)]
fn main() {
struct PrintOnDrop(&'static str);
impl Drop for PrintOnDrop {
    fn drop(&mut self) {
        println!("drop({})", self.0);
    }
}
// Drops `x` before `y`.
fn or_pattern_drop_order<T>(
    (Ok([x, y]) | Err([y, x])): Result<[T; 2], [T; 2]>
//   ^^^^^^^^^^   ^^^^^^^^^^^ This is the second subpattern.
//   |
//   This is the first subpattern.
//
//   In the first subpattern, `x` is declared before `y`. Since it is
//   the first subpattern, that is the order used even if the second
//   subpattern, where the bindings are declared in the opposite
//   order, is matched.
) {}

// Here we match the first subpattern, and the drops happen according
// to the declaration order in the first subpattern.
or_pattern_drop_order(Ok([
    PrintOnDrop("Declared first, dropped last"),
    PrintOnDrop("Declared last, dropped first"),
]));

// Here we match the second subpattern, and the drops still happen
// according to the declaration order in the first subpattern.
or_pattern_drop_order(Err([
    PrintOnDrop("Declared last, dropped first"),
    PrintOnDrop("Declared first, dropped last"),
]));
}
```

[destructors.scope.temporary](#r-destructors.scope.temporary)

### Temporary scopes

[destructors.scope.temporary.intro](#r-destructors.scope.temporary.intro)

The *temporary scope* of an expression is the scope that is used for the temporary variable that holds the result of that expression when used in a [place context](expressions.md#place-expressions-and-value-expressions), unless it is [promoted](#constant-promotion).

[destructors.scope.temporary.enclosing](#r-destructors.scope.temporary.enclosing)

Apart from lifetime extension, the temporary scope of an expression is the smallest scope that contains the expression and is one of the following:

- The entire function.
- A statement.
- The body of an [`if`](expressions/if-expr.md#if-expressions), [`while`](expressions/loop-expr.md#predicate-loops) or [`loop`](expressions/loop-expr.md#infinite-loops) expression.
- The `else` block of an `if` expression.
- The non-pattern matching condition expression of an `if` or `while` expression or a non-pattern-matching `match` [guard condition operand](expressions/match-expr.md#match-guard-chains).
- The pattern-matching guard, if present, and body expression for a `match` arm.
- Each operand of a [lazy boolean expression](expressions/operator-expr.md#lazy-boolean-operators).
- The pattern-matching condition(s) and consequent body of [`if`](expressions/if-expr.md#if-expressions) ([destructors.scope.temporary.edition2024](#r-destructors.scope.temporary.edition2024)).
- The pattern-matching condition and loop body of [`while`](expressions/loop-expr.md#predicate-loops).
- The entirety of the tail expression of a block ([destructors.scope.temporary.edition2024](#r-destructors.scope.temporary.edition2024)).

> ![](../assets/inline/destructors-1.svg)Note
>
> The [scrutinee](glossary.md#scrutinee) of a `match` expression is not a temporary scope, so temporaries in the scrutinee can be dropped after the `match` expression. For example, the temporary for `1` in `match 1 { ref mut z => z };` lives until the end of the statement.

> ![](../assets/inline/destructors-2.svg)Note
>
> The desugaring of a [destructuring assignment](expressions/operator-expr.md#r-expr.assign.destructure) restricts the temporary scope of its assigned value operand (the RHS). For details, see [expr.assign.destructure.tmp-scopes](expressions/operator-expr.md#r-expr.assign.destructure.tmp-scopes).

[destructors.scope.temporary.edition2024](#r-destructors.scope.temporary.edition2024)

> 2024 Edition differences
>
> The 2024 edition added two new temporary scope narrowing rules: `if let` temporaries are dropped before the `else` block, and temporaries of tail expressions of blocks are dropped immediately after the tail expression is evaluated.

Some examples:

``` playground
#![allow(unused)]
fn main() {
#![allow(irrefutable_let_patterns)]
struct PrintOnDrop(&'static str);
impl Drop for PrintOnDrop {
    fn drop(&mut self) {
        println!("drop({})", self.0);
    }
}
let local_var = PrintOnDrop("local var");

// Dropped once the condition has been evaluated
if PrintOnDrop("If condition").0 == "If condition" {
    // Dropped at the end of the block
    PrintOnDrop("If body").0
} else {
    unreachable!()
};

if let "if let scrutinee" = PrintOnDrop("if let scrutinee").0 {
    PrintOnDrop("if let consequent").0
    // `if let consequent` dropped here
}
// `if let scrutinee` is dropped here
else {
    PrintOnDrop("if let else").0
    // `if let else` dropped here
};

while let x = PrintOnDrop("while let scrutinee").0 {
    PrintOnDrop("while let loop body").0;
    break;
    // `while let loop body` dropped here.
    // `while let scrutinee` dropped here.
}

// Dropped before the first ||
(PrintOnDrop("first operand").0 == ""
// Dropped before the )
|| PrintOnDrop("second operand").0 == "")
// Dropped before the ;
|| PrintOnDrop("third operand").0 == "";

// Scrutinee is dropped at the end of the function, before local variables
// (because this is the tail expression of the function body block).
match PrintOnDrop("Matched value in final expression") {
    // Non-pattern-matching guards' temporaries are dropped once the
    // condition has been evaluated
    _ if PrintOnDrop("guard condition").0 == "" => (),
    // Pattern-matching guards' temporaries are dropped when leaving the
    // arm's scope
    _ if let "guard scrutinee" = PrintOnDrop("guard scrutinee").0 => {
        let _ = &PrintOnDrop("lifetime-extended temporary in inner scope");
        // `lifetime-extended temporary in inner scope` is dropped here
    }
    // `guard scrutinee` is dropped here
    _ => (),
}
}
```

[destructors.scope.operands](#r-destructors.scope.operands)

### Operands

Temporaries are also created to hold the result of operands to an expression while the other operands are evaluated. The temporaries are associated to the scope of the expression with that operand. Since the temporaries are moved from once the expression is evaluated, dropping them has no effect unless one of the operands to an expression breaks out of the expression, returns, or [panics](panic.md).

``` playground
#![allow(unused)]
fn main() {
struct PrintOnDrop(&'static str);
impl Drop for PrintOnDrop {
    fn drop(&mut self) {
        println!("drop({})", self.0);
    }
}
loop {
    // Tuple expression doesn't finish evaluating so operands drop in reverse order
    (
        PrintOnDrop("Outer tuple first"),
        PrintOnDrop("Outer tuple second"),
        (
            PrintOnDrop("Inner tuple first"),
            PrintOnDrop("Inner tuple second"),
            break,
        ),
        PrintOnDrop("Never created"),
    );
}
}
```

[destructors.scope.const-promotion](#r-destructors.scope.const-promotion)

### Constant promotion

Promotion of a value expression to a `'static` slot occurs when the expression could be written in a constant and borrowed, and that borrow could be dereferenced where the expression was originally written, without changing the runtime behavior. That is, the promoted expression can be evaluated at compile-time and the resulting value does not contain [interior mutability](interior-mutability.md) or [destructors]() (these properties are determined based on the value where possible, e.g. `&None` always has the type `&'static Option<_>`, as it contains nothing disallowed).

[destructors.scope.lifetime-extension](#r-destructors.scope.lifetime-extension)

### Temporary lifetime extension

[destructors.scope.lifetime-extension.intro](#r-destructors.scope.lifetime-extension.intro)

> ![](../assets/inline/destructors-3.svg)Note
>
> The exact rules for temporary lifetime extension are subject to change. This is describing the current behavior only.

[destructors.scope.lifetime-extension.let](#r-destructors.scope.lifetime-extension.let)

The temporary scopes for expressions in `let` statements are sometimes *extended* to the scope of the block containing the `let` statement. This is done when the usual temporary scope would be too small, based on certain syntactic rules. For example:

``` playground
#![allow(unused)]
fn main() {
let x = &mut 0;
// Usually a temporary would be dropped by now, but the temporary for `0` lives
// to the end of the block.
println!("{}", x);
}
```

[destructors.scope.lifetime-extension.static](#r-destructors.scope.lifetime-extension.static)

Lifetime extension also applies to `static` and `const` items, where it makes temporaries live until the end of the program. For example:

``` playground
#![allow(unused)]
fn main() {
const C: &Vec<i32> = &Vec::new();
// Usually this would be a dangling reference as the `Vec` would only
// exist inside the initializer expression of `C`, but instead the
// borrow gets lifetime-extended so it effectively has `'static` lifetime.
println!("{:?}", C);
}
```

[destructors.scope.lifetime-extension.sub-expressions](#r-destructors.scope.lifetime-extension.sub-expressions)

If a [borrow](expressions/operator-expr.md#r-expr.operator.borrow), [dereference](expressions/operator-expr.md#the-dereference-operator), [field](expressions/field-expr.md), or [tuple indexing expression](expressions/tuple-expr.md#tuple-indexing-expressions) has an extended temporary scope, then so does its operand. If an [indexing expression](expressions/array-expr.md#array-and-slice-indexing-expressions) has an extended temporary scope, then the indexed expression also has an extended temporary scope.

[destructors.scope.lifetime-extension.patterns](#r-destructors.scope.lifetime-extension.patterns)

#### Extending based on patterns

[destructors.scope.lifetime-extension.patterns.extending](#r-destructors.scope.lifetime-extension.patterns.extending)

An *extending pattern* is either:

- An [identifier pattern](patterns.md#identifier-patterns) that binds by reference or mutable reference.

  ``` playground
  #![allow(unused)]
  fn main() {
  fn temp() {}
  let ref x = temp(); // Binds by reference.
  x;
  let ref mut x = temp(); // Binds by mutable reference.
  x;
  }
  ```

- A [struct](patterns.md#struct-patterns), [tuple](patterns.md#tuple-patterns), [tuple struct](patterns.md#tuple-struct-patterns), [slice](patterns.md#slice-patterns), or [or-pattern](patterns.md#or-patterns) where at least one of the direct subpatterns is an extending pattern.

  ``` playground
  #![allow(unused)]
  fn main() {
  use core::sync::atomic::{AtomicU64, Ordering::Relaxed};
  static X: AtomicU64 = AtomicU64::new(0);
  struct W<T>(T);
  impl<T> Drop for W<T> { fn drop(&mut self) { X.fetch_add(1, Relaxed); } }
  let W { 0: ref x } = W(()); // Struct pattern.
  x;
  let W(ref x) = W(()); // Tuple struct pattern.
  x;
  let (W(ref x),) = (W(()),); // Tuple pattern.
  x;
  let [W(ref x), ..] = [W(())]; // Slice pattern.
  x;
  let (Ok(W(ref x)) | Err(&ref x)) = Ok(W(())); // Or pattern.
  x;
  //
  // All of the temporaries above are still live here.
  assert_eq!(0, X.load(Relaxed));
  }
  ```

So `ref x`, `V(ref x)` and `[ref x, y]` are all extending patterns, but `x`, `&ref x` and `&(ref x,)` are not.

[destructors.scope.lifetime-extension.patterns.let](#r-destructors.scope.lifetime-extension.patterns.let)

If the pattern in a `let` statement is an extending pattern then the temporary scope of the initializer expression is extended.

``` playground
#![allow(unused)]
fn main() {
fn temp() {}
// This is an extending pattern, so the temporary scope is extended.
let ref x = *&temp(); // OK
x;
}
```

``` playground
#![allow(unused)]
fn main() {
fn temp() {}
// This is neither an extending pattern nor an extending expression,
// so the temporary is dropped at the semicolon.
let &ref x = *&&temp(); // ERROR
x;
}
```

``` playground
#![allow(unused)]
fn main() {
fn temp() {}
// This is not an extending pattern but it is an extending expression,
// so the temporary lives beyond the `let` statement.
let &ref x = &*&temp(); // OK
x;
}
```

[destructors.scope.lifetime-extension.exprs](#r-destructors.scope.lifetime-extension.exprs)\
Tests Tests with this rule:

- [tests/ui/lifetimes/temporary-lifetime-extension-tuple-ctor.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/lifetimes/temporary-lifetime-extension-tuple-ctor.rs)

#### Extending based on expressions

[destructors.scope.lifetime-extension.exprs.extending](#r-destructors.scope.lifetime-extension.exprs.extending)

For a let statement with an initializer, an *extending expression* is an expression which is one of the following:

- The initializer expression.
- The operand of an extending [borrow](expressions/operator-expr.md#r-expr.operator.borrow) expression.
- The [super operands](expressions.md#r-expr.super-macros) of an extending [super macro call](expressions.md#r-expr.super-macros) expression.
- The operand(s) of an extending [array](expressions/array-expr.md#array-expressions), [cast](expressions/operator-expr.md#type-cast-expressions), [braced struct](expressions/struct-expr.md), or [tuple](expressions/tuple-expr.md#tuple-expressions) expression.
- The arguments to an extending [tuple struct](types/struct.md#r-type.struct.tuple) or [tuple enum variant](types/enum.md#r-type.enum.declaration) constructor expression.
- The final expression of an extending [block expression](expressions/block-expr.md) except for an [async block expression](expressions/block-expr.md#r-expr.block.async).
- The final expression of an extending [`if`](expressions/if-expr.md#if-expressions) expression’s consequent, `else if`, or `else` block.
- An arm expression of an extending [`match`](expressions/match-expr.md) expression.

> ![](../assets/inline/destructors-4.svg)Note
>
> The desugaring of a [destructuring assignment](expressions/operator-expr.md#r-expr.assign.destructure) makes its assigned value operand (the RHS) an extending expression within a newly-introduced block. For details, see [expr.assign.destructure.tmp-ext](expressions/operator-expr.md#r-expr.assign.destructure.tmp-ext).

So the borrow expressions in `&mut 0`, `(&1, &mut 2)`, and `Some(&mut 3)` are all extending expressions. The borrows in `&0 + &1` and `f(&mut 0)` are not.

[destructors.scope.lifetime-extension.exprs.borrows](#r-destructors.scope.lifetime-extension.exprs.borrows)

The operand of an extending [borrow](expressions/operator-expr.md#r-expr.operator.borrow) expression has its [temporary scope](#r-destructors.scope.temporary) [extended](#r-destructors.scope.lifetime-extension).

[destructors.scope.lifetime-extension.exprs.super-macros](#r-destructors.scope.lifetime-extension.exprs.super-macros)

The [super temporaries](expressions.md#r-expr.super-macros) of an extending [super macro call](expressions.md#r-expr.super-macros) expression have their [scopes](#r-destructors.scope.temporary) [extended](#r-destructors.scope.lifetime-extension).

> ![](../assets/inline/destructors-5.svg)Note
>
> `rustc` does not treat [array repeat operands](expressions/array-expr.md#r-expr.array.repeat-operand) of extending [array](types/array.md) expressions as extending expressions. Whether it should is an open question.
>
> For details, see [Rust issue \#146092](https://github.com/rust-lang/rust/issues/146092).

#### Examples

Here are some examples where expressions have extended temporary scopes:

``` playground
#![allow(unused)]
fn main() {
use core::pin::pin;
use core::sync::atomic::{AtomicU64, Ordering::Relaxed};
static X: AtomicU64 = AtomicU64::new(0);
#[derive(Debug)] struct S;
impl Drop for S { fn drop(&mut self) { X.fetch_add(1, Relaxed); } }
const fn temp() -> S { S }
let x = &temp(); // Operand of borrow.
x;
let x = &raw const *&temp(); // Operand of raw borrow.
assert_eq!(X.load(Relaxed), 0);
let x = &temp() as &dyn Send; // Operand of cast.
x;
let x = (&*&temp(),); // Operand of tuple constructor.
x;
struct W<T>(T);
let x = W(&temp()); // Argument to tuple struct constructor.
x;
let x = Some(&temp()); // Argument to tuple enum variant constructor.
x;
let x = { [Some(&temp())] }; // Final expr of block.
x;
let x = const { &temp() }; // Final expr of `const` block.
x;
let x = unsafe { &temp() }; // Final expr of `unsafe` block.
x;
let x = if true { &temp() } else { &temp() };
//              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//           Final exprs of `if`/`else` blocks.
x;
let x = match () { _ => &temp() }; // `match` arm expression.
x;
let x = pin!(temp()); // Super operand of super macro call expression.
x;
let x = pin!({ &mut temp() }); // As above.
x;
let x = format_args!("{:?}", temp()); // As above.
x;
//
// All of the temporaries above are still live here.
assert_eq!(0, X.load(Relaxed));
}
```

Here are some examples where expressions don’t have extended temporary scopes:

``` playground
#![allow(unused)]
fn main() {
fn temp() {}
// Arguments to function calls are not extending expressions. The
// temporary is dropped at the semicolon.
let x = core::convert::identity(&temp()); // ERROR
x;
}
```

``` playground
#![allow(unused)]
fn main() {
fn temp() {}
trait Use { fn use_temp(&self) -> &Self { self } }
impl Use for () {}
// Receivers of method calls are not extending expressions.
let x = (&temp()).use_temp(); // ERROR
x;
}
```

``` playground
#![allow(unused)]
fn main() {
fn temp() {}
// Scrutinees of match expressions are not extending expressions.
let x = match &temp() { x => x }; // ERROR
x;
}
```

``` playground
#![allow(unused)]
fn main() {
fn temp() {}
// Final expressions of `async` blocks are not extending expressions.
let x = async { &temp() }; // ERROR
x;
}
```

``` playground
#![allow(unused)]
fn main() {
fn temp() {}
// Final expressions of closures are not extending expressions.
let x = || &temp(); // ERROR
x;
}
```

``` playground
#![allow(unused)]
fn main() {
fn temp() {}
// Operands of loop breaks are not extending expressions.
let x = loop { break &temp() }; // ERROR
x;
}
```

``` playground
#![allow(unused)]
fn main() {
fn temp() {}
// Operands of breaks to labels are not extending expressions.
let x = 'a: { break 'a &temp() }; // ERROR
x;
}
```

``` playground
#![allow(unused)]
fn main() {
use core::pin::pin;
fn temp() {}
// The argument to `pin!` is only an extending expression if the call
// is an extending expression. Since it's not, the inner block is not
// an extending expression, so the temporaries in its trailing
// expression are dropped immediately.
pin!({ &temp() }); // ERROR
}
```

``` playground
#![allow(unused)]
fn main() {
fn temp() {}
// As above.
format_args!("{:?}", { &temp() }); // ERROR
}
```

[destructors.forget](#r-destructors.forget)

## Not running destructors

[destructors.manually-suppressing](#r-destructors.manually-suppressing)\
Tests Tests with this rule:

- [tests/ui/union/union-drop.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/union/union-drop.rs)

### Manually suppressing destructors

[`core::mem::forget`](../core/mem/fn.forget.html) can be used to prevent the destructor of a variable from being run, and [`core::mem::ManuallyDrop`](../core/mem/manually_drop/struct.ManuallyDrop.html) provides a wrapper to prevent a variable or field from being dropped automatically.

> ![](../assets/inline/destructors-6.svg)Note
>
> Preventing a destructor from being run via [`core::mem::forget`](../core/mem/fn.forget.html) or other means is safe even if it has a type that isn’t `'static`. Besides the places where destructors are guaranteed to run as defined by this document, types may *not* safely rely on a destructor being run for soundness.

[destructors.process-termination](#r-destructors.process-termination)

### Process termination without unwinding

There are some ways to terminate the process without [unwinding](panic.md#unwinding), in which case destructors will not be run.

The standard library provides [`std::process::exit`](../std/process/fn.exit.html) and [`std::process::abort`](../std/process/fn.abort.html) to do this explicitly. Additionally, if the [panic handler](panic.md#r-panic.panic_handler.std) is set to `abort`, panicking will always terminate the process without destructors being run.

There is one additional case to be aware of: when a panic reaches a [non-unwinding ABI boundary](items/functions.md#unwinding), either no destructors will run, or all destructors up until the ABI boundary will run.
