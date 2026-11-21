use std::sync::Arc;

use dok3_benchmark::{
    EvalConfig, LoadConfig, QueryCase, QueryGroup, evaluate, poc_suite_for_builtin_samples,
    run_load,
};
use dok3_core::Dok3Error;
use dok3_domain::{AclScope, Principal, SearchRequest};
use dok3_search::{InMemorySearchRepository, SearchService, load_seed_dataset};

fn service(repo: InMemorySearchRepository) -> SearchService {
    let repo = Arc::new(repo);
    SearchService::new(repo.clone(), repo)
}

fn case(id: &str, expected: &[&str], forbidden: &[&str]) -> QueryCase {
    QueryCase {
        id: id.into(),
        group: QueryGroup::Acl,
        query: String::new(),
        category_filter: None,
        tags: Vec::new(),
        expected_doc_ids: expected.iter().map(|&id| id.into()).collect(),
        forbidden_doc_ids: forbidden.iter().map(|&id| id.into()).collect(),
        principal: Principal::anonymous_public(),
    }
}

#[test]
fn recall_and_mrr_use_document_sets_and_top_k_ranks() {
    let service = service(InMemorySearchRepository::from_seed(None));
    let cases = [
        case("first", &["index-rollbacks"], &[]),
        case("second", &["rust-workspace"], &[]),
        case("missing", &["unindexed"], &[]),
        case(
            "partial",
            &["rust-workspace", "rust-workspace", "tantivy-query"],
            &[],
        ),
    ];
    let report = evaluate(&service, &cases, EvalConfig { k: 2 }).unwrap();
    let scores: Vec<_> = report
        .case_results
        .iter()
        .map(|case| (case.recall_at_k, case.reciprocal_rank_at_k))
        .collect();
    assert_eq!(scores, [(1.0, 1.0), (1.0, 0.5), (0.0, 0.0), (0.5, 0.5)]);
    assert_eq!(report.overall_recall_at_k, 0.625);
    assert_eq!(report.overall_mrr_at_k, 0.5);
    assert_eq!(report.by_group[&QueryGroup::Acl].total_cases, 4);
    assert_eq!(report.by_group[&QueryGroup::Acl].recall_at_k, 0.625);
    assert_eq!(report.by_group[&QueryGroup::Acl].mrr_at_k, 0.5);

    let cutoff = evaluate(&service, &cases[1..2], EvalConfig { k: 1 }).unwrap();
    assert_eq!(cutoff.overall_recall_at_k, 0.0);
    assert_eq!(cutoff.overall_mrr_at_k, 0.0);

    let empty = evaluate(&service, &[], EvalConfig::default()).unwrap();
    assert_eq!(empty.overall_recall_at_k, 0.0);
    assert_eq!(empty.overall_mrr_at_k, 0.0);
}

#[test]
fn evaluation_spans_the_search_page_size_limit() {
    let mut seed = load_seed_dataset(None);
    let template = seed.documents[0].clone();
    seed.documents = (1..=101)
        .map(|rank| {
            let mut doc = template.clone();
            doc.id = format!("doc-{rank}").into();
            doc
        })
        .collect();
    let service = service(InMemorySearchRepository::new(seed.documents, seed.pairs));
    let cases = [case("second-page", &["doc-101"], &["doc-101"])];
    let report = evaluate(&service, &cases, EvalConfig { k: 101 }).unwrap();
    assert_eq!(report.overall_recall_at_k, 1.0);
    assert_eq!(report.overall_mrr_at_k, 1.0 / 101.0);
    assert_eq!(report.acl_violations, 1);
}

#[test]
fn detects_forbidden_hits_once_per_case_and_respects_cutoff() {
    let service = service(InMemorySearchRepository::from_seed(None));
    let cases = [case(
        "deliberate-leak",
        &[],
        &["index-rollbacks", "rust-workspace", "tantivy-query"],
    )];
    let report = evaluate(&service, &cases, EvalConfig { k: 2 }).unwrap();
    assert_eq!(report.acl_violations, 1);
    assert_eq!(
        report.case_results[0].forbidden_hits,
        ["index-rollbacks".into(), "rust-workspace".into()]
    );
}

#[test]
fn built_in_suite_covers_each_group_and_serializes() {
    let service = service(InMemorySearchRepository::from_seed(None));
    let cases = poc_suite_for_builtin_samples();
    let report = evaluate(&service, &cases, EvalConfig::default()).unwrap();
    assert_eq!(report.k, 10);
    assert_eq!(report.total_cases, cases.len());
    assert_eq!(report.by_group.len(), 5);
    assert_eq!(report.acl_violations, 0);
    for (case, result) in cases.iter().zip(&report.case_results) {
        if !case.expected_doc_ids.is_empty() {
            assert_eq!(result.recall_at_k, 1.0, "case {}", case.id);
        }
    }
    let json = serde_json::to_value(&report).unwrap();
    assert_eq!(json["by_group"]["api_error"]["total_cases"], 2);
    assert_eq!(json["acl_violations"], 0);
}

#[test]
fn principals_see_only_their_authorized_documents() {
    let mut seed = load_seed_dataset(None);
    let mut private = seed.documents[0].clone();
    private.id = "private-rollbacks".into();
    let scope = AclScope::internal("ops");
    private.acl_scope = scope.clone();
    seed.documents.insert(0, private);
    let service = service(InMemorySearchRepository::new(seed.documents, seed.pairs));

    let mut public = case("public-only", &["index-rollbacks"], &["private-rollbacks"]);
    public.query = "롤백".into();
    let mut internal = case("ops-only", &["private-rollbacks"], &["index-rollbacks"]);
    internal.query.clone_from(&public.query);
    internal.principal.subject = "operator".into();
    internal.principal.allowed_scopes = vec![scope];

    let report = evaluate(&service, &[public, internal], EvalConfig { k: 1 }).unwrap();
    assert_eq!(report.acl_violations, 0);
    assert_eq!(report.overall_recall_at_k, 1.0);
    assert_eq!(report.overall_mrr_at_k, 1.0);
}

#[test]
fn concurrent_load_counts_measured_attempts_and_serializes_latency() {
    let service = Arc::new(service(InMemorySearchRepository::from_seed(None)));
    let principal = Principal::anonymous_public();
    let mut request = SearchRequest {
        query: "Rust".into(),
        ..SearchRequest::default()
    };
    let config = LoadConfig {
        threads: 4,
        iterations_per_thread: 8,
        warmup_iterations: 2,
    };

    let report = run_load(service.clone(), &principal, &request, config).unwrap();
    assert_eq!(report.total_requests, 32);
    assert_eq!(report.error_count, 0);
    assert!(report.qps.is_finite() && report.qps > 0.0);
    assert!(report.p50_ms <= report.p95_ms);
    assert!(report.p95_ms <= report.p99_ms);
    assert!(report.p99_ms <= report.max_ms);
    let json = serde_json::to_value(&report).unwrap();
    assert_eq!(json["total_requests"], 32);
    assert_eq!(json["p95_ms"].as_f64(), Some(report.p95_ms));

    request.query = "x".repeat(513);
    let errors = run_load(service.clone(), &principal, &request, config).unwrap();
    assert_eq!(errors.total_requests, 32);
    assert_eq!(errors.error_count, 32);

    let empty = run_load(
        service,
        &principal,
        &request,
        LoadConfig {
            iterations_per_thread: 0,
            ..config
        },
    )
    .unwrap();
    assert_eq!(empty.total_requests, 0);
    assert_eq!(empty.error_count, 0);
    assert_eq!(empty.elapsed_ms, 0.0);
    assert_eq!(empty.qps, 0.0);
    assert_eq!(empty.p95_ms, 0.0);
}

#[test]
fn rejects_invalid_configuration() {
    let service = Arc::new(service(InMemorySearchRepository::from_seed(None)));
    assert!(matches!(
        evaluate(&service, &[], EvalConfig { k: 0 }),
        Err(Dok3Error::InvalidInput(_))
    ));
    assert!(matches!(
        run_load(
            service,
            &Principal::anonymous_public(),
            &SearchRequest::default(),
            LoadConfig {
                threads: 0,
                iterations_per_thread: 1,
                warmup_iterations: 0
            }
        ),
        Err(Dok3Error::InvalidInput(_))
    ));
}
