//! `MessagePublisher` — egress trait for outbound message publishing.

use futures::future::BoxFuture;
use swe_edge_message_broker::Message;

use crate::api::types::publisher_result::PublisherResult;

/// Publishes messages to a named topic on an external message broker.
///
/// Use the SAF factories to obtain a concrete implementation:
///
/// ```rust,ignore
/// let publisher = MessagePublisherSvc::from_broker(broker);
/// publisher.publish("orders.created", Message::new(b"{}".to_vec())).await?;
/// ```
pub trait MessagePublisher: Send + Sync {
    /// Publish `msg` to `topic`.
    fn publish<'a>(&'a self, topic: &'a str, msg: Message) -> BoxFuture<'a, PublisherResult<()>>;

    /// Verify the publisher is connected and the broker is reachable.
    fn health_check(&self) -> BoxFuture<'_, PublisherResult<()>>;
}
