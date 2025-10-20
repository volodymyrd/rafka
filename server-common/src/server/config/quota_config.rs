use easy_config_def::prelude::*;

pub const NUM_QUOTA_SAMPLES_CONFIG: &str = "quota.window.num";
const NUM_QUOTA_SAMPLES_DOC: &str = "The number of samples to retain in memory for client quotas";
const NUM_QUOTA_SAMPLES_DEFAULT:u32 = 11;

pub const QUOTA_WINDOW_SIZE_SECONDS_CONFIG: &str = "quota.window.size.seconds";

#[derive(Debug, EasyConfig)]
pub struct QuotaConfig {
    #[attr(name = NUM_QUOTA_SAMPLES_CONFIG,
    default = NUM_QUOTA_SAMPLES_DEFAULT,
    validator = Range::at_least(1),
    importance = Importance::LOW,
    documentation = NUM_QUOTA_SAMPLES_DOC,
    getter)]
    num_quota_samples_config: u32,
}
