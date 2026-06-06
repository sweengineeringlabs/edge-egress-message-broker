//! Integration tests — broker publisher via SAF (replaces default/in-memory backend tests).

use swe_edge_egress_message_publisher::{Message, MessagePublisher, MessagePublisherSvc};
use swe_edge_message_broker::{BrokerError, MessageBroker, MessageStream};

struct MockBroker;
impl MessageBroker for MockBroker {
    fn publish<'a>(
        &'a self,
        _: &'a str,
        _: Message,
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

/// @covers: BrokerPublisherAdapter — publishes via SAF
#[tokio::test]
async fn test_broker_publisher_adapter_publishes_successfully() {
    let p = MessagePublisherSvc::from_broker(MockBroker);
    let msg = Message::new(b"hello".to_vec());
    assert!(p.publish("default.test", msg).await.is_ok());
}

/// @covers: BrokerPublisherAdapter — health check via SAF
#[tokio::test]
async fn test_broker_publisher_adapter_health_check_returns_ok() {
    let p = MessagePublisherSvc::from_broker(MockBroker);
    assert!(p.health_check().await.is_ok());
}
