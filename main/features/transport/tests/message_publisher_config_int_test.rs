//! Integration tests — MessagePublisherConfig type.

use swe_edge_egress_message_broker::MessagePublisherConfig;

/// @covers: MessagePublisherConfig — default capacity
#[test]
fn test_message_publisher_config_default_has_nonzero_capacity() {
    assert!(MessagePublisherConfig::default().capacity > 0);
}
