//! `MessagePublisher` — egress port for outbound message publishing.

use futures::future::BoxFuture;
use swe_edge_message_broker::Message;

use crate::api::port::publisher_result::PublisherResult;

/// Publishes messages to a named topic on an external message broker.
///
/// This is the **egress** side of the pub/sub contract. Use the SAF factories
/// to obtain a concrete implementation:
///
/// ```rust,ignore
/// let publisher = MessageBrokerSvc::default_publisher();
/// publisher.publish("orders.created", Message::new(b"{}")).await?;
/// ```
///
/// # Feature flags
///
/// | Feature    | Backend                               |
/// |------------|---------------------------------------|
/// | `in-memory`| `InMemoryMessageBroker` (tokio broadcast) |
/// | `nats`     | `NatsMessageBroker` (async-nats)      |
pub trait MessagePublisher: Send + Sync {
    /// Publish `msg` to `topic`.
    ///
    /// Fire-and-forget semantics: if no subscribers exist the message is silently
    /// dropped (in-memory backend) or acknowledged by the broker (NATS backend).
    fn publish<'a>(&'a self, topic: &'a str, msg: Message) -> BoxFuture<'a, PublisherResult<()>>;

    /// Verify the publisher is connected and the broker is reachable.
    fn health_check(&self) -> BoxFuture<'_, PublisherResult<()>>;
}
