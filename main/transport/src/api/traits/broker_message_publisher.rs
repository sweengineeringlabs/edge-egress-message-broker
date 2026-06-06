//! Interface specification for the broker-adapter message publisher implementation.
//!
//! The concrete implementation in `core/broker/publisher/` must implement
//! [`crate::api::traits::MessagePublisher`] to satisfy this interface contract.

/// Marker trait for broker-adapter message publisher implementations.
///
/// The core publisher implements this and a compile-time `PhantomData` reference
/// in `core/broker/publisher/` names it in a type position, so it is a live part
/// of the contract.
pub trait BrokerMessagePublisher: crate::api::traits::MessagePublisher {}
