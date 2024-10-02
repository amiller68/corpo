use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use axum::http::{StatusCode, header};
use axum::body::Body;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use time::OffsetDateTime;
use serde_json::Value;
use crate::app::AppState;

pub async fn handler(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<impl IntoResponse, GetItemsError> {
    let leaky_url = state.leaky_url.clone();
    let client = Client::new();
    let url = leaky_url.join(&format!("/writing/{}?html=true", name))
        .map_err(|_| GetItemsError::UrlJoinError)?;

    let response = client.get(url)
        .send()
        .await
        .map_err(|_| GetItemsError::RequestFailed)?;

    if response.status() == StatusCode::NOT_FOUND {
        return Err(GetItemsError::WritingNotFound);
    }

    let bytes = response.bytes()
        .await
        .map_err(|_| GetItemsError::ResponseReadError)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .body(Body::from(bytes))
        .map_err(|_| GetItemsError::ResponseBuildError)?)
}

#[derive(Debug, thiserror::Error)]
pub enum GetItemsError {
    #[error("Failed to construct URL")]
    UrlJoinError,
    #[error("Failed to send request")]
    RequestFailed,
    #[error("Writing not found")]
    WritingNotFound,
    #[error("Failed to read response body")]
    ResponseReadError,
    #[error("Failed to build response")]
    ResponseBuildError,
}

impl IntoResponse for GetItemsError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            GetItemsError::UrlJoinError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            GetItemsError::RequestFailed => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch writing"),
            GetItemsError::WritingNotFound => (StatusCode::NOT_FOUND, "Writing not found"),
            GetItemsError::ResponseReadError => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read response"),
            GetItemsError::ResponseBuildError => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to build response"),
        };
        (status, error_message).into_response()
    }
}