use dotenvy::dotenv;
use std::env;

use url::Url;

#[derive(Debug)]
pub struct Config {
    // Database Config
    sqlite_database_url: Url,

    // Ipfs Gateway Config
    ipfs_gateway_url: Url,
}

impl Config {
    pub fn parse_env() -> Result<Config, ConfigError> {
        if dotenv().is_err() {
            tracing::warn!("No .env file found");
        }

        let sqlite_database_url_str = match env::var("SQLITE_DATABASE_URL") {
            Ok(url) => url,
            Err(e) => {
                tracing::warn!("No SQLITE_DATABASE_URL found in .env");
                return Err(ConfigError::InvalidEnv(e));
            }
        };
        let sqlite_database_url = Url::parse(&sqlite_database_url_str)?;

        let ipfs_gateway_url_str = match env::var("IPFS_GATEWAY_URL") {
            Ok(url) => url,
            Err(e) => {
                tracing::warn!("No IPFS_GATEWAY_URL found in .env");
                return Err(ConfigError::InvalidEnv(e));
            }
        };
        let ipfs_gateway_url = Url::parse(&ipfs_gateway_url_str)?;

        Ok(Config {
            sqlite_database_url,
            ipfs_gateway_url,
        })
    }

    pub fn sqlite_database_url(&self) -> &Url {
        &self.sqlite_database_url
    }

    pub fn ipfs_gateway_url(&self) -> &Url {
        &self.ipfs_gateway_url
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),
    #[error("Missing Env: {0}")]
    InvalidEnv(#[from] env::VarError),
}
