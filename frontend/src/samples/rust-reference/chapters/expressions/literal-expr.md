# Literal expressions

[expr.literal](#r-expr.literal)

# Literal expressions

[expr.literal.syntax](#r-expr.literal.syntax)

**Syntax**\
[LiteralExpression](#railroad-LiteralExpression) →\
      [CHAR_LITERAL](../tokens.md#grammar-CHAR_LITERAL)\
    \| [STRING_LITERAL](../tokens.md#grammar-STRING_LITERAL)\
    \| [RAW_STRING_LITERAL](../tokens.md#grammar-RAW_STRING_LITERAL)\
    \| [BYTE_LITERAL](../tokens.md#grammar-BYTE_LITERAL)\
    \| [BYTE_STRING_LITERAL](../tokens.md#grammar-BYTE_STRING_LITERAL)\
    \| [RAW_BYTE_STRING_LITERAL](../tokens.md#grammar-RAW_BYTE_STRING_LITERAL)\
    \| [C_STRING_LITERAL](../tokens.md#grammar-C_STRING_LITERAL)\
    \| [RAW_C_STRING_LITERAL](../tokens.md#grammar-RAW_C_STRING_LITERAL)\
    \| [INTEGER_LITERAL](../tokens.md#grammar-INTEGER_LITERAL)\
    \| [FLOAT_LITERAL](../tokens.md#grammar-FLOAT_LITERAL)\
    \| true\
    \| false

Show Railroad

![](../../assets/inline/literal-expr-1.svg) [expr.literal.intro](#r-expr.literal.intro)

A *literal expression* is an expression consisting of a single token, rather than a sequence of tokens, that immediately and directly denotes the value it evaluates to, rather than referring to it by name or some other evaluation rule.

[expr.literal.const-expr](#r-expr.literal.const-expr)

A literal is a form of [constant expression](../const_eval.md#constant-expressions), so is evaluated (primarily) at compile time.

[expr.literal.literal-token](#r-expr.literal.literal-token)

Each of the lexical [literal](../tokens.md#literals) forms described earlier can make up a literal expression, as can the keywords `true` and `false`.

``` playground
#![allow(unused)]
fn main() {
"hello";   // string type
'5';       // character type
5;         // integer type
}
```

[expr.literal.string-representation](#r-expr.literal.string-representation)

In the descriptions below, the *string representation* of a token is the sequence of characters from the input which matched the token’s production in a *Lexer* grammar snippet.

> ![](../../assets/inline/literal-expr-2.svg)Note
>
> This string representation never includes a character `U+000D` (CR) immediately followed by `U+000A` (LF): this pair would have been previously transformed into a single `U+000A` (LF).

[expr.literal.escape](#r-expr.literal.escape)

## Escapes

[expr.literal.escape.intro](#r-expr.literal.escape.intro)

The descriptions of textual literal expressions below make use of several forms of *escape*.

[expr.literal.escape.sequence](#r-expr.literal.escape.sequence)

Each form of escape is characterised by:

- an *escape sequence*: a sequence of characters, which always begins with `U+005C` (`\`)
- an *escaped value*: either a single character or an empty sequence of characters

In the definitions of escapes below:

- An *octal digit* is any of the characters in the range \[`0`-`7`\].
- A *hexadecimal digit* is any of the characters in the ranges \[`0`-`9`\], \[`a`-`f`\], or \[`A`-`F`\].

[expr.literal.escape.simple](#r-expr.literal.escape.simple)

### Simple escapes

Each sequence of characters occurring in the first column of the following table is an escape sequence.

In each case, the escaped value is the character given in the corresponding entry in the second column.

| Escape sequence | Escaped value            |
|-----------------|--------------------------|
| `\0`            | U+0000 (NUL)             |
| `\t`            | U+0009 (HT)              |
| `\n`            | U+000A (LF)              |
| `\r`            | U+000D (CR)              |
| `\"`            | U+0022 (QUOTATION MARK)  |
| `\'`            | U+0027 (APOSTROPHE)      |
| `\\`            | U+005C (REVERSE SOLIDUS) |

[expr.literal.escape.hex-octet](#r-expr.literal.escape.hex-octet)

### 8-bit escapes

The escape sequence consists of `\x` followed by two hexadecimal digits.

The escaped value is the character whose [Unicode scalar value](http://www.unicode.org/glossary/#unicode_scalar_value) is the result of interpreting the final two characters in the escape sequence as a hexadecimal integer, as if by [`u8::from_str_radix`](https://doc.rust-lang.org/1.100.0/std/primitive.u8.html#method.from_str_radix) with radix 16.

> ![](../../assets/inline/literal-expr-3.svg)Note
>
> The escaped value therefore has a [Unicode scalar value](http://www.unicode.org/glossary/#unicode_scalar_value) in the range of [`u8`](../types/numeric.md).

[expr.literal.escape.hex-ascii](#r-expr.literal.escape.hex-ascii)

### 7-bit escapes

The escape sequence consists of `\x` followed by an octal digit then a hexadecimal digit.

The escaped value is the character whose [Unicode scalar value](http://www.unicode.org/glossary/#unicode_scalar_value) is the result of interpreting the final two characters in the escape sequence as a hexadecimal integer, as if by [`u8::from_str_radix`](https://doc.rust-lang.org/1.100.0/std/primitive.u8.html#method.from_str_radix) with radix 16.

[expr.literal.escape.unicode](#r-expr.literal.escape.unicode)

### Unicode escapes

The escape sequence consists of `\u{`, followed by a sequence of characters each of which is a hexadecimal digit or `_`, followed by `}`.

The escaped value is the character whose [Unicode scalar value](http://www.unicode.org/glossary/#unicode_scalar_value) is the result of interpreting the hexadecimal digits contained in the escape sequence as a hexadecimal integer, as if by [`u32::from_str_radix`](https://doc.rust-lang.org/1.100.0/std/primitive.u32.html#method.from_str_radix) with radix 16.

> ![](../../assets/inline/literal-expr-4.svg)Note
>
> The permitted forms of a [CHAR_LITERAL](../tokens.md#grammar-CHAR_LITERAL) or [STRING_LITERAL](../tokens.md#grammar-STRING_LITERAL) token ensure that there is such a character.

[expr.literal.continuation](#r-expr.literal.continuation)

### String continuation escapes

The escape sequence consists of `\` followed immediately by `U+000A` (LF), and all following whitespace characters before the next non-whitespace character. For this purpose, the whitespace characters are `U+0009` (HT), `U+000A` (LF), `U+000D` (CR), and `U+0020` (SPACE).

The escaped value is an empty sequence of characters.

> ![](../../assets/inline/literal-expr-5.svg)Note
>
> The effect of this form of escape is that a string continuation skips following whitespace, including additional newlines. Thus `a`, `b` and `c` are equal:
>
> ``` playground
> #![allow(unused)]
> fn main() {
> let a = "foobar";
> let b = "foo\
>          bar";
> let c = "foo\
>
>      bar";
>
> assert_eq!(a, b);
> assert_eq!(b, c);
> }
> ```
>
> Skipping additional newlines (as in example c) is potentially confusing and unexpected. This behavior may be adjusted in the future. Until a decision is made, it is recommended to avoid relying on skipping multiple newlines with line continuations. See [this issue](https://github.com/rust-lang/reference/pull/1042) for more information.

[expr.literal.char](#r-expr.literal.char)

## Character literal expressions

[expr.literal.char.intro](#r-expr.literal.char.intro)

A character literal expression consists of a single [CHAR_LITERAL](../tokens.md#grammar-CHAR_LITERAL) token.

[expr.literal.char.type](#r-expr.literal.char.type)

The expression’s type is the primitive [`char`](../types/char.md) type.

[expr.literal.char.no-suffix](#r-expr.literal.char.no-suffix)

The token must not have a suffix.

[expr.literal.char.literal-content](#r-expr.literal.char.literal-content)

The token’s *literal content* is the sequence of characters following the first `U+0027` (`'`) and preceding the last `U+0027` (`'`) in the string representation of the token.

[expr.literal.char.represented](#r-expr.literal.char.represented)

The literal expression’s *represented character* is derived from the literal content as follows:

[expr.literal.char.escape](#r-expr.literal.char.escape)

- If the literal content is one of the following forms of escape sequence, the represented character is the escape sequence’s escaped value:
  - [Simple escapes](#simple-escapes)
  - [7-bit escapes](#7-bit-escapes)
  - [Unicode escapes](#unicode-escapes)

[expr.literal.char.single](#r-expr.literal.char.single)

- Otherwise the represented character is the single character that makes up the literal content.

[expr.literal.char.result](#r-expr.literal.char.result)

The expression’s value is the [`char`](../types/char.md) corresponding to the represented character’s [Unicode scalar value](http://www.unicode.org/glossary/#unicode_scalar_value).

> ![](../../assets/inline/literal-expr-6.svg)Note
>
> The permitted forms of a [CHAR_LITERAL](../tokens.md#grammar-CHAR_LITERAL) token ensure that these rules always produce a single character.

Examples of character literal expressions:

``` playground
#![allow(unused)]
fn main() {
'R';                               // R
'\'';                              // '
'\x52';                            // R
'\u{00E6}';                        // LATIN SMALL LETTER AE (U+00E6)
}
```

[expr.literal.string](#r-expr.literal.string)

## String literal expressions

[expr.literal.string.intro](#r-expr.literal.string.intro)

A string literal expression consists of a single [STRING_LITERAL](../tokens.md#grammar-STRING_LITERAL) or [RAW_STRING_LITERAL](../tokens.md#grammar-RAW_STRING_LITERAL) token.

[expr.literal.string.type](#r-expr.literal.string.type)

The expression’s type is a shared reference (with `static` lifetime) to the primitive [`str`](../types/str.md) type. That is, the type is `&'static str`.

[expr.literal.string.no-suffix](#r-expr.literal.string.no-suffix)

The token must not have a suffix.

[expr.literal.string.literal-content](#r-expr.literal.string.literal-content)

The token’s *literal content* is the sequence of characters following the first `U+0022` (`"`) and preceding the last `U+0022` (`"`) in the string representation of the token.

[expr.literal.string.represented](#r-expr.literal.string.represented)

The literal expression’s *represented string* is a sequence of characters derived from the literal content as follows:

[expr.literal.string.escape](#r-expr.literal.string.escape)

- If the token is a [STRING_LITERAL](../tokens.md#grammar-STRING_LITERAL), each escape sequence of any of the following forms occurring in the literal content is replaced by the escape sequence’s escaped value.

  - [Simple escapes](#simple-escapes)
  - [7-bit escapes](#7-bit-escapes)
  - [Unicode escapes](#unicode-escapes)
  - [String continuation escapes](#string-continuation-escapes)

  These replacements take place in left-to-right order. For example, the token `"\\x41"` is converted to the characters `\` `x` `4` `1`.

[expr.literal.string.raw](#r-expr.literal.string.raw)

- If the token is a [RAW_STRING_LITERAL](../tokens.md#grammar-RAW_STRING_LITERAL), the represented string is identical to the literal content.

[expr.literal.string.result](#r-expr.literal.string.result)

The expression’s value is a reference to a statically allocated [`str`](../types/str.md) containing the UTF-8 encoding of the represented string.

Examples of string literal expressions:

``` playground
#![allow(unused)]
fn main() {
"foo"; r"foo";                     // foo
"\"foo\""; r#""foo""#;             // "foo"

"foo #\"# bar";
r##"foo #"# bar"##;                // foo #"# bar

"\x52"; "R"; r"R";                 // R
"\\x52"; r"\x52";                  // \x52
}
```

[expr.literal.byte-char](#r-expr.literal.byte-char)

## Byte literal expressions

[expr.literal.byte-char.intro](#r-expr.literal.byte-char.intro)

A byte literal expression consists of a single [BYTE_LITERAL](../tokens.md#grammar-BYTE_LITERAL) token.

[expr.literal.byte-char.literal](#r-expr.literal.byte-char.literal)

The expression’s type is the primitive [`u8`](../types/numeric.md) type.

[expr.literal.byte-char.no-suffix](#r-expr.literal.byte-char.no-suffix)

The token must not have a suffix.

[expr.literal.byte-char.literal-content](#r-expr.literal.byte-char.literal-content)

The token’s *literal content* is the sequence of characters following the first `U+0027` (`'`) and preceding the last `U+0027` (`'`) in the string representation of the token.

[expr.literal.byte-char.represented](#r-expr.literal.byte-char.represented)

The literal expression’s *represented character* is derived from the literal content as follows:

[expr.literal.byte-char.escape](#r-expr.literal.byte-char.escape)

- If the literal content is one of the following forms of escape sequence, the represented character is the escape sequence’s escaped value:
  - [Simple escapes](#simple-escapes)
  - [8-bit escapes](#8-bit-escapes)

[expr.literal.byte-char.single](#r-expr.literal.byte-char.single)

- Otherwise the represented character is the single character that makes up the literal content.

[expr.literal.byte-char.result](#r-expr.literal.byte-char.result)

The expression’s value is the represented character’s [Unicode scalar value](http://www.unicode.org/glossary/#unicode_scalar_value).

> ![](../../assets/inline/literal-expr-7.svg)Note
>
> The permitted forms of a [BYTE_LITERAL](../tokens.md#grammar-BYTE_LITERAL) token ensure that these rules always produce a single character, whose Unicode scalar value is in the range of [`u8`](../types/numeric.md).

Examples of byte literal expressions:

``` playground
#![allow(unused)]
fn main() {
b'R';                              // 82
b'\'';                             // 39
b'\x52';                           // 82
b'\xA0';                           // 160
}
```

[expr.literal.byte-string](#r-expr.literal.byte-string)

## Byte string literal expressions

[expr.literal.byte-string.intro](#r-expr.literal.byte-string.intro)

A byte string literal expression consists of a single [BYTE_STRING_LITERAL](../tokens.md#grammar-BYTE_STRING_LITERAL) or [RAW_BYTE_STRING_LITERAL](../tokens.md#grammar-RAW_BYTE_STRING_LITERAL) token.

[expr.literal.byte-string.type](#r-expr.literal.byte-string.type)

The expression’s type is a shared reference (with `static` lifetime) to an array whose element type is [`u8`](../types/numeric.md). That is, the type is `&'static [u8; N]`, where `N` is the number of bytes in the represented string described below.

[expr.literal.byte-string.no-suffix](#r-expr.literal.byte-string.no-suffix)

The token must not have a suffix.

[expr.literal.byte-string.literal-content](#r-expr.literal.byte-string.literal-content)

The token’s *literal content* is the sequence of characters following the first `U+0022` (`"`) and preceding the last `U+0022` (`"`) in the string representation of the token.

[expr.literal.byte-string.represented](#r-expr.literal.byte-string.represented)

The literal expression’s *represented string* is a sequence of characters derived from the literal content as follows:

[expr.literal.byte-string.escape](#r-expr.literal.byte-string.escape)

- If the token is a [BYTE_STRING_LITERAL](../tokens.md#grammar-BYTE_STRING_LITERAL), each escape sequence of any of the following forms occurring in the literal content is replaced by the escape sequence’s escaped value.

  - [Simple escapes](#simple-escapes)
  - [8-bit escapes](#8-bit-escapes)
  - [String continuation escapes](#string-continuation-escapes)

  These replacements take place in left-to-right order. For example, the token `b"\\x41"` is converted to the characters `\` `x` `4` `1`.

[expr.literal.byte-string.raw](#r-expr.literal.byte-string.raw)

- If the token is a [RAW_BYTE_STRING_LITERAL](../tokens.md#grammar-RAW_BYTE_STRING_LITERAL), the represented string is identical to the literal content.

[expr.literal.byte-string.result](#r-expr.literal.byte-string.result)

The expression’s value is a reference to a statically allocated array containing the [Unicode scalar values](http://www.unicode.org/glossary/#unicode_scalar_value) of the characters in the represented string, in the same order.

> ![](../../assets/inline/literal-expr-8.svg)Note
>
> The permitted forms of [BYTE_STRING_LITERAL](../tokens.md#grammar-BYTE_STRING_LITERAL) and [RAW_BYTE_STRING_LITERAL](../tokens.md#grammar-RAW_BYTE_STRING_LITERAL) tokens ensure that these rules always produce array element values in the range of [`u8`](../types/numeric.md).

Examples of byte string literal expressions:

``` playground
#![allow(unused)]
fn main() {
b"foo"; br"foo";                     // foo
b"\"foo\""; br#""foo""#;             // "foo"

b"foo #\"# bar";
br##"foo #"# bar"##;                 // foo #"# bar

b"\x52"; b"R"; br"R";                // R
b"\\x52"; br"\x52";                  // \x52
}
```

[expr.literal.c-string](#r-expr.literal.c-string)

## C string literal expressions

[expr.literal.c-string.intro](#r-expr.literal.c-string.intro)

A C string literal expression consists of a single [C_STRING_LITERAL](../tokens.md#grammar-C_STRING_LITERAL) or [RAW_C_STRING_LITERAL](../tokens.md#grammar-RAW_C_STRING_LITERAL) token.

[expr.literal.c-string.type](#r-expr.literal.c-string.type)

The expression’s type is a shared reference (with `static` lifetime) to the standard library [CStr](https://doc.rust-lang.org/1.100.0/core/ffi/c_str/struct.CStr.html) type. That is, the type is `&'static core::ffi::CStr`.

[expr.literal.c-string.no-suffix](#r-expr.literal.c-string.no-suffix)

The token must not have a suffix.

[expr.literal.c-string.literal-content](#r-expr.literal.c-string.literal-content)

The token’s *literal content* is the sequence of characters following the first `"` and preceding the last `"` in the string representation of the token.

[expr.literal.c-string.represented](#r-expr.literal.c-string.represented)

The literal expression’s *represented bytes* are a sequence of bytes derived from the literal content as follows:

[expr.literal.c-string.escape](#r-expr.literal.c-string.escape)

- If the token is a [C_STRING_LITERAL](../tokens.md#grammar-C_STRING_LITERAL), the literal content is treated as a sequence of items, each of which is either a single Unicode character other than `\` or an [escape](#escapes). The sequence of items is converted to a sequence of bytes as follows:
  - Each single Unicode character contributes its UTF-8 representation.
  - Each [simple escape](#simple-escapes) contributes the [Unicode scalar value](http://www.unicode.org/glossary/#unicode_scalar_value) of its escaped value.
  - Each [8-bit escape](#8-bit-escapes) contributes a single byte containing the [Unicode scalar value](http://www.unicode.org/glossary/#unicode_scalar_value) of its escaped value.
  - Each [unicode escape](#unicode-escapes) contributes the UTF-8 representation of its escaped value.
  - Each [string continuation escape](#string-continuation-escapes) contributes no bytes.

[expr.literal.c-string.raw](#r-expr.literal.c-string.raw)

- If the token is a [RAW_C_STRING_LITERAL](../tokens.md#grammar-RAW_C_STRING_LITERAL), the represented bytes are the UTF-8 encoding of the literal content.

> ![](../../assets/inline/literal-expr-9.svg)Note
>
> The permitted forms of [C_STRING_LITERAL](../tokens.md#grammar-C_STRING_LITERAL) and [RAW_C_STRING_LITERAL](../tokens.md#grammar-RAW_C_STRING_LITERAL) tokens ensure that the represented bytes never include a null byte.

[expr.literal.c-string.result](#r-expr.literal.c-string.result)

The expression’s value is a reference to a statically allocated [CStr](https://doc.rust-lang.org/1.100.0/core/ffi/c_str/struct.CStr.html) whose array of bytes contains the represented bytes followed by a null byte.

Examples of C string literal expressions:

``` playground
#![allow(unused)]
fn main() {
c"foo"; cr"foo";                     // foo
c"\"foo\""; cr#""foo""#;             // "foo"

c"foo #\"# bar";
cr##"foo #"# bar"##;                 // foo #"# bar

c"\x52"; c"R"; cr"R";                // R
c"\\x52"; cr"\x52";                  // \x52

c"æ";                                // LATIN SMALL LETTER AE (U+00E6)
c"\u{00E6}";                         // LATIN SMALL LETTER AE (U+00E6)
c"\xC3\xA6";                         // LATIN SMALL LETTER AE (U+00E6)

c"\xE6".to_bytes();                  // [230]
c"\u{00E6}".to_bytes();              // [195, 166]
}
```

[expr.literal.int](#r-expr.literal.int)

## Integer literal expressions

[expr.literal.int.intro](#r-expr.literal.int.intro)

An integer literal expression consists of a single [INTEGER_LITERAL](../tokens.md#grammar-INTEGER_LITERAL) token.

[expr.literal.int.suffix](#r-expr.literal.int.suffix)

If the token has a [suffix](../tokens.md#suffixes), the suffix must be the name of one of the [primitive integer types](../types/numeric.md): `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `u128`, `i128`, `usize`, or `isize`, and the expression has that type.

[expr.literal.int.infer](#r-expr.literal.int.infer)

If the token has no suffix, the expression’s type is determined by type inference:

[expr.literal.int.inference-unique-type](#r-expr.literal.int.inference-unique-type)

- If an integer type can be *uniquely* determined from the surrounding program context, the expression has that type.

[expr.literal.int.inference-default](#r-expr.literal.int.inference-default)

- If the program context under-constrains the type, it defaults to the signed 32-bit integer `i32`.

[expr.literal.int.inference-error](#r-expr.literal.int.inference-error)

- If the program context over-constrains the type, it is considered a static type error.

Examples of integer literal expressions:

``` playground
#![allow(unused)]
fn main() {
123;                               // type i32
123i32;                            // type i32
123u32;                            // type u32
123_u32;                           // type u32
let a: u64 = 123;                  // type u64

0xff;                              // type i32
0xff_u8;                           // type u8

0o70;                              // type i32
0o70_i16;                          // type i16

0b1111_1111_1001_0000;             // type i32
0b1111_1111_1001_0000i64;          // type i64

0usize;                            // type usize
}
```

[expr.literal.int.representation](#r-expr.literal.int.representation)

The value of the expression is determined from the string representation of the token as follows:

[expr.literal.int.radix](#r-expr.literal.int.radix)

- An integer radix is chosen by inspecting the first two characters of the string, as follows:

  - `0b` indicates radix 2
  - `0o` indicates radix 8
  - `0x` indicates radix 16
  - otherwise the radix is 10.

[expr.literal.int.radix-prefix-stripped](#r-expr.literal.int.radix-prefix-stripped)

- If the radix is not 10, the first two characters are removed from the string.

[expr.literal.int.type-suffix-stripped](#r-expr.literal.int.type-suffix-stripped)

- Any suffix is removed from the string.

[expr.literal.int.separators-stripped](#r-expr.literal.int.separators-stripped)

- Any underscores are removed from the string.

[expr.literal.int.u128-value](#r-expr.literal.int.u128-value)

- The string is converted to a `u128` value as if by [`u128::from_str_radix`](https://doc.rust-lang.org/1.100.0/std/primitive.u128.html#method.from_str_radix) with the chosen radix. If the value does not fit in `u128`, it is a compiler error.

[expr.literal.int.cast](#r-expr.literal.int.cast)

- The `u128` value is converted to the expression’s type via a [numeric cast](operator-expr.md#numeric-cast).

> ![](../../assets/inline/literal-expr-10.svg)Note
>
> The final cast will truncate the value of the literal if it does not fit in the expression’s type. `rustc` includes a [lint check](../attributes/diagnostics.md#lint-check-attributes) named `overflowing_literals`, defaulting to `deny`, which rejects expressions where this occurs.

> ![](../../assets/inline/literal-expr-11.svg)Note
>
> `-1i8`, for example, is an application of the [negation operator](operator-expr.md#negation-operators) to the literal expression `1i8`, not a single integer literal expression. See [Overflow](operator-expr.md#overflow) for notes on representing the most negative value for a signed type.

[expr.literal.float](#r-expr.literal.float)

## Floating-point literal expressions

[expr.literal.float.intro](#r-expr.literal.float.intro)

A floating-point literal expression has one of two forms:

- a single [FLOAT_LITERAL](../tokens.md#grammar-FLOAT_LITERAL) token
- a single [INTEGER_LITERAL](../tokens.md#grammar-INTEGER_LITERAL) token which has a suffix and no radix indicator

[expr.literal.float.suffix](#r-expr.literal.float.suffix)

If the token has a [suffix](../tokens.md#suffixes), the suffix must be the name of one of the [primitive floating-point types](../types/numeric.md#floating-point-types): `f32` or `f64`, and the expression has that type.

[expr.literal.float.infer](#r-expr.literal.float.infer)

If the token has no suffix, the expression’s type is determined by type inference:

[expr.literal.float.inference-unique-type](#r-expr.literal.float.inference-unique-type)

- If a floating-point type can be *uniquely* determined from the surrounding program context, the expression has that type.

[expr.literal.float.inference-default](#r-expr.literal.float.inference-default)

- If the program context under-constrains the type, it defaults to `f64`.

[expr.literal.float.inference-error](#r-expr.literal.float.inference-error)

- If the program context over-constrains the type, it is considered a static type error.

Examples of floating-point literal expressions:

``` playground
#![allow(unused)]
fn main() {
123.0f64;        // type f64
0.1f64;          // type f64
0.1f32;          // type f32
12E+99_f64;      // type f64
5f32;            // type f32
let x: f64 = 2.; // type f64
}
```

[expr.literal.float.result](#r-expr.literal.float.result)

The value of the expression is determined from the string representation of the token as follows:

[expr.literal.float.type-suffix-stripped](#r-expr.literal.float.type-suffix-stripped)

- Any suffix is removed from the string.

[expr.literal.float.separators-stripped](#r-expr.literal.float.separators-stripped)

- Any underscores are removed from the string.

[expr.literal.float.value](#r-expr.literal.float.value)

- The string is converted to the expression’s type as if by [`f32::from_str`](https://doc.rust-lang.org/1.100.0/core/primitive.f32.html#method.from_str) or [`f64::from_str`](https://doc.rust-lang.org/1.100.0/core/primitive.f64.html#method.from_str).

> ![](../../assets/inline/literal-expr-12.svg)Note
>
> `-1.0`, for example, is an application of the [negation operator](operator-expr.md#negation-operators) to the literal expression `1.0`, not a single floating-point literal expression.

> ![](../../assets/inline/literal-expr-13.svg)Note
>
> `inf` and `NaN` are not literal tokens. The [`f32::INFINITY`](https://doc.rust-lang.org/1.100.0/std/primitive.f32.html#associatedconstant.INFINITY), [`f64::INFINITY`](https://doc.rust-lang.org/1.100.0/std/primitive.f64.html#associatedconstant.INFINITY), [`f32::NAN`](https://doc.rust-lang.org/1.100.0/std/primitive.f32.html#associatedconstant.NAN), and [`f64::NAN`](https://doc.rust-lang.org/1.100.0/std/primitive.f64.html#associatedconstant.NAN) constants can be used instead of literal expressions. In `rustc`, a literal large enough to be evaluated as infinite will trigger the `overflowing_literals` lint check.

[expr.literal.bool](#r-expr.literal.bool)

## Boolean literal expressions

[expr.literal.bool.intro](#r-expr.literal.bool.intro)

A boolean literal expression consists of one of the keywords `true` or `false`.

[expr.literal.bool.result](#r-expr.literal.bool.result)

The expression’s type is the primitive [boolean type](../types/boolean.md), and its value is:

- true if the keyword is `true`
- false if the keyword is `false`
