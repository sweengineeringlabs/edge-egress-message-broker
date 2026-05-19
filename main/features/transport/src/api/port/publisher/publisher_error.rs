//! Error type for outbound message publishing operations.

use swe_edge_runtime_message_broker::BrokerError;

/// Errors returned by [`MessagePublisher`](super::message_publisher::MessagePublisher) operations.
#[derive(Debug, thiserror::Error)]
pub enum PublisherError {
    /// Failed to publish a message to the given topic.
    #[error("publish failed on topic '{topic}': {reason}")]
    Publish { topic: String, reason: String },
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_publisher_error_publish_formats_topic_and_reason() {
        let e = PublisherError::Publish {
            topic: "t".into(),
            reason: "r".into(),
        };
        assert!(e.to_string().contains("t"));
        assert!(e.to_string().contains("r"));
    }

    #[test]
    fn test_publisher_error_from_broker_error_publish() {
        let be = BrokerError::Publish {
            topic: "events".into(),
            reason: "nats down".into(),
        };
        let pe = PublisherError::from(be);
        assert!(matches!(pe, PublisherError::Publish { .. }));
    }

    #[test]
    fn test_publisher_error_from_broker_error_unavailable() {
        let be = BrokerError::Unavailable("offline".into());
        let pe = PublisherError::from(be);
        assert!(matches!(pe, PublisherError::Unavailable(_)));
    }
}
