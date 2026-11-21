/// Returns lowercase bigrams and trigrams from each alphanumeric run.
///
/// Windows count Unicode scalar values in the lowercase text. For each start
/// position, the bigram precedes the trigram. Whitespace and punctuation
/// terminate a run, and repeated grams are retained.
pub fn generate_char_ngrams(text: &str) -> Vec<String> {
    let mut grams = Vec::new();
    for word in text.split(|ch: char| !ch.is_alphanumeric()) {
        let lowercase = word.to_lowercase();
        let mut chars = lowercase.chars();
        while let Some(first) = chars.next() {
            let mut following = chars.clone();
            let Some(second) = following.next() else {
                break;
            };
            grams.push(String::from_iter([first, second]));
            if let Some(third) = following.next() {
                grams.push(String::from_iter([first, second, third]));
            }
        }
    }
    grams
}
