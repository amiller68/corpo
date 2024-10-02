use axum::routing::get;
use axum::Router;
use http::header::{ACCEPT, ORIGIN};
use http::Method;
use tower_http::cors::{Any, CorsLayer};

use crate::app::AppState;

mod get_items;
mod get_content;

pub fn router(state: AppState) -> Router<AppState> {
    let cors_layer = CorsLayer::new()
        .allow_methods(vec![Method::GET])
        .allow_headers(vec![ACCEPT, ORIGIN])
        .allow_origin(Any)
        .allow_credentials(false);

    Router::new()
        .route("/", get(get_items::handler))
        .route("/:name", get(get_content::handler))
        // TODO: get content
        .with_state(state)
        .layer(cors_layer)
}
