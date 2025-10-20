use rafka_clients::common::ConnectionMode;
use rafka_clients::common::security_protocol::SecurityProtocol;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn uses_ssl_transport_layer(security_protocol: &SecurityProtocol) -> bool {
    match security_protocol {
        SecurityProtocol::Ssl | SecurityProtocol::SaslSsl => true,
        _ => false,
    }
}

pub fn uses_sasl_authentication(security_protocol: &SecurityProtocol) -> bool {
    match security_protocol {
        SecurityProtocol::SaslPlaintext | SecurityProtocol::SaslSsl => true,
        _ => false,
    }
}

pub(crate) fn ssl_configs(
    p0: ConnectionMode,
    p1: bool,
    p2: Option<PathBuf>,
    p3: &String,
) -> HashMap<String, String> {
    // TODO: implement me
    HashMap::default()
}
