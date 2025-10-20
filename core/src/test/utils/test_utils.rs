#[path = "../../../../clients/src/test/test_utils.rs"]
mod common_test_utils;
#[path = "../security/jaas_test_utils.rs"]
mod jaas_test_utils;

use common_test_utils::{temp_directory_default, temp_relative_dir};
use rafka_clients::common::ConnectionMode;
use rafka_clients::common::security_protocol::SecurityProtocol;
use rafka_group_coordinator::group_coordinator_config;
use rafka_server::socket_server_config;
use rafka_server::{raft_config, replication_configs};
use rafka_server_common::{delegation_token_manager_configs, server_configs, server_log_configs};
use rafka_storage::cleaner_config;
use std::collections::HashMap;
use std::path::PathBuf;

/* 0 gives a random port; you can then retrieve the assigned port from the Socket object. */
const RANDOM_PORT: i32 = 0;

#[derive(Default)]
pub struct BrokerConfigPropsBuilder {
    node_id: i32,
    enable_controlled_shutdown: Option<bool>,
    enable_delete_topic: Option<bool>,
    port: Option<i32>,
    inter_broker_security_protocol: Option<SecurityProtocol>,
    trust_store_file: Option<PathBuf>,
    sasl_properties: Option<HashMap<String, String>>,
    enable_plaintext: Option<bool>,
    enable_sasl_plaintext: Option<bool>,
    sasl_plaintext_port: Option<i32>,
    enable_ssl: Option<bool>,
    ssl_port: Option<i32>,
    enable_sasl_ssl: Option<bool>,
    sasl_ssl_port: Option<i32>,
    rack: Option<String>,
    log_dir_count: Option<i32>,
    enable_token: Option<bool>,
    num_partitions: Option<i32>,
    default_replication_factor: Option<i16>,
    enable_fetch_from_follower: Option<bool>,
}

impl BrokerConfigPropsBuilder {
    pub fn builder(node_id: i32) -> Self {
        Self {
            node_id,
            ..Default::default()
        }
    }

    pub fn port(mut self, port: i32) -> Self {
        self.port = Some(port);
        self
    }

    pub fn build(self) -> HashMap<String, String> {
        let enable_controlled_shutdown = self.enable_controlled_shutdown.unwrap_or(true);
        let enable_delete_topic = self.enable_delete_topic.unwrap_or(true);
        let port = self.port.unwrap_or(RANDOM_PORT);
        let enable_plaintext = self.enable_plaintext.unwrap_or(true);
        let enable_sasl_plaintext = self.enable_sasl_plaintext.unwrap_or(false);
        let sasl_plaintext_port = self.sasl_plaintext_port.unwrap_or(RANDOM_PORT);
        let enable_ssl = self.enable_ssl.unwrap_or(false);
        let ssl_port = self.ssl_port.unwrap_or(RANDOM_PORT);
        let enable_sasl_ssl = self.enable_sasl_ssl.unwrap_or(false);
        let sasl_ssl_port = self.sasl_ssl_port.unwrap_or(RANDOM_PORT);
        let log_dir_count = self.log_dir_count.unwrap_or(1);
        let enable_token = self.enable_token.unwrap_or(false);
        let num_partitions = self.num_partitions.unwrap_or(1);
        let default_replication_factor = self.default_replication_factor.unwrap_or(1);
        let enable_fetch_from_follower = self.enable_fetch_from_follower.unwrap_or(false);

        let should_enable = |protocol: SecurityProtocol| {
            self.inter_broker_security_protocol
                .map_or(false, |p| p == protocol)
        };

        let mut protocol_and_ports = Vec::new();
        if enable_plaintext || should_enable(SecurityProtocol::Plaintext) {
            protocol_and_ports.push((SecurityProtocol::Plaintext, port));
        }
        if enable_ssl || should_enable(SecurityProtocol::Ssl) {
            protocol_and_ports.push((SecurityProtocol::Ssl, ssl_port));
        }
        if enable_sasl_plaintext || should_enable(SecurityProtocol::SaslPlaintext) {
            protocol_and_ports.push((SecurityProtocol::SaslPlaintext, sasl_plaintext_port));
        }
        if enable_sasl_ssl || should_enable(SecurityProtocol::SaslSsl) {
            protocol_and_ports.push((SecurityProtocol::SaslSsl, sasl_ssl_port));
        }

        let listeners: String = protocol_and_ports
            .iter()
            .map(|(protocol, port)| format!("{}:localhost:{}", protocol.name(), port))
            .collect::<Vec<_>>()
            .join(",");

        // Now we use HashMap<String, String>
        let mut props = HashMap::new();
        props.insert(
            server_configs::UNSTABLE_FEATURE_VERSIONS_ENABLE_CONFIG.to_string(),
            "true".to_string(),
        );
        props.insert(
            server_configs::UNSTABLE_API_VERSIONS_ENABLE_CONFIG.to_string(),
            "true".to_string(),
        );
        props.insert(
            raft_config::SERVER_MAX_STARTUP_TIME_MS_CONFIG.to_string(),
            "600000".to_string(),
        ); // Using direct value for simplicity
        props.insert(
            raft_config::NODE_ID_CONFIG.to_string(),
            self.node_id.to_string(),
        );
        props.insert(
            server_configs::BROKER_ID_CONFIG.to_string(),
            self.node_id.to_string(),
        );
        props.insert(
            socket_server_config::ADVERTISED_LISTENERS_CONFIG.to_string(),
            listeners.clone(),
        );
        props.insert(
            socket_server_config::LISTENERS_CONFIG.to_string(),
            listeners.clone(),
        );
        props.insert(
            raft_config::CONTROLLER_LISTENER_NAMES_CONFIG.to_string(),
            "CONTROLLER".to_string(),
        );
        props.insert(
            socket_server_config::LISTENER_SECURITY_PROTOCOL_MAP_CONFIG.to_string(),
            {
                let map_str = protocol_and_ports
                    .iter()
                    .map(|p| format!("{}:{}", p.0.name(), p.0.name()))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{},CONTROLLER:PLAINTEXT", map_str)
            },
        );

        if log_dir_count > 1 {
            let log_dirs: String = (0..log_dir_count)
                .map(|i| {
                    if i % 2 == 0 {
                        temp_directory_default()
                            .expect("tmp dir should be created")
                            .to_str()
                            .expect("Should be a valid path to tmp dir")
                            .to_string()
                    } else {
                        temp_relative_dir("data")
                            .expect("relative tmp dir should be created")
                            .to_str()
                            .expect("Should be a valid path to tmp dir")
                            .to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join(",");
            props.insert(server_log_configs::LOG_DIRS_CONFIG.to_string(), log_dirs);
        } else {
            props.insert(
                server_log_configs::LOG_DIR_CONFIG.to_string(),
                temp_directory_default()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            );
        }

        props.insert(
            raft_config::PROCESS_ROLES_CONFIG.to_string(),
            "broker".to_string(),
        );
        // Note: this is just a placeholder value for controller.quorum.voters. JUnit
        // tests use random port assignment, so the controller ports are not known ahead of
        // time. Therefore, we ignore controller.quorum.voters and use
        // controllerQuorumVotersFuture instead.
        // props.insert(
        //     QuorumConfig::QUORUM_VOTERS_CONFIG.to_string(),
        //     "1000@localhost:0".to_string(),
        // );
        props.insert(
            replication_configs::REPLICA_SOCKET_TIMEOUT_MS_CONFIG.to_string(),
            "1500".to_string(),
        );
        props.insert(
            replication_configs::CONTROLLER_SOCKET_TIMEOUT_MS_CONFIG.to_string(),
            "1500".to_string(),
        );
        props.insert(
            server_configs::CONTROLLED_SHUTDOWN_ENABLE_CONFIG.to_string(),
            enable_controlled_shutdown.to_string(),
        );
        props.insert(
            server_configs::DELETE_TOPIC_ENABLE_CONFIG.to_string(),
            enable_delete_topic.to_string(),
        );
        props.insert(
            server_log_configs::LOG_DELETE_DELAY_MS_CONFIG.to_string(),
            "1000".to_string(),
        );
        props.insert(
            cleaner_config::LOG_CLEANER_DEDUPE_BUFFER_SIZE_PROP.to_string(),
            "2097152".to_string(),
        );
        props.insert(
            group_coordinator_config::OFFSETS_TOPIC_REPLICATION_FACTOR_CONFIG.to_string(),
            "1".to_string(),
        );
        props.insert(
            server_log_configs::LOG_INITIAL_TASK_DELAY_MS_CONFIG.to_string(),
            "100".to_string(),
        );

        if !props.contains_key(group_coordinator_config::OFFSETS_TOPIC_PARTITIONS_CONFIG) {
            props.insert(
                group_coordinator_config::OFFSETS_TOPIC_PARTITIONS_CONFIG.to_string(),
                "5".to_string(),
            );
        }
        if !props.contains_key(group_coordinator_config::GROUP_INITIAL_REBALANCE_DELAY_MS_CONFIG) {
            props.insert(
                group_coordinator_config::GROUP_INITIAL_REBALANCE_DELAY_MS_CONFIG.to_string(),
                "0".to_string(),
            );
        }

        if let Some(rack_val) = self.rack {
            props.insert(server_configs::BROKER_RACK_CONFIG.to_string(), rack_val);
        }

        props.insert(
            socket_server_config::NUM_NETWORK_THREADS_CONFIG.to_string(),
            "2".to_string(),
        );
        props.insert(
            server_configs::BACKGROUND_THREADS_CONFIG.to_string(),
            "2".to_string(),
        );

        if protocol_and_ports
            .iter()
            .any(|(p, _)| jaas_test_utils::uses_ssl_transport_layer(p))
        {
            // This is a placeholder for the `sslConfigs` method which would return a HashMap
            let ssl_configs = jaas_test_utils::ssl_configs(
                ConnectionMode::Server,
                false,
                self.trust_store_file,
                &format!("server{}", self.node_id),
            );
            props.extend(ssl_configs);
        }

        if protocol_and_ports
            .iter()
            .any(|(p, _)| jaas_test_utils::uses_sasl_authentication(p))
        {
            if let Some(sasl_props) = self.sasl_properties {
                props.extend(sasl_props);
            }
        }

        if let Some(protocol) = self.inter_broker_security_protocol {
            props.insert(
                replication_configs::INTER_BROKER_SECURITY_PROTOCOL_CONFIG.to_string(),
                protocol.name().to_string(),
            );
        }

        if enable_token {
            props.insert(
                delegation_token_manager_configs::DELEGATION_TOKEN_SECRET_KEY_CONFIG.to_string(),
                "secretkey".to_string(),
            );
        }

        props.insert(
            server_log_configs::NUM_PARTITIONS_CONFIG.to_string(),
            num_partitions.to_string(),
        );
        props.insert(
            replication_configs::DEFAULT_REPLICATION_FACTOR_CONFIG.to_string(),
            default_replication_factor.to_string(),
        );

        if enable_fetch_from_follower {
            props.insert(
                server_configs::BROKER_RACK_CONFIG.to_string(),
                self.node_id.to_string(),
            );
            props.insert(
                replication_configs::REPLICA_SELECTOR_CLASS_CONFIG.to_string(),
                "org.apache.kafka.common.replica.RackAwareReplicaSelector".to_string(),
            );
        }

        props
    }
}
