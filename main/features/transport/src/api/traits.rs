//! SEA interface contract — egress message publisher traits.

/// Validates a publisher configuration value before use.
pub trait Validator {
    /// Returns `Ok(())` when valid, or a human-readable error string.
    fn validate(&self) -> Result<(), String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AlwaysValid;
    impl Validator for AlwaysValid {
        fn validate(&self) -> Result<(), String> {
            Ok(())
        }
    }

    #[test]
    fn test_validator_ok_returns_unit() {
        assert!(AlwaysValid.validate().is_ok());
    }
}
