use std::path::PathBuf;

use cid::Cid;
use reqwest::Client;
use url::Url;

#[derive(Debug, Clone)]
pub struct IpfsGateway {
    url: Url,
    client: Client,
}

impl Default for IpfsGateway {
    fn default() -> Self {
        Self {
            url: Url::parse("http://localhost:8080").unwrap(),
            client: Client::new(),
        }
    }
}

impl IpfsGateway {
    pub fn new(url: &Url) -> Self {
        Self {
            url: url.clone(),
            client: Client::new(),
        }
    }
    pub async fn get_bytes(
        &self,
        cid: &Cid,
        path: Option<PathBuf>,
    ) -> Result<Option<Vec<u8>>, IpfsGatewayError> {
        let path_string = match path {
            Some(p) => p.to_string_lossy().to_string(),
            None => "".to_string(),
        };
        let url = self.url.join(&format!("/ipfs/{}/{}", cid, path_string))?;

        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            match response.status().as_u16() {
                404 => return Ok(None),
                _ => {
                    return Err(IpfsGatewayError::BadRequest(format!(
                        "status code: {}",
                        response.status()
                    )))
                }
            }
        }

        Ok(Some(response.bytes().await.map(|b| b.to_vec())?))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum IpfsGatewayError {
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("url error: {0}")]
    UrlError(#[from] url::ParseError),
}
