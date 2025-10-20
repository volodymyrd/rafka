use easy_config_def::prelude::*;
use once_cell::sync::Lazy;
use rafka_clients::common::security_protocol::SecurityProtocol;

pub const LISTENER_SECURITY_PROTOCOL_MAP_CONFIG: &str = "listener.security.protocol.map";
const LISTENER_SECURITY_PROTOCOL_MAP_DEFAULT: Lazy<String> = Lazy::new(|| {
    SecurityProtocol::values()
        .map(|sp| {
            let listener_name = sp.name().to_lowercase();
            let security_protocol_name = sp.name();
            format!("{}:{}", listener_name, security_protocol_name)
        })
        .collect::<Vec<String>>()
        .join(",")
});
const LISTENER_SECURITY_PROTOCOL_MAP_DOC: &str = "Map between listener names and security protocols. This must be defined for \
    the same security protocol to be usable in more than one port or IP. For example, internal and \
    external traffic can be separated even if SSL is required for both. Concretely, the user could define listeners \
    with names INTERNAL and EXTERNAL and this property as: <code>INTERNAL:SSL,EXTERNAL:SSL</code>. As shown, key and value are \
    separated by a colon and map entries are separated by commas. Each listener name should only appear once in the map. \
    Different security (SSL and SASL) settings can be configured for each listener by adding a normalised \
    prefix (the listener name is lowercased) to the config name. For example, to set a different keystore for the \
    INTERNAL listener, a config with name <code>listener.name.internal.ssl.keystore.location</code> would be set. \
    If the config for the listener name is not set, the config will fallback to the generic config (i.e. <code>ssl.keystore.location</code>). \
    Note that in KRaft a default mapping from the listener names defined by <code>controller.listener.names</code> to PLAINTEXT \
    is assumed if no explicit mapping is provided and no other security protocol is in use.";

pub const LISTENERS_CONFIG: &str = "listeners";
const LISTENERS_DEFAULT: &str = "PLAINTEXT://:9092";

pub const ADVERTISED_LISTENERS_CONFIG: &str = "advertised.listeners";

pub const NUM_NETWORK_THREADS_CONFIG: &str = "num.network.threads";
const NUM_NETWORK_THREADS_DEFAULT: u32 = 3;
const NUM_NETWORK_THREADS_DOC: &str = "The number of threads that the server uses for receiving requests from the network and sending responses to the network. Noted: each listener (except for controller listener) creates its own thread pool.";

#[derive(Debug, EasyConfig)]
pub struct SocketServerConfig {
    #[attr(name = LISTENERS_CONFIG,
    default = vec![LISTENERS_DEFAULT.to_string()],
    validator = ValidList::any_non_duplicate_values(false),
    importance = Importance::HIGH,
    documentation = format!("Listener List - Comma-separated list of URIs we will listen on and the listener names.\
         If the listener name is not a security protocol, <code>{LISTENER_SECURITY_PROTOCOL_MAP_CONFIG}</code> must also be set.\n\
         Listener names and port numbers must be unique unless one listener is an IPv4 address and the other listener is an IPv6 address (for the same port).\n\
         Specify hostname as 0.0.0.0 to bind to all interfaces.\n\
         Leave hostname empty to bind to default interface.\n\
         Examples of legal listener lists:\n\
         `PLAINTEXT://myhost:9092,SSL://:9091`\n\
         `CLIENT://0.0.0.0:9092,REPLICATION://localhost:9093`\n\
         `PLAINTEXT://127.0.0.1:9092,SSL://[::1]:9092`\n"),
    getter)]
    listeners_config: Vec<String>,

    #[attr(name = ADVERTISED_LISTENERS_CONFIG,
    validator = ValidList::any_non_duplicate_values(false),
    importance = Importance::HIGH,
    documentation = format!("Specifies the listener addresses that the Kafka brokers will advertise \
    to clients and other brokers. The config is useful where the actual listener configuration `{LISTENERS_CONFIG}` \
    does not represent the addresses that clients should use to connect, such as in cloud environments. \
    The addresses are published to and managed by the controller, the brokers pull these data from \
    the controller as needed. In IaaS environments, this may need to be different from the interface \
    to which the broker binds. If this is not set, the value for `{LISTENERS_CONFIG}` will be used. \
    Unlike `{LISTENERS_CONFIG}`, it is not valid to advertise the 0.0.0.0 meta-address.\nAlso unlike `{LISTENERS_CONFIG}`, \
    there can be duplicated ports in this property, so that one listener can be configured \
    to advertise another listener's address. This can be useful in some cases where external \
    load balancers are used."),
    getter)]
    advertised_listeners_config: Vec<String>,

    #[attr(name = LISTENER_SECURITY_PROTOCOL_MAP_CONFIG,
    importance = Importance::LOW,
    default = LISTENER_SECURITY_PROTOCOL_MAP_DEFAULT.clone(),
    documentation = LISTENER_SECURITY_PROTOCOL_MAP_DOC,
    getter)]
    listener_security_protocol_map_config: String,

    #[attr(name = NUM_NETWORK_THREADS_CONFIG,
    default = NUM_NETWORK_THREADS_DEFAULT,
    validator = Range::at_least(1),
    importance = Importance::HIGH,
    documentation = NUM_NETWORK_THREADS_DOC,
    getter)]
    num_network_threads_config: u32,
}
