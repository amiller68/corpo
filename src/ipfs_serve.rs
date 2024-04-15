use crate::app::App;
use axum::{
    body::Body,
    extract::State,
    http::{Request, Response, StatusCode, Uri},
    response::IntoResponse,
};

use crate::database::models::RootCid;
use crate::state::AppState;

pub async fn ipfs_and_error_handler(
    uri: Uri,
    State(state): State<AppState>,
    req: Request<Body>,
) -> Result<impl IntoResponse, IpfsServeError> {
    let leptos_options = state.leptos_options.clone();
    let ipfs_gateway = state.ipfs_gateway();
    let database = state.sqlite_database();
    let mut conn = database.acquire().await.unwrap();

    // Get the path from the request
    let path = uri.path();

    // Get the most recent root cid. If not set, just pass through to the app
    let maybe_root_cid = RootCid::read_most_recent(&mut conn).await?;
    let root_cid = match maybe_root_cid {
        Some(root_cid) => root_cid,
        None => {
            let handler = leptos_axum::render_app_to_stream(leptos_options, App);
            return Ok(handler(req).await);
        }
    };

    // Fetch the bytes from ipfs. If the path is not found, just pass through to the app
    let maybe_bytes = ipfs_gateway
        .get_bytes(&root_cid.cid(), Some(path.into()))
        .await?;
    let bytes = match maybe_bytes {
        Some(bytes) => bytes,
        None => {
            let handler = leptos_axum::render_app_to_stream(leptos_options, App);
            return Ok(handler(req).await);
        }
    };

    // Return the bytes
    Ok((StatusCode::OK, bytes).into_response())
}

#[derive(Debug, thiserror::Error)]
pub enum IpfsServeError {
    #[error("error fetching file from ipfs: {0}")]
    Ipfs(#[from] crate::ipfs::IpfsGatewayError),
    #[error("root cid error: {0}")]
    Sqlx(#[from] sqlx::Error),
}

impl IntoResponse for IpfsServeError {
    fn into_response(self) -> Response<Body> {
        match self {
            IpfsServeError::Ipfs(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
            IpfsServeError::Sqlx(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }
}
