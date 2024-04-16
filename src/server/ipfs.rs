use axum::response::Response as AxumResponse;
use axum::{
    body::Body,
    extract::State,
    http::{Request, Response, StatusCode, Uri},
    response::IntoResponse,
};
use axum_extra::TypedHeader;
use cid::Cid;
use headers::ContentType;
use tower::ServiceExt;
use tower_http::services::ServeDir;

use crate::app::App;
use crate::database::models::RootCid;
use crate::state::AppState;

pub async fn serve_ipfs_root(
    uri: Uri,
    state: &AppState,
    content_type: TypedHeader<ContentType>,
) -> Result<Vec<u8>, IpfsServeError> {
    let ipfs_gateway = state.ipfs_gateway();
    let database = state.sqlite_database();

    // Determine what content is being requested

    let mut conn = database.acquire().await.unwrap();

    // Get the path from the request
    let path = uri.path();
    // Get the most recent root cid. If not set, just pass through to the app
    let maybe_root_cid = RootCid::read_most_recent(&mut conn).await?;
    let root_cid = match maybe_root_cid {
        Some(root_cid) => root_cid,
        None => return Err(IpfsServeError::MissingRootCid),
    };

    tracing::info!("get_ipfs_file: {:?}/{:?}", root_cid.cid(), path);

    // Fetch the bytes from ipfs. If the path is not found, just pass through to the app
    let maybe_bytes = ipfs_gateway
        .get_bytes(&root_cid.cid(), Some(path.into()))
        .await?;
    let bytes = match maybe_bytes {
        Some(bytes) => bytes,
        None => {
            return Err(IpfsServeError::MissingIpfsContent(root_cid.cid().clone(), path.into()))
        }
    };

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(content_type)
        .body(Body::from(bytes))
        .unwrap();

    Ok(response)
}

#[derive(Debug, thiserror::Error)]
pub enum IpfsServeError {
    #[error("missing root cid")]
    MissingRootCid,
    #[error("missing ipfs content: {0}/{1}")]
    MissingIpfsContent(Cid, String),
    #[error("error fetching file from ipfs: {0}")]
    Ipfs(#[from] crate::ipfs::IpfsGatewayError),
    #[error("root cid error: {0}")]
    Sqlx(#[from] sqlx::Error),
}
