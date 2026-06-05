//! Interface specification for the in-memory (default) message publisher backend.
//!
//! The concrete implementation in `core/default/publisher/` must implement
//! [`crate::api::port::MessagePublisher`] to satisfy this interface contract.

/// Marker trait for in-memory (default) message publisher implementations.
///
/// The core publisher implements this and a compile-time `PhantomData` reference
/// in `core/default/publisher/` names it in a type position, so it is a live
/// part of the contract.
pub trait InMemoryMessagePublisher: crate::api::port::MessagePublisher {}
