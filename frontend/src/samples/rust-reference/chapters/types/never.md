# Never type

[type.never](#r-type.never)

# Never type

[type.never.intro](#r-type.never.intro)

The never type `!` is a type with no values, representing computations that never complete, also known as [diverging](../divergence.md#r-divergence) computations.

> ![](../../assets/inline/never-1.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> fn foo() -> ! {
>     loop {}
> }
> }
> ```
>
> ``` playground
> #![allow(unused)]
> fn main() {
> unsafe extern "C" {
>     pub safe fn no_return_extern_func() -> !;
> }
> }
> ```
>
> ``` playground
> #![allow(unused)]
> fn main() {
> let _: ! = loop {};
> }
> ```
>
> ``` playground
> #![allow(unused)]
> fn main() {
> fn always_ok() -> Result {
>     Ok(42)
> }
> }
> ```
>
> ``` playground
> #![allow(unused)]
> fn main() {
> use std::str::FromStr;
> struct Anything(String);
>
> impl FromStr for Anything {
>     type Err = !;
>
>     fn from_str(s: &str) -> Result {
>         Ok(Anything(s.to_owned()))
>     }
> }
>
> // This does not need to check for the `Err` variant because
> // `FromStr::Err` is the never type.
> let Ok(s) = Anything::from_str("example");
> }
> ```

[type.never.syntax](#r-type.never.syntax)

**Syntax**\
[NeverType](#railroad-NeverType) → !

Show Railroad

![](../../assets/inline/never-2.svg) [type.never.coercion](#r-type.never.coercion)

Expressions of type `!` can be coerced into any type.

> ![](../../assets/inline/never-3.svg)Note
>
> The standard library type [`Infallible`](../../core/convert/type.Infallible.html) is a type alias for `!`.
