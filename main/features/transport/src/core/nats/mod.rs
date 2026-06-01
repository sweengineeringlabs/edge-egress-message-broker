//! NATS-backed message publisher implementations.
pub(crate) mod publisher;
pub(crate) use publisher::NatsMessagePublisher;
