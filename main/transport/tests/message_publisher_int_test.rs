//! Integration tests — MessagePublisher trait contract.

use futures::future::BoxFuture;
use swe_edge_egress_message_broker::{Message, MessagePublisher, PublisherResult};

struct NeverPublisher;
impl MessagePublisher for NeverPublisher {
    fn publish<'a>(&'a self, _: &'a str, _: Message) -> BoxFuture<'a, PublisherResult<()>> {
        Box::pin(futures::future::ready(Ok(())))
    }
    fn health_check(&self) -> BoxFuture<'_, PublisherResult<()>> {
        Box::pin(futures::future::ready(Ok(())))
    }
}

/// @covers: MessagePublisher — object safety
#[test]
fn test_message_publisher_is_object_safe() {
    fn _assert(_: &dyn MessagePublisher) {}
}

/// @covers: MessagePublisher::health_check
#[tokio::test]
async fn test_message_publisher_health_check_returns_ok() {
    assert!(NeverPublisher.health_check().await.is_ok());
}

/// @covers: MessagePublisher::publish
#[tokio::test]
async fn test_message_publisher_publish_returns_ok() {
    let msg = Message::new(b"test".to_vec());
    assert!(NeverPublisher.publish("t", msg).await.is_ok());
}
