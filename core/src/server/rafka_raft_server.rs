use crate::server::rafka_config::RaftConfig;
use crate::server::{Result, Server};

/**
 * This struct implements the KRaft (Kafka Raft) mode server which relies
 * on a KRaft quorum for maintaining cluster metadata. It is responsible for
 * constructing the controller and/or broker based on the `process.roles`
 * configuration and for managing their basic lifecycle (startup and shutdown).
 *
 */
pub(crate) struct RafkaRaftServer {
    raft_config: RaftConfig,
}

impl RafkaRaftServer {
    pub fn new(raft_config: RaftConfig) -> Self {
        Self { raft_config }
    }
}

impl Server for RafkaRaftServer {
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
