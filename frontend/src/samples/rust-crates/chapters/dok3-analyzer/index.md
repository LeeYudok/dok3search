# Crate dok3-analyzer

## Overview

Shared text analysis for indexing and query normalization.

## Items

- `AnalyzeOptions` — Controls which additional tokens text analysis emits.
- `AnalyzedTokens` — Tokens in input order. Repeated occurrences are retained.
- `SectionCandidate` — Lightweight section view used by select_matched_section_id.
- `analyze_text` — Analyzes Unicode text with optional dictionary-backed Korean expansion.
- `generate_char_ngrams` — Returns lowercase bigrams and trigrams from each alphanumeric run.
- `init` — Initializes the analyzer. Analysis requires no runtime setup.
- `normalize_query_terms` — Returns unique lowercase query terms in first-occurrence order.
- `select_matched_section_id` — Selects the best-matching non-empty section for query.
- `word_spans` — Yields analyzer words and their original UTF-8 byte ranges.

### fn dok3-analyzer::analyze_text

```rust
pub fn analyze_text(text: &str, options: AnalyzeOptions) -> AnalyzedTokens
```

Analyzes Unicode text with optional dictionary-backed Korean expansion.

Namespace separators are token boundaries. Underscores and code separators
inside forms such as `ORA-00904`, `HTTP/2` and `v1.2` are retained
in whole terms. Identifier components are derived from the original casing.

### fn dok3-analyzer::generate_char_ngrams

```rust
pub fn generate_char_ngrams(text: &str) -> Vec<String>
```

Returns lowercase bigrams and trigrams from each alphanumeric run.

Windows count Unicode scalar values in the lowercase text. For each start
position, the bigram precedes the trigram. Whitespace and punctuation
terminate a run, and repeated grams are retained.

### fn dok3-analyzer::init

```rust
pub fn init()
```

Initializes the analyzer. Analysis requires no runtime setup.

### fn dok3-analyzer::normalize_query_terms

```rust
pub fn normalize_query_terms(query: &str) -> Vec<String>
```

Returns unique lowercase query terms in first-occurrence order.

Uses the same whole-term and component rules as default text analysis.

### fn dok3-analyzer::select_matched_section_id

```rust
pub fn select_matched_section_id<'a>(
    query: &str,
    sections: &[SectionCandidate<'a>],
) -> Option<&'a str>
```

Selects the best-matching non-empty section for `query`.

Empty-body sections (for example an H1 title immediately followed by an H2 heading) are skipped
whenever at least one section has non-whitespace content. If every section is empty, the first
section is returned as a fallback so section-read endpoints still resolve deterministically.

### fn dok3-analyzer::word_spans

```rust
pub fn word_spans(text: &str) -> impl Iterator<Item = (usize, usize, &str)>
```

Yields analyzer words and their original UTF-8 byte ranges.

### struct dok3-analyzer::AnalyzeOptions

```rust
pub struct AnalyzeOptions {
    pub split_identifiers: bool,
    pub char_ngrams: bool,
}
```

Controls which additional tokens text analysis emits.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### struct dok3-analyzer::AnalyzedTokens

```rust
pub struct AnalyzedTokens {
    pub terms: Vec<String>,
    pub char_ngrams: Vec<String>,
}
```

Tokens in input order. Repeated occurrences are retained.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### struct dok3-analyzer::SectionCandidate

```rust
pub struct SectionCandidate<'a> {
    pub section_id: &'a str,
    pub heading: &'a str,
    pub body: &'a str,
}
```

Lightweight section view used by `select_matched_section_id`.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
