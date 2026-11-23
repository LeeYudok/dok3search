//! Document and source code parsers (Markdown, HTML, Code symbols, PlainText).
//!
//! Each format-specific parser implements [`dok3_domain::DocumentParserPort`] and depends only on
//! `dok3-core` and `dok3-domain`; it never touches the index or storage crates.

pub mod code;
pub mod html;
pub mod markdown;
pub mod plaintext;
pub mod registry;

pub use code::{CODE_EXTENSIONS, CodeSymbolParser};
pub use html::{HTML_EXTENSIONS, HtmlParser};
pub use markdown::MarkdownParser;
pub use plaintext::{PLAINTEXT_EXTENSIONS, PlainTextParser};
pub use registry::ParserRegistry;
