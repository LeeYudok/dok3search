use dok3_domain::Principal;

use crate::{QueryCase, QueryGroup};

/// A smoke suite for the corpus returned by `InMemorySearchRepository::from_seed(None)`.
/// Built-in documents are public, so the denied case uses a principal with no scopes.
#[must_use]
pub fn poc_suite_for_builtin_samples() -> Vec<QueryCase> {
    use QueryGroup::{Acl, ApiError, CodeIdentifier, Korean, Rust};

    let mut cases: Vec<_> = [
        (
            "rust-workspace",
            Rust,
            "Rust 워크스페이스",
            "guide",
            "Rust",
            "rust-workspace",
        ),
        (
            "rust-worker",
            Rust,
            "Rust 색인 워커",
            "guide",
            "Rust",
            "worker-guide",
        ),
        (
            "search-api",
            ApiError,
            "검색 API 요청",
            "code",
            "Tantivy",
            "search-api",
        ),
        (
            "index-error-retry",
            ApiError,
            "실패한 색인 작업",
            "manual",
            "색인",
            "retry-index",
        ),
        (
            "korean-spacing",
            Korean,
            "띄어쓰기 복합어",
            "guide",
            "한국어",
            "korean-analysis",
        ),
        (
            "korean-retention",
            Korean,
            "문서 보존 기간",
            "policy",
            "보존",
            "retention-policy",
        ),
        (
            "tantivy-identifier",
            CodeIdentifier,
            "Tantivy 검색 필드",
            "code",
            "Tantivy",
            "search-fields",
        ),
        (
            "acl-public",
            Acl,
            "문서 가시성과 권한",
            "policy",
            "색인",
            "document-visibility",
        ),
    ]
    .into_iter()
    .map(|(id, group, query, category, tag, expected)| QueryCase {
        id: id.into(),
        group,
        query: query.into(),
        category_filter: Some(category.into()),
        tags: vec![tag.into()],
        expected_doc_ids: vec![expected.into()],
        forbidden_doc_ids: Vec::new(),
        principal: Principal::anonymous_public(),
    })
    .collect();

    let mut denied = cases.last().expect("public ACL case").clone();
    denied.id = "acl-denied".into();
    denied.forbidden_doc_ids = std::mem::take(&mut denied.expected_doc_ids);
    denied.principal.allowed_scopes.clear();
    cases.push(denied);
    cases
}
