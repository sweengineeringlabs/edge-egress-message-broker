//! Egress message publisher port trait and result type.
pub(crate) mod message_publisher;
pub(crate) mod publisher_result;

pub use message_publisher::MessagePublisher;
pub use publisher_result::PublisherResult;
