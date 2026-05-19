//! `swe-edge-egress-message-broker` — opt-in egress message publisher port.
//!
//! Wraps `swe-edge-message-broker` as a structured egress port. Nothing is
//! compiled unless the caller opts in via a feature flag.
//!
//! # Quick start
//!
//! ```toml
//! [dependencies]
//! swe-edge-egress-message-broker = { path = "...", features = ["in-memory"] }
//! ```
//!
//! ```rust,ignore
//! use swe_edge_egress_message_broker::{default_publisher, Message};
//!
//! let publisher = default_publisher();
//! publisher.publish("orders.created", Message::new(b"{}")).await?;
//! ```
#![deny(unsafe_code)]
#![warn(missing_docs)]

mod api;
mod core;
mod gateway;
mod saf;

pub use gateway::*;
