use easy_config_def::prelude::*;
use rafka_server_common::server_log_configs;

#[derive(Debug, EasyConfig)]
pub struct LogConfig {
    #[attr(name = server_log_configs::NUM_PARTITIONS_CONFIG,
    default = server_log_configs::NUM_PARTITIONS_DEFAULT,
    validator = Range::at_least(1),
    importance = Importance::MEDIUM,
    documentation = server_log_configs::NUM_PARTITIONS_DOC,
    getter)]
    num_partitions_config: u32,

    #[attr(name = server_log_configs::LOG_DIR_CONFIG,
    default = vec![server_log_configs::LOG_DIR_DEFAULT.to_string()],
    validator = ValidList::any_non_duplicate_values(false),
    importance = Importance::HIGH,
    documentation = format!("A comma-separated list of the directories where the log data is stored. \
    (supplemental to {} property)", server_log_configs::LOG_DIRS_CONFIG),
    getter)]
    log_dir_config: Vec<String>,

    #[attr(name = server_log_configs::LOG_DIRS_CONFIG,
    validator = ValidList::any_non_duplicate_values(false),
    importance = Importance::HIGH,
    documentation = format!("A comma-separated list of the directories where the log data is stored. \
    If not set, the value in {} is used.", server_log_configs::LOG_DIRS_CONFIG),
    getter)]
    log_dirs_config: Option<Vec<String>>,

    #[attr(name = server_log_configs::LOG_DELETE_DELAY_MS_CONFIG,
    default = server_log_configs::LOG_DELETE_DELAY_MS_DEFAULT,
    validator = Range::at_least(0),
    importance = Importance::MEDIUM,
    documentation = server_log_configs::LOG_DELETE_DELAY_MS_DOC,
    getter)]
    log_delete_delay_ms_config: i64,

    #[attr(name = server_log_configs::LOG_INITIAL_TASK_DELAY_MS_CONFIG,
    default = server_log_configs::LOG_INITIAL_TASK_DELAY_MS_DEFAULT,
    validator = Range::at_least(0),
    importance = Importance::LOW,
    documentation = server_log_configs::LOG_INITIAL_TASK_DELAY_MS_DOC,
    getter)]
    log_initial_task_delay_ms_config: i64,
}
