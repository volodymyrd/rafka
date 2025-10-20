use easy_config_def::prelude::*;

pub const OFFSETS_TOPIC_PARTITIONS_CONFIG: &str = "offsets.topic.num.partitions";
const OFFSETS_TOPIC_PARTITIONS_DEFAULT: u32 = 50;
const OFFSETS_TOPIC_PARTITIONS_DOC: &str =
    "The number of partitions for the offset commit topic (should not change after deployment).";

pub const OFFSETS_TOPIC_REPLICATION_FACTOR_CONFIG: &str = "offsets.topic.replication.factor";
const OFFSETS_TOPIC_REPLICATION_FACTOR_DEFAULT: u16 = 3;
const OFFSETS_TOPIC_REPLICATION_FACTOR_DOC: &str = "The replication factor for the offsets topic (set higher to ensure availability). \
Internal topic creation will fail until the cluster size meets this replication factor requirement.";

pub const GROUP_INITIAL_REBALANCE_DELAY_MS_CONFIG: &str = "group.initial.rebalance.delay.ms";
const GROUP_INITIAL_REBALANCE_DELAY_MS_DOC: &str = "The amount of time the group coordinator will wait for more consumers to join a new group \
before performing the first rebalance. A longer delay means potentially fewer rebalances, but increases the time until processing begins.";
const GROUP_INITIAL_REBALANCE_DELAY_MS_DEFAULT: i32 = 3000;

#[derive(Debug, EasyConfig)]
pub struct GroupCoordinatorConfig {
    // Group coordinator configs
    #[attr(name = OFFSETS_TOPIC_REPLICATION_FACTOR_CONFIG,
    default = OFFSETS_TOPIC_REPLICATION_FACTOR_DEFAULT,
    validator = Range::at_least(1),
    importance = Importance::HIGH,
    documentation = OFFSETS_TOPIC_REPLICATION_FACTOR_DOC,
    getter)]
    offsets_topic_replication_factor_config: u16,

    #[attr(name = OFFSETS_TOPIC_PARTITIONS_CONFIG,
    default = OFFSETS_TOPIC_PARTITIONS_DEFAULT,
    validator = Range::at_least(1),
    importance = Importance::HIGH,
    documentation = OFFSETS_TOPIC_PARTITIONS_DOC,
    getter)]
    offsets_topic_partitions_config: u32,
    
    // Classic group configs
    #[attr(name = GROUP_INITIAL_REBALANCE_DELAY_MS_CONFIG,
    default = GROUP_INITIAL_REBALANCE_DELAY_MS_DEFAULT,
    importance = Importance::MEDIUM,
    documentation = GROUP_INITIAL_REBALANCE_DELAY_MS_DOC,
    getter)]
    group_initial_rebalance_delay_ms_config: i32,
}
