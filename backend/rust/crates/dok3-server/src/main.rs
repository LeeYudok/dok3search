//! dok3search server binary entrypoint.

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
use dok3_search::{InMemorySearchRepository, SearchService};
use tokio::net::TcpListener;

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
    let design_dir = config.design_docs_dir.or_else(|| {
        resolve_first_dir(&[
            "frontend/src/samples/design",
            "../../frontend/src/samples/design",
        ])
    });
    let static_dir = config
        .static_dir
        .or_else(|| resolve_static_dir(&["frontend/dist", "../../frontend/dist"]));

    // --- slice-C: persistent-store ---
    let (service, mode) = match (config.source_dir.as_deref(), config.data_dir.as_deref()) {
        (Some(root), Some(data_dir)) => {
            validate_source_bind(&config.bind)?;
            let (service, report) = persistence::build_persistent(root, data_dir)?;
            print_build_report("persistent Markdown snapshot", &report);
            (service, "tantivy-persistent")
        }
        (Some(root), None) => {
            validate_source_bind(&config.bind)?;
            let (service, report) = local_index::build(root)?;
            print_build_report("local Markdown snapshot", &report);
            (service, "tantivy-local")
        }
        (None, Some(_)) => {
            return Err("DOK3_DATA_DIR requires DOK3_SOURCE_DIR".into());
        }
        (None, None) => {
            let repo = Arc::new(InMemorySearchRepository::from_seed(design_dir.as_deref()));
            (SearchService::new(repo.clone(), repo), "in-memory-fake")
        }
    };
    // --- end slice-C: persistent-store ---
    let doc_count = service.health_ready(mode).document_count;
    // --- slice-B: mcp ---
    let search_arc = Arc::new(service);
    if mcp_stdio {
        eprintln!("dok3search MCP stdio ready (docs={doc_count}, mode={mode})");
        return mcp::serve_stdio(search_arc).await;
    }
    let mut state = ApiState::new(Arc::clone(&search_arc));
    state.mode = mode.into();
    let app = mcp::build_app(state, static_dir.clone());
    // --- /slice-B: mcp ---

    let addr = format!("{}:{}", config.bind, config.port);
    let listener = TcpListener::bind(&addr)
        .await
        .map_err(|err| format!("failed to bind {addr}: {err}"))?;

    let static_label = static_dir
        .as_ref()
        .map_or_else(|| "disabled".to_string(), |p| p.display().to_string());
    println!("dok3search listening on http://{addr} (docs={doc_count}, static={static_label})");

    dok3_api::serve_with_shutdown(listener, app, shutdown_signal())
        .await
        .map_err(|err| format!("server runtime error: {err}"))
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
    if bind.parse::<Ipv4Addr>().is_ok_and(|ip| ip.is_loopback()) {
        return Ok(());
    }
    Err("DOK3_SOURCE_DIR requires an IPv4 loopback DOK3_BIND".into())
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
    use super::validate_source_bind;

    #[test]
    fn source_mode_accepts_only_ipv4_loopback_binds() {
        assert!(validate_source_bind("127.0.0.1").is_ok());
        assert!(validate_source_bind("127.0.0.2").is_ok());
        assert!(validate_source_bind("0.0.0.0").is_err());
        assert!(validate_source_bind("localhost").is_err());
        assert!(validate_source_bind("::1").is_err());
    }
}
