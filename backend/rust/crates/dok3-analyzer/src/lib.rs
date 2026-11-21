//! Shared text analysis for indexing and query normalization.

mod identifier;
mod ngram;

use std::collections::HashSet;

pub use ngram::generate_char_ngrams;

/// Initializes the analyzer. Analysis requires no runtime setup.
pub fn init() {}

/// Controls which additional tokens text analysis emits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnalyzeOptions {
    /// Emit the components of compound code identifiers and structured codes.
    pub split_identifiers: bool,
    /// Emit character bigrams and trigrams in a separate output field.
    pub char_ngrams: bool,
}

impl Default for AnalyzeOptions {
    fn default() -> Self {
        Self {
            split_identifiers: true,
            char_ngrams: true,
        }
    }
}

/// Tokens in input order. Repeated occurrences are retained.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AnalyzedTokens {
    /// Lowercase terms. Each compound is followed by its components.
    pub terms: Vec<String>,
    /// Lowercase character bigrams and trigrams within alphanumeric runs.
    pub char_ngrams: Vec<String>,
}

/// Analyzes Unicode text without locale-specific stemming or normalization.
///
/// Namespace separators are token boundaries. Underscores and code separators
/// inside forms such as `ORA-00904`, `HTTP/2` and `v1.2` are retained
/// in whole terms. Identifier components are derived from the original casing.
pub fn analyze_text(text: &str, options: AnalyzeOptions) -> AnalyzedTokens {
    AnalyzedTokens {
        terms: analyze_terms(text, options.split_identifiers),
        char_ngrams: if options.char_ngrams {
            generate_char_ngrams(text)
        } else {
            Vec::new()
        },
    }
}

/// Returns unique lowercase query terms in first-occurrence order.
///
/// Uses the same whole-term and component rules as default text analysis.
pub fn normalize_query_terms(query: &str) -> Vec<String> {
    let mut seen = HashSet::new();
    analyze_terms(query, true)
        .into_iter()
        .filter(|term| seen.insert(term.clone()))
        .collect()
}

fn analyze_terms(text: &str, split_identifiers: bool) -> Vec<String> {
    let mut terms = Vec::new();
    for word in identifier::words(text) {
        terms.push(word.to_lowercase());
        if split_identifiers {
            terms.extend(identifier::components(word).map(str::to_lowercase));
        }
    }
    terms
}
