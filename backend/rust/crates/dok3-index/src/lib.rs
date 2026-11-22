//! Tantivy schema, custom tokenizers, and reader/writer index adapter (P1-3).

mod index;
mod schema;
mod tokenizer;

pub use index::TantivyIndex;
pub use schema::{IndexFields, SCHEMA_VERSION, TOKENIZER_NGRAM, TOKENIZER_TEXT};
