//! Example — publish a message using an injected mock broker.
//!
//! Run with:
//! ```bash
//! cargo run --example publish
//! ```

// Examples favour terse `.expect()` over production-grade error handling.
#![allow(clippy::expect_used, clippy::unwrap_used)]

use swe_edge_egress_message_publisher::{Message, MessagePublisherSvc};
use swe_edge_message_broker::{BrokerError, MessageBroker, MessageStream};

struct MockBroker;
impl MessageBroker for MockBroker {
    fn publish<'a>(&'a self, _: &'a str, _: Message) -> futures::future::BoxFuture<'a, Result<(), BrokerError>> {
        Box::pin(futures::future::ready(Ok(())))
    }
    fn subscribe<'a>(&'a self, _: &'a str) -> futures::future::BoxFuture<'a, Result<MessageStream, BrokerError>> {
        Box::pin(futures::future::ready(Ok(Box::pin(futures::stream::empty()) as MessageStream)))
    }
    fn health_check(&self) -> futures::future::BoxFuture<'_, Result<(), BrokerError>> {
        Box::pin(futures::future::ready(Ok(())))
    }
}

#[tokio::main]
async fn main() {
    // The assembler injects the broker. Here we use a mock for illustration.
    let publisher = MessagePublisherSvc::from_broker(MockBroker);
    let msg = Message::new(b"hello, world".to_vec());
    MessagePublisherSvc::publish_to(&publisher, "example.topic", msg)
        .await
        .expect("publish failed");
    println!("Message published.");
}
