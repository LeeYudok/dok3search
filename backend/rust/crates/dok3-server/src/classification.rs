//! Deterministic offline rule-based document classification and immutable-original storage.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use dok3_core::Dok3Error;
use dok3_domain::{
    ALLOWED_CLASSIFICATION_CATEGORIES, ClassificationSource, Document, DocumentClassification,
    DocumentId, MAX_CLASSIFICATION_TERMS, Principal, UpdateClassificationRequest,
    normalize_classification_terms, validate_classification_category,
    validate_classification_summary,
};
use serde::{Deserialize, Serialize};

use crate::persistence::{atomic_write_bytes, now_ms};

const MAX_RULE_SUMMARY_CHARS: usize = 200;

/// Offline deterministic classifier deriving category, summary, tags, and keywords.
pub(crate) struct RuleClassifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RuleOutput {
    pub category: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub keywords: Vec<String>,
}

impl RuleClassifier {
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn classify(doc: &Document, default_tag: &str) -> RuleOutput {
        Self::classify_with_defaults(doc, "guide", default_tag)
    }

    pub(crate) fn classify_with_defaults(
        doc: &Document,
        default_category: &str,
        default_tag: &str,
    ) -> RuleOutput {
        let path_lower = doc
            .sample_path
            .as_deref()
            .unwrap_or(&doc.source_uri)
            .to_ascii_lowercase();
        let title_lower = doc.title.to_lowercase();
        let body_lower = doc.body.to_lowercase();

        let category = classify_category(
            &path_lower,
            &title_lower,
            &doc.body,
            &body_lower,
            default_category,
        );
        let summary = summarize_document(&doc.title, &doc.body);
        let tags = extract_tags(doc, &category, default_tag, &path_lower, &body_lower);
        let keywords = extract_keywords(&doc.title, &doc.body, &doc.sections);

        RuleOutput {
            category,
            summary,
            tags,
            keywords,
        }
    }
}

fn classify_category(
    path_lower: &str,
    title_lower: &str,
    body_raw: &str,
    body_lower: &str,
    default_category: &str,
) -> String {
    if contains_any(
        path_lower,
        &["policy/", "policies/", "security/", "compliance/"],
    ) || contains_any(
        title_lower,
        &["규정", "정책", "지침", "보안", "권한", "반입 기준", "보존"],
    ) {
        return "policy".to_string();
    }
    if contains_any(
        path_lower,
        &["design/", "adr/", "architecture/", "contracts/"],
    ) || contains_any(title_lower, &["설계", "아키텍처", "architecture", "계약"])
    {
        return "design".to_string();
    }
    if contains_any(path_lower, &["manual/", "runbook/", "ops/"])
        || contains_any(
            title_lower,
            &[
                "매뉴얼",
                "운영",
                "복구",
                "백업",
                "설치",
                "점검",
                "재시도",
                "롤백",
            ],
        )
    {
        return "manual".to_string();
    }
    if has_code_extension(path_lower)
        || contains_any(path_lower, &["code/", "api/", "src/", "reference/"])
        || contains_any(title_lower, &["api", "오류", "error", "sql", "쿼리"])
        || has_error_or_code_markers(body_raw, body_lower)
    {
        return "code".to_string();
    }
    let normalized = default_category.trim().to_ascii_lowercase();
    if ALLOWED_CLASSIFICATION_CATEGORIES.contains(&normalized.as_str()) {
        normalized
    } else {
        "guide".to_string()
    }
}

const CODE_FILE_EXTENSIONS: [(&str, &str); 5] = [
    (".rs", "rust"),
    (".java", "java"),
    (".go", "go"),
    (".sql", "sql"),
    (".jsp", "jsp"),
];

fn has_code_extension(path_lower: &str) -> bool {
    CODE_FILE_EXTENSIONS
        .iter()
        .any(|(ext, _)| path_lower.ends_with(ext))
}

fn has_error_or_code_markers(body_raw: &str, body_lower: &str) -> bool {
    if body_raw.contains("ORA-") || body_raw.contains("```") {
        return true;
    }
    contains_any(
        body_lower,
        &["pub fn ", "pub struct ", "impl ", "async fn ", "select "],
    )
}

fn summarize_document(title: &str, body: &str) -> String {
    let mut parts: Vec<String> = Vec::new();
    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty()
            || trimmed.starts_with('#')
            || trimmed.starts_with("```")
            || trimmed.starts_with("---")
            || trimmed.starts_with('|')
        {
            continue;
        }
        let cleaned = strip_markdown_noise(trimmed);
        if cleaned.is_empty() || cleaned.eq_ignore_ascii_case(title.trim()) {
            continue;
        }
        parts.push(cleaned);
        if parts.iter().map(String::len).sum::<usize>() >= MAX_RULE_SUMMARY_CHARS {
            break;
        }
    }
    let joined = if parts.is_empty() {
        title.trim().to_string()
    } else {
        parts.join(" ")
    };
    truncate_chars(&joined, MAX_RULE_SUMMARY_CHARS)
}

fn strip_markdown_noise(line: &str) -> String {
    let mut out = line
        .trim_start_matches(['>', '-', '*', '+'])
        .trim()
        .replace('`', "");
    while let Some(open) = out.find('[') {
        let Some(mid) = out[open..].find("](").map(|i| open + i) else {
            break;
        };
        let Some(close) = out[mid + 2..].find(')').map(|i| mid + 2 + i) else {
            break;
        };
        let label = out[open + 1..mid].to_string();
        out.replace_range(open..=close, &label);
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn truncate_chars(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    text.chars().take(max_chars).collect()
}

fn extract_tags(
    doc: &Document,
    category: &str,
    default_tag: &str,
    path_lower: &str,
    body_lower: &str,
) -> Vec<String> {
    let mut tags: Vec<String> = Vec::new();
    let mut push_unique = |candidate: &str| {
        let trimmed = candidate.trim();
        if trimmed.is_empty()
            || tags.len() >= MAX_CLASSIFICATION_TERMS
            || tags.iter().any(|t| t.eq_ignore_ascii_case(trimmed))
        {
            return;
        }
        tags.push(trimmed.to_string());
    };

    if !doc.language.trim().is_empty() {
        push_unique(&doc.language);
    }
    push_unique(category);

    let combined = format!("{path_lower} {} {body_lower}", doc.title.to_lowercase());
    let domain_tags = [
        ("rust", "rust"),
        ("cargo", "cargo"),
        ("tantivy", "tantivy"),
        ("tokio", "tokio"),
        ("axum", "axum"),
        ("sqlite", "sqlite"),
        ("oracle", "oracle"),
        ("postgres", "postgres"),
        ("mcp", "mcp"),
        ("react", "react"),
        ("보안", "보안"),
        ("색인", "색인"),
        ("세대", "세대전환"),
        ("운영", "운영"),
        ("매뉴얼", "매뉴얼"),
    ];
    for (needle, label) in domain_tags {
        if combined.contains(needle) {
            push_unique(label);
        }
    }
    for (ext, label) in CODE_FILE_EXTENSIONS {
        if path_lower.ends_with(ext) {
            push_unique(label);
        }
    }
    if path_lower.ends_with(".html") || path_lower.ends_with(".htm") {
        push_unique("html");
    }
    if !default_tag.trim().is_empty() {
        push_unique(default_tag);
    }
    tags
}

fn push_keyword(keywords: &mut Vec<String>, token: &str) {
    let trimmed = token.trim_matches(|c: char| !c.is_alphanumeric() && c != '_' && c != '-');
    if trimmed.chars().count() < 2
        || is_stopword(trimmed)
        || keywords.len() >= MAX_CLASSIFICATION_TERMS
        || keywords.iter().any(|k| k.eq_ignore_ascii_case(trimmed))
    {
        return;
    }
    keywords.push(trimmed.to_string());
}

fn extract_keywords(
    title: &str,
    body: &str,
    sections: &[dok3_domain::DocumentSection],
) -> Vec<String> {
    let mut keywords: Vec<String> = Vec::new();

    // Prioritize section headings and title tokens.
    for word in title.split_whitespace() {
        push_keyword(&mut keywords, word);
    }
    for sec in sections {
        for word in sec.heading.split_whitespace() {
            push_keyword(&mut keywords, word);
        }
    }
    // Extract high-signal error codes, env vars, and snake/camel code identifiers from body.
    for raw in body.split(|c: char| !c.is_alphanumeric() && c != '_' && c != '-') {
        let is_signal = raw.starts_with("ORA-")
            || raw.starts_with("DOK3_")
            || (raw.contains('_') && raw.len() >= 4)
            || has_internal_uppercase(raw);
        if is_signal {
            push_keyword(&mut keywords, raw);
        }
    }
    for word in body.split_whitespace() {
        if keywords.len() >= MAX_CLASSIFICATION_TERMS {
            break;
        }
        push_keyword(&mut keywords, word);
    }
    keywords
}

fn has_internal_uppercase(token: &str) -> bool {
    let mut chars = token.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    first.is_ascii_lowercase() && chars.any(|c| c.is_ascii_uppercase())
}

fn is_stopword(token: &str) -> bool {
    let lower = token.to_ascii_lowercase();
    matches!(
        lower.as_str(),
        "the"
            | "and"
            | "for"
            | "with"
            | "this"
            | "that"
            | "from"
            | "into"
            | "are"
            | "was"
            | "will"
            | "have"
            | "not"
            | "use"
            | "있는"
            | "하는"
            | "합니다"
            | "입니다"
            | "있습니다"
            | "대한"
            | "통해"
            | "위한"
            | "에서"
            | "으로"
    )
}

fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| haystack.contains(needle))
}

#[derive(Debug, Clone, Serialize)]
struct ClassificationAuditEntry {
    timestamp_ms: u64,
    document_id: String,
    principal: String,
    action: String,
    source: ClassificationSource,
    manual_locked: bool,
    category: String,
    content_hash: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct ClassificationDisk {
    records: BTreeMap<String, DocumentClassification>,
    #[serde(default)]
    pending_index_sync: BTreeSet<String>,
}

/// Durable storage for document classification (`classifications.json`) and audit trail (`classification_audit.jsonl`).
pub(crate) struct ClassificationStore {
    path: PathBuf,
    audit_path: PathBuf,
    records: BTreeMap<String, DocumentClassification>,
    pending_index_sync: BTreeSet<String>,
}

impl ClassificationStore {
    pub(crate) fn open(storage_dir: &Path) -> Result<Self, String> {
        let path = storage_dir.join("classifications.json");
        let audit_path = storage_dir.join("classification_audit.jsonl");
        let disk = match fs::read(&path) {
            Ok(raw) => serde_json::from_slice::<ClassificationDisk>(&raw)
                .map_err(|err| format!("failed to decode {}: {err}", path.display()))?,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => ClassificationDisk::default(),
            Err(err) => return Err(format!("failed to read {}: {err}", path.display())),
        };
        Ok(Self {
            path,
            audit_path,
            records: disk.records,
            pending_index_sync: disk.pending_index_sync,
        })
    }

    pub(crate) fn is_manual_locked(&self, id: &DocumentId) -> bool {
        self.records
            .get(id.as_str())
            .is_some_and(|rec| rec.manual_locked)
    }

    pub(crate) fn get(&self, id: &DocumentId) -> Option<DocumentClassification> {
        self.records.get(id.as_str()).cloned()
    }

    pub(crate) fn pending_index_ids(&self) -> HashSet<String> {
        self.pending_index_sync.iter().cloned().collect()
    }

    pub(crate) fn is_pending_index_sync(&self, id: &DocumentId) -> bool {
        self.pending_index_sync.contains(id.as_str())
    }

    pub(crate) fn clear_pending_index_sync(&mut self, id: &DocumentId) -> Result<(), String> {
        if !self.pending_index_sync.contains(id.as_str()) {
            return Ok(());
        }
        let mut next_pending = self.pending_index_sync.clone();
        next_pending.remove(id.as_str());
        self.persist_state(&self.records, &next_pending)?;
        self.pending_index_sync = next_pending;
        Ok(())
    }

    pub(crate) fn clear_all_pending_index_sync(&mut self) -> Result<(), String> {
        if self.pending_index_sync.is_empty() {
            return Ok(());
        }
        let next_pending = BTreeSet::new();
        self.persist_state(&self.records, &next_pending)?;
        self.pending_index_sync = next_pending;
        Ok(())
    }

    /// Applies either existing manual classification (when `manual_locked`) or deterministic rule
    /// classification to `doc` before storing it.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn apply_during_sync(
        &mut self,
        doc: &mut Document,
        default_tag: &str,
        content_hash: &str,
    ) -> Result<(), String> {
        self.apply_during_sync_with_defaults(doc, "guide", default_tag, content_hash)
    }

    pub(crate) fn apply_during_sync_with_defaults(
        &mut self,
        doc: &mut Document,
        default_category: &str,
        default_tag: &str,
        content_hash: &str,
    ) -> Result<(), String> {
        if let Some(existing) = self.records.get(doc.id.as_str()).cloned()
            && existing.manual_locked
        {
            apply_classification_to_doc(doc, &existing);
            if existing.content_hash != content_hash {
                let mut updated = existing;
                updated.content_hash = content_hash.to_string();
                updated.updated_at_ms = now_ms();
                self.save_record(updated, "system", "sync_preserve_manual")?;
            }
            return Ok(());
        }

        let rule = RuleClassifier::classify_with_defaults(doc, default_category, default_tag);
        let unchanged = self.records.get(doc.id.as_str()).is_some_and(|existing| {
            !existing.manual_locked
                && existing.source == ClassificationSource::Rule
                && existing.category == rule.category
                && existing.summary == rule.summary
                && existing.tags == rule.tags
                && existing.keywords == rule.keywords
                && existing.content_hash == content_hash
        });

        let record = if unchanged {
            self.records.get(doc.id.as_str()).cloned().unwrap()
        } else {
            let next = DocumentClassification {
                document_id: doc.id.clone(),
                summary: rule.summary,
                category: rule.category,
                tags: rule.tags,
                keywords: rule.keywords,
                source: ClassificationSource::Rule,
                manual_locked: false,
                updated_at_ms: now_ms(),
                content_hash: content_hash.to_string(),
            };
            self.save_record(next.clone(), "system", "rule_classify")?;
            next
        };
        apply_classification_to_doc(doc, &record);
        Ok(())
    }

    pub(crate) fn prune_deleted(&mut self, live_ids: &HashSet<String>) -> Result<(), String> {
        let stale: Vec<String> = self
            .records
            .keys()
            .filter(|id| !live_ids.contains(id.as_str()))
            .cloned()
            .collect();
        let stale_pending: Vec<String> = self
            .pending_index_sync
            .iter()
            .filter(|id| !live_ids.contains(id.as_str()))
            .cloned()
            .collect();
        if stale.is_empty() && stale_pending.is_empty() {
            return Ok(());
        }
        let mut next_records = self.records.clone();
        for id in stale {
            next_records.remove(&id);
        }
        let mut next_pending = self.pending_index_sync.clone();
        for id in stale_pending {
            next_pending.remove(&id);
        }
        self.persist_state(&next_records, &next_pending)?;
        self.records = next_records;
        self.pending_index_sync = next_pending;
        Ok(())
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn update_manual(
        &mut self,
        doc: &mut Document,
        default_tag: &str,
        principal: &Principal,
        req: &UpdateClassificationRequest,
    ) -> Result<DocumentClassification, Dok3Error> {
        self.update_manual_with_defaults(doc, "guide", default_tag, principal, req)
    }

    pub(crate) fn update_manual_with_defaults(
        &mut self,
        doc: &mut Document,
        default_category: &str,
        default_tag: &str,
        principal: &Principal,
        req: &UpdateClassificationRequest,
    ) -> Result<DocumentClassification, Dok3Error> {
        let next = if req.reset_to_rule {
            let rule = RuleClassifier::classify_with_defaults(doc, default_category, default_tag);
            DocumentClassification {
                document_id: doc.id.clone(),
                summary: rule.summary,
                category: rule.category,
                tags: rule.tags,
                keywords: rule.keywords,
                source: ClassificationSource::Rule,
                manual_locked: false,
                updated_at_ms: now_ms(),
                content_hash: doc.content_hash.clone(),
            }
        } else {
            let summary = match &req.summary {
                Some(raw) => validate_classification_summary(raw)?,
                None => doc.summary.clone(),
            };
            let category = match &req.category {
                Some(raw) => validate_classification_category(raw)?,
                None => validate_classification_category(&doc.category)?,
            };
            let tags = match &req.tags {
                Some(raw) => normalize_classification_terms("tags", raw)?,
                None => doc.tags.clone(),
            };
            let keywords = match &req.keywords {
                Some(raw) => normalize_classification_terms("keywords", raw)?,
                None => doc.keywords.clone(),
            };
            DocumentClassification {
                document_id: doc.id.clone(),
                summary,
                category,
                tags,
                keywords,
                source: ClassificationSource::Manual,
                manual_locked: true,
                updated_at_ms: now_ms(),
                content_hash: doc.content_hash.clone(),
            }
        };

        let action = if req.reset_to_rule {
            "reset_to_rule"
        } else {
            "manual_update"
        };
        self.save_record(next.clone(), &principal.subject, action)
            .map_err(Dok3Error::Storage)?;
        apply_classification_to_doc(doc, &next);
        Ok(next)
    }

    fn save_record(
        &mut self,
        record: DocumentClassification,
        principal: &str,
        action: &str,
    ) -> Result<(), String> {
        let mut next_records = self.records.clone();
        let mut next_pending = self.pending_index_sync.clone();
        let key = record.document_id.as_str().to_string();
        next_records.insert(key.clone(), record.clone());
        next_pending.insert(key);
        self.persist_state(&next_records, &next_pending)?;
        self.records = next_records;
        self.pending_index_sync = next_pending;
        self.append_audit(&ClassificationAuditEntry {
            timestamp_ms: record.updated_at_ms,
            document_id: record.document_id.as_str().to_string(),
            principal: principal.to_string(),
            action: action.to_string(),
            source: record.source,
            manual_locked: record.manual_locked,
            category: record.category.clone(),
            content_hash: record.content_hash.clone(),
        })?;
        Ok(())
    }

    fn persist_state(
        &self,
        records: &BTreeMap<String, DocumentClassification>,
        pending_index_sync: &BTreeSet<String>,
    ) -> Result<(), String> {
        let disk = ClassificationDisk {
            records: records.clone(),
            pending_index_sync: pending_index_sync.clone(),
        };
        let bytes = serde_json::to_vec_pretty(&disk)
            .map_err(|err| format!("failed to serialize classifications: {err}"))?;
        atomic_write_bytes(&self.path, &bytes)
    }

    fn append_audit(&self, entry: &ClassificationAuditEntry) -> Result<(), String> {
        if let Some(parent) = self.audit_path.parent() {
            fs::create_dir_all(parent).map_err(|err| {
                format!("failed to create audit dir '{}': {err}", parent.display())
            })?;
        }
        let mut line = serde_json::to_vec(entry)
            .map_err(|err| format!("failed to serialize classification audit: {err}"))?;
        line.push(b'\n');
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.audit_path)
            .map_err(|err| format!("failed to open {}: {err}", self.audit_path.display()))?;
        file.write_all(&line)
            .map_err(|err| format!("failed to write {}: {err}", self.audit_path.display()))?;
        file.sync_all()
            .map_err(|err| format!("failed to sync {}: {err}", self.audit_path.display()))?;
        Ok(())
    }
}

pub(crate) fn apply_classification_to_doc(
    doc: &mut Document,
    classification: &DocumentClassification,
) {
    doc.summary = classification.summary.clone();
    doc.category = classification.category.clone();
    doc.tags = classification.tags.clone();
    doc.keywords = classification.keywords.clone();
    if let Some(primary) = doc
        .tags
        .iter()
        .find(|tag| !matches!(tag.as_str(), "ko" | "en"))
        .or_else(|| doc.tags.first())
    {
        doc.tag = primary.clone();
    } else {
        doc.tag = doc.category.clone();
    }
}

pub(crate) fn classification_from_doc(doc: &Document) -> DocumentClassification {
    DocumentClassification {
        document_id: doc.id.clone(),
        summary: doc.summary.clone(),
        category: doc.category.clone(),
        tags: doc.tags.clone(),
        keywords: doc.keywords.clone(),
        source: ClassificationSource::Rule,
        manual_locked: false,
        updated_at_ms: 0,
        content_hash: doc.content_hash.clone(),
    }
}

#[cfg(test)]
mod tests {
    use dok3_domain::{AclScope, DocumentSection, RevisionId, SectionId, SourceId};

    use super::*;

    fn sample_doc(id: &str, path: &str, title: &str, body: &str, lang: &str) -> Document {
        Document {
            id: DocumentId::new(id),
            revision_id: RevisionId::new("sha256:abc"),
            source_id: SourceId::new("local-markdown"),
            source: "Local Markdown".to_string(),
            title: title.to_string(),
            summary: String::new(),
            category: "guide".to_string(),
            topic: None,
            tag: "docs".to_string(),
            tags: vec![lang.to_string()],
            keywords: Vec::new(),
            language: lang.to_string(),
            version: "v1".to_string(),
            date: None,
            source_uri: format!("file:///{path}"),
            sample_path: Some(path.to_string()),
            acl_scope: AclScope::Public,
            content_hash: "abc".to_string(),
            body: body.to_string(),
            markdown: Some(body.to_string()),
            sections: vec![DocumentSection {
                section_id: SectionId::new("sec-1"),
                heading: title.to_string(),
                anchor: "sec-1".to_string(),
                order: 1,
                line_start: 1,
                line_end: 5,
                body: body.to_string(),
            }],
        }
    }

    #[test]
    fn rule_classifier_assigns_categories_tags_and_keywords_deterministically() {
        let manual = sample_doc(
            "m1",
            "manual/backup-runbook.md",
            "인덱스 백업 및 복구 매뉴얼",
            "# 인덱스 백업 및 복구 매뉴얼\n\n`DOK3_DATA_DIR` 세대 디렉터리를 확인하고 스냅샷을 백업합니다.\n",
            "ko",
        );
        let out_manual = RuleClassifier::classify(&manual, "docs");
        assert_eq!(out_manual.category, "manual");
        assert!(out_manual.summary.contains("DOK3_DATA_DIR"));
        assert!(out_manual.tags.contains(&"ko".to_string()));
        assert!(out_manual.keywords.iter().any(|k| k == "DOK3_DATA_DIR"));
        assert!(out_manual.tags.len() <= MAX_CLASSIFICATION_TERMS);
        assert!(out_manual.keywords.len() <= MAX_CLASSIFICATION_TERMS);

        let code = sample_doc(
            "c1",
            "notes/oracle-errors.md",
            "ORA-00904 조회 오류 대응",
            "# ORA-00904 조회 오류 대응\n\n```sql\nSELECT account_id FROM accounts;\n```\n\n`getAccountList` 호출 시 바인딩 컬럼명을 점검합니다.\n",
            "ko",
        );
        let out_code = RuleClassifier::classify(&code, "docs");
        assert_eq!(out_code.category, "code");
        assert!(out_code.keywords.iter().any(|k| k == "ORA-00904"));
        assert!(out_code.keywords.iter().any(|k| k == "getAccountList"));

        let policy = sample_doc(
            "p1",
            "policy/retention.md",
            "자료 반입 보안 규정",
            "# 자료 반입 보안 규정\n\n외부 링크 클릭을 제한하고 해시를 검증합니다.\n",
            "ko",
        );
        assert_eq!(RuleClassifier::classify(&policy, "docs").category, "policy");
    }

    #[test]
    fn manual_classification_preserves_original_and_survives_sync_until_reset() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = ClassificationStore::open(dir.path()).unwrap();
        let mut doc = sample_doc(
            "d1",
            "guides/intro.md",
            "Rust 입문 안내",
            "# Rust 입문 안내\n\n소유권과 대여 규칙을 설명합니다.\n",
            "ko",
        );
        let original_body = doc.body.clone();
        let original_markdown = doc.markdown.clone();
        let original_uri = doc.source_uri.clone();
        let original_hash = doc.content_hash.clone();

        store.apply_during_sync(&mut doc, "docs", "abc").unwrap();
        assert_eq!(doc.category, "guide");
        assert!(!store.get(&doc.id).unwrap().manual_locked);

        let admin = Principal::local_admin();
        let updated = store
            .update_manual(
                &mut doc,
                "docs",
                &admin,
                &UpdateClassificationRequest {
                    summary: Some("수동 지정 요약".to_string()),
                    category: Some("manual".to_string()),
                    tags: Some(vec!["rust".to_string(), "수동".to_string()]),
                    keywords: Some(vec!["ownership".to_string()]),
                    reset_to_rule: false,
                },
            )
            .unwrap();
        assert_eq!(updated.source, ClassificationSource::Manual);
        assert!(updated.manual_locked);
        assert_eq!(doc.body, original_body);
        assert_eq!(doc.markdown, original_markdown);
        assert_eq!(doc.source_uri, original_uri);
        assert_eq!(doc.content_hash, original_hash);

        // Subsequent sync keeps manual overrides.
        let mut resynced = sample_doc(
            "d1",
            "guides/intro.md",
            "Rust 입문 안내",
            "# Rust 입문 안내\n\n소유권과 대여 규칙을 설명합니다.\n",
            "ko",
        );
        store
            .apply_during_sync(&mut resynced, "docs", "abc")
            .unwrap();
        assert_eq!(resynced.category, "manual");
        assert_eq!(resynced.summary, "수동 지정 요약");
        assert_eq!(resynced.tags, vec!["rust", "수동"]);

        // Reset restores deterministic rule classification.
        let reset = store
            .update_manual(
                &mut resynced,
                "docs",
                &admin,
                &UpdateClassificationRequest {
                    reset_to_rule: true,
                    ..UpdateClassificationRequest::default()
                },
            )
            .unwrap();
        assert_eq!(reset.source, ClassificationSource::Rule);
        assert!(!reset.manual_locked);
        assert_eq!(resynced.category, "guide");
        assert!(dir.path().join("classification_audit.jsonl").is_file());
    }
}
