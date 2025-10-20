struct ConnectionQuotas {}

#[cfg(test)]
mod tests {
    use crate::server::rafka_config::RafkaConfig;
    use crate::test::utils::test_utils::BrokerConfigPropsBuilder;
    use easy_config_def::FromConfigDef;
    use rafka_server::{replication_configs, socket_server_config};
    use rafka_server_common::quota_config;
    use std::collections::HashMap;

    const NUM_QUOTA_SAMPLES: usize = 2;
    const QUOTA_WINDOW_SIZE_SECONDS: usize = 1;
    const EPS: f32 = 0.01;

    fn broker_props_with_default_connection_limits() -> HashMap<String, String> {
        let mut props = BrokerConfigPropsBuilder::builder(0).port(0).build();
        props.insert(
            socket_server_config::LISTENERS_CONFIG.to_string(),
            "EXTERNAL://localhost:0,REPLICATION://localhost:1,ADMIN://localhost:2".to_string(),
        );
        props.insert(
            replication_configs::INTER_BROKER_LISTENER_NAME_CONFIG.to_string(),
            "REPLICATION".to_string(),
        );
        props.insert(
            socket_server_config::ADVERTISED_LISTENERS_CONFIG.to_string(),
            "REPLICATION://localhost:1".to_string(),
        );
        props.insert(socket_server_config::LISTENER_SECURITY_PROTOCOL_MAP_CONFIG.to_string(),
                     "PLAINTEXT:PLAINTEXT,CONTROLLER:PLAINTEXT,EXTERNAL:PLAINTEXT,REPLICATION:PLAINTEXT,ADMIN:PLAINTEXT".to_string());
        props.insert(
            quota_config::NUM_QUOTA_SAMPLES_CONFIG.to_string(),
            NUM_QUOTA_SAMPLES.to_string(),
        );
        props.insert(
            quota_config::QUOTA_WINDOW_SIZE_SECONDS_CONFIG.to_string(),
            QUOTA_WINDOW_SIZE_SECONDS.to_string(),
        );
        props
    }

    #[test]
    fn test_fail_when_no_listeners() {
        let config = RafkaConfig::from_props(&broker_props_with_default_connection_limits());
        println!("{:?}", config);
    }
}
