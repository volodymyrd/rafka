use crate::{log_prefix, server_topic_config_synonyms};
use once_cell::sync::Lazy;
use rafka_clients::common::config::topic_config;

pub const NUM_PARTITIONS_CONFIG: &str = "num.partitions";
pub const NUM_PARTITIONS_DEFAULT: u32 = 1;
pub const NUM_PARTITIONS_DOC: &str = "The default number of log partitions per topic";

pub const LOG_DIRS_CONFIG: &str = log_prefix!("dirs");
pub const LOG_DIR_CONFIG: &str = log_prefix!("dir");
pub const LOG_DIR_DEFAULT: &str = "/tmp/kafka-logs";

pub static LOG_DELETE_DELAY_MS_CONFIG: Lazy<String> = Lazy::new(|| {
    server_topic_config_synonyms::server_synonym(topic_config::FILE_DELETE_DELAY_MS_CONFIG)
});
pub const LOG_DELETE_DELAY_MS_DEFAULT: i64 = 60000;
pub const LOG_DELETE_DELAY_MS_DOC: &str = "The amount of time to wait before deleting a file from \
the filesystem. If the value is 0 and there is no file to delete, the system will wait 1 millisecond. \
Low value will cause busy waiting";

pub const LOG_INITIAL_TASK_DELAY_MS_CONFIG: &str = log_prefix!("initial.task.delay.ms");
pub const LOG_INITIAL_TASK_DELAY_MS_DEFAULT: i64 = 30 * 1000;
pub const LOG_INITIAL_TASK_DELAY_MS_DOC: &str = "The initial task delay in millisecond when initializing \
tasks in LogManager. This should be used for testing only.";
