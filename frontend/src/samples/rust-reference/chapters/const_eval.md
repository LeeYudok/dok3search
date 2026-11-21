# Constant evaluation

[const-eval](#r-const-eval)

# Constant evaluation

[const-eval.intro](#r-const-eval.intro)

Constant evaluation is the process of computing the result of [expressions](expressions.md) during compilation. Only a subset of all expressions can be evaluated at compile-time.

[const-eval.const-expr](#r-const-eval.const-expr)

## Constant expressions

[const-eval.const-expr.intro](#r-const-eval.const-expr.intro)

Certain forms of expressions, called constant expressions, can be evaluated at compile time.

[const-eval.const-expr.const-context](#r-const-eval.const-expr.const-context)

Expressions in a [const context](#const-context) must be constant expressions.

[const-eval.const-expr.evaluation](#r-const-eval.const-expr.evaluation)

Expressions in const contexts are always evaluated at compile time.

[const-eval.const-expr.runtime-context](#r-const-eval.const-expr.runtime-context)

Outside of const contexts, constant expressions *may* be, but are not guaranteed to be, evaluated at compile time.

[const-eval.const-expr.error](#r-const-eval.const-expr.error)

Behaviors such as out of bounds [array indexing](expressions/array-expr.md#array-and-slice-indexing-expressions) or [overflow](expressions/operator-expr.md#overflow) are compiler errors if the value must be evaluated at compile time (i.e. in const contexts). Otherwise, these behaviors are warnings, but will likely panic at run-time.

[const-eval.const-expr.list](#r-const-eval.const-expr.list)

The following expressions are constant expressions, so long as any operands are also constant expressions and do not cause any [`Drop::drop`](destructors.md) calls to be run.

[const-eval.const-expr.literal](#r-const-eval.const-expr.literal)

- [Literals](expressions/literal-expr.md).

[const-eval.const-expr.parameter](#r-const-eval.const-expr.parameter)

- [Const parameters](items/generics.md).

[const-eval.const-expr.path-item](#r-const-eval.const-expr.path-item)

- [Paths](expressions/path-expr.md) to [functions](items/functions.md) and [constants](items/constant-items.md). Recursively defining constants is not allowed.

[const-eval.const-expr.path-static](#r-const-eval.const-expr.path-static)\
Tests Tests with this rule:

- [tests/ui/extern/issue-28324.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/extern/issue-28324.rs)

- Paths to [statics](items/static-items.md) with these restrictions:

  - Writes to `static` items are not allowed in any constant evaluation context.
  - Reads from `extern` statics are not allowed in any constant evaluation context.
  - If the evaluation is *not* carried out in an initializer of a `static` item, then reads from any mutable `static` are not allowed. A mutable `static` is a `static mut` item, or a `static` item with an interior-mutable type.

  These requirements are checked only when the constant is evaluated. In other words, having such accesses syntactically occur in const contexts is allowed as long as they never get executed.

[const-eval.const-expr.tuple](#r-const-eval.const-expr.tuple)

- [Tuple expressions](expressions/tuple-expr.md).

[const-eval.const-expr.array](#r-const-eval.const-expr.array)

- [Array expressions](expressions/array-expr.md).

[const-eval.const-expr.constructor](#r-const-eval.const-expr.constructor)

- [Struct expressions](expressions/struct-expr.md).

[const-eval.const-expr.block](#r-const-eval.const-expr.block)

- [Block expressions](expressions/block-expr.md), including `unsafe` and `const` blocks.
  - [let statements](statements.md#let-statements) and thus irrefutable [patterns](patterns.md), including mutable bindings
  - [assignment expressions](expressions/operator-expr.md#assignment-expressions)
  - [compound assignment expressions](expressions/operator-expr.md#compound-assignment-expressions)
  - [expression statements](statements.md#expression-statements)

[const-eval.const-expr.field](#r-const-eval.const-expr.field)

- [Field expressions](expressions/field-expr.md).

[const-eval.const-expr.index](#r-const-eval.const-expr.index)

- [Array and slice indexing expressions](expressions/array-expr.md#array-and-slice-indexing-expressions), where the index is a `usize`.

[const-eval.const-expr.range](#r-const-eval.const-expr.range)

- [Range expressions](expressions/range-expr.md).

[const-eval.const-expr.closure](#r-const-eval.const-expr.closure)

- [Closure expressions](expressions/closure-expr.md) which don’t capture variables from the environment.

[const-eval.const-expr.builtin-arith-logic](#r-const-eval.const-expr.builtin-arith-logic)

- Built-in [negation](expressions/operator-expr.md#negation-operators), [arithmetic](expressions/operator-expr.md#arithmetic-and-logical-binary-operators), [logical](expressions/operator-expr.md#arithmetic-and-logical-binary-operators), [comparison](expressions/operator-expr.md#comparison-operators) or [lazy boolean](expressions/operator-expr.md#lazy-boolean-operators) operators used on integer and floating point types, `bool`, and `char`.

[const-eval.const-expr.borrows](#r-const-eval.const-expr.borrows)

- All forms of [borrow](expressions/operator-expr.md#borrow-operators)s, including raw borrows, except borrows of expressions whose temporary scopes would be extended (see [temporary lifetime extension](destructors.md#r-destructors.scope.lifetime-extension)) to the end of the program and which are either:

  - Mutable borrows.
  - Shared borrows of expressions that result in values with [interior mutability](interior-mutability.md).

  ``` playground
  #![allow(unused)]
  fn main() {
  // Due to being in tail position, this borrow extends the scope of the
  // temporary to the end of the program. Since the borrow is mutable,
  // this is not allowed in a const expression.
  const C: &u8 = &mut 0; // ERROR not allowed
  }
  ```

  ``` playground
  #![allow(unused)]
  fn main() {
  // Const blocks are similar to initializers of `const` items.
  let _: &u8 = const { &mut 0 }; // ERROR not allowed
  }
  ```

  ``` playground
  #![allow(unused)]
  fn main() {
  use core::sync::atomic::AtomicU8;
  // This is not allowed as 1) the temporary scope is extended to the
  // end of the program and 2) the temporary has interior mutability.
  const C: &AtomicU8 = &AtomicU8::new(0); // ERROR not allowed
  }
  ```

  ``` playground
  #![allow(unused)]
  fn main() {
  use core::sync::atomic::AtomicU8;
  // As above.
  let _: &_ = const { &AtomicU8::new(0) }; // ERROR not allowed
  }
  ```

  ``` playground
  #![allow(unused)]
  fn main() {
  #![allow(static_mut_refs)]
  // Even though this borrow is mutable, it's not of a temporary, so
  // this is allowed.
  const C: &u8 = unsafe { static mut S: u8 = 0; &mut S }; // OK
  }
  ```

  ``` playground
  #![allow(unused)]
  fn main() {
  use core::sync::atomic::AtomicU8;
  // Even though this borrow is of a value with interior mutability,
  // it's not of a temporary, so this is allowed.
  const C: &AtomicU8 = {
      static S: AtomicU8 = AtomicU8::new(0); &S // OK
  };
  }
  ```

  ``` playground
  #![allow(unused)]
  fn main() {
  use core::sync::atomic::AtomicU8;
  // This shared borrow of an interior mutable temporary is allowed
  // because its scope is not extended.
  const C: () = { _ = &AtomicU8::new(0); }; // OK
  }
  ```

  ``` playground
  #![allow(unused)]
  fn main() {
  // Even though the borrow is mutable and the temporary lives to the
  // end of the program due to promotion, this is allowed because the
  // borrow is not in tail position and so the scope of the temporary
  // is not extended via temporary lifetime extension.
  const C: () = { let _: &'static mut [u8] = &mut []; }; // OK
  //                                              ~~
  //                                     Promoted temporary.
  }
  ```

  > ![](../assets/inline/const_eval-1.svg)Note
  >
  > In other words — to focus on what’s allowed rather than what’s not allowed — shared borrows of interior mutable data and mutable borrows are only allowed in a [const context](#const-context) when the borrowed [place expression](expressions.md#r-expr.place-value.place-memory-location) is *transient*, *indirect*, or *static*.
  >
  > A place expression is *transient* if it is a variable local to the current const context or an expression whose temporary scope is contained inside the current const context.
  >
  > ``` playground
  > #![allow(unused)]
  > fn main() {
  > // The borrow is of a variable local to the initializer, therefore
  > // this place expression is transient.
  > const C: () = { let mut x = 0; _ = &mut x; };
  > }
  > ```
  >
  > ``` playground
  > #![allow(unused)]
  > fn main() {
  > // The borrow is of a temporary whose scope has not been extended,
  > // therefore this place expression is transient.
  > const C: () = { _ = &mut 0u8; };
  > }
  > ```
  >
  > ``` playground
  > #![allow(unused)]
  > fn main() {
  > // When a temporary is promoted but not lifetime extended, its
  > // place expression is still treated as transient.
  > const C: () = { let _: &'static mut [u8] = &mut []; };
  > }
  > ```
  >
  > A place expression is *indirect* if it is a [dereference expression](expressions/operator-expr.md#r-expr.deref).
  >
  > ``` playground
  > #![allow(unused)]
  > fn main() {
  > const C: () = { _ = &mut *(&mut 0); };
  > }
  > ```
  >
  > A place expression is *static* if it is a `static` item.
  >
  > ``` playground
  > #![allow(unused)]
  > fn main() {
  > #![allow(static_mut_refs)]
  > const C: &u8 = unsafe { static mut S: u8 = 0; &mut S };
  > }
  > ```

  > ![](../assets/inline/const_eval-2.svg)Note
  >
  > One surprising consequence of these rules is that we allow this,
  >
  > ``` playground
  > #![allow(unused)]
  > fn main() {
  > const C: &[u8] = { let x: &mut [u8] = &mut []; x }; // OK
  > //                                    ~~~~~~~
  > // Empty arrays are promoted even behind mutable borrows.
  > }
  > ```
  >
  > but we disallow this similar code:
  >
  > ``` playground
  > #![allow(unused)]
  > fn main() {
  > const C: &[u8] = &mut []; // ERROR
  > //               ~~~~~~~
  > //           Tail expression.
  > }
  > ```
  >
  > The difference between these is that, in the first, the empty array is [promoted](destructors.md#constant-promotion) but its scope does not undergo [temporary lifetime extension](destructors.md#r-destructors.scope.lifetime-extension), so we consider the [place expression](expressions.md#r-expr.place-value.place-memory-location) to be transient (even though after promotion the place indeed lives to the end of the program). In the second, the scope of the empty array temporary does undergo lifetime extension, and so it is rejected due to being a mutable borrow of a lifetime-extended temporary (and therefore borrowing a non-transient place expression).
  >
  > The effect is surprising because temporary lifetime extension, in this case, causes less code to compile than would without it.
  >
  > See [issue \#143129](https://github.com/rust-lang/rust/issues/143129) for more details.

[const-eval.const-expr.deref](#r-const-eval.const-expr.deref)

- [Dereference expressions](expressions/operator-expr.md#r-expr.deref).

  ``` playground
  #![allow(unused)]
  fn main() {
  use core::cell::UnsafeCell;
  const _: u8 = unsafe {
      let x: *mut u8 = &raw mut *&mut 0;
      //                        ^^^^^^^
      //             Dereference of mutable reference.
      *x = 1; // Dereference of mutable pointer.
      *(x as *const u8) // Dereference of constant pointer.
  };
  const _: u8 = unsafe {
      let x = &UnsafeCell::new(0);
      *x.get() = 1; // Mutation of interior mutable value.
      *x.get()
  };
  }
  ```

[const-eval.const-expr.group](#r-const-eval.const-expr.group)

- [Grouped](expressions/grouped-expr.md) expressions.

[const-eval.const-expr.cast](#r-const-eval.const-expr.cast)

- [Cast](expressions/operator-expr.md#type-cast-expressions) expressions, except
  - pointer to address casts and
  - function pointer to address casts.

[const-eval.const-expr.const-fn](#r-const-eval.const-expr.const-fn)

- Calls of [const functions](items/functions.md#const-functions) and const methods.

[const-eval.const-expr.loop](#r-const-eval.const-expr.loop)

- [loop](expressions/loop-expr.md#infinite-loops) and [while](expressions/loop-expr.md#predicate-loops) expressions.

[const-eval.const-expr.if-match](#r-const-eval.const-expr.if-match)

- [if](expressions/if-expr.md#if-expressions) and [match](expressions/match-expr.md) expressions.

[const-eval.const-context](#r-const-eval.const-context)

## Const context

[const-eval.const-context.def](#r-const-eval.const-context.def)

A *const context* is one of the following:

[const-eval.const-context.array-length](#r-const-eval.const-context.array-length)

- [Array type length expressions](types/array.md)

[const-eval.const-context.repeat-length](#r-const-eval.const-context.repeat-length)

- [Array repeat length expressions](expressions/array-expr.md)

[const-eval.const-context.init](#r-const-eval.const-context.init)

- The initializer of
  - [constants](items/constant-items.md)
  - [statics](items/static-items.md)
  - [enum discriminants](items/enumerations.md#discriminants)

[const-eval.const-context.generic](#r-const-eval.const-context.generic)

- A [const generic argument](items/generics.md#const-generics)

[const-eval.const-context.block](#r-const-eval.const-context.block)

- A [const block](expressions/block-expr.md#const-blocks)

[const-eval.const-context.outer-generics](#r-const-eval.const-context.outer-generics)

Array type length expressions, array repeat length expressions, and const generic arguments are restricted in their use of outer generic parameters: such an expression must either be a single const generic parameter, or an expression that does not reference any generic parameters.

[const-eval.const-fn](#r-const-eval.const-fn)

## Const functions

[const-eval.const-fn.intro](#r-const-eval.const-fn.intro)

A *const function* is a function that can be called from a const context. It is defined with the `const` qualifier, and also includes [tuple struct](items/structs.md) and [tuple enum variant](items/enumerations.md) constructors.

> ![](../assets/inline/const_eval-3.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> const fn square(x: i32) -> i32 { x * x }
>
> const VALUE: i32 = square(12);
> }
> ```

[const-eval.const-fn.const-context](#r-const-eval.const-fn.const-context)

When called from a const context, a const function is interpreted by the compiler at compile time. The interpretation happens in the environment of the compilation target and not the host. So `usize` is `32` bits if you are compiling against a `32` bit system, irrelevant of whether you are building on a `64` bit or a `32` bit system.

[const-eval.const-fn.outside-context](#r-const-eval.const-fn.outside-context)

When a const function is called from outside a const context, it behaves the same as if it did not have the `const` qualifier.

[const-eval.const-fn.body-restriction](#r-const-eval.const-fn.body-restriction)

The body of a const function may only use [constant expressions](#constant-expressions).

[const-eval.const-fn.async](#r-const-eval.const-fn.async)

Const functions are not allowed to be [async](items/functions.md#async-functions).

[const-eval.const-fn.type-restrictions](#r-const-eval.const-fn.type-restrictions)

The types of a const function’s parameters and return type are restricted to those that are compatible with a const context.
