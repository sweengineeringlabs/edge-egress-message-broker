//! Integration tests — MessagePublisherSvc factory type.

use swe_edge_egress_message_publisher::MessagePublisherSvc;

/// @covers: MessagePublisherSvc — type exists and is constructible
#[test]
fn test_message_publisher_svc_type_exists() {
    // MessagePublisherSvc is a unit struct; it exists and the type is accessible.
    let _: MessagePublisherSvc = MessagePublisherSvc;
}
