//! NATS-backed `MessagePublisher`.

use std::sync::Arc;

use futures::future::BoxFuture;
use swe_edge_message_broker::{Message, MessageBroker};

use crate::api::error::publisher_error::PublisherError;
use crate::api::port::message_publisher::MessagePublisher;
use crate::api::port::publisher_result::PublisherResult;

/// Publisher backed by a NATS server via `async-nats`.
///
/// Construct via [`crate::saf::MessageBrokerSvc::nats_publisher`].
#[derive(Clone)]
pub(crate) struct NatsMessagePublisher {
    inner: Arc<dyn MessageBroker>,
}

impl NatsMessagePublisher {
    /// Wrap an already-connected [`MessageBroker`] as a publisher.
    pub(crate) fn new(broker: impl MessageBroker + 'static) -> Self {
        Self {
            inner: Arc::new(broker),
        }
    }
}

impl crate::api::nats::publisher::nats_message_publisher::NatsMessagePublisher
    for NatsMessagePublisher
{
}

impl MessagePublisher for NatsMessagePublisher {
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

    struct NatsMessagePublisherMockBroker;
    impl swe_edge_message_broker::MessageBroker for NatsMessagePublisherMockBroker {
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
    fn test_nats_message_publisher_is_object_safe() {
        fn _assert(_: &dyn MessagePublisher) {}
    }

    #[test]
    fn test_nats_message_publisher_new_accepts_any_broker() {
        let _ = NatsMessagePublisher::new(NatsMessagePublisherMockBroker);
    }
}
