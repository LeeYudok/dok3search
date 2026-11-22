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
        topic: None,
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

#[test]
fn exclusive_lock_blocks_second_store_writer_and_allows_coexisting_journal() {
    let root = TempRoot::new("store-lock");
    let store = FileDocumentStore::open(&root.0).unwrap();

    let second = FileDocumentStore::open(&root.0).unwrap_err();
    assert!(matches!(second, Dok3Error::Storage(msg) if msg.contains("writer lock")));

    // Journal uses its own lock file and can coexist on the same root.
    let journal = FileChangeJournal::open(&root.0).unwrap();
    drop(journal);
    drop(store);

    // Reopening succeeds once the previous store guard is dropped.
    let reopened = FileDocumentStore::open(&root.0).unwrap();
    assert_eq!(reopened.document_count(), 0);
}

#[test]
fn exclusive_lock_blocks_second_journal_writer() {
    let root = TempRoot::new("jrn-lock");
    let journal = FileChangeJournal::open(&root.0).unwrap();

    let second = FileChangeJournal::open(&root.0).unwrap_err();
    assert!(matches!(second, Dok3Error::Storage(msg) if msg.contains("writer lock")));

    drop(journal);
    let reopened = FileChangeJournal::open(&root.0).unwrap();
    assert_eq!(reopened.read_after(Sequence::new(0), 10).unwrap().len(), 0);
}

#[test]
fn put_persists_metadata_and_section_updates_with_unchanged_content() {
    let root = TempRoot::new("meta-update");
    let store = FileDocumentStore::open(&root.0).unwrap();
    let content = b"unchanged original markdown";

    let first = store
        .put(doc("m1", AclScope::Public, "v1 body"), content)
        .unwrap();
    assert!(matches!(first, PutOutcome::Stored(_)));

    let mut updated = doc("m1", AclScope::Public, "v1 body");
    updated.title = "수정된 제목".into();
    updated.summary = "분류 요약 갱신".into();
    updated.tags = vec!["Rust".into(), "Storage".into()];
    updated.sections[0].heading = "수정된 절 제목".into();

    let second = store.put(updated, content).unwrap();
    assert_eq!(second, PutOutcome::Stored(content_revision(content)));
    drop(store);

    let reopened = FileDocumentStore::open(&root.0).unwrap();
    let detail = reopened
        .get_document(&public(), &DocumentId::new("m1"))
        .unwrap();
    assert_eq!(detail.title, "수정된 제목");
    assert_eq!(detail.summary, "분류 요약 갱신");
    assert_eq!(detail.tags, vec!["Rust", "Storage"]);
    assert_eq!(detail.sections[0].heading, "수정된 절 제목");
}

#[test]
fn put_heals_corrupted_object_file_on_same_hash() {
    let root = TempRoot::new("heal-obj");
    let store = FileDocumentStore::open(&root.0).unwrap();
    let content = b"canonical original payload";
    let d = doc("h1", AclScope::Public, "body");

    assert!(matches!(
        store.put(d.clone(), content).unwrap(),
        PutOutcome::Stored(_)
    ));
    let hash = content_revision(content)
        .as_str()
        .trim_start_matches("sha256:")
        .to_string();
    fs::write(root.0.join("objects").join(&hash), b"corrupted bytes").unwrap();
    assert!(matches!(
        store.read_original(&public(), &DocumentId::new("h1")),
        Err(Dok3Error::Storage(_))
    ));

    // Re-putting identical Document + content must detect corruption and rewrite the object.
    assert_eq!(
        store.put(d, content).unwrap(),
        PutOutcome::Stored(content_revision(content))
    );
    assert_eq!(
        store
            .read_original(&public(), &DocumentId::new("h1"))
            .unwrap(),
        content
    );
}

#[test]
fn journal_recovers_corrupt_newline_terminated_trailing_line_and_rejects_midfile_corruption() {
    let root = TempRoot::new("corrupt-nl");
    let journal = FileChangeJournal::open(&root.0).unwrap();
    journal.append(event("a", ChangeKind::Upsert)).unwrap();
    drop(journal);

    // Append corrupt NUL + newline at EOF -> open recovers by truncating it.
    let jpath = root.0.join("journal.jsonl");
    let mut f = OpenOptions::new().append(true).open(&jpath).unwrap();
    f.write_all(b"\0\0corrupt-line\n").unwrap();
    drop(f);

    let reopened = FileChangeJournal::open(&root.0).unwrap();
    assert_eq!(reopened.read_after(Sequence::new(0), 10).unwrap().len(), 1);
    assert_eq!(
        reopened.append(event("b", ChangeKind::Upsert)).unwrap(),
        Sequence::new(2)
    );
    drop(reopened);

    // Mid-file corruption (corrupt line followed by valid line) must fail loudly.
    let mut original = fs::read(&jpath).unwrap();
    let valid_line = b"{\"sequence\":3,\"document_id\":\"c\",\"revision_id\":\"sha256:x\",\"kind\":\"upsert\"}\n";
    let mut mid_corrupt = Vec::new();
    let first_nl = original.iter().position(|&b| b == b'\n').unwrap() + 1;
    mid_corrupt.extend_from_slice(&original[..first_nl]);
    mid_corrupt.extend_from_slice(b"{not-json}\n");
    mid_corrupt.extend_from_slice(valid_line);
    original.clear();
    let _ = original;
    fs::write(&jpath, &mid_corrupt).unwrap();

    assert!(matches!(
        FileChangeJournal::open(&root.0),
        Err(Dok3Error::Storage(_))
    ));
}

#[test]
fn open_cleans_up_stale_temp_files_without_removing_locks() {
    let root = TempRoot::new("clean-tmp");
    let objects = root.0.join("objects");
    fs::create_dir_all(&objects).unwrap();

    let stale_root = root.0.join(".documents.json.tmp.99999.0");
    let stale_obj = objects.join(".deadbeef.tmp.99999.1");
    fs::write(&stale_root, b"partial").unwrap();
    fs::write(&stale_obj, b"partial").unwrap();

    let _store = FileDocumentStore::open(&root.0).unwrap();
    assert!(!stale_root.exists());
    assert!(!stale_obj.exists());
    assert!(root.0.join(".store.lock").exists());
}

#[test]
fn journal_open_does_not_delete_store_temp_files() {
    let root = TempRoot::new("jrn-keeps-tmp");
    let _store = FileDocumentStore::open(&root.0).unwrap();

    let active_store_tmp = root.0.join(".documents.json.tmp.99999.1");
    fs::write(&active_store_tmp, b"staging").unwrap();

    let _journal = FileChangeJournal::open(&root.0).unwrap();
    assert!(active_store_tmp.exists());
}

#[test]
fn update_metadata_preserves_original_and_rejects_corrupted_object_without_overwriting() {
    let root = TempRoot::new("meta-only");
    let store = FileDocumentStore::open(&root.0).unwrap();
    let content = b"immutable original bytes";
    let original_doc = doc("m1", AclScope::Public, "original body");

    assert!(matches!(
        store.put(original_doc, content).unwrap(),
        PutOutcome::Stored(_)
    ));
    let hash = content_revision(content)
        .as_str()
        .trim_start_matches("sha256:")
        .to_string();
    let obj_path = root.0.join("objects").join(&hash);

    // Metadata-only update succeeds and leaves original bytes intact.
    let mut updated_doc = doc("m1", AclScope::Public, "original body");
    updated_doc.summary = "updated summary".to_string();
    updated_doc.category = "policy".to_string();
    assert_eq!(
        store.update_metadata(updated_doc.clone()).unwrap(),
        PutOutcome::Stored(content_revision(content))
    );
    assert_eq!(
        store
            .read_original(&public(), &DocumentId::new("m1"))
            .unwrap(),
        content
    );

    // Repeating identical metadata returns Unchanged.
    assert_eq!(
        store.update_metadata(updated_doc.clone()).unwrap(),
        PutOutcome::Unchanged(content_revision(content))
    );

    // Corrupting the stored object must fail update_metadata without overwriting the file on disk.
    fs::write(&obj_path, b"tampered").unwrap();
    updated_doc.summary = "second update".to_string();
    assert!(matches!(
        store.update_metadata(updated_doc),
        Err(dok3_core::Dok3Error::Storage(_))
    ));
    assert_eq!(fs::read(&obj_path).unwrap(), b"tampered");
}
