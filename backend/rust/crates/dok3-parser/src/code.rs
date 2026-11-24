//! Line-based source code symbol parser for Rust, Java, Go, SQL, JSP, TypeScript, JavaScript, and Python.

use std::collections::HashMap;
use std::path::Path;

use dok3_core::Dok3Error;
use dok3_domain::{DocumentParserPort, DocumentSection, ParsedDocument, SectionId};

/// Supported source code extensions handled by [`CodeSymbolParser`].
pub const CODE_EXTENSIONS: &[&str] = &[
    "rs", "java", "go", "sql", "jsp", "ts", "tsx", "mts", "cts", "js", "jsx", "mjs", "cjs", "py",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CodeLanguage {
    Rust,
    Java,
    Go,
    Sql,
    Jsp,
    TypeScript,
    JavaScript,
    Python,
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
            "ts" | "tsx" | "mts" | "cts" => Some(Self::TypeScript),
            "js" | "jsx" | "mjs" | "cjs" => Some(Self::JavaScript),
            "py" => Some(Self::Python),
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
            "text/typescript" | "application/typescript" => Some(Self::TypeScript),
            "text/javascript" | "application/javascript" => Some(Self::JavaScript),
            "text/x-python" | "application/x-python-code" => Some(Self::Python),
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
        let mut raw_symbols: Vec<(usize, String)> = Vec::new();

        for (idx, line) in lines.iter().enumerate() {
            let line_no = idx + 1;
            if let Some(heading) = detect_symbol(lang, line, &mut warnings, line_no) {
                raw_symbols.push((line_no, heading));
            }
        }

        let title = Path::new(filename)
            .file_name()
            .and_then(|n| n.to_str())
            .filter(|s| !s.is_empty())
            .unwrap_or(filename)
            .to_string();

        let mut sections = Vec::new();
        let mut anchor_counts: HashMap<String, usize> = HashMap::new();

        let effective_symbols: Vec<(usize, String)> = raw_symbols
            .iter()
            .enumerate()
            .map(|(i, (decl_line, heading))| {
                let min_line = if i == 0 { 1 } else { raw_symbols[i - 1].0 + 1 };
                let start_line = rewind_leading_doc_start(&lines, *decl_line, min_line);
                (start_line, heading.clone())
            })
            .collect();

        if effective_symbols.is_empty() {
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
            let first_start = effective_symbols[0].0;
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

            for (i, (start_line, heading)) in effective_symbols.iter().enumerate() {
                let end_line = effective_symbols
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
        CodeLanguage::TypeScript | CodeLanguage::JavaScript => {
            detect_ts_js_symbol(raw_line, trimmed)
        }
        CodeLanguage::Python => detect_python_symbol(trimmed),
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
    loop {
        let mut stripped = false;
        for prefix in [
            "public ",
            "private ",
            "protected ",
            "static ",
            "final ",
            "abstract ",
            "synchronized ",
            "default ",
            "sealed ",
            "non-sealed ",
        ] {
            if let Some(after) = rest.strip_prefix(prefix) {
                rest = after.trim_start();
                stripped = true;
                break;
            }
        }
        if !stripped {
            break;
        }
    }
    if let Some(after) = rest.strip_prefix("@interface ") {
        let ident = extract_ident(after.trim_start());
        if !ident.is_empty() {
            return Some(format!("interface {ident}"));
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
    // Indented method declaration heuristic: `<ReturnType> <methodName>(...) {` or interface `;`
    if raw_line.starts_with([' ', '\t'])
        && trimmed.contains('(')
        && (trimmed.ends_with('{') || trimmed.ends_with(')') || trimmed.ends_with(';'))
        && ![
            "if ", "if(", "else ", "else{", "for ", "for(", "while ", "while(", "switch ",
            "switch(", "catch ", "catch(", "try ", "try(", "return ", "throw ", "new ", "super(",
            "this(",
        ]
        .iter()
        .any(|kw| trimmed.starts_with(kw))
        && let Some(before_paren) = trimmed.split('(').next()
        && !before_paren.contains('=')
    {
        let parts: Vec<&str> = before_paren.split_whitespace().collect();
        if parts.len() >= 2 {
            let candidate = parts[parts.len() - 1];
            if !candidate.contains('.') && !candidate.contains("::") {
                let method = extract_ident(candidate);
                if !method.is_empty() {
                    return Some(format!("method {method}"));
                }
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

fn extract_html_attr<'a>(tag: &'a str, attr: &str) -> Option<&'a str> {
    let mut rest = tag;
    while let Some(idx) = rest.find(attr) {
        let prev_ok = idx == 0
            || rest.as_bytes()[idx - 1].is_ascii_whitespace()
            || matches!(rest.as_bytes()[idx - 1], b'<' | b'"' | b'\'');
        let after = &rest[idx + attr.len()..];
        if prev_ok {
            let trimmed = after.trim_start();
            if let Some(after_eq) = trimmed.strip_prefix('=') {
                let val_start = after_eq.trim_start();
                if let Some(quote) = val_start.chars().next().filter(|&c| c == '"' || c == '\'') {
                    let inner = &val_start[1..];
                    if let Some(end) = inner.find(quote) {
                        let val = inner[..end].trim();
                        if !val.is_empty() {
                            return Some(val);
                        }
                    }
                }
            }
        }
        rest = after;
    }
    None
}

fn detect_jsp_section(trimmed: &str, warnings: &mut Vec<String>, line_no: usize) -> Option<String> {
    if trimmed.starts_with("<%--") || trimmed.starts_with("<%=") {
        return None;
    }
    if trimmed.starts_with("<%@") {
        let inner = trimmed.trim_start_matches("<%@").trim_start();
        if inner.starts_with("page") {
            return Some("jsp page directive".to_string());
        }
        if inner.starts_with("taglib") {
            if let Some(prefix) = extract_html_attr(inner, "prefix") {
                return Some(format!("jsp taglib {prefix}"));
            }
            return Some("jsp taglib directive".to_string());
        }
        if inner.starts_with("include") {
            if let Some(file) = extract_html_attr(inner, "file") {
                return Some(format!("jsp include {file}"));
            }
            return Some("jsp include directive".to_string());
        }
        return Some(format!("jsp directive (line {line_no})"));
    }
    if trimmed.starts_with("<%!") {
        warnings.push(format!(
            "line {line_no}: mixed JSP declaration/HTML extracted verbatim"
        ));
        return Some(format!("jsp declaration (line {line_no})"));
    }
    if trimmed.starts_with("<%") {
        warnings.push(format!(
            "line {line_no}: mixed JSP scriptlet/HTML extracted verbatim"
        ));
        return Some(format!("jsp scriptlet (line {line_no})"));
    }
    if trimmed.starts_with("<form") {
        if let Some(target) = extract_html_attr(trimmed, "action")
            .or_else(|| extract_html_attr(trimmed, "id"))
            .or_else(|| extract_html_attr(trimmed, "name"))
        {
            return Some(format!("form {target}"));
        }
        return Some(format!("html block (line {line_no})"));
    }
    if trimmed.starts_with("<c:forEach") {
        if let Some(var) =
            extract_html_attr(trimmed, "var").or_else(|| extract_html_attr(trimmed, "items"))
        {
            return Some(format!("jstl c:forEach {var}"));
        }
        return Some(format!("jstl c:forEach (line {line_no})"));
    }
    if trimmed.starts_with("<script") {
        return Some(format!("script block (line {line_no})"));
    }
    if trimmed.starts_with("<section") {
        return Some(format!("html block (line {line_no})"));
    }
    None
}

fn detect_ts_js_symbol(raw_line: &str, trimmed: &str) -> Option<String> {
    if raw_line.starts_with([' ', '\t']) || trimmed.starts_with("/*") || trimmed.starts_with('*') {
        return None;
    }
    if let Some(after) = trimmed.strip_prefix("module.exports") {
        let rhs = after.trim_start();
        if let Some(rhs) = rhs.strip_prefix('=') {
            let rhs = rhs.trim_start();
            let rhs = rhs
                .strip_prefix("async ")
                .map(str::trim_start)
                .unwrap_or(rhs);
            if let Some(after_fn) = rhs.strip_prefix("function ") {
                let ident = extract_ts_py_ident(after_fn.trim_start());
                if !ident.is_empty() {
                    return Some(format!("function {ident}"));
                }
            }
        } else if let Some(member) = rhs.strip_prefix('.') {
            let ident = extract_ts_py_ident(member);
            if !ident.is_empty() && member[ident.len()..].trim_start().starts_with('=') {
                let kind = if member.contains("function") || member.contains("=>") {
                    "function"
                } else {
                    "const"
                };
                return Some(format!("{kind} {ident}"));
            }
        }
    }
    if let Some(member) = trimmed.strip_prefix("exports.") {
        let ident = extract_ts_py_ident(member);
        if !ident.is_empty() && member[ident.len()..].trim_start().starts_with('=') {
            let kind = if member.contains("function") || member.contains("=>") {
                "function"
            } else {
                "const"
            };
            return Some(format!("{kind} {ident}"));
        }
    }
    let mut rest = trimmed;
    loop {
        let mut stripped = false;
        for prefix in ["export ", "default ", "declare ", "abstract ", "async "] {
            if let Some(after) = rest.strip_prefix(prefix) {
                rest = after.trim_start();
                stripped = true;
                break;
            }
        }
        if !stripped {
            break;
        }
    }
    for (kw, label) in [
        ("function* ", "function"),
        ("function ", "function"),
        ("class ", "class"),
        ("interface ", "interface"),
        ("type ", "type"),
        ("enum ", "enum"),
        ("const ", "const"),
        ("let ", "let"),
    ] {
        if let Some(after) = rest.strip_prefix(kw) {
            let ident = extract_ts_py_ident(after.trim_start());
            if !ident.is_empty() {
                let effective_label = if label == "const"
                    && (after.contains("=>")
                        || after.contains("forwardRef")
                        || after.contains("memo("))
                {
                    "function"
                } else {
                    label
                };
                return Some(format!("{effective_label} {ident}"));
            }
        }
    }
    None
}

fn detect_python_symbol(trimmed: &str) -> Option<String> {
    if trimmed.starts_with('#') {
        return None;
    }
    let rest = trimmed
        .strip_prefix("async ")
        .map(str::trim_start)
        .unwrap_or(trimmed);
    for kw in ["def ", "class "] {
        if let Some(after) = rest.strip_prefix(kw) {
            let ident = extract_ts_py_ident(after.trim_start());
            if !ident.is_empty() {
                return Some(format!("{} {ident}", kw.trim()));
            }
        }
    }
    None
}

fn rewind_leading_doc_start(lines: &[&str], decl_line: usize, min_line: usize) -> usize {
    let mut cur = decl_line;
    while cur > min_line {
        let prev = lines[cur - 2].trim();
        if prev.is_empty() {
            break;
        }
        if prev.starts_with("//")
            || prev.starts_with("#[")
            || prev.starts_with('@')
            || prev.starts_with("--")
            || (prev.starts_with('#') && !prev.starts_with("# "))
        {
            cur -= 1;
            continue;
        }
        if prev.ends_with("*/") {
            let mut k = cur - 1;
            let mut found_open = false;
            while k >= min_line {
                let t = lines[k - 1].trim();
                if t.starts_with("/*") {
                    cur = k;
                    found_open = true;
                    break;
                }
                if t.is_empty() || (!t.starts_with('*') && !t.contains("/*")) {
                    break;
                }
                if k == min_line {
                    break;
                }
                k -= 1;
            }
            if found_open {
                continue;
            }
        }
        break;
    }
    cur
}

fn extract_ts_py_ident(input: &str) -> &str {
    let end = input
        .find(|c: char| !c.is_alphanumeric() && c != '_' && c != '$')
        .unwrap_or(input.len());
    &input[..end]
}

fn extract_ident(input: &str) -> &str {
    let end = input
        .find(|c: char| !c.is_alphanumeric() && c != '_' && c != ':')
        .unwrap_or(input.len());
    input[..end].trim_end_matches(':')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_typescript_top_level_symbols_and_ignores_jsdoc_and_indented() {
        let src = br#"/**
 * Helper comment:
 * function fakeInComment() {}
 * const fakeConst = 1;
 */
export interface HwpxOptions {
  strict?: boolean;
}

export async function parseHwpx(buf: Uint8Array): Promise<string> {
  const localHelper = 1;
  return String(localHelper);
}

export class HwpxReader {}

export const DEFAULT_VERSION: string = "4.18.8";
"#;
        let parser = CodeSymbolParser::new();
        assert!(parser.supports("text/typescript", "reader.ts"));
        let parsed = parser.parse("reader.ts", src).unwrap();
        let headings: Vec<&str> = parsed.sections.iter().map(|s| s.heading.as_str()).collect();
        assert_eq!(
            headings,
            vec![
                "interface HwpxOptions",
                "function parseHwpx",
                "class HwpxReader",
                "const DEFAULT_VERSION",
            ]
        );
    }

    #[test]
    fn parses_python_symbols_and_ignores_comments() {
        let src = br#"# def commented_out():
class Parser:
    pass

async def parse_async(data):
    return data
"#;
        let parser = CodeSymbolParser::new();
        assert!(parser.supports("text/x-python", "parser.py"));
        let parsed = parser.parse("parser.py", src).unwrap();
        let headings: Vec<&str> = parsed.sections.iter().map(|s| s.heading.as_str()).collect();
        assert_eq!(
            headings,
            vec!["parser.py", "class Parser", "def parse_async"]
        );
    }

    #[test]
    fn attaches_leading_jsdoc_to_following_symbol_section() {
        let src = br#"export const VERSION = "1.0";

/**
 * Converts Buffer to ArrayBuffer.
 */
export function toArrayBuffer(buf: Buffer): ArrayBuffer {
  return buf.buffer;
}
"#;
        let parser = CodeSymbolParser::new();
        let parsed = parser.parse("utils.ts", src).unwrap();
        let version_sec = parsed
            .sections
            .iter()
            .find(|s| s.heading == "const VERSION")
            .unwrap();
        let fn_sec = parsed
            .sections
            .iter()
            .find(|s| s.heading == "function toArrayBuffer")
            .unwrap();
        assert_eq!(version_sec.line_start, 1);
        assert_eq!(version_sec.line_end, 2);
        assert!(!version_sec.body.contains("Converts Buffer"));
        assert_eq!(fn_sec.line_start, 3);
        assert!(fn_sec.body.starts_with("/**"));
    }

    #[test]
    fn parses_java_interface_methods_records_annotations_and_rejects_calls() {
        let src = br#"package com.doksam.orders;

public @interface Audited {}

public record OrderRow(String orderId, long amountKrw) {}

public interface OrderReader {
    List<OrderRow> findByStatus(String status) throws SQLException;
}

public class OrderRepository implements OrderReader {
    public List<OrderRow> findByStatus(String status) throws SQLException {
        if (status == null) {
            return List.of();
        }
        logger.info("fetching");
        var list = repo.query(status);
        return list;
    }
}
"#;
        let parser = CodeSymbolParser::new();
        let parsed = parser.parse("OrderRepository.java", src).unwrap();
        let headings: Vec<&str> = parsed.sections.iter().map(|s| s.heading.as_str()).collect();
        assert_eq!(
            headings,
            vec![
                "OrderRepository.java",
                "interface Audited",
                "record OrderRow",
                "interface OrderReader",
                "method findByStatus",
                "class OrderRepository",
                "method findByStatus",
            ]
        );
    }

    #[test]
    fn parses_jsp_directives_forms_jstl_scripts_and_ignores_comments_and_inline_el() {
        let src = br#"<%@ page contentType="text/html; charset=UTF-8" %>
<%@ taglib prefix="c" uri="jakarta.tags.core" %>
<%-- hidden JSP comment --%>
<%! public String formatKrw(long v) { return v + " KRW"; } %>
<form id="orderForm" action="/api/orders/search" method="get">
  <input name="q" value="<%= request.getParameter("q") %>" />
</form>
<c:forEach items="${orders}" var="orderRow">
  <div>${orderRow.id}</div>
</c:forEach>
<script>
  console.log("ready");
</script>
"#;
        let parser = CodeSymbolParser::new();
        let parsed = parser.parse("orderList.jsp", src).unwrap();
        let headings: Vec<&str> = parsed.sections.iter().map(|s| s.heading.as_str()).collect();
        assert_eq!(
            headings,
            vec![
                "jsp page directive",
                "jsp taglib c",
                "jsp declaration (line 4)",
                "form /api/orders/search",
                "jstl c:forEach orderRow",
                "script block (line 11)",
            ]
        );
    }

    #[test]
    fn parses_commonjs_exports_and_react_memo_forward_ref_as_functions() {
        let cjs = br#"exports.retryWithBackoff = async (fn, attempts = 3) => fn();
module.exports = function createClient(baseUrl) { return { baseUrl }; };
"#;
        let parser = CodeSymbolParser::new();
        let parsed_cjs = parser.parse("client.cjs", cjs).unwrap();
        let cjs_headings: Vec<&str> = parsed_cjs
            .sections
            .iter()
            .map(|s| s.heading.as_str())
            .collect();
        assert_eq!(
            cjs_headings,
            vec!["function retryWithBackoff", "function createClient"]
        );

        let tsx = br#"export const DEFAULT_POLL_MS = 5000;
export const OrderBadge = memo(forwardRef<HTMLSpanElement, Props>((props, ref) => {
  return <span ref={ref}>{props.status}</span>;
}));
"#;
        let parsed_tsx = parser.parse("OrderBadge.tsx", tsx).unwrap();
        let tsx_headings: Vec<&str> = parsed_tsx
            .sections
            .iter()
            .map(|s| s.heading.as_str())
            .collect();
        assert_eq!(
            tsx_headings,
            vec!["const DEFAULT_POLL_MS", "function OrderBadge"]
        );
    }
}
