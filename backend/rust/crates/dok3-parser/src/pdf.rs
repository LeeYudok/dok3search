//! Pure-Rust PDF parser implementing [`DocumentParserPort`] via `lopdf`.

use std::path::Path;

use dok3_core::Dok3Error;
use dok3_domain::{DocumentParserPort, DocumentSection, ParsedDocument, SectionId};
use lopdf::Document as PdfDocument;

/// File extensions treated as PDF documents.
pub const PDF_EXTENSIONS: &[&str] = &["pdf"];

/// Extracts per-page text from PDF files and records layout/image loss warnings.
#[derive(Debug, Default, Clone, Copy)]
pub struct PdfParser;

impl PdfParser {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl DocumentParserPort for PdfParser {
    fn supports(&self, media_type: &str, filename: &str) -> bool {
        let mt = media_type
            .split(';')
            .next()
            .unwrap_or("")
            .trim()
            .to_ascii_lowercase();
        if mt == "application/pdf" {
            return true;
        }
        let generic = matches!(mt.as_str(), "" | "text/plain" | "application/octet-stream");
        generic
            && Path::new(filename)
                .extension()
                .and_then(|e| e.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("pdf"))
    }

    fn parse(&self, filename: &str, content: &[u8]) -> Result<ParsedDocument, Dok3Error> {
        let doc = PdfDocument::load_mem(content)
            .map_err(|err| Dok3Error::Parse(format!("{filename}: invalid pdf: {err}")))?;
        let pages = doc.get_pages();
        if pages.is_empty() {
            return Err(Dok3Error::InvalidInput(format!(
                "{filename}: pdf has no pages"
            )));
        }

        let mut warnings =
            vec!["PDF layout, tables, and embedded images are flattened to plain text".to_string()];
        let mut sections = Vec::new();
        let mut all_lines = Vec::new();
        let mut empty_pages = 0usize;

        for (&page_num, _) in &pages {
            let text = doc.extract_text(&[page_num]).unwrap_or_default();
            let trimmed = text.trim();
            if trimmed.is_empty() {
                empty_pages += 1;
                continue;
            }
            let heading = format!("Page {page_num}");
            let start = all_lines.len() + 1;
            all_lines.push(format!("## {heading}"));
            for line in trimmed.lines() {
                all_lines.push(line.trim_end().to_string());
            }
            let end = all_lines.len();
            let anchor = format!("page-{page_num}");
            sections.push(DocumentSection {
                section_id: SectionId::new(anchor.clone()),
                heading,
                anchor,
                order: sections.len() + 1,
                line_start: start,
                line_end: end,
                body: trimmed.to_string(),
            });
        }

        if empty_pages > 0 {
            warnings.push(format!(
                "{empty_pages} page(s) yielded no extractable text (possibly scanned images)"
            ));
        }
        if sections.is_empty() {
            return Err(Dok3Error::InvalidInput(format!(
                "{filename}: pdf contains no extractable text"
            )));
        }

        let title = Path::new(filename)
            .file_stem()
            .and_then(|s| s.to_str())
            .filter(|s| !s.is_empty())
            .unwrap_or("untitled")
            .to_string();

        Ok(ParsedDocument {
            title,
            sections,
            normalized_text: all_lines.join("\n"),
            parse_warnings: warnings,
        })
    }
}
