//! Typed configuration for the egress message publisher.

/// Runtime configuration for the egress message publisher.
///
/// Loaded from the `[message_publisher]` section of `application.toml`.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct MessagePublisherConfig {
    /// Maximum in-memory channel capacity.
    pub capacity: usize,
}

impl Default for MessagePublisherConfig {
    fn default() -> Self {
        Self { capacity: 1024 }
    }
}

impl swe_edge_configbuilder::ConfigSection for MessagePublisherConfig {
    fn section_name() -> &'static str {
        "message_publisher"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: MessagePublisherConfig::default
    #[test]
    fn test_message_publisher_config_default_capacity_is_1024() {
        assert_eq!(MessagePublisherConfig::default().capacity, 1024);
    }

    /// @covers: ConfigSection::section_name
    #[test]
    fn test_message_publisher_config_section_name_matches_toml_key() {
        assert_eq!(
            <MessagePublisherConfig as swe_edge_configbuilder::ConfigSection>::section_name(),
            "message_publisher"
        );
    }
}
