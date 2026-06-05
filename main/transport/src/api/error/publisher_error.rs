//! Error type for outbound message publishing operations.

use swe_edge_runtime_message_broker::BrokerError;

/// Errors returned by [`MessagePublisher`](crate::api::port::MessagePublisher) operations.
#[derive(Debug, thiserror::Error)]
pub enum PublisherError {
    /// Failed to publish a message to the given topic.
    #[error("publish failed on topic '{topic}': {reason}")]
    Publish {
        /// The topic that the publish was attempted on.
        topic: String,
        /// The reason for the failure.
        reason: String,
    },
    /// Publisher is not connected or the broker is unreachable.
    #[error("publisher unavailable: {0}")]
    Unavailable(String),
    /// Connection to the broker failed.
    #[error("connection failed: {0}")]
    Connection(String),
}

impl From<BrokerError> for PublisherError {
    fn from(e: BrokerError) -> Self {
        match e {
            BrokerError::Publish { topic, reason } => PublisherError::Publish { topic, reason },
            BrokerError::Unavailable(m) => PublisherError::Unavailable(m),
            BrokerError::Connection(m) => PublisherError::Connection(m),
            other => PublisherError::Unavailable(other.to_string()),
        }
    }
}
