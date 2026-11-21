//! Document-level recall, reciprocal rank, and forbidden-hit checks.

use std::collections::{BTreeMap, BTreeSet};

use dok3_core::Dok3Error;
use dok3_domain::{DocumentId, Principal, SearchFilters, SearchRequest};
use dok3_search::SearchService;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryGroup {
    Rust,
    ApiError,
    Korean,
    CodeIdentifier,
    Acl,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryCase {
    pub id: String,
    pub group: QueryGroup,
    pub query: String,
    pub category_filter: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub expected_doc_ids: Vec<DocumentId>,
    #[serde(default)]
    pub forbidden_doc_ids: Vec<DocumentId>,
    pub principal: Principal,
}

impl QueryCase {
    pub(crate) fn request(&self, limit: usize) -> SearchRequest {
        SearchRequest {
            query: self.query.clone(),
            filters: SearchFilters {
                category: self.category_filter.clone(),
                tags: self.tags.clone(),
                ..SearchFilters::default()
            },
            limit,
            ..SearchRequest::default()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EvalConfig {
    pub k: usize,
}

impl Default for EvalConfig {
    fn default() -> Self {
        Self { k: 10 }
    }
}

#[derive(Debug, Serialize)]
pub struct CaseResult {
    pub id: String,
    pub group: QueryGroup,
    pub recall_at_k: f64,
    pub reciprocal_rank_at_k: f64,
    pub forbidden_hits: Vec<DocumentId>,
}

#[derive(Debug, Default, Serialize)]
pub struct GroupResult {
    pub total_cases: usize,
    pub recall_at_k: f64,
    pub mrr_at_k: f64,
}

impl GroupResult {
    fn record(&mut self, result: &CaseResult) {
        self.total_cases += 1;
        self.recall_at_k += result.recall_at_k;
        self.mrr_at_k += result.reciprocal_rank_at_k;
    }

    fn average(&mut self) {
        if self.total_cases > 0 {
            self.recall_at_k /= self.total_cases as f64;
            self.mrr_at_k /= self.total_cases as f64;
        }
    }
}

#[derive(Debug, Serialize)]
pub struct EvalReport {
    pub k: usize,
    pub total_cases: usize,
    pub overall_recall_at_k: f64,
    pub overall_mrr_at_k: f64,
    pub by_group: BTreeMap<QueryGroup, GroupResult>,
    pub case_results: Vec<CaseResult>,
    /// Number of cases with at least one forbidden hit.
    pub acl_violations: usize,
}

/// Computes macro-averaged metrics across cases. Cases with no expected documents
/// score zero for both ranking metrics and are still checked for ACL violations.
pub fn evaluate(
    service: &SearchService,
    cases: &[QueryCase],
    config: EvalConfig,
) -> Result<EvalReport, Dok3Error> {
    if config.k == 0 {
        return Err(Dok3Error::InvalidInput("k must be positive".into()));
    }

    let mut report = EvalReport {
        k: config.k,
        total_cases: cases.len(),
        overall_recall_at_k: 0.0,
        overall_mrr_at_k: 0.0,
        by_group: BTreeMap::new(),
        case_results: Vec::with_capacity(cases.len()),
        acl_violations: 0,
    };
    let mut overall = GroupResult::default();
    for case in cases {
        let result = evaluate_case(service, case, config.k)?;
        overall.record(&result);
        report
            .by_group
            .entry(case.group)
            .or_default()
            .record(&result);
        report.acl_violations += usize::from(!result.forbidden_hits.is_empty());
        report.case_results.push(result);
    }
    overall.average();
    report.overall_recall_at_k = overall.recall_at_k;
    report.overall_mrr_at_k = overall.mrr_at_k;
    for group in report.by_group.values_mut() {
        group.average();
    }
    Ok(report)
}

fn evaluate_case(
    service: &SearchService,
    case: &QueryCase,
    k: usize,
) -> Result<CaseResult, Dok3Error> {
    // SearchRequest caps page size at 100. Preserve a fixed limit across pages.
    let mut request = case.request(k.min(100));
    let expected: BTreeSet<_> = case.expected_doc_ids.iter().collect();
    let forbidden: BTreeSet<_> = case.forbidden_doc_ids.iter().collect();
    let mut matched = BTreeSet::new();
    let mut forbidden_hits = BTreeSet::new();
    let mut first_rank = None;
    let mut rank = 0;
    loop {
        let response = service.search(&case.principal, &request)?;
        for hit in response.hits.into_iter().take(k - rank) {
            rank += 1;
            if expected.contains(&hit.document_id) {
                first_rank.get_or_insert(rank);
                matched.insert(hit.document_id.clone());
            }
            if forbidden.contains(&hit.document_id) {
                forbidden_hits.insert(hit.document_id);
            }
        }
        if rank == k || response.page < request.page || request.page >= response.total_pages {
            break;
        }
        request.page += 1;
    }

    Ok(CaseResult {
        id: case.id.clone(),
        group: case.group,
        recall_at_k: matched.len() as f64 / expected.len().max(1) as f64,
        reciprocal_rank_at_k: first_rank.map_or(0.0, |rank| 1.0 / rank as f64),
        forbidden_hits: forbidden_hits.into_iter().collect(),
    })
}
