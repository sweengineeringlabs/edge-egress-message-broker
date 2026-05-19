//! SAF — egress message publisher public factory surface.
mod edge_egress_message_broker_svc;

pub use crate::api::port::{MessagePublisher, PublisherError, PublisherResult};
pub use crate::api::ApplicationConfigBuilder;
pub use edge_egress_message_broker_svc::{check_health, publish_to, validate};
pub use swe_edge_runtime_message_broker::Message;

#[cfg(feature = "in-memory")]
pub use edge_egress_message_broker_svc::default_publisher;

#[cfg(feature = "nats")]
pub use edge_egress_message_broker_svc::nats_publisher;
