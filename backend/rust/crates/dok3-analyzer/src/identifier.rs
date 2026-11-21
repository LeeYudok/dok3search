pub(super) fn words(text: &str) -> impl Iterator<Item = &str> {
    let mut chars = text.char_indices().peekable();

    std::iter::from_fn(move || {
        let (start, mut previous) = chars.find(|&(_, ch)| ch.is_alphanumeric())?;
        let mut end = start + previous.len_utf8();

        while let Some((index, ch)) = chars.next() {
            if ch == '_' {
                previous = ch;
                continue;
            }
            if !ch.is_alphanumeric()
                && !chars
                    .peek()
                    .is_some_and(|&(_, next)| joins_word(ch, previous, next))
            {
                break;
            }
            end = index + ch.len_utf8();
            previous = ch;
        }

        Some(&text[start..end])
    })
}

fn joins_word(separator: char, previous: char, next: char) -> bool {
    match separator {
        '-' | '/' => previous.is_ascii_alphabetic() && next.is_ascii_digit(),
        '.' => previous.is_ascii_digit() && next.is_ascii_digit(),
        _ => false,
    }
}

pub(super) fn components(word: &str) -> impl Iterator<Item = &str> {
    word.split(|ch: char| !ch.is_alphanumeric())
        .flat_map(case_segments)
        .filter(move |part| !part.is_empty() && *part != word)
}

fn case_segments(segment: &str) -> impl Iterator<Item = &str> {
    let mut chars = segment.char_indices();
    let mut previous = chars.next().map(|(_, ch)| ch);
    let mut start = 0;

    std::iter::from_fn(move || {
        for (index, ch) in chars.by_ref() {
            let prior = previous.replace(ch)?;
            if ch.is_uppercase()
                && (prior.is_lowercase()
                    || prior.is_numeric()
                    || (prior.is_uppercase()
                        && segment[index + ch.len_utf8()..].starts_with(char::is_lowercase)))
            {
                let part = &segment[start..index];
                start = index;
                return Some(part);
            }
        }

        (start < segment.len()).then(|| {
            let tail = &segment[start..];
            start = segment.len();
            tail
        })
    })
}
