//! Integration tests — PublisherResult type alias.

use swe_edge_egress_message_publisher::{PublisherError, PublisherResult};

/// @covers: PublisherResult — Ok variant
#[test]
fn test_publisher_result_ok_variant_holds_value() {
    let r: PublisherResult<u32> = Ok(42);
    assert_eq!(r.ok(), Some(42));
}

/// @covers: PublisherResult — Err variant
#[test]
fn test_publisher_result_err_variant_holds_error() {
    let r: PublisherResult<u32> = Err(PublisherError::Unavailable("down".into()));
    assert!(r.is_err());
}
