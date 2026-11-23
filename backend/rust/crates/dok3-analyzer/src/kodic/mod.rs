//! Korean morphological analysis over mecab-ko-dic, without third-party crates.
//!
//! - [`Dictionary::compile`] reads a mecab-ko-dic source directory (`char.def`, `unk.def`,
//!   `matrix.def`, `*.csv`) and [`Dictionary::save`] writes it as one binary file.
//! - [`Dictionary::load`] reads that file back; [`UserDictionary`] adds CSV entries.
//! - [`Analyzer`] segments text into [`Morpheme`]s with byte offsets, token positions and
//!   the entry's [`Features`] (POS, reading, type, expression...).
//!
//! Segmentation reproduces lindera 6.2 with its embedded ko-dic in `Mode::Normal`
//! (including the mecab-ko left-space penalty), so an index built with lindera keeps
//! matching queries analyzed here. Text normalization (NFKC) is left to the caller.
//!
//! ```no_run
//! use std::path::Path;
//! use dok3_analyzer::kodic::Analyzer;
//!
//! let analyzer = Analyzer::load(Path::new("dict/ko-dic.bin"), None)?;
//! let text = "계좌를 조회하는";
//! for m in analyzer.analyze(text) {
//!     println!("{}\t{}", m.surface(text), analyzer.features(m.word).pos());
//! }
//! # Ok::<(), dok3_analyzer::kodic::Error>(())
//! ```

mod analyzer;
mod chardef;
mod csv;
mod dictionary;
mod error;
mod format;
mod lexicon;
mod source;
mod trie;
mod user;

pub use analyzer::{Analyzer, Features, Lattice, Morpheme, WordId};
pub use dictionary::{Dictionary, DictionaryInfo};
pub use error::{Error, Result};
pub use user::UserDictionary;

/// Version of the compiled dictionary file format. A file of another version must be
/// rebuilt from the mecab-ko-dic source.
pub const FORMAT_VERSION: u32 = format::VERSION;
