use crate::config::Config;
use crate::database::Database;
use crate::ipfs::IpfsGateway;

pub struct State {
    sqlite_database: Database,
    ipfs_gateway: IpfsGateway,
}

#[allow(dead_code)]
impl State {
    pub fn sqlite_database(&self) -> &Database {
        &self.sqlite_database
    }

    pub fn ipfs_gateway(&self) -> &IpfsGateway {
        &self.ipfs_gateway
    }

    pub async fn from_config(config: &Config) -> Result<Self, StateSetupError> {
        let sqlite_database = Database::connect(config.sqlite_database_url()).await?;
        let ipfs_gateway = IpfsGateway::new(config.ipfs_gateway_url());

        Ok(Self {
            sqlite_database,
            ipfs_gateway,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StateSetupError {
    #[error("failed to setup the database: {0}")]
    DatabaseSetup(#[from] crate::database::DatabaseSetupError),
}
