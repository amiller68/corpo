use axum::Router;
use http::header::{ACCEPT, ORIGIN};
use http::Method;
use tower_http::cors::{Any, CorsLayer};

mod blog;
mod gallery;

use crate::app::AppState;

pub fn router(state: AppState) -> Router<AppState> {
    let cors_layer = CorsLayer::new()
        .allow_methods(vec![Method::GET])
        .allow_headers(vec![ACCEPT, ORIGIN])
        .allow_origin(Any)
        .allow_credentials(false);

    Router::new()
        .nest("/blog", blog::router(state.clone()))
        .nest("/gallery", gallery::router(state.clone()))
        .with_state(state)
        .layer(cors_layer)
}
