//! Interface specification for the in-memory (default) message publisher backend.
//!
//! The concrete implementation in `core/default/publisher/` must implement
//! [`crate::api::port::MessagePublisher`] to satisfy this interface contract.

/// Marker trait for in-memory (default) message publisher implementations.
pub trait InMemoryMessagePublisher: crate::api::port::MessagePublisher {}
