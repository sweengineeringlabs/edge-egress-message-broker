//! Interface specification for the in-memory (default) message publisher backend.
//!
//! The concrete implementation in `core/default/publisher/` must implement
//! [`crate::api::port::MessagePublisher`] to satisfy this interface contract.

/// Marker trait for in-memory (default) message publisher implementations.
#[expect(
    dead_code,
    reason = "SEA api/ interface anchor — implemented by the core publisher; not used as a bound"
)]
pub trait InMemoryMessagePublisher: crate::api::port::MessagePublisher {}
