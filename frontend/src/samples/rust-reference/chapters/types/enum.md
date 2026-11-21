# Enumerated types

[type.enum](#r-type.enum)

# Enumerated types

[type.enum.intro](#r-type.enum.intro)

An *enumerated type* is a nominal, heterogeneous disjoint union type, denoted by the name of an [`enum` item](../items/enumerations.md). [1](#footnote-enumtype)

[type.enum.declaration](#r-type.enum.declaration)

An [`enum` item](../items/enumerations.md) declares both the type and a number of *variants*, each of which is independently named and has the syntax of a struct, tuple struct or unit-like struct.

[type.enum.constructor](#r-type.enum.constructor)

New instances of an `enum` can be constructed with a [struct expression](../expressions/struct-expr.md).

[type.enum.value](#r-type.enum.value)

Any `enum` value consumes as much memory as the largest variant for its corresponding `enum` type, as well as the size needed to store a discriminant.

[type.enum.name](#r-type.enum.name)

Enum types cannot be denoted *structurally* as types, but must be denoted by named reference to an [`enum` item](../items/enumerations.md).

------------------------------------------------------------------------

1.  

    The `enum` type is analogous to a `data` constructor declaration in Haskell, or a *pick ADT* in Limbo. [↩](#fr-enumtype-1)
