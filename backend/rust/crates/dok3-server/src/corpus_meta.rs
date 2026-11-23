//! Corpus manifest (`index.json` / `provenance.json`) loader for local and persistent indexing.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use dok3_domain::Document;
use serde_json::Value;

#[derive(Debug, Clone)]
pub(crate) struct CorpusChapterMeta {
    pub source: String,
    pub category: String,
    pub topic: Option<String>,
    pub version: String,
    pub summary: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Default)]
pub(crate) struct CorpusMetadataIndex {
    by_rel_path: HashMap<String, CorpusChapterMeta>,
}

impl CorpusMetadataIndex {
    /// Discovers `index.json` / `provenance.json` manifests under `root` and indexes chapter metadata
    /// keyed by relative path from `root`.
    pub(crate) fn load(root: &Path) -> Self {
        let mut out = Self::default();
        if root.is_dir() {
            walk_manifests(root, root, &mut out);
        }
        out
    }

    pub(crate) fn get(&self, relative_path: &str) -> Option<&CorpusChapterMeta> {
        let normalized = relative_path.replace('\\', "/");
        self.by_rel_path.get(&normalized)
    }

    pub(crate) fn apply(
        &self,
        doc: &mut Document,
        relative_path: &str,
    ) -> Option<CorpusChapterMeta> {
        let meta = self.get(relative_path)?.clone();
        doc.source = meta.source.clone();
        doc.category = meta.category.clone();
        doc.topic = meta.topic.clone();
        doc.version = meta.version.clone();
        if let Some(summary) = meta.summary.as_deref().filter(|s| !s.trim().is_empty()) {
            doc.summary = summary.to_string();
        }
        for tag in &meta.tags {
            let trimmed = tag.trim();
            if !trimmed.is_empty() && !doc.tags.iter().any(|t| t.eq_ignore_ascii_case(trimmed)) {
                doc.tags.push(trimmed.to_string());
            }
        }
        if let Some(primary) = meta.tags.first() {
            doc.tag = primary.clone();
        }
        Some(meta)
    }
}

fn walk_manifests(root: &Path, dir: &Path, out: &mut CorpusMetadataIndex) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    let mut paths: Vec<_> = entries.filter_map(Result::ok).map(|e| e.path()).collect();
    paths.sort();

    if dir.join("index.json").is_file() || dir.join("provenance.json").is_file() {
        load_corpus_dir(root, dir, out);
    }

    for path in paths {
        if let Some(name) = path.file_name().and_then(|n| n.to_str())
            && name.starts_with('.')
        {
            continue;
        }
        if path.is_dir() {
            walk_manifests(root, &path, out);
        }
    }
}

fn load_corpus_dir(root: &Path, dir: &Path, out: &mut CorpusMetadataIndex) {
    let Some(slug) = dir.file_name().and_then(|n| n.to_str()) else {
        return;
    };
    let rel_prefix = dir
        .strip_prefix(root)
        .ok()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_default();

    let index_json = fs::read_to_string(dir.join("index.json"))
        .ok()
        .and_then(|raw| serde_json::from_str::<Value>(&raw).ok());
    let prov_json = fs::read_to_string(dir.join("provenance.json"))
        .ok()
        .and_then(|raw| serde_json::from_str::<Value>(&raw).ok());

    let (default_category, default_topic) = match slug {
        "rust-std" | "rust-crates" | "rustc-error-codes" => ("code", Some("rust".to_string())),
        "rust-reference" | "cargo-book" | "rust-book" | "comprehensive-rust" => {
            ("guide", Some("rust".to_string()))
        }
        "go-reference" => ("guide", Some("go".to_string())),
        "design" => ("design", None),
        _ => ("guide", None),
    };

    let corpus_source = index_json
        .as_ref()
        .and_then(|v| v.get("source"))
        .and_then(Value::as_str)
        .map(str::to_string)
        .unwrap_or_else(|| slug.to_string());

    let corpus_version = extract_corpus_version(slug, index_json.as_ref(), prov_json.as_ref());

    if let Some(chapters) = index_json
        .as_ref()
        .and_then(|v| v.get("chapters"))
        .and_then(Value::as_array)
    {
        for chapter in chapters {
            let Some(ch_path) = chapter.get("path").and_then(Value::as_str) else {
                continue;
            };
            let full_rel = if rel_prefix.is_empty() {
                ch_path.to_string()
            } else {
                format!("{rel_prefix}/{ch_path}")
            };

            let source = chapter
                .get("source")
                .and_then(Value::as_str)
                .unwrap_or(&corpus_source)
                .to_string();
            let category = chapter
                .get("category")
                .and_then(Value::as_str)
                .unwrap_or(default_category)
                .to_string();
            let version = chapter
                .get("package_version")
                .and_then(Value::as_str)
                .or_else(|| chapter.get("version").and_then(Value::as_str))
                .unwrap_or(&corpus_version)
                .to_string();
            let summary = chapter
                .get("summary")
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string);

            let mut tags = Vec::new();
            if let Some(pkg) = chapter.get("package").and_then(Value::as_str) {
                tags.push(pkg.to_string());
            }
            if let Some(arr) = chapter.get("tags").and_then(Value::as_array) {
                for item in arr {
                    if let Some(s) = item.as_str() {
                        tags.push(s.to_string());
                    }
                }
            }
            let topic = chapter
                .get("topic")
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .or_else(|| default_topic.clone());
            if let Some(topic) = &topic {
                tags.push(topic.clone());
            }

            out.by_rel_path.insert(
                full_rel,
                CorpusChapterMeta {
                    source,
                    category,
                    topic,
                    version,
                    summary,
                    tags,
                },
            );
        }
    } else if slug == "design" {
        // design/ has provenance.json without index.json; tag every .md/.yaml under it.
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_file()
                    && let Some(name) = path.file_name().and_then(|n| n.to_str())
                    && (name.ends_with(".md") || name.ends_with(".yaml") || name.ends_with(".yml"))
                {
                    let full_rel = if rel_prefix.is_empty() {
                        name.to_string()
                    } else {
                        format!("{rel_prefix}/{name}")
                    };
                    out.by_rel_path.insert(
                        full_rel,
                        CorpusChapterMeta {
                            source: "dok3search-design".to_string(),
                            category: "design".to_string(),
                            topic: None,
                            version: corpus_version.clone(),
                            summary: None,
                            tags: vec!["dok3search-design".to_string()],
                        },
                    );
                }
            }
        }
    }
}

fn extract_corpus_version(
    slug: &str,
    index_json: Option<&Value>,
    prov_json: Option<&Value>,
) -> String {
    if let Some(v) = index_json
        .and_then(|v| v.get("toolchain").or_else(|| v.get("version")))
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty())
    {
        return v.to_string();
    }
    if let Some(prov) = prov_json {
        if let Some(v) = prov
            .get("toolchain")
            .and_then(Value::as_str)
            .filter(|s| !s.trim().is_empty())
        {
            return v.to_string();
        }
        if let Some(v) = prov
            .get("release")
            .and_then(Value::as_str)
            .filter(|s| !s.trim().is_empty())
        {
            return v.to_string();
        }
        if let Some(v) = prov
            .pointer("/sources/spec/language_version")
            .and_then(Value::as_str)
            .filter(|s| !s.trim().is_empty())
        {
            return v.to_string();
        }
        if let Some(commit) = prov
            .get("commit")
            .and_then(Value::as_str)
            .filter(|s| s.len() >= 12)
        {
            return commit[..12].to_string();
        }
        if let Some(sha) = index_json
            .and_then(|v| v.get("printSha256"))
            .and_then(Value::as_str)
            .filter(|s| s.len() >= 12)
        {
            return format!("snapshot-{}", &sha[..12]);
        }
    }
    if slug == "comprehensive-rust" {
        return "ko-snapshot".to_string();
    }
    "v1".to_string()
}
