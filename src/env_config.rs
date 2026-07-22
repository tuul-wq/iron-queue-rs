use std::{
    net::{AddrParseError, SocketAddr},
    num::ParseIntError,
};

pub struct EnvConfig {
    pub api_addr: SocketAddr,
    pub database_url: String,
    pub database_connection_pool: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum EnvConfigError {
    #[error("missing environment variable {0}")]
    MissingEnvVar(String),

    #[error("invalid API_ADDR")]
    InvalidApiAddr(#[from] AddrParseError),

    #[error("invalid DATABASE_CONNECTION_POOL")]
    InvalidDatabaseConnectionPool(#[from] ParseIntError),
}

impl EnvConfig {
    pub fn from_env() -> Result<Self, EnvConfigError> {
        let api_addr = std::env::var("API_ADDR")
            .map_err(|_| EnvConfigError::MissingEnvVar("API_ADDR".to_string()))?
            .parse()?;

        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| EnvConfigError::MissingEnvVar("DATABASE_URL".to_string()))?;

        let database_connection_pool = std::env::var("DATABASE_CONNECTION_POOL")
            .map_err(|_| EnvConfigError::MissingEnvVar("DATABASE_CONNECTION_POOL".to_string()))?
            .parse()?;

        Ok(Self {
            api_addr,
            database_url,
            database_connection_pool,
        })
    }
}
