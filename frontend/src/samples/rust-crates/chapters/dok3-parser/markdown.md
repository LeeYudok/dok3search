# Module dok3-parser::markdown

## Overview

Markdown parser implementing `DocumentParserPort`.

The parser splits a Markdown document into flat, non-overlapping sections at H1/H2/H3 headings
and keeps every other line (links, code blocks, lists, tables, deeper headings) verbatim in
the section body. It is a line scanner, not a full CommonMark implementation, so the supported
syntax is deliberately small and fully listed here.

**Input handling**

- Content that is not valid UTF-8 is rejected with `Dok3Error::Parse`. Decoding lossily
would silently corrupt the indexed text, so the caller decides how to re-encode or skip the
file (for example EUC-KR documents).

- A document that is empty or contains only whitespace (a lone UTF-8 BOM counts as empty) is
rejected with `Dok3Error::InvalidInput`; there is nothing to index.

- Everything else parses. Recoverable oddities are reported in
`ParsedDocument::parse_warnings` (an unclosed code fence, an empty heading).

- A leading UTF-8 BOM is dropped. Lines end with `\n`, `\r\n` or a lone `\r`.

**Lines**

Line numbers are 1-based and count lines of `ParsedDocument::normalized_text`, which is the
input with the BOM removed and every line ending replaced by `\n`, including a final one, so
`normalized_text.lines()` yields exactly the lines the line numbers refer to (blank lines
included).

**Headings**

- Only ATX headings (`# Title`) start a section, and only levels 1 to 3. Setext headings
(`Title` underlined with `===` or `---`) are not recognized.

- A heading may be indented by up to three spaces. Four or more spaces, or any tab in the
leading whitespace, make the line indented code. Lines inside block quotes or list items
(`> # x`, `- # x`) are not headings either.

- `#` must be followed by a space, a tab or the end of the line (`#hashtag` is text). A
closing sequence preceded by whitespace is removed (`## Title ##` is `Title`, `## C#` stays
`C#`). Levels 4 to 6 are recognized as headings but do not start a section, so they stay in
the body of the section they appear in. Seven or more `#` is plain text.

- A heading without text is body text and produces a warning.

- `heading` is the heading text with inline markup preserved verbatim.

**Fenced code blocks**

- A fence opens with three or more backticks or tildes, indented by up to three spaces. For
backtick fences the info string must not contain a backtick.

- It closes with a line of the same character,

...

## Items

- `MarkdownParser` — Parses Markdown documents into heading-delimited sections.

### struct dok3-parser::markdown::MarkdownParser

```rust
pub struct MarkdownParser;
```

Parses Markdown documents into heading-delimited sections.

See the module documentation for the exact syntax rules.

**Methods**

- `pub const fn new() -> Self`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn supports(&self, media_type: &str, filename: &str) -> bool`
- `fn parse( &self, filename: &str, content: &[u8], ) -> Result<ParsedDocument, Dok3Error>`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
