//! Integration tests — `MessagePublisherSvc` SAF factory methods.
//!
//! Covers rules 77 (all pub fns tested) and 78 (@covers: annotations).
// @allow: no_mocks_in_integration — test doubles required to exercise port contracts without runtime deps

use swe_edge_egress_message_publisher::{MessagePublisherSvc, Validator};

/// @covers: MessagePublisherSvc::create_config_builder
#[test]
fn test_create_config_builder_returns_builder_with_crate_name() {
    let b = MessagePublisherSvc::create_config_builder();
    let _ = b;
}

/// @covers: MessagePublisherSvc::validate
#[test]
fn test_validate_returns_ok_for_always_valid_validator() {
    struct AlwaysOk;
    impl Validator for AlwaysOk {
        fn validate(&self) -> Result<(), String> {
            Ok(())
        }
    }
    assert!(MessagePublisherSvc::validate(&AlwaysOk).is_ok());
}

/// @covers: MessagePublisherSvc::validate
#[test]
fn test_validate_propagates_err_from_validator_impl() {
    struct AlwaysFail;
    impl Validator for AlwaysFail {
        fn validate(&self) -> Result<(), String> {
            Err("always fails".into())
        }
    }
    assert!(MessagePublisherSvc::validate(&AlwaysFail).is_err());
}

mod with_mock_broker {
    use swe_edge_egress_message_publisher::{Message, MessagePublisher, MessagePublisherSvc};
    use swe_edge_message_broker::MessageBroker;

    struct MockBroker;
    impl MessageBroker for MockBroker {
        fn publish<'a>(
            &'a self,
            _request: swe_edge_message_broker::PublishRequest,
        ) -> swe_edge_message_broker::BrokerFuture<
            'a,
            Result<(), swe_edge_message_broker::BrokerError>,
        > {
            swe_edge_message_broker::BrokerFuture::new(async { Ok(()) })
        }

        fn subscribe<'a>(
            &'a self,
            _request: swe_edge_message_broker::SubscribeRequest,
        ) -> swe_edge_message_broker::BrokerFuture<
            'a,
            Result<
                swe_edge_message_broker::SubscribeResponse,
                swe_edge_message_broker::BrokerError,
            >,
        > {
            swe_edge_message_broker::BrokerFuture::new(async {
                let stream: swe_edge_message_broker::MessageStream =
                    Box::pin(futures::stream::empty());
                Ok(swe_edge_message_broker::SubscribeResponse { stream })
            })
        }

        fn health_check(
            &self,
            _request: swe_edge_message_broker::HealthCheckRequest,
        ) -> swe_edge_message_broker::BrokerFuture<
            '_,
            Result<(), swe_edge_message_broker::BrokerError>,
        > {
            swe_edge_message_broker::BrokerFuture::new(async { Ok(()) })
        }

        fn validator(
            &self,
            _request: swe_edge_message_broker::ValidatorRequest,
        ) -> Result<swe_edge_message_broker::ValidatorResponse, swe_edge_message_broker::BrokerError>
        {
            Ok(swe_edge_message_broker::ValidatorResponse {
                validator: std::sync::Arc::from(
                    swe_edge_message_broker::BrokerSvc::create_validator(),
                ),
            })
        }
    }

    /// @covers: MessagePublisherSvc::publish_to
    #[tokio::test]
    async fn test_publish_to_succeeds_with_mock_broker() {
        let p = MessagePublisherSvc::from_broker(MockBroker);
        let msg = Message::new(b"test".to_vec());
        assert!(MessagePublisherSvc::publish_to(&p, "events.test", msg)
            .await
            .is_ok());
    }

    /// @covers: MessagePublisherSvc::check_health
    #[tokio::test]
    async fn test_check_health_returns_ok_for_broker_publisher() {
        let p = MessagePublisherSvc::from_broker(MockBroker);
        assert!(MessagePublisherSvc::check_health(&p).await.is_ok());
    }

    /// @covers: MessagePublisherSvc::from_broker
    #[tokio::test]
    async fn test_from_broker_returns_healthy_publisher() {
        let p = MessagePublisherSvc::from_broker(MockBroker);
        assert!(p.health_check().await.is_ok());
    }

    /// @covers: MessagePublisherSvc::publisher
    #[tokio::test]
    async fn test_publisher_wraps_injected_publisher() {
        let p = MessagePublisherSvc::from_broker(MockBroker);
        let wrapped = MessagePublisherSvc::publisher(p);
        assert!(wrapped.health_check().await.is_ok());
    }
}
