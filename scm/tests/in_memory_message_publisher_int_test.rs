//! Integration tests — BrokerPublisherAdapter satisfies the MessagePublisher contract.
//!
//! Previously tested the in-memory backend; now tests via injected mock broker.
// @allow: no_mocks_in_integration — test doubles required to exercise port contracts without runtime deps

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

/// @covers: BrokerPublisherAdapter — satisfied by from_broker via SAF
#[tokio::test]
async fn test_broker_publisher_satisfies_message_publisher_contract() {
    let p = MessagePublisherSvc::from_broker(MockBroker);
    let msg = Message::new(b"in-memory-test".to_vec());
    assert!(p.publish("test.topic", msg).await.is_ok());
    assert!(p.health_check().await.is_ok());
}
