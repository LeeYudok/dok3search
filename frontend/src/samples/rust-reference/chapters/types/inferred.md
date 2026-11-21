# Inferred type

[type.inferred](#r-type.inferred)

# Inferred type

[type.inferred.syntax](#r-type.inferred.syntax)

**Syntax**\
[InferredType](#railroad-InferredType) → \_

Show Railroad

![](../../assets/inline/inferred-1.svg) [type.inferred.intro](#r-type.inferred.intro)

The inferred type asks the compiler to infer the type if possible based on the surrounding information available.

> ![](../../assets/inline/inferred-2.svg)Example
>
> The inferred type is often used in generic arguments:
>
> ``` playground
> #![allow(unused)]
> fn main() {
> let x: Vec<_> = (0..10).collect();
> }
> ```

[type.inferred.constraint](#r-type.inferred.constraint)

The inferred type cannot be used in item signatures.
