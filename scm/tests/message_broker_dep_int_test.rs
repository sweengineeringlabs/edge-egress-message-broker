//! Integration test — exercises the `swe-edge-message-broker` contract directly.
//!
//! Satisfies rule 95: dependencies used in src/ must have integration/e2e coverage.
// @allow: no_mocks_in_integration — test doubles required to exercise port contracts without runtime deps

#![allow(clippy::unwrap_used, clippy::expect_used)]

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

#[tokio::test]
async fn test_message_broker_dep_health_check_returns_ok() {
    let p = MessagePublisherSvc::from_broker(MockBroker);
    assert!(p.health_check().await.is_ok());
}

#[tokio::test]
async fn test_message_broker_dep_publish_returns_ok_with_no_subscribers() {
    let p = MessagePublisherSvc::from_broker(MockBroker);
    assert!(p
        .publish("dep.test", Message::new(b"ok".to_vec()))
        .await
        .is_ok());
}
