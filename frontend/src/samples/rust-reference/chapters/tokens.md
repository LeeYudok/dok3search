# Tokens

[lex.token](#r-lex.token)

# Tokens

[lex.token.syntax](#r-lex.token.syntax)

**Lexer**\
[Token](#railroad-Token) →\
      [RESERVED_TOKEN](#grammar-RESERVED_TOKEN)\
    \| [RAW_IDENTIFIER](identifiers.md#grammar-RAW_IDENTIFIER)\
    \| [CHAR_LITERAL](#grammar-CHAR_LITERAL)\
    \| [STRING_LITERAL](#grammar-STRING_LITERAL)\
    \| [RAW_STRING_LITERAL](#grammar-RAW_STRING_LITERAL)\
    \| [BYTE_LITERAL](#grammar-BYTE_LITERAL)\
    \| [BYTE_STRING_LITERAL](#grammar-BYTE_STRING_LITERAL)\
    \| [RAW_BYTE_STRING_LITERAL](#grammar-RAW_BYTE_STRING_LITERAL)\
    \| [C_STRING_LITERAL](#grammar-C_STRING_LITERAL)\
    \| [RAW_C_STRING_LITERAL](#grammar-RAW_C_STRING_LITERAL)\
    \| [FLOAT_LITERAL](#grammar-FLOAT_LITERAL)\
    \| [INTEGER_LITERAL](#grammar-INTEGER_LITERAL)\
    \| [LIFETIME_TOKEN](#grammar-LIFETIME_TOKEN)\
    \| [PUNCTUATION](#grammar-PUNCTUATION)\
    \| [IDENTIFIER_OR_KEYWORD](identifiers.md#grammar-IDENTIFIER_OR_KEYWORD)

Show Railroad

![](../assets/inline/tokens-1.svg) [lex.token.intro](#r-lex.token.intro)

Tokens are primitive productions in the grammar defined by regular (non-recursive) languages. Rust source input can be broken down into the following kinds of tokens:

- [Keywords](keywords.md)
- [Identifiers](identifiers.md)
- [Literals](#literals)
- [Lifetimes](#lifetimes-and-loop-labels)
- [Punctuation](#punctuation)
- [Delimiters](#delimiters)

Within this documentation’s grammar, “simple” tokens are given in [string table production](notation.md#string-table-productions) form, and appear in `monospace` font.

[lex.token.literal](#r-lex.token.literal)

## Literals

[lex.token.literal.intro](#r-lex.token.literal.intro)

Literals are tokens used in [literal expressions](expressions/literal-expr.md).

### Examples

#### Characters and strings

|  | Example | `#` sets[1](#footnote-nsets) | Characters | Escapes |
|----|----|----|----|----|
| [Character](#character-literals) | `'H'` | 0 | All Unicode | [Quote](#quote-escapes) & [ASCII](#ascii-escapes) & [Unicode](#unicode-escapes) |
| [String](#string-literals) | `"hello"` | 0 | All Unicode | [Quote](#quote-escapes) & [ASCII](#ascii-escapes) & [Unicode](#unicode-escapes) |
| [Raw string](#raw-string-literals) | `r#"hello"#` | \<256 | All Unicode | `N/A` |
| [Byte](#byte-literals) | `b'H'` | 0 | All ASCII | [Quote](#quote-escapes) & [Byte](#byte-escapes) |
| [Byte string](#byte-string-literals) | `b"hello"` | 0 | All ASCII | [Quote](#quote-escapes) & [Byte](#byte-escapes) |
| [Raw byte string](#raw-byte-string-literals) | `br#"hello"#` | \<256 | All ASCII | `N/A` |
| [C string](#c-string-literals) | `c"hello"` | 0 | All Unicode | [Quote](#quote-escapes) & [Byte](#byte-escapes) & [Unicode](#unicode-escapes) |
| [Raw C string](#raw-c-string-literals) | `cr#"hello"#` | \<256 | All Unicode | `N/A` |

#### ASCII escapes

|        | Name                                                    |
|--------|---------------------------------------------------------|
| `\x41` | 7-bit character code (exactly 2 hex digits, up to 0x7F) |
| `\n`   | Newline                                                 |
| `\r`   | Carriage return                                         |
| `\t`   | Tab                                                     |
| `\\`   | Backslash                                               |
| `\0`   | Null                                                    |

#### Byte escapes

|        | Name                                        |
|--------|---------------------------------------------|
| `\x7F` | 8-bit character code (exactly 2 hex digits) |
| `\n`   | Newline                                     |
| `\r`   | Carriage return                             |
| `\t`   | Tab                                         |
| `\\`   | Backslash                                   |
| `\0`   | Null                                        |

#### Unicode escapes

|            | Name                                               |
|------------|----------------------------------------------------|
| `\u{7FFF}` | 24-bit Unicode character code (up to 6 hex digits) |

#### Quote escapes

|      | Name         |
|------|--------------|
| `\'` | Single quote |
| `\"` | Double quote |

#### Numbers

| [Number literals](#number-literals)[2](#footnote-nl) | Example | Exponentiation |
|----|----|----|
| Decimal integer | `98_222` | `N/A` |
| Hex integer | `0xff` | `N/A` |
| Octal integer | `0o77` | `N/A` |
| Binary integer | `0b1111_0000` | `N/A` |
| Floating-point | `123.0E+77` | `Optional` |

[lex.token.literal.suffix](#r-lex.token.literal.suffix)

#### Suffixes

[lex.token.literal.literal.suffix.intro](#r-lex.token.literal.literal.suffix.intro)

A suffix is a sequence of characters following (without intervening whitespace) the primary part of a literal of the same form as a non-raw identifier or keyword.

[lex.token.literal.suffix.syntax](#r-lex.token.literal.suffix.syntax)

**Lexer**\
[SUFFIX](#railroad-SUFFIX) →\
      \_ ^ [XID_Continue](identifiers.md#grammar-XID_Continue)+\
    \| [XID_Start](identifiers.md#grammar-XID_Start) [XID_Continue](identifiers.md#grammar-XID_Continue)\*

Show Railroad

![](../assets/inline/tokens-2.svg) [lex.token.literal.suffix.validity](#r-lex.token.literal.suffix.validity)

Any kind of literal (string, integer, etc.) with any suffix is valid as a token.

A literal token with any suffix can be passed to a macro without producing an error. The macro itself will decide how to interpret such a token and whether to produce an error or not. In particular, the `literal` fragment specifier for by-example macros matches literal tokens with arbitrary suffixes.

``` playground
#![allow(unused)]
fn main() {
macro_rules! blackhole { ($tt:tt) => () }
macro_rules! blackhole_lit { ($l:literal) => () }

blackhole!("string"suffix); // OK
blackhole_lit!(1suffix); // OK
}
```

[lex.token.literal.suffix.parse](#r-lex.token.literal.suffix.parse)

However, suffixes on literal tokens which are interpreted as literal expressions or patterns are restricted. Any suffixes are rejected on non-numeric literal tokens, and numeric literal tokens are accepted only with suffixes from the list below.

| Integer | Floating-point |
|----|----|
| `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `u128`, `i128`, `usize`, `isize` | `f32`, `f64` |

### Character and string literals

[lex.token.literal.char](#r-lex.token.literal.char)

#### Character literals

[lex.token.literal.char.syntax](#r-lex.token.literal.char.syntax)

**Lexer**\
[CHAR_LITERAL](#railroad-CHAR_LITERAL) →\
    '\
        ( ~\[' \\ [LF](whitespace.md#grammar-LF) [CR](whitespace.md#grammar-CR) [TAB](whitespace.md#grammar-TAB)\] \| [QUOTE_ESCAPE](#grammar-QUOTE_ESCAPE) \| [ASCII_ESCAPE](#grammar-ASCII_ESCAPE) \| [UNICODE_ESCAPE](#grammar-UNICODE_ESCAPE) )\
    ' [SUFFIX](#grammar-SUFFIX)?

[QUOTE_ESCAPE](#railroad-QUOTE_ESCAPE) → \\ \| \\

[ASCII_ESCAPE](#railroad-ASCII_ESCAPE) →\
      \x [OCT_DIGIT](#grammar-OCT_DIGIT) [HEX_DIGIT](#grammar-HEX_DIGIT)\
    \| \n \| \r \| \t \| \\ \| \0

[UNICODE_ESCAPE](#railroad-UNICODE_ESCAPE) →\
    \u{ ( [HEX_DIGIT](#grammar-HEX_DIGIT) \_\* )1..=6valid hex char value }​[3](#footnote-valid-hex-char)

Show Railroad

![](../assets/inline/tokens-3.svg) ![](../assets/inline/tokens-4.svg) ![](../assets/inline/tokens-5.svg) ![](../assets/inline/tokens-6.svg) [lex.token.literal.char.intro](#r-lex.token.literal.char.intro)

A *character literal* is a single Unicode character enclosed within two `U+0027` (single-quote) characters, with the exception of `U+0027` itself, which must be *escaped* by a preceding `U+005C` character (`\`).

[lex.token.literal.str](#r-lex.token.literal.str)

#### String literals

[lex.token.literal.str.syntax](#r-lex.token.literal.str.syntax)

**Lexer**\
[STRING_LITERAL](#railroad-STRING_LITERAL) →\
    " (\
        ~\[" \\ [CR](whitespace.md#grammar-CR)\]\
      \| [QUOTE_ESCAPE](#grammar-QUOTE_ESCAPE)\
      \| [ASCII_ESCAPE](#grammar-ASCII_ESCAPE)\
      \| [UNICODE_ESCAPE](#grammar-UNICODE_ESCAPE)\
      \| [STRING_CONTINUE](#grammar-STRING_CONTINUE)\
    )\* " [SUFFIX](#grammar-SUFFIX)?

[STRING_CONTINUE](#railroad-STRING_CONTINUE) → \\ [LF](whitespace.md#grammar-LF) \[[TAB](whitespace.md#grammar-TAB) [LF](whitespace.md#grammar-LF) [CR](whitespace.md#grammar-CR) [SP](whitespace.md#grammar-SP)\]\*

Show Railroad

![](../assets/inline/tokens-7.svg) ![](../assets/inline/tokens-8.svg) [lex.token.literal.str.intro](#r-lex.token.literal.str.intro)

A *string literal* is a sequence of any Unicode characters enclosed within two `U+0022` (double-quote) characters, with the exception of `U+0022` itself, which must be *escaped* by a preceding `U+005C` character (`\`).

[lex.token.literal.str.linefeed](#r-lex.token.literal.str.linefeed)

Line-breaks, represented by the character `U+000A` (LF), are allowed in string literals. The character `U+000D` (CR) may not appear in a string literal. When an unescaped `U+005C` character (`\`) occurs immediately before a line break, the line break does not appear in the string represented by the token. See [String continuation escapes](expressions/literal-expr.md#string-continuation-escapes) for details.

[lex.token.literal.char-escape](#r-lex.token.literal.char-escape)

#### Character escapes

[lex.token.literal.char-escape.intro](#r-lex.token.literal.char-escape.intro)

Some additional *escapes* are available in either character or non-raw string literals. An escape starts with a `U+005C` (`\`) and continues with one of the following forms:

[lex.token.literal.char-escape.ascii](#r-lex.token.literal.char-escape.ascii)

- A *7-bit code point escape* starts with `U+0078` (`x`) and is followed by exactly two *hex digits* with value up to `0x7F`. It denotes the ASCII character with value equal to the provided hex value. Higher values are not permitted because it is ambiguous whether they mean Unicode code points or byte values.

[lex.token.literal.char-escape.unicode](#r-lex.token.literal.char-escape.unicode)

- A *24-bit code point escape* starts with `U+0075` (`u`) and is followed by up to six *hex digits* surrounded by braces `U+007B` (`{`) and `U+007D` (`}`). It denotes the Unicode code point equal to the provided hex value. The value must be a valid Unicode scalar value.

[lex.token.literal.char-escape.whitespace](#r-lex.token.literal.char-escape.whitespace)

- A *whitespace escape* is one of the characters `U+006E` (`n`), `U+0072` (`r`), or `U+0074` (`t`), denoting the Unicode values `U+000A` (LF), `U+000D` (CR) or `U+0009` (HT) respectively.

[lex.token.literal.char-escape.null](#r-lex.token.literal.char-escape.null)

- The *null escape* is the character `U+0030` (`0`) and denotes the Unicode value `U+0000` (NUL).

[lex.token.literal.char-escape.slash](#r-lex.token.literal.char-escape.slash)

- The *backslash escape* is the character `U+005C` (`\`) which must be escaped in order to denote itself.

[lex.token.literal.str-raw](#r-lex.token.literal.str-raw)

#### Raw string literals

[lex.token.literal.str-raw.syntax](#r-lex.token.literal.str-raw.syntax)

**Lexer**\
[RAW_STRING_LITERAL](#railroad-RAW_STRING_LITERAL) →\
      r " ^ [RAW_STRING_CONTENT](#grammar-RAW_STRING_CONTENT) " [SUFFIX](#grammar-SUFFIX)?\
    \| r \#n:1..=255 ^ " [RAW_STRING_CONTENT_HASHED](#grammar-RAW_STRING_CONTENT_HASHED) " \#n [SUFFIX](#grammar-SUFFIX)?

[RAW_STRING_CONTENT](#railroad-RAW_STRING_CONTENT) → ( !" ~[CR](whitespace.md#grammar-CR) )\*

[RAW_STRING_CONTENT_HASHED](#railroad-RAW_STRING_CONTENT_HASHED) → ( !( " \#n ) ~[CR](whitespace.md#grammar-CR) )\*

Show Railroad

![](../assets/inline/tokens-9.svg) ![](../assets/inline/tokens-10.svg) ![](../assets/inline/tokens-11.svg) [lex.token.literal.str-raw.intro](#r-lex.token.literal.str-raw.intro)

Raw string literals do not process any escapes. They start with the character `U+0072` (`r`), followed by fewer than 256 of the character `U+0023` (`#`) and a `U+0022` (double-quote) character.

[lex.token.literal.str-raw.body](#r-lex.token.literal.str-raw.body)

The *raw string body* can contain any sequence of Unicode characters other than `U+000D` (CR). It is terminated only by another `U+0022` (double-quote) character, followed by the same number of `U+0023` (`#`) characters that preceded the opening `U+0022` (double-quote) character.

[lex.token.literal.str-raw.content](#r-lex.token.literal.str-raw.content)

All Unicode characters contained in the raw string body represent themselves, the characters `U+0022` (double-quote) (except when followed by at least as many `U+0023` (`#`) characters as were used to start the raw string literal) or `U+005C` (`\`) do not have any special meaning.

Examples for string literals:

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

### Byte and byte string literals

[lex.token.byte](#r-lex.token.byte)

#### Byte literals

[lex.token.byte.syntax](#r-lex.token.byte.syntax)

**Lexer**\
[BYTE_LITERAL](#railroad-BYTE_LITERAL) →\
    b' ^ ( [ASCII_FOR_CHAR](#grammar-ASCII_FOR_CHAR) \| [BYTE_ESCAPE](#grammar-BYTE_ESCAPE) ) ' [SUFFIX](#grammar-SUFFIX)?

[ASCII_FOR_CHAR](#railroad-ASCII_FOR_CHAR) → \![' \\ [LF](whitespace.md#grammar-LF) [CR](whitespace.md#grammar-CR) [TAB](whitespace.md#grammar-TAB)\] [ASCII](input-format.md#grammar-ASCII)

[BYTE_ESCAPE](#railroad-BYTE_ESCAPE) →\
      \x [HEX_DIGIT](#grammar-HEX_DIGIT) [HEX_DIGIT](#grammar-HEX_DIGIT)\
    \| \n \| \r \| \t \| \\ \| \0 \| \\ \| \\

Show Railroad

![](../assets/inline/tokens-12.svg) ![](../assets/inline/tokens-13.svg) ![](../assets/inline/tokens-14.svg) [lex.token.byte.intro](#r-lex.token.byte.intro)

A *byte literal* is a single ASCII character (in the `U+0000` to `U+007F` range) or a single *escape* preceded by the characters `U+0062` (`b`) and `U+0027` (single-quote), and followed by the character `U+0027`. If the character `U+0027` is present within the literal, it must be *escaped* by a preceding `U+005C` (`\`) character. It is equivalent to a `u8` unsigned 8-bit integer *number literal*.

[lex.token.str-byte](#r-lex.token.str-byte)

#### Byte string literals

[lex.token.str-byte.syntax](#r-lex.token.str-byte.syntax)

**Lexer**\
[BYTE_STRING_LITERAL](#railroad-BYTE_STRING_LITERAL) →\
    b" ^ ( [ASCII_FOR_STRING](#grammar-ASCII_FOR_STRING) \| [BYTE_ESCAPE](#grammar-BYTE_ESCAPE) \| [STRING_CONTINUE](#grammar-STRING_CONTINUE) )\* " [SUFFIX](#grammar-SUFFIX)?

[ASCII_FOR_STRING](#railroad-ASCII_FOR_STRING) → \![" \\ [CR](whitespace.md#grammar-CR)\] [ASCII](input-format.md#grammar-ASCII)

Show Railroad

![](../assets/inline/tokens-15.svg) ![](../assets/inline/tokens-16.svg) [lex.token.str-byte.intro](#r-lex.token.str-byte.intro)

A non-raw *byte string literal* is a sequence of ASCII characters and *escapes*, preceded by the characters `U+0062` (`b`) and `U+0022` (double-quote), and followed by the character `U+0022`. If the character `U+0022` is present within the literal, it must be *escaped* by a preceding `U+005C` (`\`) character. Alternatively, a byte string literal can be a *raw byte string literal*, defined below.

[lex.token.str-byte.linefeed](#r-lex.token.str-byte.linefeed)

Line-breaks, represented by the character `U+000A` (LF), are allowed in byte string literals. The character `U+000D` (CR) may not appear in a byte string literal. When an unescaped `U+005C` character (`\`) occurs immediately before a line break, the line break does not appear in the string represented by the token. See [String continuation escapes](expressions/literal-expr.md#string-continuation-escapes) for details.

[lex.token.str-byte.escape](#r-lex.token.str-byte.escape)

Some additional *escapes* are available in either byte or non-raw byte string literals. An escape starts with a `U+005C` (`\`) and continues with one of the following forms:

[lex.token.str-byte.escape-byte](#r-lex.token.str-byte.escape-byte)

- A *byte escape* escape starts with `U+0078` (`x`) and is followed by exactly two *hex digits*. It denotes the byte equal to the provided hex value.

[lex.token.str-byte.escape-whitespace](#r-lex.token.str-byte.escape-whitespace)

- A *whitespace escape* is one of the characters `U+006E` (`n`), `U+0072` (`r`), or `U+0074` (`t`), denoting the bytes values `0x0A` (ASCII LF), `0x0D` (ASCII CR) or `0x09` (ASCII HT) respectively.

[lex.token.str-byte.escape-null](#r-lex.token.str-byte.escape-null)

- The *null escape* is the character `U+0030` (`0`) and denotes the byte value `0x00` (ASCII NUL).

[lex.token.str-byte.escape-slash](#r-lex.token.str-byte.escape-slash)

- The *backslash escape* is the character `U+005C` (`\`) which must be escaped in order to denote its ASCII encoding `0x5C`.

[lex.token.str-byte-raw](#r-lex.token.str-byte-raw)

#### Raw byte string literals

[lex.token.str-byte-raw.syntax](#r-lex.token.str-byte-raw.syntax)

**Lexer**\
[RAW_BYTE_STRING_LITERAL](#railroad-RAW_BYTE_STRING_LITERAL) →\
      br " ^ [RAW_BYTE_STRING_CONTENT](#grammar-RAW_BYTE_STRING_CONTENT) " [SUFFIX](#grammar-SUFFIX)?\
    \| br \#n:1..=255 ^ " [RAW_BYTE_STRING_CONTENT_HASHED](#grammar-RAW_BYTE_STRING_CONTENT_HASHED) " \#n [SUFFIX](#grammar-SUFFIX)?

[RAW_BYTE_STRING_CONTENT](#railroad-RAW_BYTE_STRING_CONTENT) → ( !" [ASCII_FOR_RAW](#grammar-ASCII_FOR_RAW) )\*

[RAW_BYTE_STRING_CONTENT_HASHED](#railroad-RAW_BYTE_STRING_CONTENT_HASHED) → ( !( " \#n ) [ASCII_FOR_RAW](#grammar-ASCII_FOR_RAW) )\*

[ASCII_FOR_RAW](#railroad-ASCII_FOR_RAW) → \![CR](whitespace.md#grammar-CR) [ASCII](input-format.md#grammar-ASCII)

Show Railroad

![](../assets/inline/tokens-17.svg) ![](../assets/inline/tokens-18.svg) ![](../assets/inline/tokens-19.svg) ![](../assets/inline/tokens-20.svg) [lex.token.str-byte-raw.intro](#r-lex.token.str-byte-raw.intro)

Raw byte string literals do not process any escapes. They start with the character `U+0062` (`b`), followed by `U+0072` (`r`), followed by fewer than 256 of the character `U+0023` (`#`), and a `U+0022` (double-quote) character.

[lex.token.str-byte-raw.body](#r-lex.token.str-byte-raw.body)

The *raw string body* can contain any sequence of ASCII characters other than `U+000D` (CR). It is terminated only by another `U+0022` (double-quote) character, followed by the same number of `U+0023` (`#`) characters that preceded the opening `U+0022` (double-quote) character. A raw byte string literal can not contain any non-ASCII byte.

[lex.token.literal.str-byte-raw.content](#r-lex.token.literal.str-byte-raw.content)

All characters contained in the raw string body represent their ASCII encoding, the characters `U+0022` (double-quote) (except when followed by at least as many `U+0023` (`#`) characters as were used to start the raw string literal) or `U+005C` (`\`) do not have any special meaning.

Examples for byte string literals:

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

### C string and raw C string literals

[lex.token.str-c](#r-lex.token.str-c)

#### C string literals

[lex.token.str-c.syntax](#r-lex.token.str-c.syntax)

**Lexer**\
[C_STRING_LITERAL](#railroad-C_STRING_LITERAL) →\
    c" ^ (\
        ~\[" \\ [CR](whitespace.md#grammar-CR) [NUL](input-format.md#grammar-NUL)\]\
      \| !( \0 \| \x00 ) [BYTE_ESCAPE](#grammar-BYTE_ESCAPE)\
      \| !( \u{ ( 0 \_\* )1..=6 } ) [UNICODE_ESCAPE](#grammar-UNICODE_ESCAPE)\
      \| [STRING_CONTINUE](#grammar-STRING_CONTINUE)\
    )\* " [SUFFIX](#grammar-SUFFIX)?

Show Railroad

![](../assets/inline/tokens-21.svg) [lex.token.str-c.intro](#r-lex.token.str-c.intro)

A *C string literal* is a sequence of Unicode characters and *escapes*, preceded by the characters `U+0063` (`c`) and `U+0022` (double-quote), and followed by the character `U+0022`. If the character `U+0022` is present within the literal, it must be *escaped* by a preceding `U+005C` (`\`) character. Alternatively, a C string literal can be a *raw C string literal*, defined below.

[lex.token.str-c.null](#r-lex.token.str-c.null)

C strings are implicitly terminated by byte `0x00`, so the C string literal `c""` is equivalent to manually constructing a `&CStr` from the byte string literal `b"\x00"`. Other than the implicit terminator, byte `0x00` is not permitted within a C string.

[lex.token.str-c.linefeed](#r-lex.token.str-c.linefeed)

Line-breaks, represented by the character `U+000A` (LF), are allowed in C string literals. The character `U+000D` (CR) may not appear in a C string literal. When an unescaped `U+005C` character (`\`) occurs immediately before a line break, the line break does not appear in the string represented by the token. See [String continuation escapes](expressions/literal-expr.md#string-continuation-escapes) for details.

[lex.token.str-c.escape](#r-lex.token.str-c.escape)

Some additional *escapes* are available in non-raw C string literals. An escape starts with a `U+005C` (`\`) and continues with one of the following forms:

[lex.token.str-c.escape-byte](#r-lex.token.str-c.escape-byte)

- A *byte escape* escape starts with `U+0078` (`x`) and is followed by exactly two *hex digits*. It denotes the byte equal to the provided hex value.

[lex.token.str-c.escape-unicode](#r-lex.token.str-c.escape-unicode)

- A *24-bit code point escape* starts with `U+0075` (`u`) and is followed by up to six *hex digits* surrounded by braces `U+007B` (`{`) and `U+007D` (`}`). It denotes the Unicode code point equal to the provided hex value, encoded as UTF-8.

[lex.token.str-c.escape-whitespace](#r-lex.token.str-c.escape-whitespace)

- A *whitespace escape* is one of the characters `U+006E` (`n`), `U+0072` (`r`), or `U+0074` (`t`), denoting the bytes values `0x0A` (ASCII LF), `0x0D` (ASCII CR) or `0x09` (ASCII HT) respectively.

[lex.token.str-c.escape-slash](#r-lex.token.str-c.escape-slash)

- The *backslash escape* is the character `U+005C` (`\`) which must be escaped in order to denote its ASCII encoding `0x5C`.

[lex.token.str-c.char-unicode](#r-lex.token.str-c.char-unicode)

A C string represents bytes with no defined encoding, but a C string literal may contain Unicode characters above `U+007F`. Such characters will be replaced with the bytes of that character’s UTF-8 representation.

The following C string literals are equivalent:

``` playground
#![allow(unused)]
fn main() {
c"æ";        // LATIN SMALL LETTER AE (U+00E6)
c"\u{00E6}";
c"\xC3\xA6";
}
```

[lex.token.str-c.edition2021](#r-lex.token.str-c.edition2021)

> 2021 Edition differences
>
> C string literals are accepted in the 2021 edition or later. In earlier editions the token `c""` is lexed as `c ""`.

[lex.token.str-c-raw](#r-lex.token.str-c-raw)

#### Raw C string literals

[lex.token.str-c-raw.syntax](#r-lex.token.str-c-raw.syntax)

**Lexer**\
[RAW_C_STRING_LITERAL](#railroad-RAW_C_STRING_LITERAL) →\
      cr " ^ [RAW_C_STRING_CONTENT](#grammar-RAW_C_STRING_CONTENT) " [SUFFIX](#grammar-SUFFIX)?\
    \| cr \#n:1..=255 ^ " [RAW_C_STRING_CONTENT_HASHED](#grammar-RAW_C_STRING_CONTENT_HASHED) " \#n [SUFFIX](#grammar-SUFFIX)?

[RAW_C_STRING_CONTENT](#railroad-RAW_C_STRING_CONTENT) → ( !" ~\[[CR](whitespace.md#grammar-CR) [NUL](input-format.md#grammar-NUL)\] )\*

[RAW_C_STRING_CONTENT_HASHED](#railroad-RAW_C_STRING_CONTENT_HASHED) → ( !( " \#n ) ~\[[CR](whitespace.md#grammar-CR) [NUL](input-format.md#grammar-NUL)\] )\*

Show Railroad

![](../assets/inline/tokens-22.svg) ![](../assets/inline/tokens-23.svg) ![](../assets/inline/tokens-24.svg) [lex.token.str-c-raw.intro](#r-lex.token.str-c-raw.intro)

Raw C string literals do not process any escapes. They start with the character `U+0063` (`c`), followed by `U+0072` (`r`), followed by fewer than 256 of the character `U+0023` (`#`), and a `U+0022` (double-quote) character.

[lex.token.str-c-raw.body](#r-lex.token.str-c-raw.body)

The *raw C string body* can contain any sequence of Unicode characters other than `U+0000` (NUL) and `U+000D` (CR). It is terminated only by another `U+0022` (double-quote) character, followed by the same number of `U+0023` (`#`) characters that preceded the opening `U+0022` (double-quote) character.

[lex.token.str-c-raw.content](#r-lex.token.str-c-raw.content)

All characters contained in the raw C string body represent themselves in UTF-8 encoding. The characters `U+0022` (double-quote) (except when followed by at least as many `U+0023` (`#`) characters as were used to start the raw C string literal) or `U+005C` (`\`) do not have any special meaning.

[lex.token.str-c-raw.edition2021](#r-lex.token.str-c-raw.edition2021)

> 2021 Edition differences
>
> Raw C string literals are accepted in the 2021 edition or later. In earlier editions the token `cr""` is lexed as `cr ""`, and `cr#""#` is lexed as `cr #""#` (which is non-grammatical).

#### Examples for C string and raw C string literals

``` playground
#![allow(unused)]
fn main() {
c"foo"; cr"foo";                     // foo
c"\"foo\""; cr#""foo""#;             // "foo"

c"foo #\"# bar";
cr##"foo #"# bar"##;                 // foo #"# bar

c"\x52"; c"R"; cr"R";                // R
c"\\x52"; cr"\x52";                  // \x52
}
```

[lex.token.literal.num](#r-lex.token.literal.num)

### Number literals

A *number literal* is either an *integer literal* or a *floating-point literal*. The grammar for recognizing the two kinds of literals is mixed.

[lex.token.literal.int](#r-lex.token.literal.int)

#### Integer literals

[lex.token.literal.int.syntax](#r-lex.token.literal.int.syntax)

**Lexer**\
[INTEGER_LITERAL](#railroad-INTEGER_LITERAL) →\
    ( [BIN_LITERAL](#grammar-BIN_LITERAL) \| [OCT_LITERAL](#grammar-OCT_LITERAL) \| [HEX_LITERAL](#grammar-HEX_LITERAL) \| [DEC_LITERAL](#grammar-DEC_LITERAL) )\
    ^ \![RESERVED_FLOAT](#grammar-RESERVED_FLOAT) [SUFFIX](#grammar-SUFFIX)?

[DEC_LITERAL](#railroad-DEC_LITERAL) → [DEC_DIGIT](#grammar-DEC_DIGIT) ( [DEC_DIGIT](#grammar-DEC_DIGIT) \| \_ )\*

[BIN_LITERAL](#railroad-BIN_LITERAL) → 0b ^ \_\* [BIN_DIGIT](#grammar-BIN_DIGIT) ( [BIN_DIGIT](#grammar-BIN_DIGIT) \| \_ )\* \![e E 2-9\]

[OCT_LITERAL](#railroad-OCT_LITERAL) → 0o ^ \_\* [OCT_DIGIT](#grammar-OCT_DIGIT) ( [OCT_DIGIT](#grammar-OCT_DIGIT) \| \_ )\* \![e E 8-9\]

[HEX_LITERAL](#railroad-HEX_LITERAL) → 0x ^ \_\* [HEX_DIGIT](#grammar-HEX_DIGIT) ( [HEX_DIGIT](#grammar-HEX_DIGIT) \| \_ )\*

[BIN_DIGIT](#railroad-BIN_DIGIT) → \[0-1\]

[OCT_DIGIT](#railroad-OCT_DIGIT) → \[0-7\]

[DEC_DIGIT](#railroad-DEC_DIGIT) → \[0-9\]

[HEX_DIGIT](#railroad-HEX_DIGIT) → \[0-9 a-f A-F\]

[RESERVED_FLOAT](#railroad-RESERVED_FLOAT) → . !( . \| \_ \| [XID_Start](identifiers.md#grammar-XID_Start) )

Show Railroad

![](../assets/inline/tokens-25.svg) ![](../assets/inline/tokens-26.svg) ![](../assets/inline/tokens-27.svg) ![](../assets/inline/tokens-28.svg) ![](../assets/inline/tokens-29.svg) ![](../assets/inline/tokens-30.svg) ![](../assets/inline/tokens-31.svg) ![](../assets/inline/tokens-32.svg) ![](../assets/inline/tokens-33.svg) ![](../assets/inline/tokens-34.svg) [lex.token.literal.int.kind](#r-lex.token.literal.int.kind)

An *integer literal* has one of four forms:

[lex.token.literal.int.kind-dec](#r-lex.token.literal.int.kind-dec)

- A *decimal literal* starts with a *decimal digit* and continues with any mixture of *decimal digits* and *underscores*.

[lex.token.literal.int.kind-hex](#r-lex.token.literal.int.kind-hex)

- A *hex literal* starts with the character sequence `U+0030` `U+0078` (`0x`) and continues as any mixture (with at least one digit) of hex digits and underscores.

[lex.token.literal.int.kind-oct](#r-lex.token.literal.int.kind-oct)

- An *octal literal* starts with the character sequence `U+0030` `U+006F` (`0o`) and continues as any mixture (with at least one digit) of octal digits and underscores.

[lex.token.literal.int.kind-bin](#r-lex.token.literal.int.kind-bin)

- A *binary literal* starts with the character sequence `U+0030` `U+0062` (`0b`) and continues as any mixture (with at least one digit) of binary digits and underscores.

[lex.token.literal.int.suffix](#r-lex.token.literal.int.suffix)

Like any literal, an integer literal may be followed (immediately, without any spaces) by a suffix as described above. The suffix may not begin with `e` or `E`, as that would be interpreted as the exponent of a floating-point literal. See [Integer literal expressions](expressions/literal-expr.md#integer-literal-expressions) for the effect of these suffixes.

Examples of integer literals which are accepted as literal expressions:

``` playground
#![allow(unused)]
fn main() {
#![allow(overflowing_literals)]
123;
123i32;
123u32;
123_u32;

0xff;
0xff_u8;
0x01_f32; // integer 7986, not floating-point 1.0
0x01_e3;  // integer 483, not floating-point 1000.0

0o70;
0o70_i16;

0b1111_1111_1001_0000;
0b1111_1111_1001_0000i64;
0b________1;

0usize;

// These are too big for their type, but are accepted as literal expressions.
128_i8;
256_u8;

// This is an integer literal, accepted as a floating-point literal expression.
5f32;
}
```

Note that `-1i8`, for example, is analyzed as two tokens: `-` followed by `1i8`.

Examples of integer literals which are not accepted as literal expressions:

``` playground
#![allow(unused)]
fn main() {
#[cfg(false)] {
0invalidSuffix;
123AFB43;
0b010a;
0xAB_CD_EF_GH;
0b1111_f32;
}
}
```

[lex.token.literal.int.invalid](#r-lex.token.literal.int.invalid)

##### Invalid integer literals

[lex.token.literal.int.invalid.intro](#r-lex.token.literal.int.invalid.intro)

Certain integer literal forms are invalid. To avoid ambiguity, the tokenizer rejects them rather than splitting them into separate tokens.

``` playground
#![allow(unused)]
fn main() {
0b0102;  // This is not `0b010` followed by `2`.
0o1279;  // This is not `0o127` followed by `9`.
0x80.0;  // This is not `0x80` followed by `.` and `0`.
0b101e;  // This is not a suffixed literal or `0b101` followed by `e`.
0b;      // This is not an integer literal or `0` followed by `b`.
0b_;     // This is not an integer literal or `0` followed by `b_`.
2em;     // This is not a suffixed literal or `2` followed by `em`.
2.0em;   // This is not a suffixed literal or `2.0` followed by `em`.
}
```

[lex.token.literal.int.out-of-range](#r-lex.token.literal.int.out-of-range)

It is an error to have an unsuffixed binary or octal literal followed without intervening whitespace by a decimal digit outside the range for its radix.

[lex.token.literal.int.period](#r-lex.token.literal.int.period)

It is an error to have an unsuffixed binary, octal, or hexadecimal literal followed without intervening whitespace by a period character (subject to the same restrictions on what may follow the period as in floating-point literals).

[lex.token.literal.int.exp](#r-lex.token.literal.int.exp)

It is an error to have an unsuffixed binary or octal literal followed without intervening whitespace by the character `e` or `E`.

[lex.token.literal.int.empty-with-radix](#r-lex.token.literal.int.empty-with-radix)

It is an error for a radix prefix to not be followed, after any optional leading underscores, by at least one valid digit for its radix.

[lex.token.literal.int.tuple-field](#r-lex.token.literal.int.tuple-field)

#### Tuple index

[lex.token.literal.int.tuple-field.syntax](#r-lex.token.literal.int.tuple-field.syntax)

**Lexer**\
[TUPLE_INDEX](#railroad-TUPLE_INDEX) → [DEC_LITERAL](#grammar-DEC_LITERAL) \| [BIN_LITERAL](#grammar-BIN_LITERAL) \| [OCT_LITERAL](#grammar-OCT_LITERAL) \| [HEX_LITERAL](#grammar-HEX_LITERAL)

Show Railroad

![](../assets/inline/tokens-35.svg) [lex.token.literal.int.tuple-field.intro](#r-lex.token.literal.int.tuple-field.intro)

A tuple index is used to refer to the fields of [tuples](types/tuple.md), [tuple structs](items/structs.md), and [tuple enum variants](items/enumerations.md).

[lex.token.literal.int.tuple-field.eq](#r-lex.token.literal.int.tuple-field.eq)\
Tests Tests with this rule:

- [tests/ui/tuple/index-invalid.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/tuple/index-invalid.rs)

Tuple indices are compared with the literal token directly. Tuple indices start with `0` and each successive index increments the value by `1` as a decimal value. Thus, only decimal values will match, and the value must not have any extra `0` prefix characters.

Tuple indices may not include any suffixes (such as `usize`).

``` playground
#![allow(unused)]
fn main() {
let example = ("dog", "cat", "horse");
let dog = example.0;
let cat = example.1;
// The following examples are invalid.
let cat = example.01;  // ERROR no field named `01`
let horse = example.0b10;  // ERROR no field named `0b10`
let unicorn = example.0usize; // ERROR suffixes on a tuple index are invalid
let underscore = example.0_0; // ERROR no field `0_0` on type `(&str, &str, &str)`
}
```

[lex.token.literal.float](#r-lex.token.literal.float)

#### Floating-point literals

[lex.token.literal.float.syntax](#r-lex.token.literal.float.syntax)

**Lexer**\
[FLOAT_LITERAL](#railroad-FLOAT_LITERAL) →\
      [DEC_LITERAL](#grammar-DEC_LITERAL) ( . [DEC_LITERAL](#grammar-DEC_LITERAL) )? [FLOAT_EXPONENT](#grammar-FLOAT_EXPONENT) [SUFFIX](#grammar-SUFFIX)?\
    \| [DEC_LITERAL](#grammar-DEC_LITERAL) . [DEC_LITERAL](#grammar-DEC_LITERAL) [SUFFIX](#grammar-SUFFIX)?\
    \| [DEC_LITERAL](#grammar-DEC_LITERAL) . !( . \| \_ \| [XID_Start](identifiers.md#grammar-XID_Start) )

[FLOAT_EXPONENT](#railroad-FLOAT_EXPONENT) →\
    ( e \| E ) ^ ( + \| - )? \_\* [DEC_DIGIT](#grammar-DEC_DIGIT) ( [DEC_DIGIT](#grammar-DEC_DIGIT) \| \_ )\*

Show Railroad

![](../assets/inline/tokens-36.svg) ![](../assets/inline/tokens-37.svg) [lex.token.literal.float.form](#r-lex.token.literal.float.form)

A *floating-point literal* has one of two forms:

- A *decimal literal* followed by a period character `U+002E` (`.`). This is optionally followed by another decimal literal, with an optional *exponent*.
- A single *decimal literal* followed by an *exponent*.

[lex.token.literal.float.suffix](#r-lex.token.literal.float.suffix)

Like integer literals, a floating-point literal may be followed by a suffix, so long as the pre-suffix part does not end with `U+002E` (`.`). The suffix may not begin with `e` or `E` if the literal does not include an exponent. See [Floating-point literal expressions](expressions/literal-expr.md#floating-point-literal-expressions) for the effect of these suffixes.

Examples of floating-point literals which are accepted as literal expressions:

``` playground
#![allow(unused)]
fn main() {
123.0f64;
0.1f64;
0.1f32;
12E+99_f64;
let x: f64 = 2.;
}
```

This last example is different because it is not possible to use the suffix syntax with a floating point literal ending in a period. `2.f64` would attempt to call a method named `f64` on `2`.

Note that `-1.0`, for example, is analyzed as two tokens: `-` followed by `1.0`.

Examples of floating-point literals which are not accepted as literal expressions:

``` playground
#![allow(unused)]
fn main() {
#[cfg(false)] {
2.0f80;
2e5f80;
2e5e6;
2.0e5e6;
1.3e10u64;
}
}
```

[lex.token.literal.float.invalid-exponent](#r-lex.token.literal.float.invalid-exponent)

It is an error for a floating-point literal to have an exponent with no digits.

``` playground
#![allow(unused)]
fn main() {
2e;   // This is not a floating-point literal or `2` followed by `e`.
2.0e; // This is not a floating-point literal or `2.0` followed by `e`.
}
```

[lex.token.life](#r-lex.token.life)

## Lifetimes and loop labels

[lex.token.life.syntax](#r-lex.token.life.syntax)

**Lexer**\
[LIFETIME_TOKEN](#railroad-LIFETIME_TOKEN) →\
      [RAW_LIFETIME](#grammar-RAW_LIFETIME)\
    \| ' [IDENTIFIER_OR_KEYWORD](identifiers.md#grammar-IDENTIFIER_OR_KEYWORD) !'

[LIFETIME_OR_LABEL](#railroad-LIFETIME_OR_LABEL) →\
      [RAW_LIFETIME](#grammar-RAW_LIFETIME)\
    \| ' [NON_KEYWORD_IDENTIFIER](identifiers.md#grammar-NON_KEYWORD_IDENTIFIER) !'

[RAW_LIFETIME](#railroad-RAW_LIFETIME) →\
    'r# ^ [IDENTIFIER_OR_KEYWORD](identifiers.md#grammar-IDENTIFIER_OR_KEYWORD) !'

[RESERVED_RAW_LIFETIME](#railroad-RESERVED_RAW_LIFETIME) → 'r# ( \_ \| crate \| self \| Self \| super ) !( ' \| [XID_Continue](identifiers.md#grammar-XID_Continue) )

Show Railroad

![](../assets/inline/tokens-38.svg) ![](../assets/inline/tokens-39.svg) ![](../assets/inline/tokens-40.svg) ![](../assets/inline/tokens-41.svg) [lex.token.life.intro](#r-lex.token.life.intro)

Lifetime parameters and [loop labels](expressions/loop-expr.md#loop-labels) use LIFETIME_OR_LABEL tokens. Any LIFETIME_TOKEN will be accepted by the lexer, and for example, can be used in macros.

[lex.token.life.raw.intro](#r-lex.token.life.raw.intro)

A raw lifetime is like a normal lifetime, but its identifier is prefixed by `r#`. (Note that the `r#` prefix is not included as part of the actual lifetime.)

[lex.token.life.raw.allowed](#r-lex.token.life.raw.allowed)

Unlike a normal lifetime, a raw lifetime may be any strict or reserved keyword except the ones listed above for `RAW_LIFETIME`.

[lex.token.life.raw.reserved](#r-lex.token.life.raw.reserved)

It is an error to use the [RESERVED_RAW_LIFETIME](#grammar-RESERVED_RAW_LIFETIME) token.

[lex.token.life.raw.edition2021](#r-lex.token.life.raw.edition2021)

> 2021 Edition differences
>
> Raw lifetimes are accepted in the 2021 edition or later. In earlier editions the token `'r#lt` is lexed as `'r # lt`.

[lex.token.punct](#r-lex.token.punct)

## Punctuation

[lex.token.punct.intro](#r-lex.token.punct.intro)

Punctuation tokens are used as operators, separators, and other parts of the grammar.

[lex.token.punct.syntax](#r-lex.token.punct.syntax)

**Lexer**\
[PUNCTUATION](#railroad-PUNCTUATION) →\
      ...\
    \| ..=\
    \| \<\<=\
    \| \>\>=\
    \| !=\
    \| %=\
    \| &&\
    \| &=\
    \| \*=\
    \| +=\
    \| -=\
    \| -\>\
    \| ..\
    \| /=\
    \| ::\
    \| \<-\
    \| \<\<\
    \| \<=\
    \| ==\
    \| =\>\
    \| \>=\
    \| \>\>\
    \| ^=\
    \| \|=\
    \| \|\|\
    \| !\
    \| \#\
    \| \$\
    \| %\
    \| &\
    \| (\
    \| )\
    \| \*\
    \| +\
    \| ,\
    \| -\
    \| .\
    \| /\
    \| :\
    \| ;\
    \| \<\
    \| =\
    \| \>\
    \| ?\
    \| @\
    \| \[\
    \| \]\
    \| ^\
    \| {\
    \| \|\
    \| }\
    \| ~

Show Railroad

![](../assets/inline/tokens-42.svg)

> ![](../assets/inline/tokens-43.svg)Note
>
> See the [syntax index](syntax-index.md#operators-and-punctuation) for links to how punctuation characters are used.

[lex.token.delim](#r-lex.token.delim)

## Delimiters

Bracket punctuation is used in various parts of the grammar. An open bracket must always be paired with a close bracket. Brackets and the tokens within them are referred to as “token trees” in [macros](macros-by-example.md). The three types of brackets are:

| Bracket | Type            |
|---------|-----------------|
| `{` `}` | Curly braces    |
| `[` `]` | Square brackets |
| `(` `)` | Parentheses     |

[lex.token.reserved](#r-lex.token.reserved)

## Reserved tokens

[lex.token.reserved.intro](#r-lex.token.reserved.intro)

Several token forms are reserved for future use or to avoid confusion. It is an error for the source input to match one of these forms.

[lex.token.reserved.syntax](#r-lex.token.reserved.syntax)

**Lexer**\
[RESERVED_TOKEN](#railroad-RESERVED_TOKEN) →\
      [RESERVED_GUARDED_STRING_LITERAL](#grammar-RESERVED_GUARDED_STRING_LITERAL)\
    \| [RESERVED_POUNDS](#grammar-RESERVED_POUNDS)\
    \| [RESERVED_RAW_IDENTIFIER](identifiers.md#grammar-RESERVED_RAW_IDENTIFIER)\
    \| [RESERVED_RAW_LIFETIME](#grammar-RESERVED_RAW_LIFETIME)\
    \| [RESERVED_TOKEN_DOUBLE_QUOTE](#grammar-RESERVED_TOKEN_DOUBLE_QUOTE)\
    \| [RESERVED_TOKEN_LIFETIME](#grammar-RESERVED_TOKEN_LIFETIME)\
    \| [RESERVED_TOKEN_POUND](#grammar-RESERVED_TOKEN_POUND)\
    \| [RESERVED_TOKEN_SINGLE_QUOTE](#grammar-RESERVED_TOKEN_SINGLE_QUOTE)

Show Railroad

![](../assets/inline/tokens-44.svg) [lex.token.reserved-prefix](#r-lex.token.reserved-prefix)

## Reserved prefixes

[lex.token.reserved-prefix.syntax](#r-lex.token.reserved-prefix.syntax)

**Lexer**\
[RESERVED_TOKEN_DOUBLE_QUOTE](#railroad-RESERVED_TOKEN_DOUBLE_QUOTE) →\
    [IDENTIFIER_OR_KEYWORD](identifiers.md#grammar-IDENTIFIER_OR_KEYWORD)except `b` or `c` or `r` or `br` or `cr` "

[RESERVED_TOKEN_SINGLE_QUOTE](#railroad-RESERVED_TOKEN_SINGLE_QUOTE) →\
    [IDENTIFIER_OR_KEYWORD](identifiers.md#grammar-IDENTIFIER_OR_KEYWORD)except `b` '

[RESERVED_TOKEN_POUND](#railroad-RESERVED_TOKEN_POUND) →\
    [IDENTIFIER_OR_KEYWORD](identifiers.md#grammar-IDENTIFIER_OR_KEYWORD)except `r` or `br` or `cr` \#

[RESERVED_TOKEN_LIFETIME](#railroad-RESERVED_TOKEN_LIFETIME) →\
    ' [IDENTIFIER_OR_KEYWORD](identifiers.md#grammar-IDENTIFIER_OR_KEYWORD)except `r` \#

Show Railroad

![](../assets/inline/tokens-45.svg) ![](../assets/inline/tokens-46.svg) ![](../assets/inline/tokens-47.svg) ![](../assets/inline/tokens-48.svg) [lex.token.reserved-prefix.intro](#r-lex.token.reserved-prefix.intro)

Some lexical forms known as *reserved prefixes* are reserved for future use.

[lex.token.reserved-prefix.id](#r-lex.token.reserved-prefix.id)

Source input which would otherwise be lexically interpreted as a non-raw identifier (or a keyword) which is immediately followed by a `#`, `'`, or `"` character (without intervening whitespace) is identified as a reserved prefix.

[lex.token.reserved-prefix.raw-token](#r-lex.token.reserved-prefix.raw-token)

Note that raw identifiers, raw string literals, and raw byte string literals may contain a `#` character but are not interpreted as containing a reserved prefix.

[lex.token.reserved-prefix.strings](#r-lex.token.reserved-prefix.strings)

Similarly the `r`, `b`, `br`, `c`, and `cr` prefixes used in raw string literals, byte literals, byte string literals, raw byte string literals, C string literals, and raw C string literals are not interpreted as reserved prefixes.

[lex.token.reserved-prefix.life](#r-lex.token.reserved-prefix.life)

Source input which would otherwise be lexically interpreted as a non-raw lifetime (or a keyword) which is immediately followed by a `#` character (without intervening whitespace) is identified as a reserved lifetime prefix.

[lex.token.reserved-prefix.edition2021](#r-lex.token.reserved-prefix.edition2021)

> 2021 Edition differences
>
> Starting with the 2021 edition, reserved prefixes are reported as an error by the lexer (in particular, they cannot be passed to macros).
>
> Before the 2021 edition, reserved prefixes are accepted by the lexer and interpreted as multiple tokens (for example, one token for the identifier or keyword, followed by a `#` token).
>
> Examples accepted in all editions:
>
> ``` playground
> #![allow(unused)]
> fn main() {
> macro_rules! lexes {($($_:tt)*) => {}}
> lexes!{a #foo}
> lexes!{continue 'foo}
> lexes!{match "..." {}}
> lexes!{r#let#foo}         // three tokens: r#let # foo
> lexes!{'prefix #lt}
> }
> ```
>
> Examples accepted before the 2021 edition but rejected later:
>
> ``` playground
> #![allow(unused)]
> fn main() {
> macro_rules! lexes {($($_:tt)*) => {}}
> lexes!{a#foo}
> lexes!{continue'foo}
> lexes!{match"..." {}}
> lexes!{'prefix#lt}
> }
> ```

[lex.token.reserved-guards](#r-lex.token.reserved-guards)

## Reserved guards

[lex.token.reserved-guards.syntax](#r-lex.token.reserved-guards.syntax)

**Lexer**\
[RESERVED_GUARDED_STRING_LITERAL](#railroad-RESERVED_GUARDED_STRING_LITERAL) → \#+ [STRING_LITERAL](#grammar-STRING_LITERAL)

[RESERVED_POUNDS](#railroad-RESERVED_POUNDS) → \#2..

Show Railroad

![](../assets/inline/tokens-49.svg) ![](../assets/inline/tokens-50.svg) [lex.token.reserved-guards.intro](#r-lex.token.reserved-guards.intro)

The reserved guards are syntax reserved for future use, and will generate a compile error if used.

[lex.token.reserved-guards.string-literal](#r-lex.token.reserved-guards.string-literal)

The *reserved guarded string literal* is a token of one or more `U+0023` (`#`) immediately followed by a [STRING_LITERAL](#grammar-STRING_LITERAL).

[lex.token.reserved-guards.pounds](#r-lex.token.reserved-guards.pounds)

The *reserved pounds* is a token of two or more `U+0023` (`#`).

[lex.token.reserved-guards.edition2024](#r-lex.token.reserved-guards.edition2024)

> 2024 Edition differences
>
> Before the 2024 edition, reserved guards are accepted by the lexer and interpreted as multiple tokens. For example, the `#"foo"#` form is interpreted as three tokens. `##` is interpreted as two tokens.

------------------------------------------------------------------------

1.  

    The number of `#`s on each side of the same literal must be equivalent. [↩](#fr-nsets-1)

    

2.  

    All number literals allow `_` as a visual separator: `1_234.0E+18f64` [↩](#fr-nl-1)

    

3.  

    See [lex.token.literal.char-escape.unicode](#r-lex.token.literal.char-escape.unicode). [↩](#fr-valid-hex-char-1)
