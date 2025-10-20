use easy_config_def::prelude::*;

pub const CONTROLLER_SOCKET_TIMEOUT_MS_CONFIG: &str = "controller.socket.timeout.ms";
const CONTROLLER_SOCKET_TIMEOUT_MS_DEFAULT: i32 = 30000;
const CONTROLLER_SOCKET_TIMEOUT_MS_DOC: &str =
    "The socket timeout for controller-to-broker channels.";

pub const DEFAULT_REPLICATION_FACTOR_CONFIG: &str = "default.replication.factor";
const REPLICATION_FACTOR_DEFAULT: i32 = 1;
const DEFAULT_REPLICATION_FACTOR_DOC: &str = "The replication factor for automatically created topics, \
and for topics created with -1 as the replication factor";

pub const REPLICA_SOCKET_TIMEOUT_MS_CONFIG: &str = "replica.socket.timeout.ms";
const REPLICA_SOCKET_TIMEOUT_MS_DEFAULT: i32 = 30 * 1000;
const REPLICA_SOCKET_TIMEOUT_MS_DOC: &str = "The socket timeout for network requests. \
Its value should be at least replica.fetch.wait.max.ms";

pub const INTER_BROKER_SECURITY_PROTOCOL_CONFIG: &str = "security.inter.broker.protocol";
pub const INTER_BROKER_LISTENER_NAME_CONFIG: &str = "inter.broker.listener.name";

pub const REPLICA_SELECTOR_CLASS_CONFIG: &str = "replica.selector.class";
const REPLICA_SELECTOR_CLASS_DOC: &str = "The fully qualified class name that implements ReplicaSelector. This is used by the broker to find the preferred read replica. By default, we use an implementation that returns the leader.";

#[derive(Debug, EasyConfig)]
pub struct ReplicationConfigs {
    #[attr(name = CONTROLLER_SOCKET_TIMEOUT_MS_CONFIG,
    default = CONTROLLER_SOCKET_TIMEOUT_MS_DEFAULT,
    importance = Importance::MEDIUM,
    documentation = CONTROLLER_SOCKET_TIMEOUT_MS_DOC,
    getter)]
    controller_socket_timeout_ms_config: i32,

    #[attr(name = DEFAULT_REPLICATION_FACTOR_CONFIG,
    default = REPLICATION_FACTOR_DEFAULT,
    importance = Importance::MEDIUM,
    documentation = DEFAULT_REPLICATION_FACTOR_DOC,
    getter)]
    default_replication_factor_config: i32,

    #[attr(name = REPLICA_SOCKET_TIMEOUT_MS_CONFIG,
    default = REPLICA_SOCKET_TIMEOUT_MS_DEFAULT,
    importance = Importance::HIGH,
    documentation = REPLICA_SOCKET_TIMEOUT_MS_DOC,
    getter)]
    replica_socket_timeout_ms_config: i32,

    #[attr(name = INTER_BROKER_LISTENER_NAME_CONFIG,
    validator = ValidList::in_list_allow_empty(false, &["broker", "controller"]),
    importance = Importance::MEDIUM,
    documentation = format!("Name of listener used for communication between brokers. \
    If this is unset, the listener name is defined by {INTER_BROKER_SECURITY_PROTOCOL_CONFIG}. \
    It is an error to set this and {INTER_BROKER_SECURITY_PROTOCOL_CONFIG} properties at the same time."),
    getter)]
    inter_broker_listener_name_config: String,

    #[attr(name = REPLICA_SELECTOR_CLASS_CONFIG,
    importance = Importance::MEDIUM,
    documentation = REPLICA_SELECTOR_CLASS_DOC,
    getter)]
    replica_selector_class_config: String,
}
