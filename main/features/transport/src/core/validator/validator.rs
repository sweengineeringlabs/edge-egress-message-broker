//! Default Validator implementation for publisher configuration.

use crate::api::traits::Validator;

/// Passthrough validator — always valid.
///
/// Used as the default `Validator` implementation in contexts where no
/// configuration validation is needed.
pub(crate) struct DefaultValidator;

impl Validator for DefaultValidator {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_validator_always_returns_ok() {
        assert!(DefaultValidator.validate().is_ok());
    }
}
