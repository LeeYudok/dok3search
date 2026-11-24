//! dok3search server binary entrypoint.

mod classification;
mod corpus_meta;
#[cfg(test)]
mod knowledge_foundation_eval;
mod llm_classifier;
mod local_index;
// --- slice-B: mcp ---
mod mcp;
// --- /slice-B: mcp ---
mod persistence;
mod sources;
mod win_service;

use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;

use dok3_analyzer::{AnalyzerConfig, AnalyzerProfile, configure_runtime};
use dok3_api::ApiState;
use dok3_core::ServerConfig;
use dok3_domain::Principal;
use dok3_search::{InMemorySearchRepository, SearchService};
use tokio::net::TcpListener;

use crate::persistence::{PersistentRuntime, WorkerHandle};

fn main() -> ExitCode {
    // The SCM dispatcher blocks the calling thread and builds its own runtime, so it must run
    // before (and outside of) the default tokio runtime. Other modes fall through to `run`.
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = if mcp::parse_cli_args(args) == Ok(mcp::CliMode::WindowsService) {
        win_service::run_as_service()
    } else {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(|err| format!("failed to build tokio runtime: {err}"))
            .and_then(|runtime| runtime.block_on(run()))
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("dok3search server error: {err}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(handled) = maybe_compile_kodic(&args)? {
        eprintln!("{handled}");
        return Ok(());
    }
    // --- slice-B: mcp ---
    let mode = mcp::parse_cli_args(args)?;
    // --- /slice-B: mcp ---
    match mode {
        mcp::CliMode::WindowsService => win_service::run_as_service(),
        mcp::CliMode::McpStdio => {
            run_server_mode(Some(mcp::CliMode::McpStdio), shutdown_signal(), || {}).await
        }
        mcp::CliMode::AcpStdio => {
            run_server_mode(Some(mcp::CliMode::AcpStdio), shutdown_signal(), || {}).await
        }
        mcp::CliMode::Http => run_server_mode(None, shutdown_signal(), || {}).await,
    }
}

/// Runs the server until `shutdown` resolves. `on_ready` is invoked right after the HTTP listener
/// is bound (never in stdio MCP/ACP mode, which has no listener).
#[allow(dead_code)]
pub(crate) async fn run_server<F, R>(
    mcp_stdio: bool,
    shutdown: F,
    on_ready: R,
) -> Result<(), String>
where
    F: std::future::Future<Output = ()> + Send + 'static,
    R: FnOnce(),
{
    run_server_mode(
        mcp_stdio.then_some(mcp::CliMode::McpStdio),
        shutdown,
        on_ready,
    )
    .await
}

async fn run_server_mode<F, R>(
    stdio_mode: Option<mcp::CliMode>,
    shutdown: F,
    on_ready: R,
) -> Result<(), String>
where
    F: std::future::Future<Output = ()> + Send + 'static,
    R: FnOnce(),
{
    let config = ServerConfig::from_env().map_err(|err| err.to_string())?;
    let analyzer_info = configure_analyzer_from_config(&config)?;
    eprintln!(
        "dok3search analyzer ready (profile={}, dict_loaded={}, version={})",
        analyzer_info.profile, analyzer_info.dictionary_loaded, analyzer_info.version_tag
    );
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
    match stdio_mode {
        Some(mcp::CliMode::McpStdio) => {
            eprintln!("dok3search MCP stdio ready (docs={doc_count}, mode={mode})");
            return mcp::serve_stdio(search_arc).await;
        }
        Some(mcp::CliMode::AcpStdio) => {
            eprintln!("dok3search ACP stdio ready (docs={doc_count}, mode={mode})");
            return mcp::serve_acp_stdio(search_arc).await;
        }
        _ => {}
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
    on_ready();

    let serve_result = dok3_api::serve_with_shutdown(listener, app, shutdown)
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

    let classifier = config
        .classifier_endpoint
        .as_deref()
        .map(|ep| llm_classifier::OfflineLlmClassifier::new(ep, config.classifier_model.as_deref()))
        .transpose()?
        .map(Arc::new);

    // --- slice-C: persistent-store ---
    match (config.source_dir.as_deref(), config.data_dir.as_deref()) {
        (Some(root), Some(data_dir)) => {
            validate_source_bind(&config.bind, config.allow_non_loopback_bind)?;
            let (runtime, report) = PersistentRuntime::open_with_options(
                root,
                data_dir,
                config.refresh_interval_secs,
                classifier,
            )?;
            print_build_report("persistent Markdown snapshot", &report);
            let runtime = Arc::new(runtime);
            let service = runtime.service().clone();
            Ok((service, "tantivy-persistent", Some(runtime)))
        }
        (Some(root), None) => {
            validate_source_bind(&config.bind, config.allow_non_loopback_bind)?;
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
            .with_classification(Arc::clone(runtime) as _)
            .with_sources(Arc::clone(runtime) as _);
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

fn validate_source_bind(bind: &str, allow_non_loopback: bool) -> Result<(), String> {
    if allow_non_loopback {
        // Direct, explicit operator opt-in: `DOK3_ALLOW_NON_LOOPBACK_BIND=true`
        // lets `DOK3_SOURCE_DIR` run on a non-loopback bind (container behind an
        // authenticating ingress). The loopback guard stays the default.
        return Ok(());
    }
    validate_loopback_bind(bind, "DOK3_SOURCE_DIR")
}

/// Rejects any non-loopback bind. Used for the admin surface, which never
/// honors `DOK3_ALLOW_NON_LOOPBACK_BIND`.
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

fn maybe_compile_kodic(args: &[String]) -> Result<Option<String>, String> {
    let Some(first) = args.first() else {
        return Ok(None);
    };
    if first != "--compile-kodic" {
        return Ok(None);
    }
    if args.len() != 3 {
        return Err("usage: dok3-server --compile-kodic <mecab-ko-dic-src-dir> <out-bin>".into());
    }
    let src_dir = std::path::Path::new(&args[1]);
    let out_bin = std::path::Path::new(&args[2]);
    let dict = dok3_analyzer::kodic::Dictionary::compile(src_dir).map_err(|err| {
        format!(
            "failed to compile mecab-ko-dic from {}: {err}",
            src_dir.display()
        )
    })?;
    let bytes = dict
        .save(out_bin)
        .map_err(|err| format!("failed to save dictionary to {}: {err}", out_bin.display()))?;
    let info = dict.info();
    Ok(Some(format!(
        "compiled mecab-ko-dic dictionary to {} ({bytes} bytes, entries={}, surfaces={})",
        out_bin.display(),
        info.entries,
        info.surfaces
    )))
}

fn configure_analyzer_from_config(
    config: &ServerConfig,
) -> Result<dok3_analyzer::AnalyzerRuntimeInfo, String> {
    dok3_analyzer::configure_synonyms(config.synonyms_path.as_deref())?;
    let profile = match config.analyzer_profile.as_deref() {
        Some(raw) => raw.parse::<AnalyzerProfile>()?,
        None if config.dict_path.is_some() => AnalyzerProfile::NoriMixed,
        None => AnalyzerProfile::Builtin,
    };
    configure_runtime(&AnalyzerConfig {
        profile,
        dict_path: config.dict_path.clone(),
        user_dict_path: config.user_dict_path.clone(),
    })
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
        IndexJobRecord, IndexJobState, SourceListResponse,
    };
    use http_body_util::BodyExt;
    use serde_json::{Value, json};
    use tower::ServiceExt;

    use super::{build_api_state, open_runtime, validate_source_bind};
    use crate::mcp;

    #[test]
    fn source_mode_accepts_only_ipv4_loopback_binds() {
        assert!(validate_source_bind("127.0.0.1", false).is_ok());
        assert!(validate_source_bind("127.0.0.2", false).is_ok());
        assert!(validate_source_bind("0.0.0.0", false).is_err());
        assert!(validate_source_bind("localhost", false).is_err());
        assert!(validate_source_bind("::1", false).is_err());
    }

    #[test]
    fn non_loopback_override_allows_source_bind_but_not_admin() {
        // Explicit opt-in lifts the source-bind guard so a container can bind
        // 0.0.0.0 while still indexing DOK3_SOURCE_DIR.
        assert!(validate_source_bind("0.0.0.0", true).is_ok());
        assert!(validate_source_bind("127.0.0.1", true).is_ok());

        // The admin surface must never accept a non-loopback bind, even when
        // DOK3_ALLOW_NON_LOOPBACK_BIND is set.
        let admin_with_override = ServerConfig {
            bind: "0.0.0.0".to_string(),
            admin_enabled: true,
            allow_non_loopback_bind: true,
            ..ServerConfig::default()
        };
        assert!(open_runtime(&admin_with_override, None).is_err());
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

        let sources_resp = app
            .clone()
            .oneshot(Request::get("/api/v1/sources").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(sources_resp.status(), StatusCode::OK);
        let sources_body: SourceListResponse =
            serde_json::from_slice(&sources_resp.into_body().collect().await.unwrap().to_bytes())
                .unwrap();
        assert_eq!(sources_body.sources.len(), 1);
        assert_eq!(sources_body.sources[0].source_id, "local-markdown");
        assert!(sources_body.sources[0].managed_by_env);

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

    #[tokio::test]
    async fn health_alias_and_readiness_are_ready() {
        let config = ServerConfig::default();
        let (service, mode, persistent) = open_runtime(&config, None).unwrap();
        let search = Arc::new(service);
        let state = build_api_state(&config, search, mode, persistent.as_ref());
        let app = mcp::build_app(state, None);

        for path in ["/health", "/health/ready"] {
            let resp = app
                .clone()
                .oneshot(Request::get(path).body(Body::empty()).unwrap())
                .await
                .unwrap();
            assert_eq!(resp.status(), StatusCode::OK, "{path} should be 200");
        }
    }
}
