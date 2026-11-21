# Scopes

[names.scopes](#r-names.scopes)

# Scopes

[names.scopes.intro](#r-names.scopes.intro)

A *scope* is the region of source text where a named [entity](../names.md) may be referenced with that name. The following sections provide details on the scoping rules and behavior, which depend on the kind of entity and where it is declared. The process of how names are resolved to entities is described in the [name resolution](name-resolution.md) chapter. More information on “drop scopes” used for the purpose of running destructors may be found in the [destructors](../destructors.md) chapter.

[names.scopes.items](#r-names.scopes.items)

## Item scopes

[names.scopes.items.module](#r-names.scopes.items.module)

The name of an [item](../items.md) declared directly in a [module](../items/modules.md) has a scope that extends from the start of the module to the end of the module. These items are also members of the module and can be referred to with a [path](../paths.md) leading from their module.

[names.scopes.items.statement](#r-names.scopes.items.statement)

The name of an item declared as a [statement](../statements.md) has a scope that extends from the start of the block the item statement is in until the end of the block.

[names.scopes.items.duplicate](#r-names.scopes.items.duplicate)

It is an error to introduce an item with a duplicate name of another item in the same [namespace](namespaces.md) within the same module or block. [Asterisk glob imports](../items/use-declarations.md) have special behavior for dealing with duplicate names and shadowing, see the linked chapter for more details.

[names.scopes.items.shadow-prelude](#r-names.scopes.items.shadow-prelude)

Items in a module may shadow items in a [prelude](#prelude-scopes).

[names.scopes.items.nested-modules](#r-names.scopes.items.nested-modules)

Item names from outer modules are not in scope within a nested module. A [path](../paths.md) may be used to refer to an item in another module.

[names.scopes.associated-items](#r-names.scopes.associated-items)

### Associated item scopes

[names.scopes.associated-items.scope](#r-names.scopes.associated-items.scope)

[Associated items](../items/associated-items.md) are not scoped and can only be referred to by using a [path](../paths.md) leading from the type or trait they are associated with. [Methods](../items/associated-items.md#methods) can also be referred to via [call expressions](../expressions/call-expr.md).

[names.scopes.associated-items.duplicate](#r-names.scopes.associated-items.duplicate)

Similar to items within a module or block, it is an error to introduce an item within a trait or implementation that is a duplicate of another item in the trait or impl in the same namespace.

[names.scopes.pattern-bindings](#r-names.scopes.pattern-bindings)

## Pattern binding scopes

[names.scopes.pattern-bindings.intro](#r-names.scopes.pattern-bindings.intro)

The scope of a local variable [pattern](../patterns.md) binding depends on where it is used:

[names.scopes.pattern-bindings.let](#r-names.scopes.pattern-bindings.let)

- [`let` statement](../statements.md#let-statements) bindings range from just after the `let` statement until the end of the block where it is declared.

[names.scopes.pattern-bindings.parameter](#r-names.scopes.pattern-bindings.parameter)

- [Function parameter](../items/functions.md#function-parameters) bindings are within the body of the function.

[names.scopes.pattern-bindings.closure](#r-names.scopes.pattern-bindings.closure)

- [Closure parameter](../expressions/closure-expr.md) bindings are within the closure body.

[names.scopes.pattern-bindings.loop](#r-names.scopes.pattern-bindings.loop)

- [`for`](../expressions/loop-expr.md#iterator-loops) bindings are within the loop body.

[names.scopes.pattern-bindings.let-chains](#r-names.scopes.pattern-bindings.let-chains)

- [`if let`](../expressions/if-expr.md#if-let-patterns) and [`while let`](../expressions/loop-expr.md#while-let-patterns) bindings are valid in the following conditions as well as the consequent block.

[names.scopes.pattern-bindings.match-arm](#r-names.scopes.pattern-bindings.match-arm)

- [`match` arms](../expressions/match-expr.md) bindings are within the [match guard](../expressions/match-expr.md#match-guards) and the match arm expression.

[names.scopes.pattern-bindings.match-guard-let](#r-names.scopes.pattern-bindings.match-guard-let)

- [`match` guard `let`](../expressions/match-expr.md#r-expr.match.guard.let) bindings are valid in the following guard conditions and the match arm expression.

[names.scopes.pattern-bindings.items](#r-names.scopes.pattern-bindings.items)

Local variable scopes do not extend into item declarations.

### Pattern binding shadowing

[names.scopes.pattern-bindings.shadow](#r-names.scopes.pattern-bindings.shadow)

Pattern bindings are allowed to shadow any name in scope with the following exceptions which are an error:

- [Const generic parameters](../items/generics.md#const-generics)
- [Static items](../items/static-items.md)
- [Const items](../items/constant-items.md)
- Constructors for [structs](../items/structs.md) and [enums](../items/enumerations.md)

The following example illustrates how local bindings can shadow item declarations:

``` playground
#![allow(unused)]
fn main() {
fn shadow_example() {
    // Since there are no local variables in scope yet, this resolves to the function.
    foo(); // prints `function`
    let foo = || println!("closure");
    fn foo() { println!("function"); }
    // This resolves to the local closure since it shadows the item.
    foo(); // prints `closure`
}
}
```

[names.scopes.generic-parameters](#r-names.scopes.generic-parameters)

## Generic parameter scopes

[names.scopes.generic-parameters.param-list](#r-names.scopes.generic-parameters.param-list)

Generic parameters are declared in a [GenericParams](../items/generics.md#grammar-GenericParams) list. The scope of a generic parameter is within the item it is declared on.

[names.scopes.generic-parameters.order-independent](#r-names.scopes.generic-parameters.order-independent)

All parameters are in scope within the generic parameter list regardless of the order they are declared. The following shows some examples where a parameter may be referenced before it is declared:

``` playground
#![allow(unused)]
fn main() {
// The 'b bound is referenced before it is declared.
fn params_scope<'a: 'b, 'b>() {}

trait SomeTrait<const Z: usize> {}
// The const N is referenced in the trait bound before it is declared.
fn f<T: SomeTrait<N>, const N: usize>() {}
}
```

[names.scopes.generic-parameters.bounds](#r-names.scopes.generic-parameters.bounds)

Generic parameters are also in scope for type bounds and where clauses, for example:

``` playground
#![allow(unused)]
fn main() {
trait SomeTrait<'a, T> {}
// The <'a, U> for `SomeTrait` refer to the 'a and U parameters of `bounds_scope`.
fn bounds_scope<'a, T: SomeTrait<'a, U>, U>() {}

fn where_scope<'a, T, U>()
    where T: SomeTrait<'a, U>
{}
}
```

[names.scopes.generic-parameters.inner-items](#r-names.scopes.generic-parameters.inner-items)

It is an error for [items](../items.md) declared inside a function to refer to a generic parameter from their outer scope.

``` playground
#![allow(unused)]
fn main() {
fn example<T>() {
    fn inner(x: T) {} // ERROR: can't use generic parameters from outer function
}
}
```

### Generic parameter shadowing

[names.scopes.generic-parameters.shadow](#r-names.scopes.generic-parameters.shadow)

It is an error to shadow a generic parameter with the exception that items declared within functions are allowed to shadow generic parameter names from the function.

``` playground
#![allow(unused)]
fn main() {
fn example<'a, T, const N: usize>() {
    // Items within functions are allowed to shadow generic parameter in scope.
    fn inner_lifetime<'a>() {} // OK
    fn inner_type<T>() {} // OK
    fn inner_const<const N: usize>() {} // OK
}
}
```

``` playground
#![allow(unused)]
fn main() {
trait SomeTrait<'a, T, const N: usize> {
    fn example_lifetime<'a>() {} // ERROR: 'a is already in use
    fn example_type<T>() {} // ERROR: T is already in use
    fn example_const<const N: usize>() {} // ERROR: N is already in use
    fn example_mixed<const T: usize>() {} // ERROR: T is already in use
}
}
```

[names.scopes.lifetimes](#r-names.scopes.lifetimes)

### Lifetime scopes

[names.scopes.lifetimes.intro](#r-names.scopes.lifetimes.intro)

Lifetime parameters are declared in a [GenericParams](../items/generics.md#grammar-GenericParams) list and [higher-ranked trait bounds](../trait-bounds.md#higher-ranked-trait-bounds).

[names.scopes.lifetimes.special](#r-names.scopes.lifetimes.special)

The `'static` lifetime and [placeholder lifetime](../lifetime-elision.md) `'_` have a special meaning and cannot be declared as a parameter.

#### Lifetime generic parameter scopes

[names.scopes.lifetimes.generic](#r-names.scopes.lifetimes.generic)

[Constant](../items/constant-items.md) and [static](../items/static-items.md) items and [const contexts](../const_eval.md#const-context) only ever allow `'static` lifetime references, so no other lifetime may be in scope within them. [Associated consts](../items/associated-items.md#associated-constants) do allow referring to lifetimes declared in their trait or implementation.

#### Higher-ranked trait bound scopes

[names.scopes.lifetimes.higher-ranked](#r-names.scopes.lifetimes.higher-ranked)

The scope of a lifetime parameter declared as a [higher-ranked trait bound](../trait-bounds.md#higher-ranked-trait-bounds) depends on the scenario where it is used.

- As a [TypeBoundWhereClauseItem](../items/generics.md#grammar-TypeBoundWhereClauseItem) the declared lifetimes are in scope in the type and the type bounds.
- As a [TraitBound](../trait-bounds.md#grammar-TraitBound) the declared lifetimes are in scope within the bound type path.
- As a [BareFunctionType](../types/function-pointer.md#grammar-BareFunctionType) the declared lifetimes are in scope within the function parameters and return type.

``` playground
#![allow(unused)]
fn main() {
trait Trait<'a>{}

fn where_clause<T>()
    // 'a is in scope in both the type and the type bounds.
    where for <'a> &'a T: Trait<'a>
{}

fn bound<T>()
    // 'a is in scope within the bound.
    where T: for <'a> Trait<'a>
{}

struct Example<'a> {
    field: &'a u32
}

// 'a is in scope in both the parameters and return type.
type FnExample = for<'a> fn(x: Example<'a>) -> Example<'a>;
}
```

#### Impl trait restrictions

[names.scopes.lifetimes.impl-trait](#r-names.scopes.lifetimes.impl-trait)

[Impl trait](../types/impl-trait.md) types can only reference lifetimes declared on a function or implementation.

``` playground
#![allow(unused)]
fn main() {
trait Trait1 {
    type Item;
}
trait Trait2<'a> {}

struct Example;

impl Trait1 for Example {
    type Item = Element;
}

struct Element;
impl<'a> Trait2<'a> for Element {}

// The `impl Trait2` here is not allowed to refer to 'b but it is allowed to
// refer to 'a.
fn foo<'a>() -> impl for<'b> Trait1<Item = impl Trait2<'a> + use<'a>> {
    // ...
   Example
}
}
```

[names.scopes.loop-label](#r-names.scopes.loop-label)

## Loop label scopes

[names.scopes.loop-label.scope](#r-names.scopes.loop-label.scope)

[Loop labels](../expressions/loop-expr.md#loop-labels) may be declared by a [loop expression](../expressions/loop-expr.md). The scope of a loop label is from the point it is declared till the end of the loop expression. The scope does not extend into [items](../items.md), [closures](../expressions/closure-expr.md), [async blocks](../expressions/block-expr.md#async-blocks), [const arguments](../items/generics.md#const-generics), [const contexts](../const_eval.md#const-context), and the iterator expression of the defining [`for` loop](../expressions/loop-expr.md#iterator-loops).

``` playground
#![allow(unused)]
fn main() {
'a: for n in 0..3 {
    if n % 2 == 0 {
        break 'a;
    }
    fn inner() {
        // Using 'a here would be an error.
        // break 'a;
    }
}

// The label is in scope for the expression of `while` loops.
'a: while break 'a {}         // Loop does not run.
'a: while let _ = break 'a {} // Loop does not run.

// The label is not in scope in the defining `for` loop:
'a: for outer in 0..5 {
    // This will break the outer loop, skipping the inner loop and stopping
    // the outer loop.
    'a: for inner in { break 'a; 0..1 } {
        println!("{}", inner); // This does not run.
    }
    println!("{}", outer); // This does not run, either.
}

}
```

[names.scopes.loop-label.shadow](#r-names.scopes.loop-label.shadow)

Loop labels may shadow labels of the same name in outer scopes. References to a label refer to the closest definition.

``` playground
#![allow(unused)]
fn main() {
// Loop label shadowing example.
'a: for outer in 0..5 {
    'a: for inner in 0..5 {
        // This terminates the inner loop, but the outer loop continues to run.
        break 'a;
    }
}
}
```

[names.scopes.prelude](#r-names.scopes.prelude)

## Prelude scopes

[names.scopes.prelude.intro](#r-names.scopes.prelude.intro)

[Preludes](preludes.md) bring entities into scope of every module. The entities are not members of the module, but are implicitly queried during [name resolution](name-resolution.md).

[names.scopes.prelude.shadow](#r-names.scopes.prelude.shadow)

The prelude names may be shadowed by declarations in a module.

[names.scopes.prelude.layers](#r-names.scopes.prelude.layers)

The preludes are layered such that one shadows another if they contain entities of the same name. The order that preludes may shadow other preludes is the following where earlier entries may shadow later ones:

1.  [Extern prelude](preludes.md#extern-prelude)
2.  [Tool prelude](preludes.md#tool-prelude)
3.  [`macro_use` prelude](preludes.md#macro_use-prelude)
4.  [Standard library prelude](preludes.md#standard-library-prelude)
5.  [Language prelude](preludes.md#language-prelude)

[names.scopes.macro_rules](#r-names.scopes.macro_rules)

## `macro_rules` scopes

The scope of `macro_rules` macros is described in the [Macros By Example](../macros-by-example.md) chapter. The behavior depends on the use of the [`macro_use`](../macros-by-example.md#the-macro_use-attribute) and [`macro_export`](../macros-by-example.md#the-macro_export-attribute) attributes.

[names.scopes.derive](#r-names.scopes.derive)

## Derive macro helper attributes

[names.scopes.derive.scope](#r-names.scopes.derive.scope)

[Derive macro helper attributes](../procedural-macros.md#derive-macro-helper-attributes) are in scope in the item where their corresponding [`derive` attribute](../attributes/derive.md) is specified. The scope extends from just after the `derive` attribute to the end of the item.

[names.scopes.derive.shadow](#r-names.scopes.derive.shadow)

Helper attributes shadow other attributes of the same name in scope.

[names.scopes.self](#r-names.scopes.self)

## `Self` scope

[names.scopes.self.intro](#r-names.scopes.self.intro)

Although [`Self`](../paths.md#self-1) is a keyword with special meaning, it interacts with name resolution in a way similar to normal names.

[names.scopes.self.def-scope](#r-names.scopes.self.def-scope)

The implicit `Self` type in the definition of a [struct](../items/structs.md), [enum](../items/enumerations.md), [union](../items/unions.md), [trait](../items/traits.md), or [implementation](../items/implementations.md) is treated similarly to a [generic parameter](#generic-parameter-scopes), and is in scope in the same way as a generic type parameter.

[names.scopes.self.impl-scope](#r-names.scopes.self.impl-scope)

The implicit `Self` constructor in the value [namespace](namespaces.md) of an [implementation](../items/implementations.md) is in scope within the body of the implementation (the implementation’s [associated items](../items/associated-items.md)).

``` playground
#![allow(unused)]
fn main() {
// Self type within struct definition.
struct Recursive {
    f1: Option<Box<Self>>
}

// Self type within generic parameters.
struct SelfGeneric<T: Into<Self>>(T);

// Self value constructor within an implementation.
struct ImplExample();
impl ImplExample {
    fn example() -> Self { // Self type
        Self() // Self value constructor
    }
}
}
```
