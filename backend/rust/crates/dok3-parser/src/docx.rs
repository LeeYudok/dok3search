//! Pure-Rust OOXML (`.docx`) parser implementing [`DocumentParserPort`].

use std::io::{Cursor, Read};
use std::path::Path;

use dok3_core::Dok3Error;
use dok3_domain::{DocumentParserPort, DocumentSection, ParsedDocument, SectionId};
use zip::ZipArchive;

/// File extensions treated as Word OOXML documents.
pub const DOCX_EXTENSIONS: &[&str] = &["docx"];
const DOCX_MEDIA_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document";
const MAX_DOCUMENT_XML_BYTES: u64 = 16 * 1024 * 1024;

/// Parses `.docx` archives (`word/document.xml`) into heading-delimited Markdown sections.
#[derive(Debug, Default, Clone, Copy)]
pub struct DocxParser;

impl DocxParser {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl DocumentParserPort for DocxParser {
    fn supports(&self, media_type: &str, filename: &str) -> bool {
        let mt = media_type
            .split(';')
            .next()
            .unwrap_or("")
            .trim()
            .to_ascii_lowercase();
        if mt == DOCX_MEDIA_TYPE {
            return true;
        }
        let generic = matches!(
            mt.as_str(),
            "" | "text/plain" | "application/octet-stream" | "application/zip"
        );
        generic
            && Path::new(filename)
                .extension()
                .and_then(|e| e.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("docx"))
    }

    fn parse(&self, filename: &str, content: &[u8]) -> Result<ParsedDocument, Dok3Error> {
        let mut archive = ZipArchive::new(Cursor::new(content))
            .map_err(|err| Dok3Error::Parse(format!("{filename}: invalid docx zip: {err}")))?;
        let entry = archive.by_name("word/document.xml").map_err(|err| {
            Dok3Error::Parse(format!("{filename}: missing word/document.xml: {err}"))
        })?;
        let mut xml = String::new();
        entry
            .take(MAX_DOCUMENT_XML_BYTES)
            .read_to_string(&mut xml)
            .map_err(|err| {
                Dok3Error::Parse(format!("{filename}: failed to read document.xml: {err}"))
            })?;

        let mut warnings = Vec::new();
        if xml.contains("<w:drawing") || xml.contains("<w:pict") {
            warnings.push("embedded images/drawings were omitted during text extraction".into());
        }

        let lines = extract_ooxml_lines(&xml);
        let normalized_text = lines.join("\n");
        if normalized_text.trim().is_empty() {
            return Err(Dok3Error::InvalidInput(format!(
                "{filename}: docx contains no extractable text"
            )));
        }

        let fallback_title = Path::new(filename)
            .file_stem()
            .and_then(|s| s.to_str())
            .filter(|s| !s.is_empty())
            .unwrap_or("untitled")
            .to_string();

        let (title, sections) = build_sections_from_lines(&lines, &fallback_title);
        Ok(ParsedDocument {
            title,
            sections,
            normalized_text,
            parse_warnings: warnings,
        })
    }
}

fn extract_ooxml_lines(xml: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let mut pos = 0usize;
    while let Some(p_start) = xml[pos..].find("<w:p").map(|i| pos + i) {
        let next_char = xml.as_bytes().get(p_start + 4).copied();
        if !matches!(next_char, Some(b'>' | b' ')) {
            pos = p_start + 4;
            continue;
        }
        let Some(p_end) = xml[p_start..].find("</w:p>").map(|i| p_start + i + 6) else {
            break;
        };
        let para = &xml[p_start..p_end];
        let text = extract_w_t_text(para);
        let trimmed = text.trim();
        if !trimmed.is_empty() {
            if let Some(level) = detect_heading_level(para) {
                lines.push(format!("{} {trimmed}", "#".repeat(level)));
            } else {
                lines.push(trimmed.to_string());
            }
        }
        pos = p_end;
    }
    lines
}

fn detect_heading_level(para: &str) -> Option<usize> {
    for (pat, lvl) in [
        ("Heading1", 1),
        ("Heading2", 2),
        ("Heading3", 3),
        ("제목1", 1),
        ("제목2", 2),
        ("제목3", 3),
    ] {
        if para.contains(pat) {
            return Some(lvl);
        }
    }
    None
}

fn extract_w_t_text(fragment: &str) -> String {
    let mut out = String::new();
    let mut pos = 0usize;
    while let Some(t_start) = fragment[pos..].find("<w:t").map(|i| pos + i) {
        let Some(gt) = fragment[t_start..].find('>').map(|i| t_start + i + 1) else {
            break;
        };
        let Some(t_end) = fragment[gt..].find("</w:t>").map(|i| gt + i) else {
            break;
        };
        out.push_str(&unescape_xml(&fragment[gt..t_end]));
        pos = t_end + 6;
    }
    out
}

fn unescape_xml(raw: &str) -> String {
    raw.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}

fn build_sections_from_lines(
    lines: &[String],
    fallback_title: &str,
) -> (String, Vec<DocumentSection>) {
    let mut title = fallback_title.to_string();
    let mut headings: Vec<(usize, String)> = Vec::new();
    for (idx, line) in lines.iter().enumerate() {
        if let Some(h) = line.strip_prefix("# ") {
            if headings.is_empty() {
                title = h.trim().to_string();
            }
            headings.push((idx + 1, h.trim().to_string()));
        } else if let Some(h) = line.strip_prefix("## ") {
            headings.push((idx + 1, h.trim().to_string()));
        } else if let Some(h) = line.strip_prefix("### ") {
            headings.push((idx + 1, h.trim().to_string()));
        }
    }
    if headings.is_empty() {
        return (
            title.clone(),
            vec![DocumentSection {
                section_id: SectionId::new("body"),
                heading: title,
                anchor: "body".to_string(),
                order: 1,
                line_start: 1,
                line_end: lines.len().max(1),
                body: lines.join("\n"),
            }],
        );
    }
    let mut sections = Vec::with_capacity(headings.len());
    for (i, (start, heading)) in headings.iter().enumerate() {
        let end = headings
            .get(i + 1)
            .map_or(lines.len(), |(next_start, _)| next_start - 1);
        let body = if end > *start {
            lines[*start..end].join("\n")
        } else {
            String::new()
        };
        let anchor = format!("section-{}", i + 1);
        sections.push(DocumentSection {
            section_id: SectionId::new(anchor.clone()),
            heading: heading.clone(),
            anchor,
            order: i + 1,
            line_start: *start,
            line_end: end.max(*start),
            body,
        });
    }
    (title, sections)
}
