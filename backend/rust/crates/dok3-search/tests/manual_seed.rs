//! The built-in `dok3search-manual` document must be searchable and readable by section.

use std::sync::Arc;

use dok3_core::Dok3Error;
use dok3_domain::{
    AclScope, DocumentId, Principal, SearchFilters, SearchRequest, SearchResponse, SectionId,
    SectionReadRequest, SectionReadResponse,
};
use dok3_search::{InMemorySearchRepository, SearchService};

const MANUAL_ID: &str = "dok3search-manual";
const MANUAL_TITLE: &str = "dok3search 제품·운영 매뉴얼";
const BACKUP: &str = "section-백업과-복구";

fn seed_service() -> SearchService {
    let repo = Arc::new(InMemorySearchRepository::from_seed(None));
    SearchService::new(repo.clone(), repo)
}

fn hit_ids(resp: &SearchResponse) -> Vec<&str> {
    resp.hits.iter().map(|h| h.document_id.as_str()).collect()
}

fn manual_search(service: &SearchService, query: &str) -> SearchResponse {
    service
        .search(
            &Principal::anonymous_public(),
            &SearchRequest {
                query: query.to_string(),
                filters: SearchFilters {
                    category: Some("manual".to_string()),
                    ..SearchFilters::default()
                },
                limit: 100,
                ..SearchRequest::default()
            },
        )
        .unwrap()
}

#[test]
fn manual_category_search_finds_builtin_operations_manual() {
    let service = seed_service();

    let all_manual = manual_search(&service, "");
    assert!(all_manual.hits.iter().all(|hit| hit.category == "manual"));
    let manual_hit = all_manual
        .hits
        .iter()
        .find(|hit| hit.document_id.as_str() == MANUAL_ID)
        .expect("manual category must list dok3search-manual");
    assert_eq!(manual_hit.acl_scope, AclScope::Public);
    assert_eq!(manual_hit.title, MANUAL_TITLE);

    // Terms that only appear in the manual body (operations topics) still reach it.
    for query in ["DOK3_DATA_DIR", "kb_get_section", "백업"] {
        let resp = manual_search(&service, query);
        assert!(
            hit_ids(&resp).contains(&MANUAL_ID),
            "query {query:?} should find the manual"
        );
    }

    // The manual must not push the existing sample "롤백" document off the top rank.
    let rollback = manual_search(&service, "롤백");
    assert_eq!(rollback.hits[0].document_id.as_str(), "index-rollbacks");
}

#[test]
fn manual_document_and_sections_are_readable_with_citations() {
    let service = seed_service();
    let public = Principal::anonymous_public();

    let id = DocumentId::new(MANUAL_ID);
    let detail = service.get_document(&public, &id).unwrap();
    assert_eq!(detail.category, "manual");
    assert_eq!(detail.source_id.as_str(), MANUAL_ID);
    let markdown = detail.markdown.as_deref().unwrap_or_default();
    assert!(markdown.contains("SCHEMA_VERSION"));
    let section_ids: Vec<&str> = detail
        .sections
        .iter()
        .map(|s| s.section_id.as_str())
        .collect();
    assert!(section_ids.contains(&BACKUP));
    assert!(section_ids.contains(&"section-mcp-도구"));

    let backup = read_section(&service, BACKUP, None, 100_000).unwrap();
    assert_eq!(backup.heading_path, [MANUAL_TITLE, "백업과 복구"]);
    assert_eq!(backup.citation.section.as_str(), BACKUP);
    assert_eq!(
        backup.citation.source_uri,
        "repo://docs/manual/dok3search-manual.md"
    );
    assert!(backup.text.contains("DOK3_DATA_DIR"));
    assert!(!backup.truncated);

    // Cursor pagination walks the section without losing characters.
    let first = read_section(&service, BACKUP, None, 50).unwrap();
    assert!(first.truncated);
    let cursor = first.next_cursor;
    let rest = read_section(&service, BACKUP, cursor, 100_000).unwrap();
    assert_eq!(format!("{}{}", first.text, rest.text), backup.text);

    let missing = read_section(&service, "section-없는-절", None, 100);
    assert!(matches!(missing, Err(Dok3Error::NotFound(_))));
}

fn read_section(
    service: &SearchService,
    section_id: &str,
    cursor: Option<usize>,
    max_chars: usize,
) -> Result<SectionReadResponse, Dok3Error> {
    let req = SectionReadRequest {
        document_id: DocumentId::new(MANUAL_ID),
        section_id: SectionId::new(section_id),
        cursor,
        max_chars,
    };
    service.get_section(&Principal::anonymous_public(), &req)
}
