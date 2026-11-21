//! Axum HTTP router, security headers, API error mapping, and static SPA hosting.

use std::future::Future;
use std::path::PathBuf;
use std::sync::Arc;

use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::extract::{Path, Query, State};
use axum::http::{HeaderValue, Request, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{any, get, post};
use axum::{Json, Router};
use dok3_core::Dok3Error;
use dok3_domain::{DocumentId, Principal, SearchRequest, SectionId, SectionReadRequest};
use dok3_search::SearchService;
use serde::{Deserialize, Serialize};
use tower::ServiceExt;
use tower_http::services::{ServeDir, ServeFile};

const CSP_POLICY: &str = "default-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; font-src 'self'; connect-src 'self'; object-src 'none'; frame-ancestors 'none'";

/// Shared application state injected into Axum route handlers.
#[derive(Clone)]
pub struct ApiState {
    pub search: Arc<SearchService>,
    pub default_principal: Principal,
    pub mode: String,
}

impl ApiState {
    /// Creates an `ApiState` bound to the anonymous public principal (`in-memory-fake` mode).
    #[must_use]
    pub fn new(search: Arc<SearchService>) -> Self {
        Self {
            search,
            default_principal: Principal::anonymous_public(),
            mode: "in-memory-fake".to_string(),
        }
    }

    /// Overrides the caller principal (primarily for testing or future auth middleware).
    #[must_use]
    pub fn with_principal(mut self, principal: Principal) -> Self {
        self.default_principal = principal;
        self
    }
}

/// Builds the Axum router with `/health/ready`, `/api/v1/*`, JSON `/api` 404 fallback, security headers, and optional static SPA hosting.
pub fn build_router(state: ApiState, static_dir: Option<PathBuf>) -> Router {
    let router = Router::new()
        .route("/health/ready", get(health_ready))
        .route("/api/v1/search", post(search))
        .route("/api/v1/documents/{id}", get(get_document))
        .route("/api/v1/documents/{id}/sections/{sid}", get(get_section))
        .route("/api", any(api_not_found))
        .route("/api/{*rest}", any(api_not_found))
        .with_state(state);

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
    if uri_path
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
