use std::io;
use thiserror::Error;
use tokio::net::TcpListener;

pub(crate) mod rafka_config;
pub(crate) mod rafka_raft_server;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("An error occurred: {0}")]
    Err(Box<dyn std::error::Error + Send + Sync + 'static>),

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
}

impl From<Box<dyn std::error::Error + Send + Sync + 'static>> for ServerError {
    fn from(err: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        ServerError::Err(err)
    }
}

pub type Result<T> = std::result::Result<T, ServerError>;

pub(crate) trait Server {
    async fn startup(&self) -> Result<()>;

    async fn shutdown(&self) -> Result<()>;

    async fn await_shutdown(&self) -> Result<()>;
}
