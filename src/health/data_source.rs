#![allow(dead_code)]

use std::ops::Deref;
use std::sync::Arc;

use axum::async_trait;
use axum::extract::FromRequestParts;
use http::request::Parts;

#[async_trait]
pub trait DataSource {
    /// Perform various checks on the system to ensure its healthy and ready to accept requests.
    async fn is_ready(&self) -> Result<(), DataSourceError>;
}

#[derive(Debug, thiserror::Error)]
pub enum DataSourceError {
    #[error("one or more dependent services aren't available")]
    DependencyFailure,

    #[error("service has received signal indicating it should shutdown")]
    ShuttingDown,
}

pub type DynDataSource = Arc<dyn DataSource + Send + Sync>;

pub struct StateDataSource(DynDataSource);

impl StateDataSource {
    #[cfg(test)]
    pub fn new(dds: DynDataSource) -> Self {
        Self(dds)
    }
}

impl Deref for StateDataSource {
    type Target = DynDataSource;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct ExampleSource;

#[async_trait]
impl DataSource for ExampleSource {
    async fn is_ready(&self) -> Result<(), DataSourceError> {
        Ok(())
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for StateDataSource
where
    S: Send + Sync,
{
    type Rejection = ();

    async fn from_request_parts(_parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(StateDataSource(Arc::new(ExampleSource)))
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[derive(Clone)]
    pub(crate) enum MockReadiness {
        DependencyFailure,
        Ready,
        ShuttingDown,
    }

    #[async_trait]
    impl DataSource for MockReadiness {
        async fn is_ready(&self) -> Result<(), DataSourceError> {
            use MockReadiness::*;

            match self {
                DependencyFailure => Err(DataSourceError::DependencyFailure),
                Ready => Ok(()),
                ShuttingDown => Err(DataSourceError::ShuttingDown),
            }
        }
    }
}
