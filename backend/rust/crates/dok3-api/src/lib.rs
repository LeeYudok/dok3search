//! Axum HTTP router, security headers, API error mapping, and static SPA hosting.

use std::future::Future;
use std::path::PathBuf;
use std::sync::Arc;

use axum::Json;
use axum::body::Bytes;
use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, HeaderValue, Method, Request, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{any, delete, get, post};
use dok3_core::Dok3Error;
use dok3_domain::{
    ClassificationPort, CreateSourceRequest, DocumentId, IndexJobPort, Principal, SearchRequest,
    SectionId, SectionReadRequest, SourceRegistryPort, TriggerIndexJobRequest,
    UpdateClassificationRequest,
};
use dok3_search::SearchService;
use serde::{Deserialize, Serialize};
use tower::ServiceExt;
use tower_http::services::{ServeDir, ServeFile};

/// Re-exported so callers can compose extra routes without a direct `axum` dependency.
pub use axum::Router;

const CSP_POLICY: &str = "default-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; font-src 'self'; connect-src 'self'; object-src 'none'; frame-ancestors 'none'";

/// Shared application state injected into Axum route handlers.
#[derive(Clone)]
pub struct ApiState {
    pub search: Arc<SearchService>,
    pub default_principal: Principal,
    pub mode: String,
    pub index_jobs: Option<Arc<dyn IndexJobPort>>,
    pub classification: Option<Arc<dyn ClassificationPort>>,
    pub sources: Option<Arc<dyn SourceRegistryPort>>,
}

impl ApiState {
    /// Creates an `ApiState` bound to the anonymous public principal (`in-memory-fake` mode).
    #[must_use]
    pub fn new(search: Arc<SearchService>) -> Self {
        Self {
            search,
            default_principal: Principal::anonymous_public(),
            mode: "in-memory-fake".to_string(),
            index_jobs: None,
            classification: None,
            sources: None,
        }
    }

    /// Overrides the caller principal (primarily for testing or future auth middleware).
    #[must_use]
    pub fn with_principal(mut self, principal: Principal) -> Self {
        self.default_principal = principal;
        self
    }

    /// Attaches a durable indexing job port for `/api/v1/index/jobs*` endpoints.
    #[must_use]
    pub fn with_index_jobs(mut self, index_jobs: Arc<dyn IndexJobPort>) -> Self {
        self.index_jobs = Some(index_jobs);
        self
    }

    /// Attaches a classification port for `/api/v1/documents/{id}/classification` endpoints.
    #[must_use]
    pub fn with_classification(mut self, classification: Arc<dyn ClassificationPort>) -> Self {
        self.classification = Some(classification);
        self
    }

    /// Attaches a source registry port for `/api/v1/sources*` endpoints.
    #[must_use]
    pub fn with_sources(mut self, sources: Arc<dyn SourceRegistryPort>) -> Self {
        self.sources = Some(sources);
        self
    }
}

/// Builds the Axum router with `/health` (alias of `/health/ready`), `/health/ready`,
/// `/api/v1/*`, JSON `/api` 404 fallback, security headers, and optional static SPA hosting.
pub fn build_router(state: ApiState, static_dir: Option<PathBuf>) -> Router {
    build_router_with_extra(state, static_dir, None)
}

/// Builds the same router as [`build_router`] and merges `extra` routes (e.g. MCP `/mcp`)
/// before the SPA fallback and security-header layer, so extra routes win over the SPA
/// fallback and still receive the common security headers.
///
/// `extra` must not define its own fallback or overlap existing routes; Axum panics on either.
pub fn build_router_with_extra(
    state: ApiState,
    static_dir: Option<PathBuf>,
    extra: Option<Router>,
) -> Router {
    let admin_router = Router::new()
        .route(
            "/api/v1/index/jobs",
            get(list_index_jobs).post(trigger_index_job),
        )
        .route("/api/v1/index/jobs/{job_id}", get(get_index_job))
        .route("/api/v1/sources", get(list_sources).post(create_source))
        .route("/api/v1/sources/{id}", delete(delete_source))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            require_admin_middleware,
        ));

    let doc_classification_router = Router::new()
        .route(
            "/api/v1/documents/{id}/classification",
            get(get_document_classification).patch(update_document_classification),
        )
        .layer(middleware::from_fn_with_state(
            state.clone(),
            require_admin_for_mutations_middleware,
        ));

    let mut router = Router::new()
        // `/health` is a thin alias of the readiness probe so liveness/readiness
        // checks and the frontend dev proxy can share one endpoint.
        .route("/health", get(health_ready))
        .route("/health/ready", get(health_ready))
        .route("/api/v1/search", post(search))
        .route("/api/v1/documents/{id}", get(get_document))
        .route("/api/v1/documents/{id}/sections/{sid}", get(get_section))
        .merge(admin_router)
        .merge(doc_classification_router)
        .route("/api", any(api_not_found))
        .route("/api/{*rest}", any(api_not_found))
        .with_state(state);

    if let Some(extra) = extra {
        router = router.merge(extra);
    }

    let router = match static_dir {
        Some(dir) if dir.join("index.html").is_file() => {
            let index = dir.join("index.html");
            router.fallback(move |req: Request<axum::body::Body>| {
                let dir = dir.clone();
                let index = index.clone();
                async move { spa_fallback(dir, index, req).await }
            })
        }
        _ => router,
    };

    router.layer(middleware::from_fn(apply_security_headers))
}

async fn spa_fallback(dir: PathBuf, index: PathBuf, req: Request<axum::body::Body>) -> Response {
    let uri_path = req.uri().path().to_string();
    if let Ok(resp) = ServeDir::new(dir).oneshot(req).await
        && resp.status() != StatusCode::NOT_FOUND
    {
        return resp.into_response();
    }
    // `/docs/<encoded-doc-id>` is a client-side route: ids such as
    // `local-markdown:samples/.../rust-version.md` end in `.md` even after
    // percent-encoding those characters, so the dotted-segment 404 rule must
    // not fire for them — the SPA serves those paths from index.html.
    if !uri_path.starts_with("/docs/")
        && uri_path
            .rsplit('/')
            .next()
            .is_some_and(|seg| seg.contains('.'))
    {
        return StatusCode::NOT_FOUND.into_response();
    }
    ServeFile::new(index)
        .oneshot(Request::new(axum::body::Body::empty()))
        .await
        .map(IntoResponse::into_response)
        .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())
}

/// Serves the Axum router on `listener` until `shutdown` resolves.
pub async fn serve_with_shutdown<F>(
    listener: tokio::net::TcpListener,
    app: Router,
    shutdown: F,
) -> std::io::Result<()>
where
    F: Future<Output = ()> + Send + 'static,
{
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown)
        .await
}

async fn apply_security_headers(request: Request<axum::body::Body>, next: Next) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    headers.insert(header::X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));
    headers.insert(
        header::REFERRER_POLICY,
        HeaderValue::from_static("no-referrer"),
    );
    headers.insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static(CSP_POLICY),
    );
    response
}

#[derive(Debug, Serialize)]
struct ApiErrorEnvelope {
    error: ApiErrorBody,
}

#[derive(Debug, Serialize)]
struct ApiErrorBody {
    code: &'static str,
    message: String,
}

struct ApiError(Dok3Error);

impl From<Dok3Error> for ApiError {
    fn from(err: Dok3Error) -> Self {
        Self(err)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self.0 {
            Dok3Error::InvalidInput(_) => StatusCode::BAD_REQUEST,
            Dok3Error::Forbidden(_) => StatusCode::FORBIDDEN,
            Dok3Error::NotFound(_) => StatusCode::NOT_FOUND,
            Dok3Error::Parse(_)
            | Dok3Error::Index(_)
            | Dok3Error::Storage(_)
            | Dok3Error::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let body = ApiErrorEnvelope {
            error: ApiErrorBody {
                code: self.0.code(),
                message: self.0.to_string(),
            },
        };
        (status, Json(body)).into_response()
    }
}

async fn api_not_found() -> Response {
    ApiError(Dok3Error::NotFound("api route not found".to_string())).into_response()
}

async fn health_ready(State(state): State<ApiState>) -> impl IntoResponse {
    Json(state.search.health_ready(state.mode))
}

async fn search(
    State(state): State<ApiState>,
    payload: Result<Json<SearchRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Json(req) = payload.map_err(|err| ApiError(Dok3Error::InvalidInput(err.body_text())))?;
    let response = state.search.search(&state.default_principal, &req)?;
    Ok(Json(response))
}

async fn get_document(
    State(state): State<ApiState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let detail = state
        .search
        .get_document(&state.default_principal, &DocumentId::new(id))?;
    Ok(Json(detail))
}

#[derive(Debug, Deserialize)]
struct SectionQueryParams {
    cursor: Option<usize>,
    max_chars: Option<usize>,
}

async fn get_section(
    State(state): State<ApiState>,
    Path((id, sid)): Path<(String, String)>,
    query: Result<Query<SectionQueryParams>, QueryRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Query(query) = query.map_err(|err| ApiError(Dok3Error::InvalidInput(err.body_text())))?;
    let mut req = SectionReadRequest {
        document_id: DocumentId::new(id),
        section_id: SectionId::new(sid),
        cursor: query.cursor,
        max_chars: dok3_domain::DEFAULT_SECTION_MAX_CHARS,
    };
    if let Some(max_chars) = query.max_chars {
        req.max_chars = max_chars;
    }
    let response = state.search.get_section(&state.default_principal, &req)?;
    Ok(Json(response))
}

async fn require_admin_middleware(
    State(state): State<ApiState>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    if let Err(err) = check_admin_guards(&state.default_principal, &request) {
        return err.into_response();
    }
    next.run(request).await
}

async fn require_admin_for_mutations_middleware(
    State(state): State<ApiState>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    if !matches!(*request.method(), Method::GET | Method::HEAD)
        && let Err(err) = check_admin_guards(&state.default_principal, &request)
    {
        return err.into_response();
    }
    next.run(request).await
}

fn check_admin_guards(
    principal: &Principal,
    request: &Request<axum::body::Body>,
) -> Result<(), ApiError> {
    require_admin(principal)?;
    validate_admin_origin(request.headers())?;
    if is_mutating_method(request.method()) {
        require_json_content_type(request.headers())?;
    }
    Ok(())
}

fn require_admin(principal: &Principal) -> Result<(), ApiError> {
    if !principal.is_admin {
        return Err(Dok3Error::Forbidden(format!(
            "principal '{}' does not have admin privileges",
            principal.subject
        ))
        .into());
    }
    Ok(())
}

fn validate_admin_origin(headers: &HeaderMap) -> Result<(), ApiError> {
    if let Some(fetch_site) = headers.get("sec-fetch-site") {
        let Ok(site) = fetch_site.to_str().map(str::trim) else {
            return Err(Dok3Error::Forbidden("invalid Sec-Fetch-Site header".to_string()).into());
        };
        if site.eq_ignore_ascii_case("cross-site") {
            return Err(Dok3Error::Forbidden(
                "cross-site admin requests are forbidden".to_string(),
            )
            .into());
        }
    }
    if let Some(raw_origin) = headers.get(header::ORIGIN) {
        let Ok(origin) = raw_origin.to_str().map(str::trim) else {
            return Err(Dok3Error::Forbidden("invalid Origin header".to_string()).into());
        };
        if !is_allowed_loopback_origin(origin) {
            return Err(Dok3Error::Forbidden(format!(
                "origin '{origin}' is rejected by admin loopback guard"
            ))
            .into());
        }
    }
    Ok(())
}

fn is_allowed_loopback_origin(origin: &str) -> bool {
    let rest = origin
        .strip_prefix("http://")
        .or_else(|| origin.strip_prefix("https://"));
    let Some(authority) = rest else {
        return false;
    };
    if authority.is_empty() || authority.contains(['/', '?', '#', '@', '\\']) {
        return false;
    }
    if let Some(port_str) = authority.strip_prefix("[::1]") {
        return port_str.is_empty()
            || port_str
                .strip_prefix(':')
                .is_some_and(|p| !p.is_empty() && p.parse::<u16>().is_ok());
    }
    let (host, port_opt) = match authority.split_once(':') {
        Some((h, p)) => (h, Some(p)),
        None => (authority, None),
    };
    let valid_host = host.eq_ignore_ascii_case("localhost") || host == "127.0.0.1";
    let valid_port = port_opt.is_none_or(|p| !p.is_empty() && p.parse::<u16>().is_ok());
    valid_host && valid_port
}

fn is_mutating_method(method: &Method) -> bool {
    matches!(
        *method,
        Method::POST | Method::PUT | Method::PATCH | Method::DELETE
    )
}

fn require_json_content_type(headers: &HeaderMap) -> Result<(), ApiError> {
    let raw = headers.get(header::CONTENT_TYPE).ok_or_else(|| {
        ApiError(Dok3Error::InvalidInput(
            "admin mutation requests require Content-Type: application/json".to_string(),
        ))
    })?;
    let Ok(ct) = raw.to_str() else {
        return Err(Dok3Error::InvalidInput("invalid Content-Type header".to_string()).into());
    };
    let media_type = ct.split(';').next().unwrap_or("").trim();
    if !media_type.eq_ignore_ascii_case("application/json") {
        return Err(Dok3Error::InvalidInput(format!(
            "unsupported Content-Type '{media_type}' (expected application/json)"
        ))
        .into());
    }
    Ok(())
}

fn require_index_jobs(state: &ApiState) -> Result<&Arc<dyn IndexJobPort>, ApiError> {
    state.index_jobs.as_ref().ok_or_else(|| {
        ApiError(Dok3Error::InvalidInput(
            "index jobs require persistent mode (DOK3_SOURCE_DIR + DOK3_DATA_DIR)".to_string(),
        ))
    })
}

async fn list_index_jobs(State(state): State<ApiState>) -> Result<impl IntoResponse, ApiError> {
    let port = Arc::clone(require_index_jobs(&state)?);
    let response = tokio::task::spawn_blocking(move || port.list_jobs())
        .await
        .map_err(|err| {
            ApiError(Dok3Error::Internal(format!(
                "index job list task failed: {err}"
            )))
        })??;
    Ok(Json(response))
}

async fn get_index_job(
    State(state): State<ApiState>,
    Path(job_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let port = Arc::clone(require_index_jobs(&state)?);
    let record = tokio::task::spawn_blocking(move || port.get_job(&job_id))
        .await
        .map_err(|err| {
            ApiError(Dok3Error::Internal(format!(
                "index job lookup task failed: {err}"
            )))
        })??;
    Ok(Json(record))
}

async fn trigger_index_job(
    State(state): State<ApiState>,
    body: Bytes,
) -> Result<impl IntoResponse, ApiError> {
    let port = Arc::clone(require_index_jobs(&state)?);
    let req = if body.iter().all(|b| b.is_ascii_whitespace()) {
        TriggerIndexJobRequest::default()
    } else {
        serde_json::from_slice::<TriggerIndexJobRequest>(&body).map_err(|err| {
            ApiError(Dok3Error::InvalidInput(format!(
                "invalid index job request JSON: {err}"
            )))
        })?
    };
    let record = tokio::task::spawn_blocking(move || port.trigger_job(&req))
        .await
        .map_err(|err| {
            ApiError(Dok3Error::Internal(format!(
                "index job execution task failed: {err}"
            )))
        })??;
    Ok(Json(record))
}

fn require_classification(state: &ApiState) -> Result<&Arc<dyn ClassificationPort>, ApiError> {
    state.classification.as_ref().ok_or_else(|| {
        ApiError(Dok3Error::InvalidInput(
            "document classification requires persistent mode (DOK3_SOURCE_DIR + DOK3_DATA_DIR)"
                .to_string(),
        ))
    })
}

async fn get_document_classification(
    State(state): State<ApiState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let port = Arc::clone(require_classification(&state)?);
    let principal = state.default_principal.clone();
    let doc_id = DocumentId::new(id);
    let classification =
        tokio::task::spawn_blocking(move || port.get_classification(&principal, &doc_id))
            .await
            .map_err(|err| {
                ApiError(Dok3Error::Internal(format!(
                    "classification lookup task failed: {err}"
                )))
            })??;
    Ok(Json(classification))
}

async fn update_document_classification(
    State(state): State<ApiState>,
    Path(id): Path<String>,
    body: Bytes,
) -> Result<impl IntoResponse, ApiError> {
    let port = Arc::clone(require_classification(&state)?);
    let req = serde_json::from_slice::<UpdateClassificationRequest>(&body).map_err(|err| {
        ApiError(Dok3Error::InvalidInput(format!(
            "invalid classification update JSON: {err}"
        )))
    })?;
    let principal = state.default_principal.clone();
    let doc_id = DocumentId::new(id);
    let updated =
        tokio::task::spawn_blocking(move || port.update_classification(&principal, &doc_id, &req))
            .await
            .map_err(|err| {
                ApiError(Dok3Error::Internal(format!(
                    "classification update task failed: {err}"
                )))
            })??;
    Ok(Json(updated))
}

fn require_sources(state: &ApiState) -> Result<&Arc<dyn SourceRegistryPort>, ApiError> {
    state.sources.as_ref().ok_or_else(|| {
        ApiError(Dok3Error::InvalidInput(
            "source registry requires persistent mode (DOK3_SOURCE_DIR + DOK3_DATA_DIR)"
                .to_string(),
        ))
    })
}

async fn list_sources(State(state): State<ApiState>) -> Result<impl IntoResponse, ApiError> {
    let port = Arc::clone(require_sources(&state)?);
    let response = tokio::task::spawn_blocking(move || port.list_sources())
        .await
        .map_err(|err| {
            ApiError(Dok3Error::Internal(format!(
                "source list task failed: {err}"
            )))
        })??;
    Ok(Json(response))
}

async fn create_source(
    State(state): State<ApiState>,
    body: Bytes,
) -> Result<impl IntoResponse, ApiError> {
    let port = Arc::clone(require_sources(&state)?);
    let req = serde_json::from_slice::<CreateSourceRequest>(&body).map_err(|err| {
        ApiError(Dok3Error::InvalidInput(format!(
            "invalid source registration JSON: {err}"
        )))
    })?;
    let registered = tokio::task::spawn_blocking(move || port.register_source(&req))
        .await
        .map_err(|err| {
            ApiError(Dok3Error::Internal(format!(
                "source registration task failed: {err}"
            )))
        })??;
    Ok((StatusCode::CREATED, Json(registered)))
}

async fn delete_source(
    State(state): State<ApiState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let port = Arc::clone(require_sources(&state)?);
    let deleted = tokio::task::spawn_blocking(move || port.delete_source(&id))
        .await
        .map_err(|err| {
            ApiError(Dok3Error::Internal(format!(
                "source deletion task failed: {err}"
            )))
        })??;
    Ok(Json(deleted))
}
