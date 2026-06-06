//! SAF — egress message publisher public factory surface.
mod broker_svc;

pub use crate::api::error::PublisherError;
pub use crate::api::traits::MessagePublisher;
pub use crate::api::types::PublisherResult;
pub use crate::api::types::{
    ApplicationConfigBuilder, MessagePublisherConfig, MessagePublisherHandle, MessagePublisherSvc,
};
pub use swe_edge_message_broker::Message;
