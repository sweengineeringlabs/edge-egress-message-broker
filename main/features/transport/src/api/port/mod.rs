//! Egress message publisher port traits and error types.
pub(crate) mod message_publisher;
pub(crate) mod publisher;

pub use message_publisher::MessagePublisher;
pub use publisher::{PublisherError, PublisherResult};
