//! Broker-adapter `MessagePublisher`.
//!
//! Wraps any injected [`MessageBroker`] as a [`MessagePublisher`]. The assembler
//! injects the backend; this crate never constructs one itself.

use std::sync::Arc;

use futures::future::BoxFuture;
use swe_edge_message_broker::{Message, MessageBroker};

use crate::api::error::publisher_error::PublisherError;
use crate::api::traits::message_publisher::MessagePublisher;
use crate::api::types::publisher_result::PublisherResult;

/// Adapts any injected [`MessageBroker`] to the [`MessagePublisher`] port contract.
///
/// Construct via [`crate::saf::MessagePublisherSvc::from_broker`].
#[derive(Clone)]
pub(crate) struct BrokerPublisherAdapter {
    inner: Arc<dyn MessageBroker>,
}

impl BrokerPublisherAdapter {
    /// Wrap an already-constructed [`MessageBroker`] as a publisher.
    pub(crate) fn new(broker: impl MessageBroker + 'static) -> Self {
        Self {
            inner: Arc::new(broker),
        }
    }
}

impl crate::api::broker::publisher::broker_message_publisher::BrokerMessagePublisher
    for BrokerPublisherAdapter
{
}

// Name the api/ marker (SEA rule 121) in a type position so it stays a live
// part of the contract; the empty impl above proves the concrete publisher
// conforms to it.
const _: core::marker::PhantomData<
    dyn crate::api::broker::publisher::broker_message_publisher::BrokerMessagePublisher,
> = core::marker::PhantomData;

impl MessagePublisher for BrokerPublisherAdapter {
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

#[cfg(test)]
mod tests {
    use super::*;

    use swe_edge_message_broker::{BrokerError, MessageStream};

    struct MockBroker;
    impl swe_edge_message_broker::MessageBroker for MockBroker {
        fn publish<'a>(
            &'a self,
            _: &'a str,
            _: swe_edge_message_broker::Message,
        ) -> futures::future::BoxFuture<'a, Result<(), BrokerError>> {
            Box::pin(futures::future::ready(Ok(())))
        }
        fn subscribe<'a>(
            &'a self,
            _: &'a str,
        ) -> futures::future::BoxFuture<'a, Result<MessageStream, BrokerError>> {
            Box::pin(futures::future::ready(Ok(
                Box::pin(futures::stream::empty()) as MessageStream,
            )))
        }
        fn health_check(&self) -> futures::future::BoxFuture<'_, Result<(), BrokerError>> {
            Box::pin(futures::future::ready(Ok(())))
        }
    }

    #[test]
    fn test_broker_publisher_adapter_is_object_safe() {
        fn _assert(_: &dyn MessagePublisher) {}
    }

    #[test]
    fn test_broker_publisher_adapter_new_accepts_any_broker() {
        let _ = BrokerPublisherAdapter::new(MockBroker);
    }

    #[tokio::test]
    async fn test_broker_publisher_adapter_health_check_returns_ok() {
        let p = BrokerPublisherAdapter::new(MockBroker);
        assert!(p.health_check().await.is_ok());
    }

    #[tokio::test]
    async fn test_broker_publisher_adapter_publish_to_topic_returns_ok() {
        let p = BrokerPublisherAdapter::new(MockBroker);
        let msg = Message::new(b"hello".to_vec());
        assert!(p.publish("test.topic", msg).await.is_ok());
    }
}
