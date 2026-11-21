//! Markdown parser implementing [`DocumentParserPort`].
//!
//! The parser splits a Markdown document into flat, non-overlapping sections at H1/H2/H3 headings
//! and keeps every other line (links, code blocks, lists, tables, deeper headings) verbatim in
//! the section body. It is a line scanner, not a full CommonMark implementation, so the supported
//! syntax is deliberately small and fully listed here.
//!
//! # Input handling
//!
//! * Content that is not valid UTF-8 is rejected with [`Dok3Error::Parse`]. Decoding lossily
//!   would silently corrupt the indexed text, so the caller decides how to re-encode or skip the
//!   file (for example EUC-KR documents).
//! * A document that is empty or contains only whitespace (a lone UTF-8 BOM counts as empty) is
//!   rejected with [`Dok3Error::InvalidInput`]; there is nothing to index.
//! * Everything else parses. Recoverable oddities are reported in
//!   [`ParsedDocument::parse_warnings`] (an unclosed code fence, an empty heading).
//! * A leading UTF-8 BOM is dropped. Lines end with `\n`, `\r\n` or a lone `\r`.
//!
//! # Lines
//!
//! Line numbers are 1-based and count lines of [`ParsedDocument::normalized_text`], which is the
//! input with the BOM removed and every line ending replaced by `\n`, including a final one, so
//! `normalized_text.lines()` yields exactly the lines the line numbers refer to (blank lines
//! included).
//!
//! # Headings
//!
//! * Only ATX headings (`# Title`) start a section, and only levels 1 to 3. Setext headings
//!   (`Title` underlined with `===` or `---`) are not recognized.
//! * A heading may be indented by up to three spaces. Four or more spaces, or any tab in the
//!   leading whitespace, make the line indented code. Lines inside block quotes or list items
//!   (`> # x`, `- # x`) are not headings either.
//! * `#` must be followed by a space, a tab or the end of the line (`#hashtag` is text). A
//!   closing sequence preceded by whitespace is removed (`## Title ##` is `Title`, `## C#` stays
//!   `C#`). Levels 4 to 6 are recognized as headings but do not start a section, so they stay in
//!   the body of the section they appear in. Seven or more `#` is plain text.
//! * A heading without text is body text and produces a warning.
//! * `heading` is the heading text with inline markup preserved verbatim.
//!
//! # Fenced code blocks
//!
//! * A fence opens with three or more backticks or tildes, indented by up to three spaces. For
//!   backtick fences the info string must not contain a backtick.
//! * It closes with a line of the same character, at least as long as the opening run, indented
//!   by up to three spaces and followed only by spaces or tabs. A shorter run or the other
//!   character does not close it, so a four-backtick fence can contain a three-backtick fence.
//! * An unclosed fence runs to the end of the document and produces a warning.
//! * Nothing inside a fence is a heading or a fence marker.
//! * Only fences that start a line are recognized. A fence that follows a list marker or block
//!   quote marker (`- ```sh`, `> ```sh`) is not tracked.
//! * YAML front matter and HTML comments are not special-cased, so a `# comment` line inside
//!   them is a heading.
//!
//! # Sections
//!
//! * Sections are flat and tile the document: a section starts on its heading line and ends on
//!   the line before the next heading, so `next.line_start == previous.line_end + 1`. The last
//!   section ends on the last line. `DocumentSection` has no level field, so nesting is not
//!   represented; a section holds only its own text, not its subsections'.
//! * `body` is the text after the heading line, verbatim, without leading or trailing blank lines.
//! * Non-blank text before the first heading becomes a leading section whose heading is the
//!   document title and whose anchor is `preamble` (`body` when the document has no heading at
//!   all). It starts on its first non-blank line.
//! * `order` is 1-based and `section_id` equals `anchor`.
//!
//! # Anchors
//!
//! The anchor is derived from the heading text and is unique within the document:
//!
//! 1. Inline links, images and reference links are reduced to their text.
//! 2. The text is lowercased with Unicode rules.
//! 3. Letters, digits (any script, so Hangul is kept) and `_` are kept. Whitespace and `-`
//!    separate words. All other characters (punctuation, backticks, emphasis, emoji) are dropped.
//! 4. Separator runs collapse into one `-`, and leading and trailing separators are removed.
//! 5. A heading with no kept characters gets the anchor `section`.
//! 6. The second and later occurrence of an anchor gets `-1`, `-2`, ... appended, like GitHub.
//!
//! # Title
//!
//! The title is the text of the first H1 heading, or the file name without directories and
//! extension (`untitled` when that is empty).

use std::collections::HashSet;

use dok3_core::Dok3Error;
use dok3_domain::{DocumentParserPort, DocumentSection, ParsedDocument, SectionId};

/// Deepest heading level that starts a section.
const MAX_SECTION_LEVEL: usize = 3;
/// Deepest ATX heading level; `#######` is plain text.
const MAX_ATX_LEVEL: usize = 6;
/// Title used when the document has no H1 and the file name has no stem.
const FALLBACK_TITLE: &str = "untitled";
/// Anchor used when a heading has no characters that survive slugging.
const FALLBACK_ANCHOR: &str = "section";
/// Anchor of the text that precedes the first heading.
const PREAMBLE_ANCHOR: &str = "preamble";
/// Anchor of the single section of a document without any heading.
const BODY_ANCHOR: &str = "body";
/// File extensions treated as Markdown (compared case-insensitively).
const MARKDOWN_EXTENSIONS: [&str; 4] = ["md", "markdown", "mdown", "mkd"];

/// Parses Markdown documents into heading-delimited sections.
///
/// See the [module documentation](self) for the exact syntax rules.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MarkdownParser;

impl MarkdownParser {
    /// Creates a parser. The parser is stateless, so one instance can be shared freely.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl DocumentParserPort for MarkdownParser {
    /// Accepts `text/markdown` and `text/x-markdown`. A `.md`, `.markdown`, `.mdown` or `.mkd`
    /// file name is accepted when the media type is empty, `text/plain` or
    /// `application/octet-stream`; any other explicit media type (for example `text/html`) wins.
    fn supports(&self, media_type: &str, filename: &str) -> bool {
        let media = media_type
            .split(';')
            .next()
            .unwrap_or_default()
            .trim()
            .to_ascii_lowercase();
        if matches!(media.as_str(), "text/markdown" | "text/x-markdown") {
            return true;
        }
        let generic = matches!(
            media.as_str(),
            "" | "text/plain" | "application/octet-stream"
        );
        generic && has_markdown_extension(filename)
    }

    fn parse(&self, filename: &str, content: &[u8]) -> Result<ParsedDocument, Dok3Error> {
        let name = display_name(filename);
        let text = std::str::from_utf8(content).map_err(|err| {
            Dok3Error::Parse(format!(
                "{name}: content is not valid UTF-8 (first invalid byte at offset {})",
                err.valid_up_to()
            ))
        })?;
        let text = text.strip_prefix('\u{feff}').unwrap_or(text);
        if text.trim().is_empty() {
            return Err(Dok3Error::InvalidInput(format!(
                "{name}: document is empty"
            )));
        }
        Ok(parse_markdown(filename, text))
    }
}

/// A heading that starts a section, located by its 0-based line index.
#[derive(Debug)]
struct RawHeading {
    line_idx: usize,
    level: usize,
    text: String,
}

/// An open fenced code block.
#[derive(Debug, Clone, Copy)]
struct Fence {
    marker: u8,
    len: usize,
    /// 1-based line number of the opening line.
    line: usize,
}

/// Result of the line scan.
#[derive(Debug)]
struct Scan {
    headings: Vec<RawHeading>,
    warnings: Vec<String>,
}

fn parse_markdown(filename: &str, text: &str) -> ParsedDocument {
    let lines = split_lines(text);
    let Scan { headings, warnings } = scan_headings(&lines);
    let title = document_title(filename, &headings);
    let sections = build_sections(&lines, &headings, &title);
    ParsedDocument {
        title,
        sections,
        normalized_text: lines.iter().flat_map(|line| [*line, "\n"]).collect(),
        parse_warnings: warnings,
    }
}

/// Splits on `\n`, `\r\n` and lone `\r`. A final line terminator does not add an empty line.
fn split_lines(text: &str) -> Vec<&str> {
    let bytes = text.as_bytes();
    let mut lines = Vec::new();
    let mut start = 0;
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'\n' => {
                lines.push(&text[start..i]);
                i += 1;
                start = i;
            }
            b'\r' => {
                lines.push(&text[start..i]);
                i += 1;
                if bytes.get(i) == Some(&b'\n') {
                    i += 1;
                }
                start = i;
            }
            _ => i += 1,
        }
    }
    if start < bytes.len() {
        lines.push(&text[start..]);
    }
    lines
}

/// Finds H1-H3 headings outside fenced code blocks.
fn scan_headings(lines: &[&str]) -> Scan {
    let mut headings = Vec::new();
    let mut warnings = Vec::new();
    let mut open_fence: Option<Fence> = None;

    for (idx, line) in lines.iter().enumerate() {
        if let Some(fence) = open_fence {
            if closes_fence(line, fence) {
                open_fence = None;
            }
            continue;
        }
        if let Some((marker, len)) = opening_fence(line) {
            open_fence = Some(Fence {
                marker,
                len,
                line: idx + 1,
            });
            continue;
        }
        let Some((level, text)) = atx_heading(line) else {
            continue;
        };
        if level > MAX_SECTION_LEVEL {
            continue;
        }
        if text.is_empty() {
            warnings.push(format!("empty heading at line {} ignored", idx + 1));
            continue;
        }
        headings.push(RawHeading {
            line_idx: idx,
            level,
            text: text.to_string(),
        });
    }

    if let Some(fence) = open_fence {
        warnings.push(format!(
            "unclosed code fence opened at line {}; the rest of the document is treated as code",
            fence.line
        ));
    }
    Scan { headings, warnings }
}

/// Returns the number of leading spaces (at most three), or `None` when the line is indented by
/// four columns or more. A tab in the leading whitespace always reaches column four.
fn indent_of(line: &str) -> Option<usize> {
    let spaces = line.bytes().take_while(|&b| b == b' ').count();
    if spaces >= 4 || line.as_bytes().get(spaces) == Some(&b'\t') {
        None
    } else {
        Some(spaces)
    }
}

/// Recognizes a fence opening line and returns its marker byte and run length.
fn opening_fence(line: &str) -> Option<(u8, usize)> {
    let indent = indent_of(line)?;
    let rest = &line[indent..];
    let marker = *rest.as_bytes().first()?;
    if marker != b'`' && marker != b'~' {
        return None;
    }
    let len = rest.bytes().take_while(|&b| b == marker).count();
    if len < 3 {
        return None;
    }
    if marker == b'`' && rest[len..].contains('`') {
        return None;
    }
    Some((marker, len))
}

/// Whether `line` closes `fence`: same character, at least as long, nothing but blanks after it.
fn closes_fence(line: &str, fence: Fence) -> bool {
    let Some(indent) = indent_of(line) else {
        return false;
    };
    let rest = &line[indent..];
    let run = rest.bytes().take_while(|&b| b == fence.marker).count();
    run >= fence.len && rest[run..].bytes().all(|b| b == b' ' || b == b'\t')
}

/// Recognizes an ATX heading of level 1-6 and returns its level and cleaned text.
fn atx_heading(line: &str) -> Option<(usize, &str)> {
    let indent = indent_of(line)?;
    let rest = &line[indent..];
    let level = rest.bytes().take_while(|&b| b == b'#').count();
    if level == 0 || level > MAX_ATX_LEVEL {
        return None;
    }
    let after = &rest[level..];
    if !after.is_empty() && !after.starts_with([' ', '\t']) {
        return None;
    }
    Some((level, strip_closing_sequence(after.trim())))
}

/// Removes a closing `#` run that is separated from the text by whitespace.
fn strip_closing_sequence(text: &str) -> &str {
    let without = text.trim_end_matches('#');
    if without.len() == text.len() {
        return text;
    }
    if without.is_empty() {
        return "";
    }
    if without.ends_with([' ', '\t']) {
        without.trim_end()
    } else {
        text
    }
}

fn document_title(filename: &str, headings: &[RawHeading]) -> String {
    headings
        .iter()
        .find(|heading| heading.level == 1)
        .map_or_else(|| file_stem(filename), |heading| heading.text.clone())
}

fn file_stem(filename: &str) -> String {
    let name = filename.rsplit(['/', '\\']).next().unwrap_or(filename);
    let stem = match name.rfind('.') {
        Some(pos) if pos > 0 => &name[..pos],
        _ => name,
    }
    .trim();
    if stem.is_empty() {
        FALLBACK_TITLE.to_string()
    } else {
        stem.to_string()
    }
}

fn has_markdown_extension(filename: &str) -> bool {
    let name = filename.rsplit(['/', '\\']).next().unwrap_or(filename);
    match name.rsplit_once('.') {
        Some((stem, extension)) if !stem.is_empty() => MARKDOWN_EXTENSIONS
            .iter()
            .any(|known| extension.eq_ignore_ascii_case(known)),
        _ => false,
    }
}

fn display_name(filename: &str) -> &str {
    if filename.is_empty() {
        "<unnamed>"
    } else {
        filename
    }
}

/// Builds the section list: an optional leading section, then one section per heading.
fn build_sections(lines: &[&str], headings: &[RawHeading], title: &str) -> Vec<DocumentSection> {
    let total = lines.len();
    let first_heading = headings.first().map_or(total, |heading| heading.line_idx);
    let mut anchors = HashSet::new();
    let mut sections = Vec::with_capacity(headings.len() + 1);

    if let Some(start) = lines[..first_heading]
        .iter()
        .position(|line| !line.trim().is_empty())
    {
        let reserved = if headings.is_empty() {
            BODY_ANCHOR
        } else {
            PREAMBLE_ANCHOR
        };
        let anchor = unique_anchor(reserved.to_string(), &mut anchors);
        sections.push(DocumentSection {
            section_id: SectionId::new(anchor.clone()),
            heading: title.to_string(),
            anchor,
            order: 1,
            line_start: start + 1,
            line_end: first_heading,
            body: join_trimmed(&lines[start..first_heading]),
        });
    }

    for (i, heading) in headings.iter().enumerate() {
        let end = headings.get(i + 1).map_or(total, |next| next.line_idx);
        let anchor = unique_anchor(slugify(&heading.text), &mut anchors);
        sections.push(DocumentSection {
            section_id: SectionId::new(anchor.clone()),
            heading: heading.text.clone(),
            anchor,
            order: sections.len() + 1,
            line_start: heading.line_idx + 1,
            line_end: end,
            body: join_trimmed(&lines[heading.line_idx + 1..end]),
        });
    }
    sections
}

/// Joins lines with `\n` after dropping blank lines at both ends.
fn join_trimmed(lines: &[&str]) -> String {
    let start = lines
        .iter()
        .position(|line| !line.trim().is_empty())
        .unwrap_or(lines.len());
    let end = lines
        .iter()
        .rposition(|line| !line.trim().is_empty())
        .map_or(start, |last| last + 1);
    lines[start..end].join("\n")
}

/// Reserves `base`, or `base-1`, `base-2`, ... when it is taken.
fn unique_anchor(base: String, used: &mut HashSet<String>) -> String {
    if used.insert(base.clone()) {
        return base;
    }
    let mut n = 1_usize;
    loop {
        let candidate = format!("{base}-{n}");
        if used.insert(candidate.clone()) {
            return candidate;
        }
        n += 1;
    }
}

/// Derives the anchor slug of a heading, following the rules in the module documentation.
fn slugify(heading: &str) -> String {
    let plain = strip_link_targets(heading);
    let mut slug = String::new();
    let mut pending_separator = false;
    for ch in plain.chars().flat_map(char::to_lowercase) {
        if ch.is_alphanumeric() || ch == '_' {
            if pending_separator && !slug.is_empty() {
                slug.push('-');
            }
            pending_separator = false;
            slug.push(ch);
        } else if ch.is_whitespace() || ch == '-' {
            pending_separator = true;
        }
    }
    if slug.is_empty() {
        FALLBACK_ANCHOR.to_string()
    } else {
        slug
    }
}

/// Drops the `(destination)` of inline links and images and the `[label]` of reference links,
/// keeping the link text.
fn strip_link_targets(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    while i < chars.len() {
        let ch = chars[i];
        out.push(ch);
        if ch == ']' {
            let skip_to = match chars.get(i + 1) {
                Some('(') => closing_index(&chars, i + 1, '(', ')'),
                Some('[') => closing_index(&chars, i + 1, '[', ']'),
                _ => None,
            };
            if let Some(close) = skip_to {
                i = close;
            }
        }
        i += 1;
    }
    out
}

/// Index of the delimiter that closes the one at `open_idx`, honoring nesting.
fn closing_index(chars: &[char], open_idx: usize, open: char, close: char) -> Option<usize> {
    let mut depth = 0_usize;
    for (offset, &ch) in chars[open_idx..].iter().enumerate() {
        if ch == open {
            depth += 1;
        } else if ch == close {
            depth -= 1;
            if depth == 0 {
                return Some(open_idx + offset);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{
        Fence, atx_heading, closes_fence, has_markdown_extension, opening_fence, slugify,
        split_lines, strip_closing_sequence,
    };

    #[test]
    fn split_lines_handles_all_line_endings() {
        assert_eq!(split_lines("a\nb\r\nc\rd"), ["a", "b", "c", "d"]);
        assert_eq!(split_lines("a\n"), ["a"]);
        assert_eq!(split_lines("a\n\n"), ["a", ""]);
        assert_eq!(split_lines("\r\n\r\n"), ["", ""]);
        assert!(split_lines("").is_empty());
        assert_eq!(split_lines("한글\n줄"), ["한글", "줄"]);
    }

    #[test]
    fn atx_heading_follows_commonmark_rules() {
        assert_eq!(atx_heading("# Title"), Some((1, "Title")));
        assert_eq!(atx_heading("   ### Title  "), Some((3, "Title")));
        assert_eq!(atx_heading("## Title ##"), Some((2, "Title")));
        assert_eq!(atx_heading("## C#"), Some((2, "C#")));
        assert_eq!(atx_heading("##\tTab"), Some((2, "Tab")));
        assert_eq!(atx_heading("#"), Some((1, "")));
        assert_eq!(atx_heading("## ##"), Some((2, "")));
        assert_eq!(atx_heading("###### Six"), Some((6, "Six")));

        assert_eq!(atx_heading("#hashtag"), None);
        assert_eq!(atx_heading("####### Seven"), None);
        assert_eq!(atx_heading("    # indented"), None);
        assert_eq!(atx_heading("\t# tabbed"), None);
        assert_eq!(atx_heading(" \t# space then tab"), None);
        assert_eq!(atx_heading("> # quoted"), None);
        assert_eq!(atx_heading("- # listed"), None);
        assert_eq!(atx_heading("\\# escaped"), None);
        assert_eq!(atx_heading("text # not heading"), None);
    }

    #[test]
    fn closing_sequence_needs_leading_whitespace() {
        assert_eq!(strip_closing_sequence("Title ##"), "Title");
        assert_eq!(strip_closing_sequence("Title#"), "Title#");
        assert_eq!(strip_closing_sequence("foo \\#"), "foo \\#");
        assert_eq!(strip_closing_sequence("###"), "");
    }

    #[test]
    fn opening_fence_rules() {
        assert_eq!(opening_fence("```"), Some((b'`', 3)));
        assert_eq!(opening_fence("```rust"), Some((b'`', 3)));
        assert_eq!(opening_fence("   ~~~~ sh"), Some((b'~', 4)));
        assert_eq!(opening_fence("~~~ a`b"), Some((b'~', 3)));

        assert_eq!(opening_fence("``"), None);
        assert_eq!(opening_fence("~~"), None);
        assert_eq!(opening_fence("``` a`b"), None);
        assert_eq!(opening_fence("    ```"), None);
        assert_eq!(opening_fence("\t```"), None);
        assert_eq!(opening_fence("- ```"), None);
        assert_eq!(opening_fence("text ```"), None);
    }

    #[test]
    fn closing_fence_rules() {
        let backticks = Fence {
            marker: b'`',
            len: 4,
            line: 1,
        };
        assert!(closes_fence("````", backticks));
        assert!(closes_fence("`````  ", backticks));
        assert!(closes_fence("  ````\t", backticks));
        assert!(!closes_fence("```", backticks));
        assert!(!closes_fence("~~~~", backticks));
        assert!(!closes_fence("```` rust", backticks));
        assert!(!closes_fence("    ````", backticks));
    }

    #[test]
    fn slugify_keeps_letters_digits_and_underscore() {
        assert_eq!(slugify("Getting Started"), "getting-started");
        assert_eq!(slugify("소유권과 대여"), "소유권과-대여");
        assert_eq!(slugify("Ünïcode ÇASE"), "ünïcode-çase");
        assert_eq!(slugify("snake_case name"), "snake_case-name");
        assert_eq!(slugify("v1.2 릴리스 노트"), "v12-릴리스-노트");
        assert_eq!(slugify("C# & F#"), "c-f");
        assert_eq!(slugify("  a  -  b  "), "a-b");
        assert_eq!(slugify("`Vec<T>` guide"), "vect-guide");
        assert_eq!(slugify("**Bold** and *it*"), "bold-and-it");
        assert_eq!(slugify("Done ✅"), "done");
    }

    #[test]
    fn slugify_reduces_links_to_their_text() {
        assert_eq!(
            slugify("See [the docs](https://example.com/a_(b)) now"),
            "see-the-docs-now"
        );
        assert_eq!(slugify("Logo ![alt text](logo.png)"), "logo-alt-text");
        assert_eq!(slugify("[Label][ref] end"), "label-end");
        assert_eq!(slugify("broken [link](unclosed"), "broken-linkunclosed");
    }

    #[test]
    fn slugify_falls_back_when_nothing_survives() {
        assert_eq!(slugify("???"), "section");
        assert_eq!(slugify("✅ 🎉"), "section");
        assert_eq!(slugify("---"), "section");
    }

    #[test]
    fn markdown_extension_detection() {
        assert!(has_markdown_extension("guide.md"));
        assert!(has_markdown_extension("dir/GUIDE.MD"));
        assert!(has_markdown_extension("C:\\docs\\a.markdown"));
        assert!(has_markdown_extension("a.b.mkd"));
        assert!(!has_markdown_extension("guide.txt"));
        assert!(!has_markdown_extension("guide"));
        assert!(!has_markdown_extension(".md"));
        assert!(!has_markdown_extension("dir.md/file"));
    }
}
