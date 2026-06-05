//! Example — publish a message using the in-memory backend.
//!
//! Run with:
//! ```bash
//! cargo run --example publish --features in-memory
//! ```

// Examples favour terse `.expect()` over production-grade error handling.
#![allow(clippy::expect_used, clippy::unwrap_used)]

#[cfg(feature = "in-memory")]
#[tokio::main]
async fn main() {
    use swe_edge_egress_message_broker::{Message, MessageBrokerSvc};

    let publisher = MessageBrokerSvc::default_publisher();
    let msg = Message::new(b"hello, world".to_vec());
    MessageBrokerSvc::publish_to(&publisher, "example.topic", msg)
        .await
        .expect("publish failed");
    println!("Message published.");
}

#[cfg(not(feature = "in-memory"))]
fn main() {
    eprintln!("Enable the `in-memory` feature to run this example.");
}
