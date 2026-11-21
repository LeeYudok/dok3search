# Variables

A variable is a storage location for holding a *value*. The set of permissible values is determined by the variable's *[type](types.md)*.

A [variable declaration](declarations-and-scope.md#variable-declarations) or, for function parameters and results, the signature of a [function declaration](declarations-and-scope.md#function-declarations) or [function literal](expressions.md#function-literals) reserves storage for a named variable. Calling the built-in function [`new`](built-in-functions.md#allocation) or taking the address of a [composite literal](expressions.md#composite-literals) allocates storage for a variable at run time. Such an anonymous variable is referred to via a (possibly implicit) [pointer indirection](expressions.md#address-operators).

*Structured* variables of [array](types.md#array-types), [slice](types.md#slice-types), and [struct](types.md#struct-types) types have elements and fields that may be [addressed](expressions.md#address-operators) individually. Each such element acts like a variable.

The *static type* (or just *type*) of a variable is the type given in its declaration, the type provided in the `new` call or composite literal, or the type of an element of a structured variable. Variables of interface type also have a distinct *dynamic type*, which is the (non-interface) type of the value assigned to the variable at run time (unless the value is the predeclared identifier `nil`, which has no type). The dynamic type may vary during execution but values stored in interface variables are always [assignable](properties-of-types-and-values.md#assignability) to the static type of the variable.

``` go
var x interface{}  // x is nil and has static type interface{}
var v *T           // v has value nil, static type *T
x = 42             // x has value 42 and dynamic type int
x = v              // x has value (*T)(nil) and dynamic type *T
```

A variable's value is retrieved by referring to the variable in an [expression](expressions.md); it is the most recent value [assigned](statements.md#assignment-statements) to the variable. If a variable has not yet been assigned a value, its value is the [zero value](program-initialization-and-execution.md#the-zero-value) for its type.
