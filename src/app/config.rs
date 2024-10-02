use std::env;
use std::str::FromStr;
use std::net::SocketAddr;

use dotenvy::dotenv;

use url::Url;

#[derive(Debug)]
pub struct Config {
    // Listen address
    listen_addr: SocketAddr,

    // leaky url Config
    leaky_url: Url,

    // Logging Level
    log_level: tracing::Level,
}

impl Config {
    pub fn from_env() -> Result<Config, ConfigError> {
        if dotenv().is_err() {
            tracing::warn!("No .env file found");
        }
    
        let listen_addr_str = match env::var("LISTEN_ADDR") {
            Ok(addr) => addr,
            Err(_e) => {
                tracing::warn!("No LISTEN_ADDR found in .env. Using default");
                "127.0.0.1:3001".to_string()
            }
        };
        let listen_addr = listen_addr_str.parse()?;

        let leaky_url_str = match env::var("LEAKY_URL") {
            Ok(url) => url,
            Err(_e) => {
                tracing::warn!("No LEAKY_URL found in .env");
                "http://localhost:3001".to_string()
            }
        };
        let leaky_url = Url::parse(&leaky_url_str)?;

        let log_level_str = match env::var("LOG_LEVEL") {
            Ok(level) => level,
            Err(_e) => {
                tracing::warn!("No LOG_LEVEL found in .env. Using default");
                "info".to_string()
            }
        };
        let log_level = match tracing::Level::from_str(&log_level_str) {
            Ok(level) => level,
            Err(_e) => {
                tracing::warn!("Invalid LOG_LEVEL found in .env. Using default");
                tracing::Level::INFO
            }
        };

        Ok(Config {
            listen_addr,
            leaky_url,
            log_level,
        })
    }

    pub fn listen_addr(&self) -> &SocketAddr {
        &self.listen_addr
    }

    pub fn leaky_url(&self) -> &Url {
        &self.leaky_url
    }

    pub fn log_level(&self) -> &tracing::Level {
        &self.log_level
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),
    #[error("Missing Env: {0}")]
    InvalidEnv(#[from] env::VarError),
    #[error("Invalid LogLevel: {0}")]
    InvalidLogLevel(#[from] std::num::ParseIntError),
    #[error("Invalid SocketAddr: {0}")]
    InvalidSocketAddr(#[from] std::net::AddrParseError),
}
