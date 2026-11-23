//! Word lists: a trie of surfaces plus per-entry context ids, costs and feature strings.

use std::collections::BTreeMap;

use super::error::{Error, Result};
use super::trie::{Keys, Trie};

/// Connection-cost ids and emission cost of one dictionary entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Entry {
    pub left_id: u16,
    pub right_id: u16,
    pub cost: i16,
}

/// Feature strings (CSV columns after the cost, joined with `,`), one per entry.
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct FeatureTable {
    pub text: String,
    /// `offsets[i]..offsets[i + 1]` is entry `i`.
    pub offsets: Vec<u32>,
}

impl FeatureTable {
    pub fn new() -> Self {
        FeatureTable {
            text: String::new(),
            offsets: vec![0],
        }
    }

    pub fn push(&mut self, features: &str) {
        self.text.push_str(features);
        self.offsets.push(self.text.len() as u32);
    }

    pub fn len(&self) -> usize {
        self.offsets.len().saturating_sub(1)
    }

    #[inline]
    pub fn get(&self, i: usize) -> &str {
        match (self.offsets.get(i), self.offsets.get(i + 1)) {
            (Some(&s), Some(&e)) => self.text.get(s as usize..e as usize).unwrap_or(""),
            _ => "",
        }
    }

    pub fn validate(&self, count: usize) -> Result<()> {
        if self.offsets.len() != count + 1
            || self.offsets.first() != Some(&0)
            || *self.offsets.last().unwrap_or(&0) as usize != self.text.len()
            || !self.offsets.windows(2).all(|w| w[0] <= w[1])
            || !self
                .offsets
                .iter()
                .all(|&o| self.text.is_char_boundary(o as usize))
        {
            return Err(Error::Format("feature table offsets are invalid".into()));
        }
        Ok(())
    }
}

/// A searchable word list.
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct Lexicon {
    pub trie: Trie,
    pub entries: Vec<Entry>,
    pub features: FeatureTable,
}

/// One parsed dictionary row.
pub(crate) struct Row {
    /// Sort key: the surface column as written.
    pub raw_surface: String,
    /// Lookup key.
    pub surface: String,
    pub entry: Entry,
    pub features: String,
}

impl Lexicon {
    /// Builds a lexicon from rows. Rows are stably sorted by their raw surface and grouped
    /// by surface, so entries of one surface keep their input order (file order, then line
    /// order), the order lindera's dictionary builder produces.
    pub fn from_rows(mut rows: Vec<Row>) -> Lexicon {
        rows.sort_by(|a, b| a.raw_surface.cmp(&b.raw_surface));
        let mut groups: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
        for (i, row) in rows.iter().enumerate() {
            groups.entry(row.surface.as_str()).or_default().push(i);
        }
        let mut keys = Keys::new();
        let mut counts = Vec::with_capacity(groups.len());
        let mut members = Vec::with_capacity(groups.len());
        for (surface, rows_of_surface) in &groups {
            keys.push(surface);
            counts.push(rows_of_surface.len() as u32);
            members.push(rows_of_surface);
        }
        let (trie, order) = Trie::build(&keys, &counts);
        let mut entries = Vec::with_capacity(rows.len());
        let mut features = FeatureTable::new();
        for k in order {
            for &r in members[k as usize] {
                entries.push(rows[r].entry);
                features.push(&rows[r].features);
            }
        }
        Lexicon {
            trie,
            entries,
            features,
        }
    }

    pub fn validate(&self, left_size: usize, right_size: usize) -> Result<()> {
        self.trie.validate(self.entries.len())?;
        self.features.validate(self.entries.len())?;
        if let Some(e) = self
            .entries
            .iter()
            .find(|e| e.left_id as usize >= left_size || e.right_id as usize >= right_size)
        {
            return Err(Error::Invalid(format!(
                "context ids {}/{} are outside the connection matrix ({left_size} left x {right_size} right)",
                e.left_id, e.right_id
            )));
        }
        Ok(())
    }
}
