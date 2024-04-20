use axum::body::Body;
use axum::http::Request;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum_extra::headers::ContentType;
use axum_extra::TypedHeader;

use crate::app::AppState;
use crate::web::WebApp;

pub async fn not_found_handler(TypedHeader(content_type): TypedHeader<ContentType>) -> Response {
    let content_type = content_type.to_string();

    match content_type.as_str() {
        "application/json" => {
            let err_msg = serde_json::json!({"msg": "not found"});
            (StatusCode::NOT_FOUND, Json(err_msg)).into_response()
        }
        "text/html" => {
            let body = "<h1>Not Found</h1>";
            (StatusCode::NOT_FOUND, body).into_response()
        }
        _ => (StatusCode::NOT_FOUND, "not found").into_response(),
    }
}

pub async fn redirect_to_app(state: &AppState, req: Request<Body>) -> Response {
    let handler = leptos_axum::render_app_to_stream(state.leptos_options.clone(), WebApp);
    handler(req).await.into_response()
}
