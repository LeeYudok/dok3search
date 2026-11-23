# Module dok3-parser::html

## Overview

HTML parser implementing `DocumentParserPort`.

The parser normalizes HTML into the same Markdown-shaped contract as the Markdown parser:
heading-delimited sections over a normalized, line-numbered text payload. It keeps semantic
prose, links, inline code and fenced code blocks, and intentionally removes executable or
styling-only HTML. Structural information that cannot survive that normalization is recorded
as a loss warning rather than being silently discarded.

## Items

- `HtmlParser` — Parses HTML documents into heading-delimited sections.
- `HTML_EXTENSIONS` — File extensions treated as HTML (compared case-insensitively).

### constant dok3-parser::html::HTML_EXTENSIONS

```rust
pub const HTML_EXTENSIONS: [&str; 2];
```

File extensions treated as HTML (compared case-insensitively).

### struct dok3-parser::html::HtmlParser

```rust
pub struct HtmlParser;
```

Parses HTML documents into heading-delimited sections.

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
