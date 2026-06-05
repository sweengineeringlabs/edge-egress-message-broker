//! Integration tests — MessagePublisherConfig.

use swe_edge_egress_message_publisher::MessagePublisherConfig;

/// @covers: MessagePublisherConfig::default
#[test]
fn test_message_publisher_config_default_capacity_is_1024() {
    assert_eq!(MessagePublisherConfig::default().capacity, 1024);
}

/// @covers: MessagePublisherConfig — ConfigSection::section_name
#[test]
fn test_message_publisher_config_section_name_is_message_publisher() {
    use swe_edge_configbuilder::ConfigSection;
    assert_eq!(MessagePublisherConfig::section_name(), "message_publisher");
}
