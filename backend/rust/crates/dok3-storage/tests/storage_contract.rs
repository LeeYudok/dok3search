use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use dok3_core::Dok3Error;
use dok3_domain::{
    AclScope, ChangeEvent, ChangeJournalPort, ChangeKind, Document, DocumentId, DocumentSection,
    DocumentStorePort, Principal, RevisionId, SectionId, SectionReadRequest, Sequence, SourceId,
};
use dok3_storage::{FileChangeJournal, FileDocumentStore, PutOutcome, content_revision};

struct TempRoot(PathBuf);

impl TempRoot {
    fn new(name: &str) -> Self {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!(
            "dok3-storage-{name}-{}-{nanos}",
            std::process::id()
        ));
        fs::create_dir_all(&dir).unwrap();
        Self(dir)
    }
}

impl Drop for TempRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn doc(id: &str, scope: AclScope, body: &str) -> Document {
    Document {
        id: DocumentId::new(id),
        revision_id: RevisionId::new("caller-supplied"),
        source_id: SourceId::new("files"),
        source: "files".into(),
        title: format!("제목 {id}"),
        summary: String::new(),
        category: "guide".into(),
        tag: "Rust".into(),
        tags: vec!["Rust".into()],
        keywords: vec![],
        language: "ko".into(),
        version: "v1".into(),
        date: None,
        source_uri: format!("file://{id}.md"),
        sample_path: None,
        acl_scope: scope,
        content_hash: "caller-supplied".into(),
        body: body.into(),
        markdown: Some(body.into()),
        sections: vec![DocumentSection {
            section_id: SectionId::new("body"),
            heading: "본문".into(),
            anchor: "body".into(),
            order: 1,
            line_start: 1,
            line_end: 1,
            body: body.into(),
        }],
    }
}

fn public() -> Principal {
    Principal::anonymous_public()
}

#[test]
fn put_derives_sha256_revision_and_is_idempotent_and_persistent() {
    let root = TempRoot::new("put");
    let store = FileDocumentStore::open(&root.0).unwrap();
    let content = "# 소유권\n본문".as_bytes();

    let first = store
        .put(doc("a", AclScope::Public, "본문"), content)
        .unwrap();
    assert!(matches!(first, PutOutcome::Stored(_)));
    assert_eq!(first.revision(), &content_revision(content));
    assert!(first.revision().as_str().starts_with("sha256:"));

    let again = store
        .put(doc("a", AclScope::Public, "본문"), content)
        .unwrap();
    assert_eq!(again, PutOutcome::Unchanged(content_revision(content)));

    let changed = store
        .put(doc("a", AclScope::Public, "새 본문"), b"changed")
        .unwrap();
    assert_eq!(changed, PutOutcome::Stored(content_revision(b"changed")));

    drop(store);
    let reopened = FileDocumentStore::open(&root.0).unwrap();
    assert_eq!(reopened.document_count(), 1);
    let detail = reopened
        .get_document(&public(), &DocumentId::new("a"))
        .unwrap();
    assert_eq!(detail.revision_id, content_revision(b"changed"));
    assert_eq!(
        reopened
            .read_original(&public(), &DocumentId::new("a"))
            .unwrap(),
        b"changed"
    );
}

#[test]
fn acl_blocks_internal_docs_for_public_principal() {
    let root = TempRoot::new("acl");
    let store = FileDocumentStore::open(&root.0).unwrap();
    store
        .put(doc("secret", AclScope::internal("ops"), "기밀"), b"x")
        .unwrap();
    let id = DocumentId::new("secret");

    let err = store.get_document(&public(), &id).unwrap_err();
    assert!(matches!(err, Dok3Error::Forbidden(_)));
    assert!(matches!(
        store.read_original(&public(), &id).unwrap_err(),
        Dok3Error::Forbidden(_)
    ));
    let req = SectionReadRequest {
        document_id: id,
        section_id: SectionId::new("body"),
        cursor: None,
        max_chars: 10,
    };
    assert!(matches!(
        store.get_section(&public(), &req).unwrap_err(),
        Dok3Error::Forbidden(_)
    ));
}

#[test]
fn section_paging_delete_and_integrity_check() {
    let root = TempRoot::new("section");
    let store = FileDocumentStore::open(&root.0).unwrap();
    store
        .put(doc("k", AclScope::Public, "가나다라마바사"), b"orig")
        .unwrap();
    let id = DocumentId::new("k");

    let mut req = SectionReadRequest {
        document_id: id.clone(),
        section_id: SectionId::new("body"),
        cursor: None,
        max_chars: 3,
    };
    let page = store.get_section(&public(), &req).unwrap();
    assert_eq!(page.text, "가나다");
    assert_eq!(page.next_cursor, Some(3));
    req.cursor = Some(6);
    let last = store.get_section(&public(), &req).unwrap();
    assert_eq!((last.text.as_str(), last.truncated), ("사", false));
    req.cursor = Some(99);
    assert!(matches!(
        store.get_section(&public(), &req).unwrap_err(),
        Dok3Error::InvalidInput(_)
    ));

    // Tamper with the original object: integrity check must fail.
    let hash = content_revision(b"orig")
        .as_str()
        .trim_start_matches("sha256:")
        .to_string();
    fs::write(root.0.join("objects").join(hash), b"tampered").unwrap();
    assert!(matches!(
        store.read_original(&public(), &id).unwrap_err(),
        Dok3Error::Storage(_)
    ));

    assert!(store.delete(&id).unwrap());
    assert!(!store.delete(&id).unwrap());
    assert!(matches!(
        store.get_document(&public(), &id).unwrap_err(),
        Dok3Error::NotFound(_)
    ));
}

fn event(id: &str, kind: ChangeKind) -> ChangeEvent {
    ChangeEvent {
        sequence: Sequence::new(999),
        document_id: DocumentId::new(id),
        revision_id: RevisionId::new("sha256:x"),
        kind,
    }
}

#[test]
fn journal_assigns_monotonic_sequences_and_replays_after_reopen() {
    let root = TempRoot::new("journal");
    let journal = FileChangeJournal::open(&root.0).unwrap();
    assert_eq!(
        journal.append(event("a", ChangeKind::Upsert)).unwrap(),
        Sequence::new(1)
    );
    assert_eq!(
        journal.append(event("b", ChangeKind::Upsert)).unwrap(),
        Sequence::new(2)
    );
    assert_eq!(
        journal.append(event("a", ChangeKind::Delete)).unwrap(),
        Sequence::new(3)
    );

    let after1 = journal.read_after(Sequence::new(1), 10).unwrap();
    assert_eq!(
        after1.iter().map(|e| e.sequence.get()).collect::<Vec<_>>(),
        vec![2, 3]
    );
    assert_eq!(journal.read_after(Sequence::new(0), 1).unwrap().len(), 1);
    drop(journal);

    let reopened = FileChangeJournal::open(&root.0).unwrap();
    assert_eq!(
        reopened.append(event("c", ChangeKind::Upsert)).unwrap(),
        Sequence::new(4)
    );
    let all = reopened.read_after(Sequence::new(0), 100).unwrap();
    assert_eq!(all.len(), 4);
    assert_eq!(all[2].kind, ChangeKind::Delete);
}

#[test]
fn journal_recovers_from_torn_trailing_write() {
    let root = TempRoot::new("torn");
    let journal = FileChangeJournal::open(&root.0).unwrap();
    journal.append(event("a", ChangeKind::Upsert)).unwrap();
    drop(journal);

    let mut f = OpenOptions::new()
        .append(true)
        .open(root.0.join("journal.jsonl"))
        .unwrap();
    f.write_all(br#"{"sequence":2,"document_id":"b""#).unwrap();
    drop(f);

    let reopened = FileChangeJournal::open(&root.0).unwrap();
    assert_eq!(reopened.read_after(Sequence::new(0), 10).unwrap().len(), 1);
    assert_eq!(
        reopened.append(event("b", ChangeKind::Upsert)).unwrap(),
        Sequence::new(2)
    );
    assert_eq!(reopened.read_after(Sequence::new(0), 10).unwrap().len(), 2);
}
