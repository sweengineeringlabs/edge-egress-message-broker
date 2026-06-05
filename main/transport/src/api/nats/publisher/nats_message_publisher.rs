//! Interface specification for the NATS message publisher implementation.
//!
//! The concrete type in `core/nats/publisher/nats_message_publisher.rs`
//! implements [`crate::api::port::MessagePublisher`] to satisfy this contract.

/// Marker trait for NATS-backed message publisher implementations.
///
/// The core publisher implements this and a compile-time `PhantomData` reference
/// in `core/nats/publisher/` names it in a type position, so it is a live part
/// of the contract.
pub trait NatsMessagePublisher: crate::api::port::MessagePublisher {}
