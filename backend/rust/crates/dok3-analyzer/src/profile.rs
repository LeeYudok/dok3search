//! Analyzer profile selection and mecab-ko-dic runtime management.

use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::{Arc, OnceLock, RwLock};

use crate::kodic;
use crate::{AnalyzeOptions, AnalyzedTokens, generate_char_ngrams, identifier, korean};

/// Base analyzer version for the built-in heuristic Korean analyzer.
pub const BUILTIN_ANALYZER_VERSION: u64 = 2;

/// Morphological analysis profile used for Korean text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnalyzerProfile {
    /// Zero-setup built-in heuristic dictionary (`korean.rs`).
    #[default]
    Builtin,
    /// `mecab-ko-dic` Normal mode compatible with Lindera 6.2.
    Lindera,
    /// Nori-style `mixed` decomposition: keeps compound surfaces + parts, drops J*/E*/S*.
    NoriMixed,
    /// Nori-style `discard` decomposition: emits compound parts only, drops J*/E*/S*.
    NoriDiscard,
}

impl AnalyzerProfile {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Builtin => "builtin",
            Self::Lindera => "lindera",
            Self::NoriMixed => "nori-mixed",
            Self::NoriDiscard => "nori-discard",
        }
    }

    #[must_use]
    pub const fn requires_dictionary(self) -> bool {
        !matches!(self, Self::Builtin)
    }
}

impl fmt::Display for AnalyzerProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for AnalyzerProfile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "builtin" => Ok(Self::Builtin),
            "lindera" => Ok(Self::Lindera),
            "nori-mixed" => Ok(Self::NoriMixed),
            "nori-discard" => Ok(Self::NoriDiscard),
            other => Err(format!(
                "invalid analyzer profile '{other}' (expected builtin, lindera, nori-mixed, or nori-discard)"
            )),
        }
    }
}

/// Configuration used to construct an [`AnalyzerEngine`].
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AnalyzerConfig {
    pub profile: AnalyzerProfile,
    pub dict_path: Option<PathBuf>,
    pub user_dict_path: Option<PathBuf>,
}

/// Summary of the active analyzer runtime configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnalyzerRuntimeInfo {
    pub profile: AnalyzerProfile,
    pub version_tag: u64,
    pub dictionary_loaded: bool,
}

/// Self-contained text analyzer bound to a specific [`AnalyzerProfile`] and optional dictionary.
pub struct AnalyzerEngine {
    profile: AnalyzerProfile,
    kodic: Option<kodic::Analyzer>,
    version_tag: u64,
}

impl AnalyzerEngine {
    /// Returns the default built-in heuristic analyzer (`version_tag == 2`).
    #[must_use]
    pub fn builtin() -> Self {
        Self {
            profile: AnalyzerProfile::Builtin,
            kodic: None,
            version_tag: BUILTIN_ANALYZER_VERSION,
        }
    }

    /// Builds an analyzer engine from `config`.
    pub fn from_config(config: &AnalyzerConfig) -> Result<Self, String> {
        if !config.profile.requires_dictionary() {
            return Ok(Self::builtin());
        }

        let dict_path = config.dict_path.as_deref().ok_or_else(|| {
            format!(
                "DOK3_DICT_PATH is required when analyzer profile is '{}'",
                config.profile
            )
        })?;
        let kodic_analyzer = kodic::Analyzer::load(dict_path, config.user_dict_path.as_deref())
            .map_err(|err| {
                format!(
                    "failed to load mecab-ko-dic dictionary from {}: {err}",
                    dict_path.display()
                )
            })?;
        let version_tag = compute_version_tag(
            config.profile,
            &kodic_analyzer,
            dict_path,
            config.user_dict_path.as_deref(),
        )?;

        Ok(Self {
            profile: config.profile,
            kodic: Some(kodic_analyzer),
            version_tag,
        })
    }

    #[must_use]
    pub const fn profile(&self) -> AnalyzerProfile {
        self.profile
    }

    #[must_use]
    pub const fn version_tag(&self) -> u64 {
        self.version_tag
    }

    #[must_use]
    pub const fn dictionary_loaded(&self) -> bool {
        self.kodic.is_some()
    }

    #[must_use]
    pub fn analyze_text(&self, text: &str, options: AnalyzeOptions) -> AnalyzedTokens {
        AnalyzedTokens {
            terms: self.analyze_terms(text, options.split_identifiers),
            char_ngrams: if options.char_ngrams {
                generate_char_ngrams(text)
            } else {
                Vec::new()
            },
        }
    }

    #[must_use]
    pub fn normalize_query_terms(&self, query: &str) -> Vec<String> {
        let mut seen = std::collections::HashSet::new();
        self.analyze_terms(query, true)
            .into_iter()
            .filter(|term| seen.insert(term.clone()))
            .collect()
    }

    fn analyze_terms(&self, text: &str, split_identifiers: bool) -> Vec<String> {
        let mut terms = Vec::new();
        for word in identifier::words(text) {
            let lower = word.to_lowercase();
            terms.push(lower.clone());
            if split_identifiers {
                terms.extend(identifier::components(word).map(str::to_lowercase));
                if let Some(kodic) = &self.kodic {
                    if contains_hangul(word) {
                        for token in extract_kodic_terms(kodic, self.profile, word) {
                            if token != lower {
                                terms.push(token);
                            }
                        }
                    }
                } else {
                    terms.extend(korean::expansions(word).map(str::to_owned));
                }
            }
        }
        terms
    }
}

fn contains_hangul(word: &str) -> bool {
    word.chars()
        .any(|c| matches!(c, '\u{AC00}'..='\u{D7A3}' | '\u{1100}'..='\u{11FF}' | '\u{3130}'..='\u{318F}'))
}

fn extract_kodic_terms(
    analyzer: &kodic::Analyzer,
    profile: AnalyzerProfile,
    word: &str,
) -> Vec<String> {
    let mut out = Vec::new();
    for morpheme in analyzer.analyze(word) {
        let surface = morpheme.surface(word);
        if surface.is_empty() {
            continue;
        }
        let features = analyzer.features(morpheme.word);
        let pos = features.pos();

        if matches!(
            profile,
            AnalyzerProfile::NoriMixed | AnalyzerProfile::NoriDiscard
        ) && is_filtered_pos(pos)
        {
            continue;
        }

        let is_compound = features.kind().eq_ignore_ascii_case("Compound");
        match profile {
            AnalyzerProfile::Builtin => {}
            AnalyzerProfile::Lindera => {
                out.push(surface.to_lowercase());
            }
            AnalyzerProfile::NoriMixed => {
                out.push(surface.to_lowercase());
                if is_compound {
                    push_compound_parts(features.expression(), &mut out);
                }
            }
            AnalyzerProfile::NoriDiscard => {
                if is_compound {
                    let before = out.len();
                    push_compound_parts(features.expression(), &mut out);
                    if out.len() == before {
                        out.push(surface.to_lowercase());
                    }
                } else {
                    out.push(surface.to_lowercase());
                }
            }
        }
    }
    out
}

fn is_filtered_pos(pos: &str) -> bool {
    let primary = pos.split('+').next().unwrap_or(pos);
    primary.starts_with('J') || primary.starts_with('E') || primary.starts_with('S')
}

fn push_compound_parts(expression: &str, out: &mut Vec<String>) {
    if expression.is_empty() || expression == "*" {
        return;
    }
    for part in expression.split('+') {
        let surface = part.split('/').next().unwrap_or("").trim();
        if !surface.is_empty() && surface != "*" {
            out.push(surface.to_lowercase());
        }
    }
}

fn compute_version_tag(
    profile: AnalyzerProfile,
    analyzer: &kodic::Analyzer,
    dict_path: &Path,
    user_dict_path: Option<&Path>,
) -> Result<u64, String> {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    fn mix(hash: &mut u64, bytes: &[u8]) {
        for &b in bytes {
            *hash ^= u64::from(b);
            *hash = hash.wrapping_mul(0x0100_0000_01b3);
        }
    }

    mix(&mut hash, &BUILTIN_ANALYZER_VERSION.to_le_bytes());
    mix(&mut hash, profile.as_str().as_bytes());
    let info = analyzer.dictionary().info();
    mix(&mut hash, info.label.as_bytes());
    mix(&mut hash, &info.entries.to_le_bytes());
    mix(&mut hash, &info.surfaces.to_le_bytes());
    mix(&mut hash, &info.trie_nodes.to_le_bytes());
    mix(&mut hash, &info.unknown_entries.to_le_bytes());
    let dict_len = fs::metadata(dict_path)
        .map_err(|err| format!("failed to stat dictionary {}: {err}", dict_path.display()))?
        .len();
    mix(&mut hash, &dict_len.to_le_bytes());

    if let Some(user_path) = user_dict_path {
        let user_bytes = fs::read(user_path).map_err(|err| {
            format!(
                "failed to read user dictionary {}: {err}",
                user_path.display()
            )
        })?;
        mix(&mut hash, b"user:");
        mix(&mut hash, &user_bytes);
    }

    // Keep version tags distinct from legacy numeric constants (1, 2).
    Ok(hash | (1u64 << 63))
}

static ACTIVE_ENGINE: OnceLock<RwLock<Arc<AnalyzerEngine>>> = OnceLock::new();

fn runtime_lock() -> &'static RwLock<Arc<AnalyzerEngine>> {
    ACTIVE_ENGINE.get_or_init(|| RwLock::new(Arc::new(AnalyzerEngine::builtin())))
}

pub(crate) fn current_engine() -> Arc<AnalyzerEngine> {
    let guard = runtime_lock()
        .read()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    Arc::clone(&guard)
}

/// Configures the process-wide analyzer runtime used by [`analyze_text`] and
/// [`normalize_query_terms`].
pub fn configure_runtime(config: &AnalyzerConfig) -> Result<AnalyzerRuntimeInfo, String> {
    let engine = Arc::new(AnalyzerEngine::from_config(config)?);
    let info = AnalyzerRuntimeInfo {
        profile: engine.profile(),
        version_tag: engine.version_tag(),
        dictionary_loaded: engine.dictionary_loaded(),
    };
    let mut guard = runtime_lock()
        .write()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    *guard = engine;
    Ok(info)
}

/// Returns the version tag of the active process-wide analyzer runtime.
#[must_use]
pub fn active_analyzer_version() -> u64 {
    current_engine().version_tag()
}

/// Returns the active process-wide analyzer profile.
#[must_use]
pub fn active_profile() -> AnalyzerProfile {
    current_engine().profile()
}
