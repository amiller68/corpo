use axum::extract::FromRef;
use leptos::{get_configuration, LeptosOptions};
use url::Url;

use super::config::Config;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub leaky_url: Url,
}

#[allow(dead_code)]
impl AppState {
    pub async fn from_config(config: &Config) -> Result<Self, AppStateSetupError> {
        let conf = get_configuration(None).await?;
        let leptos_options = conf.leptos_options;
        let leaky_url = config.leaky_url().clone();

        Ok(Self {
            leptos_options,
            leaky_url,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AppStateSetupError {
    #[error("leptos config error")]
    LeptosConfigError(#[from] leptos_config::errors::LeptosConfigError),
}
