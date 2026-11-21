use std::io::Cursor;
use std::path::PathBuf;
use std::sync::Arc;

use dok3_agent::{
    DEFAULT_PROTOCOL_VERSION, JSONRPC_INVALID_PARAMS, JSONRPC_INVALID_REQUEST,
    JSONRPC_METHOD_NOT_FOUND, JSONRPC_PARSE_ERROR, JSONRPC_VERSION_MISMATCH,
    MAX_JSONRPC_LINE_BYTES, McpServer,
};
use dok3_domain::{
    AclScope, Document, DocumentSection, Principal, RevisionId, SectionId, SourceId,
};
use dok3_search::{InMemorySearchRepository, SearchService};
use serde_json::{Value, json};

fn make_doc(
    id: &str,
    title: &str,
    category: &str,
    tag: &str,
    scope: AclScope,
    body: &str,
) -> Document {
    Document {
        id: id.into(),
        revision_id: RevisionId::new("rev-1"),
        source_id: SourceId::new("test-src"),
        source: "테스트 출처".to_string(),
        title: title.to_string(),
        summary: format!("{title} 요약"),
        category: category.to_string(),
        tag: tag.to_string(),
        tags: vec![tag.to_string()],
        keywords: vec![tag.to_string()],
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
            heading: "본문 절".to_string(),
            anchor: "sec-1".to_string(),
            order: 1,
            line_start: 1,
            line_end: 3,
            body: body.to_string(),
        }],
    }
}

fn build_fixture_service() -> Arc<SearchService> {
    let long_korean = "가나다라마바사아자차카타파하".repeat(3);
    let docs = vec![
        make_doc(
            "pub-api",
            "REST 검색 API 계약",
            "code",
            "API",
            AclScope::Public,
            "POST /api/v1/search 요청으로 문서를 검색합니다.",
        ),
        make_doc(
            "pub-err",
            "오라클 ORA-00904 오류 조치 가이드",
            "manual",
            "오류",
            AclScope::Public,
            "ORA-00904 부적합한 식별자 오류 시 컬럼명을 대조합니다.",
        ),
        make_doc(
            "pub-code",
            "Tokio tokio::spawn 및 SearchService 예제",
            "guide",
            "Rust",
            AclScope::Public,
            &long_korean,
        ),
        make_doc(
            "sec-runbook",
            "대외비 장애 복구 절차 ORA-00904",
            "policy",
            "보안",
            AclScope::internal("security"),
            "내부 전용 장애 복구 매뉴얼입니다.",
        ),
    ];
    let repo = Arc::new(InMemorySearchRepository::new(docs, vec![]));
    Arc::new(SearchService::new(repo.clone(), repo))
}

#[test]
fn supports_initialize_2025_and_discover_2026_stateless() {
    let server = McpServer::new(build_fixture_service(), Principal::anonymous_public());

    let init_2025 = server
        .handle_message(
            r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25","clientInfo":{"name":"claude-code"}}}"#,
        )
        .expect("initialize returns response");
    let init_res = init_2025.result.unwrap();
    assert_eq!(init_res["protocolVersion"], "2025-11-25");

    let init_unknown = server
        .handle_message(
            r#"{"jsonrpc":"2.0","id":2,"method":"initialize","params":{"protocolVersion":"2099-01-01"}}"#,
        )
        .unwrap()
        .result
        .unwrap();
    assert_eq!(init_unknown["protocolVersion"], DEFAULT_PROTOCOL_VERSION);

    assert!(
        server
            .handle_message(r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#)
            .is_none()
    );

    let discover = server
        .handle_message(
            r#"{"jsonrpc":"2.0","id":"disc-1","method":"server/discover","params":{"_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28"}}}"#,
        )
        .unwrap()
        .result
        .unwrap();
    assert_eq!(discover["resultType"], "complete");
    assert!(
        discover["supportedVersions"]
            .as_array()
            .unwrap()
            .contains(&json!("2026-07-28"))
    );
    assert_eq!(discover["protocolVersion"], "2026-07-28");
    assert_eq!(discover["transport"]["stateless"], true);
    let discover_tools = discover["tools"].as_array().unwrap();
    assert_eq!(discover_tools.len(), 7);

    let bad_meta_ver = server
        .handle_message(
            r#"{"jsonrpc":"2.0","id":3,"method":"tools/list","params":{"_meta":{"io.modelcontextprotocol/protocolVersion":"1999-01-01"}}}"#,
        )
        .unwrap();
    let bad_err = bad_meta_ver.error.unwrap();
    assert_eq!(bad_err.code, JSONRPC_VERSION_MISMATCH);
    let data = bad_err.data.unwrap();
    assert_eq!(data["requested"], "1999-01-01");
    assert!(
        data["supported"]
            .as_array()
            .unwrap()
            .contains(&json!("2026-07-28"))
    );
}

#[test]
fn exposes_all_seven_ssot_tools_and_enforces_acl_on_every_tool() {
    let svc = build_fixture_service();
    let public_server = McpServer::new(svc.clone(), Principal::anonymous_public());
    let sec_server = McpServer::new(
        svc,
        Principal {
            subject: "sec-user".to_string(),
            allowed_scopes: vec![AclScope::Public, AclScope::internal("security")],
            is_admin: false,
        },
    );

    let tools_list = public_server
        .handle_message(r#"{"jsonrpc":"2.0","id":10,"method":"tools/list"}"#)
        .unwrap()
        .result
        .unwrap();
    let names: Vec<&str> = tools_list["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["name"].as_str().unwrap())
        .collect();
    assert_eq!(
        names,
        vec![
            "kb_search",
            "kb_get_document",
            "kb_get_section",
            "kb_search_api",
            "kb_search_error",
            "kb_search_code",
            "kb_list_categories",
        ]
    );

    // kb_search_error finds public ORA-00904 doc while hiding internal policy doc.
    let err_search = public_server
        .handle_message(
            r#"{"jsonrpc":"2.0","id":11,"method":"tools/call","params":{"name":"kb_search_error","arguments":{"error_code":"ORA-00904"}}}"#,
        )
        .unwrap()
        .result
        .unwrap();
    assert_eq!(err_search["isError"], false);
    assert_eq!(err_search["structuredContent"]["total_hits"], 1);
    assert_eq!(
        err_search["structuredContent"]["hits"][0]["document_id"],
        "pub-err"
    );

    // kb_search_api & kb_search_code return matching hits.
    let api_search = public_server
        .handle_message(
            r#"{"jsonrpc":"2.0","id":12,"method":"tools/call","params":{"name":"kb_search_api","arguments":{"endpoint":"/api/v1/search"}}}"#,
        )
        .unwrap()
        .result
        .unwrap();
    assert_eq!(
        api_search["structuredContent"]["hits"][0]["document_id"],
        "pub-api"
    );

    let code_search = public_server
        .handle_message(
            r#"{"jsonrpc":"2.0","id":13,"method":"tools/call","params":{"name":"kb_search_code","arguments":{"symbol":"tokio::spawn"}}}"#,
        )
        .unwrap()
        .result
        .unwrap();
    assert_eq!(
        code_search["structuredContent"]["hits"][0]["document_id"],
        "pub-code"
    );

    // kb_search advisory requested_scope cannot escalate ACLs.
    let escalation = public_server
        .handle_message(
            r#"{"jsonrpc":"2.0","id":14,"method":"tools/call","params":{"name":"kb_search","arguments":{"query":"ORA-00904","requested_scope":"internal:security"}}}"#,
        )
        .unwrap()
        .result
        .unwrap();
    assert_eq!(escalation["structuredContent"]["total_hits"], 0);

    // kb_list_categories hides internal-only policy counts from anonymous_public.
    let pub_cats = public_server
        .handle_message(
            r#"{"jsonrpc":"2.0","id":15,"method":"tools/call","params":{"name":"kb_list_categories"}}"#,
        )
        .unwrap()
        .result
        .unwrap();
    assert_eq!(pub_cats["structuredContent"]["total_documents"], 3);
    let pub_policy = pub_cats["structuredContent"]["categories"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["id"] == "policy")
        .unwrap();
    assert_eq!(pub_policy["count"], 0);

    let sec_cats = sec_server
        .handle_message(
            r#"{"jsonrpc":"2.0","id":16,"method":"tools/call","params":{"name":"kb_list_categories"}}"#,
        )
        .unwrap()
        .result
        .unwrap();
    assert_eq!(sec_cats["structuredContent"]["total_documents"], 4);

    // Direct kb_get_document & kb_get_section deny unauthorized scope cleanly with FORBIDDEN.
    let forbidden_doc = public_server
        .handle_message(
            r#"{"jsonrpc":"2.0","id":17,"method":"tools/call","params":{"name":"kb_get_document","arguments":{"document_id":"sec-runbook"}}}"#,
        )
        .unwrap()
        .result
        .unwrap();
    assert_eq!(forbidden_doc["isError"], true);
    assert_eq!(
        forbidden_doc["structuredContent"]["error"]["code"],
        "FORBIDDEN"
    );

    let forbidden_sec = public_server
        .handle_message(
            r#"{"jsonrpc":"2.0","id":18,"method":"tools/call","params":{"name":"kb_get_section","arguments":{"document_id":"sec-runbook","section_id":"sec-1"}}}"#,
        )
        .unwrap()
        .result
        .unwrap();
    assert_eq!(forbidden_sec["isError"], true);
    assert_eq!(
        forbidden_sec["structuredContent"]["error"]["code"],
        "FORBIDDEN"
    );

    // Authorized caller reads both document and paginated Korean section cleanly.
    let allowed_doc = sec_server
        .handle_message(
            r#"{"jsonrpc":"2.0","id":19,"method":"tools/call","params":{"name":"kb_get_document","arguments":{"document_id":"sec-runbook"}}}"#,
        )
        .unwrap()
        .result
        .unwrap();
    assert_eq!(allowed_doc["isError"], false);
    assert_eq!(allowed_doc["structuredContent"]["id"], "sec-runbook");

    let page1 = public_server
        .handle_message(
            r#"{"jsonrpc":"2.0","id":20,"method":"tools/call","params":{"name":"kb_get_section","arguments":{"document_id":"pub-code","section_id":"sec-1","max_chars":14}}}"#,
        )
        .unwrap()
        .result
        .unwrap();
    assert_eq!(
        page1["structuredContent"]["text"],
        "가나다라마바사아자차카타파하"
    );
    assert_eq!(page1["structuredContent"]["truncated"], true);
    assert_eq!(page1["structuredContent"]["next_cursor"], 14);
}

#[test]
fn rejects_malformed_requests_unknown_methods_and_oversized_stdio_lines() {
    let server = McpServer::new(build_fixture_service(), Principal::anonymous_public());

    assert_eq!(
        server
            .handle_message("{not json")
            .unwrap()
            .error
            .unwrap()
            .code,
        JSONRPC_PARSE_ERROR
    );
    assert_eq!(
        server
            .handle_message(r#"{"jsonrpc":"1.0","id":1,"method":"ping"}"#)
            .unwrap()
            .error
            .unwrap()
            .code,
        JSONRPC_INVALID_REQUEST
    );
    assert_eq!(
        server
            .handle_message(r#"{"jsonrpc":"2.0","id":2,"method":"no_such_method"}"#)
            .unwrap()
            .error
            .unwrap()
            .code,
        JSONRPC_METHOD_NOT_FOUND
    );
    assert_eq!(
        server
            .handle_message(
                r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"kb_get_document","arguments":{}}}"#,
            )
            .unwrap()
            .error
            .unwrap()
            .code,
        JSONRPC_INVALID_PARAMS
    );

    // Oversized line on stdio returns INVALID_REQUEST and recovers on subsequent valid ping.
    let huge = format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":4,\"method\":\"ping\",\"pad\":\"{}\"}}\n{{\"jsonrpc\":\"2.0\",\"id\":5,\"method\":\"ping\"}}\n",
        "x".repeat(MAX_JSONRPC_LINE_BYTES + 64)
    );
    let mut out = Vec::new();
    server
        .serve_reader_writer(Cursor::new(huge.into_bytes()), &mut out)
        .unwrap();
    let stdout = String::from_utf8(out).unwrap();
    let lines: Vec<Value> = stdout
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0]["error"]["code"], json!(JSONRPC_INVALID_REQUEST));
    assert_eq!(lines[1]["id"], 5);
    assert_eq!(lines[1]["result"], json!({}));
}

#[test]
fn serves_seed_design_dataset_over_stdio_roundtrip() {
    let design_dir =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../frontend/src/samples/design");
    assert!(design_dir.is_dir());
    let repo = Arc::new(InMemorySearchRepository::from_seed(Some(&design_dir)));
    let service = Arc::new(SearchService::new(repo.clone(), repo));
    let server = McpServer::new(service, Principal::anonymous_public());

    let session = [
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25","clientInfo":{"name":"codex"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"kb_search","arguments":{"query":"Tantivy","limit":3}}}"#,
        r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"kb_get_document","arguments":{"document_id":"design-index-md"}}}"#,
    ]
    .join("\n")
        + "\n";

    let mut out = Vec::new();
    server
        .serve_reader_writer(Cursor::new(session.into_bytes()), &mut out)
        .unwrap();
    let responses: Vec<Value> = String::from_utf8(out)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    assert_eq!(responses.len(), 3);
    assert!(
        responses[1]["result"]["structuredContent"]["total_hits"]
            .as_u64()
            .unwrap()
            >= 1
    );
    assert_eq!(
        responses[2]["result"]["structuredContent"]["id"],
        "design-index-md"
    );
}
