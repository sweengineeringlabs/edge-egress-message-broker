//! In-memory `MessagePublisher` backed by `swe_edge_runtime_message_broker::in_memory_broker()`.

use std::sync::Arc;

use futures::future::BoxFuture;
use swe_edge_runtime_message_broker::{in_memory_broker, Message, MessageBroker};

use crate::api::port::message_publisher::MessagePublisher;
use crate::api::port::publisher::publisher_error::PublisherError;
use crate::api::port::publisher::publisher_result::PublisherResult;
use crate::api::traits::Validator;

/// In-process publisher backed by a tokio broadcast channel.
///
/// Suitable for single-process deployments, integration tests, and development.
#[derive(Clone)]
pub(crate) struct DefaultMessagePublisher {
    inner: Arc<dyn MessageBroker>,
}

impl DefaultMessagePublisher {
    pub(crate) fn new() -> Self {
        Self {
            inner: Arc::new(in_memory_broker()),
        }
    }
}

impl MessagePublisher for DefaultMessagePublisher {
    fn publish<'a>(&'a self, topic: &'a str, msg: Message) -> BoxFuture<'a, PublisherResult<()>> {
        Box::pin(async move {
            self.inner
                .publish(topic, msg)
                .await
                .map_err(PublisherError::from)
        })
    }

    fn health_check(&self) -> BoxFuture<'_, PublisherResult<()>> {
        Box::pin(async move {
            self.inner
                .health_check()
                .await
                .map_err(PublisherError::from)
        })
    }
}

impl Validator for DefaultMessagePublisher {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_message_publisher_new_is_constructible() {
        let _ = DefaultMessagePublisher::new();
    }

    #[tokio::test]
    async fn test_default_message_publisher_health_check_returns_ok() {
        let p = DefaultMessagePublisher::new();
        assert!(p.health_check().await.is_ok());
    }

    #[tokio::test]
    async fn test_default_message_publisher_publish_to_topic_succeeds() {
        let p = DefaultMessagePublisher::new();
        let msg = Message::new(b"hello".to_vec());
        assert!(p.publish("test.topic", msg).await.is_ok());
    }
}
