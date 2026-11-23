use std::path::PathBuf;
use std::sync::Arc;

use dok3_core::Dok3Error;
use dok3_domain::{
    AclScope, Document, DocumentId, DocumentSection, Principal, RevisionId, SearchFilters,
    SearchRequest, SectionId, SectionReadRequest, SourceId,
};
use dok3_search::{InMemorySearchRepository, SearchService};

fn sample_design_dir() -> PathBuf {
    let dir =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../frontend/src/samples/design");
    assert!(
        dir.is_dir(),
        "missing design samples dir: {}",
        dir.display()
    );
    dir
}

fn make_doc(id: &str, title: &str, category: &str, scope: AclScope, body: &str) -> Document {
    Document {
        id: DocumentId::new(id),
        revision_id: RevisionId::new("rev-1"),
        source_id: SourceId::new("test-source"),
        source: "테스트 출처".to_string(),
        title: title.to_string(),
        summary: format!("{title} 요약"),
        category: category.to_string(),
        topic: None,
        tag: "테스트".to_string(),
        tags: vec!["테스트".to_string()],
        keywords: vec![],
        language: "ko".to_string(),
        version: "1.0".to_string(),
        date: Some("2026-11-22".to_string()),
        source_uri: format!("test://{id}"),
        sample_path: None,
        acl_scope: scope,
        content_hash: "hash-1".to_string(),
        body: body.to_string(),
        markdown: None,
        sections: vec![DocumentSection {
            section_id: SectionId::new("sec-1"),
            heading: "첫째 절".to_string(),
            anchor: "sec-1".to_string(),
            order: 1,
            line_start: 1,
            line_end: 2,
            body: body.to_string(),
        }],
    }
}

#[test]
fn seed_search_pairs_markdown_and_yaml_and_preserves_unfiltered_counts() {
    let design_dir = sample_design_dir();
    let repo = Arc::new(InMemorySearchRepository::from_seed(Some(&design_dir)));
    let service = SearchService::new(repo.clone(), repo);
    let public = Principal::anonymous_public();

    let guide_resp = service
        .search(
            &public,
            &SearchRequest {
                filters: SearchFilters {
                    category: Some("guide".to_string()),
                    ..SearchFilters::default()
                },
                limit: 100,
                ..SearchRequest::default()
            },
        )
        .unwrap();
    assert!(guide_resp.hits.iter().any(|hit| {
        hit.document_id.as_str() == "rust-workspace" && hit.topic.as_deref() == Some("rust")
    }));

    // "entrypoints" exists in index.yaml and not in index.md body; paired search must return design-index-md once.
    let yaml_term_resp = service
        .search(
            &public,
            &SearchRequest {
                query: "entrypoints".to_string(),
                ..SearchRequest::default()
            },
        )
        .unwrap();
    assert!(
        yaml_term_resp
            .hits
            .iter()
            .any(|hit| hit.document_id.as_str() == "design-index-md" && hit.has_paired_formats)
    );
    assert!(
        !yaml_term_resp
            .hits
            .iter()
            .any(|hit| hit.document_id.as_str() == "design-index-yaml")
    );

    // Filtering by category preserves unfiltered category_counts for the query.
    let tantivy_all = service
        .search(
            &public,
            &SearchRequest {
                query: "Tantivy".to_string(),
                limit: 5,
                page: 1,
                ..SearchRequest::default()
            },
        )
        .unwrap();
    let tantivy_code = service
        .search(
            &public,
            &SearchRequest {
                query: "Tantivy".to_string(),
                filters: SearchFilters {
                    category: Some("code".to_string()),
                    ..SearchFilters::default()
                },
                limit: 5,
                page: 1,
                ..SearchRequest::default()
            },
        )
        .unwrap();
    assert_eq!(tantivy_all.category_counts, tantivy_code.category_counts);
    assert_eq!(
        tantivy_all.total_unfiltered_hits,
        tantivy_code.total_unfiltered_hits
    );
    assert!(!tantivy_code.related_searches.is_empty());
    assert!(tantivy_code.hits.iter().all(|hit| hit.category == "code"));

    // Paired document detail exposes both format IDs.
    let detail = service
        .get_document(&public, &DocumentId::new("design-index-md"))
        .unwrap();
    let formats = detail.formats.expect("index.md should expose YAML pair");
    assert_eq!(formats.markdown_id.as_str(), "design-index-md");
    assert_eq!(formats.yaml_id.as_str(), "design-index-yaml");
}

#[test]
fn section_read_paginates_multibyte_korean_safely() {
    let body = "가나다라마바사아자차카타파하";
    let docs = vec![make_doc("ko-1", "한글 절", "guide", AclScope::Public, body)];
    let repo = Arc::new(InMemorySearchRepository::new(docs, vec![]));
    let service = SearchService::new(repo.clone(), repo);
    let public = Principal::anonymous_public();

    let first = service
        .get_section(
            &public,
            &SectionReadRequest {
                document_id: DocumentId::new("ko-1"),
                section_id: SectionId::new("sec-1"),
                cursor: None,
                max_chars: 5,
            },
        )
        .unwrap();
    assert_eq!(first.text, "가나다라마");
    assert!(first.truncated);
    assert_eq!(first.next_cursor, Some(5));

    let second = service
        .get_section(
            &public,
            &SectionReadRequest {
                document_id: DocumentId::new("ko-1"),
                section_id: SectionId::new("sec-1"),
                cursor: first.next_cursor,
                max_chars: 10,
            },
        )
        .unwrap();
    assert_eq!(second.text, "바사아자차카타파하");
    assert!(!second.truncated);
    assert_eq!(second.next_cursor, None);
}

#[test]
fn acl_denies_internal_docs_across_search_counts_and_readers() {
    let docs = vec![
        make_doc(
            "pub-1",
            "공개 문서",
            "guide",
            AclScope::Public,
            "공통 키워드",
        ),
        make_doc(
            "sec-1",
            "보안 대외비",
            "policy",
            AclScope::internal("security"),
            "공통 키워드 비밀내용",
        ),
    ];
    let repo = Arc::new(InMemorySearchRepository::new(docs, vec![]));
    let service = SearchService::new(repo.clone(), repo);

    let public = Principal::anonymous_public();
    let authorized = Principal {
        subject: "sec-user".to_string(),
        allowed_scopes: vec![AclScope::Public, AclScope::internal("security")],
        is_admin: false,
    };

    // Public search hides internal doc from hits and category counts.
    let pub_res = service
        .search(
            &public,
            &SearchRequest {
                query: "공통".to_string(),
                ..SearchRequest::default()
            },
        )
        .unwrap();
    assert_eq!(pub_res.total_hits, 1);
    assert_eq!(pub_res.hits[0].document_id.as_str(), "pub-1");
    let policy_count = pub_res
        .category_counts
        .iter()
        .find(|c| c.id == "policy")
        .unwrap()
        .count;
    assert_eq!(policy_count, 0);

    // Public caller attempting scope escalation via requested_scope gets zero hits.
    let escalated = service
        .search(
            &public,
            &SearchRequest {
                query: "공통".to_string(),
                requested_scope: Some(AclScope::internal("security")),
                ..SearchRequest::default()
            },
        )
        .unwrap();
    assert_eq!(escalated.total_hits, 0);

    // Direct document and section reads return Forbidden for unauthorized caller.
    assert!(matches!(
        service.get_document(&public, &DocumentId::new("sec-1")),
        Err(Dok3Error::Forbidden(_))
    ));
    assert!(matches!(
        service.get_section(
            &public,
            &SectionReadRequest {
                document_id: DocumentId::new("sec-1"),
                section_id: SectionId::new("sec-1"),
                cursor: None,
                max_chars: 100,
            }
        ),
        Err(Dok3Error::Forbidden(_))
    ));

    // Authorized caller sees both documents and can read internal document.
    let auth_res = service
        .search(
            &authorized,
            &SearchRequest {
                query: "공통".to_string(),
                ..SearchRequest::default()
            },
        )
        .unwrap();
    assert_eq!(auth_res.total_hits, 2);
    assert!(
        service
            .get_document(&authorized, &DocumentId::new("sec-1"))
            .is_ok()
    );

    // Corpus catalog respects ACL: public sees only pub-1, authorized sees both.
    let pub_corpora = service.list_corpora(&public, "in-memory-fake").unwrap();
    assert_eq!(pub_corpora.total_documents, 1);
    let auth_corpora = service.list_corpora(&authorized, "in-memory-fake").unwrap();
    assert_eq!(auth_corpora.total_documents, 2);
}
