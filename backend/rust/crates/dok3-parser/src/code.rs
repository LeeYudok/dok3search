//! Line-based source code symbol parser for Rust, Java, Go, SQL, and JSP.

use std::collections::HashMap;
use std::path::Path;

use dok3_core::Dok3Error;
use dok3_domain::{DocumentParserPort, DocumentSection, ParsedDocument, SectionId};

/// Supported source code extensions handled by [`CodeSymbolParser`].
pub const CODE_EXTENSIONS: &[&str] = &["rs", "java", "go", "sql", "jsp"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CodeLanguage {
    Rust,
    Java,
    Go,
    Sql,
    Jsp,
}

impl CodeLanguage {
    fn from_filename(filename: &str) -> Option<Self> {
        let ext = Path::new(filename)
            .extension()
            .and_then(|e| e.to_str())?
            .to_ascii_lowercase();
        match ext.as_str() {
            "rs" => Some(Self::Rust),
            "java" => Some(Self::Java),
            "go" => Some(Self::Go),
            "sql" => Some(Self::Sql),
            "jsp" => Some(Self::Jsp),
            _ => None,
        }
    }

    fn from_media_type(media_type: &str) -> Option<Self> {
        let mt = media_type
            .split(';')
            .next()
            .unwrap_or("")
            .trim()
            .to_ascii_lowercase();
        match mt.as_str() {
            "text/x-rust" | "application/x-rust" => Some(Self::Rust),
            "text/x-java-source" | "text/x-java" => Some(Self::Java),
            "text/x-go" => Some(Self::Go),
            "application/sql" | "text/x-sql" => Some(Self::Sql),
            "application/jsp" | "text/x-jsp" => Some(Self::Jsp),
            _ => None,
        }
    }
}

/// Extracts top-level symbols, SQL DDL objects, and JSP scriptlet/page sections
/// without external parser dependencies.
#[derive(Debug, Default, Clone, Copy)]
pub struct CodeSymbolParser;

impl CodeSymbolParser {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl DocumentParserPort for CodeSymbolParser {
    fn supports(&self, media_type: &str, filename: &str) -> bool {
        CodeLanguage::from_filename(filename).is_some()
            || CodeLanguage::from_media_type(media_type).is_some()
    }

    fn parse(&self, filename: &str, content: &[u8]) -> Result<ParsedDocument, Dok3Error> {
        let text = std::str::from_utf8(content)
            .map_err(|err| Dok3Error::Parse(format!("invalid UTF-8 in {filename}: {err}")))?;
        let normalized = normalize_lines(text);
        if normalized.trim().is_empty() {
            return Err(Dok3Error::InvalidInput(format!(
                "empty source file: {filename}"
            )));
        }

        let lang = CodeLanguage::from_filename(filename).unwrap_or(CodeLanguage::Rust);
        let lines: Vec<&str> = normalized.lines().collect();
        let mut warnings = Vec::new();
        let raw_symbols: Vec<(usize, String)> = match lang {
            CodeLanguage::Rust | CodeLanguage::Java | CodeLanguage::Go => {
                extract_treesitter_symbols(lang, &normalized)
            }
            CodeLanguage::Sql | CodeLanguage::Jsp => {
                let mut syms = Vec::new();
                for (idx, line) in lines.iter().enumerate() {
                    let line_no = idx + 1;
                    if let Some(heading) = detect_symbol(lang, line, &mut warnings, line_no) {
                        syms.push((line_no, heading));
                    }
                }
                syms
            }
        };

        let title = Path::new(filename)
            .file_name()
            .and_then(|n| n.to_str())
            .filter(|s| !s.is_empty())
            .unwrap_or(filename)
            .to_string();

        let mut sections = Vec::new();
        let mut anchor_counts: HashMap<String, usize> = HashMap::new();

        if raw_symbols.is_empty() {
            let body = normalized.trim().to_string();
            sections.push(DocumentSection {
                section_id: SectionId::new("body"),
                heading: title.clone(),
                anchor: "body".to_string(),
                order: 1,
                line_start: 1,
                line_end: lines.len(),
                body,
            });
        } else {
            let first_start = raw_symbols[0].0;
            if first_start > 1 {
                let preamble = lines[..first_start - 1].join("\n");
                if !preamble.trim().is_empty() {
                    let anchor = unique_anchor("preamble", &mut anchor_counts);
                    sections.push(DocumentSection {
                        section_id: SectionId::new(anchor.clone()),
                        heading: title.clone(),
                        anchor,
                        order: sections.len() + 1,
                        line_start: 1,
                        line_end: first_start - 1,
                        body: preamble.trim().to_string(),
                    });
                }
            }

            for (i, (start_line, heading)) in raw_symbols.iter().enumerate() {
                let end_line = raw_symbols
                    .get(i + 1)
                    .map_or(lines.len(), |(next_start, _)| next_start - 1);
                let body = lines[*start_line - 1..end_line].join("\n");
                let anchor = unique_anchor(&slugify_anchor(heading), &mut anchor_counts);
                sections.push(DocumentSection {
                    section_id: SectionId::new(anchor.clone()),
                    heading: heading.clone(),
                    anchor,
                    order: sections.len() + 1,
                    line_start: *start_line,
                    line_end: end_line,
                    body: body.trim().to_string(),
                });
            }
        }

        Ok(ParsedDocument {
            title,
            sections,
            normalized_text: normalized,
            parse_warnings: warnings,
        })
    }
}

pub(crate) fn normalize_lines(input: &str) -> String {
    let stripped = input.strip_prefix('\u{feff}').unwrap_or(input);
    let unified = stripped.replace("\r\n", "\n").replace('\r', "\n");
    if unified.ends_with('\n') {
        unified
    } else {
        format!("{unified}\n")
    }
}

pub(crate) fn slugify_anchor(heading: &str) -> String {
    let mut out = String::new();
    let mut last_dash = false;
    for ch in heading.chars() {
        if ch.is_alphanumeric() || ch == '_' {
            for lower in ch.to_lowercase() {
                out.push(lower);
            }
            last_dash = false;
        } else if !last_dash && !out.is_empty() {
            out.push('-');
            last_dash = true;
        }
    }
    let trimmed = out.trim_matches('-').to_string();
    if trimmed.is_empty() {
        "section".to_string()
    } else {
        trimmed
    }
}

pub(crate) fn unique_anchor(base: &str, counts: &mut HashMap<String, usize>) -> String {
    let count = counts.entry(base.to_string()).or_insert(0);
    *count += 1;
    if *count == 1 {
        base.to_string()
    } else {
        format!("{base}-{}", *count - 1)
    }
}

fn detect_symbol(
    lang: CodeLanguage,
    raw_line: &str,
    warnings: &mut Vec<String>,
    line_no: usize,
) -> Option<String> {
    let trimmed = raw_line.trim();
    if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with("--") {
        return None;
    }

    match lang {
        CodeLanguage::Rust => detect_rust_symbol(trimmed),
        CodeLanguage::Java => detect_java_symbol(raw_line, trimmed),
        CodeLanguage::Go => detect_go_symbol(trimmed),
        CodeLanguage::Sql => detect_sql_symbol(trimmed),
        CodeLanguage::Jsp => detect_jsp_section(trimmed, warnings, line_no),
    }
}

fn detect_rust_symbol(trimmed: &str) -> Option<String> {
    let mut rest = trimmed;
    for prefix in ["pub(crate) ", "pub(super) ", "pub ", "async ", "unsafe "] {
        if let Some(after) = rest.strip_prefix(prefix) {
            rest = after.trim_start();
        }
    }
    for kw in ["fn ", "struct ", "enum ", "trait ", "mod ", "impl "] {
        if let Some(after) = rest.strip_prefix(kw) {
            let ident = extract_ident(after.trim_start());
            if !ident.is_empty() {
                return Some(format!("{} {ident}", kw.trim()));
            }
        }
    }
    None
}

fn detect_java_symbol(raw_line: &str, trimmed: &str) -> Option<String> {
    let mut rest = trimmed;
    for prefix in [
        "public ",
        "private ",
        "protected ",
        "static ",
        "final ",
        "abstract ",
        "synchronized ",
    ] {
        while let Some(after) = rest.strip_prefix(prefix) {
            rest = after.trim_start();
        }
    }
    for kw in ["class ", "interface ", "enum ", "record "] {
        if let Some(after) = rest.strip_prefix(kw) {
            let ident = extract_ident(after.trim_start());
            if !ident.is_empty() {
                return Some(format!("{} {ident}", kw.trim()));
            }
        }
    }
    // Indented method declaration heuristic: `<ReturnType> <methodName>(...) {`
    if raw_line.starts_with([' ', '\t'])
        && trimmed.contains('(')
        && (trimmed.ends_with('{') || trimmed.ends_with(')'))
        && !trimmed.starts_with("if ")
        && !trimmed.starts_with("for ")
        && !trimmed.starts_with("while ")
        && !trimmed.starts_with("switch ")
        && !trimmed.starts_with("catch ")
        && let Some(before_paren) = trimmed.split('(').next()
    {
        let parts: Vec<&str> = before_paren.split_whitespace().collect();
        if parts.len() >= 2 {
            let method = extract_ident(parts[parts.len() - 1]);
            if !method.is_empty() {
                return Some(format!("method {method}"));
            }
        }
    }
    None
}

fn detect_go_symbol(trimmed: &str) -> Option<String> {
    if let Some(after) = trimmed.strip_prefix("func ") {
        let after = after.trim_start();
        if let Some(rest) = after.strip_prefix('(')
            && let Some((_, tail)) = rest.split_once(')')
        {
            let ident = extract_ident(tail.trim_start());
            if !ident.is_empty() {
                return Some(format!("func {ident}"));
            }
        }
        let ident = extract_ident(after);
        if !ident.is_empty() {
            return Some(format!("func {ident}"));
        }
    }
    if let Some(after) = trimmed.strip_prefix("type ") {
        let ident = extract_ident(after.trim_start());
        if !ident.is_empty() && (trimmed.contains(" struct") || trimmed.contains(" interface")) {
            return Some(format!("type {ident}"));
        }
    }
    None
}

fn detect_sql_symbol(trimmed: &str) -> Option<String> {
    let upper = trimmed.to_ascii_uppercase();
    let rest = if let Some(r) = upper.strip_prefix("CREATE OR REPLACE ") {
        (trimmed.len() - r.len(), r)
    } else {
        let r = upper.strip_prefix("CREATE ")?;
        (trimmed.len() - r.len(), r)
    };

    let orig_after = &trimmed[rest.0..];
    for kw in ["TABLE ", "VIEW ", "INDEX ", "PROCEDURE ", "FUNCTION "] {
        if let Some(after_kw) = rest.1.strip_prefix(kw) {
            let offset = orig_after.len() - after_kw.len();
            let raw_name = orig_after[offset..].trim_start();
            let name = raw_name
                .split(|c: char| c.is_whitespace() || c == '(' || c == ';')
                .next()
                .unwrap_or("");
            if !name.is_empty() {
                return Some(format!("{} {name}", kw.trim()));
            }
        }
    }
    None
}

fn detect_jsp_section(trimmed: &str, warnings: &mut Vec<String>, line_no: usize) -> Option<String> {
    if trimmed.starts_with("<%@") {
        return Some(format!("jsp directive (line {line_no})"));
    }
    if trimmed.starts_with("<%!") || trimmed.starts_with("<%") {
        warnings.push(format!(
            "line {line_no}: mixed JSP scriptlet/HTML extracted verbatim"
        ));
        return Some(format!("jsp scriptlet (line {line_no})"));
    }
    if trimmed.starts_with("<form") || trimmed.starts_with("<section") {
        return Some(format!("html block (line {line_no})"));
    }
    None
}

fn extract_ident(input: &str) -> &str {
    let end = input
        .find(|c: char| !c.is_alphanumeric() && c != '_' && c != ':')
        .unwrap_or(input.len());
    input[..end].trim_end_matches(':')
}

fn extract_treesitter_symbols(lang: CodeLanguage, source: &str) -> Vec<(usize, String)> {
    let mut parser = tree_sitter::Parser::new();
    let ts_lang = match lang {
        CodeLanguage::Rust => tree_sitter_rust::LANGUAGE.into(),
        CodeLanguage::Java => tree_sitter_java::LANGUAGE.into(),
        CodeLanguage::Go => tree_sitter_go::LANGUAGE.into(),
        _ => return Vec::new(),
    };
    if parser.set_language(&ts_lang).is_err() {
        return Vec::new();
    }
    let Some(tree) = parser.parse(source, None) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    collect_ts_node(lang, tree.root_node(), source.as_bytes(), &mut out);
    out.sort();
    out.dedup();
    out
}

fn collect_ts_node(
    lang: CodeLanguage,
    node: tree_sitter::Node<'_>,
    src: &[u8],
    out: &mut Vec<(usize, String)>,
) {
    let kind = node.kind();
    let line = node.start_position().row + 1;
    let name_text = |field: &str| -> Option<String> {
        node.child_by_field_name(field)
            .and_then(|n| n.utf8_text(src).ok())
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string)
    };
    match lang {
        CodeLanguage::Rust => {
            let label = match kind {
                "function_item" => name_text("name").map(|n| format!("fn {n}")),
                "struct_item" => name_text("name").map(|n| format!("struct {n}")),
                "enum_item" => name_text("name").map(|n| format!("enum {n}")),
                "trait_item" => name_text("name").map(|n| format!("trait {n}")),
                "mod_item" => name_text("name").map(|n| format!("mod {n}")),
                "impl_item" => name_text("type").map(|n| format!("impl {n}")),
                _ => None,
            };
            if let Some(h) = label {
                out.push((line, h));
            }
        }
        CodeLanguage::Java => {
            let label = match kind {
                "class_declaration" => name_text("name").map(|n| format!("class {n}")),
                "interface_declaration" => name_text("name").map(|n| format!("interface {n}")),
                "enum_declaration" => name_text("name").map(|n| format!("enum {n}")),
                "record_declaration" => name_text("name").map(|n| format!("record {n}")),
                "method_declaration" => name_text("name").map(|n| format!("method {n}")),
                _ => None,
            };
            if let Some(h) = label {
                out.push((line, h));
            }
        }
        CodeLanguage::Go => {
            let label = match kind {
                "function_declaration" | "method_declaration" => {
                    name_text("name").map(|n| format!("func {n}"))
                }
                "type_spec" => name_text("name").map(|n| format!("type {n}")),
                _ => None,
            };
            if let Some(h) = label {
                out.push((line, h));
            }
        }
        _ => {}
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_ts_node(lang, child, src, out);
    }
}
