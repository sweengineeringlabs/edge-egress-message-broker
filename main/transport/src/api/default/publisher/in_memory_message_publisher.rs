//! Interface specification for the in-memory (default) message publisher backend.
//!
//! The concrete implementation in `core/default/publisher/` must implement
//! [`crate::api::port::MessagePublisher`] to satisfy this interface contract.

/// Marker trait for in-memory (default) message publisher implementations.
// SEA api/ interface anchor — implemented by the core publisher (under the
// in-memory feature) but never used as a bound, so its dead/used status varies
// by feature; `allow` tolerates both without an unfulfilled-expectation warning.
#[allow(dead_code)]
pub trait InMemoryMessagePublisher: crate::api::port::MessagePublisher {}
