use crate::server::config::config_synonym;
use crate::server::config::config_synonym::ConfigSynonym;
use indexmap::IndexMap;
use once_cell::sync::Lazy;
use rafka_clients::common::config::topic_config;
use rafka_clients::common::utils::utils::mk_map;
use std::sync::Arc;

const LOG_PREFIX: &str = "log.";

#[macro_export]
macro_rules! log_prefix {
    ($suffix:expr) => {
        concat!("log.", $suffix)
    };
}

pub const LOG_CLEANER_PREFIX: &str = log_prefix!("cleaner.");

#[macro_export]
macro_rules! log_cleaner_prefix {
    ($suffix:expr) => {
        log_prefix!(concat!("cleaner.", $suffix))
    };
}

type ConfigEntry = (&'static str, Vec<ConfigSynonym>);

/// Maps official topic configurations to their equivalent server-level (broker) configurations.
///
/// # Purpose
///
/// Topics in Kafka can be configured in two primary ways:
/// 1.  **Dynamic Topic Configuration:** Setting a property directly for a specific topic
/// (e.g., for `my-topic`, setting `retention.ms`).
/// 2.  **Broker-level Configuration:** Setting a server-wide default that applies to all topics
/// unless they have a specific override.
///
/// This map serves as the definitive lookup table to find the correct broker-level setting(s)
/// that corresponds to a topic-level setting.
///
/// # Key Features and Rationale
///
/// This mapping is essential for three main reasons:
///
/// ### 1. Backward Compatibility
/// For historical reasons, many broker-level configurations have different names than their modern
/// topic-level counterparts. For example, the topic config `segment.ms` corresponds to the older
/// broker config `log.roll.ms`. This map maintains these legacy relationships so that existing
/// server configurations continue to work as expected.
///
/// ### 2. Unit Transformation
/// In some cases, a broker-level synonym uses different units than the topic configuration.
/// For instance, `log.roll.hours` is a valid way to set the segment time, but the system
/// internally requires milliseconds. Each `ConfigSynonym` can hold a `converter` function
/// to handle these transformations automatically.
///
/// ### 3. Priority and Precedence
/// A single topic configuration can have multiple broker-level synonyms.
/// The value of this map is a `Vec<ConfigSynonym>`, which is an **ordered list defining priority**.
/// When Kafka resolves a configuration, it checks for the presence of each synonym in the list,
/// in order. The *first* synonym that is found in the user's configuration is used,
/// and the rest are ignored.
///
/// # Example: `retention.ms`
///
/// Consider the entry for `topic_config::RETENTION_MS_CONFIG`:
/// ```rust
/// // The lookup key is "retention.ms"
/// // The value is a vector of synonyms, ordered by priority:
/// vec![
///     // 1. Highest priority: "log.retention.ms" (direct equivalent)
///     ConfigSynonym::new_identity("retention.ms".to_string()),
///
///     // 2. Second priority: "log.retention.minutes" (requires conversion)
///     ConfigSynonym::new("retention.minutes".to_string(), minutes_to_milliseconds),
///
///     // 3. Lowest priority: "log.retention.hours" (requires conversion)
///     ConfigSynonym::new("retention.hours".to_string(), hours_to_milliseconds),
/// ]
/// ```
/// If a user's `server.properties` file contains both `log.retention.minutes`
/// and `log.retention.hours`, only the value from `log.retention.minutes` will be used because
/// it appears earlier in the priority list.
pub static ALL_TOPIC_CONFIG_SYNONYMS: Lazy<IndexMap<&'static str, Vec<ConfigSynonym>>> =
    Lazy::new(|| {
        mk_map(&[
            same_name_with_log_prefix(topic_config::SEGMENT_BYTES_CONFIG),
            list_with_log_prefix(
                topic_config::SEGMENT_MS_CONFIG,
                vec![
                    ConfigSynonym::new_identity("roll.ms".to_string()),
                    ConfigSynonym::new(
                        "roll.hours".to_string(),
                        Arc::new(config_synonym::hours_to_milliseconds),
                    ),
                ],
            ),
            list_with_log_prefix(
                topic_config::SEGMENT_JITTER_MS_CONFIG,
                vec![
                    ConfigSynonym::new_identity("roll.jitter.ms".to_string()),
                    ConfigSynonym::new(
                        "roll.jitter.hours".to_string(),
                        Arc::new(config_synonym::hours_to_milliseconds),
                    ),
                ],
            ),
            single_with_log_prefix(
                topic_config::SEGMENT_INDEX_BYTES_CONFIG,
                "index.size.max.bytes",
            ),
            single_with_log_prefix(
                topic_config::FLUSH_MESSAGES_INTERVAL_CONFIG,
                "flush.interval.messages",
            ),
            list_with_log_prefix(
                topic_config::FLUSH_MS_CONFIG,
                vec![
                    ConfigSynonym::new_identity("flush.interval.ms".to_string()),
                    ConfigSynonym::new_identity("flush.scheduler.interval.ms".to_string()),
                ],
            ),
            same_name_with_log_prefix(topic_config::RETENTION_BYTES_CONFIG),
            list_with_log_prefix(
                topic_config::RETENTION_MS_CONFIG,
                vec![
                    ConfigSynonym::new_identity("retention.ms".to_string()),
                    ConfigSynonym::new(
                        "retention.minutes".to_string(),
                        Arc::new(config_synonym::minutes_to_milliseconds),
                    ),
                    ConfigSynonym::new(
                        "retention.hours".to_string(),
                        Arc::new(config_synonym::hours_to_milliseconds),
                    ),
                ],
            ),
            single(topic_config::MAX_MESSAGE_BYTES_CONFIG, "message.max.bytes"),
            same_name_with_log_prefix(topic_config::INDEX_INTERVAL_BYTES_CONFIG),
            same_name_with_log_cleaner_prefix(topic_config::DELETE_RETENTION_MS_CONFIG),
            same_name_with_log_cleaner_prefix(topic_config::MIN_COMPACTION_LAG_MS_CONFIG),
            same_name_with_log_cleaner_prefix(topic_config::MAX_COMPACTION_LAG_MS_CONFIG),
            single_with_log_prefix(
                topic_config::FILE_DELETE_DELAY_MS_CONFIG,
                "segment.delete.delay.ms",
            ),
            single_with_log_cleaner_prefix(
                topic_config::MIN_CLEANABLE_DIRTY_RATIO_CONFIG,
                "min.cleanable.ratio",
            ),
            same_name_with_log_prefix(topic_config::CLEANUP_POLICY_CONFIG),
            same_name(topic_config::UNCLEAN_LEADER_ELECTION_ENABLE_CONFIG),
            same_name(topic_config::MIN_IN_SYNC_REPLICAS_CONFIG),
            same_name(topic_config::COMPRESSION_TYPE_CONFIG),
            same_name(topic_config::COMPRESSION_GZIP_LEVEL_CONFIG),
            same_name(topic_config::COMPRESSION_LZ4_LEVEL_CONFIG),
            same_name(topic_config::COMPRESSION_ZSTD_LEVEL_CONFIG),
            same_name_with_log_prefix(topic_config::PREALLOCATE_CONFIG),
            same_name_with_log_prefix(topic_config::MESSAGE_TIMESTAMP_TYPE_CONFIG),
            same_name_with_log_prefix(topic_config::MESSAGE_TIMESTAMP_BEFORE_MAX_MS_CONFIG),
            same_name_with_log_prefix(topic_config::MESSAGE_TIMESTAMP_AFTER_MAX_MS_CONFIG),
            same_name_with_log_prefix(topic_config::LOCAL_LOG_RETENTION_MS_CONFIG),
            same_name_with_log_prefix(topic_config::LOCAL_LOG_RETENTION_BYTES_CONFIG),
        ])
    });

/// Map topic config to the server config with the highest priority.
pub static TOPIC_CONFIG_SYNONYMS: Lazy<IndexMap<&'static str, String>> = Lazy::new(|| {
    ALL_TOPIC_CONFIG_SYNONYMS
        .iter()
        .map(|(&key, synonym_list)| {
            let first_synonym = synonym_list
                .first()
                .expect("Synonym list should never be empty")
                .name()
                .to_string();
            (key, first_synonym)
        })
        .collect()
});

/// Return the server config with the highest priority for `topic_config_name`.
/// Panics if the synonym is not found, mimicking Java's `NoSuchElementException`.
pub fn server_synonym(topic_config_name: &str) -> String {
    TOPIC_CONFIG_SYNONYMS
        .get(topic_config_name)
        .cloned()
        .unwrap_or_else(|| panic!("No server synonym found for {}", topic_config_name))
}

fn same_name(config_name: &'static str) -> ConfigEntry {
    (
        config_name,
        vec![ConfigSynonym::new_identity(config_name.to_string())],
    )
}

fn same_name_with_log_prefix(config_name: &'static str) -> ConfigEntry {
    (
        config_name,
        vec![ConfigSynonym::new_identity(format!(
            "{}{}",
            LOG_PREFIX, config_name
        ))],
    )
}

fn same_name_with_log_cleaner_prefix(config_name: &'static str) -> ConfigEntry {
    (
        config_name,
        vec![ConfigSynonym::new_identity(format!(
            "{}{}",
            LOG_CLEANER_PREFIX, config_name
        ))],
    )
}

fn single_with_log_prefix(
    topic_config_name: &'static str,
    broker_config_name: &str,
) -> ConfigEntry {
    (
        topic_config_name,
        vec![ConfigSynonym::new_identity(format!(
            "{}{}",
            LOG_PREFIX, broker_config_name
        ))],
    )
}

fn single_with_log_cleaner_prefix(
    topic_config_name: &'static str,
    broker_config_name: &str,
) -> ConfigEntry {
    (
        topic_config_name,
        vec![ConfigSynonym::new_identity(format!(
            "{}{}",
            LOG_CLEANER_PREFIX, broker_config_name
        ))],
    )
}

fn list_with_log_prefix(
    topic_config_name: &'static str,
    synonyms: Vec<ConfigSynonym>,
) -> ConfigEntry {
    let synonyms_with_prefix = synonyms
        .into_iter()
        .map(|s| {
            let new_name = format!("{}{}", LOG_PREFIX, s.name());
            ConfigSynonym::new(new_name, s.own_converter())
        })
        .collect();
    (topic_config_name, synonyms_with_prefix)
}

fn single(topic_config_name: &'static str, broker_config_name: &str) -> ConfigEntry {
    (
        topic_config_name,
        vec![ConfigSynonym::new_identity(broker_config_name.to_string())],
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_log_prefix() {
        assert_eq!(log_prefix!("test"), "log.test");
    }

    #[test]
    fn test_log_cleaner_prefix() {
        assert_eq!(log_cleaner_prefix!("test"), "log.cleaner.test");
    }
}
