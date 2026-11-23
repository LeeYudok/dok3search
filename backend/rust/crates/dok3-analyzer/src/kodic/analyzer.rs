//! Viterbi segmentation over the dictionary lattice.
//!
//! Reproduces lindera 6.2 `Segmenter` in `Mode::Normal` with ko-dic's defaults: sentences
//! split at `\n`, `\t`, `。`, `、` or every 32 KiB; system and user words found by prefix
//! lookup at each reachable position; unknown words from char.def (invoke / group / length
//! ladder, no grouping cap); the left-space penalty; whitespace tokens dropped. Candidate
//! order and the strict `<` comparisons decide ties exactly as lindera does, so equal-cost
//! paths resolve to the same tokens.

use std::path::Path;
use std::sync::Arc;

use super::dictionary::{Dictionary, PosTags, first_field};
use super::error::{Error, Result};
use super::lexicon::{Entry, Lexicon};
use super::user::UserDictionary;

/// Longest sentence handed to the lattice, in bytes (lindera `MAX_SENTENCE_BYTES`).
const MAX_SENTENCE_BYTES: usize = 32 * 1024;

/// Upper bound for stored path costs (lindera `PATH_COST_CLAMP`).
const PATH_COST_CLAMP: i32 = i32::MAX - 131_072;

const NO_LEFT: u32 = u32::MAX;

/// Which word list a morpheme comes from, and its index there.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WordId {
    System(u32),
    User(u32),
    Unknown(u32),
}

/// One segment of the analyzed text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Morpheme {
    /// Byte offsets into the analyzed text.
    pub start: usize,
    pub end: usize,
    /// Token position: counts kept morphemes, whitespace excluded.
    pub position: usize,
    pub word: WordId,
}

impl Morpheme {
    pub fn surface<'t>(&self, text: &'t str) -> &'t str {
        text.get(self.start..self.end).unwrap_or("")
    }
}

/// Feature columns of a dictionary entry: `POS,semantic,jongseong,reading,type,first_pos,
/// last_pos,expression` for ko-dic. Missing columns read as `*`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Features<'a>(&'a str);

impl<'a> Features<'a> {
    /// All columns joined with `,`.
    pub fn as_str(&self) -> &'a str {
        self.0
    }

    pub fn get(&self, index: usize) -> &'a str {
        if self.0.is_empty() {
            return "*";
        }
        self.0.split(',').nth(index).unwrap_or("*")
    }

    /// Part-of-speech tag, e.g. `NNG` or `VV+EC`.
    pub fn pos(&self) -> &'a str {
        first_field(self.0)
    }

    /// Entry type: `Inflect`, `Compound`, `Preanalysis` or `*`.
    pub fn kind(&self) -> &'a str {
        self.get(4)
    }

    /// Decomposition of an `Inflect`/`Compound`/`Preanalysis` entry, e.g.
    /// `갔/VV/*+었/EP/*`, or `*`.
    pub fn expression(&self) -> &'a str {
        self.get(7)
    }
}

/// Segments text with a system dictionary and an optional user dictionary. Cloning is
/// cheap: the dictionaries are shared.
#[derive(Clone)]
pub struct Analyzer {
    inner: Arc<Inner>,
}

struct Inner {
    dict: Arc<Dictionary>,
    user: Option<UserDictionary>,
    user_penalty: Vec<u8>,
    /// The dictionary's tags plus those only the user dictionary uses.
    pos: PosTags,
    user_pos: Vec<u16>,
}

/// Reusable buffers for [`Analyzer::analyze_into`]. One per thread.
#[derive(Default)]
pub struct Lattice {
    chars: Vec<char>,
    /// Byte offset of each char in the sentence, plus the sentence length.
    offsets: Vec<u32>,
    /// Category list (offset, length) of each char.
    categories: Vec<(u32, u16)>,
    /// Whether each char counts as whitespace for the left-space penalty.
    space: Vec<bool>,
    /// `ends_at[i]`: edges ending at char position `i` (0 = BOS).
    ends_at: Vec<Vec<Edge>>,
    used_slots: usize,
    matches: Vec<(u32, u32, u32)>,
    path: Vec<(u32, WordId)>,
}

#[derive(Clone, Copy)]
struct Edge {
    path_cost: i32,
    left_index: u32,
    start: u32,
    word: WordId,
    right_id: u16,
}

impl Analyzer {
    /// Combines a system dictionary with an optional user dictionary. Fails when a user
    /// entry's context ids lie outside the system dictionary's connection matrix.
    pub fn new(dict: Arc<Dictionary>, user: Option<UserDictionary>) -> Result<Analyzer> {
        let mut user_penalty = Vec::new();
        let mut pos = dict.pos.clone();
        let mut user_pos = Vec::new();
        if let Some(user) = &user {
            user.lexicon
                .validate(dict.matrix.left_size, dict.matrix.right_size)
                .map_err(|e| Error::Invalid(format!("user dictionary: {e}")))?;
            user_penalty = dict.penalty.classes(&user.lexicon.features);
            user_pos = pos.index_all(&user.lexicon.features)?;
        }
        Ok(Analyzer {
            inner: Arc::new(Inner {
                dict,
                user,
                user_penalty,
                pos,
                user_pos,
            }),
        })
    }

    /// Loads a compiled system dictionary and, when given, a user dictionary CSV.
    pub fn load(dictionary: &Path, user_csv: Option<&Path>) -> Result<Analyzer> {
        let dict = Arc::new(Dictionary::load(dictionary)?);
        let user = user_csv.map(UserDictionary::from_csv_file).transpose()?;
        Analyzer::new(dict, user)
    }

    pub fn dictionary(&self) -> &Dictionary {
        &self.inner.dict
    }

    /// Feature columns of a morpheme's entry.
    pub fn features(&self, word: WordId) -> Features<'_> {
        let inner = &*self.inner;
        Features(match word {
            WordId::System(i) => inner.dict.words.features.get(i as usize),
            WordId::User(i) => inner
                .user
                .as_ref()
                .map_or("", |u| u.lexicon.features.get(i as usize)),
            WordId::Unknown(i) => inner.dict.unknown.features.get(i as usize),
        })
    }

    /// Index of the word's part-of-speech tag in [`Analyzer::pos_tags`]: the tag
    /// [`Features::pos`] returns, read from a small table instead of the feature text. Use it
    /// to filter many tokens by tag.
    #[inline]
    pub fn pos_index(&self, word: WordId) -> usize {
        let inner = &*self.inner;
        let tag = match word {
            WordId::System(i) => inner.dict.word_pos.get(i as usize),
            WordId::User(i) => inner.user_pos.get(i as usize),
            WordId::Unknown(i) => inner.dict.unknown_pos.get(i as usize),
        };
        tag.map_or(0, |&t| t as usize)
    }

    /// Distinct part-of-speech tags of the system and user dictionaries, `*` first. Indexed
    /// by [`Analyzer::pos_index`].
    pub fn pos_tags(&self) -> &[String] {
        &self.inner.pos.names
    }

    /// Segments `text`. Never fails; whitespace is not returned.
    pub fn analyze(&self, text: &str) -> Vec<Morpheme> {
        let mut out = Vec::new();
        self.analyze_into(text, &mut Lattice::default(), &mut out);
        out
    }

    /// Segments `text` into `out` (cleared first), reusing `lattice` across calls.
    pub fn analyze_into(&self, text: &str, lattice: &mut Lattice, out: &mut Vec<Morpheme>) {
        out.clear();
        let inner = &*self.inner;
        let chardef = &inner.dict.chardef;
        let has_space = chardef.space.is_some();
        let mut position = 0;
        let mut start = 0;
        while start < text.len() {
            let end = sentence_end(text, start);
            let Some(sentence) = text.get(start..end) else {
                break;
            };
            start = end;
            // Every token of a sentence made only of SPACE chars is whitespace and dropped
            // below, however the lattice splits it, so skip the lattice. Indentation tabs and
            // blank lines make up more than half of the sentences in source code.
            if has_space
                && sentence
                    .chars()
                    .all(|c| chardef.lookup_with_space(c).1 == Some(true))
            {
                continue;
            }
            lattice.run(inner, sentence);
            let path = &lattice.path;
            let sentence_start = end - sentence.len();
            for (k, &(char_start, word)) in path.iter().enumerate() {
                let char_end = path
                    .get(k + 1)
                    .map_or(lattice.chars.len(), |&(next, _)| next as usize);
                // Whitespace-only tokens are dropped (MeCab behavior); positions skip them.
                // A dictionary without a SPACE category keeps them, as lindera does. With a
                // SPACE category, `lattice.space` holds exactly that category test.
                if has_space
                    && lattice.space[char_start as usize..char_end]
                        .iter()
                        .all(|&s| s)
                {
                    continue;
                }
                let from = lattice.offsets[char_start as usize] as usize;
                let to = lattice.offsets[char_end] as usize;
                out.push(Morpheme {
                    start: sentence_start + from,
                    end: sentence_start + to,
                    position,
                    word,
                });
                position += 1;
            }
        }
    }
}

/// End (exclusive) of the sentence starting at `start`: after the next `\n`, `\t`, `。` or
/// `、`, or at the first char boundary at least [`MAX_SENTENCE_BYTES`] in.
fn sentence_end(text: &str, start: usize) -> usize {
    let bytes = text.as_bytes();
    let mut cap = start.saturating_add(MAX_SENTENCE_BYTES).min(bytes.len());
    while !text.is_char_boundary(cap) {
        cap += 1;
    }
    for (k, &b) in bytes[start..cap].iter().enumerate() {
        let end = start + k + 1;
        match b {
            b'\n' | b'\t' => return end,
            // Last byte of 。 (E3 80 82) or 、 (E3 80 81).
            0x81 | 0x82 if end >= 3 && bytes[end - 3] == 0xE3 && bytes[end - 2] == 0x80 => {
                return end;
            }
            _ => {}
        }
    }
    cap
}

impl Lattice {
    /// Builds the lattice for one sentence and leaves the best path in `self.path`.
    fn run(&mut self, inner: &Inner, sentence: &str) {
        let dict = &*inner.dict;
        let chardef = &dict.chardef;
        self.chars.clear();
        self.offsets.clear();
        self.categories.clear();
        self.space.clear();
        for (byte, c) in sentence.char_indices() {
            let (categories, space) = chardef.lookup_with_space(c);
            self.chars.push(c);
            self.offsets.push(byte as u32);
            self.categories.push(categories);
            self.space.push(space.unwrap_or_else(|| c.is_whitespace()));
        }
        self.offsets.push(sentence.len() as u32);
        let n = self.chars.len();

        for slot in &mut self.ends_at[..self.used_slots] {
            slot.clear();
        }
        if self.ends_at.len() < n + 1 {
            self.ends_at.resize_with(n + 1, Vec::new);
        }
        self.used_slots = n + 1;
        self.ends_at[0].push(Edge {
            path_cost: 0,
            left_index: NO_LEFT,
            start: 0,
            word: WordId::System(u32::MAX),
            right_id: 0,
        });

        let has_penalty = !dict.penalty.rules.is_empty();
        let mut unknown_end: Option<usize> = None;
        for i in 0..n {
            if self.ends_at[i].is_empty() {
                continue;
            }
            let penalized = has_penalty && i > 0 && self.space[i - 1];
            let mut found = false;

            // User words first, then system words; each longest first and, within one
            // surface, in reverse entry order: the order lindera inserts them.
            if let Some(user) = &inner.user {
                found |= self.add_words(
                    dict,
                    &user.lexicon,
                    i,
                    penalized,
                    &inner.user_penalty,
                    WordId::User,
                );
            }
            found |= self.add_words(
                dict,
                &dict.words,
                i,
                penalized,
                &dict.word_penalty,
                WordId::System,
            );

            if unknown_end.is_none_or(|end| end <= i) {
                let (offset, count) = self.categories[i];
                for ord in 0..count as usize {
                    let category = chardef.category_at(offset, ord);
                    unknown_end =
                        self.add_unknown(dict, category, ord, unknown_end, i, found, penalized);
                }
            }
        }

        // EOS: left context id 0.
        if !self.ends_at[n].is_empty() {
            let row = dict.matrix.row(0);
            let mut best = i32::MAX;
            let mut best_left = NO_LEFT;
            for (k, left) in self.ends_at[n].iter().enumerate() {
                let cost = left
                    .path_cost
                    .wrapping_add(row[left.right_id as usize] as i32);
                if cost < best {
                    best = cost;
                    best_left = k as u32;
                }
            }
            if best_left != NO_LEFT {
                self.ends_at[n].push(Edge {
                    path_cost: best,
                    left_index: best_left,
                    start: n as u32,
                    word: WordId::System(u32::MAX),
                    right_id: 0,
                });
            }
        }
        self.backtrace(n);
    }

    /// Adds the edges of every `lexicon` word starting at char `i`. Returns whether any
    /// word was found.
    fn add_words(
        &mut self,
        dict: &Dictionary,
        lexicon: &Lexicon,
        i: usize,
        penalized: bool,
        penalty: &[u8],
        id: fn(u32) -> WordId,
    ) -> bool {
        let mut matches = std::mem::take(&mut self.matches);
        matches.clear();
        lexicon.trie.prefixes(&self.chars[i..], |len, from, to| {
            matches.push(((i + len) as u32, from, to))
        });
        for &(end, from, to) in matches.iter().rev() {
            for e in (from..to).rev() {
                let extra = if penalized {
                    dict.penalty.cost(penalty[e as usize])
                } else {
                    0
                };
                self.add_edge(
                    dict,
                    i,
                    end as usize,
                    lexicon.entries[e as usize],
                    id(e),
                    extra,
                );
            }
        }
        let found = !matches.is_empty();
        self.matches = matches;
        found
    }

    /// Unknown-word candidates of one category at char `i` (lindera `process_unknown_word`
    /// in Normal mode, length ladder on, no grouping cap). Returns the new end of the
    /// unknown-word run.
    #[allow(clippy::too_many_arguments)]
    fn add_unknown(
        &mut self,
        dict: &Dictionary,
        category: u16,
        ord: usize,
        unknown_end: Option<usize>,
        i: usize,
        found: bool,
        penalized: bool,
    ) -> Option<usize> {
        let Some(def) = dict.chardef.categories.get(category as usize) else {
            return unknown_end;
        };
        let n = self.chars.len();
        // Same category at the same position of the next char's category list.
        let same = |lattice: &Lattice, j: usize| {
            let (offset, count) = lattice.categories[j];
            ord < count as usize && dict.chardef.category_at(offset, ord) == category
        };
        let mut length = 0;
        let mut group_run = None;
        if def.invoke || !found {
            length = 1;
            if def.group {
                let mut run = 1;
                while i + run < n && same(self, i + run) {
                    run += 1;
                }
                group_run = Some(run);
                length = run;
            }
        }
        if length == 0 {
            return unknown_end;
        }
        self.emit_unknown(dict, category, i, length, penalized);
        if def.length > 0 {
            let cap = def.length as usize;
            let run = match group_run {
                Some(run) => run.min(cap),
                None => {
                    let mut run = 1;
                    for k in 1..cap {
                        if i + k >= n || !same(self, i + k) {
                            break;
                        }
                        run += 1;
                    }
                    run
                }
            };
            for len in 1..=run {
                if len != length {
                    self.emit_unknown(dict, category, i, len, penalized);
                }
            }
        }
        Some(i + length)
    }

    fn emit_unknown(
        &mut self,
        dict: &Dictionary,
        category: u16,
        i: usize,
        len: usize,
        penalized: bool,
    ) {
        let Some(ids) = dict.unknown.by_category.get(category as usize) else {
            return;
        };
        for &id in ids {
            let extra = if penalized {
                dict.penalty.cost(dict.unknown_penalty[id as usize])
            } else {
                0
            };
            self.add_edge(
                dict,
                i,
                i + len,
                dict.unknown.entries[id as usize],
                WordId::Unknown(id),
                extra,
            );
        }
    }

    /// Connects a candidate spanning chars `start..end` to its cheapest left neighbour
    /// (first one on ties) and stores it.
    fn add_edge(
        &mut self,
        dict: &Dictionary,
        start: usize,
        end: usize,
        entry: Entry,
        word: WordId,
        extra: i32,
    ) {
        let left = &self.ends_at[start];
        if left.is_empty() {
            return;
        }
        let row = dict.matrix.row(entry.left_id);
        let mut best = i32::MAX;
        let mut best_left = NO_LEFT;
        for (k, l) in left.iter().enumerate() {
            let cost = l.path_cost.wrapping_add(row[l.right_id as usize] as i32);
            if cost < best {
                best = cost;
                best_left = k as u32;
            }
        }
        if best_left == NO_LEFT {
            return;
        }
        let path_cost = best
            .saturating_add(extra)
            .saturating_add(entry.cost as i32)
            .min(PATH_COST_CLAMP);
        self.ends_at[end].push(Edge {
            path_cost,
            left_index: best_left,
            start: start as u32,
            word,
            right_id: entry.right_id,
        });
    }

    /// Walks back from EOS (or, when EOS was not reached, from the last edge of the last
    /// non-empty slot, as lindera does) and stores (start char, word) pairs in order.
    fn backtrace(&mut self, n: usize) {
        self.path.clear();
        let mut last = n;
        while last > 0 && self.ends_at[last].is_empty() {
            last -= 1;
        }
        let Some(mut edge) = self.ends_at[last].last().copied() else {
            return;
        };
        while edge.left_index != NO_LEFT {
            self.path.push((edge.start, edge.word));
            edge = self.ends_at[edge.start as usize][edge.left_index as usize];
        }
        self.path.reverse();
        // The first pushed element (EOS) is now last.
        self.path.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sentence_end_splits_like_lindera() {
        let text = "가\t나\n다。라、마";
        let mut ends = Vec::new();
        let mut start = 0;
        while start < text.len() {
            start = sentence_end(text, start);
            ends.push(&text[..start]);
        }
        assert_eq!(
            ends,
            ["가\t", "가\t나\n", "가\t나\n다。", "가\t나\n다。라、", text]
        );
        let long = "가".repeat(20_000);
        let first = sentence_end(&long, 0);
        assert!(first >= MAX_SENTENCE_BYTES && long.is_char_boundary(first));
    }

    /// The byte-at-a-time loop `sentence_end` replaced.
    fn sentence_end_reference(text: &str, start: usize) -> usize {
        let bytes = text.as_bytes();
        let mut end = start;
        while end < bytes.len() {
            let b = bytes[end];
            end += 1;
            if b == b'\n' || b == b'\t' {
                return end;
            }
            if (b == 0x81 || b == 0x82) && end >= 3 {
                let last = &bytes[end - 3..end];
                if last == "。".as_bytes() || last == "、".as_bytes() {
                    return end;
                }
            }
            if end - start >= MAX_SENTENCE_BYTES && text.is_char_boundary(end) {
                return end;
            }
        }
        end
    }

    #[test]
    fn sentence_end_matches_reference() {
        // Terminators and multi-byte chars around the 32 KiB cut, plus chars that end in
        // 0x81/0x82 without being 。 or 、: ꂁ (EA 82 81), 䂂 (E4 82 82), ぁ (E3 81 81), も (E3 82 82).
        let pieces = [
            "a", "가", "\n", "\t", "。", "、", "ꂁ", "䂂", "ぁ", "も", "😀", " ",
        ];
        let mut seed = 0x2545_f491_4f6c_dd1du64;
        for round in 0..200u32 {
            let mut text = String::new();
            let long = round.is_multiple_of(4);
            let target = if long {
                MAX_SENTENCE_BYTES * 2 + 7
            } else {
                300
            };
            while text.len() < target {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                // Long runs without terminators reach the cap.
                let piece = if long && !seed.is_multiple_of(64) {
                    pieces[(seed % 2) as usize]
                } else {
                    pieces[(seed % pieces.len() as u64) as usize]
                };
                text.push_str(piece);
            }
            let mut start = 0;
            while start < text.len() {
                let (got, want) = (
                    sentence_end(&text, start),
                    sentence_end_reference(&text, start),
                );
                assert_eq!(got, want, "round {round} start {start}");
                start = got;
            }
        }
    }

    #[test]
    fn features_pad_missing_columns() {
        let f = Features("NNG,*,T,휴인증");
        assert_eq!(f.pos(), "NNG");
        assert_eq!(f.get(3), "휴인증");
        assert_eq!(f.kind(), "*");
        assert_eq!(Features("").pos(), "*");
    }
}
