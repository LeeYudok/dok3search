# Shebang

[shebang](#r-shebang)

# Shebang

[shebang.intro](#r-shebang.intro)

A *[shebang](https://en.wikipedia.org/wiki/Shebang_(Unix))* is an optional line that is typically used in Unix-like systems to specify an interpreter for executing the file.

> ![](../assets/inline/shebang-1.svg)Example
>
> ``` rust
> #!/usr/bin/env rustx
>
> fn main() {
>     println!("Hello!");
> }
> ```

[shebang.syntax](#r-shebang.syntax)\
Tests Tests with this rule:

- [tests/ui/parser/shebang/shebang-comment.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/parser/shebang/shebang-comment.rs)
- [tests/ui/parser/shebang/shebang-empty.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/parser/shebang/shebang-empty.rs)
- [tests/ui/parser/shebang/shebang-space.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/parser/shebang/shebang-space.rs)
- [tests/ui/parser/shebang/valid-shebang.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/parser/shebang/valid-shebang.rs)

**Lexer**\
[SHEBANG](#railroad-SHEBANG) →\
    #! !( ( [WHITESPACE](whitespace.md#grammar-WHITESPACE) \| [LINE_COMMENT](comments.md#grammar-LINE_COMMENT) \| [SHEBANG_BLOCK_COMMENT](#grammar-SHEBANG_BLOCK_COMMENT) )\* \[ )\
    ~[LF](whitespace.md#grammar-LF)\* ( [LF](whitespace.md#grammar-LF) \| [EOF](input-format.md#grammar-EOF) )

[SHEBANG_BLOCK_COMMENT](#railroad-SHEBANG_BLOCK_COMMENT) →\
    /\* !( ! \| \* \![\* /\] )\
      ( [SHEBANG_NESTED_BLOCK_COMMENT](#grammar-SHEBANG_NESTED_BLOCK_COMMENT) \| ( !( \*/ \| /\* ) [CHAR](input-format.md#grammar-CHAR) ) )\*\
    \*/

[SHEBANG_NESTED_BLOCK_COMMENT](#railroad-SHEBANG_NESTED_BLOCK_COMMENT) →\
    /\*\
      ( [SHEBANG_NESTED_BLOCK_COMMENT](#grammar-SHEBANG_NESTED_BLOCK_COMMENT) \| ( !( \*/ \| /\* ) [CHAR](input-format.md#grammar-CHAR) ) )\*\
    \*/

Show Railroad

![](../assets/inline/shebang-2.svg) ![](../assets/inline/shebang-3.svg) ![](../assets/inline/shebang-4.svg) [shebang.syntax-description](#r-shebang.syntax-description)\
Tests Tests with this rule:

- [tests/ui/parser/shebang/multiline-attrib.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/parser/shebang/multiline-attrib.rs)
- [tests/ui/parser/shebang/regular-attrib.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/parser/shebang/regular-attrib.rs)
- [tests/ui/parser/shebang/shebang-and-attrib.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/parser/shebang/shebang-and-attrib.rs)
- [tests/ui/parser/shebang/shebang-doc-comment.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/parser/shebang/shebang-doc-comment.rs)
- [tests/ui/parser/shebang/sneaky-attrib.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/parser/shebang/sneaky-attrib.rs)

The shebang starts with the characters `#!` and extends through the first `U+000A` (LF) or through EOF if no LF is present. If the `#!` characters are followed by `[` (ignoring any intervening [whitespace](whitespace.md) or [non-doc comments](comments.md#r-comments.normal)), the line is not considered a shebang (to avoid ambiguity with an [inner attribute](attributes.md)).

> ![](../assets/inline/shebang-5.svg)Note
>
> Doc comments are not ignored when determining whether `[` follows the `#!` characters. For example, `#! /*! */ [allow(unused)]` at the start of a file is a shebang, not an inner attribute, because `/*! */` is a doc comment. Likewise, text following `#!` that resembles an unterminated block comment, as in `#!/*`, does not cause an error; the line is a shebang.

[shebang.position](#r-shebang.position)\
Tests Tests with this rule:

- [tests/ui/parser/shebang/issue-71471-ignore-tidy.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/parser/shebang/issue-71471-ignore-tidy.rs)
- [tests/ui/parser/shebang/shebang-must-start-file.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/parser/shebang/shebang-must-start-file.rs)

The shebang may appear immediately at the start of the file or after the optional [byte order mark](https://en.wikipedia.org/wiki/Byte_order_mark#UTF-8).
