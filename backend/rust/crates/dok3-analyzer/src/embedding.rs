//! Pure-Rust deterministic feature-hashing text embeddings for hybrid/semantic search.

use crate::synonyms::canonical_synonym;
use crate::{AnalyzeOptions, analyze_text};

/// Fixed dimensionality for lightweight feature-hashed embeddings.
pub const EMBEDDING_DIM: usize = 128;

const FNV_OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0100_0000_01b3;

fn fnv1a_64(bytes: &[u8]) -> u64 {
    let mut hash = FNV_OFFSET_BASIS;
    for &b in bytes {
        hash ^= u64::from(b);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

fn add_feature(vec: &mut [f32; EMBEDDING_DIM], key: &str, weight: f32) {
    let h = fnv1a_64(key.as_bytes());
    let idx = (h as usize) % EMBEDDING_DIM;
    let sign = if ((h >> 63) & 1) == 0 {
        1.0_f32
    } else {
        -1.0_f32
    };
    vec[idx] += sign * weight;
}

/// Computes an L2-normalized 128-dim embedding from analyzed tokens, synonym canonicals,
/// and character n-grams.
#[must_use]
pub fn embed_text(text: &str) -> [f32; EMBEDDING_DIM] {
    let mut vec = [0.0_f32; EMBEDDING_DIM];
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return vec;
    }

    let analyzed = analyze_text(
        trimmed,
        AnalyzeOptions {
            split_identifiers: true,
            char_ngrams: true,
        },
    );

    for term in &analyzed.terms {
        add_feature(&mut vec, term, 1.0);
        if let Some(canon) = canonical_synonym(term) {
            add_feature(&mut vec, &format!("syn:{canon}"), 1.3);
        }
    }
    for gram in &analyzed.char_ngrams {
        if gram.chars().count() >= 3 {
            add_feature(&mut vec, &format!("g:{gram}"), 0.35);
        }
    }

    let norm: f32 = vec.iter().map(|v| v * v).sum::<f32>().sqrt();
    if norm > 0.0 {
        for v in &mut vec {
            *v /= norm;
        }
    }
    vec
}

/// Computes cosine similarity between two equal-length L2-normalized vectors.
#[must_use]
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn synonym_and_overlapping_terms_have_positive_similarity() {
        let v_airgap = embed_text("에어갭 환경 검색 구축");
        let v_closed = embed_text("폐쇄망 검색 가이드");
        let v_unrelated = embed_text("금융 계좌 이체 한도");

        let sim_related = cosine_similarity(&v_airgap, &v_closed);
        let sim_unrelated = cosine_similarity(&v_airgap, &v_unrelated);
        assert!(
            sim_related > sim_unrelated,
            "{sim_related} <= {sim_unrelated}"
        );
        assert!(sim_related > 0.2);
    }

    #[test]
    fn empty_text_produces_zero_vector() {
        let v = embed_text("   ");
        assert!(v.iter().all(|&x| x == 0.0));
    }
}
