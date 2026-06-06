//! Integration tests — PublisherError type.

use swe_edge_egress_message_publisher::PublisherError;

/// @covers: PublisherError::Publish
#[test]
fn test_publisher_error_publish_formats_topic_and_reason() {
    let e = PublisherError::Publish {
        topic: "orders".into(),
        reason: "broker down".into(),
    };
    let s = e.to_string();
    assert!(s.contains("orders"), "should contain topic");
    assert!(s.contains("broker down"), "should contain reason");
}

/// @covers: PublisherError::Unavailable
#[test]
fn test_publisher_error_unavailable_formats_message() {
    let e = PublisherError::Unavailable("offline".into());
    assert!(e.to_string().contains("offline"));
}

/// @covers: PublisherError::Connection
#[test]
fn test_publisher_error_connection_formats_message() {
    let e = PublisherError::Connection("refused".into());
    assert!(e.to_string().contains("refused"));
}

/// @covers: PublisherError::from(BrokerError)
#[test]
fn test_publisher_error_from_broker_error_publish() {
    use swe_edge_message_broker::BrokerError;
    let be = BrokerError::Publish {
        topic: "events".into(),
        reason: "nats down".into(),
    };
    let pe = PublisherError::from(be);
    assert!(matches!(pe, PublisherError::Publish { .. }));
}

/// @covers: PublisherError::from(BrokerError)
#[test]
fn test_publisher_error_from_broker_error_unavailable() {
    use swe_edge_message_broker::BrokerError;
    let be = BrokerError::Unavailable("offline".into());
    let pe = PublisherError::from(be);
    assert!(matches!(pe, PublisherError::Unavailable(_)));
}
