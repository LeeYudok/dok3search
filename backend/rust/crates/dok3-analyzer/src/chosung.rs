//! Korean Hangul initial consonant (chosung) extraction and query detection.

const CHOSUNG_TABLE: [char; 19] = [
    'ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ', 'ㅋ',
    'ㅌ', 'ㅍ', 'ㅎ',
];

const HANGUL_SYLLABLE_START: u32 = 0xAC00;
const HANGUL_SYLLABLE_END: u32 = 0xD7A3;

fn is_chosung_jamo(ch: char) -> bool {
    CHOSUNG_TABLE.contains(&ch)
}

/// Extracts Hangul initial consonants from `text`, preserving ASCII alphanumerics and whitespace.
#[must_use]
pub fn extract_chosung(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut prev_space = true;
    for ch in text.chars() {
        let code = ch as u32;
        if (HANGUL_SYLLABLE_START..=HANGUL_SYLLABLE_END).contains(&code) {
            let idx = ((code - HANGUL_SYLLABLE_START) / (21 * 28)) as usize;
            if let Some(&jamo) = CHOSUNG_TABLE.get(idx) {
                out.push(jamo);
                prev_space = false;
            }
        } else if is_chosung_jamo(ch) || ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            prev_space = false;
        } else if !prev_space {
            out.push(' ');
            prev_space = true;
        }
    }
    out.trim().to_string()
}

/// Returns `true` when `query` has at least two non-whitespace characters and every
/// non-whitespace character is a Hangul initial consonant jamo.
#[must_use]
pub fn is_chosung_query(query: &str) -> bool {
    let mut count = 0usize;
    for ch in query.chars() {
        if ch.is_whitespace() {
            continue;
        }
        if !is_chosung_jamo(ch) {
            return false;
        }
        count += 1;
    }
    count >= 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_chosung_for_korean_and_mixed_titles() {
        assert_eq!(extract_chosung("러스트 강의"), "ㄹㅅㅌ ㄱㅇ");
        assert_eq!(extract_chosung("폐쇄망 개발 가이드"), "ㅍㅅㅁ ㄱㅂ ㄱㅇㄷ");
        assert_eq!(extract_chosung("Rust 소유권"), "rust ㅅㅇㄱ");
    }

    #[test]
    fn detects_chosung_only_queries() {
        assert!(is_chosung_query("ㄹㅅㅌ"));
        assert!(is_chosung_query("ㅍㅅㅁ ㄱㅂ"));
        assert!(!is_chosung_query("ㄱ"));
        assert!(!is_chosung_query("러스트"));
        assert!(!is_chosung_query("rust"));
    }
}
