//! SAF — egress message publisher public factory surface.
mod broker_svc;

pub use crate::api::error::PublisherError;
pub use crate::api::port::{MessagePublisher, PublisherResult};
pub use crate::api::types::{
    ApplicationConfigBuilder, MessageBrokerSvc, MessagePublisherConfig, MessagePublisherHandle,
};
pub use swe_edge_message_broker::Message;
