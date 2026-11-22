//! dok3search server binary entrypoint.

mod classification;
mod local_index;
// --- slice-B: mcp ---
mod mcp;
// --- /slice-B: mcp ---
mod persistence;

use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;

use dok3_api::ApiState;
use dok3_core::ServerConfig;
use dok3_domain::Principal;
use dok3_search::{InMemorySearchRepository, SearchService};
use tokio::net::TcpListener;

use crate::persistence::{PersistentRuntime, WorkerHandle};

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("dok3search server error: {err}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), String> {
    // --- slice-B: mcp ---
    let mcp_stdio = mcp::stdio_requested(std::env::args().skip(1))?;
    // --- /slice-B: mcp ---
    let config = ServerConfig::from_env().map_err(|err| err.to_string())?;
    let design_dir = config.design_docs_dir.clone().or_else(|| {
        resolve_first_dir(&[
            "frontend/src/samples/design",
            "../../frontend/src/samples/design",
        ])
    });
    let static_dir = config
        .static_dir
        .clone()
        .or_else(|| resolve_static_dir(&["frontend/dist", "../../frontend/dist"]));

    let (service, mode, persistent_runtime) = open_runtime(&config, design_dir)?;
    let doc_count = service.health_ready(mode).document_count;
    // --- slice-B: mcp ---
    let search_arc = Arc::new(service);
    if mcp_stdio {
        eprintln!("dok3search MCP stdio ready (docs={doc_count}, mode={mode})");
        return mcp::serve_stdio(search_arc).await;
    }
    let state = build_api_state(
        &config,
        Arc::clone(&search_arc),
        mode,
        persistent_runtime.as_ref(),
    );
    let app = mcp::build_app(state, static_dir.clone());
    // --- /slice-B: mcp ---

    let worker: Option<WorkerHandle> =
        match (persistent_runtime.as_ref(), config.refresh_interval_secs) {
            (Some(runtime), Some(interval_secs)) => {
                Some(runtime.spawn_periodic_worker(interval_secs))
            }
            _ => None,
        };

    let addr = format!("{}:{}", config.bind, config.port);
    let listener = TcpListener::bind(&addr)
        .await
        .map_err(|err| format!("failed to bind {addr}: {err}"))?;

    let static_label = static_dir
        .as_ref()
        .map_or_else(|| "disabled".to_string(), |p| p.display().to_string());
    println!("dok3search listening on http://{addr} (docs={doc_count}, static={static_label})");

    let serve_result = dok3_api::serve_with_shutdown(listener, app, shutdown_signal())
        .await
        .map_err(|err| format!("server runtime error: {err}"));
    if let Some(handle) = worker {
        handle.stop();
    }
    serve_result
}

fn open_runtime(
    config: &ServerConfig,
    design_dir: Option<PathBuf>,
) -> Result<(SearchService, &'static str, Option<Arc<PersistentRuntime>>), String> {
    if config.admin_enabled {
        validate_loopback_bind(&config.bind, "DOK3_ENABLE_ADMIN")?;
    }
    if config.refresh_interval_secs.is_some()
        && (config.source_dir.is_none() || config.data_dir.is_none())
    {
        return Err(
            "DOK3_REFRESH_INTERVAL_SECS requires both DOK3_SOURCE_DIR and DOK3_DATA_DIR".into(),
        );
    }

    // --- slice-C: persistent-store ---
    match (config.source_dir.as_deref(), config.data_dir.as_deref()) {
        (Some(root), Some(data_dir)) => {
            validate_source_bind(&config.bind)?;
            let (runtime, report) =
                PersistentRuntime::open_with_refresh(root, data_dir, config.refresh_interval_secs)?;
            print_build_report("persistent Markdown snapshot", &report);
            let runtime = Arc::new(runtime);
            let service = runtime.service().clone();
            Ok((service, "tantivy-persistent", Some(runtime)))
        }
        (Some(root), None) => {
            validate_source_bind(&config.bind)?;
            let (service, report) = local_index::build(root)?;
            print_build_report("local Markdown snapshot", &report);
            Ok((service, "tantivy-local", None))
        }
        (None, Some(_)) => Err("DOK3_DATA_DIR requires DOK3_SOURCE_DIR".into()),
        (None, None) => {
            let repo = Arc::new(InMemorySearchRepository::from_seed(design_dir.as_deref()));
            Ok((
                SearchService::new(repo.clone(), repo),
                "in-memory-fake",
                None,
            ))
        }
    }
    // --- end slice-C: persistent-store ---
}

fn build_api_state(
    config: &ServerConfig,
    search: Arc<SearchService>,
    mode: &str,
    persistent_runtime: Option<&Arc<PersistentRuntime>>,
) -> ApiState {
    let mut state = ApiState::new(search);
    if config.admin_enabled {
        state = state.with_principal(Principal::local_admin());
    }
    if let Some(runtime) = persistent_runtime {
        state = state
            .with_index_jobs(Arc::clone(runtime) as _)
            .with_classification(Arc::clone(runtime) as _);
    }
    state.mode = mode.into();
    state
}

fn print_build_report(label: &str, report: &local_index::BuildReport) {
    // Emit diagnostics to stderr so `--mcp-stdio` keeps stdout a clean JSON-RPC stream.
    eprintln!(
        "{label}: indexed={} skipped={}",
        report.indexed,
        report.skipped.len()
    );
    for skipped in &report.skipped {
        eprintln!("local Markdown skipped: {skipped}");
    }
}

fn validate_source_bind(bind: &str) -> Result<(), String> {
    validate_loopback_bind(bind, "DOK3_SOURCE_DIR")
}

fn validate_loopback_bind(bind: &str, feature: &str) -> Result<(), String> {
    if bind.parse::<Ipv4Addr>().is_ok_and(|ip| ip.is_loopback()) {
        return Ok(());
    }
    Err(format!("{feature} requires an IPv4 loopback DOK3_BIND"))
}

fn resolve_first_dir(candidates: &[&str]) -> Option<PathBuf> {
    candidates
        .iter()
        .map(PathBuf::from)
        .find(|path| path.is_dir())
}

fn resolve_static_dir(candidates: &[&str]) -> Option<PathBuf> {
    candidates
        .iter()
        .map(PathBuf::from)
        .find(|path| path.join("index.html").is_file())
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::sync::Arc;

    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode, header};
    use dok3_core::ServerConfig;
    use dok3_domain::{
        ClassificationSource, DocumentClassification, IndexJobListResponse, IndexJobMode,
        IndexJobRecord, IndexJobState,
    };
    use http_body_util::BodyExt;
    use serde_json::{Value, json};
    use tower::ServiceExt;

    use super::{build_api_state, open_runtime, validate_source_bind};
    use crate::mcp;

    #[test]
    fn source_mode_accepts_only_ipv4_loopback_binds() {
        assert!(validate_source_bind("127.0.0.1").is_ok());
        assert!(validate_source_bind("127.0.0.2").is_ok());
        assert!(validate_source_bind("0.0.0.0").is_err());
        assert!(validate_source_bind("localhost").is_err());
        assert!(validate_source_bind("::1").is_err());
    }

    #[test]
    fn rejects_admin_on_non_loopback_or_refresh_without_persistent_dirs() {
        let admin_non_loopback = ServerConfig {
            bind: "0.0.0.0".to_string(),
            admin_enabled: true,
            ..ServerConfig::default()
        };
        assert!(open_runtime(&admin_non_loopback, None).is_err());

        let refresh_no_dirs = ServerConfig {
            refresh_interval_secs: Some(15),
            ..ServerConfig::default()
        };
        assert!(open_runtime(&refresh_no_dirs, None).is_err());

        let source_root = tempfile::tempdir().unwrap();
        let refresh_local_only = ServerConfig {
            source_dir: Some(source_root.path().to_path_buf()),
            refresh_interval_secs: Some(15),
            ..ServerConfig::default()
        };
        assert!(open_runtime(&refresh_local_only, None).is_err());
    }

    #[tokio::test]
    async fn admin_http_jobs_work_while_mcp_stays_anonymous_public() {
        let source_root = tempfile::tempdir().unwrap();
        let data_root = tempfile::tempdir().unwrap();
        fs::write(
            source_root.path().join("guide-a.md"),
            "# Admin Wiring\n\nadminmcpwiringtoken body\n",
        )
        .unwrap();

        let config = ServerConfig {
            source_dir: Some(source_root.path().to_path_buf()),
            data_dir: Some(data_root.path().to_path_buf()),
            refresh_interval_secs: Some(30),
            admin_enabled: true,
            ..ServerConfig::default()
        };

        let (service, mode, persistent) = open_runtime(&config, None).unwrap();
        let search_arc = Arc::new(service);
        let state = build_api_state(&config, Arc::clone(&search_arc), mode, persistent.as_ref());
        assert!(state.default_principal.is_admin);
        let app = mcp::build_app(state, None);

        let list_resp = app
            .clone()
            .oneshot(
                Request::get("/api/v1/index/jobs")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(list_resp.status(), StatusCode::OK);
        let list: IndexJobListResponse =
            serde_json::from_slice(&list_resp.into_body().collect().await.unwrap().to_bytes())
                .unwrap();
        assert_eq!(list.jobs.len(), 1);
        assert_eq!(list.jobs[0].trigger, "startup");

        let trigger_resp = app
            .clone()
            .oneshot(
                Request::post("/api/v1/index/jobs")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(r#"{"mode":"full"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(trigger_resp.status(), StatusCode::OK);
        let record: IndexJobRecord =
            serde_json::from_slice(&trigger_resp.into_body().collect().await.unwrap().to_bytes())
                .unwrap();
        assert_eq!(record.mode, IndexJobMode::Full);
        assert_eq!(record.state, IndexJobState::Succeeded);
        assert!(record.rebuilt);

        let cls_resp = app
            .clone()
            .oneshot(
                Request::patch("/api/v1/documents/local-markdown:guide-a.md/classification")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        r#"{"category":"manual","summary":"수동 분류 요약"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(cls_resp.status(), StatusCode::OK);
        let cls_body: DocumentClassification =
            serde_json::from_slice(&cls_resp.into_body().collect().await.unwrap().to_bytes())
                .unwrap();
        assert_eq!(cls_body.source, ClassificationSource::Manual);
        assert!(cls_body.manual_locked);
        assert_eq!(cls_body.category, "manual");
        assert_eq!(cls_body.summary, "수동 분류 요약");

        // Verify /mcp remains anonymous_public (not escalated to admin) and searches live docs.
        let mcp_req = Request::builder()
            .method(Method::POST)
            .uri("/mcp")
            .header(header::CONTENT_TYPE, "application/json")
            .header("MCP-Protocol-Version", "2026-07-28")
            .body(Body::from(
                json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "method": "tools/call",
                    "params": {
                        "name": "kb_search",
                        "arguments": { "query": "adminmcpwiringtoken" }
                    }
                })
                .to_string(),
            ))
            .unwrap();
        let mcp_resp = app.oneshot(mcp_req).await.unwrap();
        assert_eq!(mcp_resp.status(), StatusCode::OK);
        let mcp_body: Value =
            serde_json::from_slice(&mcp_resp.into_body().collect().await.unwrap().to_bytes())
                .unwrap();
        let text = mcp_body["result"]["content"][0]["text"].as_str().unwrap();
        assert!(text.contains("local-markdown:guide-a.md"));
    }
}
