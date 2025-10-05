pub(crate) struct RaftConfig {}
pub(crate) struct BrokerConfig {}
pub(crate) struct ControllerConfig {}

pub(crate) struct RafkaConfig {
    raft_config: RaftConfig,
    broker_config: BrokerConfig,
    controller_config: ControllerConfig,
}
