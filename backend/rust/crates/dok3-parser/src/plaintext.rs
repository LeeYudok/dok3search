//! Plain-text and structured configuration parser implementing [`DocumentParserPort`].

use std::path::Path;

use dok3_core::Dok3Error;
use dok3_domain::{DocumentParserPort, DocumentSection, ParsedDocument, SectionId};

use crate::code::normalize_lines;

/// Supported plain-text and configuration extensions handled by [`PlainTextParser`].
pub const PLAINTEXT_EXTENSIONS: &[&str] =
    &["txt", "log", "json", "yaml", "yml", "toml", "xml", "csv"];

/// Fallback parser for plain text, logs, and structured configuration files.
#[derive(Debug, Default, Clone, Copy)]
pub struct PlainTextParser;

impl PlainTextParser {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl DocumentParserPort for PlainTextParser {
    fn supports(&self, media_type: &str, filename: &str) -> bool {
        let mt = media_type
            .split(';')
            .next()
            .unwrap_or("")
            .trim()
            .to_ascii_lowercase();
        if matches!(
            mt.as_str(),
            "text/plain" | "application/json" | "application/yaml" | "application/xml" | "text/csv"
        ) {
            return true;
        }
        Path::new(filename)
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|ext| PLAINTEXT_EXTENSIONS.contains(&ext.to_ascii_lowercase().as_str()))
    }

    fn parse(&self, filename: &str, content: &[u8]) -> Result<ParsedDocument, Dok3Error> {
        let text = std::str::from_utf8(content)
            .map_err(|err| Dok3Error::Parse(format!("invalid UTF-8 in {filename}: {err}")))?;
        let normalized = normalize_lines(text);
        if normalized.trim().is_empty() {
            return Err(Dok3Error::InvalidInput(format!(
                "empty text document: {filename}"
            )));
        }

        let title = Path::new(filename)
            .file_name()
            .and_then(|n| n.to_str())
            .filter(|s| !s.is_empty())
            .unwrap_or(filename)
            .to_string();
        let line_end = normalized.lines().count().max(1);

        Ok(ParsedDocument {
            title: title.clone(),
            sections: vec![DocumentSection {
                section_id: SectionId::new("body"),
                heading: title,
                anchor: "body".to_string(),
                order: 1,
                line_start: 1,
                line_end,
                body: normalized.trim().to_string(),
            }],
            normalized_text: normalized,
            parse_warnings: Vec::new(),
        })
    }
}
