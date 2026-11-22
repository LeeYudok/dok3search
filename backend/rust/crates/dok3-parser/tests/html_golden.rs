//! Golden and behavior tests for [`HtmlParser`].
//!
//! Inputs live in `fixtures/docs/html/<name>.html` at the repository root and the expected parse
//! result in `fixtures/docs/html/<name>.golden`. After an intended behavior change, regenerate the
//! goldens with `UPDATE_GOLDEN=1 cargo test -p dok3-parser --test html_golden` and review the diff
//! by hand before committing.

use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use dok3_core::Dok3Error;
use dok3_domain::{DocumentParserPort, DocumentSection, ParsedDocument};
use dok3_parser::{HTML_EXTENSIONS, HtmlParser};

/// Fixtures that parse successfully and have a `.golden` file.
const GOLDEN_FIXTURES: [&str; 2] = ["code-and-links", "nested-headings"];

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../fixtures/docs/html")
}

fn read_fixture(name: &str) -> Vec<u8> {
    let path = fixtures_dir().join(format!("{name}.html"));
    fs::read(&path).unwrap_or_else(|err| panic!("cannot read {}: {err}", path.display()))
}

fn parse_fixture(name: &str) -> ParsedDocument {
    HtmlParser::new()
        .parse(&format!("{name}.html"), &read_fixture(name))
        .unwrap_or_else(|err| panic!("fixture {name} should parse: {err}"))
}

fn parse_text(filename: &str, text: &str) -> ParsedDocument {
    HtmlParser::new()
        .parse(filename, text.as_bytes())
        .unwrap_or_else(|err| panic!("{filename} should parse: {err}"))
}

/// Renders a parse result as stable text. Body lines are prefixed with `|` so blank lines and
/// lines that look like headings stay unambiguous.
fn render(doc: &ParsedDocument) -> String {
    let mut out = String::new();
    writeln!(out, "title: {}", doc.title).unwrap();
    writeln!(out, "warnings: {}", doc.parse_warnings.len()).unwrap();
    for warning in &doc.parse_warnings {
        writeln!(out, "  ! {warning}").unwrap();
    }
    writeln!(out, "sections: {}", doc.sections.len()).unwrap();
    for section in &doc.sections {
        writeln!(
            out,
            "\n[{}] id={} anchor={} lines={}-{}",
            section.order, section.section_id, section.anchor, section.line_start, section.line_end
        )
        .unwrap();
        writeln!(out, "heading: {}", section.heading).unwrap();
        for line in section.body.lines() {
            if line.is_empty() {
                out.push_str("|\n");
            } else {
                writeln!(out, "| {line}").unwrap();
            }
        }
    }
    out
}

fn assert_golden(name: &str) {
    let actual = render(&parse_fixture(name));
    let path = fixtures_dir().join(format!("{name}.golden"));
    if std::env::var_os("UPDATE_GOLDEN").is_some() {
        fs::write(&path, &actual).unwrap();
        return;
    }
    let expected = fs::read_to_string(&path).unwrap_or_else(|err| {
        panic!(
            "cannot read {}: {err} (run with UPDATE_GOLDEN=1 to create it)",
            path.display()
        )
    });
    assert_eq!(
        actual, expected,
        "golden mismatch for {name}; rerun with UPDATE_GOLDEN=1 and review the diff"
    );
}

fn anchors(doc: &ParsedDocument) -> Vec<&str> {
    doc.sections
        .iter()
        .map(|section| section.anchor.as_str())
        .collect()
}

fn headings(doc: &ParsedDocument) -> Vec<&str> {
    doc.sections
        .iter()
        .map(|section| section.heading.as_str())
        .collect()
}

fn section<'a>(doc: &'a ParsedDocument, anchor: &str) -> &'a DocumentSection {
    doc.sections
        .iter()
        .find(|section| section.anchor == anchor)
        .unwrap_or_else(|| panic!("no section with anchor {anchor}: {:?}", anchors(doc)))
}

#[test]
fn golden_code_and_links() {
    assert_golden("code-and-links");
}

#[test]
fn golden_nested_headings() {
    assert_golden("nested-headings");
}

#[test]
fn every_fixture_has_a_golden_entry() {
    for name in GOLDEN_FIXTURES {
        assert!(fixtures_dir().join(format!("{name}.golden")).exists());
    }
}

#[test]
fn sections_tile_the_document() {
    for name in GOLDEN_FIXTURES {
        let doc = parse_fixture(name);
        assert_eq!(doc.sections.iter().map(|s| s.order).sum::<usize>(), {
            let n = doc.sections.len();
            n * (n + 1) / 2
        });
        assert_eq!(doc.sections.first().unwrap().line_start, 1);
        assert_eq!(
            doc.sections.last().unwrap().line_end,
            doc.normalized_text.lines().count()
        );
        for pair in doc.sections.windows(2) {
            assert_eq!(pair[1].line_start, pair[0].line_end + 1);
        }
    }
}

#[test]
fn section_bodies_are_slices_of_the_normalized_text() {
    let doc = parse_fixture("code-and-links");
    let normalized = &doc.normalized_text;
    for section in &doc.sections {
        let lines: Vec<&str> = normalized.lines().collect();
        let leading_section = section.anchor == "preamble" || section.anchor == "body";
        let from = if leading_section {
            section.line_start - 1
        } else {
            assert!(
                lines[section.line_start - 1].trim_start().starts_with('#'),
                "{} does not start on a heading line",
                section.anchor
            );
            section.line_start
        };
        let segment = lines[from..section.line_end].join("\n");
        let start = segment
            .lines()
            .position(|line| !line.trim().is_empty())
            .unwrap_or(segment.len());
        let end = segment
            .lines()
            .collect::<Vec<_>>()
            .iter()
            .rposition(|line| !line.trim().is_empty())
            .map_or(start, |position| position + 1);
        assert_eq!(
            segment.lines().collect::<Vec<_>>()[start..end].join("\n"),
            section.body
        );
    }
}

#[test]
fn links_code_and_headings_survive() {
    let doc = parse_fixture("code-and-links");
    assert_eq!(doc.title, "HTML Guide");
    assert_eq!(
        headings(&doc),
        vec!["HTML Guide", "Lists", "Structured Data"]
    );
    let intro = section(&doc, "html-guide");
    assert!(
        intro
            .body
            .contains("[Getting started](https://example.test/start)")
    );
    assert!(intro.body.contains("`cargo check`"));
    assert!(doc.normalized_text.contains("```rust"));
    assert!(
        doc.normalized_text
            .contains("println!(\"offline search\");")
    );
}

#[test]
fn scripts_styles_and_losses_are_recorded() {
    let doc = parse_fixture("code-and-links");
    for expected in [
        "discarded 1 HTML comment(s)",
        "discarded 1 script element(s)",
        "discarded 1 style element(s)",
        "table cell text was retained, but table formatting was not preserved",
    ] {
        assert!(
            doc.parse_warnings.iter().any(|warning| warning == expected),
            "missing warning {expected}; got {:?}",
            doc.parse_warnings
        );
    }
    assert!(!doc.normalized_text.contains("window.discarded"));
    assert!(!doc.normalized_text.contains("color: red"));
    assert!(doc.normalized_text.contains("Presentation only"));
}

#[test]
fn repeated_headings_get_unique_anchors() {
    let doc = parse_fixture("nested-headings");
    assert_eq!(
        anchors(&doc),
        vec!["architecture", "rust", "ownership", "rust-1"]
    );
    assert_eq!(
        section(&doc, "rust-1").body,
        "A repeated heading receives a unique anchor.\n\n#### Deep Heading\n\nLevel four remains in the containing section."
    );
}

#[test]
fn title_falls_back_to_title_element_then_file_stem() {
    assert_eq!(parse_fixture("code-and-links").title, "HTML Guide");
    assert_eq!(
        parse_text(
            "fallback.html",
            "<html><head><title>Fallback</title></head><body><p>Text</p></body></html>"
        )
        .title,
        "Fallback"
    );
    assert_eq!(parse_text("notes.htm", "<p>Text</p>").title, "notes");
    assert_eq!(parse_text("", "<p>Text</p>").title, "untitled");
}

#[test]
fn empty_or_contentless_documents_are_rejected() {
    let parser = HtmlParser::new();
    let empty = parser.parse("empty.html", b"").unwrap_err();
    assert!(matches!(empty, Dok3Error::InvalidInput(_)), "{empty:?}");
    let empty = parser.parse("empty.html", b"   \n\t").unwrap_err();
    assert!(matches!(empty, Dok3Error::InvalidInput(_)), "{empty:?}");
    let contentless = parser
        .parse(
            "empty.html",
            b"<html><head><title>Only title</title></head></html>",
        )
        .unwrap_err();
    assert!(
        matches!(contentless, Dok3Error::Parse(_)),
        "{contentless:?}"
    );
    let unnamed = parser.parse("", b"").unwrap_err();
    assert!(matches!(unnamed, Dok3Error::InvalidInput(_)), "{unnamed:?}");
}

#[test]
fn non_utf8_input_is_a_parse_error_with_the_offending_offset() {
    let err = HtmlParser::new()
        .parse("legacy.html", &[104, 105, 255])
        .unwrap_err();
    assert!(matches!(err, Dok3Error::Parse(_)), "{err:?}");
    assert_eq!(err.code(), "PARSE_ERROR");
}

#[test]
fn supports_html_media_types_and_extensions() {
    let parser = HtmlParser::new();
    assert_eq!(HTML_EXTENSIONS, ["html", "htm"]);
    assert!(parser.supports("text/html", "anything.bin"));
    assert!(parser.supports("application/xhtml+xml", ""));
    assert!(parser.supports("Text/HTML; charset=utf-8", "a.txt"));
    assert!(parser.supports("", "guide.html"));
    assert!(parser.supports("text/plain", "dir/Guide.HTM"));
    assert!(parser.supports("application/octet-stream", "a.html"));
    assert!(!parser.supports("", "guide.txt"));
    assert!(!parser.supports("", "guide"));
    assert!(!parser.supports("", ".html"));
    assert!(!parser.supports("text/markdown", "guide.html"));
    assert!(!parser.supports("application/pdf", "guide.html"));
}

#[test]
fn parse_is_deterministic() {
    let first = parse_fixture("nested-headings");
    let second = parse_fixture("nested-headings");
    assert_eq!(first, second);
}

#[test]
fn usable_as_a_shared_trait_object() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<HtmlParser>();

    let parser: Box<dyn DocumentParserPort> = Box::new(HtmlParser::new());
    assert!(parser.supports("text/html", "a.html"));
    let doc = parser.parse("a.html", b"<h1>A</h1>\n<p>text</p>").unwrap();
    assert_eq!(doc.title, "A");
}
