use axum::extract::FromRef;
use leptos::{get_configuration, LeptosOptions};

use super::config::Config;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
}

#[allow(dead_code)]
impl AppState {
    pub async fn from_config(_config: &Config) -> Result<Self, AppStateSetupError> {
        let conf = get_configuration(None).await?;
        let leptos_options = conf.leptos_options;

        Ok(Self {
            leptos_options,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AppStateSetupError {
    #[error("leptos config error")]
    LeptosConfigError(#[from] leptos_config::errors::LeptosConfigError),
}
