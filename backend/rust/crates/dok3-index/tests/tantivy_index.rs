use std::fs;
use std::path::PathBuf;

use dok3_domain::{
    AclScope, Document, DocumentId, DocumentSection, GenerationId, Principal, RevisionId,
    SearchFilters, SearchPort, SearchRequest, SectionId, Sequence, SourceId,
};
use dok3_index::TantivyIndex;

struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new(name: &str) -> Self {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("dok3-index-{name}-{}-{nanos}", std::process::id()));
        fs::create_dir_all(&path).unwrap();
        Self { path }
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[allow(clippy::too_many_arguments)]
fn sample_doc(
    id: &str,
    title: &str,
    summary: &str,
    body: &str,
    category: &str,
    language: &str,
    source_id: &str,
    tag: &str,
    acl_scope: AclScope,
) -> Document {
    Document {
        id: DocumentId::new(id),
        revision_id: RevisionId::new(format!("sha256:{id}")),
        source_id: SourceId::new(source_id),
        source: source_id.to_string(),
        title: title.to_string(),
        summary: summary.to_string(),
        category: category.to_string(),
        tag: tag.to_string(),
        tags: vec![tag.to_string(), language.to_string()],
        keywords: vec![tag.to_string()],
        language: language.to_string(),
        version: "v1".to_string(),
        date: Some("2026-11-22".to_string()),
        source_uri: format!("kb://{source_id}/{id}"),
        sample_path: None,
        acl_scope,
        content_hash: format!("hash-{id}"),
        body: body.to_string(),
        markdown: None,
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

fn search(index: &TantivyIndex, principal: &Principal, query: &str) -> dok3_domain::SearchResponse {
    index
        .search(
            principal,
            &SearchRequest {
                query: query.to_string(),
                ..SearchRequest::default()
            },
        )
        .unwrap()
}

#[test]
fn upsert_commit_reload_and_search_korean_english_and_code() {
    let index = TantivyIndex::create_in_ram(GenerationId::new("gen-1")).unwrap();
    let public = Principal::anonymous_public();

    let doc_kr = sample_doc(
        "doc-kr",
        "인덱스 세대전환 절차",
        "안전한 교체를 설명합니다.",
        "Blue-Green 세대전환 중에는 기존 리더를 유지합니다.",
        "manual",
        "ko",
        "ops-manual",
        "색인",
        AclScope::Public,
    );
    let doc_rust = sample_doc(
        "doc-rust",
        "Tokio 비동기 작업",
        "async runtime guide",
        "Use tokio::spawn for concurrent tasks and handle ORA-00904 errors safely.",
        "code",
        "Rust",
        "rust-docs",
        "Async",
        AclScope::Public,
    );

    index.upsert_document(&doc_kr).unwrap();
    index.upsert_document(&doc_rust).unwrap();

    // Before commit_and_reload, newly added documents are not yet searchable.
    assert_eq!(index.doc_count(), 0);
    assert_eq!(search(&index, &public, "tokio::spawn").total_hits, 0);

    let snap = index.commit_and_reload(Some(Sequence::new(2))).unwrap();
    assert_eq!(snap.epoch, 2);
    assert_eq!(snap.applied_change_sequence, Sequence::new(2));
    assert_eq!(index.doc_count(), 2);

    // Korean substring via body_ngram ("세대" inside "세대전환").
    let kr_res = search(&index, &public, "세대");
    assert_eq!(kr_res.total_hits, 1, "kr_res={:?}", kr_res.hits);
    assert_eq!(kr_res.hits[0].document_id, DocumentId::new("doc-kr"));
    assert!(!kr_res.hits[0].snippet.is_empty());

    // Code identifier and error code.
    let rust_res = search(&index, &public, "tokio::spawn");
    assert_eq!(rust_res.total_hits, 1, "rust_res={:?}", rust_res.hits);
    assert_eq!(rust_res.hits[0].document_id, DocumentId::new("doc-rust"));

    let err_res = search(&index, &public, "ORA-00904");
    assert_eq!(err_res.total_hits, 1, "err_res={:?}", err_res.hits);
    assert_eq!(err_res.hits[0].document_id, DocumentId::new("doc-rust"));

    assert_eq!(search(&index, &public, "nonexistent query").total_hits, 0);

    // Idempotent upsert must replace rather than duplicate.
    let updated_kr = sample_doc(
        "doc-kr",
        "인덱스 세대전환 절차 v2",
        "안전한 교체를 설명합니다.",
        "Blue-Green 세대전환 v2 검증 단계.",
        "manual",
        "ko",
        "ops-manual",
        "색인",
        AclScope::Public,
    );
    index.upsert_document(&updated_kr).unwrap();
    let snap2 = index.commit_and_reload(Some(Sequence::new(3))).unwrap();
    assert_eq!(snap2.epoch, 3);
    assert_eq!(index.doc_count(), 2);
    let kr_v2 = search(&index, &public, "v2");
    assert_eq!(kr_v2.total_hits, 1);
    assert_eq!(kr_v2.hits[0].title, "인덱스 세대전환 절차 v2");
}

#[test]
fn delete_removes_document_after_commit_and_reload() {
    let index = TantivyIndex::create_in_ram(GenerationId::new("gen-del")).unwrap();
    let public = Principal::anonymous_public();

    index
        .upsert_document(&sample_doc(
            "d1",
            "삭제 대상 문서",
            "요약",
            "uniquedelterm 본문",
            "guide",
            "ko",
            "src",
            "Guide",
            AclScope::Public,
        ))
        .unwrap();
    index.commit_and_reload(None).unwrap();
    assert_eq!(index.doc_count(), 1);
    assert_eq!(search(&index, &public, "uniquedelterm").total_hits, 1);

    index.delete_document(&DocumentId::new("d1")).unwrap();
    let del_snap = index.commit_and_reload(None).unwrap();
    assert_eq!(del_snap.applied_change_sequence, Sequence::new(0));
    assert_eq!(index.doc_count(), 0);
    assert_eq!(search(&index, &public, "uniquedelterm").total_hits, 0);

    // Explicit monotonic sequences never regress even on None or lower-sequence commits.
    let s10 = index.commit_and_reload(Some(Sequence::new(10))).unwrap();
    assert_eq!(s10.applied_change_sequence, Sequence::new(10));
    let s_none = index.commit_and_reload(None).unwrap();
    assert_eq!(s_none.applied_change_sequence, Sequence::new(10));
    let s_lower = index.commit_and_reload(Some(Sequence::new(3))).unwrap();
    assert_eq!(s_lower.applied_change_sequence, Sequence::new(10));
}

#[test]
fn acl_enforced_before_hits_and_category_counts() {
    let index = TantivyIndex::create_in_ram(GenerationId::new("gen-acl")).unwrap();
    let ops_scope = AclScope::internal("ops");

    index
        .upsert_document(&sample_doc(
            "pub-1",
            "공개 색인 가이드",
            "공개 요약",
            "sharedaclterm 공개 문서",
            "guide",
            "ko",
            "public-docs",
            "PublicTag",
            AclScope::Public,
        ))
        .unwrap();
    index
        .upsert_document(&sample_doc(
            "int-1",
            "내부 장애 복구",
            "내부 요약",
            "sharedaclterm 비밀 절차",
            "policy",
            "ko",
            "ops-docs",
            "SecretTag",
            ops_scope.clone(),
        ))
        .unwrap();
    index.commit_and_reload(None).unwrap();

    // Anonymous caller sees only public docs when requested_scope is None,
    // and gets 0 hits (never escalates) if requesting an unauthorized scope.
    let public = Principal::anonymous_public();
    let public_res = index
        .search(
            &public,
            &SearchRequest {
                query: "sharedaclterm".to_string(),
                requested_scope: None,
                ..SearchRequest::default()
            },
        )
        .unwrap();
    assert_eq!(public_res.total_hits, 1);
    assert_eq!(public_res.total_unfiltered_hits, 1);
    assert_eq!(public_res.hits[0].document_id, DocumentId::new("pub-1"));
    let public_policy_count = public_res
        .category_counts
        .iter()
        .find(|c| c.id == "policy")
        .map_or(0, |c| c.count);
    assert_eq!(public_policy_count, 0);

    let escalated_req = SearchRequest {
        query: "sharedaclterm".to_string(),
        requested_scope: Some(ops_scope.clone()),
        ..SearchRequest::default()
    };
    let escalated_res = index.search(&public, &escalated_req).unwrap();
    assert_eq!(escalated_res.total_hits, 0);
    assert_eq!(escalated_res.total_unfiltered_hits, 0);
    assert!(escalated_res.hits.is_empty());
    let escalated_policy_count = escalated_res
        .category_counts
        .iter()
        .find(|c| c.id == "policy")
        .map_or(0, |c| c.count);
    assert_eq!(escalated_policy_count, 0);

    // Authorized ops principal with no requested_scope sees both public and internal:ops.
    let ops_principal = Principal {
        subject: "ops-user".to_string(),
        allowed_scopes: vec![AclScope::Public, ops_scope.clone()],
        is_admin: false,
    };
    let ops_all_req = SearchRequest {
        query: "sharedaclterm".to_string(),
        ..SearchRequest::default()
    };
    let ops_res = index.search(&ops_principal, &ops_all_req).unwrap();
    assert_eq!(ops_res.total_hits, 2);
    assert_eq!(ops_res.total_unfiltered_hits, 2);
    let ops_policy = ops_res
        .category_counts
        .iter()
        .find(|c| c.id == "policy")
        .map_or(0, |c| c.count);
    assert_eq!(ops_policy, 1);

    // Narrowing to internal:ops via requested_scope returns only int-1.
    let ops_narrow_res = index.search(&ops_principal, &escalated_req).unwrap();
    assert_eq!(ops_narrow_res.total_hits, 1);
    assert_eq!(ops_narrow_res.hits[0].document_id, DocumentId::new("int-1"));
    assert_eq!(
        ops_narrow_res
            .category_counts
            .iter()
            .find(|c| c.id == "policy")
            .map_or(0, |c| c.count),
        1
    );
}

#[test]
fn disk_index_persists_across_reopen_and_filters_paginate() {
    let dir = TempDir::new("persist");
    let public = Principal::anonymous_public();

    {
        let index = TantivyIndex::open_or_create(&dir.path, GenerationId::new("gen-disk")).unwrap();
        for i in 1..=3 {
            index
                .upsert_document(&sample_doc(
                    &format!("g-{i}"),
                    &format!("Rust 가이드 {i}"),
                    "가이드 요약",
                    &format!("paginationterm rust guide body {i}"),
                    "guide",
                    "Rust",
                    "rust-book",
                    "Ownership",
                    AclScope::Public,
                ))
                .unwrap();
        }
        index
            .upsert_document(&sample_doc(
                "m-1",
                "운영 매뉴얼 1",
                "운영 요약",
                "paginationterm manual body",
                "manual",
                "ko",
                "ops-book",
                "Ops",
                AclScope::Public,
            ))
            .unwrap();
        index.commit_and_reload(Some(Sequence::new(4))).unwrap();
    }

    let reopened =
        TantivyIndex::open_or_create(&dir.path, GenerationId::new("gen-disk-reopen")).unwrap();
    assert_eq!(reopened.doc_count(), 4);

    let req = SearchRequest {
        query: "paginationterm".to_string(),
        filters: SearchFilters {
            category: Some("guide".to_string()),
            language: Some("rust".to_string()),
            source_id: Some(SourceId::new("rust-book")),
            tags: vec!["ownership".to_string()],
        },
        page: 2,
        limit: 2,
        include_snippets: true,
        requested_scope: None,
    };
    let res = reopened.search(&public, &req).unwrap();
    assert_eq!(res.total_hits, 3);
    assert_eq!(res.total_unfiltered_hits, 3);
    assert_eq!(res.total_pages, 2);
    assert_eq!(res.page, 2);
    assert_eq!(res.hits.len(), 1);
    assert!(
        res.category_counts
            .iter()
            .any(|c| c.id == "guide" && c.count == 3)
    );

    // Unfiltered except category=guide preserves manual count too.
    let cat_req = SearchRequest {
        query: "paginationterm".to_string(),
        filters: SearchFilters {
            category: Some("guide".to_string()),
            ..SearchFilters::default()
        },
        ..SearchRequest::default()
    };
    let cat_res = reopened.search(&public, &cat_req).unwrap();
    assert_eq!(cat_res.total_unfiltered_hits, 4);
    assert_eq!(cat_res.total_hits, 3);
    assert!(
        cat_res
            .category_counts
            .iter()
            .any(|c| c.id == "manual" && c.count == 1)
    );
}

#[test]
fn empty_query_and_include_snippets_false() {
    let index = TantivyIndex::create_in_ram(GenerationId::new("gen-empty")).unwrap();
    let public = Principal::anonymous_public();

    index
        .upsert_document(&sample_doc(
            "e1",
            "빈 질의 테스트",
            "요약",
            "본문 텍스트",
            "guide",
            "ko",
            "src",
            "Test",
            AclScope::Public,
        ))
        .unwrap();
    index.commit_and_reload(None).unwrap();

    let res = index
        .search(
            &public,
            &SearchRequest {
                query: "   ".to_string(),
                include_snippets: false,
                ..SearchRequest::default()
            },
        )
        .unwrap();
    assert_eq!(res.total_hits, 1);
    assert_eq!(res.hits[0].snippet, "");
}
