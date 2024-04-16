use axum::extract::FromRef;
use leptos::{get_configuration, LeptosOptions};

use super::config::Config;
use crate::database::Database;
use crate::ipfs::IpfsGateway;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    sqlite_database: Database,
    ipfs_gateway: IpfsGateway,
}

#[allow(dead_code)]
impl AppState {
    pub fn sqlite_database(&self) -> &Database {
        &self.sqlite_database
    }

    pub fn ipfs_gateway(&self) -> &IpfsGateway {
        &self.ipfs_gateway
    }

    pub async fn from_config(config: &Config) -> Result<Self, AppStateSetupError> {
        let conf = get_configuration(None).await?;
        let leptos_options = conf.leptos_options;
        let sqlite_database = Database::connect(config.sqlite_database_url()).await?;
        let ipfs_gateway = IpfsGateway::new(config.ipfs_gateway_url());

        Ok(Self {
            leptos_options,
            sqlite_database,
            ipfs_gateway,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AppStateSetupError {
    #[error("failed to setup the database: {0}")]
    DatabaseSetup(#[from] crate::database::DatabaseSetupError),
    #[error("leptos config error")]
    LeptosConfigError(#[from] leptos_config::errors::LeptosConfigError),
}
