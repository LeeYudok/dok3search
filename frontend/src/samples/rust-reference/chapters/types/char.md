# Character type

[type.char](#r-type.char)

# Character type

[type.char.intro](#r-type.char.intro)

The `char` type represents a single [Unicode scalar value](http://www.unicode.org/glossary/#unicode_scalar_value) (i.e., a code point that is not a surrogate).

> ![](../../assets/inline/char-1.svg)Example
>
> ``` playground
> #![allow(unused)]
> fn main() {
> let c: char = 'a';
> let emoji: char = '😀';
> let unicode: char = '\u{1F600}';
> }
> ```

> ![](../../assets/inline/char-2.svg)Note
>
> See [the standard library docs](../../std/primitive.char.html) for information on the impls of the `char` type.

[type.char.value](#r-type.char.value)

A value of type `char` is represented as a 32-bit unsigned word in the 0x0000 to 0xD7FF or 0xE000 to 0x10FFFF range. It is immediate [undefined behavior](../behavior-considered-undefined.md) to create a `char` that falls outside this range.

[type.char.layout](#r-type.char.layout)

`char` is guaranteed to have the same size and alignment as `u32` on all platforms.

[type.char.validity](#r-type.char.validity)

Every byte of a `char` is guaranteed to be initialized. In other words, `transmute::<char, [u8; size_of::<char>()]>(...)` is always sound – but since some bit patterns are invalid `char`s, the inverse is not always sound.
