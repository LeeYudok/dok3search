//! Binary file format of a compiled dictionary.
//!
//! Little endian throughout. Layout (format version 1):
//!
//! ```text
//! magic "DKKODIC\0", u32 version
//! str label
//! u32 n, n x (str name, u8 invoke, u8 group, u32 length)          character categories
//! u32 n, n x (u32 lo, u32 hi, u16s categories)                     character ranges
//! u32 n, n x (i32 cost, u32 m, m x str tag)                        left-space penalty rules
//! u32 right_size, u32 left_size, i16s costs                        connection matrix
//! u16s category, u16s left_id, u16s right_id, i16s cost, features  unknown-word templates
//! u32s labels, u32s child_start, u32s entry_start,                 word trie
//! u16s left_id, u16s right_id, i16s cost, features                 word entries
//! "END\0"
//! ```
//!
//! `str` is a u32 byte length plus UTF-8 bytes, `u16s`/`u32s`/`i16s` a u32 count plus the
//! values, and `features` a `u32s` offset table plus one `str`. Every count and offset is
//! validated on load, so a damaged file is an error rather than a panic.

use super::chardef::{Category, CharRange};
use super::dictionary::{Dictionary, Matrix, SpacePenalty};
use super::error::{Error, Result};
use super::lexicon::{Entry, FeatureTable, Lexicon};
use super::trie::Trie;

pub(crate) const MAGIC: &[u8; 8] = b"DKKODIC\0";
pub(crate) const VERSION: u32 = 1;
const END: &[u8; 4] = b"END\0";

pub(crate) fn write(dict: &Dictionary) -> Vec<u8> {
    let mut w = Writer(Vec::with_capacity(96 << 20));
    w.bytes(MAGIC);
    w.u32(VERSION);
    w.str(&dict.label);

    let chardef = &dict.chardef;
    w.u32(chardef.categories.len() as u32);
    for c in &chardef.categories {
        w.str(&c.name);
        w.u8(c.invoke as u8);
        w.u8(c.group as u8);
        w.u32(c.length);
    }
    w.u32(chardef.ranges.len() as u32);
    for r in &chardef.ranges {
        w.u32(r.lo);
        w.u32(r.hi);
        w.u16s(&r.categories);
    }

    w.u32(dict.penalty.rules.len() as u32);
    for (cost, tags) in &dict.penalty.rules {
        w.i32(*cost);
        w.u32(tags.len() as u32);
        for t in tags {
            w.str(t);
        }
    }

    w.u32(dict.matrix.right_size as u32);
    w.u32(dict.matrix.left_size as u32);
    w.i16s(&dict.matrix.costs);

    w.u16s(&dict.unknown.categories);
    w.entries(&dict.unknown.entries);
    w.features(&dict.unknown.features);

    let trie = &dict.words.trie;
    w.u32s(&trie.labels);
    w.u32s(&trie.child_starts());
    w.u32s(&trie.entry_starts());
    w.entries(&dict.words.entries);
    w.features(&dict.words.features);

    w.bytes(END);
    w.0
}

pub(crate) fn read(data: &[u8]) -> Result<Dictionary> {
    let mut r = Reader { data, pos: 0 };
    if r.take(8)? != MAGIC {
        return Err(Error::Format(
            "not a doksam-korean dictionary (bad magic)".into(),
        ));
    }
    let version = r.u32()?;
    if version != VERSION {
        return Err(Error::Format(format!(
            "format version {version}, this build reads {VERSION}; rebuild the dictionary"
        )));
    }
    let label = r.str()?;

    let n = r.count(10)?;
    let mut categories = Vec::with_capacity(n);
    for _ in 0..n {
        categories.push(Category {
            name: r.str()?,
            invoke: r.u8()? != 0,
            group: r.u8()? != 0,
            length: r.u32()?,
        });
    }
    let n = r.count(12)?;
    let mut ranges = Vec::with_capacity(n);
    for _ in 0..n {
        ranges.push(CharRange {
            lo: r.u32()?,
            hi: r.u32()?,
            categories: r.u16s()?,
        });
    }

    let n = r.count(8)?;
    let mut rules = Vec::with_capacity(n);
    for _ in 0..n {
        let cost = r.i32()?;
        let m = r.count(4)?;
        let mut tags = Vec::with_capacity(m);
        for _ in 0..m {
            tags.push(r.str()?);
        }
        rules.push((cost, tags));
    }

    let right_size = r.u32()? as usize;
    let left_size = r.u32()? as usize;
    let costs = r.i16s()?;
    let matrix = Matrix {
        right_size,
        left_size,
        costs,
    };

    let unknown_categories = r.u16s()?;
    let unknown_entries = r.entries()?;
    let unknown_features = r.features()?;

    let (labels, child_start, entry_start) = (r.u32s()?, r.u32s()?, r.u32s()?);
    let trie = Trie::from_arrays(labels, child_start, entry_start)?;
    let entries = r.entries()?;
    let features = r.features()?;

    if r.take(4)? != END || r.pos != data.len() {
        return Err(Error::Format("trailing or missing data at the end".into()));
    }
    Dictionary::from_parts(
        label,
        categories,
        ranges,
        unknown_entries,
        unknown_categories,
        unknown_features,
        matrix,
        Lexicon {
            trie,
            entries,
            features,
        },
        SpacePenalty { rules },
    )
}

struct Writer(Vec<u8>);

impl Writer {
    fn bytes(&mut self, b: &[u8]) {
        self.0.extend_from_slice(b);
    }
    fn u8(&mut self, v: u8) {
        self.0.push(v);
    }
    fn u32(&mut self, v: u32) {
        self.bytes(&v.to_le_bytes());
    }
    fn i32(&mut self, v: i32) {
        self.bytes(&v.to_le_bytes());
    }
    fn str(&mut self, s: &str) {
        self.u32(s.len() as u32);
        self.bytes(s.as_bytes());
    }
    fn u16s(&mut self, v: &[u16]) {
        self.u32(v.len() as u32);
        self.0.reserve(v.len() * 2);
        for x in v {
            self.bytes(&x.to_le_bytes());
        }
    }
    fn i16s(&mut self, v: &[i16]) {
        self.u32(v.len() as u32);
        self.0.reserve(v.len() * 2);
        for x in v {
            self.bytes(&x.to_le_bytes());
        }
    }
    fn u32s(&mut self, v: &[u32]) {
        self.u32(v.len() as u32);
        self.0.reserve(v.len() * 4);
        for x in v {
            self.bytes(&x.to_le_bytes());
        }
    }
    fn entries(&mut self, entries: &[Entry]) {
        self.u16s(&entries.iter().map(|e| e.left_id).collect::<Vec<_>>());
        self.u16s(&entries.iter().map(|e| e.right_id).collect::<Vec<_>>());
        self.i16s(&entries.iter().map(|e| e.cost).collect::<Vec<_>>());
    }
    fn features(&mut self, f: &FeatureTable) {
        self.u32s(&f.offsets);
        self.str(&f.text);
    }
}

struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8]> {
        let end = self
            .pos
            .checked_add(n)
            .filter(|&e| e <= self.data.len())
            .ok_or_else(|| {
                Error::Format(format!("truncated at byte {} (needs {n} more)", self.pos))
            })?;
        let slice = &self.data[self.pos..end];
        self.pos = end;
        Ok(slice)
    }
    fn u8(&mut self) -> Result<u8> {
        Ok(self.take(1)?[0])
    }
    fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(
            self.take(4)?.try_into().expect("4 bytes"),
        ))
    }
    fn i32(&mut self) -> Result<i32> {
        Ok(i32::from_le_bytes(
            self.take(4)?.try_into().expect("4 bytes"),
        ))
    }
    /// A count of items that take at least `min_item_bytes` each; rejects counts the
    /// remaining data cannot hold before anything is allocated.
    fn count(&mut self, min_item_bytes: usize) -> Result<usize> {
        let n = self.u32()? as usize;
        if n.saturating_mul(min_item_bytes) > self.data.len() - self.pos {
            return Err(Error::Format(format!(
                "count {n} at byte {} exceeds the file",
                self.pos - 4
            )));
        }
        Ok(n)
    }
    fn str(&mut self) -> Result<String> {
        let n = self.count(1)?;
        let bytes = self.take(n)?;
        String::from_utf8(bytes.to_vec()).map_err(|_| Error::Format("string is not UTF-8".into()))
    }
    fn u16s(&mut self) -> Result<Vec<u16>> {
        let n = self.count(2)?;
        Ok(self
            .take(n * 2)?
            .as_chunks::<2>()
            .0
            .iter()
            .copied()
            .map(u16::from_le_bytes)
            .collect())
    }
    fn i16s(&mut self) -> Result<Vec<i16>> {
        let n = self.count(2)?;
        Ok(self
            .take(n * 2)?
            .as_chunks::<2>()
            .0
            .iter()
            .copied()
            .map(i16::from_le_bytes)
            .collect())
    }
    fn u32s(&mut self) -> Result<Vec<u32>> {
        let n = self.count(4)?;
        Ok(self
            .take(n * 4)?
            .as_chunks::<4>()
            .0
            .iter()
            .copied()
            .map(u32::from_le_bytes)
            .collect())
    }
    fn entries(&mut self) -> Result<Vec<Entry>> {
        let (left, right, cost) = (self.u16s()?, self.u16s()?, self.i16s()?);
        if left.len() != right.len() || left.len() != cost.len() {
            return Err(Error::Format("entry arrays differ in length".into()));
        }
        Ok((0..left.len())
            .map(|i| Entry {
                left_id: left[i],
                right_id: right[i],
                cost: cost[i],
            })
            .collect())
    }
    fn features(&mut self) -> Result<FeatureTable> {
        Ok(FeatureTable {
            offsets: self.u32s()?,
            text: self.str()?,
        })
    }
}
