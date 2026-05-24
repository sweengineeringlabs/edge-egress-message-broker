//! Integration tests — Validator trait via SAF.

use swe_edge_egress_message_broker::{validate, MessagePublisherConfig, Validator};

/// @covers: validate — delegates to the Validator impl; non-zero default capacity passes.
#[test]
fn test_validate_default_publisher_config_capacity_nonzero_returns_ok() {
    struct CfgValidator(MessagePublisherConfig);
    impl Validator for CfgValidator {
        fn validate(&self) -> Result<(), String> {
            if self.0.capacity == 0 {
                Err("capacity must be > 0".into())
            } else {
                Ok(())
            }
        }
    }

    assert!(validate(&CfgValidator(MessagePublisherConfig::default())).is_ok());
}

/// @covers: validate — propagates Err from the Validator impl.
#[test]
fn test_validate_returns_err_for_zero_capacity() {
    struct ZeroCapacity;
    impl Validator for ZeroCapacity {
        fn validate(&self) -> Result<(), String> {
            Err("capacity must be > 0".into())
        }
    }
    assert!(validate(&ZeroCapacity).is_err());
}
