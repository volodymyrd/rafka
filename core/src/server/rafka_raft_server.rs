use crate::server::rafka_config::RafkaConfig;
use crate::server::{Result, Server};

pub(crate) struct RaftServer {
    config: RafkaConfig,
}

impl RaftServer {
    pub fn new(config: RafkaConfig) -> Self {
        Self { config }
    }
}

impl Server for RaftServer {
    async fn startup(&self) -> Result<()> {
        todo!()
    }

    async fn shutdown(&self) -> Result<()> {
        todo!()
    }

    async fn await_shutdown(&self) -> Result<()> {
        todo!()
    }
}
