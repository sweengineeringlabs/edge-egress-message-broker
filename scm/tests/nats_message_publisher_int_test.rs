//! Integration tests — BrokerPublisherAdapter with injected broker (replaces NATS feature-gated tests).
//!
//! The NATS backend is a runtime concern wired by the assembler. This test verifies
//! the from_broker() injection path using a mock broker.
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

/// @covers: BrokerPublisherAdapter — injection-based publisher is healthy
#[tokio::test]
async fn test_broker_publisher_adapter_from_injected_broker_is_healthy() {
    let p = MessagePublisherSvc::from_broker(MockBroker);
    assert!(p.health_check().await.is_ok());
}

/// @covers: BrokerPublisherAdapter — injection-based publisher can publish
#[tokio::test]
async fn test_broker_publisher_adapter_from_injected_broker_can_publish() {
    let p = MessagePublisherSvc::from_broker(MockBroker);
    let msg = Message::new(b"nats.replaced".to_vec());
    assert!(p.publish("nats.replaced", msg).await.is_ok());
}
