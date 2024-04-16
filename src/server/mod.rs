use std::net::SocketAddr;
use std::time::Duration;

use axum::body::Body;
use axum::error_handling::HandleErrorLayer;
use axum::extract::{DefaultBodyLimit, State};
use axum::handler::HandlerWithoutStateExt;
use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use axum::ServiceExt;
use axum_extra::headers::ContentType;
use axum_extra::TypedHeader;
use http::uri::PathAndQuery;
use http::{header, Request};
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use time::OffsetDateTime;
use tokio::sync::watch;
use tower::ServiceBuilder;
use tower_http::sensitive_headers::{
    SetSensitiveRequestHeadersLayer, SetSensitiveResponseHeadersLayer,
};
use tower_http::services::ServeDir;
use tower_http::trace::{DefaultOnFailure, DefaultOnResponse, MakeSpan, TraceLayer};
use tower_http::{LatencyUnit, ServiceBuilderExt};
use tracing::{Level, Span};

use crate::app::{AppState, AppStateSetupError};
use crate::health;
use crate::web::WebApp;

mod error_handlers;
mod ipfs;

static FILTERED_VALUE: &str = "<filtered>";

static MISSING_VALUE: &str = "<not_provided>";

/// The largest size content that any client can send us before we reject it. This is a pretty
/// heavily restricted default but most JSON responses are relatively tiny.
const REQUEST_MAX_SIZE: usize = 256 * 1_024;

/// The maximum number of seconds that any individual request can take before it is dropped with an
/// error.
const REQUEST_TIMEOUT_SECS: u64 = 5;

const SENSITIVE_HEADERS: &[http::HeaderName] = &[
    header::AUTHORIZATION,
    header::COOKIE,
    header::PROXY_AUTHORIZATION,
    header::SET_COOKIE,
];

#[derive(Clone, Default)]
struct SensitiveRequestMakeSpan;

impl<B> MakeSpan<B> for SensitiveRequestMakeSpan {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        let path_and_query = request
            .uri()
            .clone()
            .into_parts()
            .path_and_query
            .expect("http requests to have a path");

        tracing::span!(
            Level::INFO,
            "http_request",
            method = %request.method(),
            uri = %filter_path_and_query(&path_and_query),
            version = ?request.version(),
        )
    }
}

fn filter_path_and_query(path_and_query: &PathAndQuery) -> String {
    let query = match path_and_query.query() {
        Some(q) => q,
        None => {
            return path_and_query.to_string();
        }
    };

    let mut filtered_query_pairs = vec![];
    for query_pair in query.split('&') {
        let mut qp_iter = query_pair.split('=');

        match (qp_iter.next(), qp_iter.next()) {
            (Some(key), Some(val)) if !key.is_empty() && !val.is_empty() => {
                filtered_query_pairs.push([key, FILTERED_VALUE].join("="));
            }
            (Some(key), None) if !key.is_empty() => {
                filtered_query_pairs.push([key, MISSING_VALUE].join("="));
            }
            unknown => {
                tracing::warn!("encountered weird query pair: {unknown:?}");
            }
        }
    }

    if filtered_query_pairs.is_empty() {
        return path_and_query.path().to_string();
    }

    format!(
        "{}?{}",
        path_and_query.path(),
        filtered_query_pairs.join("&")
    )
}

async fn serve_ipfs_handler(
    uri: Uri,
    State(state): State<AppState>,
    req: Request<Body>,
    TypedHeader(content_type): TypedHeader<ContentType>,
) -> impl IntoResponse {
    let maybe_bytes = ipfs::serve_root(uri, &state).await;
    let bytes = match maybe_bytes {
        Ok(bytes) => bytes,
        Err(e) => match e {
            ipfs::IpfsServeError::MissingRootCid
            | ipfs::IpfsServeError::MissingIpfsContent(_, _) => {
                // Pass through to the not found handler
                return error_handlers::redirect_to_app(&state, req)
                    .await
                    .into_response();
            }
            _ => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    };
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type.to_string())
        .body(Body::from(bytes))
        .expect("response builder to succeed");

    response
}

async fn leptos_routes_handler(
    State(app_state): State<AppState>,
    axum::extract::State(option): axum::extract::State<leptos::LeptosOptions>,
    request: Request<Body>,
) -> axum::response::Response {
    let handler = leptos_axum::render_app_async_with_context(
        option.clone(),
        move || {
            provide_context(app_state.clone());
        },
        move || view! {  <WebApp/> },
    );

    handler(request).await.into_response()
}

pub async fn run(
    log_level: Level,
    state: AppState,
    mut shutdown_rx: watch::Receiver<()>,
) -> Result<(), HttpServerError> {
    let leptos_options = state.leptos_options.clone();
    let leptos_root = leptos_options.site_root.clone();
    let leptos_site_addr = leptos_options.site_addr.clone();

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(SensitiveRequestMakeSpan)
        .on_response(
            DefaultOnResponse::new()
                .include_headers(false)
                .level(log_level)
                .latency_unit(LatencyUnit::Micros),
        )
        .on_failure(DefaultOnFailure::new().latency_unit(LatencyUnit::Micros));

    let static_assets = ServeDir::new(format!("{}/assets", leptos_root))
        .precompressed_br()
        .precompressed_gzip()
        .not_found_service(error_handlers::not_found_handler.into_service());

    let leptos_routes = generate_route_list(WebApp);
    let root_router = Router::new()
        .leptos_routes_with_handler(leptos_routes, get(leptos_routes_handler))
        .nest_service("/assets", static_assets)
        .nest("/_status", health::router(state.clone()))
        .with_state(state)
        //.fallback_service(error_handlers::not_found_handler.into_service())
        // Tracing and log handling get setup before anything else
        .layer(trace_layer);
    //.layer(HandleErrorLayer::new(error_handlers::server_error_handler))
    //.layer(SetSensitiveRequestHeadersLayer::from_shared(
    //    SENSITIVE_HEADERS.into(),
    //))
    // .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
    // .load_shed()
    // .concurrency_limit(1024)
    //.layer(DefaultBodyLimit::max(REQUEST_MAX_SIZE))
    //.layer(SetSensitiveResponseHeadersLayer::from_shared(
    //    SENSITIVE_HEADERS.into(),
    //));

    tracing::info!(addr = ?leptos_site_addr, "server listening");
    let listener = tokio::net::TcpListener::bind(leptos_site_addr).await?;

    axum::serve(listener, root_router)
        .with_graceful_shutdown(async move {
            let _ = shutdown_rx.changed().await;
        })
        .await?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum HttpServerError {
    #[error("an error occurred running the HTTP server: {0}")]
    ServingFailed(#[from] std::io::Error),

    #[error("state initialization failed: {0}")]
    StateInitializationFailed(#[from] AppStateSetupError),
}
