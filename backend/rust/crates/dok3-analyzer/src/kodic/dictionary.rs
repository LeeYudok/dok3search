//! The compiled system dictionary: character categories, unknown-word templates,
//! connection costs, words and left-space penalty rules.

use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};
use std::path::Path;

use super::chardef::{Category, CharDef, CharRange};
use super::error::{Error, Result};
use super::lexicon::{Entry, FeatureTable, Lexicon};

/// Connection costs. `row(left_id)[right_id]` is the cost of a word whose right context id
/// is `right_id` followed by a word whose left context id is `left_id`.
pub(crate) struct Matrix {
    /// Number of right context ids (first number of the matrix.def header).
    pub right_size: usize,
    /// Number of left context ids (second number of the matrix.def header).
    pub left_size: usize,
    /// `costs[right_id + left_id * right_size]`.
    pub costs: Vec<i16>,
}

impl Matrix {
    #[inline]
    pub fn row(&self, left_id: u16) -> &[i16] {
        let start = left_id as usize * self.right_size;
        &self.costs[start..start + self.right_size]
    }
}

/// Unknown-word templates from unk.def.
pub(crate) struct Unknown {
    pub entries: Vec<Entry>,
    /// Category id per entry (`u16::MAX` when the name matches no category).
    pub categories: Vec<u16>,
    pub features: FeatureTable,
    /// Entry ids per category id, in file order. Derived at load.
    pub by_category: Vec<Vec<u32>>,
}

/// Left-space penalty rules: a candidate word that starts right after a whitespace
/// character gets `cost` added when the first tag of its POS (`VV` of `VV+EC`) is listed.
/// The first matching rule wins.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SpacePenalty {
    pub rules: Vec<(i32, Vec<String>)>,
}

impl SpacePenalty {
    /// mecab-ko-dic's `left-space-penalty-factor` (dicrc), expressed by POS the way
    /// lindera-ko-dic 6.2 ships it: endings and suffixes 3000, particles 6000.
    pub fn ko_dic() -> Self {
        let tags = |list: &[&str]| list.iter().map(|t| t.to_string()).collect();
        SpacePenalty {
            rules: vec![
                (
                    3000,
                    tags(&["EC", "EF", "EP", "ETM", "ETN", "VCP", "XSA", "XSN", "XSV"]),
                ),
                (
                    6000,
                    tags(&["JC", "JKB", "JKC", "JKG", "JKO", "JKQ", "JKS", "JKV", "JX"]),
                ),
            ],
        }
    }

    /// Rule class of a POS tag: 0 = no penalty, `k + 1` = rule `k`.
    pub fn class_of(&self, pos: &str) -> u8 {
        let first = pos.split('+').next().unwrap_or(pos);
        self.rules
            .iter()
            .position(|(_, tags)| tags.iter().any(|t| t == first))
            .map_or(0, |k| (k + 1) as u8)
    }

    #[inline]
    pub fn cost(&self, class: u8) -> i32 {
        if class == 0 {
            0
        } else {
            self.rules[class as usize - 1].0
        }
    }

    pub fn classes(&self, features: &FeatureTable) -> Vec<u8> {
        (0..features.len())
            .map(|i| self.class_of(first_field(features.get(i))))
            .collect()
    }
}

/// Distinct part-of-speech tags (the first feature column as [`first_field`] reads it, so
/// `VV+EC` is one tag) in order of first appearance; tag 0 is `*`. Entries refer to them by
/// index, so filtering tokens by tag does not read the feature text.
#[derive(Clone, Debug)]
pub(crate) struct PosTags {
    pub names: Vec<String>,
    index: HashMap<String, u16, BuildHasherDefault<Fnv>>,
}

impl PosTags {
    pub fn new() -> Self {
        let mut tags = PosTags {
            names: Vec::new(),
            index: HashMap::default(),
        };
        tags.names.push("*".into());
        tags.index.insert("*".into(), 0);
        tags
    }

    /// The tag index of every entry of `features`, adding new tags.
    pub fn index_all(&mut self, features: &FeatureTable) -> Result<Vec<u16>> {
        (0..features.len())
            .map(|i| self.intern(first_field(features.get(i))))
            .collect()
    }

    fn intern(&mut self, tag: &str) -> Result<u16> {
        if let Some(&id) = self.index.get(tag) {
            return Ok(id);
        }
        let id = u16::try_from(self.names.len())
            .map_err(|_| Error::Format("more than 65536 distinct part-of-speech tags".into()))?;
        self.names.push(tag.to_owned());
        self.index.insert(tag.to_owned(), id);
        Ok(id)
    }

    /// Left-space penalty class of every tag.
    fn penalty_classes(&self, penalty: &SpacePenalty) -> Vec<u8> {
        self.names.iter().map(|tag| penalty.class_of(tag)).collect()
    }
}

/// FNV-1a: the tag keys are a few bytes long, where it is much faster than SipHash.
#[derive(Default)]
pub(crate) struct Fnv(u64);

impl Hasher for Fnv {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        let mut h = if self.0 == 0 {
            0xcbf2_9ce4_8422_2325
        } else {
            self.0
        };
        for &b in bytes {
            h = (h ^ b as u64).wrapping_mul(0x0000_0100_0000_01b3);
        }
        self.0 = h;
    }
}

/// First feature (the POS tag). Missing features read as `*`, as lindera pads them.
pub(crate) fn first_field(features: &str) -> &str {
    if features.is_empty() {
        "*"
    } else {
        features.split(',').next().unwrap_or("*")
    }
}

/// A compiled mecab-ko-dic system dictionary. Build it once with [`Dictionary::compile`],
/// store it with [`Dictionary::save`] and load it with [`Dictionary::load`]. Immutable and
/// shareable across threads.
pub struct Dictionary {
    pub(crate) label: String,
    pub(crate) chardef: CharDef,
    pub(crate) unknown: Unknown,
    pub(crate) matrix: Matrix,
    pub(crate) words: Lexicon,
    pub(crate) penalty: SpacePenalty,
    pub(crate) word_penalty: Vec<u8>,
    pub(crate) unknown_penalty: Vec<u8>,
    pub(crate) pos: PosTags,
    pub(crate) word_pos: Vec<u16>,
    pub(crate) unknown_pos: Vec<u16>,
}

/// Sizes of a dictionary, for logs and `dict build` output.
#[derive(Clone, Debug)]
pub struct DictionaryInfo {
    /// Source label (the mecab-ko-dic directory name the dictionary was compiled from).
    pub label: String,
    /// Dictionary entries (CSV rows).
    pub entries: usize,
    /// Distinct surfaces.
    pub surfaces: usize,
    pub trie_nodes: usize,
    pub unknown_entries: usize,
    pub categories: usize,
    pub left_ids: usize,
    pub right_ids: usize,
}

impl Dictionary {
    /// Compiles a mecab-ko-dic source directory (char.def, unk.def, matrix.def, *.csv).
    pub fn compile(src_dir: &Path) -> Result<Dictionary> {
        super::source::compile(src_dir)
    }

    /// Loads a dictionary written by [`Dictionary::save`].
    pub fn load(path: &Path) -> Result<Dictionary> {
        let bytes = std::fs::read(path).map_err(|e| Error::io(path, e))?;
        Dictionary::from_bytes(&bytes)
    }

    /// Writes the dictionary to `path` (through a temporary file, so readers never see a
    /// half-written file). Returns the file size.
    pub fn save(&self, path: &Path) -> Result<u64> {
        let bytes = self.to_bytes();
        if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
            std::fs::create_dir_all(parent).map_err(|e| Error::io(parent, e))?;
        }
        let mut tmp = path.as_os_str().to_owned();
        tmp.push(".tmp");
        let tmp = std::path::PathBuf::from(tmp);
        std::fs::write(&tmp, &bytes).map_err(|e| Error::io(&tmp, e))?;
        std::fs::rename(&tmp, path).map_err(|e| Error::io(path, e))?;
        Ok(bytes.len() as u64)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Dictionary> {
        super::format::read(bytes)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        super::format::write(self)
    }

    pub fn info(&self) -> DictionaryInfo {
        let trie = &self.words.trie;
        DictionaryInfo {
            label: self.label.clone(),
            entries: self.words.entries.len(),
            surfaces: trie.surface_count(),
            trie_nodes: trie.node_count(),
            unknown_entries: self.unknown.entries.len(),
            categories: self.chardef.categories.len(),
            left_ids: self.matrix.left_size,
            right_ids: self.matrix.right_size,
        }
    }

    /// Validates the parts and derives the lookup tables.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_parts(
        label: String,
        categories: Vec<Category>,
        ranges: Vec<CharRange>,
        unknown_entries: Vec<Entry>,
        unknown_categories: Vec<u16>,
        unknown_features: FeatureTable,
        matrix: Matrix,
        words: Lexicon,
        penalty: SpacePenalty,
    ) -> Result<Dictionary> {
        if matrix.left_size == 0
            || matrix.right_size == 0
            || matrix.left_size > 1 << 16
            || matrix.right_size > 1 << 16
        {
            return Err(Error::Format(
                "connection matrix has an invalid size".into(),
            ));
        }
        if matrix.costs.len() != matrix.left_size * matrix.right_size {
            return Err(Error::Format(
                "connection matrix size does not match its header".into(),
            ));
        }
        if penalty.rules.len() >= u8::MAX as usize {
            return Err(Error::Format("too many space penalty rules".into()));
        }
        let chardef = CharDef::new(categories, ranges)?;
        words.validate(matrix.left_size, matrix.right_size)?;
        let n_unknown = unknown_entries.len();
        if unknown_categories.len() != n_unknown {
            return Err(Error::Format(
                "unknown-word table lengths do not match".into(),
            ));
        }
        unknown_features.validate(n_unknown)?;
        if unknown_entries.iter().any(|e| {
            e.left_id as usize >= matrix.left_size || e.right_id as usize >= matrix.right_size
        }) {
            return Err(Error::Format(
                "unknown-word context id outside the connection matrix".into(),
            ));
        }
        let mut by_category = vec![Vec::new(); chardef.categories.len()];
        for (id, &cat) in unknown_categories.iter().enumerate() {
            if let Some(list) = by_category.get_mut(cat as usize) {
                list.push(id as u32);
            }
        }
        let mut pos = PosTags::new();
        let word_pos = pos.index_all(&words.features)?;
        let unknown_pos = pos.index_all(&unknown_features)?;
        let by_tag = pos.penalty_classes(&penalty);
        let word_penalty = word_pos.iter().map(|&t| by_tag[t as usize]).collect();
        let unknown_penalty = unknown_pos.iter().map(|&t| by_tag[t as usize]).collect();
        Ok(Dictionary {
            label,
            chardef,
            unknown: Unknown {
                entries: unknown_entries,
                categories: unknown_categories,
                features: unknown_features,
                by_category,
            },
            matrix,
            words,
            penalty,
            word_penalty,
            unknown_penalty,
            pos,
            word_pos,
            unknown_pos,
        })
    }
}
