use easy_config_def::prelude::*;

/** ********* Delegation Token Configuration ****************/
pub const DELEGATION_TOKEN_SECRET_KEY_CONFIG: &str = "delegation.token.secret.key";
const DELEGATION_TOKEN_SECRET_KEY_DOC: &str = "Secret key to generate and verify delegation tokens. The same key must be configured across all the brokers. \
 If using Kafka with KRaft, the key must also be set across all controllers. \
 If the key is not set or set to empty string, brokers will disable the delegation token support.";

#[derive(Debug, EasyConfig)]
pub struct DelegationTokenManagerConfigs {
    #[attr(name = DELEGATION_TOKEN_SECRET_KEY_CONFIG,
    importance = Importance::MEDIUM,
    documentation = DELEGATION_TOKEN_SECRET_KEY_DOC,
    getter)]
    delegation_token_secret_key_config: Option<Password>,
}
