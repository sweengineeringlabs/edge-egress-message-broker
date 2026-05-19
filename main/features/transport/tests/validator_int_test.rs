//! Integration tests — Validator trait via SAF.

use swe_edge_egress_message_broker::{validate, ApplicationConfigBuilder, Validator};

#[test]
fn test_validate_application_config_builder_returns_ok() {
    struct CfgValidator(ApplicationConfigBuilder);
    impl Validator for CfgValidator {
        fn validate(&self) -> Result<(), String> {
            if self.0.capacity == 0 {
                Err("capacity must be > 0".into())
            } else {
                Ok(())
            }
        }
    }

    let v = CfgValidator(ApplicationConfigBuilder::new());
    assert!(validate(&v).is_ok());
}

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
