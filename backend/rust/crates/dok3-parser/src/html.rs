//! HTML parser implementing [`DocumentParserPort`].
//!
//! The parser normalizes HTML into the same Markdown-shaped contract as the Markdown parser:
//! heading-delimited sections over a normalized, line-numbered text payload. It keeps semantic
//! prose, links, inline code and fenced code blocks, and intentionally removes executable or
//! styling-only HTML. Structural information that cannot survive that normalization is recorded
//! as a loss warning rather than being silently discarded.

use std::collections::HashSet;

use dok3_core::Dok3Error;
use dok3_domain::{DocumentParserPort, DocumentSection, ParsedDocument, SectionId};
use ego_tree::NodeRef;
use scraper::{ElementRef, Html, Node};

/// File extensions treated as HTML (compared case-insensitively).
pub const HTML_EXTENSIONS: [&str; 2] = ["html", "htm"];
/// Deepest heading level that starts a section.
const MAX_SECTION_LEVEL: usize = 3;
/// Deepest heading level represented as Markdown syntax.
const MAX_HEADING_LEVEL: usize = 6;
/// Title used when no H1, `<title>` or file stem is available.
const FALLBACK_TITLE: &str = "untitled";
/// Anchor used when a heading has no characters that survive slugging.
const FALLBACK_ANCHOR: &str = "section";
/// Anchor of the text that precedes the first heading.
const PREAMBLE_ANCHOR: &str = "preamble";
/// Anchor of the single section of a document without any heading.
const BODY_ANCHOR: &str = "body";

/// Parses HTML documents into heading-delimited sections.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct HtmlParser;

impl HtmlParser {
    /// Creates a parser. The parser is stateless, so one instance can be shared freely.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl DocumentParserPort for HtmlParser {
    /// Accepts `text/html` and `application/xhtml+xml`. An `.html` or `.htm` file name is
    /// accepted when the media type is empty, `text/plain` or `application/octet-stream`.
    fn supports(&self, media_type: &str, filename: &str) -> bool {
        let media = media_type
            .split(';')
            .next()
            .unwrap_or_default()
            .trim()
            .to_ascii_lowercase();
        if matches!(media.as_str(), "text/html" | "application/xhtml+xml") {
            return true;
        }
        let generic = matches!(
            media.as_str(),
            "" | "text/plain" | "application/octet-stream"
        );
        generic && has_html_extension(filename)
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

        let document = Html::parse_document(text);
        let mut renderer = HtmlRenderer::new();
        renderer.render_document(&document);
        for error in &document.errors {
            renderer
                .warnings
                .push(format!("HTML5 parser corrected markup: {error}"));
        }
        renderer.push_loss_summary();

        let RenderedHtml {
            lines,
            headings,
            warnings,
        } = renderer.finish();
        if lines.iter().all(|line| line.trim().is_empty()) {
            return Err(Dok3Error::Parse(format!(
                "{name}: document has no renderable content"
            )));
        }

        let title = document_title(filename, &headings, &document);
        let sections = build_sections(&lines, &headings, &title);
        Ok(ParsedDocument {
            title,
            sections,
            normalized_text: lines
                .iter()
                .flat_map(|line| [line.clone(), "\n".to_string()])
                .collect(),
            parse_warnings: warnings,
        })
    }
}

/// A heading that starts a section, located by its 0-based rendered-line index.
#[derive(Debug)]
struct RawHeading {
    line_idx: usize,
    level: usize,
    text: String,
}

/// Counted conversion losses, collapsed into one warning per category.
#[derive(Debug, Default)]
struct LossCounts {
    comments: usize,
    scripts: usize,
    styles: usize,
    tables: usize,
    presentation_elements: usize,
}

/// Renders an HTML document body into normalized lines while recording section headings.
#[derive(Debug)]
struct HtmlRenderer {
    lines: Vec<String>,
    headings: Vec<RawHeading>,
    warnings: Vec<String>,
    losses: LossCounts,
    presentation_warnings: Vec<String>,
}

impl HtmlRenderer {
    fn new() -> Self {
        Self {
            lines: Vec::new(),
            headings: Vec::new(),
            warnings: Vec::new(),
            losses: LossCounts::default(),
            presentation_warnings: Vec::new(),
        }
    }

    fn render_document(&mut self, document: &Html) {
        let root = document.root_element();
        for head in root
            .child_elements()
            .filter(|element| element.value().name() == "head")
        {
            for child in head.descendent_elements() {
                match child.value().name() {
                    "script" => self.losses.scripts += 1,
                    "style" => self.losses.styles += 1,
                    _ => {}
                }
            }
        }
        let body = root
            .child_elements()
            .find(|element| element.value().name() == "body");
        if let Some(body) = body {
            self.render_nodes(body.children());
        } else {
            self.render_nodes(root.children());
        }
    }

    fn render_nodes<'a>(&mut self, nodes: impl Iterator<Item = NodeRef<'a, Node>>) {
        for node in nodes {
            self.render_node(node);
        }
    }

    fn render_node(&mut self, node: NodeRef<'_, Node>) {
        match node.value() {
            Node::Text(text) => self.append_plain(text),
            Node::Comment(_) => self.losses.comments += 1,
            Node::Element(_) => {
                let element = ElementRef::wrap(node).expect("element node wraps as ElementRef");
                self.render_element(element, element.value().name());
            }
            _ => {}
        }
    }

    fn render_element(&mut self, element: ElementRef<'_>, name: &str) {
        match name {
            "script" => {
                self.losses.scripts += 1;
            }
            "style" => {
                self.losses.styles += 1;
            }
            "head" => {}
            "title" | "meta" | "link" | "base" => {}
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                let level = name[1..].parse::<usize>().unwrap_or(MAX_HEADING_LEVEL);
                self.render_heading(element, level);
            }
            "p" => {
                self.separate_blocks();
                self.render_paragraph(element);
            }
            "pre" => {
                self.separate_blocks();
                self.render_pre(element);
            }
            "ul" | "ol" => {
                self.separate_blocks();
                self.render_list(element, name == "ol");
            }
            "table" => {
                self.separate_blocks();
                self.losses.tables += 1;
                self.render_nodes(element.children());
                self.separate_blocks();
            }
            "hr" => {
                self.separate_blocks();
                self.push_block_line("---");
                self.separate_blocks();
            }
            "br" => self.append_plain(" "),
            "font" | "center" | "marquee" => {
                self.losses.presentation_elements += 1;
                self.note_presentation(name);
                self.render_nodes(element.children());
            }
            "blockquote" => {
                self.separate_blocks();
                self.render_nodes(element.children());
                self.separate_blocks();
            }
            _ => {
                if is_block_like(name) {
                    self.separate_blocks();
                    self.render_nodes(element.children());
                    self.separate_blocks();
                } else {
                    self.append_inline(element);
                }
            }
        }
        self.note_dropped_attributes(element, name);
    }

    fn render_heading(&mut self, element: ElementRef<'_>, level: usize) {
        let text = inline_text(element);
        self.separate_blocks();
        if text.trim().is_empty() {
            let line = self.lines.len() + 1;
            self.warnings
                .push(format!("empty heading at line {line} ignored"));
            return;
        }
        let marker = "#".repeat(level.min(MAX_HEADING_LEVEL));
        let text = normalize_inline(&text);
        self.lines.push(format!("{marker} {text}"));
        if level <= MAX_SECTION_LEVEL {
            self.headings.push(RawHeading {
                line_idx: self.lines.len() - 1,
                level,
                text,
            });
        }
        self.separate_blocks();
    }

    fn render_paragraph(&mut self, element: ElementRef<'_>) {
        let mut text = String::new();
        render_inline_nodes(element.children(), &mut text, &mut self.warnings);
        self.push_inline_text(&text);
        self.separate_blocks();
    }

    fn render_pre(&mut self, element: ElementRef<'_>) {
        let language = pre_language(element);
        let marker = String::from("```");
        self.lines.push(if language.is_empty() {
            marker
        } else {
            format!("```{language}")
        });
        let code = element.text().collect::<String>();
        let code = code.trim_matches('\n');
        for line in code.lines() {
            self.lines.push(line.trim_end().to_string());
        }
        self.lines.push("```".to_string());
        self.separate_blocks();
    }

    fn render_list(&mut self, element: ElementRef<'_>, ordered: bool) {
        let mut number = 0_usize;
        for child in element.child_elements() {
            if child.value().name() != "li" {
                self.render_element(child, child.value().name());
                continue;
            }
            number += 1;
            self.separate_blocks();
            let prefix = if ordered {
                format!("{number}. ")
            } else {
                "- ".to_string()
            };
            let mut item_text = String::new();
            let mut warnings = Vec::new();
            for node in child.children() {
                match node.value() {
                    Node::Element(_) => {
                        let nested = ElementRef::wrap(node).expect("element node wraps");
                        let nested_name = nested.value().name();
                        if nested_name == "ul" || nested_name == "ol" {
                            continue;
                        }
                        render_inline_node(node, &mut item_text, &mut warnings);
                    }
                    _ => render_inline_node(node, &mut item_text, &mut warnings),
                }
            }
            self.warnings.extend(warnings);
            self.push_block_line(&format!("{prefix}{}", normalize_inline(&item_text)));
            for nested in child.child_elements() {
                let nested_name = nested.value().name();
                if nested_name == "ul" || nested_name == "ol" {
                    self.render_list(nested, nested_name == "ol");
                }
            }
        }
        self.separate_blocks();
    }

    fn append_inline(&mut self, element: ElementRef<'_>) {
        let mut text = String::new();
        let mut warnings = Vec::new();
        render_inline_node(*element, &mut text, &mut warnings);
        self.warnings.extend(warnings);
        self.append_inline_text(&text);
    }

    fn append_plain(&mut self, text: &str) {
        self.append_inline_text(text);
    }

    fn append_inline_text(&mut self, value: &str) {
        let value = normalize_inline(value);
        if value.is_empty() {
            return;
        }
        match self.lines.last_mut() {
            Some(line) if !line.is_empty() => {
                if !line.ends_with([' ', '\t']) {
                    line.push(' ');
                }
                line.push_str(&value);
            }
            Some(_) => self.lines.push(value),
            None => self.lines.push(value),
        }
    }

    fn push_inline_text(&mut self, value: &str) {
        for piece in value.split('\n') {
            self.append_inline_text(piece);
            if piece != value.split('\n').next_back().unwrap_or_default() {
                self.lines.push(String::new());
            }
        }
    }

    fn push_block_line(&mut self, line: &str) {
        if line.trim().is_empty() {
            return;
        }
        self.lines.push(line.trim_end().to_string());
    }

    fn separate_blocks(&mut self) {
        if self
            .lines
            .last()
            .is_some_and(|line| !line.trim().is_empty())
        {
            self.lines.push(String::new());
        }
    }

    fn note_presentation(&mut self, name: &str) {
        let warning = format!("presentation element <{name}> was dropped; text was retained");
        if !self.presentation_warnings.contains(&warning) {
            self.presentation_warnings.push(warning);
        }
    }

    fn note_dropped_attributes(&mut self, element: ElementRef<'_>, name: &str) {
        let class = element.attr("class");
        let style = element.attr("style");
        let language_class = class.is_some_and(|value| {
            value.split_ascii_whitespace().any(|class| {
                ["language-", "lang-", "highlight-source-"]
                    .iter()
                    .any(|prefix| class.starts_with(prefix))
            })
        });
        if (name == "pre" || name == "code") && language_class && style.is_none() {
            return;
        }
        let class_has_content = class.is_some_and(|value| !value.trim().is_empty());
        let style_has_content = style.is_some_and(|value| !value.trim().is_empty());
        if class_has_content || style_has_content {
            let warning = format!("presentation attributes on <{name}> were dropped");
            if !self.presentation_warnings.contains(&warning) {
                self.presentation_warnings.push(warning);
            }
        }
    }

    fn push_loss_summary(&mut self) {
        let LossCounts {
            comments,
            scripts,
            styles,
            tables,
            presentation_elements,
        } = self.losses;
        if comments > 0 {
            self.warnings
                .push(format!("discarded {comments} HTML comment(s)"));
        }
        if scripts > 0 {
            self.warnings
                .push(format!("discarded {scripts} script element(s)"));
        }
        if styles > 0 {
            self.warnings
                .push(format!("discarded {styles} style element(s)"));
        }
        if tables > 0 {
            self.warnings.push(
                "table cell text was retained, but table formatting was not preserved".to_string(),
            );
        }
        if presentation_elements > 0 {
            self.warnings.push(format!(
                "dropped {presentation_elements} presentation element(s); text was retained"
            ));
        }
        for warning in self.presentation_warnings.drain(..) {
            self.warnings.push(warning);
        }
    }

    fn finish(mut self) -> RenderedHtml {
        while self.lines.last().is_some_and(|line| line.trim().is_empty()) {
            self.lines.pop();
        }
        self.warnings = dedupe_warnings(self.warnings);
        RenderedHtml {
            lines: self.lines,
            headings: self.headings,
            warnings: self.warnings,
        }
    }
}

struct RenderedHtml {
    lines: Vec<String>,
    headings: Vec<RawHeading>,
    warnings: Vec<String>,
}

fn is_block_like(name: &str) -> bool {
    matches!(
        name,
        "address"
            | "article"
            | "aside"
            | "blockquote"
            | "details"
            | "dd"
            | "div"
            | "dl"
            | "dt"
            | "fieldset"
            | "figcaption"
            | "figure"
            | "footer"
            | "form"
            | "header"
            | "li"
            | "main"
            | "nav"
            | "section"
            | "summary"
            | "tbody"
            | "td"
            | "tfoot"
            | "th"
            | "thead"
            | "tr"
    )
}

fn render_inline_nodes<'a>(
    nodes: impl Iterator<Item = NodeRef<'a, Node>>,
    out: &mut String,
    warnings: &mut Vec<String>,
) {
    for node in nodes {
        render_inline_node(node, out, warnings);
    }
}

fn render_inline_node(node: NodeRef<'_, Node>, out: &mut String, warnings: &mut Vec<String>) {
    match node.value() {
        Node::Text(text) => out.push_str(text),
        Node::Element(_) => {
            let element = ElementRef::wrap(node).expect("element node wraps as ElementRef");
            render_inline_element(element, out, warnings);
        }
        _ => {}
    }
}

fn render_inline_element(element: ElementRef<'_>, out: &mut String, warnings: &mut Vec<String>) {
    match element.value().name() {
        "script" | "style" | "title" | "head" | "meta" | "link" | "base" => {}
        "a" => {
            let mut inner = String::new();
            render_inline_nodes(element.children(), &mut inner, warnings);
            let inner = normalize_inline(&inner);
            match element.attr("href") {
                Some(href) if !href.trim().is_empty() => {
                    let label = if inner.is_empty() {
                        normalize_inline(href)
                    } else {
                        inner
                    };
                    out.push('[');
                    out.push_str(&label);
                    out.push_str("](");
                    out.push_str(href.trim());
                    out.push(')');
                }
                _ => out.push_str(&inner),
            }
            if element
                .attr("title")
                .is_some_and(|value| !value.trim().is_empty())
            {
                warnings.push("link tooltip text was dropped".to_string());
            }
        }
        "img" => {
            let alt = normalize_inline(element.attr("alt").unwrap_or_default());
            let src = element.attr("src").unwrap_or_default().trim();
            if alt.is_empty() {
                warnings.push("image without alternative text was dropped".to_string());
            } else if !src.is_empty() {
                out.push_str(&format!("![{alt}]({src})"));
            } else {
                out.push_str(&alt);
            }
        }
        "code" => {
            let value = normalize_inline(&element.text().collect::<String>());
            let delimiter = "`".repeat(1 + value.matches('`').count());
            out.push_str(&delimiter);
            out.push_str(&value);
            out.push_str(&delimiter);
        }
        "strong" | "b" => {
            out.push_str("**");
            render_inline_nodes(element.children(), out, warnings);
            out.push_str("**");
        }
        "em" | "i" => {
            out.push('*');
            render_inline_nodes(element.children(), out, warnings);
            out.push('*');
        }
        "br" => out.push(' '),
        _ => render_inline_nodes(element.children(), out, warnings),
    }
}

fn inline_text(element: ElementRef<'_>) -> String {
    let mut text = String::new();
    let mut warnings = Vec::new();
    render_inline_nodes(element.children(), &mut text, &mut warnings);
    text
}

fn pre_language(element: ElementRef<'_>) -> String {
    let Some(code) = element
        .descendent_elements()
        .find(|element| element.value().name() == "code")
    else {
        return String::new();
    };
    element
        .attr("class")
        .or_else(|| code.attr("class"))
        .unwrap_or_default()
        .split_ascii_whitespace()
        .find_map(|class| {
            ["language-", "lang-", "highlight-source-"]
                .iter()
                .find_map(|prefix| class.strip_prefix(prefix))
        })
        .unwrap_or_default()
        .to_ascii_lowercase()
}

fn normalize_inline(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn dedupe_warnings(warnings: Vec<String>) -> Vec<String> {
    let mut used = HashSet::new();
    warnings
        .into_iter()
        .filter(|warning| used.insert(warning.clone()))
        .collect()
}

fn document_title(filename: &str, headings: &[RawHeading], document: &Html) -> String {
    headings
        .iter()
        .find(|heading| heading.level == 1)
        .map(|heading| heading.text.clone())
        .or_else(|| {
            document
                .root_element()
                .descendent_elements()
                .find(|element| element.value().name() == "title")
                .map(|title| normalize_inline(&title.text().collect::<String>()))
                .filter(|title| !title.is_empty())
        })
        .unwrap_or_else(|| file_stem(filename))
}

fn build_sections(lines: &[String], headings: &[RawHeading], title: &str) -> Vec<DocumentSection> {
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

    for (index, heading) in headings.iter().enumerate() {
        let end = headings.get(index + 1).map_or(total, |next| next.line_idx);
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

fn join_trimmed(lines: &[String]) -> String {
    let start = lines
        .iter()
        .position(|line| !line.trim().is_empty())
        .unwrap_or(lines.len());
    let end = lines
        .iter()
        .rposition(|line| !line.trim().is_empty())
        .map_or(start, |last| last + 1);
    lines[start..end]
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join("\n")
}

fn unique_anchor(base: String, used: &mut HashSet<String>) -> String {
    if used.insert(base.clone()) {
        return base;
    }
    let mut number = 1_usize;
    loop {
        let candidate = format!("{base}-{number}");
        if used.insert(candidate.clone()) {
            return candidate;
        }
        number += 1;
    }
}

fn slugify(heading: &str) -> String {
    let mut slug = String::new();
    let mut pending_separator = false;
    for character in heading.chars().flat_map(char::to_lowercase) {
        if character.is_alphanumeric() || character == '_' {
            if pending_separator && !slug.is_empty() {
                slug.push('-');
            }
            pending_separator = false;
            slug.push(character);
        } else if character.is_whitespace() || character == '-' {
            pending_separator = true;
        }
    }
    if slug.is_empty() {
        FALLBACK_ANCHOR.to_string()
    } else {
        slug
    }
}

fn file_stem(filename: &str) -> String {
    let name = filename.rsplit(['/', '\\']).next().unwrap_or(filename);
    let stem = match name.rfind('.') {
        Some(position) if position > 0 => &name[..position],
        _ => name,
    }
    .trim();
    if stem.is_empty() {
        FALLBACK_TITLE.to_string()
    } else {
        stem.to_string()
    }
}

fn has_html_extension(filename: &str) -> bool {
    let name = filename.rsplit(['/', '\\']).next().unwrap_or(filename);
    match name.rsplit_once('.') {
        Some((stem, extension)) if !stem.is_empty() => HTML_EXTENSIONS
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
