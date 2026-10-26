//! Integration tests — BrokerPublisherAdapter: port contract via injected broker.
//!
//! Tests that from_broker() correctly adapts a MessageBroker to the MessagePublisher
//! port contract. Uses a mock broker — no runtime deps required.
// @allow: no_mocks_in_integration — test doubles required to exercise port contracts without runtime deps

use swe_edge_egress_message_publisher::{Message, MessagePublisher, MessagePublisherSvc};
use swe_edge_message_broker::MessageBroker;

struct MockBroker;
impl MessageBroker for MockBroker {
    fn publish<'a>(
        &'a self,
        _request: swe_edge_message_broker::PublishRequest,
    ) -> swe_edge_message_broker::BrokerFuture<'a, Result<(), swe_edge_message_broker::BrokerError>>
    {
        swe_edge_message_broker::BrokerFuture::new(async { Ok(()) })
    }

    fn subscribe<'a>(
        &'a self,
        _request: swe_edge_message_broker::SubscribeRequest,
    ) -> swe_edge_message_broker::BrokerFuture<
        'a,
        Result<swe_edge_message_broker::SubscribeResponse, swe_edge_message_broker::BrokerError>,
    > {
        swe_edge_message_broker::BrokerFuture::new(async {
            let stream: swe_edge_message_broker::MessageStream = Box::pin(futures::stream::empty());
            Ok(swe_edge_message_broker::SubscribeResponse { stream })
        })
    }

    fn health_check(
        &self,
        _request: swe_edge_message_broker::HealthCheckRequest,
    ) -> swe_edge_message_broker::BrokerFuture<'_, Result<(), swe_edge_message_broker::BrokerError>>
    {
        swe_edge_message_broker::BrokerFuture::new(async { Ok(()) })
    }

    fn validator(
        &self,
        _request: swe_edge_message_broker::ValidatorRequest,
    ) -> Result<swe_edge_message_broker::ValidatorResponse, swe_edge_message_broker::BrokerError>
    {
        Ok(swe_edge_message_broker::ValidatorResponse {
            validator: std::sync::Arc::from(swe_edge_message_broker::BrokerSvc::create_validator()),
        })
    }
}

/// @covers: BrokerPublisherAdapter — satisfies MessagePublisher contract
#[tokio::test]
async fn test_broker_publisher_adapter_satisfies_message_publisher_contract() {
    let p = MessagePublisherSvc::from_broker(MockBroker);
    let msg = Message::new(b"test".to_vec());
    assert!(p.publish("test.topic", msg).await.is_ok());
    assert!(p.health_check().await.is_ok());
}

/// @covers: BrokerPublisherAdapter — publish succeeds
#[tokio::test]
async fn test_broker_publisher_adapter_publishes_successfully() {
    let p = MessagePublisherSvc::from_broker(MockBroker);
    let msg = Message::new(b"hello".to_vec());
    assert!(p.publish("adapter.test", msg).await.is_ok());
}

/// @covers: BrokerPublisherAdapter — health check
#[tokio::test]
async fn test_broker_publisher_adapter_health_check_returns_ok() {
    let p = MessagePublisherSvc::from_broker(MockBroker);
    assert!(p.health_check().await.is_ok());
}
