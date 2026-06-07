//! Integration tests — MessagePublisherHandle type.
// @allow: no_mocks_in_integration — test doubles required to exercise port contracts without runtime deps

use swe_edge_egress_message_publisher::{
    MessagePublisher, MessagePublisherHandle, MessagePublisherSvc,
};
use swe_edge_message_broker::{BrokerError, Message, MessageBroker, MessageStream};

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

/// @covers: MessagePublisherHandle — wraps publisher via MessagePublisherSvc::from_broker
#[tokio::test]
async fn test_message_publisher_handle_wraps_broker_publisher() {
    let h: MessagePublisherHandle = MessagePublisherSvc::from_broker(MockBroker);
    assert!(h.health_check().await.is_ok());
}
