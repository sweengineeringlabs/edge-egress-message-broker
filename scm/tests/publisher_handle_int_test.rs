//! Integration tests — MessagePublisherHandle.
// @allow: no_mocks_in_integration — test doubles required to exercise port contracts without runtime deps

use swe_edge_egress_message_publisher::{
    Message, MessagePublisher, MessagePublisherHandle, MessagePublisherSvc,
};
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
