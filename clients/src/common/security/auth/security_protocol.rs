/// An enum representing the security protocols supported by Kafka.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SecurityProtocol {
    /// Unauthenticated, non-encrypted channel
    Plaintext,
    /// SSL channel
    Ssl,
    /// SASL authenticated, non-encrypted channel
    SaslPlaintext,
    /// SASL authenticated, SSL channel
    SaslSsl,
}

impl SecurityProtocol {
    /// The permanent and immutable id of a security protocol -- this can't change,
    /// and must match kafka.cluster.SecurityProtocol
    pub fn id(&self) -> i16 {
        match self {
            SecurityProtocol::Plaintext => 0,
            SecurityProtocol::Ssl => 1,
            SecurityProtocol::SaslPlaintext => 2,
            SecurityProtocol::SaslSsl => 3,
        }
    }

    /// Name of the security protocol. This may be used by client configuration.
    pub fn name(&self) -> &str {
        match self {
            SecurityProtocol::Plaintext => "PLAINTEXT",
            SecurityProtocol::Ssl => "SSL",
            SecurityProtocol::SaslPlaintext => "SASL_PLAINTEXT",
            SecurityProtocol::SaslSsl => "SASL_SSL",
        }
    }

    /// Returns a list of all security protocol names.
    pub fn names() -> Vec<&'static str> {
        vec!["PLAINTEXT", "SSL", "SASL_PLAINTEXT", "SASL_SSL"]
    }

    /// Returns the `SecurityProtocol` corresponding to the given id.
    pub fn for_id(id: i16) -> Option<Self> {
        match id {
            0 => Some(SecurityProtocol::Plaintext),
            1 => Some(SecurityProtocol::Ssl),
            2 => Some(SecurityProtocol::SaslPlaintext),
            3 => Some(SecurityProtocol::SaslSsl),
            _ => None,
        }
    }

    /// Case-insensitive lookup by protocol name
    pub fn for_name(name: &str) -> Option<Self> {
        match name.to_uppercase().as_str() {
            "PLAINTEXT" => Some(SecurityProtocol::Plaintext),
            "SSL" => Some(SecurityProtocol::Ssl),
            "SASL_PLAINTEXT" => Some(SecurityProtocol::SaslPlaintext),
            "SASL_SSL" => Some(SecurityProtocol::SaslSsl),
            _ => None,
        }
    }

    // A helper to get all enum variants
    pub fn values() -> impl Iterator<Item = Self> {
        [
            SecurityProtocol::Plaintext,
            SecurityProtocol::Ssl,
            SecurityProtocol::SaslPlaintext,
            SecurityProtocol::SaslSsl,
        ]
        .into_iter()
    }
}
