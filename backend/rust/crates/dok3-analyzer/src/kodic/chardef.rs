//! Character categories from mecab `char.def`.
//!
//! A character belongs to every category whose range contains it, in file order without
//! duplicates (lindera semantics; MeCab would let a later line replace an earlier one).
//! A character in no range belongs to `DEFAULT`.

use std::collections::HashMap;
use std::path::Path;

use super::error::{Error, Result};

pub(crate) const DEFAULT_CATEGORY: &str = "DEFAULT";
pub(crate) const SPACE_CATEGORY: &str = "SPACE";

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Category {
    pub name: String,
    /// Generate unknown words here even when a dictionary word starts at this position.
    pub invoke: bool,
    /// Group a run of same-category characters into one unknown word.
    pub group: bool,
    /// Also emit unknown words of 1..=length characters (0 = none).
    pub length: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CharRange {
    pub lo: u32,
    pub hi: u32,
    pub categories: Vec<u16>,
}

/// Parsed `char.def` plus lookup tables derived at load time.
pub(crate) struct CharDef {
    pub categories: Vec<Category>,
    pub ranges: Vec<CharRange>,
    /// Category id lists, one per row.
    pool: Vec<u16>,
    /// (offset into `pool`, length) per row.
    rows: Vec<(u32, u16)>,
    /// Whether the row's categories include `SPACE`, per row.
    row_space: Vec<bool>,
    /// Row index per BMP code point.
    bmp: Vec<u16>,
    /// Sorted code point boundaries; `rows[k]` covers `boundaries[k]..boundaries[k + 1]`.
    boundaries: Vec<u32>,
    pub space: Option<u16>,
}

impl CharDef {
    /// Parses `char.def` text (comments start with `#`).
    pub fn parse(path: &Path, text: &str) -> Result<(Vec<Category>, Vec<CharRange>)> {
        let mut categories: Vec<Category> = Vec::new();
        let mut ids: HashMap<String, u16> = HashMap::new();
        let mut ranges = Vec::new();
        for (i, raw) in text.lines().enumerate() {
            let line_no = i + 1;
            let line = raw.split('#').next().unwrap_or("").trim();
            if line.is_empty() {
                continue;
            }
            if line.starts_with("0x") {
                let fields: Vec<&str> = line.split_whitespace().collect();
                let bounds: Vec<&str> = fields[0].split("..").collect();
                let hex = |s: &str| {
                    u32::from_str_radix(s.trim_start_matches("0x"), 16).map_err(|e| {
                        Error::source(path, line_no, format!("bad code point {s:?}: {e}"))
                    })
                };
                let (lo, hi) = match bounds.as_slice() {
                    [one] => (hex(one)?, hex(one)?),
                    [lo, hi] => (hex(lo)?, hex(hi)?),
                    _ => {
                        return Err(Error::source(
                            path,
                            line_no,
                            "expected 0xSTART..0xEND or 0xCODE",
                        ));
                    }
                };
                if lo > hi || hi > char::MAX as u32 {
                    return Err(Error::source(path, line_no, "invalid code point range"));
                }
                let mut cats = Vec::with_capacity(fields.len() - 1);
                for name in &fields[1..] {
                    let id = *ids.get(*name).ok_or_else(|| {
                        Error::source(
                            path,
                            line_no,
                            format!("category {name} is used before it is defined"),
                        )
                    })?;
                    cats.push(id);
                }
                if cats.is_empty() {
                    return Err(Error::source(path, line_no, "range without a category"));
                }
                ranges.push(CharRange {
                    lo,
                    hi,
                    categories: cats,
                });
            } else {
                let fields: Vec<&str> = line.split_ascii_whitespace().collect();
                let [name, invoke, group, length] = fields.as_slice() else {
                    return Err(Error::source(
                        path,
                        line_no,
                        "expected: NAME INVOKE GROUP LENGTH",
                    ));
                };
                let num = |s: &str| {
                    s.parse::<u32>()
                        .map_err(|e| Error::source(path, line_no, format!("bad number {s:?}: {e}")))
                };
                if ids.contains_key(*name) {
                    return Err(Error::source(
                        path,
                        line_no,
                        format!("category {name} is defined twice"),
                    ));
                }
                let id = u16::try_from(categories.len())
                    .map_err(|_| Error::source(path, line_no, "too many categories"))?;
                ids.insert(name.to_string(), id);
                categories.push(Category {
                    name: name.to_string(),
                    invoke: num(invoke)? == 1,
                    group: num(group)? == 1,
                    length: num(length)?,
                });
            }
        }
        if !ids.contains_key(DEFAULT_CATEGORY) {
            return Err(Error::source(
                path,
                0,
                "char.def defines no DEFAULT category",
            ));
        }
        Ok((categories, ranges))
    }

    /// Builds the lookup tables. Fails when `DEFAULT` is missing or a range names an
    /// unknown category id.
    pub fn new(categories: Vec<Category>, ranges: Vec<CharRange>) -> Result<Self> {
        let default = categories
            .iter()
            .position(|c| c.name == DEFAULT_CATEGORY)
            .ok_or_else(|| Error::Format("char.def has no DEFAULT category".into()))?
            as u16;
        if ranges
            .iter()
            .flat_map(|r| &r.categories)
            .any(|&c| c as usize >= categories.len())
        {
            return Err(Error::Format(
                "character range refers to an unknown category".into(),
            ));
        }
        let space = categories
            .iter()
            .position(|c| c.name == SPACE_CATEGORY)
            .map(|i| i as u16);

        let mut boundaries: Vec<u32> = vec![0];
        for r in &ranges {
            boundaries.push(r.lo);
            if r.hi < char::MAX as u32 {
                boundaries.push(r.hi + 1);
            }
        }
        boundaries.sort_unstable();
        boundaries.dedup();

        let mut pool = Vec::new();
        let mut rows = Vec::with_capacity(boundaries.len());
        for &b in &boundaries {
            let start = pool.len();
            for r in ranges.iter().filter(|r| r.lo <= b && b <= r.hi) {
                for &c in &r.categories {
                    if !pool[start..].contains(&c) {
                        pool.push(c);
                    }
                }
            }
            if pool.len() == start {
                pool.push(default);
            }
            rows.push((start as u32, (pool.len() - start) as u16));
        }

        let mut bmp = Vec::with_capacity(0x10000);
        let mut row = 0usize;
        for cp in 0..0x10000u32 {
            while row + 1 < boundaries.len() && boundaries[row + 1] <= cp {
                row += 1;
            }
            bmp.push(row as u16);
        }

        let row_space = rows
            .iter()
            .map(|&(offset, len)| {
                space.is_some_and(|sp| pool[offset as usize..][..len as usize].contains(&sp))
            })
            .collect();

        Ok(CharDef {
            categories,
            ranges,
            pool,
            rows,
            row_space,
            bmp,
            boundaries,
            space,
        })
    }

    #[inline]
    fn row(&self, c: char) -> usize {
        let cp = c as u32;
        if cp < 0x10000 {
            self.bmp[cp as usize] as usize
        } else {
            self.boundaries.partition_point(|&b| b <= cp) - 1
        }
    }

    /// The categories of `c` as (offset, length) into [`CharDef::category_at`].
    #[cfg(test)]
    pub fn lookup(&self, c: char) -> (u32, u16) {
        self.rows[self.row(c)]
    }

    /// [`CharDef::lookup`] plus [`CharDef::is_space_category`] with one table lookup.
    #[inline]
    pub fn lookup_with_space(&self, c: char) -> ((u32, u16), Option<bool>) {
        let row = self.row(c);
        (self.rows[row], self.space.map(|_| self.row_space[row]))
    }

    #[inline]
    pub fn category_at(&self, offset: u32, index: usize) -> u16 {
        self.pool[offset as usize + index]
    }

    #[cfg(test)]
    pub fn categories_of(&self, c: char) -> &[u16] {
        let (offset, len) = self.lookup(c);
        &self.pool[offset as usize..offset as usize + len as usize]
    }

    /// Whether `c` carries the `SPACE` category. `None` when the dictionary has no such
    /// category.
    #[cfg(test)]
    pub fn is_space_category(&self, c: char) -> Option<bool> {
        self.space
            .map(|space| self.categories_of(c).contains(&space))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chardef(text: &str) -> CharDef {
        let (cats, ranges) = CharDef::parse(Path::new("char.def"), text).unwrap();
        CharDef::new(cats, ranges).unwrap()
    }

    fn names(def: &CharDef, c: char) -> Vec<&str> {
        def.categories_of(c)
            .iter()
            .map(|&i| def.categories[i as usize].name.as_str())
            .collect()
    }

    #[test]
    fn union_in_file_order_and_default() {
        let def = chardef(
            "DEFAULT 0 1 0\nSPACE 0 1 0\nHANJA 0 0 1\nNUM 1 1 0\nSYMBOL 1 1 0 # sym\n\
             0x0020 SPACE\n0x4E00..0x9FA5 HANJA\n0x4E00 NUM HANJA\n0x3000..0x303F SYMBOL\n0x3007 SYMBOL NUM\n",
        );
        assert_eq!(names(&def, '一'), ["HANJA", "NUM"]);
        assert_eq!(names(&def, '〇'), ["SYMBOL", "NUM"]);
        assert_eq!(names(&def, 'a'), ["DEFAULT"]);
        assert_eq!(names(&def, '😀'), ["DEFAULT"]);
        assert_eq!(def.is_space_category(' '), Some(true));
        assert_eq!(def.is_space_category('\u{3000}'), Some(false));
        for c in [' ', '\u{3000}', '一', '〇', 'a', '😀'] {
            assert_eq!(
                def.lookup_with_space(c),
                (def.lookup(c), def.is_space_category(c)),
                "{c:?}"
            );
        }
        let no_space = chardef("DEFAULT 0 1 0\nALPHA 1 1 0\n0x0061 ALPHA\n");
        assert_eq!(
            no_space.lookup_with_space(' '),
            (no_space.lookup(' '), None)
        );
    }

    #[test]
    fn rejects_undefined_category() {
        assert!(CharDef::parse(Path::new("char.def"), "DEFAULT 0 1 0\n0x0020 SPACE\n").is_err());
    }
}
