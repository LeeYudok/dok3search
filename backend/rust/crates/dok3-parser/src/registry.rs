//! Format-aware parser registry dispatching to [`DocumentParserPort`] implementations.

use dok3_core::Dok3Error;
use dok3_domain::{DocumentParserPort, ParsedDocument};

use crate::{CodeSymbolParser, HtmlParser, MarkdownParser, PlainTextParser};

/// Registry of [`DocumentParserPort`] implementations tried in registration order.
pub struct ParserRegistry {
    parsers: Vec<Box<dyn DocumentParserPort>>,
}

impl Default for ParserRegistry {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl ParserRegistry {
    /// Creates an empty registry.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            parsers: Vec::new(),
        }
    }

    /// Creates a registry pre-populated with Markdown, HTML, CodeSymbol, and PlainText parsers.
    #[must_use]
    pub fn with_defaults() -> Self {
        let mut reg = Self::empty();
        reg.register(MarkdownParser);
        reg.register(HtmlParser);
        reg.register(CodeSymbolParser::new());
        reg.register(PlainTextParser::new());
        reg
    }

    /// Appends a parser implementation to the dispatch chain.
    pub fn register<P: DocumentParserPort + 'static>(&mut self, parser: P) {
        self.parsers.push(Box::new(parser));
    }

    /// Returns true if any registered parser accepts `(media_type, filename)`.
    #[must_use]
    pub fn supports(&self, media_type: &str, filename: &str) -> bool {
        self.parsers
            .iter()
            .any(|parser| parser.supports(media_type, filename))
    }

    /// Dispatches parsing to the first matching registered parser.
    pub fn parse_auto(
        &self,
        media_type: &str,
        filename: &str,
        content: &[u8],
    ) -> Result<ParsedDocument, Dok3Error> {
        for parser in &self.parsers {
            if parser.supports(media_type, filename) {
                return parser.parse(filename, content);
            }
        }
        Err(Dok3Error::InvalidInput(format!(
            "no parser registered for {filename} (media_type={media_type})"
        )))
    }
}
