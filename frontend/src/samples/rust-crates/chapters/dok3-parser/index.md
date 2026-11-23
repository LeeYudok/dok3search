# Crate dok3-parser

## Overview

Document and source code parsers (Markdown, HTML, PDF, DOCX, Tree-sitter).

Each format-specific parser implements `dok3_domain::DocumentParserPort` and depends only on
`dok3-core` and `dok3-domain`; it never touches the index or storage crates.

## Items

- `html` — HTML parser implementing DocumentParserPort.
- `markdown` — Markdown parser implementing DocumentParserPort.
