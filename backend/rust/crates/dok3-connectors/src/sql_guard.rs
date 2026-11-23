//! SQL-aware read-only SELECT/CTE validator for database source connectors.

use dok3_core::Dok3Error;

const FORBIDDEN_KEYWORDS: &[&str] = &[
    "INSERT", "UPDATE", "DELETE", "DROP", "ALTER", "CREATE", "TRUNCATE", "REPLACE", "MERGE",
    "ATTACH", "DETACH", "PRAGMA", "CALL", "EXEC", "EXECUTE", "GRANT", "REVOKE", "VACUUM",
    "REINDEX", "ANALYZE",
];

/// Validates that `query` is a single read-only `SELECT` or `WITH ... SELECT` statement
/// and strips at most one trailing semicolon.
///
/// Semicolons and keywords inside string literals (`'...'`), quoted identifiers
/// (`"..."`, `` `...` ``), line comments (`-- ...`), and block comments (`/* ... */`)
/// are ignored rather than misclassified as statement separators or DML.
pub fn validate_readonly_select(query: &str) -> Result<String, Dok3Error> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return Err(Dok3Error::InvalidInput(
            "select_query must not be empty".to_string(),
        ));
    }

    let tokens = scan_sql_tokens(trimmed)?;
    if tokens.is_empty() {
        return Err(Dok3Error::InvalidInput(
            "select_query must not be empty".to_string(),
        ));
    }

    let mut cut_at = trimmed.len();
    for (idx, tok) in tokens.iter().enumerate() {
        if let Token::Semicolon(pos) = tok {
            if idx + 1 < tokens.len() {
                return Err(Dok3Error::InvalidInput(
                    "select_query must contain exactly one SELECT statement".to_string(),
                ));
            }
            cut_at = *pos;
        }
    }

    let meaningful: Vec<&Token> = tokens
        .iter()
        .filter(|t| !matches!(t, Token::Semicolon(_)))
        .collect();
    let Some(first) = meaningful.first() else {
        return Err(Dok3Error::InvalidInput(
            "select_query must not be empty".to_string(),
        ));
    };

    let first_word = match first {
        Token::Word(w) => w.to_ascii_uppercase(),
        _ => {
            return Err(Dok3Error::InvalidInput(
                "select_query must start with SELECT or WITH".to_string(),
            ));
        }
    };
    if first_word != "SELECT" && first_word != "WITH" {
        return Err(Dok3Error::InvalidInput(
            "select_query is SELECT-only; write and DDL statements are rejected".to_string(),
        ));
    }

    let mut has_top_level_select = first_word == "SELECT";
    let mut prev_into = false;
    for tok in &meaningful {
        match tok {
            Token::Word(w) => {
                let upper = w.to_ascii_uppercase();
                if FORBIDDEN_KEYWORDS.contains(&upper.as_str()) {
                    return Err(Dok3Error::InvalidInput(format!(
                        "select_query rejects forbidden SQL keyword '{upper}'"
                    )));
                }
                if prev_into && upper == "OUTFILE" {
                    return Err(Dok3Error::InvalidInput(
                        "select_query rejects INTO OUTFILE".to_string(),
                    ));
                }
                prev_into = upper == "INTO";
                if upper == "SELECT" {
                    has_top_level_select = true;
                }
            }
            Token::Other(_) | Token::Semicolon(_) => {
                prev_into = false;
            }
        }
    }

    if !has_top_level_select {
        return Err(Dok3Error::InvalidInput(
            "select_query must contain a SELECT clause".to_string(),
        ));
    }

    Ok(trimmed[..cut_at].trim_end().to_string())
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Word(String),
    Semicolon(usize),
    Other(char),
}

fn scan_sql_tokens(input: &str) -> Result<Vec<Token>, Dok3Error> {
    let bytes = input.as_bytes();
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < bytes.len() {
        let b = bytes[i];
        if b.is_ascii_whitespace() {
            i += 1;
            continue;
        }
        // Line comment: -- ...
        if b == b'-' && bytes.get(i + 1) == Some(&b'-') {
            i += 2;
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        // Block comment: /* ... */
        if b == b'/' && bytes.get(i + 1) == Some(&b'*') {
            i += 2;
            let mut closed = false;
            while i + 1 < bytes.len() {
                if bytes[i] == b'*' && bytes[i + 1] == b'/' {
                    i += 2;
                    closed = true;
                    break;
                }
                i += 1;
            }
            if !closed {
                return Err(Dok3Error::InvalidInput(
                    "unterminated SQL block comment in select_query".to_string(),
                ));
            }
            continue;
        }
        // String literal or quoted identifier: '...', "...", `...`
        if matches!(b, b'\'' | b'"' | b'`') {
            let quote = b;
            i += 1;
            let mut closed = false;
            while i < bytes.len() {
                if bytes[i] == quote {
                    if bytes.get(i + 1) == Some(&quote) {
                        i += 2;
                        continue;
                    }
                    i += 1;
                    closed = true;
                    break;
                }
                if quote == b'\'' && bytes[i] == b'\\' && i + 1 < bytes.len() {
                    i += 2;
                    continue;
                }
                i += 1;
            }
            if !closed {
                return Err(Dok3Error::InvalidInput(
                    "unterminated string or identifier literal in select_query".to_string(),
                ));
            }
            tokens.push(Token::Other('q'));
            continue;
        }
        if b == b';' {
            tokens.push(Token::Semicolon(i));
            i += 1;
            continue;
        }
        if b.is_ascii_alphabetic() || b == b'_' {
            let start = i;
            i += 1;
            while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            tokens.push(Token::Word(input[start..i].to_string()));
            continue;
        }
        let ch = input[i..].chars().next().unwrap_or(' ');
        tokens.push(Token::Other(ch));
        i += ch.len_utf8();
    }

    Ok(tokens)
}
