use easy_config_def::prelude::*;
use rafka_group_coordinator::group_coordinator_config::GroupCoordinatorConfig;
use rafka_server::{raft_config::RaftConfigs, socket_server_config::SocketServerConfig};
use rafka_server_common::{
    delegation_token_manager_configs::DelegationTokenManagerConfigs, quota_config::QuotaConfig,
    server_configs::ServerConfig,
};
use rafka_storage::{CleanerConfig, LogConfig};

#[derive(Debug, EasyConfig)]
pub struct RafkaConfig {
    #[merge]
    server_configs: ServerConfig,

    #[merge]
    raft_configs: RaftConfigs,

    #[merge]
    socket_server_config: SocketServerConfig,

    #[merge]
    group_coordinator_config: GroupCoordinatorConfig,

    #[merge]
    cleaner_config: CleanerConfig,

    #[merge]
    log_config: LogConfig,

    #[merge]
    quota_config: QuotaConfig,

    #[merge]
    delegation_token_manager_configs: DelegationTokenManagerConfigs,
}
