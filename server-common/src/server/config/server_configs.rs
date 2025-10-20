use easy_config_def::prelude::*;

/** ********* General Configuration ***********/
pub const BROKER_ID_CONFIG: &str = "broker.id";
const BROKER_ID_DEFAULT: i32 = -1;
const BROKER_ID_DOC: &str = "The broker id for this server.";

pub const BACKGROUND_THREADS_CONFIG: &str = "background.threads";
const BACKGROUND_THREADS_DEFAULT: u32 = 10;
const BACKGROUND_THREADS_DOC: &str =
    "The number of threads to use for various background processing tasks";

pub const DELETE_TOPIC_ENABLE_CONFIG: &str = "delete.topic.enable";
const DELETE_TOPIC_ENABLE_DEFAULT: bool = true;
const DELETE_TOPIC_ENABLE_DOC: &str = "When set to true, topics can be deleted by the admin client. \
When set to false, deletion requests will be explicitly rejected by the broker.";

/***************** rack configuration *************/
pub const BROKER_RACK_CONFIG: &str = "broker.rack";
const BROKER_RACK_DOC: &str = "Rack of the broker. This will be used in rack aware replication assignment for fault tolerance. Examples: <code>RACK1</code>, <code>us-east-1d</code>";

/** ********* Controlled shutdown configuration ***********/
pub const CONTROLLED_SHUTDOWN_ENABLE_CONFIG: &str = "controlled.shutdown.enable";
const CONTROLLED_SHUTDOWN_ENABLE_DEFAULT: bool = true;
const CONTROLLED_SHUTDOWN_ENABLE_DOC: &str = "Enable controlled shutdown of the server.";

/// Internal Configurations
pub const UNSTABLE_API_VERSIONS_ENABLE_CONFIG: &str = "unstable.api.versions.enable";
pub const UNSTABLE_FEATURE_VERSIONS_ENABLE_CONFIG: &str = "unstable.feature.versions.enable";

#[derive(Debug, EasyConfig)]
pub struct ServerConfig {
    #[attr(name = BROKER_ID_CONFIG,
    default = BROKER_ID_DEFAULT,
    importance = Importance::HIGH,
    documentation = BROKER_ID_DOC,
    getter)]
    broker_id_config: i32,

    #[attr(name = BACKGROUND_THREADS_CONFIG,
    default = BACKGROUND_THREADS_DEFAULT,
    importance = Importance::HIGH,
    documentation = BACKGROUND_THREADS_DOC,
    getter)]
    background_threads_config: u32,

    /************ Rack Configuration ******************/
    #[attr(name = BROKER_RACK_CONFIG,
    importance = Importance::MEDIUM,
    documentation = BROKER_RACK_DOC,
    getter)]
    broker_rack_config: Option<String>,

    /** ********* Controlled shutdown configuration ***********/
    #[attr(name = CONTROLLED_SHUTDOWN_ENABLE_CONFIG,
    default = CONTROLLED_SHUTDOWN_ENABLE_DEFAULT,
    importance = Importance::MEDIUM,
    documentation = CONTROLLED_SHUTDOWN_ENABLE_DOC,
    getter)]
    controlled_shutdown_enable_config: bool,

    #[attr(name = DELETE_TOPIC_ENABLE_CONFIG,
    default = DELETE_TOPIC_ENABLE_DEFAULT,
    importance = Importance::HIGH,
    documentation = DELETE_TOPIC_ENABLE_DOC,
    getter)]
    delete_topic_enable_config: bool,

    /** Internal Configurations **/
    /// This indicates whether unreleased APIs should be advertised by this node.
    #[attr(name = UNSTABLE_API_VERSIONS_ENABLE_CONFIG,
    default = false,
    importance = Importance::HIGH,
    getter)]
    unstable_api_versions_enable_config: bool,

    /// This indicates whether unreleased MetadataVersions should be enabled on this node.
    #[attr(name = UNSTABLE_FEATURE_VERSIONS_ENABLE_CONFIG,
    default = false,
    importance = Importance::HIGH,
    getter)]
    unstable_feature_versions_enable_config: bool,
}
