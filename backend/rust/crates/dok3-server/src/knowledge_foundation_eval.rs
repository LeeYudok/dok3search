//! End-to-end Knowledge Foundation coding & codebase comprehension evaluation suite.
//! Simulates an air-gapped LLM agent using ONLY `dok3search` MCP tools (`kb_search`,
//! `kb_search_code`, `kb_search_api`, `kb_search_error`, `kb_get_section`) across
//! Java, JSP, CommonJS, TypeScript, React TSX, Next.js App Router, and `kordoc`.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use dok3_agent::McpServer;
use dok3_domain::Principal;
use serde_json::{Value, json};

use crate::classification::RuleClassifier;
use crate::local_index;

fn samples_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../frontend/src/samples")
        .canonicalize()
        .expect("frontend/src/samples should exist")
}

fn copy_dir_recursive(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).unwrap();
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let target = dst.join(entry.file_name());
        if path.is_dir() {
            copy_dir_recursive(&path, &target);
        } else {
            fs::copy(&path, &target).unwrap();
        }
    }
}

fn call_mcp_tool(mcp: &McpServer, id: u64, name: &str, arguments: Value) -> (Value, usize) {
    let req = json!({
        "jsonrpc": "2.0",
        "id": id,
        "method": "tools/call",
        "params": {
            "name": name,
            "arguments": arguments,
        }
    });
    let resp = mcp
        .handle_message(&req.to_string())
        .expect("MCP response expected");
    assert!(
        resp.error.is_none(),
        "MCP error for {name}: {:?}",
        resp.error
    );
    let result = resp.result.expect("MCP result expected");
    let text_len = result
        .get("content")
        .and_then(Value::as_array)
        .and_then(|arr| arr.first())
        .and_then(|item| item.get("text"))
        .and_then(Value::as_str)
        .map_or(0, str::len);
    let structured = result.get("structuredContent").cloned().unwrap_or_else(|| {
        let text = result["content"][0]["text"].as_str().unwrap_or("{}");
        serde_json::from_str(text).unwrap_or(Value::Null)
    });
    (structured, text_len)
}

fn section_text(sec: &Value) -> &str {
    ["markdown", "body", "content", "text"]
        .iter()
        .find_map(|k| sec[k].as_str())
        .expect("section text field expected")
}

#[test]
fn airgapped_agent_comprehends_polyglot_and_kordoc_codebases_via_mcp_within_token_budget() {
    let tmp = tempfile::tempdir().unwrap();
    let polyglot_src = samples_root().join("polyglot-samples");
    copy_dir_recursive(&polyglot_src, &tmp.path().join("polyglot-samples"));

    // Include representative kordoc modules for real-world TypeScript comprehension.
    let kordoc_dst = tmp.path().join("kordoc/src");
    fs::create_dir_all(&kordoc_dst).unwrap();
    for file in ["utils.ts", "detect.ts"] {
        fs::copy(
            samples_root().join("kordoc/src").join(file),
            kordoc_dst.join(file),
        )
        .unwrap();
    }

    let (service, report) = local_index::build(tmp.path()).unwrap();
    assert!(
        report.indexed >= 12,
        "expected >=12 docs, got {}",
        report.indexed
    );
    let mcp = McpServer::new(Arc::new(service), Principal::anonymous_public());

    // 1. Java API -> JDBC ORA-00904 root-cause & fallback comprehension
    let (api_res, api_len) = call_mcp_tool(
        &mcp,
        1,
        "kb_search_api",
        json!({ "endpoint": "/api/orders", "limit": 5 }),
    );
    let api_hits = api_res["hits"].as_array().unwrap();
    assert!(!api_hits.is_empty(), "expected /api/orders hit");
    assert!(
        api_hits.iter().any(|h| {
            h["title"]
                .as_str()
                .is_some_and(|t| t.contains("OrderController"))
        }),
        "expected OrderController in /api/orders hits: {api_hits:?}"
    );

    let (err_res, err_len) = call_mcp_tool(
        &mcp,
        2,
        "kb_search_error",
        json!({ "error_code": "ORA-00904", "limit": 5 }),
    );
    let err_hit = err_res["hits"]
        .as_array()
        .unwrap()
        .iter()
        .find(|h| {
            h["title"]
                .as_str()
                .is_some_and(|t| t.contains("OrderRepository"))
        })
        .expect("expected OrderRepository in ORA-00904 hits");
    let err_sec_id = err_hit["matched_section_id"]
        .as_str()
        .expect("matched_section_id for ORA-00904");
    let (err_sec, err_sec_len) = call_mcp_tool(
        &mcp,
        3,
        "kb_get_section",
        json!({
            "document_id": err_hit["document_id"],
            "section_id": err_sec_id,
        }),
    );
    let err_md = section_text(&err_sec);
    assert!(err_md.contains("ORA-00904"));
    assert!(err_sec.to_string().contains("line_start"), "{err_sec}");
    assert!(api_len < 6_000 && err_len + err_sec_len < 6_000);

    let (method_res, method_len) = call_mcp_tool(
        &mcp,
        17,
        "kb_search",
        json!({ "query": "SETTLEMENT_FEE_KRW path:OrderRepository", "limit": 5 }),
    );
    let method_hit = &method_res["hits"].as_array().unwrap()[0];
    let (method_sec, method_sec_len) = call_mcp_tool(
        &mcp,
        18,
        "kb_get_section",
        json!({
            "document_id": method_hit["document_id"],
            "section_id": method_hit["matched_section_id"].as_str().unwrap(),
        }),
    );
    assert!(section_text(&method_sec).contains("SETTLEMENT_FEE_KRW"));
    assert!(method_len + method_sec_len < 6_000);

    // 2. JSP legacy screen -> form & JSTL comprehension
    let (jsp_res, jsp_len) = call_mcp_tool(
        &mcp,
        4,
        "kb_search",
        json!({ "query": "ext:jsp orderSearchForm", "limit": 5 }),
    );
    let jsp_hit = &jsp_res["hits"].as_array().unwrap()[0];
    assert!(jsp_hit["title"].as_str().unwrap().contains("orderList.jsp"));
    let (jsp_sec, jsp_sec_len) = call_mcp_tool(
        &mcp,
        5,
        "kb_get_section",
        json!({
            "document_id": jsp_hit["document_id"],
            "section_id": jsp_hit["matched_section_id"].as_str().unwrap(),
        }),
    );
    let jsp_md = section_text(&jsp_sec);
    assert!(jsp_md.contains("/api/orders/search"));
    assert!(jsp_len + jsp_sec_len < 6_000);

    // 3. CommonJS .cjs retry & X-Idempotency-Key comprehension
    let (cjs_res, cjs_len) = call_mcp_tool(
        &mcp,
        6,
        "kb_search_code",
        json!({ "symbol": "fetchOrderSummary", "ext": "cjs", "limit": 5 }),
    );
    let cjs_hit = &cjs_res["hits"].as_array().unwrap()[0];
    assert!(
        cjs_hit["title"]
            .as_str()
            .unwrap()
            .contains("legacyOrderClient.cjs")
    );
    let (cjs_sec, cjs_sec_len) = call_mcp_tool(
        &mcp,
        7,
        "kb_get_section",
        json!({
            "document_id": cjs_hit["document_id"],
            "section_id": cjs_hit["matched_section_id"].as_str().unwrap(),
        }),
    );
    assert!(section_text(&cjs_sec).contains("X-Idempotency-Key"));
    assert!(cjs_len + cjs_sec_len < 6_000);

    // 4. TypeScript domain formula + real kordoc symbol comprehension
    let (ts_res, ts_len) = call_mcp_tool(
        &mcp,
        8,
        "kb_search_code",
        json!({ "symbol": "calculateSettlement", "ext": "ts", "limit": 5 }),
    );
    let ts_hit = &ts_res["hits"].as_array().unwrap()[0];
    let (ts_sec, ts_sec_len) = call_mcp_tool(
        &mcp,
        9,
        "kb_get_section",
        json!({
            "document_id": ts_hit["document_id"],
            "section_id": ts_hit["matched_section_id"].as_str().unwrap(),
        }),
    );
    let ts_md = section_text(&ts_sec);
    assert!(ts_md.contains("Math.floor"));
    assert!(ts_md.contains("feeBps"));
    assert!(ts_len + ts_sec_len < 6_000);

    let (kordoc_res, kordoc_len) = call_mcp_tool(
        &mcp,
        10,
        "kb_search_code",
        json!({ "symbol": "toArrayBuffer", "path": "kordoc", "limit": 5 }),
    );
    let kordoc_hit = &kordoc_res["hits"].as_array().unwrap()[0];
    assert!(
        kordoc_hit["document_id"]
            .as_str()
            .unwrap()
            .contains("utils.ts")
    );
    assert!(kordoc_len < 6_000);

    // 5. React .tsx hook & memo/forwardRef component comprehension
    let (react_res, react_len) = call_mcp_tool(
        &mcp,
        11,
        "kb_search_code",
        json!({ "symbol": "OrderStatusBadge", "ext": "tsx", "limit": 5 }),
    );
    let react_hit = &react_res["hits"].as_array().unwrap()[0];
    let (react_sec, react_sec_len) = call_mcp_tool(
        &mcp,
        12,
        "kb_get_section",
        json!({
            "document_id": react_hit["document_id"],
            "section_id": react_hit["matched_section_id"].as_str().unwrap(),
        }),
    );
    assert!(section_text(&react_sec).contains("aria-live=\"polite\""));
    assert!(react_len + react_sec_len < 6_000);

    // 6. Next.js App Router page.tsx disambiguation & POST Route Handler
    let (orders_page, _) = call_mcp_tool(
        &mcp,
        13,
        "kb_search",
        json!({ "query": "path:orders page.tsx", "limit": 5 }),
    );
    let (users_page, _) = call_mcp_tool(
        &mcp,
        14,
        "kb_search",
        json!({ "query": "path:users page.tsx", "limit": 5 }),
    );
    let orders_title = orders_page["hits"][0]["title"].as_str().unwrap();
    let users_title = users_page["hits"][0]["title"].as_str().unwrap();
    assert_ne!(orders_title, users_title);
    assert!(orders_title.contains("orders/page.tsx"), "{orders_title}");
    assert!(users_title.contains("users/page.tsx"), "{users_title}");

    let (route_res, route_len) = call_mcp_tool(
        &mcp,
        15,
        "kb_search_code",
        json!({ "symbol": "POST", "path": "api/orders", "limit": 5 }),
    );
    let route_hit = &route_res["hits"].as_array().unwrap()[0];
    let (route_sec, route_sec_len) = call_mcp_tool(
        &mcp,
        16,
        "kb_get_section",
        json!({
            "document_id": route_hit["document_id"],
            "section_id": route_hit["matched_section_id"].as_str().unwrap(),
        }),
    );
    assert!(section_text(&route_sec).contains("x-idempotency-key"));
    assert!(route_len + route_sec_len < 6_000);
}

#[test]
fn raw_airgapped_mount_without_manifests_classifies_and_disambiguates_all_stacks() {
    let raw_dir = tempfile::tempdir().unwrap();
    fs::create_dir_all(raw_dir.path().join("security")).unwrap();
    fs::create_dir_all(raw_dir.path().join("design")).unwrap();
    fs::create_dir_all(raw_dir.path().join("app/orders")).unwrap();
    fs::create_dir_all(raw_dir.path().join("app/users")).unwrap();

    fs::copy(
        samples_root().join("polyglot-samples/java/OrderController.java"),
        raw_dir.path().join("security/OrderController.java"),
    )
    .unwrap();
    fs::copy(
        samples_root().join("polyglot-samples/react/OrderStatusBadge.tsx"),
        raw_dir.path().join("design/OrderStatusBadge.tsx"),
    )
    .unwrap();
    fs::copy(
        samples_root().join("polyglot-samples/nextjs/app/orders/page.tsx"),
        raw_dir.path().join("app/orders/page.tsx"),
    )
    .unwrap();
    fs::copy(
        samples_root().join("polyglot-samples/nextjs/app/users/page.tsx"),
        raw_dir.path().join("app/users/page.tsx"),
    )
    .unwrap();

    let (service, report) = local_index::build(raw_dir.path()).unwrap();
    assert_eq!(report.indexed, 4);

    let mcp = McpServer::new(Arc::new(service), Principal::anonymous_public());
    let (res, _) = call_mcp_tool(
        &mcp,
        1,
        "kb_search",
        json!({ "query": "page.tsx", "limit": 5 }),
    );
    let titles: Vec<&str> = res["hits"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|h| h["title"].as_str())
        .collect();
    assert!(titles.contains(&"app/orders/page.tsx"), "{titles:?}");
    assert!(titles.contains(&"app/users/page.tsx"), "{titles:?}");

    // Also verify RuleClassifier never misclassifies code under security/ or design/
    let mut dummy_doc = dok3_domain::Document {
        id: dok3_domain::DocumentId::new("raw:security/OrderController.java"),
        revision_id: dok3_domain::RevisionId::new("v1"),
        source_id: dok3_domain::SourceId::new("raw"),
        source: "raw".to_string(),
        title: "OrderController.java".to_string(),
        summary: String::new(),
        category: "guide".to_string(),
        topic: None,
        tag: String::new(),
        tags: Vec::new(),
        keywords: Vec::new(),
        language: "ko".to_string(),
        version: "v1".to_string(),
        date: None,
        source_uri: "file:///security/OrderController.java".to_string(),
        sample_path: Some("security/OrderController.java".to_string()),
        acl_scope: dok3_domain::AclScope::Public,
        content_hash: String::new(),
        body: "public class OrderController {}".to_string(),
        markdown: None,
        sections: Vec::new(),
    };
    let out = RuleClassifier::classify(&dummy_doc, "");
    assert_eq!(out.category, "code");
    dummy_doc.sample_path = Some("design/OrderStatusBadge.tsx".to_string());
    let out_tsx = RuleClassifier::classify(&dummy_doc, "");
    assert_eq!(out_tsx.category, "code");
    assert!(out_tsx.tags.iter().any(|t| t == "react"));
}
