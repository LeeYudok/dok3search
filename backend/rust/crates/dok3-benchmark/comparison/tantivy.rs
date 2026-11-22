use std::error::Error;
use std::sync::Arc;

use dok3_benchmark::{EvalConfig, LoadConfig, QueryCase, QueryGroup, evaluate, run_load};
use dok3_domain::{AclScope, Document, GenerationId, Principal, SearchPort, SearchRequest};
use dok3_index::TantivyIndex;
use dok3_search::{InMemorySearchRepository, SearchService};
use serde_json::json;

const CASES: &[(&str, QueryGroup, &str, &str)] = &[
    ("camel", QueryGroup::CodeIdentifier, "write x", "writeX"),
    ("pascal", QueryGroup::CodeIdentifier, "load x", "LoadX"),
    ("snake", QueryGroup::CodeIdentifier, "read y", "read_y"),
    (
        "namespace",
        QueryGroup::CodeIdentifier,
        "cache::lookup",
        "cache::LookupRecord",
    ),
    (
        "code",
        QueryGroup::ApiError,
        "ORA-00904",
        "ORA-00904 invalid identifier",
    ),
    (
        "korean",
        QueryGroup::Korean,
        "전환절차",
        "안전한 세대전환절차 안내",
    ),
    ("acl", QueryGroup::Acl, "rotate token", "rotateToken"),
];

const KOREAN_CASES: &[(&str, &str, &str)] = &[
    ("bank-particle", "은행은", "은행에서 안내합니다."),
    ("loan-particle", "대출을", "대출부터 안내합니다."),
    ("interest-particle", "금리의", "금리도 안내합니다."),
    (
        "account-compound",
        "계좌잔액조회",
        "계좌 잔액 조회 방법입니다.",
    ),
    (
        "credit-compound",
        "여신 심사 기준을",
        "여신심사기준은 참고 항목입니다.",
    ),
    (
        "warning-compound",
        "조기경보모형",
        "조기 경보 모형을 설명합니다.",
    ),
    ("deploy-compound", "폐쇄망배포", "폐쇄망 배포 절차입니다."),
    (
        "storage-compound",
        "원문저장소",
        "원문 저장소의 구조입니다.",
    ),
    ("recovery-bare", "장애복구", "장애복구를 설명합니다."),
    ("results-bare", "검색 결과", "검색결과에서 확인합니다."),
];

fn document(id: &str, body: &str, scope: AclScope, category: &str) -> Document {
    Document {
        id: id.into(),
        revision_id: "sha256:fixture".into(),
        source_id: "comparison".into(),
        source: "comparison".into(),
        title: "Reference".into(),
        summary: String::new(),
        category: category.into(),
        topic: None,
        tag: id.into(),
        tags: vec![id.into()],
        keywords: Vec::new(),
        language: "text".into(),
        version: "v1".into(),
        date: None,
        source_uri: format!("kb://comparison/{id}"),
        sample_path: None,
        acl_scope: scope,
        content_hash: id.into(),
        body: body.into(),
        markdown: None,
        sections: Vec::new(),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let candidate = std::env::args().nth(1).as_deref() == Some("candidate");
    let korean = std::env::args().nth(2).as_deref() == Some("korean");
    let fixtures: Vec<_> = if korean {
        KOREAN_CASES
            .iter()
            .map(|&(id, query, body)| (id, QueryGroup::Korean, query, body))
            .collect()
    } else {
        CASES.to_vec()
    };
    let ops = AclScope::internal("ops");
    let index = Arc::new(TantivyIndex::create_in_ram(GenerationId::new(
        "comparison",
    ))?);
    for n in 0..1000 {
        index.upsert_document(&document(
            &format!("background-{n}"),
            &format!("Runtime reference module {n}: memory allocation, scheduling, diagnostics"),
            AclScope::Public,
            "guide",
        ))?;
    }
    let mut cases = Vec::new();
    for &(id, group, query, body) in &fixtures {
        let private = format!("private-{id}");
        index.upsert_document(&document(id, body, AclScope::Public, "code"))?;
        index.upsert_document(&document(&private, body, ops.clone(), "policy"))?;
        cases.push(QueryCase {
            id: id.into(),
            group,
            query: query.into(),
            category_filter: None,
            tags: Vec::new(),
            expected_doc_ids: vec![id.into()],
            forbidden_doc_ids: vec![private.into()],
            principal: Principal::anonymous_public(),
        });
    }
    index.commit_and_reload(None)?;
    let service = Arc::new(SearchService::new(
        index.clone(),
        Arc::new(InMemorySearchRepository::new(Vec::new(), Vec::new())),
    ));
    let eval = evaluate(&service, &cases, EvalConfig { k: 5 })?;
    let mut acl_metadata_violations = 0;
    let mut authorized_cases = 0;
    let mut load = Vec::new();
    let config = LoadConfig {
        threads: 4,
        iterations_per_thread: 500,
        warmup_iterations: 50,
    };
    let operator = Principal {
        subject: "operator".into(),
        allowed_scopes: vec![ops.clone()],
        is_admin: false,
    };

    for case in &cases {
        let request = SearchRequest {
            query: case.query.clone(),
            limit: 5,
            ..SearchRequest::default()
        };
        let public = index.search(&case.principal, &request)?;
        let narrowed = SearchRequest {
            requested_scope: Some(ops.clone()),
            ..request.clone()
        };
        let denied = index.search(&case.principal, &narrowed)?;
        let authorized = index.search(&operator, &narrowed)?;
        authorized_cases += usize::from(
            authorized.total_hits == 1
                && authorized
                    .hits
                    .iter()
                    .any(|hit| case.forbidden_doc_ids.contains(&hit.document_id)),
        );
        acl_metadata_violations += usize::from(
            public.total_hits > 1
                || public.total_unfiltered_hits > 1
                || public
                    .category_counts
                    .iter()
                    .any(|c| c.id == "policy" && c.count > 0)
                || public
                    .related_searches
                    .iter()
                    .any(|tag| case.forbidden_doc_ids.iter().any(|id| id.as_str() == tag))
                || denied.total_hits != 0
                || denied.total_unfiltered_hits != 0
                || denied.category_counts.iter().any(|c| c.count > 0)
                || !denied.hits.is_empty()
                || !denied.related_searches.is_empty(),
        );
        let report = run_load(service.clone(), &case.principal, &request, config)?;
        assert_eq!(report.error_count, 0, "{}", case.id);
        load.push(json!({ "id": case.id, "load": report }));
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "schema_version": index.snapshot().schema_version,
            "documents": index.doc_count(),
            "quality": eval,
            "acl_metadata_violations": acl_metadata_violations,
            "authorized_cases": authorized_cases,
            "load_by_case": load,
        }))?
    );
    assert_eq!(eval.acl_violations, 0);
    assert_eq!(acl_metadata_violations, 0);
    if korean {
        let quality = &eval.by_group[&QueryGroup::Korean];
        let expected = if candidate { 1.0 } else { 0.2 };
        assert_eq!(quality.recall_at_k, expected);
        assert_eq!(quality.mrr_at_k, expected);
    }
    if candidate {
        assert_eq!(authorized_cases, cases.len());
        assert!(
            eval.case_results
                .iter()
                .all(|case| case.reciprocal_rank_at_k == 1.0)
        );
    }
    Ok(())
}
