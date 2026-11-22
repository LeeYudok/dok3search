//! Adapts shared analysis to Tantivy token streams.

use dok3_analyzer::{AnalyzeOptions, analyze_text, generate_char_ngrams, word_spans};
use tantivy::tokenizer::{Token, TokenStream, Tokenizer};

/// Retains the original byte ranges so snippets can highlight the original identifier.
#[derive(Clone, Default)]
pub(crate) struct TextTokenizer;

impl Tokenizer for TextTokenizer {
    type TokenStream<'a> = AnalyzedStream;

    fn token_stream<'a>(&'a mut self, text: &'a str) -> Self::TokenStream<'a> {
        AnalyzedStream::new(word_spans(text).flat_map(|(from, to, word)| {
            // Components share the identifier's byte range. Tantivy snippets
            // require monotonically increasing offsets.
            analyze_text(
                word,
                AnalyzeOptions {
                    split_identifiers: true,
                    char_ngrams: false,
                },
            )
            .terms
            .into_iter()
            .map(move |term| (from, to, term))
        }))
    }
}

/// The gram field stores basic postings and is never used to generate snippets.
#[derive(Clone, Default)]
pub(crate) struct GramTokenizer;

impl Tokenizer for GramTokenizer {
    type TokenStream<'a> = AnalyzedStream;

    fn token_stream<'a>(&'a mut self, text: &'a str) -> Self::TokenStream<'a> {
        let grams = generate_char_ngrams(text);
        AnalyzedStream::new(grams.into_iter().map(|gram| (0, text.len(), gram)))
    }
}

pub(crate) struct AnalyzedStream {
    tokens: std::vec::IntoIter<Token>,
    current: Token,
}

impl AnalyzedStream {
    fn new(tokens: impl Iterator<Item = (usize, usize, String)>) -> Self {
        let tokens: Vec<_> = tokens
            .enumerate()
            .map(|(position, (offset_from, offset_to, text))| Token {
                offset_from,
                offset_to,
                position,
                text,
                position_length: 1,
            })
            .collect();
        Self {
            tokens: tokens.into_iter(),
            current: Token::default(),
        }
    }
}

impl TokenStream for AnalyzedStream {
    fn advance(&mut self) -> bool {
        if let Some(token) = self.tokens.next() {
            self.current = token;
            true
        } else {
            false
        }
    }

    fn token(&self) -> &Token {
        &self.current
    }

    fn token_mut(&mut self) -> &mut Token {
        &mut self.current
    }
}
