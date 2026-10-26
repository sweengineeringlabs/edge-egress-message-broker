//! Integration tests — SAF public API for the egress message publisher.
//!
//! Covers rules 125 (SAF pub fn must have API-level tests) and 77 (all pub fns tested).
// @allow: no_mocks_in_integration — test doubles required to exercise port contracts without runtime deps

use swe_edge_egress_message_publisher::{
    ApplicationConfigBuilder, Message, MessagePublisher, MessagePublisherSvc, Validator,
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

struct AlwaysValid;
impl Validator for AlwaysValid {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[tokio::test]
async fn test_from_broker_saf_factory_returns_healthy_publisher() {
    let p = MessagePublisherSvc::from_broker(MockBroker);
    assert!(p.health_check().await.is_ok());
}

#[tokio::test]
async fn test_publish_to_succeeds_with_mock_broker() {
    let p = MessagePublisherSvc::from_broker(MockBroker);
    let msg = Message::new(b"test".to_vec());
    assert!(MessagePublisherSvc::publish_to(&p, "events.test", msg)
        .await
        .is_ok());
}

#[tokio::test]
async fn test_check_health_returns_ok_for_broker_publisher() {
    let p = MessagePublisherSvc::from_broker(MockBroker);
    assert!(MessagePublisherSvc::check_health(&p).await.is_ok());
}

#[test]
fn test_validate_returns_ok_for_always_valid() {
    assert!(MessagePublisherSvc::validate(&AlwaysValid).is_ok());
}

#[test]
fn test_application_config_builder_builds_with_custom_capacity() {
    let cfg = ApplicationConfigBuilder::new().with_capacity(256);
    assert_eq!(cfg.capacity, 256);
}
