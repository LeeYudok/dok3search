//! Document and source code parsers (Markdown, HTML, PDF, DOCX, Tree-sitter).
//!
//! Each format-specific parser implements [`dok3_domain::DocumentParserPort`] and depends only on
//! `dok3-core` and `dok3-domain`; it never touches the index or storage crates.

pub mod markdown;

pub mod html;

pub use html::{HTML_EXTENSIONS, HtmlParser};
pub use markdown::MarkdownParser;
