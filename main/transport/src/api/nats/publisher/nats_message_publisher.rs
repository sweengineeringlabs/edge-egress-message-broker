//! Interface specification for the NATS message publisher implementation.
//!
//! The concrete type in `core/nats/publisher/nats_message_publisher.rs`
//! implements [`crate::api::port::MessagePublisher`] to satisfy this contract.

/// Marker trait for NATS-backed message publisher implementations.
#[expect(
    dead_code,
    reason = "SEA api/ interface anchor — implemented by the core publisher; not used as a bound"
)]
pub trait NatsMessagePublisher: crate::api::port::MessagePublisher {}
