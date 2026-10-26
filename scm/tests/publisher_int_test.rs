//! Integration tests — MessagePublisher port via SAF factories.
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

#[tokio::test]
async fn test_from_broker_publisher_health_check_returns_ok() {
    let p = MessagePublisherSvc::from_broker(MockBroker);
    assert!(p.health_check().await.is_ok());
}

#[tokio::test]
async fn test_from_broker_publisher_publish_returns_ok_with_no_subscribers() {
    let p = MessagePublisherSvc::from_broker(MockBroker);
    let msg = Message::new(b"ping".to_vec());
    assert!(p.publish("test.events", msg).await.is_ok());
}

#[tokio::test]
async fn test_from_broker_publisher_clone_produces_independent_handle() {
    let p1 = MessagePublisherSvc::from_broker(MockBroker);
    let p2 = p1.clone();
    let msg = Message::new(b"x".to_vec());
    assert!(p1.publish("a", msg.clone()).await.is_ok());
    assert!(p2.publish("b", msg).await.is_ok());
}
