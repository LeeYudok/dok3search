# Whitespace

[lex.whitespace](#r-lex.whitespace)

# Whitespace

[whitespace.syntax](#r-whitespace.syntax)

**Lexer**\
[WHITESPACE](#railroad-WHITESPACE) →\
      U+0009 // Horizontal tab, `'\t'`\
    \| U+000A // Line feed, `'\n'`\
    \| U+000B // Vertical tab\
    \| U+000C // Form feed\
    \| U+000D // Carriage return, `'\r'`\
    \| U+0020 // Space, `' '`\
    \| U+0085 // Next line\
    \| U+200E // Left-to-right mark\
    \| U+200F // Right-to-left mark\
    \| U+2028 // Line separator\
    \| U+2029 // Paragraph separator

[TAB](#railroad-TAB) → U+0009 // Horizontal tab, `'\t'`

[LF](#railroad-LF) → U+000A // Line feed, `'\n'`

[CR](#railroad-CR) → U+000D // Carriage return, `'\r'`

[SP](#railroad-SP) → U+0020 // Space, `' '`

Show Railroad

![](../assets/inline/whitespace-1.svg) ![](../assets/inline/whitespace-2.svg) ![](../assets/inline/whitespace-3.svg) ![](../assets/inline/whitespace-4.svg) ![](../assets/inline/whitespace-5.svg) [lex.whitespace.intro](#r-lex.whitespace.intro)

Whitespace is any non-empty string containing only characters that have the [`Pattern_White_Space`](https://www.unicode.org/reports/tr31/) Unicode property.

[lex.whitespace.token-sep](#r-lex.whitespace.token-sep)

Rust is a “free-form” language, meaning that all forms of whitespace serve only to separate *tokens* in the grammar, and have no semantic significance.

[lex.whitespace.replacement](#r-lex.whitespace.replacement)

A Rust program has identical meaning if each whitespace element is replaced with any other legal whitespace element, such as a single space character.
