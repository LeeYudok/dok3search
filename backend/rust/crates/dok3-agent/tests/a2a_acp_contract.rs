use std::sync::Arc;

use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use dok3_agent::{AcpAdapter, AcpRequest, AcpResponse, McpServer, build_agent_router};
use dok3_domain::{
    AclScope, Document, DocumentSection, Principal, RevisionId, SectionId, SourceId,
};
use dok3_search::{InMemorySearchRepository, SearchService};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tower::ServiceExt;

fn sample_doc(id: &str, title: &str, body: &str, acl_scope: AclScope) -> Document {
    Document {
        id: id.into(),
        revision_id: RevisionId::new("rev-1"),
        source_id: SourceId::new("src"),
        source: "테스트".to_string(),
        title: title.to_string(),
        summary: title.to_string(),
        category: "guide".to_string(),
        topic: None,
        tag: "Rust".to_string(),
        tags: vec!["Rust".to_string()],
        keywords: vec![],
        language: "ko".to_string(),
        version: "1.0".to_string(),
        date: Some("2026-11-24".to_string()),
        source_uri: format!("test://{id}"),
        sample_path: None,
        acl_scope,
        content_hash: format!("hash-{id}"),
        body: body.to_string(),
        markdown: None,
        sections: vec![DocumentSection {
            section_id: SectionId::new("s1"),
            heading: "개요".to_string(),
            anchor: "s1".to_string(),
            order: 1,
            line_start: 1,
            line_end: 2,
            body: body.to_string(),
        }],
    }
}

fn search_service() -> Arc<SearchService> {
    let repo = Arc::new(InMemorySearchRepository::new(
        vec![
            sample_doc(
                "pub-1",
                "공개 Rust 가이드",
                "borrow checker 설명",
                AclScope::Public,
            ),
            sample_doc(
                "priv-1",
                "내부 대외비 문서",
                "borrow checker 내부 정책",
                AclScope::Internal("team-a".to_string()),
            ),
        ],
        Vec::new(),
    ));
    Arc::new(SearchService::new(repo.clone(), repo))
}

#[tokio::test]
async fn serves_agent_card_llms_txt_and_a2a_tasks_with_acl() {
    let search = search_service();
    let mcp = Arc::new(McpServer::new(search, Principal::anonymous_public()));
    let app = build_agent_router(mcp, 2, "in-memory-fake");

    let card_resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/.well-known/agent.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(card_resp.status(), StatusCode::OK);
    let card_body = card_resp.into_body().collect().await.unwrap().to_bytes();
    let card: Value = serde_json::from_slice(&card_body).unwrap();
    assert_eq!(card["name"], "dok3search");
    assert_eq!(card["url"], "/a2a");

    let llms_resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/llms.txt")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(llms_resp.status(), StatusCode::OK);
    let llms_text = String::from_utf8(
        llms_resp
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes()
            .to_vec(),
    )
    .unwrap();
    assert!(llms_text.contains("POST /mcp"));
    assert!(llms_text.contains("POST /a2a"));

    let send_req = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tasks/send",
        "params": {
            "id": "task-1",
            "message": {
                "role": "user",
                "parts": [{ "type": "text", "text": "borrow checker" }]
            }
        }
    });
    let send_resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/a2a")
                .header("content-type", "application/json")
                .body(Body::from(send_req.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(send_resp.status(), StatusCode::OK);
    let send_bytes = send_resp.into_body().collect().await.unwrap().to_bytes();
    let send_json: Value = serde_json::from_slice(&send_bytes).unwrap();
    assert_eq!(send_json["result"]["status"]["state"], "completed");
    let hits = send_json["result"]["artifacts"][0]["parts"][0]["data"]["hits"]
        .as_array()
        .unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0]["document_id"], "pub-1");

    let get_req = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tasks/get",
        "params": { "id": "task-1" }
    });
    let get_resp = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/a2a")
                .header("content-type", "application/json")
                .body(Body::from(get_req.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(get_resp.status(), StatusCode::OK);
    let get_bytes = get_resp.into_body().collect().await.unwrap().to_bytes();
    let get_json: Value = serde_json::from_slice(&get_bytes).unwrap();
    assert_eq!(get_json["result"]["id"], "task-1");
}

#[test]
fn acp_adapter_delegates_search_doc_and_section_with_acl() {
    let adapter = AcpAdapter::new(search_service());
    let anon = Principal::anonymous_public();

    let res = adapter
        .handle(
            &anon,
            AcpRequest::Search {
                query: "borrow".to_string(),
                limit: 5,
            },
        )
        .unwrap();
    let AcpResponse::Search(search_res) = res else {
        panic!("expected search response");
    };
    assert_eq!(search_res.total_hits, 1);
    assert_eq!(search_res.hits[0].document_id.as_str(), "pub-1");

    assert!(
        adapter
            .handle(
                &anon,
                AcpRequest::GetDocument {
                    document_id: "priv-1".to_string(),
                    toc_only: false,
                    max_chars: None,
                }
            )
            .is_err()
    );
    let sec = adapter
        .handle(
            &anon,
            AcpRequest::GetSection {
                document_id: "pub-1".to_string(),
                section_id: "s1".to_string(),
            },
        )
        .unwrap();
    assert!(matches!(sec, AcpResponse::Section(_)));
}
