use std::str::FromStr;

use dok3_domain::{
    AclScope, CategoryCount, ChangeEvent, ChangeKind, DEFAULT_SECTION_MAX_CHARS, Document,
    DocumentDetail, DocumentFormatLinks, DocumentId, DocumentSection, GenerationId,
    IndexJobListResponse, IndexJobMode, IndexJobRecord, IndexJobState, MAX_SECTION_MAX_CHARS,
    Principal, ReaderState, RevisionId, SearchFilters, SearchHit, SearchRequest, SearchResponse,
    SearchSnapshotMeta, SectionCitation, SectionId, SectionReadRequest, SectionReadResponse,
    SectionSummary, Sequence, SourceId, TriggerIndexJobRequest,
};

#[test]
fn acl_scope_round_trip_and_rejection() {
    assert_eq!(
        serde_json::to_string(&AclScope::Public).unwrap(),
        "\"public\""
    );
    assert_eq!(
        serde_json::to_string(&AclScope::internal("security")).unwrap(),
        "\"internal:security\""
    );

    assert_eq!(
        serde_json::from_str::<AclScope>("\"public\"").unwrap(),
        AclScope::Public
    );
    assert_eq!(
        serde_json::from_str::<AclScope>("\"internal:jbb\"").unwrap(),
        AclScope::internal("jbb")
    );

    for invalid in ["\"\"", "\"internal:\"", "\"internal:a b\"", "\"private\""] {
        assert!(
            serde_json::from_str::<AclScope>(invalid).is_err(),
            "expected error for {invalid}"
        );
    }
    assert!(AclScope::from_str("internal:").is_err());
    assert!(AclScope::try_internal("   ").is_err());
}

#[test]
fn principal_default_deny_and_never_escalates_requested_scope() {
    let public = Principal::anonymous_public();
    let internal_sec = AclScope::internal("security");
    let internal_hr = AclScope::internal("hr");

    assert!(public.allows(&AclScope::Public));
    assert!(!public.allows(&internal_sec));
    assert_eq!(
        public.effective_scopes(Some(&internal_sec)),
        Vec::<AclScope>::new()
    );
    assert_eq!(public.effective_scopes(None), vec![AclScope::Public]);

    let employee = Principal {
        subject: "alice".to_string(),
        allowed_scopes: vec![AclScope::Public, internal_sec.clone()],
        is_admin: false,
    };
    assert_eq!(
        employee.effective_scopes(Some(&internal_sec)),
        vec![internal_sec.clone()]
    );
    assert_eq!(
        employee.effective_scopes(Some(&internal_hr)),
        Vec::<AclScope>::new()
    );
    assert_eq!(
        employee.effective_scopes(None),
        vec![AclScope::Public, internal_sec]
    );
}

#[test]
fn search_request_defaults_and_normalization() {
    let minimal: SearchRequest = serde_json::from_str("{}").unwrap();
    assert_eq!(minimal.query, "");
    assert_eq!(minimal.page, 1);
    assert_eq!(minimal.limit, 15);
    assert!(minimal.include_snippets);
    assert_eq!(minimal.requested_scope, None);

    let clamped = SearchRequest {
        query: "rust".to_string(),
        filters: SearchFilters::default(),
        page: 0,
        limit: 0,
        include_snippets: false,
        requested_scope: Some(AclScope::Public),
    }
    .normalized();
    assert_eq!(clamped.page, 1);
    assert_eq!(clamped.limit, 15);

    let capped = SearchRequest {
        limit: 999,
        ..SearchRequest::default()
    }
    .normalized();
    assert_eq!(capped.limit, 100);
}

#[test]
fn section_read_request_defaults_and_normalization() {
    let minimal: SectionReadRequest =
        serde_json::from_str(r#"{"document_id":"doc-1","section_id":"sec-1"}"#).unwrap();
    assert_eq!(minimal.cursor, None);
    assert_eq!(minimal.max_chars, DEFAULT_SECTION_MAX_CHARS);

    assert_eq!(
        SectionReadRequest {
            max_chars: 0,
            ..minimal.clone()
        }
        .normalized()
        .max_chars,
        1
    );
    assert_eq!(
        SectionReadRequest {
            max_chars: MAX_SECTION_MAX_CHARS + 50,
            ..minimal
        }
        .normalized()
        .max_chars,
        MAX_SECTION_MAX_CHARS
    );
}

#[test]
fn contract_dtos_round_trip_json() {
    let section = DocumentSection {
        section_id: SectionId::new("ownership"),
        heading: "Ownership".to_string(),
        anchor: "ownership".to_string(),
        order: 1,
        line_start: 1,
        line_end: 40,
        body: "Rust ownership rules".to_string(),
    };
    let doc = Document {
        id: DocumentId::new("manual:rust:ownership:001"),
        revision_id: RevisionId::new("sha256-placeholder"),
        source_id: SourceId::new("rust_manual"),
        source: "rust_manual".to_string(),
        title: "Ownership".to_string(),
        summary: "Rust ownership rules".to_string(),
        category: "guide".to_string(),
        topic: Some("rust".to_string()),
        tag: "rust".to_string(),
        tags: vec!["rust".to_string(), "ownership".to_string()],
        keywords: vec!["borrow".to_string(), "move".to_string()],
        language: "rust".to_string(),
        version: "pinned".to_string(),
        date: Some("2026-11-22".to_string()),
        source_uri: "rust/book/ownership.md".to_string(),
        sample_path: None,
        acl_scope: AclScope::Public,
        content_hash: "sha256-placeholder".to_string(),
        body: "Original document content".to_string(),
        markdown: Some("## Ownership\n\nRust ownership rules".to_string()),
        sections: vec![section.clone()],
    };
    let round_doc: Document = serde_json::from_str(&serde_json::to_string(&doc).unwrap()).unwrap();
    assert_eq!(round_doc, doc);

    let detail = DocumentDetail {
        id: doc.id.clone(),
        revision_id: doc.revision_id.clone(),
        source_id: doc.source_id.clone(),
        source: doc.source.clone(),
        title: doc.title.clone(),
        summary: doc.summary.clone(),
        category: doc.category.clone(),
        topic: doc.topic.clone(),
        tag: doc.tag.clone(),
        tags: doc.tags.clone(),
        keywords: doc.keywords.clone(),
        language: doc.language.clone(),
        version: doc.version.clone(),
        date: doc.date.clone(),
        source_uri: doc.source_uri.clone(),
        sample_path: doc.sample_path.clone(),
        acl_scope: doc.acl_scope.clone(),
        body: doc.body.clone(),
        markdown: doc.markdown.clone(),
        sections: vec![SectionSummary::from(&section)],
        formats: Some(DocumentFormatLinks {
            markdown_id: DocumentId::new("design-index-md"),
            yaml_id: DocumentId::new("design-index-yaml"),
        }),
    };
    let round_detail: DocumentDetail =
        serde_json::from_str(&serde_json::to_string(&detail).unwrap()).unwrap();
    assert_eq!(round_detail, detail);

    let snapshot = SearchSnapshotMeta {
        generation_id: GenerationId::new("gen-000002"),
        epoch: 12,
        schema_version: 3,
        acl_policy_version: 2,
        applied_change_sequence: Sequence::new(452_110),
        reader_state: ReaderState::Ready,
    };
    let search_resp = SearchResponse {
        query: "ownership".to_string(),
        page: 1,
        limit: 15,
        total_hits: 1,
        total_pages: 1,
        total_unfiltered_hits: 1,
        category_counts: vec![CategoryCount {
            id: "guide".to_string(),
            label: "개발 가이드".to_string(),
            count: 1,
        }],
        related_searches: vec!["Rust".to_string()],
        hits: vec![SearchHit {
            document_id: doc.id.clone(),
            revision_id: doc.revision_id.clone(),
            title: doc.title.clone(),
            summary: doc.summary.clone(),
            snippet: "Rust ownership rules".to_string(),
            category: doc.category.clone(),
            topic: doc.topic.clone(),
            source: doc.source.clone(),
            tag: doc.tag.clone(),
            tags: doc.tags.clone(),
            date: doc.date.clone(),
            sample_path: None,
            acl_scope: AclScope::Public,
            has_paired_formats: false,
            matched_section_id: Some(SectionId::new("ownership")),
            score: 1.5,
        }],
        snapshot,
    };
    let round_search: SearchResponse =
        serde_json::from_str(&serde_json::to_string(&search_resp).unwrap()).unwrap();
    assert_eq!(round_search, search_resp);

    let section_resp = SectionReadResponse {
        source_revision: RevisionId::new("sha256-example"),
        heading_path: vec!["Rust".to_string(), "Ownership".to_string()],
        citation: SectionCitation {
            source_uri: "kb://rust_manual/chapter-01".to_string(),
            section: SectionId::new("ownership"),
        },
        text: "Section content here".to_string(),
        next_cursor: None,
        truncated: false,
    };
    let round_sec: SectionReadResponse =
        serde_json::from_str(&serde_json::to_string(&section_resp).unwrap()).unwrap();
    assert_eq!(round_sec, section_resp);

    let change = ChangeEvent {
        sequence: Sequence::new(42),
        document_id: doc.id,
        revision_id: doc.revision_id,
        kind: ChangeKind::Upsert,
    };
    let round_change: ChangeEvent =
        serde_json::from_str(&serde_json::to_string(&change).unwrap()).unwrap();
    assert_eq!(round_change, change);

    let admin = Principal::local_admin();
    assert!(admin.is_admin);
    assert!(admin.allows(&AclScope::Public));
    assert!(!admin.allows(&AclScope::internal("security")));

    let trigger_default: TriggerIndexJobRequest = serde_json::from_str("{}").unwrap();
    assert_eq!(trigger_default.mode, IndexJobMode::Incremental);
    assert_eq!(trigger_default.source_id, None);

    let job_record = IndexJobRecord {
        job_id: "job-0001".to_string(),
        source_id: Some("local-markdown".to_string()),
        trigger: "manual".to_string(),
        mode: IndexJobMode::Full,
        state: IndexJobState::Succeeded,
        started_at_ms: 1_700_000_000_000,
        finished_at_ms: Some(1_700_000_000_125),
        indexed_documents: 3,
        rebuilt: true,
        active_generation: Some("gen-000001-seq-000003".to_string()),
        active_epoch: 1,
        skipped_files: vec![],
        error: None,
    };
    let job_list = IndexJobListResponse {
        jobs: vec![job_record],
        active_generation: Some("gen-000001-seq-000003".to_string()),
        active_epoch: 1,
        refresh_interval_secs: Some(30),
    };
    let round_jobs: IndexJobListResponse =
        serde_json::from_str(&serde_json::to_string(&job_list).unwrap()).unwrap();
    assert_eq!(round_jobs, job_list);
}
