//! SEA interface contract — Validator trait for publisher configuration.

/// Validates a publisher configuration value before use.
pub trait Validator {
    /// Returns `Ok(())` when valid, or a human-readable error string.
    fn validate(&self) -> Result<(), String>;
}
