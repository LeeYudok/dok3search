//! Compiling a mecab-ko-dic source directory.
//!
//! Reads the files the way lindera 6.2 builds its embedded ko-dic, so the compiled
//! dictionary segments text identically: every `*.csv` in file-name order, rows stably
//! sorted by surface, `char.def` categories as unions, `unk.def` lines split on commas, and
//! `matrix.def` cells that are not listed left at `i16::MAX`.

use std::borrow::Cow;
use std::fs;
use std::path::{Path, PathBuf};

use super::chardef::CharDef;
use super::csv::Records;
use super::dictionary::{Dictionary, Matrix, SpacePenalty};
use super::error::{Error, Result};
use super::lexicon::{Entry, FeatureTable, Lexicon, Row};

pub(crate) fn compile(dir: &Path) -> Result<Dictionary> {
    let read = |name: &str| {
        let path = dir.join(name);
        fs::read_to_string(&path).map_err(|e| Error::io(&path, e))
    };
    let chardef_path = dir.join("char.def");
    let (categories, ranges) = CharDef::parse(&chardef_path, &read("char.def")?)?;
    let unk_path = dir.join("unk.def");
    let (unknown_entries, unknown_categories, unknown_features) =
        parse_unknown(&unk_path, &read("unk.def")?, &categories)?;
    let matrix_path = dir.join("matrix.def");
    let matrix_bytes = fs::read(&matrix_path).map_err(|e| Error::io(&matrix_path, e))?;
    let matrix = parse_matrix(&matrix_path, &matrix_bytes)?;
    drop(matrix_bytes);

    let mut rows = Vec::new();
    for path in csv_files(dir)? {
        let text = fs::read_to_string(&path).map_err(|e| Error::io(&path, e))?;
        read_system_rows(&path, &text, &mut rows)?;
    }
    if rows.is_empty() {
        return Err(Error::source(dir, 0, "no dictionary rows (*.csv) found"));
    }
    let words = Lexicon::from_rows(rows);

    let label = dir
        .canonicalize()
        .ok()
        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
        .unwrap_or_default();
    Dictionary::from_parts(
        label,
        categories,
        ranges,
        unknown_entries,
        unknown_categories,
        unknown_features,
        matrix,
        words,
        SpacePenalty::ko_dic(),
    )
}

/// `*.csv` files of `dir` sorted by file name (byte order), as lindera's glob lists them.
/// Hidden files are skipped: macOS writes `._NNG.csv` AppleDouble files on non-APFS disks.
fn csv_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut names = Vec::new();
    for entry in fs::read_dir(dir).map_err(|e| Error::io(dir, e))? {
        let entry = entry.map_err(|e| Error::io(dir, e))?;
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if name_str.ends_with(".csv") && !name_str.starts_with('.') && entry.path().is_file() {
            names.push(name);
        }
    }
    names.sort_by(|a, b| a.as_encoded_bytes().cmp(b.as_encoded_bytes()));
    Ok(names.into_iter().map(|n| dir.join(n)).collect())
}

/// Appends the rows of one system dictionary CSV. Like lindera, a row whose surface, ids or
/// cost is blank is skipped, and an unparsable number is an error.
fn read_system_rows(path: &Path, text: &str, rows: &mut Vec<Row>) -> Result<()> {
    let mut records = Records::new(text);
    let mut fields: Vec<Cow<str>> = Vec::new();
    while let Some(line) = records.next_record(&mut fields) {
        let field = |i: usize| fields.get(i).map(|f| f.trim()).filter(|f| !f.is_empty());
        let (Some(surface), Some(left), Some(right), Some(cost)) =
            (field(0), field(1), field(2), field(3))
        else {
            continue;
        };
        let entry = parse_entry(path, line, left, right, cost)?;
        rows.push(Row {
            raw_surface: fields[0].to_string(),
            surface: surface.to_string(),
            entry,
            features: join_features(path, line, &fields[4..])?,
        });
    }
    Ok(())
}

pub(crate) fn parse_entry(
    path: &Path,
    line: usize,
    left: &str,
    right: &str,
    cost: &str,
) -> Result<Entry> {
    let id = |s: &str, what: &str| {
        s.trim()
            .parse::<u16>()
            .map_err(|_| Error::source(path, line, format!("invalid {what} {s:?}")))
    };
    Ok(Entry {
        left_id: id(left, "left context id")?,
        right_id: id(right, "right context id")?,
        cost: cost
            .trim()
            .parse::<i16>()
            .map_err(|_| Error::source(path, line, format!("invalid cost {cost:?}")))?,
    })
}

/// Joins feature columns with `,`. A column that itself contains a comma would make the
/// joined string ambiguous, so it is rejected.
pub(crate) fn join_features(path: &Path, line: usize, fields: &[Cow<str>]) -> Result<String> {
    if fields.iter().any(|f| f.contains(',')) {
        return Err(Error::source(
            path,
            line,
            "a feature column contains a comma",
        ));
    }
    Ok(fields.join(","))
}

/// unk.def: `CATEGORY,left_id,right_id,cost,features...` per line, plain comma split.
fn parse_unknown(
    path: &Path,
    text: &str,
    categories: &[super::chardef::Category],
) -> Result<(Vec<Entry>, Vec<u16>, FeatureTable)> {
    let mut entries = Vec::new();
    let mut cats = Vec::new();
    let mut features = FeatureTable::new();
    for (i, line) in text.lines().enumerate() {
        let line_no = i + 1;
        if line.trim().is_empty() {
            continue;
        }
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() < 4 {
            return Err(Error::source(
                path,
                line_no,
                "expected CATEGORY,left_id,right_id,cost,...",
            ));
        }
        let cost = fields[3]
            .trim()
            .parse::<i32>()
            .map_err(|_| Error::source(path, line_no, format!("invalid cost {:?}", fields[3])))?;
        let mut entry = parse_entry(path, line_no, fields[1], fields[2], "0")?;
        entry.cost = cost as i16;
        entries.push(entry);
        cats.push(
            categories
                .iter()
                .position(|c| c.name == fields[0])
                .map_or(u16::MAX, |p| p as u16),
        );
        features.push(&fields[4..].join(","));
    }
    Ok((entries, cats, features))
}

/// matrix.def: header `RIGHT_SIZE LEFT_SIZE`, then `right_id left_id cost` lines.
fn parse_matrix(path: &Path, bytes: &[u8]) -> Result<Matrix> {
    let bytes = bytes.strip_prefix(b"\xEF\xBB\xBF").unwrap_or(bytes);
    let mut lines = bytes.split(|&b| b == b'\n');
    let header = lines.next().unwrap_or_default();
    let mut pos = 0;
    let (Some(right_size), Some(left_size)) =
        (next_int(header, &mut pos), next_int(header, &mut pos))
    else {
        return Err(Error::source(
            path,
            1,
            "expected the header RIGHT_SIZE LEFT_SIZE",
        ));
    };
    if !(1..=1 << 16).contains(&right_size) || !(1..=1 << 16).contains(&left_size) {
        return Err(Error::source(path, 1, "matrix size out of range"));
    }
    let (right_size, left_size) = (right_size as usize, left_size as usize);
    let mut costs = vec![i16::MAX; right_size * left_size];
    for (i, line) in lines.enumerate() {
        let mut pos = 0;
        let Some(right) = next_int(line, &mut pos) else {
            continue;
        };
        let (Some(left), Some(cost)) = (next_int(line, &mut pos), next_int(line, &mut pos)) else {
            return Err(Error::source(path, i + 2, "expected RIGHT_ID LEFT_ID COST"));
        };
        if right < 0 || left < 0 || right as usize >= right_size || left as usize >= left_size {
            return Err(Error::source(path, i + 2, "context id out of range"));
        }
        // Costs are stored as i16; lindera truncates the same way.
        costs[right as usize + left as usize * right_size] = cost as i32 as i16;
    }
    Ok(Matrix {
        right_size,
        left_size,
        costs,
    })
}

/// Parses the next whitespace-separated integer of a matrix.def line.
fn next_int(line: &[u8], pos: &mut usize) -> Option<i64> {
    while *pos < line.len() && matches!(line[*pos], b' ' | b'\t' | b'\r') {
        *pos += 1;
    }
    if *pos >= line.len() {
        return None;
    }
    let negative = line[*pos] == b'-';
    if negative {
        *pos += 1;
    }
    let start = *pos;
    let mut value: i64 = 0;
    while *pos < line.len() && line[*pos].is_ascii_digit() {
        value = value
            .saturating_mul(10)
            .saturating_add((line[*pos] - b'0') as i64);
        *pos += 1;
    }
    if *pos == start {
        return None;
    }
    Some(if negative { -value } else { value })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_rows_are_indexed_by_left_id() {
        // Header: 2 right ids, 3 left ids. Line: right_id 1, left_id 2, cost 7.
        let m = parse_matrix(Path::new("matrix.def"), b"2 3\n1 2 7\n\n0 0 -5\r\n").unwrap();
        assert_eq!((m.right_size, m.left_size), (2, 3));
        assert_eq!(m.row(2), [i16::MAX, 7]);
        assert_eq!(m.row(0), [-5, i16::MAX]);
        assert!(parse_matrix(Path::new("matrix.def"), b"2 3\n2 0 1\n").is_err());
        assert!(parse_matrix(Path::new("matrix.def"), b"2 3\n1 1\n").is_err());
    }

    #[test]
    fn system_rows_skip_blank_columns_and_reject_bad_numbers() {
        let mut rows = Vec::new();
        read_system_rows(
            Path::new("a.csv"),
            "가,1,2,3,NNG,*\n,1,2,3,NNG\n나, 1 ,2,3\n",
            &mut rows,
        )
        .unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(
            rows[1].entry,
            Entry {
                left_id: 1,
                right_id: 2,
                cost: 3
            }
        );
        assert_eq!(rows[1].features, "");
        assert!(read_system_rows(Path::new("a.csv"), "가,x,2,3\n", &mut rows).is_err());
    }
}
