use easy_config_def::prelude::*;
use rafka_server_common::{log_cleaner_prefix, log_prefix};

const LOG_CLEANER_DEDUPE_BUFFER_SIZE: i64 = 128 * 1024 * 1024;

pub const LOG_CLEANER_DEDUPE_BUFFER_SIZE_PROP: &str = log_cleaner_prefix!("dedupe.buffer.size");

const LOG_CLEANER_DEDUPE_BUFFER_SIZE_DOC: &str =
    "The total memory used for log deduplication across all cleaner threads";

#[derive(Debug, EasyConfig)]
pub struct CleanerConfig {
    #[attr(name = LOG_CLEANER_DEDUPE_BUFFER_SIZE_PROP,
    default = LOG_CLEANER_DEDUPE_BUFFER_SIZE,
    importance = Importance::MEDIUM,
    documentation = LOG_CLEANER_DEDUPE_BUFFER_SIZE_DOC,
    getter)]
    log_cleaner_dedupe_buffer_size_prop: i64,
}
