//! Shared text analysis for indexing and query normalization.

mod identifier;
pub mod kodic;
mod korean;
mod ngram;
mod profile;

use std::collections::HashSet;

pub use ngram::generate_char_ngrams;
pub use profile::{
    AnalyzerConfig, AnalyzerEngine, AnalyzerProfile, AnalyzerRuntimeInfo, BUILTIN_ANALYZER_VERSION,
    active_analyzer_version, active_profile, configure_runtime,
};

/// Initializes the analyzer. Analysis requires no runtime setup.
pub fn init() {}

/// Controls which additional tokens text analysis emits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnalyzeOptions {
    /// Emit identifier components, Korean stems, and Korean compound components.
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

/// Analyzes Unicode text with optional dictionary-backed Korean expansion.
///
/// Namespace separators are token boundaries. Underscores and code separators
/// inside forms such as `ORA-00904`, `HTTP/2` and `v1.2` are retained
/// in whole terms. Identifier components are derived from the original casing.
pub fn analyze_text(text: &str, options: AnalyzeOptions) -> AnalyzedTokens {
    profile::current_engine().analyze_text(text, options)
}

/// Returns unique lowercase query terms in first-occurrence order.
///
/// Uses the same whole-term and component rules as default text analysis.
pub fn normalize_query_terms(query: &str) -> Vec<String> {
    profile::current_engine().normalize_query_terms(query)
}

/// Yields analyzer words and their original UTF-8 byte ranges.
pub fn word_spans(text: &str) -> impl Iterator<Item = (usize, usize, &str)> {
    identifier::words(text).map(move |word| {
        let start = word.as_ptr() as usize - text.as_ptr() as usize;
        (start, start + word.len(), word)
    })
}

/// Lightweight section view used by [`select_matched_section_id`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SectionCandidate<'a> {
    pub section_id: &'a str,
    pub heading: &'a str,
    pub body: &'a str,
}

/// Selects the best-matching non-empty section for `query`.
///
/// Empty-body sections (for example an H1 title immediately followed by an H2 heading) are skipped
/// whenever at least one section has non-whitespace content. If every section is empty, the first
/// section is returned as a fallback so section-read endpoints still resolve deterministically.
#[must_use]
pub fn select_matched_section_id<'a>(
    query: &str,
    sections: &[SectionCandidate<'a>],
) -> Option<&'a str> {
    let non_empty: Vec<&SectionCandidate<'a>> = sections
        .iter()
        .filter(|s| !s.body.trim().is_empty())
        .collect();
    if non_empty.is_empty() {
        return sections.first().map(|s| s.section_id);
    }

    let trimmed = query.trim();
    if trimmed.is_empty() {
        return Some(non_empty[0].section_id);
    }

    let q_lower = trimmed.to_lowercase();
    let analyzed_query = analyze_text(
        trimmed,
        AnalyzeOptions {
            split_identifiers: true,
            char_ngrams: true,
        },
    );
    let mut query_terms = Vec::new();
    let mut seen_q = HashSet::new();
    for term in analyzed_query.terms {
        if seen_q.insert(term.clone()) {
            query_terms.push(term);
        }
    }

    let mut scores = vec![0.0_f32; non_empty.len()];
    for (idx, cand) in non_empty.iter().enumerate() {
        let h_lower = cand.heading.to_lowercase();
        let b_lower = cand.body.to_lowercase();
        let mut h_terms: Option<HashSet<String>> = None;
        let mut b_terms: Option<HashSet<String>> = None;

        let mut score = 0.0_f32;
        if h_lower == q_lower || cand.section_id.eq_ignore_ascii_case(&q_lower) {
            score += 20.0;
        }
        if q_lower.chars().count() >= 2 && h_lower.contains(&q_lower) {
            score += 10.0;
        }
        if q_lower.chars().count() >= 2 && b_lower.contains(&q_lower) {
            score += 5.0;
        }

        for term in &query_terms {
            let is_code_or_err = term.contains("::")
                || term.contains('_')
                || term.contains('-')
                || term.chars().any(|c| c.is_ascii_digit());
            let h_match = h_lower.contains(term.as_str())
                || (!term.is_ascii()
                    && h_terms
                        .get_or_insert_with(|| {
                            analyze_text(
                                cand.heading,
                                AnalyzeOptions {
                                    split_identifiers: true,
                                    char_ngrams: false,
                                },
                            )
                            .terms
                            .into_iter()
                            .collect()
                        })
                        .contains(term));
            if h_match {
                score += if is_code_or_err { 6.0 } else { 4.0 };
            }
            let b_match = b_lower.contains(term.as_str())
                || (!term.is_ascii()
                    && b_terms
                        .get_or_insert_with(|| {
                            analyze_text(
                                cand.body,
                                AnalyzeOptions {
                                    split_identifiers: true,
                                    char_ngrams: false,
                                },
                            )
                            .terms
                            .into_iter()
                            .collect()
                        })
                        .contains(term));
            if b_match {
                score += if is_code_or_err { 3.0 } else { 2.0 };
                let extra_hits = b_lower
                    .matches(term.as_str())
                    .count()
                    .saturating_sub(1)
                    .min(4);
                score += 0.25 * (extra_hits as f32);
            }
        }
        scores[idx] = score;
    }

    if scores.iter().all(|&s| s == 0.0) && !analyzed_query.char_ngrams.is_empty() {
        let mut unique_grams = Vec::new();
        let mut seen_g = HashSet::new();
        for gram in analyzed_query.char_ngrams {
            if seen_g.insert(gram.clone()) {
                unique_grams.push(gram);
            }
        }
        for (idx, cand) in non_empty.iter().enumerate() {
            let h_lower = cand.heading.to_lowercase();
            let b_lower = cand.body.to_lowercase();
            let mut gram_score = 0.0_f32;
            for gram in &unique_grams {
                if h_lower.contains(gram.as_str()) {
                    gram_score += 1.0;
                }
                if b_lower.contains(gram.as_str()) {
                    gram_score += 0.5;
                }
            }
            scores[idx] = gram_score;
        }
    }

    let mut best_idx = 0usize;
    let mut best_score = 0.0_f32;
    for (idx, &score) in scores.iter().enumerate() {
        if score > best_score {
            best_score = score;
            best_idx = idx;
        }
    }

    Some(non_empty[best_idx].section_id)
}

#[cfg(test)]
mod tests {
    use super::{SectionCandidate, select_matched_section_id};

    #[test]
    fn skips_empty_h1_section_and_prefers_matching_subsection() {
        let sections = [
            SectionCandidate {
                section_id: "title",
                heading: "Title",
                body: "",
            },
            SectionCandidate {
                section_id: "overview",
                heading: "Overview",
                body: "General introduction.",
            },
            SectionCandidate {
                section_id: "struct-stdvecvec",
                heading: "struct std::vec::Vec",
                body: "A contiguous growable array type with heap-allocated contents.",
            },
            SectionCandidate {
                section_id: "fn-tokio-task-spawn",
                heading: "fn tokio::task::spawn",
                body: "Spawns a new asynchronous task, returning a JoinHandle.",
            },
        ];

        assert_eq!(select_matched_section_id("", &sections), Some("overview"));
        assert_eq!(
            select_matched_section_id("Vec", &sections),
            Some("struct-stdvecvec")
        );
        assert_eq!(
            select_matched_section_id("tokio::task::spawn", &sections),
            Some("fn-tokio-task-spawn")
        );
        assert_eq!(
            select_matched_section_id("JoinHandle", &sections),
            Some("fn-tokio-task-spawn")
        );
    }

    #[test]
    fn matches_error_code_korean_compound_and_falls_back_when_all_empty() {
        let err_sections = [
            SectionCandidate {
                section_id: "summary",
                heading: "Summary",
                body: "Ownership move error.",
            },
            SectionCandidate {
                section_id: "explanation",
                heading: "Explanation",
                body: "Error E0382 occurs when a moved value is borrowed or used again.",
            },
            SectionCandidate {
                section_id: "세대전환",
                heading: "세대전환 절차",
                body: "블루-그린 세대전환 시 활성 리더를 검증합니다.",
            },
        ];
        assert_eq!(
            select_matched_section_id("E0382", &err_sections),
            Some("explanation")
        );
        assert_eq!(
            select_matched_section_id("세대전환은", &err_sections),
            Some("세대전환")
        );

        let all_empty = [
            SectionCandidate {
                section_id: "lexical-structure",
                heading: "Lexical structure",
                body: "   ",
            },
            SectionCandidate {
                section_id: "second",
                heading: "Second",
                body: "",
            },
        ];
        assert_eq!(
            select_matched_section_id("anything", &all_empty),
            Some("lexical-structure")
        );
    }
}
