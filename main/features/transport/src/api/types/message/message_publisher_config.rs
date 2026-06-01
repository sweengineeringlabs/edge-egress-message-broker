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
        // @allow: no_stub_fn_bodies
        "message_publisher"
    }
}
