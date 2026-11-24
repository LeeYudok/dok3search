//! Symmetric synonym groups with built-in defaults and optional file override.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::sync::{OnceLock, RwLock};

const BUILTIN_SYNONYM_GROUPS: &[&[&str]] = &[
    &["장애", "트러블슈팅", "오류", "에러", "error"],
    &["폐쇄망", "에어갭", "오프라인", "airgap", "offline"],
    &["러스트", "rust"],
    &["고랭", "golang", "go"],
    &["인증", "로그인", "sso", "oidc", "oauth"],
    &["검색", "조회", "search"],
    &["배포", "릴리스", "롤아웃", "deploy"],
    &["색인", "인덱스", "index"],
    &["보안", "취약점", "cve", "security"],
    &["데이터베이스", "db", "database"],
];

#[derive(Debug, Clone)]
struct SynonymTable {
    /// Lowercase term -> peer lowercase terms (excluding self).
    peers: HashMap<String, Vec<String>>,
    /// Lowercase term -> canonical group key (first term of group) for semantic hashing.
    canonical: HashMap<String, String>,
}

impl SynonymTable {
    fn from_groups(groups: impl Iterator<Item = Vec<String>>) -> Self {
        let mut peer_sets: HashMap<String, HashSet<String>> = HashMap::new();
        let mut canonical: HashMap<String, String> = HashMap::new();

        for group in groups {
            let clean: Vec<String> = group
                .into_iter()
                .map(|s| s.trim().to_lowercase())
                .filter(|s| !s.is_empty())
                .collect();
            if clean.len() < 2 {
                continue;
            }
            let canon = clean[0].clone();
            for term in &clean {
                canonical
                    .entry(term.clone())
                    .or_insert_with(|| canon.clone());
                let entry = peer_sets.entry(term.clone()).or_default();
                for other in &clean {
                    if other != term {
                        entry.insert(other.clone());
                    }
                }
            }
        }

        let peers = peer_sets
            .into_iter()
            .map(|(k, set)| {
                let mut v: Vec<String> = set.into_iter().collect();
                v.sort();
                (k, v)
            })
            .collect();

        Self { peers, canonical }
    }

    fn builtin() -> Self {
        Self::from_groups(
            BUILTIN_SYNONYM_GROUPS
                .iter()
                .map(|grp| grp.iter().map(|s| (*s).to_string()).collect()),
        )
    }
}

static SYNONYM_TABLE: OnceLock<RwLock<SynonymTable>> = OnceLock::new();

fn table_lock() -> &'static RwLock<SynonymTable> {
    SYNONYM_TABLE.get_or_init(|| RwLock::new(SynonymTable::builtin()))
}

/// Parses a comma-separated synonym file (`#` comments and blank lines ignored)
/// and merges it with built-in synonym groups.
pub fn load_synonyms_file(path: &Path) -> Result<usize, String> {
    let raw = fs::read_to_string(path)
        .map_err(|err| format!("failed to read synonyms file {}: {err}", path.display()))?;
    let mut groups: Vec<Vec<String>> = BUILTIN_SYNONYM_GROUPS
        .iter()
        .map(|grp| grp.iter().map(|s| (*s).to_string()).collect())
        .collect();
    let mut custom_count = 0usize;
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let terms: Vec<String> = trimmed
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect();
        if terms.len() >= 2 {
            groups.push(terms);
            custom_count += 1;
        }
    }
    let new_table = SynonymTable::from_groups(groups.into_iter());
    let mut guard = table_lock()
        .write()
        .map_err(|_| "synonym table lock poisoned".to_string())?;
    *guard = new_table;
    Ok(custom_count)
}

/// Configures runtime synonyms from an optional file path (resets to built-in when `None`).
pub fn configure_synonyms(path: Option<&Path>) -> Result<usize, String> {
    if let Some(p) = path {
        load_synonyms_file(p)
    } else {
        let mut guard = table_lock()
            .write()
            .map_err(|_| "synonym table lock poisoned".to_string())?;
        *guard = SynonymTable::builtin();
        Ok(0)
    }
}

/// Returns symmetric synonym expansions for `term` (lowercased, deduplicated, excluding self).
#[must_use]
pub fn expand_synonyms(term: &str) -> Vec<String> {
    let key = term.trim().to_lowercase();
    if key.is_empty() {
        return Vec::new();
    }
    let guard = match table_lock().read() {
        Ok(g) => g,
        Err(_) => return Vec::new(),
    };
    guard.peers.get(&key).cloned().unwrap_or_default()
}

/// Returns a canonical synonym representative for `term` if one exists.
#[must_use]
pub fn canonical_synonym(term: &str) -> Option<String> {
    let key = term.trim().to_lowercase();
    if key.is_empty() {
        return None;
    }
    let guard = table_lock().read().ok()?;
    guard.canonical.get(&key).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_synonyms_expand_symmetrically() {
        let peers = expand_synonyms("에어갭");
        assert!(peers.contains(&"폐쇄망".to_string()));
        assert!(peers.contains(&"airgap".to_string()));
        assert!(!peers.contains(&"에어갭".to_string()));

        let reverse = expand_synonyms("폐쇄망");
        assert!(reverse.contains(&"에어갭".to_string()));
    }

    #[test]
    fn canonical_maps_synonyms_to_same_key() {
        assert_eq!(canonical_synonym("트러블슈팅"), canonical_synonym("장애"));
        assert_eq!(canonical_synonym("rust"), canonical_synonym("러스트"));
    }
}
