//! Integration tests — MessagePublisherHandle.
// @allow: no_mocks_in_integration — test doubles required to exercise port contracts without runtime deps

use swe_edge_egress_message_publisher::{
    Message, MessagePublisher, MessagePublisherHandle, MessagePublisherSvc,
};
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

/// @covers: MessagePublisherHandle — Clone
#[tokio::test]
async fn test_publisher_handle_clone_shares_underlying_channel() {
    let h1: MessagePublisherHandle = MessagePublisherSvc::from_broker(MockBroker);
    let h2 = h1.clone();
    let msg = Message::new(b"x".to_vec());
    assert!(h1.publish("a", msg.clone()).await.is_ok());
    assert!(h2.publish("b", msg).await.is_ok());
}

/// @covers: MessagePublisherHandle — MessagePublisher::health_check
#[tokio::test]
async fn test_publisher_handle_health_check_returns_ok() {
    let h: MessagePublisherHandle = MessagePublisherSvc::from_broker(MockBroker);
    assert!(h.health_check().await.is_ok());
}
