# Function pointer types

[type.fn-pointer](#r-type.fn-pointer)

# Function pointer types

[type.fn-pointer.syntax](#r-type.fn-pointer.syntax)

**Syntax**\
[BareFunctionType](#railroad-BareFunctionType) →\
    [ForLifetimes](../trait-bounds.md#grammar-ForLifetimes)? [FunctionTypeQualifiers](#grammar-FunctionTypeQualifiers) fn\
       ( [FunctionParametersMaybeNamedVariadic](#grammar-FunctionParametersMaybeNamedVariadic)? ) [BareFunctionReturnType](#grammar-BareFunctionReturnType)?

[FunctionTypeQualifiers](#railroad-FunctionTypeQualifiers) → unsafe? ( extern [Abi](../items/functions.md#grammar-Abi)? )?

[BareFunctionReturnType](#railroad-BareFunctionReturnType) → -\> [TypeNoBounds](../types.md#grammar-TypeNoBounds)

[FunctionParametersMaybeNamedVariadic](#railroad-FunctionParametersMaybeNamedVariadic) →\
    [MaybeNamedFunctionParameters](#grammar-MaybeNamedFunctionParameters) \| [MaybeNamedFunctionParametersVariadic](#grammar-MaybeNamedFunctionParametersVariadic)

[MaybeNamedFunctionParameters](#railroad-MaybeNamedFunctionParameters) →\
    [MaybeNamedParam](#grammar-MaybeNamedParam) ( , [MaybeNamedParam](#grammar-MaybeNamedParam) )\* ,?

[MaybeNamedParam](#railroad-MaybeNamedParam) →\
    [OuterAttribute](../attributes.md#grammar-OuterAttribute)\* ( ( [IDENTIFIER](../identifiers.md#grammar-IDENTIFIER) \| \_ ) : )? [Type](../types.md#grammar-Type)

[MaybeNamedFunctionParametersVariadic](#railroad-MaybeNamedFunctionParametersVariadic) →\
    ( [MaybeNamedParam](#grammar-MaybeNamedParam) , )\* [MaybeNamedParam](#grammar-MaybeNamedParam) , [OuterAttribute](../attributes.md#grammar-OuterAttribute)\* ...

Show Railroad

![](../../assets/inline/function-pointer-1.svg) ![](../../assets/inline/function-pointer-2.svg) ![](../../assets/inline/function-pointer-3.svg) ![](../../assets/inline/function-pointer-4.svg) ![](../../assets/inline/function-pointer-5.svg) ![](../../assets/inline/function-pointer-6.svg) ![](../../assets/inline/function-pointer-7.svg) [type.fn-pointer.intro](#r-type.fn-pointer.intro)

A function pointer type, written using the `fn` keyword, refers to a function whose identity is not necessarily known at compile-time.

An example where `Binop` is defined as a function pointer type:

``` playground
#![allow(unused)]
fn main() {
fn add(x: i32, y: i32) -> i32 {
    x + y
}

let mut x = add(5,7);

type Binop = fn(i32, i32) -> i32;
let bo: Binop = add;
x = bo(5,7);
}
```

[type.fn-pointer.coercion](#r-type.fn-pointer.coercion)

Function pointers can be created via a coercion from both [function items](function-item.md) and non-capturing, non-async [closures](closure.md).

[type.fn-pointer.qualifiers](#r-type.fn-pointer.qualifiers)

The `unsafe` qualifier indicates that the type’s value is an [unsafe function](../unsafe-keyword.md), and the `extern` qualifier indicates it is an [extern function](../items/functions.md#extern-function-qualifier).

[type.fn-pointer.constraint-variadic](#r-type.fn-pointer.constraint-variadic)

For the function to be variadic, its `extern` ABI must be one of those listed in [items.extern.variadic.conventions](../items/external-blocks.md#r-items.extern.variadic.conventions).

[type.fn-pointer.extern-custom](#r-type.fn-pointer.extern-custom)

An `extern "custom"` function pointer must follow the rules in [items.fn.extern.custom.signature](../items/functions.md#r-items.fn.extern.custom.signature).

[type.fn-pointer.attributes](#r-type.fn-pointer.attributes)

## Attributes on function pointer parameters

Attributes on function pointer parameters follow the same rules and restrictions as [regular function parameters](../items/functions.md#attributes-on-function-parameters).
