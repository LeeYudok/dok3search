# Identifiers

[ident](#r-ident)

# Identifiers

[ident.syntax](#r-ident.syntax)

**Lexer**\
[IDENTIFIER_OR_KEYWORD](#railroad-IDENTIFIER_OR_KEYWORD) → ( [XID_Start](#grammar-XID_Start) \| \_ ) [XID_Continue](#grammar-XID_Continue)\*

[XID_Start](#railroad-XID_Start) → \<`XID_Start` defined by Unicode\>

[XID_Continue](#railroad-XID_Continue) → \<`XID_Continue` defined by Unicode\>

[RAW_IDENTIFIER](#railroad-RAW_IDENTIFIER) → r# [IDENTIFIER_OR_KEYWORD](#grammar-IDENTIFIER_OR_KEYWORD)

[NON_KEYWORD_IDENTIFIER](#railroad-NON_KEYWORD_IDENTIFIER) → [IDENTIFIER_OR_KEYWORD](#grammar-IDENTIFIER_OR_KEYWORD)except a [strict](keywords.md#r-lex.keywords.strict) or [reserved](keywords.md#r-lex.keywords.reserved) keyword

[IDENTIFIER](#railroad-IDENTIFIER) → [NON_KEYWORD_IDENTIFIER](#grammar-NON_KEYWORD_IDENTIFIER) \| [RAW_IDENTIFIER](#grammar-RAW_IDENTIFIER)

[RESERVED_RAW_IDENTIFIER](#railroad-RESERVED_RAW_IDENTIFIER) →\
    r# ( \_ \| crate \| self \| Self \| super ) \![XID_Continue](#grammar-XID_Continue)

Show Railroad

![](../assets/inline/identifiers-1.svg) ![](../assets/inline/identifiers-2.svg) ![](../assets/inline/identifiers-3.svg) ![](../assets/inline/identifiers-4.svg) ![](../assets/inline/identifiers-5.svg) ![](../assets/inline/identifiers-6.svg) ![](../assets/inline/identifiers-7.svg) [ident.unicode](#r-ident.unicode)\
Tests Tests with this rule:

- [tests/ui-fulldeps/lexer/unicode-version.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui-fulldeps/lexer/unicode-version.rs)

Identifiers follow the specification in [Unicode Standard Annex \#31](https://www.unicode.org/reports/tr31/tr31-43.html) for Unicode version 17.0, with the additions described below. Some examples of identifiers:

- `foo`
- `_identifier`
- `r#true`
- `Москва`
- `東京`

[ident.profile](#r-ident.profile)

The profile used from UAX \#31 is:

- Start := [`XID_Start`](http://unicode.org/cldr/utility/list-unicodeset.jsp?a=%5B%3AXID_Start%3A%5D&abb=on&g=&i=), plus the underscore character (U+005F)
- Continue := [`XID_Continue`](http://unicode.org/cldr/utility/list-unicodeset.jsp?a=%5B%3AXID_Continue%3A%5D&abb=on&g=&i=)
- Medial := empty

> ![](../assets/inline/identifiers-8.svg)Note
>
> Identifiers starting with an underscore are typically used to indicate an identifier that is intentionally unused, and will silence the unused warning in `rustc`.

[ident.keyword](#r-ident.keyword)

Identifiers may not be a [strict](keywords.md#strict-keywords) or [reserved](keywords.md#reserved-keywords) keyword without the `r#` prefix described below in [raw identifiers](#raw-identifiers).

[ident.zero-width-chars](#r-ident.zero-width-chars)

Zero width non-joiner (ZWNJ U+200C) and zero width joiner (ZWJ U+200D) characters are not allowed in identifiers.

[ident.ascii-limitations](#r-ident.ascii-limitations)

Identifiers are restricted to the ASCII subset of [`XID_Start`](http://unicode.org/cldr/utility/list-unicodeset.jsp?a=%5B%3AXID_Start%3A%5D&abb=on&g=&i=) and [`XID_Continue`](http://unicode.org/cldr/utility/list-unicodeset.jsp?a=%5B%3AXID_Continue%3A%5D&abb=on&g=&i=) in the following situations:

- [`extern crate`](items/extern-crates.md) declarations (except the [AsClause](items/extern-crates.md#grammar-AsClause) identifier)
- External crate names referenced in a [path](paths.md)
- [Module](items/modules.md) names loaded from the filesystem without a [`path` attribute](items/modules.md#the-path-attribute)
- [`no_mangle`](abi.md#the-no_mangle-attribute) attributed items
- Item names in [external blocks](items/external-blocks.md)

[ident.normalization](#r-ident.normalization)\
Tests Tests with this rule:

- [tests/ui-fulldeps/lexer/unicode-version.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui-fulldeps/lexer/unicode-version.rs)

## Normalization

Identifiers are normalized using Normalization Form C (NFC) as defined in [Unicode Standard Annex \#15](https://www.unicode.org/reports/tr15/tr15-57.html). Two identifiers are equal if their NFC forms are equal.

[Procedural](procedural-macros.md) and [declarative](macros-by-example.md) macros receive normalized identifiers in their input.

[ident.raw](#r-ident.raw)

## Raw identifiers

[ident.raw.intro](#r-ident.raw.intro)

A raw identifier is like a normal identifier, but prefixed by `r#`. (Note that the `r#` prefix is not included as part of the actual identifier.)

[ident.raw.allowed](#r-ident.raw.allowed)\
Tests Tests with this rule:

- [tests/ui/rust-2018/async-ident.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/rust-2018/async-ident.rs)
- [tests/ui/rust-2018/dyn-keyword.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/rust-2018/dyn-keyword.rs)
- [tests/ui/rust-2018/try-ident.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/rust-2018/try-ident.rs)
- [tests/ui/rust-2018/try-macro.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/rust-2018/try-macro.rs)

Unlike a normal identifier, a raw identifier may be any strict or reserved keyword except the ones listed above for `RAW_IDENTIFIER`.

[ident.raw.reserved](#r-ident.raw.reserved)

It is an error to use the [RESERVED_RAW_IDENTIFIER](#grammar-RESERVED_RAW_IDENTIFIER) token.
