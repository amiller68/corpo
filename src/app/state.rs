use axum::extract::FromRef;
use url::Url;

use super::config::Config;
use crate::database::Database;

#[derive(Clone)]
pub struct AppState {
    sqlite_database: Database,
    ipfs_api_url: Url,
    // TODO: better proxy solution
    //    ipfs_api_proxy: IpfsApiProxy,
}

#[allow(dead_code)]
impl AppState {
    pub fn sqlite_database(&self) -> &Database {
        &self.sqlite_database
    }

    pub fn ipfs_api_url(&self) -> &Url {
        &self.ipfs_api_url
    }

    /*
        pub fn ipfs_api_proxy(&self) -> &IpfsApiProxy {
            &self.ipfs_api_proxy
        }
    */
    pub async fn from_config(config: &Config) -> Result<Self, AppStateSetupError> {
        let sqlite_database = Database::connect(config.sqlite_database_url()).await?;
        let ipfs_api_url = config.ipfs_api_url().clone();

        Ok(Self {
            sqlite_database,
            ipfs_api_url,
        })
    }
}

impl FromRef<AppState> for Database {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.sqlite_database.clone()
    }
}
/*
impl FromRef<AppState> for IpfsApiProxy {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.ipfs_api_proxy.clone()
    }
}
*/

#[derive(Debug, thiserror::Error)]
pub enum AppStateSetupError {
    #[error("failed to setup the database: {0}")]
    DatabaseSetup(#[from] crate::database::DatabaseSetupError),
    #[error("leptos config error")]
    LeptosConfigError(#[from] leptos_config::errors::LeptosConfigError),
}
