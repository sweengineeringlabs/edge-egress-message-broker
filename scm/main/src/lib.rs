//! `swe-edge-egress-message-publisher` — injection-only egress message publisher port.
//!
//! Defines the [`MessagePublisher`] port contract. The assembler injects a concrete
//! backend via [`MessagePublisherSvc::from_broker`] or [`MessagePublisherSvc::publisher`].
//! This crate never constructs runtime brokers itself.
//!
//! # Quick start
//!
//! ```toml
//! [dependencies]
//! swe-edge-egress-message-publisher = { path = "..." }
//! ```
//!
//! ```rust,ignore
//! use swe_edge_egress_message_publisher::{MessagePublisherSvc, Message};
//!
//! // The assembler constructs and injects the broker:
//! let publisher = MessagePublisherSvc::from_broker(broker);
//! publisher.publish("orders.created", Message::new(b"{}".to_vec())).await?;
//! ```
#![deny(unsafe_code)]
#![warn(missing_docs)]

mod api;
mod core;
mod gateway;
mod saf;
pub use gateway::*;
