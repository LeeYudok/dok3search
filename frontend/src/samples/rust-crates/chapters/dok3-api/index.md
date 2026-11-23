# Crate dok3-api

## Overview

Axum HTTP router, security headers, API error mapping, and static SPA hosting.

## Items

- `ApiState` — Shared application state injected into Axum route handlers.
- `Router` — Re-exported so callers can compose extra routes without a direct axum dependency. The router type for composing handlers and services.
- `build_router` — Builds the Axum router with /health (alias of /health/ready), /health/ready, /api/v1/*, JSON /api 404 fallback, security headers, and optional static SPA hosting.
- `build_router_with_extra` — Builds the same router as build_router and merges extra routes (e.g. MCP /mcp) before the SPA fallback and security-header layer, so extra routes win over the SPA fallback and still receive the common security headers.
- `serve_with_shutdown` — Serves the Axum router on listener until shutdown resolves.

### fn dok3-api::build_router

```rust
pub fn build_router(state: ApiState, static_dir: Option<PathBuf>) -> Router
```

Builds the Axum router with `/health` (alias of `/health/ready`), `/health/ready`,
`/api/v1/*`, JSON `/api` 404 fallback, security headers, and optional static SPA hosting.

### fn dok3-api::build_router_with_extra

```rust
pub fn build_router_with_extra(
    state: ApiState,
    static_dir: Option<PathBuf>,
    extra: Option<Router>,
) -> Router
```

Builds the same router as `build_router` and merges `extra` routes (e.g. MCP `/mcp`)
before the SPA fallback and security-header layer, so extra routes win over the SPA
fallback and still receive the common security headers.

`extra` must not define its own fallback or overlap existing routes; Axum panics on either.

### fn dok3-api::serve_with_shutdown

```rust
pub async fn serve_with_shutdown<F>(
    listener: TcpListener,
    app: Router,
    shutdown: F,
) -> Result<()>where
    F: Future<Output = ()> + Send + 'static,
```

Serves the Axum router on `listener` until `shutdown` resolves.

### struct dok3-api::ApiState

```rust
pub struct ApiState {
    pub search: Arc<SearchService>,
    pub default_principal: Principal,
    pub mode: String,
    pub index_jobs: Option<Arc<dyn IndexJobPort>>,
    pub classification: Option<Arc<dyn ClassificationPort>>,
    pub sources: Option<Arc<dyn SourceRegistryPort>>,
}
```

Shared application state injected into Axum route handlers.

**Methods**

- `pub fn new(search: Arc<SearchService>) -> Self`
- `pub fn with_principal(self, principal: Principal) -> Self`
- `pub fn with_index_jobs(self, index_jobs: Arc<dyn IndexJobPort>) -> Self`
- `pub fn with_classification( self, classification: Arc<dyn ClassificationPort>, ) -> Self`
- `pub fn with_sources(self, sources: Arc<dyn SourceRegistryPort>) -> Self`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`

### struct dok3-api::Router

```rust
pub struct Router<S = ()> { /* private fields */ }
```

Re-exported so callers can compose extra routes without a direct `axum` dependency.
The router type for composing handlers and services.

`Router<S>` means a router that is missing a state of type `S` to be able
to handle requests. Thus, only `Router<()>` (i.e. without missing state) can
be passed to `serve`. See `Router::with_state` for more details.

**Methods**

- `pub fn new() -> Router<S>`
- `pub fn without_v07_checks(self) -> Router<S>`
- `pub fn route(self, path: &str, method_router: MethodRouter<S>) -> Router<S>`
- `pub fn route_service<T>(self, path: &str, service: T) -> Router<S>where T: Service<Request<Body>, Error = !> + Clone + Send + Sync + 'static, <T as Service<Request<Body>>>::Response: IntoResponse, <T as Service<Request<Body>>>::Future: Send + 'static,`
- `pub fn nest(self, path: &str, router: Router<S>) -> Router<S>`
- `pub fn nest_service<T>(self, path: &str, service: T) -> Router<S>where T: Service<Request<Body>, Error = !> + Clone + Send + Sync + 'static, <T as Service<Request<Body>>>::Response: IntoResponse, <T as Service<Request<Body>>>::Future: Send + 'static,`
- `pub fn merge<R>(self, other: R) -> Router<S>where R: Into<Router<S>>,`
- `pub fn layer<L>(self, layer: L) -> Router<S>where L: Layer<Route> + Clone + Send + Sync + 'static, <L as Layer<Route>>::Service: Service<Request<Body>> + Clone + Send + Sync + 'static, <<L as Layer<Route>>::Service as Service<Request<Body>>>::Response: IntoResponse + 'static, <<L as Layer<Route>>::Service as Service<Request<Body>>>::Error: Into<!> + 'static, <<L as Layer<Route>>::Service as Service<Request<Body>>>::Future: Send + 'static,`
- `pub fn route_layer<L>(self, layer: L) -> Router<S>where L: Layer<Route> + Clone + Send + Sync + 'static, <L as Layer<Route>>::Service: Service<Request<Body>> + Clone + Send + Sync + 'static, <<L as Layer<Route>>::Service as Service<Request<Body>>>::Response: IntoResponse + 'static, <<L as Layer<Route>>::Service as Service<Request<Body>>>::Error: Into<!> + 'static, <<L as Layer<Route>>::Service as Service<Request<Body>>>::Future: Send + 'static,`
- `pub fn has_routes(&self) -> bool`
- `pub fn fallback<H, T>(self, handler: H) -> Router<S>where H: Handler<T, S>, T: 'static,`
- `pub fn fallback_service<T>(self, service: T) -> Router<S>where T: Service<Request<Body>, Error = !> + Clone + Send + Sync + 'static, <T as Service<Request<Body>>>::Response: IntoResponse, <T as Service<Request<Body>>>::Future: Send + 'static,`
- `pub fn method_not_allowed_fallback<H, T>(self, handler: H) -> Router<S>where H: Handler<T, S>, T: 'static,`
- `pub fn reset_fallback(self) -> Router<S>`
- `pub fn with_state<S2>(self, state: S) -> Router<S2>`
- `pub fn as_service<B>(&mut self) -> RouterAsService<'_, B, S>`
- `pub fn into_service<B>(self) -> RouterIntoService<B, S>`
- `pub fn into_make_service(self) -> IntoMakeService<Router>`
- `pub fn into_make_service_with_connect_info<C>( self, ) -> IntoMakeServiceWithConnectInfo<Router, C>`
- `fn clone(&self) -> Router<S>`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn default() -> Router<S>`
- `fn poll_ready( &mut self, _cx: &mut Context<'_>, ) -> Poll<Result<(), <Router as Service<IncomingStream<'_, L>>>::Error>>`
- `fn call( &mut self, _req: IncomingStream<'_, L>, ) -> <Router as Service<IncomingStream<'_, L>>>::Future`
- `fn poll_ready( &mut self, _: &mut Context<'_>, ) -> Poll<Result<(), <Router as Service<Request<B>>>::Error>>`
- `fn call(&mut self, req: Request<B>) -> <Router as Service<Request<B>>>::Future`
