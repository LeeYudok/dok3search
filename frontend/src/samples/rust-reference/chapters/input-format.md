# Input format

[input](#r-input)

# Input format

[input.syntax](#r-input.syntax)

**Lexer**\
[CHAR](#railroad-CHAR) → \[U+0000-U+D7FF U+E000-U+10FFFF\] // a Unicode scalar value

[ASCII](#railroad-ASCII) → \[U+0000-U+007F\]

[NUL](#railroad-NUL) → U+0000

[EOF](#railroad-EOF) → \![CHAR](#grammar-CHAR) // End of file or input

Show Railroad

![](../assets/inline/input-format-1.svg) ![](../assets/inline/input-format-2.svg) ![](../assets/inline/input-format-3.svg) ![](../assets/inline/input-format-4.svg) [input.intro](#r-input.intro)

This chapter describes how a source file is interpreted as a sequence of tokens.

See [Crates and source files](crates-and-source-files.md) for a description of how programs are organised into files.

[input.encoding](#r-input.encoding)

## Source encoding

[input.encoding.utf8](#r-input.encoding.utf8)\
Tests Tests with this rule:

- [tests/ui/macros/not-utf8.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/not-utf8.rs)
- [tests/ui/parser/utf16-be-without-bom.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/parser/utf16-be-without-bom.rs)
- [tests/ui/parser/utf16-le-without-bom.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/parser/utf16-le-without-bom.rs)

Each source file is interpreted as a sequence of Unicode characters encoded in UTF-8.

[input.encoding.invalid](#r-input.encoding.invalid)\
Tests Tests with this rule:

- [tests/ui/macros/not-utf8.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/macros/not-utf8.rs)

It is an error if the file is not valid UTF-8.

[input.byte-order-mark](#r-input.byte-order-mark)\
Tests Tests with this rule:

- [tests/ui/codemap_tests/utf8-bom.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/codemap_tests/utf8-bom.rs)
- [tests/ui/json/json-bom-plus-crlf-multifile.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/json/json-bom-plus-crlf-multifile.rs)
- [tests/ui/json/json-bom-plus-crlf.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/json/json-bom-plus-crlf.rs)

## Byte order mark removal

If the first character in the sequence is `U+FEFF` ([BYTE ORDER MARK](https://en.wikipedia.org/wiki/Byte_order_mark#UTF-8)), it is removed.

[input.crlf](#r-input.crlf)\
Tests Tests with this rule:

- [tests/ui/json/json-bom-plus-crlf-multifile.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/json/json-bom-plus-crlf-multifile.rs)
- [tests/ui/json/json-bom-plus-crlf.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/json/json-bom-plus-crlf.rs)
- [tests/ui/lexer/lexer-crlf-line-endings-string-literal-doc-comment.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/lexer/lexer-crlf-line-endings-string-literal-doc-comment.rs)

## CRLF normalization

Each pair of characters `U+000D` (CR) immediately followed by `U+000A` (LF) is replaced by a single `U+000A` (LF). This happens once, not repeatedly, so after the normalization, there can still exist `U+000D` (CR) immediately followed by `U+000A` (LF) in the input (e.g. if the raw input contained “CR CR LF LF”).

Other occurrences of the character `U+000D` (CR) are left in place (they are treated as [whitespace](whitespace.md)).

[input.shebang](#r-input.shebang)

## Shebang removal

[input.shebang.removal](#r-input.shebang.removal)

If a [shebang](shebang.md) is present, it is removed from the input sequence (and is therefore ignored).

[input.tokenization](#r-input.tokenization)

## Tokenization

The resulting sequence of characters is then converted into tokens as described in the remainder of this chapter.

> ![](../assets/inline/input-format-5.svg)Note
>
> The standard library [`include!`](../core/macro.include.html) macro applies the following transformations to the file it reads:
>
> - Byte order mark removal.
> - CRLF normalization.
> - Shebang removal when invoked in an item context (as opposed to expression or statement contexts).
>
> The [`include_str!`](../core/macro.include_str.html) and [`include_bytes!`](../core/macro.include_bytes.html) macros do not apply these transformations.
