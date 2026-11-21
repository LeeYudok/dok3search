//! dok3search server binary entrypoint.

use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;

use dok3_api::{ApiState, build_router};
use dok3_core::ServerConfig;
use dok3_domain::DocumentStorePort;
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

    let repo = Arc::new(InMemorySearchRepository::from_seed(design_dir.as_deref()));
    let doc_count = repo.document_count();
    let service = Arc::new(SearchService::new(repo.clone(), repo));
    let app = build_router(ApiState::new(service), static_dir.clone());

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
