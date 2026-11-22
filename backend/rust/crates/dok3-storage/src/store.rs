//! Content-addressed original store plus document metadata implementing `DocumentStorePort`.

use std::collections::BTreeMap;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::sync::RwLock;

use dok3_core::Dok3Error;
use dok3_domain::{
    Document, DocumentDetail, DocumentId, DocumentStorePort, Principal, RevisionId,
    SectionCitation, SectionReadRequest, SectionReadResponse, SectionSummary,
};
use sha2::{Digest, Sha256};

use crate::fsutil::{acquire_exclusive_lock, atomic_write, cleanup_stale_temps, storage_err};

/// Returns the canonical revision id (`sha256:<hex>`) for original bytes.
#[must_use]
pub fn content_revision(content: &[u8]) -> RevisionId {
    RevisionId::new(format!("sha256:{}", sha256_hex(content)))
}

fn sha256_hex(content: &[u8]) -> String {
    let digest = Sha256::digest(content);
    let mut out = String::with_capacity(64);
    for b in digest {
        out.push_str(&format!("{b:02x}"));
    }
    out
}

/// Result of [`FileDocumentStore::put`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PutOutcome {
    /// New document, changed metadata/sections, or healed object; carries the revision.
    Stored(RevisionId),
    /// Stored metadata and object file match identically; nothing was written.
    Unchanged(RevisionId),
}

impl PutOutcome {
    #[must_use]
    pub fn revision(&self) -> &RevisionId {
        match self {
            Self::Stored(r) | Self::Unchanged(r) => r,
        }
    }
}

/// File-backed store: originals in `objects/`, metadata in `documents.json`.
#[derive(Debug)]
pub struct FileDocumentStore {
    _lock: File,
    root: PathBuf,
    docs: RwLock<BTreeMap<String, Document>>,
}

impl FileDocumentStore {
    /// Opens (or creates) a store rooted at `root` under an exclusive `.store.lock`.
    pub fn open(root: &Path) -> Result<Self, Dok3Error> {
        let objects_dir = root.join("objects");
        fs::create_dir_all(&objects_dir).map_err(|e| storage_err("create store", e))?;
        let lock = acquire_exclusive_lock(&root.join(".store.lock"))?;
        cleanup_stale_temps(root);
        cleanup_stale_temps(&objects_dir);

        let meta = root.join("documents.json");
        let docs = match fs::read(&meta) {
            Ok(raw) => {
                serde_json::from_slice(&raw).map_err(|e| storage_err("decode metadata", e))?
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => BTreeMap::new(),
            Err(e) => return Err(storage_err("read metadata", e)),
        };
        Ok(Self {
            _lock: lock,
            root: root.to_path_buf(),
            docs: RwLock::new(docs),
        })
    }

    /// Stores original bytes and metadata. `doc.revision_id` and `doc.content_hash`
    /// are derived from `content` (SHA-256) and override caller-provided values.
    ///
    /// If the content hash matches an intact object on disk, object writing is skipped,
    /// while metadata or section edits still persist. Corrupted existing objects are healed.
    pub fn put(&self, mut doc: Document, content: &[u8]) -> Result<PutOutcome, Dok3Error> {
        if doc.id.as_str().trim().is_empty() {
            return Err(Dok3Error::InvalidInput(
                "document id must not be empty".into(),
            ));
        }
        let hash = sha256_hex(content);
        let revision = RevisionId::new(format!("sha256:{hash}"));
        doc.revision_id = revision.clone();
        doc.content_hash = hash.clone();

        let mut docs = self.write()?;
        let object = self.object_path(&hash);
        let intact = object_intact(&object, &hash);
        let metadata_changed = docs.get(doc.id.as_str()) != Some(&doc);
        if !metadata_changed && intact {
            return Ok(PutOutcome::Unchanged(revision));
        }
        if !intact {
            atomic_write(&object, content)?;
        }
        if metadata_changed {
            let mut next = docs.clone();
            next.insert(doc.id.as_str().to_string(), doc);
            if let Err(err) = self.persist(&next) {
                self.resync_from_disk(&mut docs);
                return Err(err);
            }
            *docs = next;
        }
        Ok(PutOutcome::Stored(revision))
    }

    /// Removes document metadata. Returns `false` when the id was unknown.
    /// Original objects are kept (content-addressed, may be shared or audited).
    pub fn delete(&self, id: &DocumentId) -> Result<bool, Dok3Error> {
        let mut docs = self.write()?;
        if !docs.contains_key(id.as_str()) {
            return Ok(false);
        }
        let mut next = docs.clone();
        next.remove(id.as_str());
        if let Err(err) = self.persist(&next) {
            self.resync_from_disk(&mut docs);
            return Err(err);
        }
        *docs = next;
        Ok(true)
    }

    /// Reads original bytes for an authorized document, verifying the stored hash.
    pub fn read_original(
        &self,
        principal: &Principal,
        id: &DocumentId,
    ) -> Result<Vec<u8>, Dok3Error> {
        let doc = self.authorized(principal, id)?;
        let bytes = fs::read(self.object_path(&doc.content_hash))
            .map_err(|e| storage_err("read original", e))?;
        if sha256_hex(&bytes) != doc.content_hash {
            return Err(Dok3Error::Storage(format!(
                "original for '{id}' failed integrity check"
            )));
        }
        Ok(bytes)
    }

    /// Returns an authorized copy of the stored domain [`Document`].
    pub fn get_stored_document(
        &self,
        principal: &Principal,
        id: &DocumentId,
    ) -> Result<Document, Dok3Error> {
        self.authorized(principal, id)
    }

    /// Returns sorted document IDs currently tracked in metadata.
    pub fn document_ids(&self) -> Result<Vec<DocumentId>, Dok3Error> {
        let docs = self
            .docs
            .read()
            .map_err(|_| Dok3Error::Internal("store lock poisoned".into()))?;
        Ok(docs.keys().map(|id| DocumentId::new(id.clone())).collect())
    }

    /// Returns clones of all documents currently stored in metadata.
    pub fn list_documents(&self) -> Result<Vec<Document>, Dok3Error> {
        let docs = self
            .docs
            .read()
            .map_err(|_| Dok3Error::Internal("store lock poisoned".into()))?;
        Ok(docs.values().cloned().collect())
    }

    fn object_path(&self, hash: &str) -> PathBuf {
        self.root.join("objects").join(hash)
    }

    fn persist(&self, docs: &BTreeMap<String, Document>) -> Result<(), Dok3Error> {
        let raw = serde_json::to_vec_pretty(docs).map_err(|e| storage_err("encode metadata", e))?;
        atomic_write(&self.root.join("documents.json"), &raw)
    }

    fn resync_from_disk(&self, docs: &mut BTreeMap<String, Document>) {
        match fs::read(self.root.join("documents.json")) {
            Ok(raw) => {
                if let Ok(on_disk) = serde_json::from_slice::<BTreeMap<String, Document>>(&raw) {
                    *docs = on_disk;
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                docs.clear();
            }
            Err(_) => {}
        }
    }

    fn write(
        &self,
    ) -> Result<std::sync::RwLockWriteGuard<'_, BTreeMap<String, Document>>, Dok3Error> {
        self.docs
            .write()
            .map_err(|_| Dok3Error::Internal("store lock poisoned".into()))
    }

    fn authorized(&self, principal: &Principal, id: &DocumentId) -> Result<Document, Dok3Error> {
        let docs = self
            .docs
            .read()
            .map_err(|_| Dok3Error::Internal("store lock poisoned".into()))?;
        let doc = docs
            .get(id.as_str())
            .ok_or_else(|| Dok3Error::NotFound(format!("document '{id}' not found")))?;
        if !principal.allows(&doc.acl_scope) {
            return Err(Dok3Error::Forbidden(format!(
                "principal '{}' is not permitted to access scope '{}'",
                principal.subject, doc.acl_scope
            )));
        }
        Ok(doc.clone())
    }
}

fn object_intact(path: &Path, expected_hash: &str) -> bool {
    let Ok(bytes) = fs::read(path) else {
        return false;
    };
    sha256_hex(&bytes) == expected_hash
}

impl DocumentStorePort for FileDocumentStore {
    fn get_document(
        &self,
        principal: &Principal,
        id: &DocumentId,
    ) -> Result<DocumentDetail, Dok3Error> {
        let doc = self.authorized(principal, id)?;
        Ok(DocumentDetail {
            sections: doc.sections.iter().map(SectionSummary::from).collect(),
            id: doc.id,
            revision_id: doc.revision_id,
            source_id: doc.source_id,
            source: doc.source,
            title: doc.title,
            summary: doc.summary,
            category: doc.category,
            topic: doc.topic,
            tag: doc.tag,
            tags: doc.tags,
            keywords: doc.keywords,
            language: doc.language,
            version: doc.version,
            date: doc.date,
            source_uri: doc.source_uri,
            sample_path: doc.sample_path,
            acl_scope: doc.acl_scope,
            body: doc.body,
            markdown: doc.markdown,
            formats: None,
        })
    }

    fn get_section(
        &self,
        principal: &Principal,
        req: &SectionReadRequest,
    ) -> Result<SectionReadResponse, Dok3Error> {
        let req = req.clone().normalized();
        let doc = self.authorized(principal, &req.document_id)?;
        let section = doc
            .sections
            .iter()
            .find(|s| s.section_id == req.section_id)
            .ok_or_else(|| {
                Dok3Error::NotFound(format!(
                    "section '{}' not found in document '{}'",
                    req.section_id, req.document_id
                ))
            })?;
        let cursor = req.cursor.unwrap_or(0);
        let total = section.body.chars().count();
        if cursor > total {
            return Err(Dok3Error::InvalidInput(format!(
                "section cursor {cursor} exceeds section length {total}"
            )));
        }
        let end = cursor.saturating_add(req.max_chars).min(total);
        let text: String = section
            .body
            .chars()
            .skip(cursor)
            .take(end - cursor)
            .collect();
        let truncated = end < total;
        Ok(SectionReadResponse {
            source_revision: doc.revision_id.clone(),
            heading_path: vec![doc.title.clone(), section.heading.clone()],
            citation: SectionCitation {
                source_uri: doc.source_uri.clone(),
                section: section.section_id.clone(),
            },
            text,
            next_cursor: truncated.then_some(end),
            truncated,
        })
    }

    fn document_count(&self) -> usize {
        self.docs.read().map_or(0, |d| d.len())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    use dok3_domain::{AclScope, Document, DocumentId, RevisionId, SourceId};

    use super::FileDocumentStore;

    static TEMP_SEQ: AtomicU64 = AtomicU64::new(1);

    struct TempDir(PathBuf);

    impl TempDir {
        fn new(label: &str) -> Self {
            let dir = std::env::temp_dir().join(format!(
                "dok3_store_test_{label}_{}_{}",
                std::process::id(),
                TEMP_SEQ.fetch_add(1, Ordering::Relaxed)
            ));
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir_all(&dir).unwrap();
            Self(dir)
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn sample_doc(id: &str, body: &str) -> Document {
        Document {
            id: DocumentId::new(id),
            revision_id: RevisionId::new("placeholder"),
            source_id: SourceId::new("src"),
            source: "test".into(),
            title: id.into(),
            summary: String::new(),
            category: "guide".into(),
            topic: None,
            tag: "test".into(),
            tags: vec![],
            keywords: vec![],
            language: "en".into(),
            version: "1.0".into(),
            date: None,
            source_uri: format!("file:///{id}.md"),
            sample_path: None,
            acl_scope: AclScope::Public,
            content_hash: String::new(),
            body: body.into(),
            markdown: Some(body.into()),
            sections: vec![],
        }
    }

    #[test]
    fn document_ids_and_list_documents_reflect_puts_and_deletes() {
        let dir = TempDir::new("list");
        let store = FileDocumentStore::open(&dir.0).unwrap();
        assert!(store.document_ids().unwrap().is_empty());
        assert!(store.list_documents().unwrap().is_empty());

        store.put(sample_doc("b-doc", "second"), b"second").unwrap();
        store.put(sample_doc("a-doc", "first"), b"first").unwrap();
        assert_eq!(
            store.document_ids().unwrap(),
            vec![DocumentId::new("a-doc"), DocumentId::new("b-doc")]
        );
        assert_eq!(store.list_documents().unwrap().len(), 2);

        assert!(store.delete(&DocumentId::new("a-doc")).unwrap());
        assert_eq!(
            store.document_ids().unwrap(),
            vec![DocumentId::new("b-doc")]
        );
    }
}
