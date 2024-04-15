use axum::response::Response as AxumResponse;
use axum::{
    body::Body,
    extract::State,
    http::{Request, Response, StatusCode, Uri},
    response::IntoResponse,
};
use tower::ServiceExt;
use tower_http::services::ServeDir;

use crate::app::App;
use crate::database::models::RootCid;
use crate::state::AppState;

pub async fn file_and_error_handler(
    uri: Uri,
    State(state): State<AppState>,
    req: Request<Body>,
) -> AxumResponse {
    println!("file_and_error_handler: uri={}", uri);
    tracing::info!("file_and_error_handler: uri={}", uri);
    // Attempt to find the static file
    let maybe_res = get_static_file(uri.clone(), &state.leptos_options.site_root)
        .await
        .unwrap();

    match maybe_res {
        Some(res) => {
            // If the static file is found, return it
            println!("file_and_error_handler: static file found");
            return res.into_response();
        }
        None => {
            tracing::info!("file_and_error_handler: static file not found");
        }
    }

    tracing::info!("file_and_error_handler: static file not found");

    // If the static file is not found, try to fetch it from IPFS
    let ipfs_res = get_ipfs_file(&uri, &state).await;

    // If the IPFS file is found, return it
    if ipfs_res.is_ok() {
        println!("file_and_error_handler: ipfs file found");
        // Insert image/svg into the headers
        let mut ipfs_res = ipfs_res.unwrap();
        if uri.path().ends_with(".svg") {
            ipfs_res
                .headers_mut()
                .insert("Content-Type", "image/svg+xml".parse().unwrap());
        }
        return ipfs_res;
    }

    // If both static file and IPFS file are not found, route back to the app
    let handler = leptos_axum::render_app_to_stream(state.leptos_options.clone(), App);
    handler(req).await.into_response()
}

async fn get_static_file(
    uri: Uri,
    root: &str,
) -> Result<Option<Response<Body>>, (StatusCode, String)> {
    let req = Request::builder()
        .uri(uri.clone())
        .body(Body::empty())
        .unwrap();

    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => {
            if res.status() == StatusCode::OK {
                Ok(Some(res.into_response()))
            } else {
                Ok(None)
            }
        }
        Err(err) => {
            if err.to_string().contains("file not found") {
                println!("get_static_file: file not found");
                Ok(None)
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Something went wrong: {err}"),
                ))
            }
        }
    }
}

async fn get_ipfs_file(uri: &Uri, state: &AppState) -> Result<AxumResponse, IpfsServeError> {
    let leptos_options = state.leptos_options.clone();
    let ipfs_gateway = state.ipfs_gateway();
    let database = state.sqlite_database();
    let mut conn = database.acquire().await.unwrap();

    // Get the path from the request
    let path = uri.path();

    tracing::info!("get_ipfs_file: path={}", path);
    println!("get_ipfs_file: path={}", path);

    // Get the most recent root cid. If not set, just pass through to the app
    let maybe_root_cid = RootCid::read_most_recent(&mut conn).await?;
    println!("get_ipfs_file: maybe_root_cid={:?}", maybe_root_cid);
    let root_cid = match maybe_root_cid {
        Some(root_cid) => root_cid,
        None => {
            let handler = leptos_axum::render_app_to_stream(leptos_options, App);
            return Ok(handler(Request::new(Body::empty())).await.into_response());
        }
    };

    tracing::info!("get_ipfs_file: root_cid={:?}", root_cid);
    println!("get_ipfs_file: root_cid={:?}", root_cid);

    // Fetch the bytes from ipfs. If the path is not found, just pass through to the app
    let maybe_bytes = ipfs_gateway
        .get_bytes(&root_cid.cid(), Some(path.into()))
        .await?;
    let bytes = match maybe_bytes {
        Some(bytes) => bytes,
        None => {
            let handler = leptos_axum::render_app_to_stream(leptos_options, App);
            return Ok(handler(Request::new(Body::empty())).await.into_response());
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
