//! Minimal CSV record reader for dictionary sources.
//!
//! Follows the conventions of the `csv` crate that lindera reads ko-dic with: comma
//! separator, double-quote quoting with `""` as an escaped quote, `\n`, `\r\n` or `\r` as
//! the record terminator, blank lines skipped, no comment syntax, fields not trimmed.

use std::borrow::Cow;

pub(crate) struct Records<'a> {
    text: &'a str,
    pos: usize,
    line: usize,
}

impl<'a> Records<'a> {
    pub(crate) fn new(text: &'a str) -> Self {
        let text = text.strip_prefix('\u{feff}').unwrap_or(text);
        Records {
            text,
            pos: 0,
            line: 1,
        }
    }

    /// Reads the next record into `fields` and returns its 1-based starting line number,
    /// or `None` at the end of the input.
    pub(crate) fn next_record(&mut self, fields: &mut Vec<Cow<'a, str>>) -> Option<usize> {
        fields.clear();
        let bytes = self.text.as_bytes();
        // Skip blank lines.
        while self.pos < bytes.len() && matches!(bytes[self.pos], b'\n' | b'\r') {
            self.consume_line_end();
        }
        if self.pos >= bytes.len() {
            return None;
        }
        let start_line = self.line;
        loop {
            let (field, at_record_end) = self.read_field();
            fields.push(field);
            if at_record_end {
                return Some(start_line);
            }
        }
    }

    fn consume_line_end(&mut self) {
        let bytes = self.text.as_bytes();
        if bytes[self.pos] == b'\r' && bytes.get(self.pos + 1) == Some(&b'\n') {
            self.pos += 2;
        } else {
            self.pos += 1;
        }
        self.line += 1;
    }

    /// Reads one field. Returns it and whether it ended the record.
    fn read_field(&mut self) -> (Cow<'a, str>, bool) {
        let bytes = self.text.as_bytes();
        if bytes.get(self.pos) != Some(&b'"') {
            let start = self.pos;
            while self.pos < bytes.len() && !matches!(bytes[self.pos], b',' | b'\n' | b'\r') {
                self.pos += 1;
            }
            let field = Cow::Borrowed(&self.text[start..self.pos]);
            return (field, self.end_field());
        }
        // Quoted field: "" is a literal quote; text after the closing quote is kept, as the
        // csv crate does.
        self.pos += 1;
        let mut value = String::new();
        let mut chunk_start = self.pos;
        loop {
            match bytes.get(self.pos) {
                None => {
                    value.push_str(&self.text[chunk_start..self.pos]);
                    return (Cow::Owned(value), true);
                }
                Some(b'"') if bytes.get(self.pos + 1) == Some(&b'"') => {
                    value.push_str(&self.text[chunk_start..=self.pos]);
                    self.pos += 2;
                    chunk_start = self.pos;
                }
                Some(b'"') => {
                    value.push_str(&self.text[chunk_start..self.pos]);
                    self.pos += 1;
                    let rest_start = self.pos;
                    while self.pos < bytes.len() && !matches!(bytes[self.pos], b',' | b'\n' | b'\r')
                    {
                        self.pos += 1;
                    }
                    value.push_str(&self.text[rest_start..self.pos]);
                    return (Cow::Owned(value), self.end_field());
                }
                Some(b'\n') => {
                    self.line += 1;
                    self.pos += 1;
                }
                Some(_) => self.pos += 1,
            }
        }
    }

    /// Consumes the separator after a field. Returns true when the record ends here.
    fn end_field(&mut self) -> bool {
        let bytes = self.text.as_bytes();
        match bytes.get(self.pos) {
            Some(b',') => {
                self.pos += 1;
                false
            }
            Some(b'\n' | b'\r') => {
                self.consume_line_end();
                true
            }
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all(text: &str) -> Vec<(usize, Vec<String>)> {
        let mut records = Records::new(text);
        let mut fields = Vec::new();
        let mut out = Vec::new();
        while let Some(line) = records.next_record(&mut fields) {
            out.push((line, fields.iter().map(|f| f.to_string()).collect()));
        }
        out
    }

    #[test]
    fn plain_quoted_and_blank_lines() {
        let got = all("a,1,2\n\n\",\",3,\"x\"\"y\"\r\nlast");
        assert_eq!(
            got,
            vec![
                (1, vec!["a".into(), "1".into(), "2".into()]),
                (3, vec![",".into(), "3".into(), "x\"y".into()]),
                (4, vec!["last".into()]),
            ]
        );
    }

    #[test]
    fn trailing_empty_field_is_kept() {
        assert_eq!(all("a,\n"), vec![(1, vec!["a".into(), "".into()])]);
    }
}
