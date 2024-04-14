use crate::app::App;
use crate::ipfs::IpfsGateway;
use axum::response::Response as AxumResponse;
use axum::{
    body::Body,
    extract::State,
    http::{Request, Response, StatusCode, Uri},
    response::IntoResponse,
};
use tower::ServiceExt;

use crate::database::models::RootCid;
use crate::state::AppState;

pub async fn ipfs_and_error_handler(
    uri: Uri,
    State(state): State<AppState>,
    req: Request<Body>,
) -> impl IntoResponse {
    let ipfs_gateway = state.ipfs_gateway();
    let database = state.database();
    let conn = database.acquire().await.unwrap();

    // Get the path from the request
    let path = uri.path();
    // Get the most recent root cid
    let root_cid = RootCid::read_most_recent(&conn).await?;

    // Fetch the bytes from ipfs
    let bytes = ipfs_gateway
        .get_bytes(&root_cid.cid, Some(path.into()))
        .await?;

    // TODO: more intelligent content type detection
    // Create a response with the bytes
    let res = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(bytes));

    if res.status() == StatusCode::OK {
        res.into_response()
    } else {
        let handler = leptos_axum::render_app_to_stream(options.to_owned(), App);
        handler(req).await.into_response()
    }
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
