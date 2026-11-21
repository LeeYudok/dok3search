//! Golden and behavior tests for [`MarkdownParser`].
//!
//! Inputs live in `fixtures/docs/md/<name>.md` at the repository root and the expected parse
//! result in `fixtures/docs/md/<name>.golden`. After an intended behavior change, regenerate the
//! goldens with `UPDATE_GOLDEN=1 cargo test -p dok3-parser --test markdown_golden` and review the
//! diff by hand before committing.

use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use dok3_core::Dok3Error;
use dok3_domain::{DocumentParserPort, DocumentSection, ParsedDocument};
use dok3_parser::MarkdownParser;

/// Fixtures that parse successfully and have a `.golden` file.
const GOLDEN_FIXTURES: [&str; 7] = [
    "korean-guide",
    "nested-headings",
    "code-fences",
    "links",
    "unclosed-fence",
    "no-headings",
    "preamble",
];

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../fixtures/docs/md")
}

fn read_fixture(name: &str) -> Vec<u8> {
    let path = fixtures_dir().join(format!("{name}.md"));
    fs::read(&path).unwrap_or_else(|err| panic!("cannot read {}: {err}", path.display()))
}

fn parse_fixture(name: &str) -> ParsedDocument {
    MarkdownParser::new()
        .parse(&format!("{name}.md"), &read_fixture(name))
        .unwrap_or_else(|err| panic!("fixture {name} should parse: {err}"))
}

fn parse_text(filename: &str, text: &str) -> ParsedDocument {
    MarkdownParser::new()
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
    doc.sections.iter().map(|s| s.anchor.as_str()).collect()
}

fn headings(doc: &ParsedDocument) -> Vec<&str> {
    doc.sections.iter().map(|s| s.heading.as_str()).collect()
}

fn ranges(doc: &ParsedDocument) -> Vec<(usize, usize)> {
    doc.sections
        .iter()
        .map(|s| (s.line_start, s.line_end))
        .collect()
}

fn section<'a>(doc: &'a ParsedDocument, anchor: &str) -> &'a DocumentSection {
    doc.sections
        .iter()
        .find(|s| s.anchor == anchor)
        .unwrap_or_else(|| panic!("no section with anchor {anchor}: {:?}", anchors(doc)))
}

#[test]
fn golden_korean_guide() {
    assert_golden("korean-guide");
}

#[test]
fn golden_nested_headings() {
    assert_golden("nested-headings");
}

#[test]
fn golden_code_fences() {
    assert_golden("code-fences");
}

#[test]
fn golden_links() {
    assert_golden("links");
}

#[test]
fn golden_unclosed_fence() {
    assert_golden("unclosed-fence");
}

#[test]
fn golden_no_headings() {
    assert_golden("no-headings");
}

#[test]
fn golden_preamble() {
    assert_golden("preamble");
}

#[test]
fn every_fixture_has_a_golden_entry() {
    let mut on_disk: Vec<String> = fs::read_dir(fixtures_dir())
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .filter_map(|name| name.strip_suffix(".md").map(str::to_string))
        .collect();
    on_disk.sort();

    let mut expected: Vec<String> = GOLDEN_FIXTURES.iter().map(|s| (*s).to_string()).collect();
    expected.push("empty".to_string());
    expected.sort();
    assert_eq!(on_disk, expected);
}

#[test]
fn sections_tile_the_document_for_every_fixture() {
    for name in GOLDEN_FIXTURES {
        let doc = parse_fixture(name);
        let total = doc.normalized_text.lines().count();
        assert!(!doc.sections.is_empty(), "{name}: no sections");

        for (i, s) in doc.sections.iter().enumerate() {
            assert_eq!(s.order, i + 1, "{name}: order of {}", s.anchor);
            assert_eq!(s.section_id.as_str(), s.anchor, "{name}: id vs anchor");
            assert!(s.line_start >= 1, "{name}: lines are 1-based");
            assert!(s.line_end >= s.line_start, "{name}: {} is empty", s.anchor);
            assert!(s.line_end <= total, "{name}: {} past the end", s.anchor);
            if let Some(next) = doc.sections.get(i + 1) {
                assert_eq!(
                    next.line_start,
                    s.line_end + 1,
                    "{name}: gap after {}",
                    s.anchor
                );
            }
        }
        assert_eq!(doc.sections.last().unwrap().line_end, total, "{name}: last");

        let mut sorted = anchors(&doc);
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(
            sorted.len(),
            doc.sections.len(),
            "{name}: duplicate anchors"
        );
    }
}

#[test]
fn section_bodies_are_verbatim_slices_of_the_normalized_text() {
    for name in GOLDEN_FIXTURES {
        let doc = parse_fixture(name);
        let lines: Vec<&str> = doc.normalized_text.lines().collect();
        for s in &doc.sections {
            // The heading line starts the range, except for the unnamed leading section.
            let from = if s.anchor == "preamble" || s.anchor == "body" {
                s.line_start - 1
            } else {
                assert!(
                    lines[s.line_start - 1].trim_start().starts_with('#'),
                    "{name}: {} does not start on a heading line",
                    s.anchor
                );
                s.line_start
            };
            let slice = lines[from..s.line_end].join("\n");
            assert_eq!(slice.trim_matches('\n'), s.body.trim_matches('\n'));
        }
    }
}

#[test]
fn korean_headings_keep_hangul_in_anchors() {
    let doc = parse_fixture("korean-guide");
    assert_eq!(doc.title, "소유권 가이드");
    assert_eq!(
        headings(&doc),
        [
            "소유권 가이드",
            "소유권이란?",
            "이동(move)",
            "복사와 클론",
            "대여와 참조",
            "가변 참조",
            "맺음말 ✅",
        ]
    );
    assert_eq!(
        anchors(&doc),
        [
            "소유권-가이드",
            "소유권이란",
            "이동move",
            "복사와-클론",
            "대여와-참조",
            "가변-참조",
            "맺음말",
        ]
    );
    assert_eq!(
        ranges(&doc),
        [
            (1, 4),
            (5, 8),
            (9, 15),
            (16, 19),
            (20, 23),
            (24, 27),
            (28, 30)
        ]
    );
    assert!(doc.parse_warnings.is_empty());
}

#[test]
fn only_h1_to_h3_start_sections_and_deeper_levels_stay_in_the_body() {
    let doc = parse_fixture("nested-headings");
    assert_eq!(doc.title, "Handbook");
    assert_eq!(
        headings(&doc),
        [
            "Handbook",
            "Setup",
            "Install",
            "Configure",
            "Usage",
            "Install",
            "Appendix",
            "Skipped level",
            "Setup",
            "Two-space indent is still a heading",
        ]
    );

    let configure = section(&doc, "configure");
    assert!(configure.body.contains("#### Advanced options"));
    assert!(
        configure
            .body
            .contains("Level four stays inside Configure.")
    );

    let setup_again = section(&doc, "setup-1");
    assert!(setup_again.body.contains("#hashtag is not a heading."));
    assert!(
        setup_again
            .body
            .contains("####### seven hashes are plain text.")
    );
    assert!(
        setup_again
            .body
            .contains("    # four spaces is indented code")
    );

    let last = doc.sections.last().unwrap();
    assert!(last.body.starts_with("##\n"));
    assert_eq!(
        doc.parse_warnings,
        ["empty heading at line 45 ignored".to_string()]
    );
}

#[test]
fn repeated_headings_get_numbered_anchors_in_document_order() {
    let doc = parse_fixture("nested-headings");
    assert_eq!(
        anchors(&doc),
        [
            "handbook",
            "setup",
            "install",
            "configure",
            "usage",
            "install-1",
            "appendix",
            "skipped-level",
            "setup-1",
            "two-space-indent-is-still-a-heading",
        ]
    );
}

#[test]
fn anchor_suffixes_never_collide_with_existing_anchors() {
    let doc = parse_text("a.md", "# Foo\n\n# Foo\n\n# Foo 1\n\n# Foo\n");
    assert_eq!(anchors(&doc), ["foo", "foo-1", "foo-1-1", "foo-2"]);
}

#[test]
fn headings_without_slug_characters_get_the_section_anchor() {
    let doc = parse_text("a.md", "# ???\n\n# ✅\n");
    assert_eq!(anchors(&doc), ["section", "section-1"]);
    assert_eq!(headings(&doc), ["???", "✅"]);
}

#[test]
fn hash_lines_inside_code_fences_are_not_headings() {
    let doc = parse_fixture("code-fences");
    assert_eq!(doc.title, "Code fences");
    assert_eq!(
        headings(&doc),
        [
            "Code fences",
            "After backticks",
            "Longer fence",
            "Mixed markers",
            "Indented fence",
            "Inline triple backticks",
            "End",
        ]
    );
    assert!(doc.parse_warnings.is_empty());

    let intro = section(&doc, "code-fences");
    assert!(
        intro
            .body
            .contains("```sh\n# not a heading\n## still not a heading\necho done\n```")
    );
    assert_eq!((intro.line_start, intro.line_end), (1, 10));

    let tilde = section(&doc, "after-backticks");
    assert!(
        tilde
            .body
            .contains("~~~python\n# comment, not a heading\ndef f(): pass\n~~~")
    );

    let longer = section(&doc, "longer-fence");
    assert!(
        longer
            .body
            .starts_with("````markdown\n# Fake title inside a four-backtick fence\n```\n")
    );
    assert!(longer.body.contains("## Fake section\n````"));

    let mixed = section(&doc, "mixed-markers");
    assert!(mixed.body.contains("~~~\n# still inside the code\n```"));

    let indented = section(&doc, "indented-fence");
    assert!(
        indented
            .body
            .contains("  ```text\n  # two spaces of indentation are allowed\n  ```")
    );
    assert!(
        indented
            .body
            .contains("    # indented code block, so this is not a fence")
    );
}

#[test]
fn unclosed_fence_swallows_the_rest_and_warns() {
    let doc = parse_fixture("unclosed-fence");
    assert_eq!(headings(&doc), ["Unclosed fence", "Before", "Broken"]);
    assert_eq!(
        doc.parse_warnings,
        [
            "unclosed code fence opened at line 9; the rest of the document is treated as code"
                .to_string()
        ]
    );
    let broken = section(&doc, "broken");
    assert!(broken.body.contains("## not a heading"));
    assert!(broken.body.contains("## also not a heading"));
    assert_eq!(broken.line_end, doc.normalized_text.lines().count());
}

#[test]
fn fence_closing_on_the_last_line_without_newline() {
    let doc = parse_text("a.md", "# A\n```\n# x\n```");
    assert!(doc.parse_warnings.is_empty());
    assert_eq!(headings(&doc), ["A"]);
    assert_eq!(ranges(&doc), [(1, 4)]);
}

#[test]
fn tab_and_deep_indentation_prevent_headings_and_fences() {
    let doc = parse_text(
        "a.md",
        "# Top\n\t# tabbed\n    # four spaces\n    ```\n# real\n    ```\n",
    );
    assert_eq!(headings(&doc), ["Top", "real"]);
}

#[test]
fn links_and_code_survive_in_section_bodies() {
    let doc = parse_fixture("links");
    assert_eq!(doc.title, "Links and images");
    assert_eq!(
        headings(&doc),
        [
            "Links and images",
            "See [the docs](https://example.com/docs) now",
            "Image ![logo](logo.png) heading",
        ]
    );
    assert_eq!(
        anchors(&doc),
        ["links-and-images", "see-the-docs-now", "image-logo-heading"]
    );

    let top = section(&doc, "links-and-images");
    assert!(
        top.body
            .contains("[link](https://example.com/path_(with)_parens)")
    );
    assert!(top.body.contains("<https://example.com/auto>"));
    assert!(top.body.contains("[Rust][rust-lang]"));
    assert!(
        top.body
            .contains("![Diagram](images/diagram.png \"Title\")")
    );
    assert!(top.body.contains("[rust-lang]: https://www.rust-lang.org"));

    let docs = section(&doc, "see-the-docs-now");
    assert!(docs.body.contains("`let x = 1;`"));
    assert!(docs.body.contains("[fragment link](#see-the-docs-now)"));

    let table = section(&doc, "image-logo-heading");
    assert!(table.body.contains("| a    | [a](https://a.example) |"));
}

#[test]
fn text_before_the_first_heading_becomes_a_preamble_section() {
    let doc = parse_fixture("preamble");
    assert_eq!(doc.title, "preamble");
    assert_eq!(headings(&doc), ["preamble", "First"]);
    assert_eq!(anchors(&doc), ["preamble", "first"]);
    // The preamble starts on its first non-blank line (line 3) and runs up to the heading.
    assert_eq!(ranges(&doc), [(3, 4), (5, 9)]);
    assert_eq!(
        doc.sections[0].body,
        "Text before any heading, with a [link](https://example.com)."
    );
    assert_eq!(doc.sections[1].body, "Body.");
}

#[test]
fn document_without_headings_is_a_single_body_section() {
    let doc = parse_fixture("no-headings");
    assert_eq!(doc.title, "no-headings");
    assert_eq!(doc.sections.len(), 1);
    let only = &doc.sections[0];
    assert_eq!(only.anchor, "body");
    assert_eq!(only.section_id.as_str(), "body");
    assert_eq!((only.line_start, only.line_end), (1, 3));
    assert!(
        only.body
            .contains("[the manual](https://example.com/manual)")
    );
    assert!(only.body.contains("`inline code`"));
}

#[test]
fn blank_lines_before_the_first_heading_do_not_create_a_section() {
    let doc = parse_text("a.md", "\n\n# A\n\ntext\n");
    assert_eq!(headings(&doc), ["A"]);
    assert_eq!(ranges(&doc), [(3, 5)]);
}

#[test]
fn trailing_blank_lines_belong_to_the_last_section_range_but_not_its_body() {
    let doc = parse_text("a.md", "# A\n\ntext\n\n\n");
    assert_eq!(ranges(&doc), [(1, 5)]);
    assert_eq!(doc.sections[0].body, "text");
}

#[test]
fn title_prefers_the_first_h1_and_falls_back_to_the_file_stem() {
    assert_eq!(
        parse_text("x.md", "## A\n\n# Real\n\n# Later\n").title,
        "Real"
    );
    assert_eq!(parse_text("docs/guide.v2.md", "## A\n").title, "guide.v2");
    assert_eq!(
        parse_text("C:\\docs\\notes.markdown", "## A\n").title,
        "notes"
    );
    assert_eq!(parse_text("README", "text\n").title, "README");
    assert_eq!(parse_text(".md", "text\n").title, ".md");
    assert_eq!(parse_text("", "text\n").title, "untitled");
    assert_eq!(parse_text("dir/", "text\n").title, "untitled");
}

#[test]
fn heading_text_is_kept_verbatim_apart_from_closing_hashes() {
    let doc = parse_text("a.md", "# `Vec<T>` and **bold** ##\n\n## C#\n");
    assert_eq!(headings(&doc), ["`Vec<T>` and **bold**", "C#"]);
    assert_eq!(anchors(&doc), ["vect-and-bold", "c"]);
}

#[test]
fn empty_document_is_rejected_as_invalid_input() {
    let err = MarkdownParser::new()
        .parse("empty.md", &read_fixture("empty"))
        .unwrap_err();
    assert_eq!(
        err,
        Dok3Error::InvalidInput("empty.md: document is empty".into())
    );
    assert_eq!(err.code(), "INVALID_INPUT");
}

#[test]
fn whitespace_only_and_bom_only_documents_are_empty() {
    for content in [
        &b""[..],
        b"\n",
        b"  \r\n\t\r\n",
        b"\xEF\xBB\xBF",
        b"\xEF\xBB\xBF\n  \n",
    ] {
        let err = MarkdownParser::new()
            .parse("blank.md", content)
            .unwrap_err();
        assert!(
            matches!(err, Dok3Error::InvalidInput(_)),
            "{content:?} should be empty, got {err:?}"
        );
    }
}

#[test]
fn unnamed_input_is_still_reported_with_a_label() {
    let err = MarkdownParser::new().parse("", b"").unwrap_err();
    assert_eq!(
        err,
        Dok3Error::InvalidInput("<unnamed>: document is empty".into())
    );
}

#[test]
fn non_utf8_input_is_a_parse_error_with_the_offending_offset() {
    // "안녕" encoded as EUC-KR: a typical legacy Korean document.
    let euc_kr = [b'#', b' ', 0xBE, 0xC8, 0xB3, 0xE7, b'\n'];
    let err = MarkdownParser::new()
        .parse("legacy.md", &euc_kr)
        .unwrap_err();
    assert_eq!(
        err,
        Dok3Error::Parse(
            "legacy.md: content is not valid UTF-8 (first invalid byte at offset 2)".into()
        )
    );
    assert_eq!(err.code(), "PARSE_ERROR");

    // A valid document that is cut in the middle of a multi-byte character.
    let mut truncated = "# 제목\n\n본문".as_bytes().to_vec();
    truncated.pop();
    let err = MarkdownParser::new()
        .parse("cut.md", &truncated)
        .unwrap_err();
    assert!(matches!(err, Dok3Error::Parse(_)), "{err:?}");

    // A stray 0xFF in the middle of otherwise valid text.
    let mut stray = b"# Title\n\nbefore ".to_vec();
    stray.push(0xFF);
    stray.extend_from_slice(b" after\n");
    let err = MarkdownParser::new().parse("stray.md", &stray).unwrap_err();
    assert_eq!(
        err,
        Dok3Error::Parse(
            "stray.md: content is not valid UTF-8 (first invalid byte at offset 16)".into()
        )
    );
}

#[test]
fn utf8_bom_is_dropped_without_shifting_lines() {
    let plain = parse_fixture("korean-guide");
    let mut with_bom = vec![0xEF, 0xBB, 0xBF];
    with_bom.extend_from_slice(&read_fixture("korean-guide"));
    let bom = MarkdownParser::new()
        .parse("korean-guide.md", &with_bom)
        .unwrap();
    assert_eq!(bom, plain);
    assert_eq!(bom.title, "소유권 가이드");
}

#[test]
fn crlf_and_lone_cr_line_endings_parse_like_lf() {
    for name in GOLDEN_FIXTURES {
        let lf = String::from_utf8(read_fixture(name)).unwrap();
        let expected = parse_text(&format!("{name}.md"), &lf);
        for eol in ["\r\n", "\r"] {
            let converted = lf.replace('\n', eol);
            let doc = parse_text(&format!("{name}.md"), &converted);
            assert_eq!(doc, expected, "{name} with {eol:?}");
        }
    }
}

#[test]
fn normalized_text_ends_every_line_with_lf() {
    let doc = parse_text("a.md", "# A\r\n\r\ntext\r\n\r\n");
    assert_eq!(doc.normalized_text, "# A\n\ntext\n\n");
    assert_eq!(doc.normalized_text.lines().count(), 4);
    assert_eq!(ranges(&doc), [(1, 4)]);

    let unterminated = parse_text("a.md", "# A\ntext");
    assert_eq!(unterminated.normalized_text, "# A\ntext\n");
}

#[test]
fn parse_is_deterministic() {
    let first = parse_fixture("nested-headings");
    let second = parse_fixture("nested-headings");
    assert_eq!(first, second);
}

#[test]
fn supports_markdown_media_types_and_extensions() {
    let parser = MarkdownParser::new();
    assert!(parser.supports("text/markdown", "anything.bin"));
    assert!(parser.supports("text/x-markdown", ""));
    assert!(parser.supports("Text/Markdown; charset=utf-8", "a.txt"));
    assert!(parser.supports("", "guide.md"));
    assert!(parser.supports("text/plain", "dir/Guide.MARKDOWN"));
    assert!(parser.supports("application/octet-stream", "a.mdown"));
    assert!(parser.supports("", "a.mkd"));

    assert!(!parser.supports("", "guide.txt"));
    assert!(!parser.supports("", "guide"));
    assert!(!parser.supports("", ".md"));
    assert!(!parser.supports("text/html", "guide.md"));
    assert!(!parser.supports("application/pdf", "guide.md"));
    assert!(!parser.supports("text/plain", "guide.rs"));
}

#[test]
fn usable_as_a_shared_trait_object() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<MarkdownParser>();

    let parser: Box<dyn DocumentParserPort> = Box::new(MarkdownParser::new());
    assert!(parser.supports("text/markdown", "a.md"));
    let doc = parser.parse("a.md", b"# A\n\ntext\n").unwrap();
    assert_eq!(doc.title, "A");
}
