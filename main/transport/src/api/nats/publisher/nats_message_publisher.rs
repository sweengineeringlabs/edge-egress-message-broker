//! Interface specification for the NATS message publisher implementation.
//!
//! The concrete type in `core/nats/publisher/nats_message_publisher.rs`
//! implements [`crate::api::port::MessagePublisher`] to satisfy this contract.

/// Marker trait for NATS-backed message publisher implementations.
// SEA api/ interface anchor — implemented by the core publisher (under the
// nats feature) but never used as a bound, so its dead/used status varies by
// feature; `allow` tolerates both without an unfulfilled-expectation warning.
#[allow(dead_code)]
pub trait NatsMessagePublisher: crate::api::port::MessagePublisher {}
