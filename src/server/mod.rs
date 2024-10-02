use axum::body::Body;
use axum::extract::State;
use axum::http::{header, Request as AxumRequest, Response as AxumResponse, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use http::Request;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use tokio::sync::watch;
use tower::ServiceExt;
use tower_http::services::ServeDir;
use tower_http::trace::{DefaultOnFailure, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing::Level;

use crate::app::{AppState, AppStateSetupError};
use crate::health;
use crate::api;
use crate::web::WebApp;

const HEALTH_ROUTE: &str = "/_status";
const API_ROUTE: &str = "/api/v0";

pub async fn file_and_error_handler(
    uri: Uri,
    State(state): State<AppState>,
    req: AxumRequest<Body>,
) -> Response {
    let options = state.leptos_options.clone();
    let root = options.site_root.clone();
    
    // Try to get the static file first
    match get_static_file(uri.clone(), &root).await {
        Ok(res) => {
            // If it's a static file, return it with the correct content type
            res.into_response()
        }
        Err(_) => {
            // If it's not a static file, render the app
            let handler = leptos_axum::render_app_to_stream(options.to_owned(), WebApp);
            let mut response = handler(req).await.into_response();
            
            // Ensure the content type is set for SSR responses
            response.headers_mut().insert(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("text/html; charset=utf-8"),
            );
            
            response
        }
    }
}

async fn get_static_file(uri: Uri, root: &str) -> Result<AxumResponse<Body>, (StatusCode, String)> {
    let req = AxumRequest::builder()
        .uri(uri.clone())
        .body(Body::empty())
        .unwrap();
    
    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => {
            let mut response = res.into_response();
            
            // Ensure content type is set for static files
            if response.headers().get(header::CONTENT_TYPE).is_none() {
                let content_type = mime_guess::from_path(uri.path())
                    .first_or_octet_stream()
                    .to_string();
                response.headers_mut().insert(
                    header::CONTENT_TYPE,
                    header::HeaderValue::from_str(&content_type).unwrap(),
                );
            }
            
            Ok(response)
        }
        Err(_) => Err((StatusCode::NOT_FOUND, "File not found".to_string())),
    }
}

async fn leptos_routes_handler(
    State(app_state): State<AppState>,
    State(option): axum::extract::State<leptos::LeptosOptions>,
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
    let leptos_site_addr = leptos_options.site_addr;

    let trace_layer = TraceLayer::new_for_http()
        .on_response(
            DefaultOnResponse::new()
                .include_headers(false)
                .level(log_level)
                .latency_unit(LatencyUnit::Micros),
        )
        .on_failure(DefaultOnFailure::new().latency_unit(LatencyUnit::Micros));

    let leptos_routes = generate_route_list(WebApp);
    let root_router = Router::new()
        .leptos_routes_with_handler(leptos_routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .nest(HEALTH_ROUTE, health::router(state.clone()))
        .nest(API_ROUTE, api::router(state.clone()))
        .with_state(state)
        .layer(trace_layer);

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
