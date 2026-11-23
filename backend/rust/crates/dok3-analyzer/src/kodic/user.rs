//! User dictionaries in the CSV formats lindera accepts for ko-dic.

use std::borrow::Cow;
use std::path::Path;

use super::csv::Records;
use super::error::{Error, Result};
use super::lexicon::{Entry, Lexicon, Row};
use super::source::{join_features, parse_entry};

/// Emission cost of a 3-column row (lindera-ko-dic `default_word_cost`).
const SIMPLE_COST: i16 = -10000;

/// Words added on top of the system dictionary. User entries are looked up before system
/// entries, so they win a tie.
///
/// Rows take one of two forms:
///
/// - mecab ko-dic form, 12 or more columns:
///   `surface,left_id,right_id,cost,POS,semantic,jongseong,reading,type,first_pos,last_pos,expression`
/// - simple form, 3 columns: `surface,POS,reading`. Context ids are 0 and the cost is
///   -10000, which can disturb neighbouring morphemes; prefer the full form with ids copied
///   from a similar ko-dic row.
pub struct UserDictionary {
    pub(crate) lexicon: Lexicon,
}

impl UserDictionary {
    pub fn from_csv_file(path: &Path) -> Result<UserDictionary> {
        let text = std::fs::read_to_string(path).map_err(|e| Error::io(path, e))?;
        UserDictionary::from_csv(path, &text)
    }

    /// Parses CSV text; `path` is only used in error messages.
    pub fn from_csv(path: &Path, text: &str) -> Result<UserDictionary> {
        let mut records = Records::new(text);
        let mut fields: Vec<Cow<str>> = Vec::new();
        let mut rows = Vec::new();
        while let Some(line) = records.next_record(&mut fields) {
            let surface = fields[0].to_string();
            if surface.is_empty() {
                return Err(Error::source(path, line, "empty surface"));
            }
            let (entry, features) = match fields.len() {
                3 => {
                    join_features(path, line, &fields[1..])?;
                    // Same columns lindera fills for ko-dic: POS and reading, `*` elsewhere.
                    (
                        Entry {
                            left_id: 0,
                            right_id: 0,
                            cost: SIMPLE_COST,
                        },
                        format!("{},*,*,{},*,*,*,*", fields[1], fields[2]),
                    )
                }
                n if n >= 12 => (
                    parse_entry(path, line, &fields[1], &fields[2], &fields[3])?,
                    join_features(path, line, &fields[4..])?,
                ),
                n => {
                    return Err(Error::source(
                        path,
                        line,
                        format!(
                            "expected 3 columns (surface,POS,reading) or 12+ ko-dic columns, got {n}"
                        ),
                    ));
                }
            };
            rows.push(Row {
                raw_surface: surface.clone(),
                surface,
                entry,
                features,
            });
        }
        Ok(UserDictionary {
            lexicon: Lexicon::from_rows(rows),
        })
    }

    /// Number of entries.
    pub fn len(&self) -> usize {
        self.lexicon.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.lexicon.entries.is_empty()
    }
}
