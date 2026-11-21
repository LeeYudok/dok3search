# Appendix

### Language versions

The [Go 1 compatibility guarantee](https://go.dev/doc/go1compat) ensures that programs written to the Go 1 specification will continue to compile and run correctly, unchanged, over the lifetime of that specification. More generally, as adjustments are made and features added to the language, the compatibility guarantee ensures that a Go program that works with a specific Go language version will continue to work with any subsequent version.

For instance, the ability to use the prefix `0b` for binary integer literals was introduced with Go 1.13, indicated by \[[Go 1.13](#go-113)\] in the section on [integer literals](lexical-elements.md#integer-literals). Source code containing an integer literal such as `0b1011` will be rejected if the implied or required language version used by the compiler is older than Go 1.13.

The following table describes the minimum language version required for features introduced after Go 1.

#### Go 1.9

- An [alias declaration](declarations-and-scope.md#alias-declarations) may be used to declare an alias name for a type.

#### Go 1.13

- [Integer literals](lexical-elements.md#integer-literals) may use the prefixes `0b`, `0B`, `0o`, and `0O` for binary, and octal literals, respectively.
- Hexadecimal [floating-point literals](lexical-elements.md#floating-point-literals) may be written using the prefixes `0x` and `0X`.
- The [imaginary suffix](lexical-elements.md#imaginary-literals) `i` may be used with any (binary, decimal, hexadecimal) integer or floating-point literal, not just decimal literals.
- The digits of any number literal may be [separated](lexical-elements.md#integer-literals) (grouped) using underscores `_`.
- The shift count in a [shift operation](expressions.md#operators) may be a signed integer type.

#### Go 1.14

- Emdedding a method more than once through different [embedded interfaces](types.md#embedded-interfaces) is not an error.

#### Go 1.17

- A slice may be [converted](expressions.md#conversions) to an array pointer if the slice and array element types match, and the array is not longer than the slice.
- The built-in [package `unsafe`](system-considerations.md#package-unsafe) includes the new functions `Add` and `Slice`.

#### Go 1.18

The 1.18 release adds polymorphic functions and types ("generics") to the language. Specifically:

- The set of [operators and punctuation](lexical-elements.md#operators-and-punctuation) includes the new token `~`.
- Function and type declarations may declare [type parameters](declarations-and-scope.md#type-parameter-declarations).
- Interface types may [embed arbitrary types](types.md#general-interfaces) (not just type names of interfaces) as well as union and `~T` type elements.
- The set of [predeclared](declarations-and-scope.md#predeclared-identifiers) types includes the new types `any` and `comparable`.

#### Go 1.20

- A slice may be [converted](expressions.md#conversions) to an array if the slice and array element types match and the array is not longer than the slice.
- The built-in [package `unsafe`](system-considerations.md#package-unsafe) includes the new functions `SliceData`, `String`, and `StringData`.
- [Comparable types](expressions.md#comparison-operators) (such as ordinary interfaces) may satisfy `comparable` constraints, even if the type arguments are not strictly comparable.

#### Go 1.21

- The set of [predeclared](declarations-and-scope.md#predeclared-identifiers) functions includes the new functions `min`, `max`, and `clear`.
- [Type inference](expressions.md#type-inference) uses the types of interface methods for inference. It also infers type arguments for generic functions assigned to variables or passed as arguments to other (possibly generic) functions.

#### Go 1.22

- In a ["for" statement](statements.md#for-statements), each iteration has its own set of iteration variables rather than sharing the same variables in each iteration.
- A "for" statement with ["range" clause](statements.md#for-statements-with-range-clause) may iterate over integer values from zero to an upper limit.

#### Go 1.23

- A "for" statement with ["range" clause](statements.md#for-statements-with-range-clause) accepts an iterator function as range expression.

#### Go 1.24

- An [alias declaration](declarations-and-scope.md#alias-declarations) may declare [type parameters](declarations-and-scope.md#type-parameter-declarations).

### Type unification rules

The type unification rules describe if and how two types unify. The precise details are relevant for Go implementations, affect the specifics of error messages (such as whether a compiler reports a type inference or other error), and may explain why type inference fails in unusual code situations. But by and large these rules can be ignored when writing Go code: type inference is designed to mostly "work as expected", and the unification rules are fine-tuned accordingly.

Type unification is controlled by a *matching mode*, which may be *exact* or *loose*. As unification recursively descends a composite type structure, the matching mode used for elements of the type, the *element matching mode*, remains the same as the matching mode except when two types are unified for [assignability](properties-of-types-and-values.md#assignability) (`≡`_(`A`)): in this case, the matching mode is *loose* at the top level but then changes to *exact* for element types, reflecting the fact that types don't have to be identical to be assignable.

Two types that are not bound type parameters unify exactly if any of following conditions is true:

- Both types are [identical](properties-of-types-and-values.md#type-identity).
- Both types have identical structure and their element types unify exactly.
- Exactly one type is an [unbound](expressions.md#type-inference) type parameter, and all the types in its type set unify with the other type per the unification rules for `≡`_(`A`) (loose unification at the top level and exact unification for element types).

If both types are bound type parameters, they unify per the given matching modes if:

- Both type parameters are identical.
- At most one of the type parameters has a known type argument. In this case, the type parameters are *joined*: they both stand for the same type argument. If neither type parameter has a known type argument yet, a future type argument inferred for one the type parameters is simultaneously inferred for both of them.
- Both type parameters have a known type argument and the type arguments unify per the given matching modes.

A single bound type parameter `P` and another type `T` unify per the given matching modes if:

- `P` doesn't have a known type argument. In this case, `T` is inferred as the type argument for `P`.
- `P` does have a known type argument `A`, `A` and `T` unify per the given matching modes, and one of the following conditions is true:
  - Both `A` and `T` are interface types: In this case, if both `A` and `T` are also [defined](declarations-and-scope.md#type-definitions) types, they must be [identical](properties-of-types-and-values.md#type-identity). Otherwise, if neither of them is a defined type, they must have the same number of methods (unification of `A` and `T` already established that the methods match).
  - Neither `A` nor `T` are interface types: In this case, if `T` is a defined type, `T` replaces `A` as the inferred type argument for `P`.

Finally, two types that are not bound type parameters unify loosely (and per the element matching mode) if:

- Both types unify exactly.
- One type is a [defined type](declarations-and-scope.md#type-definitions), the other type is a type literal, but not an interface, and their underlying types unify per the element matching mode.
- Both types are interfaces (but not type parameters) with identical [type terms](types.md#interface-types), both or neither embed the predeclared type [comparable](declarations-and-scope.md#predeclared-identifiers), corresponding method types unify exactly, and the method set of one of the interfaces is a subset of the method set of the other interface.
- Only one type is an interface (but not a type parameter), corresponding methods of the two types unify per the element matching mode, and the method set of the interface is a subset of the method set of the other type.
- Both types have the same structure and their element types unify per the element matching mode.
