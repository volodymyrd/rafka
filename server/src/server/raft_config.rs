use easy_config_def::prelude::*;

pub const PROCESS_ROLES_CONFIG: &str = "process.roles";
const PROCESS_ROLES_DOC: &str = "The roles that this process plays: 'broker', 'controller', \
or 'broker,controller' if it is both. ";

pub const NODE_ID_CONFIG: &str = "node.id";
const NODE_ID_DOC: &str = "The node ID associated with the roles this process is playing \
when <code>process.roles</code> is non-empty. This is required configuration when running in KRaft mode.";

pub const CONTROLLER_LISTENER_NAMES_CONFIG: &str = "controller.listener.names";
const CONTROLLER_LISTENER_NAMES_DOC: &str = "A comma-separated list of the names of the listeners used by the controller. This is required \
    when communicating with the controller quorum, the broker will always use the first listener in this list.";

pub const SERVER_MAX_STARTUP_TIME_MS_CONFIG: &str = "server.max.startup.time.ms";
const SERVER_MAX_STARTUP_TIME_MS_DEFAULT: u32 = u32::MAX;
const SERVER_MAX_STARTUP_TIME_MS_DOC: &str = "The maximum number of milliseconds we will wait \
for the server to come up. By default there is no limit. This should be used for testing only.";

#[derive(Debug, EasyConfig)]
pub struct RaftConfigs {
    #[attr(name = PROCESS_ROLES_CONFIG,
    validator = ValidList::in_list_allow_empty(false, &["broker", "controller"]),
    importance = Importance::HIGH,
    documentation = PROCESS_ROLES_DOC,
    getter)]
    process_roles_config: Vec<String>,

    #[attr(name = NODE_ID_CONFIG,
    validator = Range::at_least(0),
    importance = Importance::HIGH,
    documentation = NODE_ID_DOC,
    getter)]
    node_id_config: u32,

    #[attr(name = CONTROLLER_LISTENER_NAMES_CONFIG,
    validator = ValidList::any_non_duplicate_values(false),
    importance = Importance::HIGH,
    documentation = CONTROLLER_LISTENER_NAMES_DOC,
    getter)]
    controller_listener_names_config: Vec<String>,

    #[attr(name = SERVER_MAX_STARTUP_TIME_MS_CONFIG,
    default = SERVER_MAX_STARTUP_TIME_MS_DEFAULT,
    validator = Range::at_least(0),
    importance = Importance::MEDIUM,
    documentation = SERVER_MAX_STARTUP_TIME_MS_DOC,
    getter)]
    server_max_startup_time_ms_config: u32,
}
